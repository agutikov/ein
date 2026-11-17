//! `.einb` — the binary knowledge-base container
//! ([03-design/09-binary-format.md](../../../../docs/ein/ein.rs/03-design/09-binary-format.md)).
//!
//! A loaded — and optionally saturated — KB on disk, in a form the engine
//! opens with no import resolution, no macro expansion and no filesystem walk.
//! Parse + load is a fixed cost every run pays before it saturates or
//! searches; this is the section of that cost the container removes.
//!
//! **It is a private, versioned cache format, not an interchange format.**
//! `.ein` text is still the only authoring format and the only thing anyone
//! edits. Nothing that crosses a tool boundary is `.einb`.
//!
//! ## What is in a file
//!
//! `META` (what it was made from), `SYMBOLS` / `FACTS` (the interned
//! tables, in id order), `PRESENT`
//! (which of those facts this KB believes, in
//! insertion order), `PROV` (the derivation records and the per-fact primary /
//! alternative tables), `PROGRAM` (the resolved, import-flattened form list),
//! and optionally `SOLUTIONS`, `CLOSURE` (each derived fact's closure and
//! smallest-closure arrival, which a full alternatives list keeps beside it)
//! and `UNIVERSAL` (the facts each derivation's universals read, which a core
//! follows). The format is **2.0**: no time in `META`, so a save is a
//! function of what was saved ([`Meta`], Q-M1f.110), and no integer table
//! (below). This reader opens 2.x alone, and refuses another major with the
//! version line.
//!
//! **No integer table, and a value with the tag bits `0b01` is refused**
//! (Q-M1f.10). Ein has no integers — a run of digits is a name — so there is
//! no int pool and no id space for one. The integer table's kind number, 3,
//! is not reused. A stored value tagged `0b01` is refused,
//! [`EinbError::Int`], never read as a name or a fact.
//!
//! ## What is not a section
//!
//! **`INDEXES`.** The kind is known and this writer never emits
//! one: every reverse index is a projection of the fact list in insertion
//! order, and [`Kb::rebuild_indexes`] is the projection — the same function
//! the loader ends with, so a rebuilt index is not *equal to* the original by
//! argument, it is produced by the code that defines what the original is. It
//! costs one linear pass on open against a larger file, and a stored index is
//! a second encoding of
//! derived state for a round trip to disagree about. A reader still skips a
//! kind-8 section rather than refusing it, so a writer that changes its mind
//! does not need a major bump.
//!
//! **The equality classes.** [`ein_core::EqClasses`] is inert: the engine
//! never unions, and the only callers in the tree are tests (Q1). A `.einb`
//! of a KB with equality classes would lose them, and there is no way to make
//! one.
//!
//! ## Ids across the boundary
//!
//! `Symbol` / `FactId` / `ProvId` are process-local. Each space is
//! stored in id order and re-interned on open, which yields a translation
//! table per space; every other section is remapped through them in one linear
//! pass, and the pass is skipped when the tables come back the identity —
//! which is what a fresh process opening one file gets. [`tables`] is that,
//! and
//! [03-design/09-binary-format.md § 3](../../../../docs/ein/ein.rs/03-design/09-binary-format.md#3-ids-across-the-boundary)
//! is why.
//!
//! ## Trusting a file
//!
//! The header carries BLAKE3 of everything after it, so a truncated or
//! bit-flipped file is refused rather than misread. `META` carries the engine
//! semver, the stdlib manifest hash and the source digests: differing *inputs*
//! are a **cache miss**, not an error, and a differing *engine* keeps
//! `PROGRAM` — re-loading it is exactly what reading the `.ein` would have
//! done — and drops everything derived. See [`meta`].
//!
//! ### The `u32` offsets
//!
//! `ein-core` narrows a `len()` into a `u32` arena offset, and this container
//! is the crate where that shape matters most: it is the one [`cast`] permits
//! `unsafe` in and the one that reads bytes it did not write.
//!
//! - **The reader is covered, and not by the digest.** A string table's
//!   offsets must *close* (`offsets[count] == blob.len()`) and be *sorted*,
//!   and every slice is taken with `get(..)` rather than indexed — three
//!   named refusals in [`sections`], on the path a forged file takes after
//!   the digest has already been made to match.
//! - **The writer is not**, and does not need to be while every offset it
//!   writes comes from a `Terms` that refuses to exceed
//!   [`ein_core::intern::ARENA_CAPACITY`]. The bound is one crate down, which
//!   is where it belongs: a container cannot serialise a store that cannot
//!   exist.
//! - **A fact's arity is no offset and is bounded the same way.**
//!   `FactStore::intern`, two crates below this one, refuses a fact wider than
//!   its arity field from *both* directions — a 65 536-argument fact in a
//!   `.ein`, or a forged `Facts` row here — so this reader's `?` carries it.
//!   The fuzzers in `tests/corruption.rs` cannot reach it: they mutate the
//!   bytes of a seed with narrow rows, and a byte flip cannot manufacture
//!   65 536 arguments.

