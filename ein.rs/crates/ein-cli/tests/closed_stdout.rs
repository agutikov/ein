//! **A closed stdout ends `ein` by `SIGPIPE`, as it ends `cat`** — M1f
//! S1f.5.25, on the user's ruling of 2026-11-10 on the close review's finding
//! 5 and Q-M1f.68, (b): *"why not handle sigpipe properly?"*.
//!
//! Rust's runtime sets `SIGPIPE` to *ignored* before `main` runs, so a write
//! to a pipe whose reader had gone failed with `EPIPE`, and every stdout write
//! in `ein-cli` is a `print!` or a `println!`, which panic on a failed write:
//! `ein saturate … | head` printed *failed printing to stdout: Broken pipe (os
//! error 32)* and exited 101 (register rows W75). Since S1f.5.25 the binary's
//! `main` restores the default disposition first (`ein-cli/src/main.rs`'s
//! `restore_sigpipe`), so the kernel ends the process at that write, with
//! nothing on stderr, and a shell reports 141, 128 + 13.
//!
//! Every subcommand and every option that writes to stdout, three ways
//! (Q-M1f.97):
//!
//! - **read in full**: each exits 0, writes stdout and nothing on stderr, so
//!   the two readers below mean what they say, and a run that stops writing
//!   to stdout, or starts writing stderr, fails here first;
//! - **its reader closed before the run starts**: every write the run makes
//!   meets no reader, whatever the timing, so each ends by `SIGPIPE` at its
//!   first write, with an empty stderr, deterministically;
//! - **its reader closing after the first line**, as `head -1` does, the pipe
//!   one page where the kernel lets a pipe shrink: a run whose stdout is more
//!   than the first line and what the pipe holds must write after the reader
//!   has gone, so it ends by `SIGPIPE`; one whose stdout fits may have written
//!   all of it before the reader closed, and exits 0, as `cat` does. Either
//!   way never 101 and never a word on stderr.
//!
//! An option that only changes what a view draws (`--rule-mode`,
//! `--colour-by`, `--since`, …) prints through its view's one write and is
//! not listed; an option that adds a write is. Writing to stdout through an
//! artefact flag means naming `/dev/stdout`, `-` being a file of that name.
//! `kb save` writes nothing to stdout and is the control: a closed stdout does
//! not end it.
//!
//! ```text
//! cargo test -p ein-cli --test closed_stdout
//! ```

#![cfg(unix)]

use std::io::Read;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

use ein_corpus::{Scratch, repo_root};

/// The witness of every run but `ein test`'s: nine candidates, four rules, R
/// five states, and no claim, so no question prints a `note:` (W75's).
const W: &str = "examples/branching/05_mini_zebra.ein";
/// `ein test`'s: a claim that holds, so the run exits 0 (W75/test's).
const T: &str = "examples/features/11_expect_ambiguity.ein";
/// SIGPIPE, on every Unix the binary ships for.
const SIGPIPE: i32 = 13;

