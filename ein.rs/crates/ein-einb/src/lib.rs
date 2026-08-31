//! `.einb` — the binary knowledge-base container
//! ([P1a.8](../../../../docs/history/m1a_rust/README.md#p1a8--binary-kb-container),
//! [design/10](../../../../docs/history/m1a_rust/design/10_binary_format.md)).
//!
//! A loaded — and optionally saturated — KB on disk, in a form the engine
//! opens with no import resolution, no macro expansion and no filesystem walk.
//! Parse + load is 0.63 s of a 5.7 s CPython `zebra2` and stays the dominant
//! fixed cost once the engine is fast; this is the section of that cost the
//! container removes.
//!
//! **It is a private, versioned cache format, not an interchange format.**
//! `.ein` text is still the only authoring format and the only thing anyone
//! edits. Nothing that crosses a tool boundary is `.einb`.
//!
//! ## What is in a file
//!
//! `META` (what it was made from), `SYMBOLS` / `INTS` / `FACTS` (the interned
//! tables, in id order), `PRESENT` (which of those facts this KB believes, in
//! insertion order), `PROV` (the derivation records and the per-fact primary /
//! alternative tables), `PROGRAM` (the resolved, import-flattened form list),
//! and optionally `NOGOODS` and `SOLUTIONS`.
//!
//! ## What is not a section
//!
//! **`INDEXES`.** design/10 lists it as optional and this writer never emits
//! one: every reverse index is a projection of the fact list in insertion
//! order, and [`Kb::rebuild_indexes`] is the projection — the same function
//! the loader ends with, so a rebuilt index is not *equal to* the original by
//! argument, it is produced by the code that defines what the original is. It
//! costs one linear pass on open (`zebra2` saturated: 378 facts) against
//! roughly doubling the file, and a stored index is a second encoding of
//! derived state for a round trip to disagree about. A reader still skips a
//! kind-8 section rather than refusing it, so a writer that changes its mind
//! does not need a major bump.
//!
//! **The equality classes.** [`ein_core::EqClasses`] is the M1 placeholder for
//! F4's e-graph and the engine never unions — the only caller in the tree is
//! one test. A `.einb` of a KB with equality classes would lose them, and
//! there is no way to make one.
//!
//! ## Ids across the boundary
//!
//! `Symbol` / `IntId` / `FactId` / `ProvId` are process-local. Each space is
//! stored in id order and re-interned on open, which yields a translation
//! table per space; every other section is remapped through them in one linear
//! pass, and the pass is skipped when the tables come back the identity —
//! which is what a fresh process opening one file gets. [`tables`] is that,
//! and [design/10 §3] is why.
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
//! ### The `u32`-offset sweep — M1e S1e.4.1
//!
//! `CO-L1` found `ein-core` narrowing a `len()` into a `u32` arena offset with
//! only an *id-count* guard behind it, and asked whether this container has
//! the same shape. It is the crate worth asking about, because it is the one
//! [`cast`] permits `unsafe` in and the one that reads bytes it did not write.
//! The answer, swept 2026-09-01:
//!
//! - **The reader is covered, and not by the digest.** A string table's
//!   offsets must *close* (`offsets[count] == blob.len()`) and be *sorted*,
//!   and every slice is taken with `get(..)` rather than indexed — three
//!   named refusals in [`sections`], on the path a forged file takes after
//!   the digest has already been made to match.
//! - **The writer is not**, and does not need to be while every offset it
//!   writes comes from a `Terms` that now refuses to exceed
//!   [`ein_core::intern::ARENA_CAPACITY`]. The bound moved one crate down,
//!   which is where it belongs: a container cannot serialise a store that
//!   cannot exist.
//! - **The one hole was not an offset.** It was `FactStore::intern`'s arity
//!   `expect`, two crates below this one and reachable from *both* directions
//!   — a 65 536-argument fact in a `.ein`, or a forged `Facts` row here. It is
//!   a refusal now, so this reader's `?` carries it, and the fuzzers in
//!   `tests/corruption.rs` could never have found it: they mutate bytes of a
//!   20 KB seed whose widest row has 147 arguments, and a byte flip cannot
//!   manufacture 65 536.

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
pub use solutions::{SolutionNode, Solutions};

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
    bodies.push((Kind::Ints, sections::write_ints(terms)));
    bodies.push((Kind::Facts, sections::write_facts(terms)));
    bodies.push((Kind::Present, sections::write_present(kb)));
    bodies.push((Kind::Prov, sections::write_prov(kb, terms)));
    bodies.push((Kind::Program, program));
    let nogoods = sections::write_nogoods(kb);
    if !nogoods.is_empty() {
        bodies.push((Kind::Nogoods, nogoods));
    }
    if let Some(s) = &opts.solutions {
        bodies.push((Kind::Solutions, solutions::write(s)));
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
        created_unix: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
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
    /// `--trust-cache` turns this off (design/10 §4). On by default, because a
    /// digest over 30 KB is microseconds and a silently misread KB is a wrong
    /// answer with a straight face.
    pub verify_digest: bool,
    /// The engine asking. Defaults to this build's.
    pub engine: String,
    /// Which `(query …)` block the rebuilt program is *about* — M1c S1c.1.2.
    ///
    /// A container stores its program as canonical text and re-parses it, so
    /// every query in the source is in the file; without this the reader would
    /// always rebuild about the first one, and `ein solve x.einb` would answer
    /// question 1 while saying it answered question 2.
    pub query: usize,
}