#![deny(unsafe_code)]

pub mod cast;
pub mod header;
pub mod meta;
pub mod sections;
pub mod solutions;
pub mod tables;
pub mod wire;

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

pub use header::{FORMAT_MAJOR, FORMAT_MINOR, Kind, is_einb};
pub use meta::{Freshness, KbState, Meta, Source, engine_version, stdlib_digest};
pub use solutions::{BaseStats, MonotonicStats, SolutionNode, Solutions};

use header::{ALIGN, Entry, HEADER_LEN, Header};
use wire::{Reader, Writer};

/// Everything that can go wrong reading a file that may not be one.
#[derive(Debug)]
pub enum EinbError {
    Io(std::io::Error),
    /// The magic is not `EINB\0`.
    NotEinb,
    /// A major version this reader does not implement.
    Version {
        major: u16,
        minor: u16,
    },
    /// The header digest does not describe the body.
    Digest,
    /// A read ran off the end of the file, or of a section.
    Truncated,
    /// The bytes are structurally wrong in a way that is worth naming.
    Malformed(&'static str),
    /// An id names a table entry that does not exist.
    BadId {
        what: &'static str,
        id: u32,
    },
    MissingSection(&'static str),
    /// A section is compressed and this build cannot decompress it.
    Compressed,
    /// Re-interning the file's tables overflowed the 30-bit id space.
    Overflow(ein_core::Overflow),
    /// `PROGRAM` did not parse, or did not load.
    Program(String),
    /// The file disagrees with itself: re-interning entry *i* did not yield
    /// the id the file says it has.
    NotInjective(&'static str),
    /// The file holds a value with the tag bits `0b01`, which no value has
    /// (Q-M1f.10). There is no value to read it as.
    Int,
}

impl std::fmt::Display for EinbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EinbError::Io(e) => write!(f, "{e}"),
            EinbError::NotEinb => write!(f, "not an .einb file (bad magic)"),
            EinbError::Version { major, minor } => write!(
                f,
                ".einb format {major}.{minor} — this engine reads {FORMAT_MAJOR}.x"
            ),
            EinbError::Digest => write!(f, ".einb digest mismatch — the file is damaged"),
            EinbError::Truncated => write!(f, ".einb is truncated"),
            EinbError::Malformed(what) => write!(f, ".einb is malformed: {what}"),
            EinbError::BadId { what, id } => {
                write!(f, ".einb names {what} {id}, which is not in its table")
            }
            EinbError::MissingSection(s) => write!(f, ".einb has no {s} section"),
            EinbError::Compressed => write!(f, ".einb section is compressed — unsupported"),
            EinbError::Overflow(o) => write!(f, "{o}"),
            EinbError::Program(m) => write!(f, ".einb PROGRAM does not load: {m}"),
            EinbError::NotInjective(what) => {
                write!(f, ".einb {what} table does not re-intern to its own ids")
            }
            EinbError::Int => write!(
                f,
                ".einb holds an integer, which Ein does not have (a run of digits is a \
                 name, Q-M1e2.24): save it again from its .ein"
            ),
        }
    }
}

