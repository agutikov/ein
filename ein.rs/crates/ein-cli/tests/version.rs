//! T1a.9.3.4 — **`ein --version` reports the four things a version number
//! does not**: the event protocol, the container format, the features
//! compiled in, and the stdlib this binary would load.
//!
//! No golden. Three of the five lines are machine-dependent (the stdlib path)
//! or move on every release (the semver), and a golden that has to be
//! re-blessed to bump a version stops being read. What is asserted instead is
//! that each line is *there*, that each says something checkable, and — for
//! the two that can be wrong rather than merely absent — that it agrees with
//! an independent computation: the digest against `sha2` over the file, the
//! feature list against the build's own `cfg!`.

use std::path::PathBuf;
use std::process::Command;

use ein_corpus::repo_root;
use sha2::{Digest, Sha256};

/// `ein --version`, with the stdlib pinned to the checkout so the digest is
/// the repo's rather than whatever the runner's environment resolved to.
fn version_with(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .env("EIN_STDLIB", repo_root().join("stdlib"))
        .output()
        .expect("run ein");
    assert!(
        out.status.success(),
        "ein {args:?} exited {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8")
}

/// The value of one `key value` line.
fn field<'a>(text: &'a str, key: &str) -> &'a str {
    text.lines()
        .find(|l| l.starts_with(key))
        .unwrap_or_else(|| panic!("no {key:?} line in:\n{text}"))
        .strip_prefix(key)
        .expect("the prefix that just matched")
        .trim()
}

#[test]
fn the_version_line_names_the_program_and_its_semver() {
    let text = version_with(&["--version"]);
    let first = text.lines().next().expect("a first line");
    assert_eq!(first, format!("ein {}", env!("CARGO_PKG_VERSION")));
    // The workspace shares one version, so the binary's is the engine's.
    assert!(
        first.split_whitespace().nth(1).is_some_and(|v| {
            v.split('.').count() == 3 && v.split('.').all(|p| p.parse::<u32>().is_ok())
        }),
        "not a semver: {first:?}"
    );
}

/// The protocol string is the one a `--events` file's first line declares.
/// Read out of the engine rather than copied, because copying it is how the
/// two come apart.
#[test]
fn the_protocol_is_the_one_an_events_file_declares() {
    let text = version_with(&["--version"]);
    assert_eq!(field(&text, "protocol"), ein_infer::events::SCHEMA);
}

/// `einb/<major>.<minor>` on a default build, `none` where the container was
/// not compiled in — never a version number for a format this binary cannot
/// read.
#[test]
fn the_container_line_says_what_this_build_can_open() {
    let text = version_with(&["--version"]);
    let got = field(&text, "container");
    if cfg!(feature = "einb") {
        assert!(
            got.starts_with("einb/") && got["einb/".len()..].split('.').count() == 2,
            "{got:?}"
        );
    } else {
        assert_eq!(got, "none");
    }
}

/// The feature line is the build's own, checked against `cfg!` on both sides
/// of the crate boundary — `ein-cli`'s for `einb` and `snmalloc`, the
/// engine's for `parallel`, which `ein-cli` cannot see.
#[test]
fn the_feature_line_is_this_build() {
    let text = version_with(&["--version"]);
    let raw = field(&text, "features");
    // A build with no features says `none`, not nothing: no line in the
    // report is allowed to be empty or to trail into whitespace.
    assert!(!raw.is_empty(), "the features line is blank");
    let listed: Vec<&str> = if raw == "none" {
        Vec::new()
    } else {
        raw.split(", ").collect()
    };

    assert_eq!(listed.contains(&"einb"), cfg!(feature = "einb"));
    assert_eq!(listed.contains(&"snmalloc"), cfg!(feature = "snmalloc"));
    assert_eq!(
        listed.contains(&"parallel"),
        ein_infer::build::features().contains(&"parallel"),
        "the engine and the version line disagree about the fan-out"
    );

    let mut sorted = listed.clone();
    sorted.sort_unstable();
    assert_eq!(listed, sorted, "the feature line is not sorted");
    assert_eq!(
        listed.len(),
        listed
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "a forwarded feature was reported twice: {listed:?}"
    );
}

/// The digest is what `sha256sum stdlib/MANIFEST.sha256` prints. That is the
/// whole point of choosing SHA-256 over the BLAKE3 `.einb` already carries:
/// the verification instruction is a command the reader has.
#[test]
fn the_stdlib_digest_is_the_manifest_sha256() {
    let manifest = repo_root().join("stdlib").join(ein_ir::stdlib::MARKER);
    let bytes = std::fs::read(&manifest).expect("the manifest");
    let want = format!("sha256:{:x}", Sha256::digest(&bytes));

    let text = version_with(&["--version"]);
    let got = field(&text, "stdlib");
    assert!(
        got.starts_with(&want),
        "{got:?} does not start with {want:?}"
    );
    // …and it names *which* resolution step answered, not only the path:
    // `embedded` and `checkout <same bytes>` are two different programs.
    let source = got[want.len()..].trim();
    let (step, path) = source.split_once(' ').expect("<step> <path>");
    assert_eq!(step, "$EIN_STDLIB", "{source:?}");
    assert_eq!(PathBuf::from(path), repo_root().join("stdlib"));
}

/// A binary pointed at a different stdlib reports a different digest. Without
/// this the line could be a constant baked in at build time and every
/// assertion above would still pass.
#[test]
fn a_different_stdlib_is_a_different_digest() {
    let here = version_with(&["--version"]);
    let elsewhere = String::from_utf8(
        Command::new(env!("CARGO_BIN_EXE_ein"))
            .arg("--version")
            .env("EIN_STDLIB", repo_root().join("examples"))
            .output()
            .expect("run ein")
            .stdout,
    )
    .expect("utf-8");
    // `examples/` has no MANIFEST.sha256, so the honest answer is that it
    // could not read one — not a stale digest from somewhere else.
    assert_ne!(field(&here, "stdlib"), field(&elsewhere, "stdlib"));
    assert!(
        field(&elsewhere, "stdlib").starts_with("unreadable"),
        "{:?}",
        field(&elsewhere, "stdlib")
    );
}

/// No line trails into whitespace, and none is empty after its key. Cheap,
/// and it is what caught `features` printing nothing at all on a
/// `--no-default-features` build.
#[test]
fn every_line_says_something() {
    let text = version_with(&["--version"]);
    assert!(text.ends_with('\n'));
    for line in text.lines() {
        assert_eq!(line, line.trim_end(), "trailing whitespace: {line:?}");
        assert!(
            line.split_whitespace().count() >= 2,
            "a key with no value: {line:?}"
        );
    }
}

#[test]
fn the_short_and_long_flags_are_the_same_report() {
    assert_eq!(version_with(&["--version"]), version_with(&["-V"]));
}

/// Only in first position. `--version` after a subcommand is a usage error,
/// because that is what it was before this flag existed and I1 does not stop
/// applying to the flags a release adds.
#[test]
fn a_version_flag_after_a_subcommand_is_a_usage_error() {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["render", "rules", "--version"])
        .output()
        .expect("run ein");
    assert_eq!(out.status.code(), Some(2), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stdout).is_empty());
}

/// `ein --help` lists it. The flag is registered on the parser purely for
/// this, since `run` answers it before the parser is reached — the same
/// arrangement `saturate` has, and the same hazard: a registration removed as
/// "dead" would silently un-document a shipped flag.
#[test]
fn the_help_lists_it() {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("--help")
        .output()
        .expect("run ein");
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("--version"), "{help}");
    assert!(help.contains("-V"), "{help}");
}
