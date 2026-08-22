//! The sections — T1a.8.1.2.
//!
//! Every one of them is written in **id order** and read back by re-interning
//! in the same order, which is what makes the translation tables of
//! [`crate::tables`] a `Vec` lookup rather than a search. The arrays that are
//! worth borrowing rather than decoding — the string offset tables, the fact
//! rows, the argument arena, the fact-id lists — are laid out so
//! [`crate::cast`] can hand back a `&[T]` over the file's own bytes.

use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;

use ein_core::facts::FactId;
use ein_core::intern::Symbol;
use ein_core::prov::{NafArg, NafRef, Prov, ProvId, ProvKind};
use ein_core::value::Value;
use ein_core::{Kb, Loc, Program, Terms};
use ein_ir::{Ast, NodeId};

use crate::cast::{self, RawRow};
use crate::tables::Maps;
use crate::wire::{Reader, Writer};
use crate::{EinbError, Result};

/// How deep a negative premise's pattern may nest before the reader calls it
/// malformed.
///
/// `(absent (R ?a (S ?b …)))` nests as deep as the puzzle wrote it, which is
/// two in the corpus; the cap is not a semantic limit but a stack one, because
/// the decoder recurses and the input may be arbitrary bytes.
const MAX_NAF_DEPTH: u32 = 64;

// ── SYMBOLS / INTS ─────────────────────────────────────────────────

/// A string table: `count`, the blob length, `count + 1` prefix-sum offsets,
/// then the blob.
///
/// The offsets are stored rather than the lengths so that a reader can borrow
/// them as `&[u32]` and take entry *i* with two loads and no scan.
fn write_strings<'a>(count: usize, text: impl Fn(usize) -> &'a str) -> Vec<u8> {
    let mut w = Writer::new();
    w.u32(count as u32);
    let total: usize = (0..count).map(|i| text(i).len()).sum();
    w.u32(total as u32);
    let mut at = 0u32;
    for i in 0..count {
        w.u32(at);
        at += text(i).len() as u32;
    }
    w.u32(at);
    for i in 0..count {
        w.bytes(text(i).as_bytes());
    }
    w.align(crate::header::ALIGN);
    w.into_vec()
}

/// The offsets and the blob of a string table, with the offsets borrowed when
/// the bytes allow it.
fn read_strings(body: &[u8]) -> Result<(Cow<'_, [u32]>, &[u8])> {
    let mut r = Reader::new(body);
    let count = r.u32()? as usize;
    let total = r.u32()? as usize;
    let offsets = u32_array(&mut r, count + 1)?;
    let blob = r.take(total)?;
    if offsets[count] as usize != total {
        return Err(EinbError::Malformed("string table offsets do not close"));
    }
    if offsets.windows(2).any(|w| w[0] > w[1]) {
        return Err(EinbError::Malformed("string table offsets are not sorted"));
    }
    Ok((offsets, blob))
}

fn nth_str<'a>(offsets: &[u32], blob: &'a [u8], i: usize) -> Result<&'a str> {
    let (from, to) = (offsets[i] as usize, offsets[i + 1] as usize);
    let bytes = blob.get(from..to).ok_or(EinbError::Truncated)?;
    std::str::from_utf8(bytes).map_err(|_| EinbError::Malformed("string table is not UTF-8"))
}

pub fn write_symbols(terms: &Terms) -> Vec<u8> {
    write_strings(terms.syms.len(), |i| terms.syms.text(Symbol(i as u32)))
}

/// Re-intern every symbol in id order, filling [`Maps::sym`].
pub fn read_symbols(body: &[u8], terms: &mut Terms, maps: &mut Maps) -> Result<()> {
    let (offsets, blob) = read_strings(body)?;
    maps.sym = Vec::with_capacity(offsets.len().saturating_sub(1));
    for i in 0..offsets.len() - 1 {
        maps.sym
            .push(terms.syms.intern(nth_str(&offsets, blob, i)?)?);
    }
    Ok(())
}

pub fn write_ints(terms: &Terms) -> Vec<u8> {
    // The canonical decimal text is the whole entry: `IntPool::intern`
    // re-derives the `Option<i64>` fast field from it, and re-deriving is what
    // makes a stored pool provably the pool that text produces.
    write_strings(terms.ints.len(), |i| {
        terms.ints.text(ein_core::IntId(i as u32))
    })
}

