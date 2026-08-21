//! `META` — what the file was made from, and therefore when to disbelieve it
//! (T1a.8.1.5, [design/10
//! §4](../../../../plans/m1a_rust/design/10_binary_format.md#4-versioning-and-invalidation)).
//!
//! Two different questions live here and they have different answers.
//!
//! **Are the inputs the same?** The source digests and the stdlib manifest
//! hash. A difference is a **cache miss**, not an error: the file describes a
//! program that is not the one on disk any more, so the caller re-reads the
//! `.ein` and carries on. Treating it as an error would make a stale cache
//! able to change a verdict, which is the one thing a cache must never do.
//!
//! **Was it made by this engine?** The semver. Derived state — a saturated
//! fact set, a solution — is only meaningful under the engine that derived it,
//! so a mismatch keeps `PROGRAM` and drops the rest. `PROGRAM` survives
//! because it is the *input*, restated: re-loading it is exactly what reading
//! the `.ein` would have done.

use ein_core::SolverConfig;
use ein_core::config::FieldKind;

use crate::wire::{Reader, Writer};
use crate::{EinbError, Result};

/// How much of a KB's history the file holds.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum KbState {
    /// Straight out of the loader: every fact is a `:source` fact, and the
    /// file is the loader's output rather than the engine's.
    #[default]
    Loaded,
    /// A least fixpoint has been reached — the fact set includes derivations.
    Saturated,
    /// Saturated, and a `SOLUTIONS` section carries what the search found.
    Solved,
}

impl KbState {
    /// Does this state contain anything an engine *derived*? The question
    /// [`Freshness`] answers a version mismatch with.
    pub fn is_derived(self) -> bool {
        self != KbState::Loaded
    }

    fn tag(self) -> u8 {
        match self {
            KbState::Loaded => 0,
            KbState::Saturated => 1,
            KbState::Solved => 2,
        }
    }

    fn from_tag(t: u8) -> Result<KbState> {
        Ok(match t {
            0 => KbState::Loaded,
            1 => KbState::Saturated,
            2 => KbState::Solved,
            _ => return Err(EinbError::Malformed("unknown KB state")),
        })
    }
}

/// One input file, by path and content.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Source {
    pub path: String,
    pub digest: [u8; 32],
}

