//! Where `std.*` comes from — the Rust half of the shared stdlib
//! (M1a S1a.0.3, [design/11](../../../../docs/history/m1a_rust/design/11_shared_assets.md)).
//!
//! The stdlib is checked in **once**, at repo-root `stdlib/`, and both
//! implementations read those bytes. It is not test data: 1 231 lines of
//! ein-lang across seven modules, three of which `zebra2.ein` imports from. A
//! second copy would make every parity result meaningless — a T2 diff would
//! report "the engines disagree" when in fact the *programs* differ.
//!
//! Two sources behind one trait, resolved in the same three steps `ein.py`'s
//! `imports._stdlib_root()` uses:
//!
//! 1. `$EIN_STDLIB` — an explicit override, always wins. It is what points a
//!    run at a stdlib that is not the checkout's, which is how a test can
//!    prove the resolution order rather than assume it.
//! 2. **The checkout**, found by walking up for a `stdlib/` carrying
//!    [`MARKER`]. A source tree is authoritative, so editing a module takes
//!    effect with no rebuild.
//! 3. **The embedded copy**, compiled in by `include_dir!`. This is what makes
//!    `ein.rs` one self-contained binary, and it is the step Python answers
//!    with the wheel's packaged copy.
//!
//! `Missing` is a fourth outcome, not an error here: the caller reports
//! "module not found at `<path>`", which names the path it looked at — the
//! message ein.py produces, and therefore the one to produce.

use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};

/// Identifies a directory as the stdlib. *Presence* is what the walk tests,
/// because a directory called `stdlib/` proves nothing on its own; the
/// content is checked by `tests::the_embedded_copy_matches_the_manifest`,
/// and written by `utils/stdlib_manifest.py --write`, which is the half no
/// test can do (a test that rewrote the file it checks would check nothing).
pub const MARKER: &str = "MANIFEST.sha256";

/// The compiled-in tree. Path is relative to this crate's manifest, so it is
/// resolved at build time and the bytes land in the binary.
static EMBEDDED: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../../stdlib");

/// Where a module's text came from — kept so a diagnostic can say which.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// An explicit `$EIN_STDLIB`.
    Override(PathBuf),
    /// A checkout found by walking up from the executable.
    Checkout(PathBuf),
    /// Compiled into this binary.
    Embedded,
}

impl Source {
    /// The directory this source reads from, for a message that names a path.
    /// The embedded copy has none, and reports itself as `<embedded>`.
    pub fn describe(&self) -> String {
        match self {
            Source::Override(p) | Source::Checkout(p) => p.display().to_string(),
            Source::Embedded => "<embedded>".to_string(),
        }
    }

    /// The path a module *would* live at — what "module not found at …"
    /// prints. For the embedded copy there is no filesystem path, so this
    /// composes one from the describe string; parity of that message with
    /// ein.py's is a P1a.2 question (ein.py has no embedded case).
    pub fn module_path(&self, rel: &str) -> String {
        format!("{}/{rel}", self.describe())
    }

    /// Why this source cannot be a stdlib — `None` when it can.
    ///
    /// **Only `$EIN_STDLIB` can answer anything but `None`**, and that
    /// asymmetry is the finding (M1e
    /// [S1e.3.5](../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.5_error_handling.md),
    /// `EH-M2`). The checkout walk *requires* [`MARKER`] — the marker exists
    /// precisely because a directory called `stdlib/` proves nothing — and the
    /// embedded copy is checked against the manifest at build time by
    /// `tests::the_embedded_copy_matches_the_manifest`. The
    /// highest-precedence source was the one that skipped the proof: an
    /// override was taken on faith, and a typo or a stale tree surfaced as
    /// *"module not found at &lt;path&gt;/algebra.ein"* — a true sentence that
    /// names the module and never mentions the variable that chose the
    /// directory.
    ///
    /// So the override has to prove itself the way the walk does, and the
    /// message is the fix rather than the check: what it costs a reader is
    /// the diagnosis, not the wrong answer.
    ///
    /// `ein --version` deliberately does **not** consult this — it reports the
    /// manifest as `unreadable` and keeps printing, because a version line
    /// that refused to render would be a worse way to learn the same thing.
    pub fn problem(&self) -> Option<String> {
        let Source::Override(path) = self else {
            return None;
        };
        let at = path.display();
        if !path.is_dir() {
            return Some(format!("$EIN_STDLIB names {at}, which is not a directory"));
        }
        if !path.join(MARKER).is_file() {
            return Some(format!(
                "$EIN_STDLIB names {at}, which has no {MARKER} — a directory is \
                 the stdlib only if it carries the marker, which is the same \
                 test the checkout walk applies (unset $EIN_STDLIB to use the \
                 checkout or the embedded copy, or run \
                 `utils/stdlib_manifest.py --write` in that tree)"
            ));
        }
        None
    }

