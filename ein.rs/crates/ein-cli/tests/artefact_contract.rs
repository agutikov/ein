//! M1e [S1e.3.5](../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.5_error_handling.md)
//! — **what the five artefact flags do when the file cannot be written.**
//!
//! `EH-M1` is one sentence: a failed `--events` open, a failed
//! `--json-summary` write and a failed `--trace` write each print a line and
//! the run exits as if the artefact existed. The stage's job was to *rule* on
//! that rather than call it arguably fine, and the ruling is
//! [`defined_behaviour.md` § 4.4](../../../../docs/kernel/defined_behaviour.md):
//! the additive four leave the exit code alone, `--dump-states` does not, and
//! every one of them says the same sentence.
//!
//! Probing it turned up two things the finding did not name, and both are
//! fixed here rather than documented:
//!
//! - **four flags, four diagnostics.** Three printed a bare OS error — *Is a
//!   directory (os error 21)*, on a run that may carry three artefact flags —
//!   one named the path but not the flag, and one carried its failure in the
//!   exit code alone. If stderr is the whole of what a consumer gets, it has
//!   to say which artefact was lost and where it was going.
//! - **an empty path was accepted by all five**, and `--dump-states ""`
//!   *succeeded*: `create_dir_all("")` is `Ok`, so four files landed in the
//!   caller's current directory. It is a usage error now, refused by the
//!   value parser at exit 2.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("crates/<crate>/ is three below the root")
        .to_path_buf()
}

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// A directory this process owns, made read-only so a write into it fails for
/// a reason that is nothing to do with the path being malformed.
struct ReadOnly(PathBuf);

impl ReadOnly {
    fn new(tag: &str) -> ReadOnly {
        let dir = std::env::temp_dir().join(format!("ein-artefact-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let mut perms = std::fs::metadata(&dir).expect("stat").permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            perms.set_mode(0o500);
        }
        #[cfg(not(unix))]
        perms.set_readonly(true);
        std::fs::set_permissions(&dir, perms).expect("chmod");
        ReadOnly(dir)
    }

    /// A path *inside* the read-only directory — writable by nobody.
    fn at(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }
}

impl Drop for ReadOnly {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(m) = std::fs::metadata(&self.0) {
                let mut p = m.permissions();
                p.set_mode(0o700);
                let _ = std::fs::set_permissions(&self.0, p);
            }
        }
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const FILE: &str = "examples/features/10_expect.ein";

/// Every artefact flag, and which subcommand carries it.
const ARTEFACTS: [(&str, &str); 6] = [
    ("solve", "events"),
    ("solve", "json-summary"),
    ("solve", "trace"),
    ("solve", "dump-states"),
    ("test", "json-report"),
    ("test", "json-summary"),
];

/// **One sentence, whichever artefact could not be written.**
///
/// `error: --<flag> <path>: <os error>` — the flag, so a run carrying three of
/// them says which; the path, so a shell that expanded a variable to the wrong
/// place can see it; and the OS error, which is the only part that was there
/// before.
#[test]
fn every_artefact_failure_names_its_flag_and_its_path() {
    let ro = ReadOnly::new("shape");
    for (cmd, flag) in ARTEFACTS {
        let path = ro.at("out");
        let r = ein(&[cmd, FILE, &format!("--{flag}"), &path]);
        let want = format!("error: --{flag} {path}: ");
        assert!(
            r.err.contains(&want),
            "{cmd} --{flag}: stderr does not carry {want:?}\n{}",
            r.err
        );
        assert!(
            r.err.contains("Permission denied") || r.err.contains("os error"),
            "{cmd} --{flag}: the OS error is gone from {:?}",
            r.err
        );
    }
}

/// **The additive four leave the exit code alone; `--dump-states` does not.**
///
/// Two arms and the split is not arbitrary. Four flags say *additive: stdout,
/// stderr and the exit code are unchanged* in their own `--help`, and a failed
/// write does not make that false: the answer on stdout is correct and
/// complete, and the artefact is paperwork. `--dump-states` claims no
/// additivity, opens a **directory before the search** rather than writing a
/// file after it, and so its failure means the run cannot do what it was asked
/// — which is worth an exit code, and is the one signal the family has.
///
/// Whether the additive arm should have a code of its own is
/// [Q-M1e.22](../../../../plans/m1e_review_processing/open_questions.md),
/// filed with `TE-M4`'s exit-2 overload because the two are one conversation.
#[test]
fn a_failed_artefact_write_leaves_the_exit_code_alone_except_for_dump_states() {
    let ro = ReadOnly::new("exit");
    for (cmd, flag) in ARTEFACTS {
        let r = ein(&[cmd, FILE, &format!("--{flag}"), &ro.at("out")]);
        let want = i32::from(flag == "dump-states");
        assert_eq!(
            r.code, want,
            "{cmd} --{flag} exited {} and the contract says {want}\n{}",
            r.code, r.err
        );
    }
    // …and the additive arm really did answer: the run's own output is there,
    // which is what makes "the artefact is paperwork" true rather than a way
    // of not noticing.
    let r = ein(&["solve", FILE, "--json-summary", &ro.at("out")]);
    assert!(r.out.contains("solutions (k)"), "{}", r.out);
}

/// **An empty path is a usage error**, refused before anything runs.
///
/// It reached all five options and three of them then failed with a bare *No
/// such file or directory*. `--dump-states ""` did not fail at all:
/// `create_dir_all("")` is `Ok`, `""/00_timeline.jsonl` is a relative path,
/// and the run dropped `00_root_initial.ein`, `00_timeline.jsonl`,
/// `summary.json` and `layers/` into whatever directory the caller happened to
/// be in. Refused at the value parser, for `--solutions 0`'s reason: the CLI
/// is where usage errors are answered.
#[test]
fn an_empty_artefact_path_is_refused() {
    for (cmd, flag) in ARTEFACTS {
        let r = ein(&[cmd, FILE, &format!("--{flag}"), ""]);
        assert_eq!(
            r.code, 2,
            "{cmd} --{flag} accepted an empty path\n{}",
            r.err
        );
        assert!(
            r.err.contains("expected a path, got an empty string")
                && r.err.contains(&format!("--{flag}")),
            "{cmd} --{flag}: {:?}",
            r.err
        );
        assert!(r.out.is_empty(), "{cmd} --{flag} ran anyway:\n{}", r.out);
    }
    // The one that used to succeed, checked where it used to succeed: nothing
    // is written into the working directory.
    let before: Vec<String> = std::fs::read_dir(repo_root())
        .expect("readdir")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    ein(&["solve", FILE, "--dump-states", ""]);
    for name in ["00_timeline.jsonl", "00_root_initial.ein", "summary.json"] {
        assert!(
            !before.contains(&name.to_string()) && !repo_root().join(name).exists(),
            "an empty --dump-states wrote {name} into the working directory"
        );
    }
}
