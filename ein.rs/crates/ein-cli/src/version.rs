//! `ein --version` — what this binary *is*, in five lines
//! ([S1a.9.3](../../../../plans/m1a_rust/p1a.9_release/s1a.9.3_packaging.md)
//! T1a.9.3.4).
//!
//! A version number alone answers almost nothing about this program. Three of
//! the four things that decide what a run does are not in it:
//!
//! - the **event protocol** a `--events` file will declare, which is what a
//!   consumer of the stream needs before it reads a byte;
//! - the **features** compiled in, because `--jobs 8` is accepted and inert
//!   without `parallel`, `ein kb` is not registered without `einb`, and
//!   neither absence is visible from the outside;
//! - the **stdlib**, which is the one input that can differ between a binary
//!   and the checkout beside it. `std.*` is resolved at run time in three
//!   steps — `$EIN_STDLIB`, the checkout, the embedded copy
//!   ([`ein_ir::stdlib`]) — so two runs of the same binary can load different
//!   programs, and "the rule I edited did nothing" has no other explanation
//!   that is this cheap to rule out.
//!
//! The digest is **SHA-256 of `stdlib/MANIFEST.sha256` as resolved**, printed
//! whole, so the check is a command a reader already knows:
//!
//! ```text
//! $ ein --version | grep stdlib
//! stdlib     sha256:9d1f…  checkout /home/user/work/ein/stdlib
//! $ sha256sum stdlib/MANIFEST.sha256
//! 9d1f…  stdlib/MANIFEST.sha256
//! ```
//!
//! **Not the same digest `.einb` carries.** [`ein_einb::meta::stdlib_digest`]
//! hashes the same bytes with BLAKE3 because it is a field of a binary
//! container — content addressing, frozen into the format
//! ([design/10](../../../../plans/m1a_rust/design/10_binary_format.md) §2).
//! This one is for a human with `sha256sum`, and the manifest it hashes is
//! itself a list of SHA-256 digests, so a second algorithm here would make
//! the file describe itself in two ways.

use sha2::{Digest, Sha256};

/// The report, newline-terminated per line and without a trailing blank.
pub fn report() -> String {
    let mut out = format!("ein {}\n", env!("CARGO_PKG_VERSION"));
    row(&mut out, "protocol", ein_infer::events::SCHEMA);
    row(&mut out, "container", &container());
    // `none` rather than the empty string: a fully dependency-light build has
    // no features, and a line that trails off into whitespace reads as a bug
    // in `--version` rather than as a fact about the binary. `container` says
    // `none` for the same reason.
    let feats = features();
    let feats = if feats.is_empty() {
        "none".to_string()
    } else {
        feats.join(", ")
    };
    row(&mut out, "features", &feats);
    row(&mut out, "stdlib", &stdlib());
    out
}

/// One `key value` line, the keys aligned to the longest of them.
fn row(out: &mut String, key: &str, value: &str) {
    out.push_str(&format!("{key:<10} {value}\n"));
}

/// `.einb`'s format version, or what a build with no container should say
/// rather than a version number for something it cannot read.
fn container() -> String {
    #[cfg(feature = "einb")]
    {
        format!(
            "einb/{}.{}",
            ein_einb::header::FORMAT_MAJOR,
            ein_einb::header::FORMAT_MINOR
        )
    }
    #[cfg(not(feature = "einb"))]
    {
        "none".to_string()
    }
}

/// This build's features — the binary's own, merged with the engine's, sorted
/// and de-duplicated.
///
/// De-duplicated because `fork-delta` and `spec-audit` are *forwarded*: they
/// are features of both crates and one build turns on both halves. Sorted
/// because the line is compared by eye between two binaries, and a set
/// printed in declaration order is a set two crates can disagree about the
/// order of.
pub fn features() -> Vec<&'static str> {
    let mut out = ein_infer::build::features();
    if cfg!(feature = "einb") {
        out.push("einb");
    }
    if cfg!(feature = "snmalloc") {
        out.push("snmalloc");
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The resolved stdlib: its manifest digest and where it came from.
///
/// `resolve_default` walks from the running executable, which is what a run
/// does, so this reports the stdlib **this binary would load** rather than
/// the one it was built with. Those differ exactly when the diagnosis is
/// wanted.
fn stdlib() -> String {
    let source = ein_ir::stdlib::resolve_default();
    match source.read(ein_ir::stdlib::MARKER) {
        Some(text) => format!(
            "sha256:{:x}  {}",
            Sha256::digest(text.as_bytes()),
            where_(&source)
        ),
        // Not an error and not a panic: `--version` is what a user runs to
        // find out *why* the engine is behaving oddly, and a broken stdlib
        // path is one of the reasons.
        None => format!("unreadable  {}", where_(&source)),
    }
}

/// **Which of the three resolution steps answered**, not just the path.
///
/// The path alone leaves the question that sent the reader here unanswered: a
/// binary run outside a checkout reports `embedded`, the same binary run
/// inside one reports `checkout …`, and those are two different programs.
/// `$EIN_STDLIB` is named rather than shown as a bare path because it is the
/// step that overrides the other two silently.
fn where_(source: &ein_ir::stdlib::Source) -> String {
    match source {
        ein_ir::stdlib::Source::Override(p) => format!("$EIN_STDLIB {}", p.display()),
        ein_ir::stdlib::Source::Checkout(p) => format!("checkout {}", p.display()),
        ein_ir::stdlib::Source::Embedded => "embedded".to_string(),
    }
}