pub fn read_ints(body: &[u8], terms: &mut Terms, maps: &mut Maps) -> Result<()> {
    let (offsets, blob) = read_strings(body)?;
    maps.int = Vec::with_capacity(offsets.len().saturating_sub(1));
    for i in 0..offsets.len() - 1 {
        maps.int
            .push(terms.ints.intern(nth_str(&offsets, blob, i)?)?);
    }
    Ok(())
}

// ── FACTS ──────────────────────────────────────────────────────────

/// `n_rows`, `n_args`, the rows, then the flat argument arena.
///
/// Every argument goes to the arena, including the one or two an in-memory
/// `Row` would hold inline — see [`RawRow`].
pub fn write_facts(terms: &Terms) -> Vec<u8> {
    let mut rows: Vec<RawRow> = Vec::with_capacity(terms.facts.len());
    let mut args: Vec<u32> = Vec::new();
    for i in 0..terms.facts.len() {
        let (rel, a) = terms.facts.get(FactId(i as u32));
        rows.push(RawRow {
            rel: rel.0,
            args_at: args.len() as u32,
            arity: a.len() as u32,
        });
        args.extend(a.iter().map(|v| v.bits()));
    }
    let mut w = Writer::new();
    w.u32(rows.len() as u32);
    w.u32(args.len() as u32);
    w.bytes(cast::bytes_of(&rows));
    w.bytes(cast::bytes_of(&args));
    w.align(crate::header::ALIGN);
    w.into_vec()
}

/// Re-intern every row in `FactId` order, filling [`Maps::fact`].
///
/// A row's arguments may name an earlier fact — `(not (color-loc Red H1))` is
/// a fact whose one argument is another — and interning in id order is what
/// makes that always already-translated. A *forward* reference is refused: it
/// cannot arise from a writer, and following one would read an id that does
/// not exist yet.
pub fn read_facts(body: &[u8], terms: &mut Terms, maps: &mut Maps) -> Result<()> {
    let mut r = Reader::new(body);
    let n_rows = r.u32()? as usize;
    let n_args = r.u32()? as usize;
    let rows: Cow<'_, [RawRow]> = {
        let bytes = r.take(
            n_rows
                .checked_mul(size_of::<RawRow>())
                .ok_or(EinbError::Truncated)?,
        )?;
        match cast::slice_of::<RawRow>(bytes) {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(decode_rows(bytes)),
        }
    };
    let args = u32_array(&mut r, n_args)?;

    maps.fact = Vec::with_capacity(n_rows);
    let mut buf: Vec<Value> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let at = row.args_at as usize;
        let end = at
            .checked_add(row.arity as usize)
            .ok_or(EinbError::Truncated)?;
        let slice = args.get(at..end).ok_or(EinbError::Truncated)?;
        buf.clear();
        for &bits in slice {
            // The check is on the *file's* id, before the remap: a live id is
            // whatever this process's store had room for, and comparing one
            // against a count of file entries would call a perfectly ordinary
            // nested fact a forward reference in every crowded interner.
            let stored = Value::from_bits(bits);
            if bits != Value::UNBOUND.bits()
                && stored.tag() == ein_core::Tag::Fact
                && stored.payload() as usize >= i
            {
                return Err(EinbError::Malformed("a fact argument names a later fact"));
            }
            buf.push(maps.value(bits)?);
        }
        let rel = maps.symbol(row.rel)?;
        let id = terms.intern_fact(rel, &buf)?;
        maps.fact.push(id);
        // The file claims its rows are distinct. If two of them intern to one
        // id the table would alias, and every id past that point would mean
        // something else.
        if maps.fact.len() != i + 1 {
            return Err(EinbError::NotInjective("fact"));
        }
    }
    Ok(())
}

fn decode_rows(bytes: &[u8]) -> Vec<RawRow> {
    bytes
        .chunks_exact(size_of::<RawRow>())
        .map(|c| RawRow {
            rel: u32::from_le_bytes([c[0], c[1], c[2], c[3]]),
            args_at: u32::from_le_bytes([c[4], c[5], c[6], c[7]]),
            arity: u32::from_le_bytes([c[8], c[9], c[10], c[11]]),
        })
        .collect()
}

