//! `ein kb save` and `ein solve <file.einb>` — T1a.8.1.7, and the phase's
//! strongest acceptance:
//!
//! > `ein solve zebra2.einb` is byte-identical to `ein solve zebra2.ein` —
//! > the strongest evidence the round-trip is faithful.
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

use ein_corpus::Scratch;

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
fn scratch(name: &str) -> Scratch {
    Scratch::new(&format!("ein-einb-{name}"))
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
}

/// **`--dump-answer` on a container writes the directory its source writes**
/// — M1f S1f.5.12 (T1f.5.12.5), W43's fix. Until then `ein solve X.einb
/// --dump-answer DIR` was refused before the run, exit 2, *a .einb container
/// carries no program source*, where the same run on the `.ein` answered and
/// every other command read the program the container stores. The same files,
/// each the source's byte for byte but for the path it names; on a program
/// that imports, a state file's program is the one the container stores,
/// every declaration its imports brought (Q-M1f.27), and its header and its
/// state are the source's.
#[test]
fn the_answer_directory_of_a_container_is_its_sources() {
    const STATE: &str = ";; ── the state ──";
    let dir = scratch("dump-answer");
    for (puzzle, imports) in [
        (
            "examples/ein-bugs/wrong-answers/einb-dump-answer.ein",
            false,
        ),
        ("examples/branching/01_saturate_only.ein", false),
        ("examples/lattice/02_genuine_3set_death.ein", false),
        ("examples/zebra2-hints.ein", true),
    ] {
        let einb = dir.join("kb.einb");
        let container = einb.to_str().expect("utf-8");
        let saved = ein(&["kb", "save", puzzle, container]);
        assert_eq!(saved.code, 0, "kb save {puzzle}: {}", saved.stderr);
        let (a, b) = (dir.join("from-text"), dir.join("from-container"));
        let _ = std::fs::remove_dir_all(&a);
        let _ = std::fs::remove_dir_all(&b);
        let from_text = ein(&["solve", puzzle, "--dump-answer", a.to_str().expect("utf-8")]);
        let from_container = ein(&[
            "solve",
            container,
            "--dump-answer",
            b.to_str().expect("utf-8"),
        ]);
        assert_eq!(
            (from_container.code, from_container.stderr.as_str()),
            (from_text.code, from_text.stderr.as_str()),
            "{puzzle}: the container's exit and stderr"
        );
        assert_eq!(
            normalise(&from_container.stdout, container, puzzle),
            from_text.stdout,
            "{puzzle}: stdout"
        );
        let files = |root: &Path| -> Vec<PathBuf> {
            let mut out = Vec::new();
            let mut stack = vec![root.to_path_buf()];
            while let Some(d) = stack.pop() {
                for e in std::fs::read_dir(&d).expect("a directory") {
                    let p = e.expect("an entry").path();
                    if p.is_dir() {
                        stack.push(p);
                    } else {
                        out.push(p.strip_prefix(root).expect("under root").to_path_buf());
                    }
                }
            }
            out.sort();
            out
        };
        let written = files(&a);
        assert_eq!(files(&b), written, "{puzzle}: the files written");
        assert!(
            written.iter().any(|f| f.starts_with("models")) && written.len() >= 5,
            "{puzzle}: {written:?}"
        );
        for f in &written {
            let want = std::fs::read_to_string(a.join(f)).expect("utf-8");
            let got = normalise(
                &std::fs::read_to_string(b.join(f)).expect("utf-8"),
                container,
                puzzle,
            );
            if !(imports && f.extension().is_some_and(|e| e == "ein")) {
                assert_eq!(got, want, "{puzzle}: {}", f.display());
                continue;
            }
            // A state file of a program that imports: its header and its
            // state are the source's, and its program the container's.
            let header = |t: &str| -> String {
                t.lines()
                    .take_while(|l| l.starts_with(";;;"))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            let state = |t: &str| t.split_once(STATE).map(|(_, s)| s.to_string());
            assert_eq!(
                header(&got),
                header(&want),
                "{puzzle}: {}'s header",
                f.display()
            );
            assert!(
                state(&want).is_some(),
                "{puzzle}: {} holds a state",
                f.display()
            );
            assert_eq!(
                state(&got),
                state(&want),
                "{puzzle}: {}'s state",
                f.display()
            );
        }
    }
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
    // zebra2 is described in part at the default budget, and its stderr says
    // so in a `note:` — so what is asked of it is that nothing *else* is
    // there.
    assert!(
        same.stderr.lines().all(|l| l.starts_with("note: ")),
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
    assert_eq!(
        &bytes[..ein_einb::header::MAGIC.len()],
        ein_einb::header::MAGIC
    );
    assert!(!looks_like_einb(Path::new("examples/zebra.ein")));
}

/// Named for what it returns, which the previous name (`ein_is_text`) was the
/// negation of — M1e S1e.4.4, `EH-L2`. It sniffed five bytes too.
fn looks_like_einb(rel: &Path) -> bool {
    let bytes = std::fs::read(repo_root().join(rel)).expect("readable");
    bytes.starts_with(&ein_einb::header::MAGIC)
}

/// **The two magic constants agree** — M1e S1e.4.4, `EH-L2`.
///
/// `ein-ir` carries its own copy (`ein_ir::read::EINB_MAGIC`, `ein-cli`'s
/// `common::EINB_MAGIC` until M1f S1f.5.22 moved the read there) because
/// `ein-einb` is an optional dependency of `ein-cli` and a
/// `--no-default-features` build must still recognise a container in order
/// to refuse it. That is the state
/// [`AR-M1`] permits for a pair too small to share a crate — *compared by a
/// test* — and this is the test. It can only run in the default build, which
/// is the one where both constants exist; the **behaviour** half, which has to
/// run in both, is
/// `cli_semantics::a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build`.
///
/// The copy sniffed **five** of the eight bytes until this stage.
///
/// [`AR-M1`]: ../../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium
#[test]
fn the_two_magic_constants_agree() {
    // The reader's copy is public since M1f S1f.5.22, `ein_ir::read`'s, so it
    // is compared directly; and through the one thing both builds ship, a
    // container the writer just wrote, opened by the reader through the CLI's
    // own sniff.
    assert_eq!(ein_einb::header::MAGIC, ein_ir::read::EINB_MAGIC);
    assert!(ein_ir::read::looks_like_einb(&ein_einb::header::MAGIC));
    assert!(!ein_ir::read::looks_like_einb(b"EINB\0xyz"));
    assert_eq!(ein_einb::header::MAGIC, *b"EINB\0\0\0\0");
    assert_eq!(ein_einb::header::MAGIC.len(), 8);
    assert!(ein_einb::is_einb(&ein_einb::header::MAGIC));
    // Five bytes are not eight, which is the whole finding.
    assert!(!ein_einb::is_einb(b"EINB\0xyz"));
}

/// **A container saved again is the container** (M1f S1f.5.28's item 9).
/// `ein kb save` of a `.einb` writes the bytes the first save wrote, the
/// loaded state and the saturated one alike, however many times, and each
/// answers as the program it was saved from. Until S1f.5.28 each save of a
/// save also wrote the provenance records the open had pushed and nothing
/// referenced, the loader's for each given fact: the witness's container went
/// 1 248 → 1 288 → 1 328 bytes, and a saturated `zebra2.einb` saved once more
/// passed design/10 §6's 64 KB, with the same answers.
#[test]
fn a_container_saved_again_is_the_container() {
    let dir = scratch("saved-again");
    for (source, flags) in [
        (
            "examples/ein-bugs/wrong-answers/einb-every-command.ein",
            &[][..],
        ),
        ("examples/zebra2.ein", &["--saturate"][..]),
    ] {
        let path = |n: usize| dir.join(format!("{n}.einb")).to_string_lossy().to_string();
        let mut args = vec!["kb", "save"];
        args.extend_from_slice(flags);
        let first = path(0);
        args.extend([source, first.as_str()]);
        let r = ein(&args);
        assert_eq!(r.code, 0, "{source}: {}", r.stderr);
        for n in 1..=2 {
            let (from, to) = (path(n - 1), path(n));
            let r = ein(&["kb", "save", &from, &to]);
            assert_eq!(r.code, 0, "{source}, save {n}: {}", r.stderr);
            let (a, b) = (
                std::fs::read(&from).expect("read"),
                std::fs::read(&to).expect("read"),
            );
            assert!(
                a == b,
                "{source}: save {n} of the container is {} bytes where the one it was saved \
                 from is {}",
                b.len(),
                a.len()
            );
        }
        let from_text = ein(&["solve", source]);
        let last = path(2);
        let from_container = ein(&["solve", &last]);
        assert_eq!(from_container.code, from_text.code, "{source}");
        assert_eq!(
            normalise(&from_container.stdout, &last, source),
            from_text.stdout,
            "{source}: the container saved again answers otherwise"
        );
    }
}