impl Default for OpenOptions {
    fn default() -> OpenOptions {
        OpenOptions {
            sources: Vec::new(),
            verify_digest: true,
            engine: engine_version().to_string(),
            query: 0,
        }
    }
}

/// A file, opened.
pub struct Opened {
    pub kb: Kb,
    /// The AST the program was rebuilt into. The KB's `ExprRef`s index it, so
    /// it has to outlive the KB.
    pub ast: Ast,
    pub meta: Meta,
    pub freshness: Freshness,
    /// True when the file's derived sections were dropped and the KB is the
    /// loader's output rather than the file's. Never true for a
    /// [`KbState::Loaded`] file, which has nothing derived to drop.
    pub derived_dropped: bool,
    /// Present only when the file carried one *and* it was believed.
    pub solutions: Option<Solutions>,
    /// False when every id space came back the identity — design/10 §3's fast
    /// path, and the number a benchmark wants to know it took.
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

    let mut found: [Option<&[u8]>; 11] = [None; 11];
    for e in &entries {
        if e.flags != 0 || e.len != e.raw_len {
            return Err(EinbError::Compressed);
        }
        let (off, len) = (e.off as usize, e.len as usize);
        let end = off.checked_add(len).ok_or(EinbError::Truncated)?;
        let body = bytes.get(off..end).ok_or(EinbError::Truncated)?;
        // An unknown kind in this major is skipped, not refused: that is what
        // makes a minor bump additive (design/10 §4).
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
    sections::read_ints(want(Kind::Ints, "INTS")?, terms, &mut maps)?;
    sections::read_facts(want(Kind::Facts, "FACTS")?, terms, &mut maps)?;
    let remapped = !maps.identity();

    // `PROV` before the registries are rebuilt, and the program's file-name
    // table before both: rebuilding pushes the loader's own records into the
    // arena, so anything read afterwards lands at an offset — and the case
    // worth keeping cheap is the one where nothing does.
    let mut ast = Ast::new();
    let program =
        sections::read_program_names(want(Kind::Program, "PROGRAM")?, &mut ast, &mut maps)?;
    let provs = sections::read_prov(want(Kind::Prov, "PROV")?, terms, &mut maps)?;
    let loaded = sections::load_program(&program, &mut ast, terms, opts.query)?;

    if meta.state.is_derived() && !freshness.keeps_derived() {
        return Ok(Opened {
            kb: loaded,
            ast,
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
    if let Some(body) = found[Kind::Nogoods as usize] {
        sections::read_nogoods(body, &kb, &maps)?;
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
/// The size acceptance is a number about a whole file (design/10 §6), and a
/// whole-file number tells nobody which section grew. This is what a test — or
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

/// [`open_bytes`], from a path, through an 8-byte-aligned buffer so the
/// zero-copy casts are the path taken.
pub fn open(path: &Path, terms: &mut Terms, opts: &OpenOptions) -> Result<Opened> {
    let bytes = cast::Aligned::read(path)?;
    open_bytes(bytes.as_slice(), terms, opts)
}