/// `n` little-endian `u32`s, borrowed when the bytes are aligned.
fn u32_array<'a>(r: &mut Reader<'a>, n: usize) -> Result<Cow<'a, [u32]>> {
    let bytes = r.take(n.checked_mul(4).ok_or(EinbError::Truncated)?)?;
    Ok(match cast::slice_of::<u32>(bytes) {
        Some(s) => Cow::Borrowed(s),
        None => Cow::Owned(
            bytes
                .chunks_exact(4)
                .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        ),
    })
}

// ── PRESENT ────────────────────────────────────────────────────────

/// The believed facts, in insertion order.
///
/// The presence bitset design/10 §2 names is **not** stored beside it: a fact
/// is present exactly when it is in this list, because the only writer of
/// either is `push_fact`, which writes both, and nothing removes. Storing the
/// bitset would be storing the list's own membership twice, with a way for the
/// two to disagree.
pub fn write_present(kb: &Kb) -> Vec<u8> {
    let mut w = Writer::new();
    w.u32(kb.n_facts() as u32);
    for f in kb.facts() {
        w.u32(f.0);
    }
    w.align(crate::header::ALIGN);
    w.into_vec()
}

pub fn read_present(
    body: &[u8],
    terms: &Terms,
    maps: &Maps,
    provs: &ProvTables,
    program: Arc<Program>,
) -> Result<Kb> {
    let mut r = Reader::new(body);
    let n = r.count(4)?;
    let ids = u32_array(&mut r, n)?;
    let mut kb = Kb::with_program(program);
    let mut seen = vec![false; terms.facts.len()];
    for &raw in ids.iter() {
        let id = maps.fact(raw)?;
        let slot = seen.get_mut(id.0 as usize).ok_or(EinbError::BadId {
            what: "fact",
            id: raw,
        })?;
        if *slot {
            return Err(EinbError::Malformed("a fact is believed twice"));
        }
        *slot = true;
        kb.restore_fact(id, provs.primary(id));
    }
    for (fact, alts) in &provs.alts {
        kb.restore_alternatives(*fact, alts.clone());
    }
    Ok(kb)
}

// ── PROV ───────────────────────────────────────────────────────────

/// The per-fact tables `PROV` carries beside the arena.
#[derive(Default)]
pub struct ProvTables {
    /// Indexed by live `FactId`; `None` where the fact has no record.
    primary: Vec<Option<ProvId>>,
    alts: Vec<(FactId, Box<[ProvId]>)>,
}

impl ProvTables {
    fn primary(&self, fact: FactId) -> Option<ProvId> {
        self.primary.get(fact.0 as usize).copied().flatten()
    }
}

/// The whole arena, then the primary map and the alternative lists.
///
/// The arena is written in full — including records no *believed* fact points
/// at any more — because a `ProvId` is only meaningful as an index into it and
/// every other section stores one. Since T1a.7.1.7 that is a much smaller
/// claim than it was: what a search left behind used to be every record every
/// fork ever derived (2 135 093 of them on `features/01 -e`, of which twelve
/// were live) and is now only what root itself wrote.
///
/// **A fork's id is not writable.** The id stored below is the record's
/// position in the scan above, which holds for the arena proper and for
/// nothing else; a `ProvId` from the fork region indexes a table this file
/// does not carry and would not survive the run. Saving happens between
/// enterings, so there are none — asserted rather than assumed, because the
/// failure is a saved KB whose derivations silently point at the wrong
/// records.
pub fn write_prov(kb: &Kb, terms: &Terms) -> Vec<u8> {
    let mut w = Writer::new();
    w.u32(terms.provs.len() as u32);
    // `scan`, not `get`: this walks the arena end to end rather than
    // following a reference, and a record a finished fork left behind is
    // exactly what the comment above says it writes.
    for record in terms.provs.scan() {
        write_record(&mut w, record);
    }
    let believed: Vec<FactId> = kb.facts().collect();
    let primary: Vec<(FactId, ProvId)> = believed
        .iter()
        .filter_map(|&f| kb.primary(f).map(|p| (f, p)))
        .collect();
    w.u32(primary.len() as u32);
    for (f, p) in primary {
        assert!(!p.is_fork(), "a fork's provenance cannot be saved");
        w.u32(f.0);
        w.u32(p.0);
    }
    let alts: Vec<(FactId, &[ProvId])> = believed
        .iter()
        .map(|&f| (f, kb.alternatives(f)))
        .filter(|(_, a)| !a.is_empty())
        .collect();
    w.u32(alts.len() as u32);
    for (f, a) in alts {
        w.u32(f.0);
        w.u32(a.len() as u32);
        for p in a {
            assert!(!p.is_fork(), "a fork's provenance cannot be saved");
            w.u32(p.0);
        }
    }
    w.align(crate::header::ALIGN);
    w.into_vec()
}