    /// Read one module, by its path relative to the stdlib root
    /// (`"algebra.ein"`, `"sub/mod.ein"`).
    pub fn read(&self, rel: &str) -> Option<String> {
        match self {
            Source::Override(root) | Source::Checkout(root) => {
                std::fs::read_to_string(root.join(rel)).ok()
            }
            Source::Embedded => EMBEDDED
                .get_file(rel)
                .and_then(|f| f.contents_utf8())
                .map(str::to_string),
        }
    }

    /// Every module name in this source, sorted. Sorted because the caller
    /// may report them and iteration order must not reach an observable
    /// ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md)).
    pub fn modules(&self) -> Vec<String> {
        let mut out: Vec<String> = match self {
            Source::Override(root) | Source::Checkout(root) => std::fs::read_dir(root)
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| n.ends_with(".ein"))
                .collect(),
            Source::Embedded => EMBEDDED
                .files()
                .map(|f| f.path().display().to_string())
                .filter(|n| n.ends_with(".ein"))
                .collect(),
        };
        out.sort();
        out
    }
}

/// [`resolve`] starting from the running executable — what a caller with no
/// opinion about where to look should use.
///
/// In a test the executable lives under `target/`, which is inside the
/// checkout, so the walk finds the source tree; in an installed binary it does
/// not, and the embedded copy answers.
pub fn resolve_default() -> Source {
    let from = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));
    resolve(&from)
}

/// Resolve the stdlib source, in the three steps above.
///
/// `from` is where to start the checkout walk — the running executable's
/// directory in the binary, the crate directory in a test.
pub fn resolve(from: &Path) -> Source {
    resolve_with(from, std::env::var_os("EIN_STDLIB").map(PathBuf::from))
}