/// Every subcommand and every option that writes to stdout, by name.
fn runs(dir: &Path) -> Vec<(String, Vec<String>)> {
    let models = dir.join("two.models");
    std::fs::write(
        &models,
        "closes co-located\nmodel\n  (co-located Alice Cat) (co-located Alice Tea)\n",
    )
    .expect("the models file");
    let region = dir.join("region");
    let dumped = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["oracle", W, "--dump"])
        .arg(&region)
        .current_dir(repo_root())
        .output()
        .expect("ein oracle --dump");
    assert!(dumped.status.success(), "ein oracle {W} --dump failed");
    let (m, r) = (models.display().to_string(), region.display().to_string());
    let mut v: Vec<(&str, Vec<&str>)> = vec![
        ("--version", vec!["--version"]),
        ("-V", vec!["-V"]),
        ("--help", vec!["--help"]),
        ("-h", vec!["-h"]),
        ("solve --help", vec!["solve", "--help"]),
        ("saturate --help", vec!["saturate", "--help"]),
        ("oracle --help", vec!["oracle", "--help"]),
        ("test --help", vec!["test", "--help"]),
        ("render --help", vec!["render", "--help"]),
        ("render rules --help", vec!["render", "rules", "--help"]),
        ("render rule --help", vec!["render", "rule", "--help"]),
        (
            "render constraints --help",
            vec!["render", "constraints", "--help"],
        ),
        ("render program --help", vec!["render", "program", "--help"]),
        ("render kb --help", vec!["render", "kb", "--help"]),
        ("render region --help", vec!["render", "region", "--help"]),
        ("render lattice --help", vec!["render", "lattice", "--help"]),
        ("saturate", vec!["saturate", W]),
        ("saturate --dump", vec!["saturate", W, "--dump"]),
        (
            "saturate --max-steps",
            vec!["saturate", W, "--max-steps", "1"],
        ),
        (
            "saturate --progress-every",
            vec!["saturate", W, "--progress-every", "1"],
        ),
        (
            "saturate --events",
            vec!["saturate", W, "--events", "/dev/stdout"],
        ),
        ("solve", vec!["solve", W]),
        ("solve --minimal", vec!["solve", W, "--minimal"]),
        ("solve --maximal", vec!["solve", W, "--maximal"]),
        ("solve --single", vec!["solve", W, "--single"]),
        ("solve --count", vec!["solve", W, "--count"]),
        ("solve --monotone", vec!["solve", W, "--monotone"]),
        ("solve -n", vec!["solve", W, "-n", "2"]),
        ("solve -c", vec!["solve", W, "-c"]),
        ("solve -H", vec!["solve", W, "-H"]),
        ("solve -t", vec!["solve", W, "-t"]),
        ("solve -s", vec!["solve", W, "-s"]),
        ("solve -y", vec!["solve", W, "-y"]),
        (
            "solve --models key",
            vec!["solve", W, "--maximal", "--models", "key"],
        ),
        (
            "solve --models grid",
            vec!["solve", W, "--maximal", "--models", "grid"],
        ),
        (
            "solve --diff-models",
            vec!["solve", W, "--maximal", "--diff-models", &m],
        ),
        ("solve -p", vec!["solve", W, "--maximal", "-p"]),
        ("solve -P", vec!["solve", W, "--maximal", "-P"]),
        ("solve -f", vec!["solve", W, "--maximal", "-f"]),
        (
            "solve --events",
            vec!["solve", W, "--events", "/dev/stdout"],
        ),
        (
            "solve --json-summary",
            vec!["solve", W, "--json-summary", "/dev/stdout"],
        ),
        ("solve --trace", vec!["solve", W, "--trace", "/dev/stdout"]),
        ("oracle", vec!["oracle", W]),
        ("oracle -s", vec!["oracle", W, "-s"]),
        ("oracle --models key", vec!["oracle", W, "--models", "key"]),
        (
            "oracle --models grid",
            vec!["oracle", W, "--models", "grid"],
        ),
        ("oracle -p", vec!["oracle", W, "-p"]),
        ("oracle -P", vec!["oracle", W, "-P"]),
        (
            "oracle --json-summary",
            vec!["oracle", W, "--json-summary", "/dev/stdout"],
        ),
        ("test", vec!["test", T]),
        ("test -v", vec!["test", T, "-v"]),
        ("test -q", vec!["test", T, "-q"]),
        (
            "test --json-report",
            vec!["test", T, "--json-report", "/dev/stdout"],
        ),
        (
            "test --json-summary",
            vec!["test", T, "--json-summary", "/dev/stdout"],
        ),
        ("test --events", vec!["test", T, "--events", "/dev/stdout"]),
        ("test a directory", vec!["test", "tests/stdlib/closure"]),
        ("render rules", vec!["render", "rules", W]),
        (
            "render rule",
            vec!["render", "rule", "--name", "functional", W],
        ),
        ("render constraints", vec!["render", "constraints", W]),
        ("render program", vec!["render", "program", W]),
        (
            "render program --levi",
            vec!["render", "program", "--levi", W],
        ),
        ("render kb", vec!["render", "kb", W]),
        ("render lattice", vec!["render", "lattice", W]),
        (
            "render lattice --view full",
            vec!["render", "lattice", "--view", "full", W],
        ),
        ("render region", vec!["render", "region", &r]),
        (
            "render region --view table",
            vec!["render", "region", "--view", "table", &r],
        ),
    ];
    if cfg!(feature = "einb") {
        v.push(("kb --help", vec!["kb", "--help"]));
        v.push(("kb save --help", vec!["kb", "save", "--help"]));
    }
    v.into_iter()
        .map(|(n, a)| (n.to_string(), a.into_iter().map(str::to_string).collect()))
        .collect()
}