impl std::error::Error for EinbError {}

impl From<std::io::Error> for EinbError {
    fn from(e: std::io::Error) -> EinbError {
        EinbError::Io(e)
    }
}

impl From<ein_core::Overflow> for EinbError {
    fn from(e: ein_core::Overflow) -> EinbError {
        EinbError::Overflow(e)
    }
}

pub type Result<T> = std::result::Result<T, EinbError>;

/// What to put in the file beyond the KB itself.
#[derive(Default)]
pub struct SaveOptions {
    pub state: KbState,
    /// The `.ein` files this KB came from, hashed — [`Source::of`].
    pub sources: Vec<Source>,
    pub solutions: Option<Solutions>,
}

/// Serialise a KB, its interned tables and the program behind it.
///
/// `forms` are the file's **parsed** top-level forms and `base_dir` is what
/// their `(import …)` resolves against; the container resolves them itself and
/// stores the flattened result, which is the whole reason an opened file needs
/// no filesystem.
pub fn save_to_vec(
    kb: &Kb,
    terms: &Terms,
    ast: &mut Ast,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    opts: &SaveOptions,
) -> Result<Vec<u8>> {
    let program = sections::program_section(ast, terms, forms, base_dir)?;

    let mut bodies: Vec<(Kind, Vec<u8>)> = Vec::new();
    bodies.push((Kind::Meta, {
        let mut w = Writer::new();
        meta_for(opts, kb).write(&mut w);
        w.into_vec()
    }));
    bodies.push((Kind::Symbols, sections::write_symbols(terms)));
    bodies.push((Kind::Facts, sections::write_facts(terms)));
    bodies.push((Kind::Present, sections::write_present(kb)));
    let kept = sections::Kept::of(kb);
    bodies.push((Kind::Prov, sections::write_prov(kb, terms, &kept)));
    bodies.push((Kind::Program, program));
    if let Some(s) = &opts.solutions {
        bodies.push((Kind::Solutions, solutions::write(s)));
    }
    let universal = sections::write_universal(terms, &kept);
    if !universal.is_empty() {
        bodies.push((Kind::Universal, universal));
    }
    let closure = sections::write_closure(kb, &kept);
    if !closure.is_empty() {
        bodies.push((Kind::Closure, closure));
    }

    // The table's size is known from the section count, so offsets can be laid
    // out before a byte of it is written.
    let mut table = Writer::new();
    let mut body = Writer::new();
    let start = HEADER_LEN + bodies.len() * header::ENTRY_LEN;
    for (kind, bytes) in &bodies {
        body.align(ALIGN);
        Entry {
            kind: *kind as u32,
            flags: 0,
            off: (start + body.len()) as u64,
            len: bytes.len() as u64,
            raw_len: bytes.len() as u64,
        }
        .write(&mut table);
        body.bytes(bytes);
    }
    body.align(ALIGN);

    let mut after_header = table.into_vec();
    after_header.extend_from_slice(body.as_slice());
    let mut out = Writer::new();
    Header {
        major: FORMAT_MAJOR,
        minor: FORMAT_MINOR,
        flags: 0,
        n_sections: bodies.len() as u32,
        digest: *blake3::hash(&after_header).as_bytes(),
    }
    .write(&mut out);
    out.bytes(&after_header);
    Ok(out.into_vec())
}

/// [`save_to_vec`], to a path.
pub fn save(
    path: &Path,
    kb: &Kb,
    terms: &Terms,
    ast: &mut Ast,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    opts: &SaveOptions,
) -> Result<()> {
    let bytes = save_to_vec(kb, terms, ast, forms, base_dir, opts)?;
    std::fs::write(path, bytes)?;
    Ok(())
}