/// Append the file's records to the live arena and fill [`Maps::prov`].
///
/// A record's `Loc` names a file by index, and [`read_program_names`] has
/// already re-interned those names into the reconstructed `Ast`, so the
/// position still says where the fact came from rather than pointing into the
/// canonical text the reader is about to parse.
pub fn read_prov(body: &[u8], terms: &mut Terms, maps: &mut Maps) -> Result<ProvTables> {
    let mut r = Reader::new(body);
    let n = r.count(9)?;
    maps.prov = Vec::with_capacity(n);
    for _ in 0..n {
        let record = read_record(&mut r, maps)?;
        maps.prov.push(terms.provs.push(record));
    }
    let mut out = ProvTables {
        primary: vec![None; terms.facts.len()],
        alts: Vec::new(),
    };
    let n_primary = r.count(8)?;
    for _ in 0..n_primary {
        let fact = maps.fact(r.u32()?)?;
        let prov = maps.prov(r.u32()?)?;
        let slot = out
            .primary
            .get_mut(fact.0 as usize)
            .ok_or(EinbError::BadId {
                what: "fact",
                id: fact.0,
            })?;
        *slot = Some(prov);
    }
    let n_alts = r.count(8)?;
    for _ in 0..n_alts {
        let fact = maps.fact(r.u32()?)?;
        let len = r.count(4)?;
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            list.push(maps.prov(r.u32()?)?);
        }
        out.alts.push((fact, list.into_boxed_slice()));
    }
    Ok(out)
}

fn write_record(w: &mut Writer, p: &Prov) {
    w.u8(match p.kind {
        ProvKind::Source => 0,
        ProvKind::Rule => 1,
        ProvKind::Hypothesis => 2,
        ProvKind::Rejected => 3,
    });
    w.opt_u32(p.source.map(|s| s.0));
    w.opt_u32(p.rule.map(|s| s.0));
    w.u32(p.premises.len() as u32);
    for f in &p.premises {
        w.u32(f.0);
    }
    w.u32(p.bindings.len() as u32);
    for (name, value) in &p.bindings {
        w.u32(name.0);
        w.u32(value.bits());
    }
    w.u32(p.absent.len() as u32);
    for a in &p.absent {
        write_naf(w, a);
    }
    w.opt_u32(p.branch);
    match p.loc {
        None => {
            w.u8(0);
            w.u32(0);
            w.u32(0);
            w.u32(0);
        }
        Some(l) => {
            w.u8(1);
            w.u32(l.file);
            w.u32(l.line);
            w.u32(l.col);
        }
    }
}

fn read_record(r: &mut Reader<'_>, maps: &Maps) -> Result<Prov> {
    let kind = match r.u8()? {
        0 => ProvKind::Source,
        1 => ProvKind::Rule,
        2 => ProvKind::Hypothesis,
        3 => ProvKind::Rejected,
        _ => return Err(EinbError::Malformed("unknown provenance kind")),
    };
    let source = r.opt_u32()?.map(|s| maps.symbol(s)).transpose()?;
    let rule = r.opt_u32()?.map(|s| maps.symbol(s)).transpose()?;
    let n = r.count(4)?;
    let mut premises = Vec::with_capacity(n);
    for _ in 0..n {
        premises.push(maps.fact(r.u32()?)?);
    }
    let n = r.count(8)?;
    let mut bindings = Vec::with_capacity(n);
    for _ in 0..n {
        let name = maps.symbol(r.u32()?)?;
        bindings.push((name, maps.value(r.u32()?)?));
    }
    let n = r.count(8)?;
    let mut absent = Vec::with_capacity(n);
    for _ in 0..n {
        absent.push(read_naf(r, maps, 0)?);
    }
    let branch = r.opt_u32()?;
    let has_loc = r.u8()?;
    let (file, line, col) = (r.u32()?, r.u32()?, r.u32()?);
    let loc = match has_loc {
        0 => None,
        1 => Some(Loc {
            file: maps.loc_file(file).unwrap_or(file),
            line,
            col,
        }),
        _ => return Err(EinbError::Malformed("loc flag is not 0 or 1")),
    };
    Ok(Prov {
        kind,
        source,
        rule,
        premises: premises.into_boxed_slice(),
        bindings: bindings.into_boxed_slice(),
        absent: absent.into_boxed_slice(),
        branch,
        loc,
    })
}

