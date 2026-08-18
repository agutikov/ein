//! `SolverConfig` — the typed view over the IR `(config …)` head.
//!
//! Each field maps 1:1 to a `:kebab-flag`, and the field *list with its
//! declaration order* is observable twice over: `--dump-config` prints it in
//! declaration order, and an unknown flag is rejected with the valid names
//! **sorted** ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §3c).
//! [`FIELDS`] is that list, and the loader
//! ([S1a.2.3](../../../../plans/m1a_rust/p1a.2_kb_core/s1a.2.3_loader.md))
//! parses through it.
//!
//! It lives here rather than beside the engine because the KB holds one and
//! `fork()` carries it over: ein.py keeps the same edge, as a checker-only
//! `TYPE_CHECKING` import that exists to avoid a runtime cycle.
//!
//! Resolution precedence, unchanged: an explicit `solve(config=…)` argument,
//! then `kb.config` from the IR, then these defaults.

/// What kind of value a flag takes — the type dispatch ein.py reads off
/// `field.type`, which is a *string* there because the module uses
/// `from __future__ import annotations`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldKind {
    Bool,
    Int,
    Float,
    Str,
}

/// Every flag, in **declaration order**.
pub const FIELDS: &[(&str, FieldKind)] = &[
    ("enable-pre-branch-lookahead", FieldKind::Bool),
    ("enable-lookahead-kill-cache", FieldKind::Bool),
    ("hypgen-scoring", FieldKind::Str),
    ("hypgen-rel-weight", FieldKind::Float),
    ("hypgen-obj-weight", FieldKind::Float),
    ("print-alive", FieldKind::Bool),
    ("warn-derived-naf", FieldKind::Bool),
    ("candidate-order-seed", FieldKind::Int),
    ("lattice-sanity-check", FieldKind::Bool),
    ("lattice-order", FieldKind::Str),
    ("lattice-order-seed", FieldKind::Int),
    ("enable-path-nogoods", FieldKind::Bool),
    ("enable-symmetric-mirror", FieldKind::Bool),
    ("enable-singleton-writeback", FieldKind::Bool),
    ("enable-forced-positive", FieldKind::Bool),
    ("record-alternative-justifications", FieldKind::Bool),
    ("enable-fail-fast-fork", FieldKind::Bool),
];

#[derive(Clone, PartialEq, Debug)]
pub struct SolverConfig {
    /// Topic B Tier B — the one-step rule simulator `_dies_immediately`.
    pub enable_pre_branch_lookahead: bool,
    /// Cache a lookahead kill as a `(not h)` fact, so later enumerations skip
    /// `h` through the O(1) negated index instead of re-running the match.
    pub enable_lookahead_kill_cache: bool,
    /// `"popularity"` (the default since S1.5a.7) or `"most-constrained"`.
    pub hypgen_scoring: String,
    pub hypgen_rel_weight: f64,
    pub hypgen_obj_weight: f64,
    pub print_alive: bool,
    /// Warn when an `(absent …)` guard watches a rule-derived relation —
    /// since S1.21.8 a *stratification* signal, not a soundness one.
    pub warn_derived_naf: bool,
    /// Negative means the S1.5a.1a content sort; non-negative applies a
    /// per-branch deterministic permutation of it.
    pub candidate_order_seed: i64,
    pub lattice_sanity_check: bool,
    /// `"lex"` (the default, and what the baselines were recorded under) or
    /// `"score-sum"`.
    pub lattice_order: String,
    /// `None` disables the per-layer shuffle; a seed makes traversal order a
    /// deterministic permutation, which is how shuffle-invariance is probed.
    pub lattice_order_seed: Option<i64>,
    pub enable_path_nogoods: bool,
    pub enable_symmetric_mirror: bool,
    pub enable_singleton_writeback: bool,
    pub enable_forced_positive: bool,
    /// S1.21.7 — record a re-derivation as an alternative justification
    /// rather than dropping it, making the proof structure an AND/OR graph.
    pub record_alternative_justifications: bool,
    /// S1.9.E23 — stop a fork's saturation at the firing that makes it
    /// inconsistent. The verdict is unchanged either way.
    pub enable_fail_fast_fork: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        SolverConfig {
            enable_pre_branch_lookahead: true,
            enable_lookahead_kill_cache: true,
            hypgen_scoring: "popularity".to_string(),
            hypgen_rel_weight: 1.0,
            hypgen_obj_weight: 1.0,
            print_alive: false,
            warn_derived_naf: false,
            candidate_order_seed: -1,
            lattice_sanity_check: false,
            lattice_order: "lex".to_string(),
            lattice_order_seed: None,
            enable_path_nogoods: true,
            enable_symmetric_mirror: true,
            enable_singleton_writeback: true,
            enable_forced_positive: true,
            record_alternative_justifications: true,
            enable_fail_fast_fork: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_field_has_a_flag_and_no_flag_is_orphaned() {
        // The two lists are maintained by hand, and a mismatch would show up
        // as a flag `--dump-config` never prints or one the loader accepts
        // and then drops. Counting is the cheap half; the loader's own tests
        // check that each name reaches its field.
        assert_eq!(FIELDS.len(), 17);
        let mut names: Vec<&str> = FIELDS.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), FIELDS.len(), "a flag name repeats");
        for (name, _) in FIELDS {
            assert!(!name.contains('_'), "{name} should be kebab-case");
        }
    }
}