fn meta_for(opts: &SaveOptions, kb: &Kb) -> Meta {
    Meta {
        engine: engine_version().to_string(),
        writer: format!("ein-einb {}", engine_version()),
        state: opts.state,
        config: kb.program().config.clone(),
        sources: opts.sources.clone(),
        stdlib: stdlib_digest(),
    }
}

/// How much of a file to believe, and what to check first.
pub struct OpenOptions {
    /// The inputs as they are *now*, for the cache-miss test. Empty means
    /// "did not look", which claims nothing.
    pub sources: Vec<Source>,
    /// `--trust-cache` turns this off
    /// ([03-design/09-binary-format.md § 4.2](../../../../docs/ein/ein.rs/03-design/09-binary-format.md#42-a-file-refused)).
    /// On by default, because a digest is cheap and a silently misread KB is a
    /// wrong answer with a straight face.
    pub verify_digest: bool,
    /// The engine asking. Defaults to this build's.
    pub engine: String,
    /// Which `(query …)` block the rebuilt program is *about*.
    ///
    /// A container stores its program as canonical text and re-parses it, so
    /// every query in the source is in the file; without this the reader would
    /// always rebuild about the first one, and `ein solve x.einb` would answer
    /// question 1 while saying it answered question 2.
    pub query: usize,
    /// What the rebuilt program's load requires. A container
    /// re-loads its program from the text it stores, so the requirement of a
    /// `:why`, the default, refuses a `.einb` exactly where it
    /// refuses the `.ein` it was saved from, and `--no-require-why` loads both.
    pub load: ein_ir::LoadOptions,
    /// The container's own name, its path as the caller was handed it, which
    /// every position in the stored program names through [`program_name`]
    /// (Q-M1f.15). [`open`] fills it from its path; `None` is a
    /// container read from bytes with no name.
    pub name: Option<String>,
}

impl Default for OpenOptions {
    fn default() -> OpenOptions {
        OpenOptions {
            sources: Vec::new(),
            verify_digest: true,
            engine: engine_version().to_string(),
            query: 0,
            load: ein_ir::LoadOptions::default(),
            name: None,
        }
    }
}

/// The name a container's stored program is parsed under, and so the file
/// every position in it names: `<program stored in x.einb>`, or `<program
/// stored in a .einb>` for a container read from bytes with no name
/// (Q-M1f.15).
///
/// A line and column of it are the stored text's, the resolved and
/// import-flattened program [`ein_ir::dump_canonical`] wrote, and not the
/// source's: the container keeps no source position past the provenance
/// table's (`sections::program_section`). The
/// name says which text the position is in, in brackets as `<string>` is,
/// since no file holds it.
pub fn program_name(container: Option<&str>) -> String {
    match container {
        Some(path) => format!("<program stored in {path}>"),
        None => "<program stored in a .einb>".to_string(),
    }
}

/// A file, opened.
pub struct Opened {
    pub kb: Kb,
    /// The AST the program was rebuilt into. The KB's `ExprRef`s index it, so
    /// it has to outlive the KB.
    pub ast: Ast,
    /// The stored program's top-level forms in [`Self::ast`]: the resolved,
    /// import-flattened program, which is what a caller that re-saves the
    /// container hands [`save`].
    pub forms: Vec<NodeId>,
    pub meta: Meta,
    pub freshness: Freshness,
    /// True when the file's derived sections were dropped and the KB is the
    /// loader's output rather than the file's. Never true for a
    /// [`KbState::Loaded`] file, which has nothing derived to drop.
    pub derived_dropped: bool,
    /// Present only when the file carried one *and* it was believed.
    pub solutions: Option<Solutions>,
    /// False when every id space came back the identity — the fast path of
    /// [03-design/09-binary-format.md § 3](../../../../docs/ein/ein.rs/03-design/09-binary-format.md#3-ids-across-the-boundary),
    /// and the number a benchmark wants to know it took.
    pub remapped: bool,
}