/// A run read in full: its status, stdout and stderr.
fn full(argv: &[String]) -> (ExitStatus, Vec<u8>, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(argv)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|e| panic!("ein {argv:?}: {e}"));
    (
        out.status,
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// A run whose stdout is a pipe whose reader was dropped before it was
/// spawned: its status and stderr.
fn closed_before(argv: &[String]) -> (ExitStatus, String) {
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(argv)
        .current_dir(repo_root())
        .stdout(Stdio::from(writer));
    let out = cmd.output().unwrap_or_else(|e| panic!("ein {argv:?}: {e}"));
    (
        out.status,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Shrink a pipe to one page where the kernel lets it, and say what it holds
/// then; `None` where its size cannot be read.
#[cfg(target_os = "linux")]
fn one_page(fd: &impl std::os::fd::AsRawFd) -> Option<usize> {
    let fd = fd.as_raw_fd();
    // SAFETY: `fd` is a pipe this test owns and holds open across both calls;
    // `F_SETPIPE_SZ` and `F_GETPIPE_SZ` take an int and touch no memory of
    // ours.
    let held = unsafe {
        libc::fcntl(fd, libc::F_SETPIPE_SZ, 4096);
        libc::fcntl(fd, libc::F_GETPIPE_SZ)
    };
    usize::try_from(held).ok()
}

#[cfg(not(target_os = "linux"))]
fn one_page(_: &impl std::os::fd::AsRawFd) -> Option<usize> {
    None
}

/// A run whose stdout is read up to and including its first newline, then
/// closed, as `head -1` does: its status, stderr, the bytes read, and what
/// the pipe holds.
fn first_line(argv: &[String]) -> (ExitStatus, String, usize, Option<usize>) {
    let (mut reader, writer) = std::io::pipe().expect("a pipe");
    let held = one_page(&writer);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(argv)
        .current_dir(repo_root())
        .stdout(Stdio::from(writer))
        .stderr(Stdio::piped());
    let child = cmd.spawn().unwrap_or_else(|e| panic!("ein {argv:?}: {e}"));
    // The command holds the pipe's writer; dropped, the run holds the only
    // one, so a run that ends before a newline is an end of file here.
    drop(cmd);
    let mut read = 0usize;
    let mut b = [0u8; 1];
    loop {
        match reader.read(&mut b) {
            Ok(0) => break,
            Ok(_) => {
                read += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => panic!("ein {argv:?}: reading stdout: {e}"),
        }
    }
    drop(reader);
    let out = child
        .wait_with_output()
        .unwrap_or_else(|e| panic!("ein {argv:?}: {e}"));
    (
        out.status,
        String::from_utf8_lossy(&out.stderr).into_owned(),
        read,
        held,
    )
}

fn ended(s: ExitStatus) -> String {
    match (s.signal(), s.code()) {
        (Some(n), _) => format!("signal {n}"),
        (None, Some(c)) => format!("exit {c}"),
        (None, None) => format!("{s:?}"),
    }
}

/// **Every run listed writes stdout, exits 0 and writes nothing on stderr**,
/// read in full: what the two readers below hold each run to is then a
/// closed stdout's doing and nothing else's.
#[test]
fn every_run_listed_writes_stdout_and_nothing_on_stderr() {
    let scratch = Scratch::new("ein-closed-stdout");
    let mut bad = Vec::new();
    for (name, argv) in runs(&scratch) {
        let (status, stdout, stderr) = full(&argv);
        if !status.success() || stdout.is_empty() || !stderr.is_empty() {
            bad.push(format!(
                "{name}: {}, {} bytes of stdout, stderr {stderr:?}",
                ended(status),
                stdout.len()
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A reader closed before the run ends every run by `SIGPIPE`**, at its
/// first write, with nothing on stderr: never the panic, *failed printing to
/// stdout*, exit 101 (W75), and never exit 0 with the output lost, as `clap`'s
/// help did, its write's error ignored, until S1f.5.25.
#[test]
fn a_reader_closed_before_the_run_ends_every_run_by_sigpipe() {
    let scratch = Scratch::new("ein-closed-stdout");
    let mut bad = Vec::new();
    for (name, argv) in runs(&scratch) {
        let (status, stderr) = closed_before(&argv);
        if status.signal() != Some(SIGPIPE) || !stderr.is_empty() {
            bad.push(format!("{name}: {}, stderr {stderr:?}", ended(status)));
        }
    }
    assert!(
        bad.is_empty(),
        "a run whose stdout's reader closed before it started did not end by \
         SIGPIPE ({SIGPIPE}) with nothing on stderr:\n{}",
        bad.join("\n")
    );
}

/// **A reader that closes after the first line, as `head -1` does**: a run
/// that must write after it has gone ends by `SIGPIPE`; one whose stdout fits
/// in what the reader took and the pipe holds may end first, exit 0, as `cat`
/// does. Never 101, and never a word on stderr. Which of the two a run is, is
/// read off its stdout's size read in full, so the test is not a race: the
/// pipe is one page where the kernel lets it shrink, and a run whose stdout is
/// larger than the first line and that page cannot have finished writing.
#[test]
fn a_reader_that_closes_after_the_first_line_ends_a_run_that_writes_again() {
    let scratch = Scratch::new("ein-closed-stdout");
    let mut bad = Vec::new();
    let (mut strict, mut either) = (0usize, 0usize);
    for (name, argv) in runs(&scratch) {
        let (_, stdout, _) = full(&argv);
        let (status, stderr, read, held) = first_line(&argv);
        let must_write_again = held.is_some_and(|h| stdout.len() > read + h);
        let ok = if must_write_again {
            strict += 1;
            status.signal() == Some(SIGPIPE)
        } else {
            either += 1;
            status.signal() == Some(SIGPIPE) || status.code() == Some(0)
        };
        if !ok || !stderr.is_empty() {
            bad.push(format!(
                "{name}: {} ({} bytes of stdout, {read} read, the pipe {held:?}), stderr {stderr:?}",
                ended(status),
                stdout.len()
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // Where the pipe can be made one page, the strict arm is not empty: the
    // census of T1f.5.25.2 had saturate's, the help of `solve`, the artefacts
    // on `/dev/stdout` and the region's views over a page.
    if cfg!(target_os = "linux") {
        assert!(
            strict > 0,
            "no run wrote more than a page: the strict arm held nothing ({either} either way)"
        );
    }
}

/// **A command that writes nothing to stdout is not ended by a closed one**:
/// `kb save` writes its container and exits 0, its reader closed before it
/// started, the control of the two tests above.
#[cfg(feature = "einb")]
#[test]
fn a_command_that_writes_nothing_to_stdout_is_not_ended_by_a_closed_one() {
    let scratch = Scratch::new("ein-closed-stdout");
    let out = scratch.join("w.einb");
    let argv: Vec<String> = ["kb", "save", W]
        .into_iter()
        .map(str::to_string)
        .chain([out.display().to_string()])
        .collect();
    let (status, stdout, stderr) = full(&argv);
    assert!(
        status.success() && stdout.is_empty() && stderr.is_empty(),
        "ein kb save, read in full: {}, stdout {stdout:?}, stderr {stderr:?}",
        ended(status)
    );
    std::fs::remove_file(&out).expect("the container was written");
    let (status, stderr) = closed_before(&argv);
    assert!(
        status.code() == Some(0) && stderr.is_empty() && out.is_file(),
        "ein kb save, its stdout closed: {}, stderr {stderr:?}, container written: {}",
        ended(status),
        out.is_file()
    );
}