fn write_naf(w: &mut Writer, n: &NafRef) {
    w.u32(n.rel.0);
    w.u32(n.args.len() as u32);
    for a in &n.args {
        write_naf_arg(w, a);
    }
}

fn write_naf_arg(w: &mut Writer, a: &NafArg) {
    match a {
        NafArg::Free => w.u8(0),
        NafArg::Value(v) => {
            w.u8(1);
            w.u32(v.bits());
        }
        NafArg::Nested { rel, args } => {
            w.u8(2);
            w.u32(rel.0);
            w.u32(args.len() as u32);
            for inner in args {
                write_naf_arg(w, inner);
            }
        }
    }
}

fn read_naf(r: &mut Reader<'_>, maps: &Maps, depth: u32) -> Result<NafRef> {
    if depth > MAX_NAF_DEPTH {
        return Err(EinbError::Malformed("negative premise nests too deep"));
    }
    let rel = maps.symbol(r.u32()?)?;
    let n = r.count(1)?;
    let mut args = Vec::with_capacity(n);
    for _ in 0..n {
        args.push(read_naf_arg(r, maps, depth + 1)?);
    }
    Ok(NafRef {
        rel,
        args: args.into_boxed_slice(),
    })
}

fn read_naf_arg(r: &mut Reader<'_>, maps: &Maps, depth: u32) -> Result<NafArg> {
    if depth > MAX_NAF_DEPTH {
        return Err(EinbError::Malformed("negative premise nests too deep"));
    }
    Ok(match r.u8()? {
        0 => NafArg::Free,
        1 => NafArg::Value(maps.value(r.u32()?)?),
        2 => {
            let rel = maps.symbol(r.u32()?)?;
            let n = r.count(1)?;
            let mut args = Vec::with_capacity(n);
            for _ in 0..n {
                args.push(read_naf_arg(r, maps, depth + 1)?);
            }
            NafArg::Nested {
                rel,
                args: args.into_boxed_slice(),
            }
        }
        _ => return Err(EinbError::Malformed("unknown negative-premise argument")),
    })
}

// ── PROGRAM ────────────────────────────────────────────────────────

/// The resolved, import-flattened form list, as canonical text, plus the file
/// names its provenance refers to.
///
/// **Text, and not the AST arenas.** Two reasons, one of them measured: the
/// arenas for a resolved `zebra2` are 3 024 nodes and 3 024 optional `Loc`s —
/// past 60 KB before a fact is stored, against a design budget of 64 KB for
/// the whole file — while the canonical dump of the same forms is 11 KB. The
/// other is the stage's own note: `dump_canonical` is already the frontend's
/// serialiser, and a second one is a second thing to keep in parity.
///
/// What it costs is a parse of already-resolved text on open — no imports, no
/// filesystem, no macro invocations left to chase — and what it buys, beyond
/// the size, is that the reconstructed `Ast` is the parse of exactly these
/// bytes, so every `ExprRef` the rebuilt registries hold indexes the AST that
/// is there.
pub fn program_section(
    ast: &mut Ast,
    terms: &Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
) -> Result<Vec<u8>> {
    let resolved =
        ein_ir::resolve_imports(ast, forms, base_dir).map_err(|e| EinbError::Program(e.0))?;
    let text = ein_ir::dump_canonical(ast, &resolved);
    // Only the file names provenance can name: the rebuilt registries' own
    // `Loc`s point into the canonical text and are not preserved.
    // As `write_prov`: a scan of the arena, not a walk of what is live. It
    // therefore sizes the file table against records a finished fork left
    // behind too, which is a superset and so harmless — but it is why this
    // reads through `scan`.
    let max = terms
        .provs
        .scan()
        .filter_map(|p| p.loc.map(|l| l.file))
        .max();
    let mut w = Writer::new();
    match max {
        None => w.u32(0),
        Some(max) => {
            w.u32(max + 1);
            for i in 0..=max {
                w.str(ast.file(ein_ir::FileId(i)));
            }
        }
    }
    w.str(&text);
    w.align(crate::header::ALIGN);
    Ok(w.into_vec())
}