/// Open a `.einb` into `terms`, which may already hold other content.
pub fn open_bytes(bytes: &[u8], terms: &mut Terms, opts: &OpenOptions) -> Result<Opened> {
    let mut r = Reader::new(bytes);
    let head = Header::read(&mut r)?;
    if head.flags != 0 {
        return Err(EinbError::Malformed("unknown header flags"));
    }
    let after_header = bytes.get(HEADER_LEN..).ok_or(EinbError::Truncated)?;
    if opts.verify_digest && blake3::hash(after_header).as_bytes() != &head.digest {
        return Err(EinbError::Digest);
    }
    let mut entries = Vec::with_capacity(head.n_sections.min(1024) as usize);
    for _ in 0..head.n_sections {
        entries.push(Entry::read(&mut r)?);
    }

    let mut found: [Option<&[u8]>; 13] = [None; 13];
    for e in &entries {
        if e.flags != 0 || e.len != e.raw_len {
            return Err(EinbError::Compressed);
        }
        let (off, len) = (e.off as usize, e.len as usize);
        let end = off.checked_add(len).ok_or(EinbError::Truncated)?;
        let body = bytes.get(off..end).ok_or(EinbError::Truncated)?;
        // An unknown kind in this major is skipped, not refused: that is what
        // makes a minor bump additive (09-binary-format.md § 4.1).
        if let Some(kind) = Kind::from_u32(e.kind) {
            found[kind as usize] = Some(body);
        }
    }
    let want =
        |k: Kind, name: &'static str| found[k as usize].ok_or(EinbError::MissingSection(name));

    let meta = Meta::read(&mut Reader::new(want(Kind::Meta, "META")?))?;
    let freshness = meta.freshness(&opts.engine, &opts.sources, &stdlib_digest());

    let mut maps = tables::Maps::default();
    sections::read_symbols(want(Kind::Symbols, "SYMBOLS")?, terms, &mut maps)?;
    sections::read_facts(want(Kind::Facts, "FACTS")?, terms, &mut maps)?;
    let remapped = !maps.identity();

    // `PROV` before the registries are rebuilt, and the program's file-name
    // table before both: rebuilding pushes the loader's own records into the
    // arena, so anything read afterwards lands at an offset — and the case
    // worth keeping cheap is the one where nothing does.
    let mut ast = Ast::new();
    let program =
        sections::read_program_names(want(Kind::Program, "PROGRAM")?, &mut ast, &mut maps)?;
    let provs = sections::read_prov(
        want(Kind::Prov, "PROV")?,
        found[Kind::Universal as usize],
        terms,
        &mut maps,
    )?;
    let name = program_name(opts.name.as_deref());
    let (loaded, forms) =
        sections::load_program(&program, &mut ast, terms, opts.query, &opts.load, &name)?;

    if meta.state.is_derived() && !freshness.keeps_derived() {
        return Ok(Opened {
            kb: loaded,
            ast,
            forms,
            meta,
            freshness,
            derived_dropped: true,
            solutions: None,
            remapped,
        });
    }

    let mut kb = sections::read_present(
        want(Kind::Present, "PRESENT")?,
        terms,
        &maps,
        &provs,
        loaded.program_arc(),
    )?;
    if let Some(body) = found[Kind::Closure as usize] {
        sections::read_closure(body, &mut kb, &maps)?;
    }
    // The rebuild is from the file's fact list; the rules-by-relation snapshot
    // is not, and `loaded` is where a load-time one comes from.
    kb.rebuild_indexes_from(terms, &loaded);
    let solutions = match found[Kind::Solutions as usize] {
        Some(body) => Some(solutions::read(body, &maps)?),
        None => None,
    };
    Ok(Opened {
        kb,
        ast,
        forms,
        meta,
        freshness,
        derived_dropped: false,
        solutions,
        remapped,
    })
}