impl Source {
    /// Hash a file's bytes, or `None` when it cannot be read — a source that
    /// has since been deleted is a cache miss like any other change.
    pub fn of(path: &std::path::Path) -> Option<Source> {
        let bytes = std::fs::read(path).ok()?;
        Some(Source {
            path: path.display().to_string(),
            digest: *blake3::hash(&bytes).as_bytes(),
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Meta {
    /// The engine semver that wrote the file.
    pub engine: String,
    /// Free text naming the writer, for a human reading a `.einb` header.
    pub writer: String,
    /// Seconds since the Unix epoch, or `0` when the clock was unreadable.
    /// Informational only: nothing compares it, because a cache keyed on time
    /// is a cache that expires correct answers.
    pub created_unix: u64,
    pub state: KbState,
    /// The `SolverConfig` in force when the file was written — the same
    /// resolution `solve` did, banked, so a reader can see what produced the
    /// derived sections.
    pub config: Option<SolverConfig>,
    pub sources: Vec<Source>,
    /// BLAKE3 of `stdlib/MANIFEST.sha256`, which is the one input whose
    /// divergence nothing else would notice ([design/11
    /// §3](../../../../plans/m1a_rust/design/11_shared_assets.md)).
    pub stdlib: [u8; 32],
}

/// What a reader decided about a file's inputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Freshness {
    /// Same engine, same inputs: everything in the file may be believed.
    Fresh,
    /// A source file's bytes moved. The file describes another program.
    StaleSource,
    /// The stdlib moved under it — same puzzle text, different `std.*`.
    StaleStdlib,
    /// A different engine derived it. `PROGRAM` stands; the rest does not.
    OtherEngine,
}

impl Freshness {
    /// May the derived sections be believed?
    pub fn keeps_derived(self) -> bool {
        self == Freshness::Fresh
    }
}

impl Meta {
    /// Compare against the inputs a reader can see right now.
    ///
    /// `sources` is what the caller re-hashed; an empty list means "did not
    /// look", which is not evidence of staleness and so does not report any.
    pub fn freshness(&self, engine: &str, sources: &[Source], stdlib: &[u8; 32]) -> Freshness {
        if self.engine != engine {
            return Freshness::OtherEngine;
        }
        if &self.stdlib != stdlib {
            return Freshness::StaleStdlib;
        }
        for want in &self.sources {
            match sources.iter().find(|s| s.path == want.path) {
                Some(got) if got.digest == want.digest => {}
                Some(_) => return Freshness::StaleSource,
                // Not re-hashed by this caller: nothing is claimed about it.
                None => {}
            }
        }
        Freshness::Fresh
    }

    pub fn write(&self, w: &mut Writer) {
        w.str(&self.engine);
        w.str(&self.writer);
        w.u64(self.created_unix);
        w.u8(self.state.tag());
        w.bytes(&self.stdlib);
        w.u32(self.sources.len() as u32);
        for s in &self.sources {
            w.str(&s.path);
            w.bytes(&s.digest);
        }
        match &self.config {
            None => w.u8(0),
            Some(c) => {
                w.u8(1);
                write_config(w, c);
            }
        }
    }

    pub fn read(r: &mut Reader<'_>) -> Result<Meta> {
        let engine = r.str()?.to_string();
        let writer = r.str()?.to_string();
        let created_unix = r.u64()?;
        let state = KbState::from_tag(r.u8()?)?;
        let stdlib = r.array::<32>()?;
        // 4 bytes of path length + 32 of digest is the floor for one source.
        let n = r.count(36)?;
        let mut sources = Vec::with_capacity(n);
        for _ in 0..n {
            sources.push(Source {
                path: r.str()?.to_string(),
                digest: r.array::<32>()?,
            });
        }
        let config = match r.u8()? {
            0 => None,
            1 => Some(read_config(r)?),
            _ => return Err(EinbError::Malformed("config flag is not 0 or 1")),
        };
        Ok(Meta {
            engine,
            writer,
            created_unix,
            state,
            config,
            sources,
            stdlib,
        })
    }
}

/// The engine semver a fresh file carries — the workspace version, which every
/// crate in it shares.
pub fn engine_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// BLAKE3 of the stdlib manifest, through the same three-step resolution the
/// engine uses, so the hash describes the `std.*` a run would actually read.
pub fn stdlib_digest() -> [u8; 32] {
    match ein_ir::stdlib::resolve_default().read(ein_ir::stdlib::MARKER) {
        Some(text) => *blake3::hash(text.as_bytes()).as_bytes(),
        None => [0; 32],
    }
}

/// `SolverConfig`, field by field in [`ein_core::config::FIELDS`] order.
///
/// Walking the declared list rather than writing seventeen lines by hand is
/// not brevity: the list is what `--dump-config` prints and what the loader
/// parses, so a field added there is a field this reads, and a field it did
/// not know about would otherwise be silently dropped from every `.einb`.
fn write_config(w: &mut Writer, c: &SolverConfig) {
    for (name, kind) in ein_core::config::FIELDS {
        match kind {
            FieldKind::Bool => w.u8(u8::from(config_bool(c, name))),
            FieldKind::Int => match config_int(c, name) {
                Some(v) => {
                    w.u8(1);
                    w.i64(v);
                }
                None => {
                    w.u8(0);
                    w.i64(0);
                }
            },
            FieldKind::Float => w.f64(config_float(c, name)),
            FieldKind::Str => w.str(config_str(c, name)),
        }
    }
}

fn read_config(r: &mut Reader<'_>) -> Result<SolverConfig> {
    let mut c = SolverConfig::default();
    for (name, kind) in ein_core::config::FIELDS {
        match kind {
            FieldKind::Bool => {
                let v = r.u8()?;
                if v > 1 {
                    return Err(EinbError::Malformed("config bool is not 0 or 1"));
                }
                set_bool(&mut c, name, v == 1);
            }
            FieldKind::Int => {
                let present = r.u8()?;
                let v = r.i64()?;
                match present {
                    0 => set_int(&mut c, name, None),
                    1 => set_int(&mut c, name, Some(v)),
                    _ => return Err(EinbError::Malformed("config int flag is not 0 or 1")),
                }
            }
            FieldKind::Float => {
                let v = r.f64()?;
                set_float(&mut c, name, v);
            }
            FieldKind::Str => {
                let v = r.str()?.to_string();
                set_str(&mut c, name, v);
            }
        }
    }
    Ok(c)
}

/// The flag-name → field map, in one place per direction.
///
/// `unreachable!` rather than a default: a name in [`ein_core::config::FIELDS`]
/// that no arm here handles is a field added to `SolverConfig` without being
/// added to the container, and the compiler cannot say so — the panic is at
/// `save` time, in the writer's own tests, which is where it is cheap.
fn config_bool(c: &SolverConfig, name: &str) -> bool {
    match name {
        "enable-pre-branch-lookahead" => c.enable_pre_branch_lookahead,
        "enable-lookahead-kill-cache" => c.enable_lookahead_kill_cache,
        "print-alive" => c.print_alive,
        "warn-derived-naf" => c.warn_derived_naf,
        "lattice-sanity-check" => c.lattice_sanity_check,
        "enable-path-nogoods" => c.enable_path_nogoods,
        "enable-symmetric-mirror" => c.enable_symmetric_mirror,
        "enable-singleton-writeback" => c.enable_singleton_writeback,
        "enable-forced-positive" => c.enable_forced_positive,
        "record-alternative-justifications" => c.record_alternative_justifications,
        "enable-fail-fast-fork" => c.enable_fail_fast_fork,
        other => unreachable!("`{other}` is a bool flag no `.einb` field reads"),
    }
}

fn set_bool(c: &mut SolverConfig, name: &str, v: bool) {
    match name {
        "enable-pre-branch-lookahead" => c.enable_pre_branch_lookahead = v,
        "enable-lookahead-kill-cache" => c.enable_lookahead_kill_cache = v,
        "print-alive" => c.print_alive = v,
        "warn-derived-naf" => c.warn_derived_naf = v,
        "lattice-sanity-check" => c.lattice_sanity_check = v,
        "enable-path-nogoods" => c.enable_path_nogoods = v,
        "enable-symmetric-mirror" => c.enable_symmetric_mirror = v,
        "enable-singleton-writeback" => c.enable_singleton_writeback = v,
        "enable-forced-positive" => c.enable_forced_positive = v,
        "record-alternative-justifications" => c.record_alternative_justifications = v,
        "enable-fail-fast-fork" => c.enable_fail_fast_fork = v,
        other => unreachable!("`{other}` is a bool flag no `.einb` field reads"),
    }
}

fn config_int(c: &SolverConfig, name: &str) -> Option<i64> {
    match name {
        "candidate-order-seed" => Some(c.candidate_order_seed),
        "lattice-order-seed" => c.lattice_order_seed,
        other => unreachable!("`{other}` is an int flag no `.einb` field reads"),
    }
}

fn set_int(c: &mut SolverConfig, name: &str, v: Option<i64>) {
    match name {
        "candidate-order-seed" => c.candidate_order_seed = v.unwrap_or(-1),
        "lattice-order-seed" => c.lattice_order_seed = v,
        other => unreachable!("`{other}` is an int flag no `.einb` field reads"),
    }
}

fn config_float(c: &SolverConfig, name: &str) -> f64 {
    match name {
        "hypgen-rel-weight" => c.hypgen_rel_weight,
        "hypgen-obj-weight" => c.hypgen_obj_weight,
        other => unreachable!("`{other}` is a float flag no `.einb` field reads"),
    }
}

fn set_float(c: &mut SolverConfig, name: &str, v: f64) {
    match name {
        "hypgen-rel-weight" => c.hypgen_rel_weight = v,
        "hypgen-obj-weight" => c.hypgen_obj_weight = v,
        other => unreachable!("`{other}` is a float flag no `.einb` field reads"),
    }
}

fn config_str<'a>(c: &'a SolverConfig, name: &str) -> &'a str {
    match name {
        "hypgen-scoring" => &c.hypgen_scoring,
        "lattice-order" => &c.lattice_order,
        other => unreachable!("`{other}` is a str flag no `.einb` field reads"),
    }
}

fn set_str(c: &mut SolverConfig, name: &str, v: String) {
    match name {
        "hypgen-scoring" => c.hypgen_scoring = v,
        "lattice-order" => c.lattice_order = v,
        other => unreachable!("`{other}` is a str flag no `.einb` field reads"),
    }
}