/// The program section, read but not yet loaded.
pub struct ProgramSection<'a> {
    text: &'a str,
    root: Option<String>,
}

/// The file-name table, interned into a fresh `Ast`, and the text set aside.
///
/// Split from [`load_program`] for one reason, and it is the reason the whole
/// reader is ordered the way it is: rebuilding the registries **pushes the
/// loader's own provenance records into the arena**, so anything read after it
/// lands at an offset. Reading `PROV` first keeps `ProvId` the identity in the
/// case that matters — a fresh process opening one file — and the file names
/// have to be interned before that, because a record's `Loc` names one.
pub fn read_program_names<'a>(
    body: &'a [u8],
    ast: &mut Ast,
    maps: &mut Maps,
) -> Result<ProgramSection<'a>> {
    let mut r = Reader::new(body);
    let n_files = r.count(4)?;
    maps.file = Vec::with_capacity(n_files);
    let mut root: Option<String> = None;
    for i in 0..n_files {
        let name = r.str()?;
        if i == 0 {
            root = Some(name.to_string());
        }
        maps.file.push(ast.intern_file(Some(name)).0);
    }
    Ok(ProgramSection {
        text: r.str()?,
        root,
    })
}

/// Parse the stored text and rebuild the registries from it.
///
/// The returned `Kb` is the loader's own output — the program *and* the source
/// facts it declares. A fresh file's reader throws the facts away and installs
/// the ones the file stored; a reader that has decided not to believe the
/// derived sections keeps this one, because re-loading `PROGRAM` is exactly
/// what reading the `.ein` would have done (design/10 §4).
pub fn load_program(section: &ProgramSection<'_>, ast: &mut Ast, terms: &mut Terms) -> Result<Kb> {
    let forms = ein_ir::parse(ast, section.text, section.root.as_deref())
        .map_err(|e| EinbError::Program(e.to_string()))?;
    ein_ir::load(ast, terms, &forms, None).map_err(|e| EinbError::Program(e.0))
}

// ── NOGOODS ────────────────────────────────────────────────────────

/// The learned clauses, sorted.
///
/// `Nogoods` is a hash set and its iteration order is not reproducible, so the
/// clauses are sorted before they are written — by length then by id, a total
/// order over data that already sorts its own elements
/// ([design/02 §6](../../../../plans/m1a_rust/design/02_determinism_and_order.md)).
/// Two runs that learn the same clauses therefore write the same bytes.
pub fn write_nogoods(kb: &Kb) -> Vec<u8> {
    // determinism-ok: collected and then sorted, which is what makes the bytes
    // independent of the set's iteration order.
    let mut clauses: Vec<Vec<u32>> = kb
        .nogoods()
        .read()
        .expect("no writer panicked")
        .iter()
        .map(|c| c.iter().map(|f| f.0).collect())
        .collect();
    if clauses.is_empty() {
        return Vec::new();
    }
    clauses.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    let mut w = Writer::new();
    w.u32(clauses.len() as u32);
    for c in &clauses {
        w.u32(c.len() as u32);
        for f in c {
            w.u32(*f);
        }
    }
    w.align(crate::header::ALIGN);
    w.into_vec()
}

pub fn read_nogoods(body: &[u8], kb: &Kb, maps: &Maps) -> Result<()> {
    let mut r = Reader::new(body);
    let n = r.count(4)?;
    let mut store = kb.nogoods().write().expect("no reader panicked");
    for _ in 0..n {
        let len = r.count(4)?;
        let mut clause = Vec::with_capacity(len);
        for _ in 0..len {
            clause.push(maps.fact(r.u32()?)?);
        }
        // The stored order is the sort's; a clause's own canonical form is
        // sorted by id, which is what `emit_nogood` writes and what
        // subsumption compares.
        clause.sort();
        store.insert(clause.into_boxed_slice());
    }
    Ok(())
}
