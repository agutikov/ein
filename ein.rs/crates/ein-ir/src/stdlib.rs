//! Where `std.*` comes from — the Rust half of the shared stdlib
//! (M1a S1a.0.3, [design/11](../../../../plans/m1a_rust/design/11_shared_assets.md)).
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
//! "module not found at <path>", which names the path it looked at — the
//! message ein.py produces, and therefore the one to produce.

use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};

/// Identifies a directory as the stdlib. Content is checked by
/// `utils/stdlib_manifest.py`; *presence* is what the walk tests, because a
/// directory called `stdlib/` proves nothing on its own.
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
    /// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md)).
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
    if let Some(over) = std::env::var_os("EIN_STDLIB") {
        return Source::Override(PathBuf::from(over));
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

    #[test]
    fn a_checkout_wins_over_the_embedded_copy() {
        // Guarded on the env var being unset: `resolve` honours an override
        // first, and the harness sets one.
        if std::env::var_os("EIN_STDLIB").is_some() {
            return;
        }
        match resolve(&crate_dir()) {
            Source::Checkout(p) => assert!(p.join(MARKER).is_file(), "{}", p.display()),
            other => panic!("expected the checkout, got {other:?}"),
        }
    }

    #[test]
    fn a_directory_without_the_marker_is_not_the_stdlib() {
        // Walking up from a temp dir finds no marker, so the embedded copy is
        // the answer — which is exactly what an installed binary should get.
        let tmp = std::env::temp_dir().join("ein-stdlib-none");
        std::fs::create_dir_all(tmp.join("stdlib")).expect("mkdir");
        if std::env::var_os("EIN_STDLIB").is_some() {
            return;
        }
        // `temp_dir` may itself sit under a checkout on some systems; only
        // assert the marker rule, which is the claim being made.
        if let Source::Checkout(p) = resolve(&tmp) {
            assert!(p.join(MARKER).is_file());
        }
    }
}