/// The `META` section alone — what the file says it was made from, without
/// re-interning a byte of it.
///
/// The reason it is separate: the freshness test needs the *recorded* source
/// paths before it can re-hash them, and re-hashing is the caller's to decide
/// (a container shipped without its sources should not read as stale).
pub fn meta_of(bytes: &[u8]) -> Result<Meta> {
    let mut r = Reader::new(bytes);
    let head = Header::read(&mut r)?;
    for _ in 0..head.n_sections {
        let e = Entry::read(&mut r)?;
        if Kind::from_u32(e.kind) == Some(Kind::Meta) {
            let (off, len) = (e.off as usize, e.len as usize);
            let end = off.checked_add(len).ok_or(EinbError::Truncated)?;
            let body = bytes.get(off..end).ok_or(EinbError::Truncated)?;
            return Meta::read(&mut Reader::new(body));
        }
    }
    Err(EinbError::MissingSection("META"))
}

/// What each section costs, without opening the file.
///
/// The size ceiling is a number about a whole file
/// (`tests/roundtrip.rs`' `a_saturated_zebra2_is_under_80_kb_and_opens_cold_under_6_ms_or_20_with_assertions`),
/// and a whole-file number tells nobody which section grew. This is what a test — or
/// anyone holding a `.einb` that got big — reads instead.
pub fn section_sizes(bytes: &[u8]) -> Result<Vec<(Kind, u64)>> {
    let mut r = Reader::new(bytes);
    let head = Header::read(&mut r)?;
    let mut out = Vec::new();
    for _ in 0..head.n_sections {
        let e = Entry::read(&mut r)?;
        if let Some(kind) = Kind::from_u32(e.kind) {
            out.push((kind, e.len));
        }
    }
    Ok(out)
}

/// The program a container stores, as text, without opening its KB: what a
/// command that parses a program and never loads it reads from a `.einb`
/// (`ein render rules`, `constraints`, `program`).
///
/// The header and, with `verify_digest`, the digest are checked as
/// [`open_bytes`] checks them, so a damaged file is refused here as it is
/// there. The text is the resolved, import-flattened program
/// [`ein_ir::dump_canonical`] wrote; parse it under [`program_name`].
pub fn program_of(bytes: &[u8], verify_digest: bool) -> Result<String> {
    let mut r = Reader::new(bytes);
    let head = Header::read(&mut r)?;
    if head.flags != 0 {
        return Err(EinbError::Malformed("unknown header flags"));
    }
    let after_header = bytes.get(HEADER_LEN..).ok_or(EinbError::Truncated)?;
    if verify_digest && blake3::hash(after_header).as_bytes() != &head.digest {
        return Err(EinbError::Digest);
    }
    for _ in 0..head.n_sections {
        let e = Entry::read(&mut r)?;
        if Kind::from_u32(e.kind) != Some(Kind::Program) {
            continue;
        }
        if e.flags != 0 || e.len != e.raw_len {
            return Err(EinbError::Compressed);
        }
        let (off, len) = (e.off as usize, e.len as usize);
        let end = off.checked_add(len).ok_or(EinbError::Truncated)?;
        let body = bytes.get(off..end).ok_or(EinbError::Truncated)?;
        let mut scratch = Ast::new();
        let program =
            sections::read_program_names(body, &mut scratch, &mut tables::Maps::default())?;
        return Ok(program.text().to_string());
    }
    Err(EinbError::MissingSection("PROGRAM"))
}

/// [`open_bytes`], from a path, through an 8-byte-aligned buffer so the
/// zero-copy casts are the path taken. The path names the stored program
/// ([`OpenOptions::name`]) where the caller left it unnamed.
pub fn open(path: &Path, terms: &mut Terms, opts: &OpenOptions) -> Result<Opened> {
    let bytes = cast::Aligned::read(path)?;
    if opts.name.is_some() {
        return open_bytes(bytes.as_slice(), terms, opts);
    }
    let named = OpenOptions {
        sources: opts.sources.clone(),
        verify_digest: opts.verify_digest,
        engine: opts.engine.clone(),
        query: opts.query,
        load: opts.load,
        name: Some(path.display().to_string()),
    };
    open_bytes(bytes.as_slice(), terms, &named)
}