/// [`resolve`] with the override supplied rather than read.
///
/// The three tiers as a **pure function**, so a test can drive each of them
/// without writing to the process environment — which, in a `#[test]` running
/// as one thread of a shared binary, is a write every other test can see. The
/// tier tests used to be guarded on `EIN_STDLIB` being unset and returned
/// early when it was not; a check that answers *nothing* when the harness is
/// configured one way is [TE-M1](../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.6_tests.md)'s
/// shape, and M1e S1e.3.5 removed it here rather than leave two of the three
/// tiers conditionally unchecked.
pub fn resolve_with(from: &Path, over: Option<PathBuf>) -> Source {
    if let Some(over) = over {
        return Source::Override(over);
    }
    for parent in from.ancestors() {
        let candidate = parent.join("stdlib");
        if candidate.join(MARKER).is_file() {
            return Source::Checkout(candidate);
        }
    }
    Source::Embedded
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn crate_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The drift check, Rust side: what got compiled in is what is checked in.
    ///
    /// This is the whole point of the arrangement. `include_dir!` copies the
    /// tree at build time, so a binary built before an edit carries the old
    /// bytes — and a stdlib that differs between the two implementations is
    /// the one failure mode no parity tier can diagnose, because both engines
    /// would be *correct* about different programs.
    #[test]
    fn the_embedded_copy_matches_the_manifest() {
        let manifest = crate_dir().join("../../../stdlib").join(MARKER);
        let text = std::fs::read_to_string(&manifest)
            .unwrap_or_else(|e| panic!("{}: {e}", manifest.display()));
        let mut checked = 0;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let (sha, name) = line.split_once("  ").expect("<sha256>  <name>");
            let content = Source::Embedded
                .read(name)
                .unwrap_or_else(|| panic!("{name} is in the manifest but not embedded"));
            let got = format!("{:x}", Sha256::digest(content.as_bytes()));
            assert_eq!(got, sha, "{name}: embedded copy differs from the manifest");
            checked += 1;
        }
        assert!(checked >= 7, "only {checked} modules checked");
    }

    #[test]
    fn the_embedded_copy_has_no_extra_modules() {
        let manifest = crate_dir().join("../../../stdlib").join(MARKER);
        let text = std::fs::read_to_string(manifest).expect("manifest");
        let listed: Vec<&str> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.split_once("  ").expect("<sha256>  <name>").1)
            .collect();
        assert_eq!(Source::Embedded.modules(), listed);
    }

    /// **Tier 2 — the checkout walk**, driven with the override absent.
    ///
    /// Unconditional since M1e S1e.3.5: it used to `return` when
    /// `EIN_STDLIB` was set, which made it a test that answered nothing under
    /// exactly the configuration the review believed the harness used.
    #[test]
    fn a_checkout_wins_over_the_embedded_copy() {
        match resolve_with(&crate_dir(), None) {
            Source::Checkout(p) => assert!(p.join(MARKER).is_file(), "{}", p.display()),
            other => panic!("expected the checkout, got {other:?}"),
        }
    }

    /// **Tier 3 — the embedded copy**, which is what an installed binary uses:
    /// no override, and a walk that finds no marker.
    #[test]
    fn a_directory_without_the_marker_is_not_the_stdlib() {
        let tmp = std::env::temp_dir().join("ein-stdlib-none");
        std::fs::create_dir_all(tmp.join("stdlib")).expect("mkdir");
        // A bare `stdlib/` with no marker, and every ancestor of `temp_dir`
        // above it — on a machine where `/tmp` sits inside a checkout the walk
        // would find *that* marker, so the claim is asserted as the rule
        // rather than as the outcome.
        match resolve_with(&tmp, None) {
            Source::Embedded => {}
            Source::Checkout(p) => assert!(
                p.join(MARKER).is_file() && p != tmp.join("stdlib"),
                "the markerless {} was taken for the stdlib",
                tmp.join("stdlib").display()
            ),
            other => panic!("an absent override resolved to {other:?}"),
        }
    }

    /// **Tier 1 — the override**, and the three answers it can give.
    ///
    /// M1e S1e.3.5, `EH-M2`. The override was taken on faith: the *checkout*
    /// walk requires the marker and the highest-precedence source did not, so
    /// a typo surfaced as "module not found at <typo>/algebra.ein" — a
    /// sentence that names the module and never the variable that chose the
    /// directory.
    #[test]
    fn an_override_has_to_prove_itself() {
        let root = crate_dir().join("../../../stdlib");
        assert!(
            resolve_with(&crate_dir(), Some(root.clone()))
                .problem()
                .is_none(),
            "the repo's own stdlib does not satisfy the check"
        );

        let missing = std::env::temp_dir().join("ein-stdlib-does-not-exist");
        let _ = std::fs::remove_dir_all(&missing);
        let why = resolve_with(&crate_dir(), Some(missing))
            .problem()
            .expect("a path that is not a directory is refused");
        assert!(
            why.contains("$EIN_STDLIB") && why.contains("not a directory"),
            "{why}"
        );

        let markerless = std::env::temp_dir().join("ein-stdlib-markerless");
        std::fs::create_dir_all(&markerless).expect("mkdir");
        let why = resolve_with(&crate_dir(), Some(markerless))
            .problem()
            .expect("a directory without the marker is refused");
        assert!(
            why.contains("$EIN_STDLIB") && why.contains(MARKER),
            "the refusal names neither the variable nor what is missing: {why}"
        );

        // The other two tiers can never answer anything but `None`: the walk
        // already tested the marker, and the embedded copy is checked against
        // the manifest by `the_embedded_copy_matches_the_manifest`.
        assert!(Source::Embedded.problem().is_none());
        assert!(Source::Checkout(crate_dir()).problem().is_none());
    }
}
