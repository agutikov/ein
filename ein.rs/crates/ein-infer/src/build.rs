//! What this build of the engine has compiled in — the half of `ein
//! --version` that the binary cannot work out for itself
//! ([S1a.9.3](../../../../docs/history/m1a_rust/README.md#s1a93--packaging-and-release)
//! T1a.9.3.4).
//!
//! `cfg!(feature = "parallel")` in `ein-cli` reads **`ein-cli`'s** feature
//! table, not this crate's, so a dependent has no way to ask. It asks here
//! instead, and the answer is compiled in beside the code it describes.
//!
//! Only features that change what the engine *is* are listed. `parallel` is
//! the one a user can observe without reading a profile: without it `--jobs
//! N` is accepted and inert, every layer running on the committing thread
//! ([design/12 §3](../../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md#3-feature-flags)),
//! and a version line that did not say so would leave "I passed `--jobs 8`
//! and nothing happened" undiagnosable.

/// The engine's features, in declaration order (which is also alphabetical —
/// the caller sorts the merged list anyway).
pub fn features() -> Vec<&'static str> {
    let mut out = Vec::new();
    if cfg!(feature = "counters") {
        out.push("counters");
    }
    if cfg!(feature = "fork-delta") {
        out.push("fork-delta");
    }
    if cfg!(feature = "parallel") {
        out.push("parallel");
    }
    if cfg!(feature = "spec-audit") {
        out.push("spec-audit");
    }
    out
}

#[cfg(test)]
mod tests {
    /// The list is hand-written, so the floor is that it is not empty on a
    /// default build and that the one feature the default turns on is in it.
    /// A feature added to `Cargo.toml` and not here reports a build as
    /// something it is not, which is the whole failure mode.
    #[test]
    fn a_default_build_reports_parallel() {
        let f = super::features();
        assert_eq!(f.contains(&"parallel"), cfg!(feature = "parallel"), "{f:?}");
    }
}
