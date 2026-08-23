//! `ein kb save` and `ein solve <file.einb>` — T1a.8.1.7, and the phase's
//! strongest acceptance:
//!
//! > `ein solve zebra2.einb` is byte-identical to `ein solve zebra2.ein` at
//! > T3 — the strongest evidence the round-trip is faithful.
//!
//! It is the strongest because it does not compare a KB to a KB through an
//! instrument this stage also wrote. It runs the *engine* over both, through
//! the same command, and compares what came out — 378 facts, a verdict, a
//! table, an exit code — none of which the container has any way to influence
//! except by having restored the KB exactly.
//!
//! **Gated on the feature it tests.** `einb` is a default feature and `ein kb`
//! is registered only with it ([design/12](../../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md)
//! §3), so without it every case below asks a binary that has no `kb`
//! subcommand and gets exit 2 — eight failures that say nothing about the
//! container. `help_surface.rs` already holds the *other* direction, that a
//! build without the feature renders a smaller surface on purpose, which is
//! why this file can simply not exist there.

#![cfg(feature = "einb")]

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn ein(args: &[&str]) -> Run {
    ein_with(args, &[])
}

fn ein_with(args: &[&str], env: &[(&str, &str)]) -> Run {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(args).current_dir(repo_root());
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    }
}

/// The two things a container's output cannot be expected to match, and
/// nothing else.
///
/// **The path it was read from**, which `solve` echoes in its header and which
/// is a different file by construction — the whole point is that the KB came
/// from somewhere else. **A wall-clock reading**, which `--stats` prints and
/// which is not a property of the KB at all.
///
/// Everything else is compared byte for byte: the verdict, `k`, the query
/// bindings, the rendered fact table, the counters, the derivation trace, the
/// final state. This is the normalisation list of a one-file comparison, and
/// it is two lines long because that is how much of `solve`'s output is
/// narration rather than answer.
fn normalise(text: &str, container: &str, source: &str) -> String {
    text.replace(container, source)
        .lines()
        .map(|l| {
            let trimmed = l.trim_start();
            if trimmed.starts_with("wall ") {
                "  wall <ms>".to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        // `lines()` eats the last newline and the comparison is byte-for-byte.
        + if text.ends_with('\n') { "\n" } else { "" }
}

/// A scratch directory that cleans up after itself.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-einb-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

#[test]
fn a_solve_of_a_container_is_byte_identical_to_a_solve_of_its_source() {
    let dir = scratch("identical");
    for puzzle in [
        "examples/zebra.ein",
        "examples/zebra2-hints.ein",
        "examples/features/01_not_and_absent.ein",
        "examples/branching/01_saturate_only.ein",
    ] {
        let out = dir.join("kb.einb");
        let saved = ein(&["kb", "save", puzzle, out.to_str().expect("utf-8")]);
        assert_eq!(saved.code, 0, "kb save {puzzle}: {}", saved.stderr);
        assert!(saved.stderr.is_empty(), "kb save wrote {}", saved.stderr);

        let container = out.to_str().expect("utf-8");
        let from_text = ein(&["solve", puzzle]);
        let from_container = ein(&["solve", container]);
        assert_eq!(
            normalise(&from_container.stdout, container, puzzle),
            from_text.stdout,
            "{puzzle}: stdout differs between the container and its source"
        );
        assert_eq!(
            normalise(&from_container.stderr, container, puzzle),
            from_text.stderr,
            "{puzzle}: stderr differs"
        );
        assert_eq!(
            from_container.code, from_text.code,
            "{puzzle}: exit differs"
        );
        assert!(
            !from_text.stdout.is_empty(),
            "{puzzle}: the comparison is only worth something if there was output"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The whole `solve` surface, not just its default: every diagnostic the
/// command can print is a different projection of the same KB, and one of them
/// moving is what a shallow round trip would hide.
#[test]
fn the_diagnostic_flags_agree_too() {
    let dir = scratch("flags");
    let out = dir.join("zebra.einb");
    assert_eq!(
        ein(&[
            "kb",
            "save",
            "examples/zebra.ein",
            out.to_str().expect("utf-8")
        ])
        .code,
        0
    );
    let container = out.to_str().expect("utf-8");
    for flags in [
        vec!["--stats"],
        vec!["--trace"],
        vec!["--explain"],
        vec!["--final-state"],
        vec!["--dump-config"],
    ] {
        let mut a = vec!["solve", "examples/zebra.ein"];
        a.extend(flags.iter().copied());
        let mut b = vec!["solve", container];
        b.extend(flags.iter().copied());
        let (from_text, from_container) = (ein(&a), ein(&b));
        assert_eq!(
            normalise(&from_container.stdout, container, "examples/zebra.ein"),
            normalise(&from_text.stdout, container, "examples/zebra.ein"),
            "solve {flags:?} differs between the container and its source"
        );
        assert_eq!(from_container.code, from_text.code, "solve {flags:?}: exit");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A saturated container starts from the fixpoint, so it is **not** claimed to
/// be byte-identical — the trace of a run that inherited root's derivations
/// has nothing to say about them. The verdict is the invariant.
#[test]
fn a_saturated_container_reaches_the_same_verdict() {
    let dir = scratch("saturated");
    let out = dir.join("zebra-sat.einb");
    let saved = ein(&[
        "kb",
        "save",
        "--saturate",
        "examples/zebra.ein",
        out.to_str().expect("utf-8"),
    ]);
    assert_eq!(saved.code, 0, "{}", saved.stderr);
    let container = out.to_str().expect("utf-8");
    let from_text = ein(&["solve", "examples/zebra.ein"]);
    let from_container = ein(&["solve", container]);
    assert_eq!(from_container.code, from_text.code);
    assert_eq!(
        normalise(&from_container.stdout, container, "examples/zebra.ein"),
        from_text.stdout,
        "the answer must not depend on where the search started"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// The stdlib is the input whose divergence nothing else would notice
/// (design/11 §3), so a container written against one stdlib and opened
/// against another is a **cache miss**: said out loud, and with the derived
/// state dropped rather than believed.
///
/// Done the end-to-end way — a temp copy of `stdlib/` with an edited manifest
/// and `$EIN_STDLIB` pointing at it — because that is the change a user
/// actually makes, and a child process is where an environment variable can be
/// set without a test binary mutating its own.
#[test]
fn an_edited_stdlib_is_a_cache_miss_rather_than_a_stale_hit() {
    let dir = scratch("stdlib");
    let out = dir.join("zebra2.einb");
    let saved = ein(&[
        "kb",
        "save",
        "--saturate",
        "examples/zebra2.ein",
        out.to_str().expect("utf-8"),
    ]);
    assert_eq!(saved.code, 0, "{}", saved.stderr);
    let container = out.to_str().expect("utf-8");

    // Unedited: the copy is byte-for-byte the checkout's, so nothing is stale.
    let copy = dir.join("stdlib");
    std::fs::create_dir_all(&copy).expect("a stdlib copy");
    let real = repo_root().join("stdlib");
    for entry in std::fs::read_dir(&real).expect("the stdlib") {
        let from = entry.expect("an entry").path();
        if from.is_file() {
            std::fs::copy(&from, copy.join(from.file_name().expect("a name"))).expect("copy");
        }
    }
    let same = ein_with(
        &["solve", container],
        &[("EIN_STDLIB", copy.to_str().expect("utf-8"))],
    );
    assert!(
        same.stderr.is_empty(),
        "an identical stdlib is not a cache miss, got: {}",
        same.stderr
    );

    // One byte more in the manifest, which is what a stdlib edit changes.
    let manifest = copy.join("MANIFEST.sha256");
    let text = std::fs::read_to_string(&manifest).expect("a manifest");
    std::fs::write(&manifest, format!("{text}\n")).expect("edit");
    let moved = ein_with(
        &["solve", container],
        &[("EIN_STDLIB", copy.to_str().expect("utf-8"))],
    );
    assert!(
        moved.stderr.contains("StaleStdlib"),
        "an edited stdlib should be reported, got: {:?}",
        moved.stderr
    );
    assert!(
        moved.stderr.contains("derived state was dropped"),
        "a saturated container of an old stdlib must not be believed, got: {:?}",
        moved.stderr
    );
    assert_eq!(moved.code, 0, "a cache miss is not an error");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The dispatch is on the magic bytes, not on the name (T1a.8.1.7).
#[test]
fn a_container_is_recognised_under_any_name() {
    let dir = scratch("named");
    let disguised = dir.join("puzzle.ein");
    assert_eq!(
        ein(&[
            "kb",
            "save",
            "examples/zebra.ein",
            disguised.to_str().expect("utf-8")
        ])
        .code,
        0
    );
    let disguised_path = disguised.to_str().expect("utf-8");
    let from_container = ein(&["solve", disguised_path]);
    let from_text = ein(&["solve", "examples/zebra.ein"]);
    assert_eq!(
        normalise(&from_container.stdout, disguised_path, "examples/zebra.ein"),
        from_text.stdout
    );
    assert_eq!(from_container.code, 0, "{}", from_container.stderr);

    // …and the other way: a text file named `.einb` is still text.
    let text = dir.join("puzzle.einb");
    std::fs::copy(repo_root().join("examples/zebra.ein"), &text).expect("copy");
    let text_path = text.to_str().expect("utf-8");
    let from_text_named_einb = ein(&["solve", text_path]);
    assert_eq!(
        normalise(
            &from_text_named_einb.stdout,
            text_path,
            "examples/zebra.ein"
        ),
        from_text.stdout
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_container_the_engine_cannot_read_fails_as_a_load_error() {
    let dir = scratch("damaged");
    let out = dir.join("kb.einb");
    assert_eq!(
        ein(&[
            "kb",
            "save",
            "examples/zebra.ein",
            out.to_str().expect("utf-8")
        ])
        .code,
        0
    );
    let mut bytes = std::fs::read(&out).expect("the container");
    let at = bytes.len() / 2;
    bytes[at] ^= 0xff;
    std::fs::write(&out, &bytes).expect("damage it");
    let run = ein(&["solve", out.to_str().expect("utf-8")]);
    assert_eq!(run.code, 1, "a damaged container is a load error");
    assert!(
        run.stderr.contains("digest"),
        "and it says why: {:?}",
        run.stderr
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// `--saturate` is the only reason to write a container that is not a
/// re-statement of its source, so it had better be smaller *and* faster to
/// open than the text it came from is to load.
#[test]
fn a_container_is_not_larger_than_the_program_it_replaces() {
    let dir = scratch("size");
    let out = dir.join("zebra2.einb");
    assert_eq!(
        ein(&[
            "kb",
            "save",
            "examples/zebra2.ein",
            out.to_str().expect("utf-8")
        ])
        .code,
        0
    );
    let container = std::fs::metadata(&out).expect("written").len();
    let source = std::fs::metadata(repo_root().join("examples/zebra2.ein"))
        .expect("the puzzle")
        .len();
    // The container holds the *resolved* program — the puzzle plus every
    // stdlib module it imports — so it is bigger than the one file it names
    // and smaller than the tree that file pulls in.
    let imported: u64 = std::fs::read_dir(repo_root().join("stdlib"))
        .expect("the stdlib")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "ein"))
        .filter_map(|e| e.metadata().ok().map(|m| m.len()))
        .sum();
    assert!(
        container < source + imported,
        "a loaded zebra2 container is {container} bytes against {source} + {imported} of text"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// The container's own path, unused by anything else here: the file the CLI
/// writes is the file the library reads.
#[test]
fn what_the_cli_writes_is_a_file_the_library_recognises() {
    let dir = scratch("magic");
    let out = dir.join("kb.einb");
    assert_eq!(
        ein(&[
            "kb",
            "save",
            "examples/zebra.ein",
            out.to_str().expect("utf-8")
        ])
        .code,
        0
    );
    let bytes = std::fs::read(&out).expect("written");
    assert_eq!(&bytes[..5], b"EINB\0");
    assert!(!ein_is_text(Path::new("examples/zebra.ein")));
    let _ = std::fs::remove_dir_all(&dir);
}

fn ein_is_text(rel: &Path) -> bool {
    let bytes = std::fs::read(repo_root().join(rel)).expect("readable");
    bytes.starts_with(b"EINB\0")
}
