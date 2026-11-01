//! `SolverConfig` — the typed view over the IR `(config …)` head.
//!
//! Each field maps 1:1 to a `:kebab-flag`, and the field *list with its
//! declaration order* is observable twice over: `--dump-config` prints it in
//! declaration order, and an unknown flag is rejected with the valid names
//! **sorted** ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §3c).
//! [`FIELDS`] is that list, and the loader
//! ([S1a.2.3](../../../../docs/history/m1a_rust/README.md#s1a23--the-loader-from_ir))
//! parses through it.
//!
//! It lives here rather than beside the engine because the KB holds one and
//! `fork()` carries it over.
//!
//! **Resolution precedence**, and it is two orders rather than one. The
//! engine's is `SolveOptions::config` (an explicit argument), then
//! `kb.config` from the IR, then these defaults; the binary's is the six CLI
//! options that shadow a flag, then the program's **last** `(config …)` head
//! taken whole, then these defaults — `ein-cli`'s `resolved_config` computes
//! the second and hands the result to the first.
//!
//! Every flag, with its default, what it changes, whether it changes the
//! answer and what depending on it commits you to:
//! [`docs/kernel/configuration.md`](../../../../docs/kernel/configuration.md),
//! whose defaults block is `--dump-config`'s own output and whose flag list is
//! diffed against [`FIELDS`] by `ein-cli/tests/config_reference.rs`.

/// What kind of value a flag takes — the type the loader coerces it to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldKind {
    Bool,
    Int,
    Float,
    Str,
}

/// The flags a `(config …)` takes, in declaration order: the eight
/// `solve_interface.md` § 2.4 kept at M1e2 S1e2.1a.5's switch, less
/// `enable-symmetric-mirror`, retired at S1e2.1a.6. What `--dump-config`
/// prints, what the KB-shape dump renders, and what the loader parses.
pub const FLAGS: &[(&str, FieldKind)] = &[
    ("enable-pre-branch-lookahead", FieldKind::Bool),
    ("enable-lookahead-kill-cache", FieldKind::Bool),
    ("warn-derived-naf", FieldKind::Bool),
    ("lattice-sanity-check", FieldKind::Bool),
    ("enable-path-nogoods", FieldKind::Bool),
    ("record-alternative-justifications", FieldKind::Bool),
    ("enable-fail-fast-fork", FieldKind::Bool),
];

/// The retired flags, each with the M1e2 stage that retired it and why — the
/// loader's error, so a program that sets one is refused with the reason
/// rather than as an unknown flag. Nine went with the walk at S1e2.1a.5's
/// switch (`solve_interface.md` § 2.4), and `enable-symmetric-mirror` at
/// S1e2.1a.6, the one flag that changed R (the user's ruling of 2026-11-01).
/// Their fields stay on [`SolverConfig`] at their defaults: the nine for the
/// walk's library code, which no command reaches, and the mirror's for the
/// `.einb` container, since no code reads it — the mirror is always on.
pub const RETIRED: &[(&str, &str, &str)] = &[
    (
        "hypgen-scoring",
        "S1e2.1a.5",
        "an order lever, and every route's order is part of its definition",
    ),
    (
        "hypgen-rel-weight",
        "S1e2.1a.5",
        "an order lever, and every route's order is part of its definition",
    ),
    (
        "hypgen-obj-weight",
        "S1e2.1a.5",
        "an order lever, and every route's order is part of its definition",
    ),
    (
        "print-alive",
        "S1e2.1a.5",
        "it was inert: no code path read it (Q-M1e.10)",
    ),
    (
        "candidate-order-seed",
        "S1e2.1a.5",
        "it was inert: no code path read it (Q-M1e.10)",
    ),
    (
        "lattice-order",
        "S1e2.1a.5",
        "an order lever, and every route's order is part of its definition",
    ),
    (
        "lattice-order-seed",
        "S1e2.1a.5",
        "an order lever, and every route's order is part of its definition",
    ),
    (
        "enable-symmetric-mirror",
        "S1e2.1a.6",
        "off, it changed the answer, and no config option may. The mirror is \
         what `(__symmetric__ R)` means, so it is always on: a relation that is \
         not to be mirrored is not marked",
    ),
    (
        "enable-singleton-writeback",
        "S1e2.1a.5",
        "it wrote into root, which no route does: a candidate dead alone leaves \
         the universe by L1",
    ),
    (
        "enable-forced-positive",
        "S1e2.1a.5",
        "it wrote into root, which no route does: a candidate dead alone leaves \
         the universe by L1",
    ),
];

/// Every field of [`SolverConfig`], in **declaration order**: [`FLAGS`] and
/// [`RETIRED`]'s, interleaved as declared. What the `.einb` container
/// round-trips, so its format did not move with the switch.
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
    /// Topic B Tier B — the one-step rule simulator, `Lookahead::dies_immediately`.
    pub enable_pre_branch_lookahead: bool,
    /// Cache a lookahead kill as a `(not h)` fact, so later enumerations skip
    /// `h` through the O(1) negated index instead of re-running the match.
    pub enable_lookahead_kill_cache: bool,
    /// `"popularity"` (the default since S1.5a.7) or `"most-constrained"`.
    pub hypgen_scoring: String,
    pub hypgen_rel_weight: f64,
    pub hypgen_obj_weight: f64,
    /// **Inert** — parsed, type-checked, dumped, echoed into
    /// `--json-summary`, rendered into the KB-shape digest and round-tripped
    /// through `.einb`, and read by no code path. What it names — the inherited
    /// alive-set size and the per-filter prune counts of each layer — is
    /// what `--verbose`, `--layer-progress` and `--events`' `layer` counters
    /// say. Held by
    /// `ein-cli/tests/config_reference.rs::the_two_inert_flags_are_still_inert`
    /// and filed as
    /// [Q-M1e.10](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert).
    pub print_alive: bool,
    /// Warn about an `(absent …)` guard — **two questions behind one flag**,
    /// emitted once each after root's saturation.
    ///
    /// - `DerivedNafWarning`: the guard watches a **rule-derived**
    ///   relation. Since S1.21.8 a *stratification* signal, not a soundness
    ///   one — the guard is judged at a fixpoint either way.
    /// - `RefutationUnderAbsentWarning`, M1e S1e.2.3's: the rule concludes
    ///   `(false)` or a `(not …)` from a guard over a relation the program's
    ///   **hypothesis generator can still propose**. That one *is* a
    ///   soundness signal — `dead` is not upward-closed under `absent`
    ///   ([Q-M1e.9]) — and it rides this flag rather than an eighteenth
    ///   field because `SolverConfig` is rendered into the KB-shape digest,
    ///   so a new field re-blesses every shape golden in the corpus.
    ///
    /// Off by default, and the second is why that is a decision rather than
    /// an inheritance: the corpus is **not** silent — ten rules over eight
    /// entries carry the shape (nine over seven when S1e.2.3 measured it; the
    /// tenth is M1e2's banked twin of the probe), four of them stdlib scans
    /// over a membership relation, and a warning that fires on a working
    /// example is one that gets turned off. The set is named and pinned in
    /// `ein-infer/tests/refutation_under_absent.rs`.
    ///
    /// [Q-M1e.9]: ../../../../docs/history/m1e_review_processing/open_questions.md
    pub warn_derived_naf: bool,
    /// Meant as: negative means the S1.5a.1a content sort; non-negative
    /// applies a per-branch deterministic permutation of it. Nothing
    /// implements that — the field is [`print_alive`]'s twin and equally
    /// **inert**: nothing outside the loader, the four renderers and the
    /// container reads it, and the per-layer shuffle that *is* wired is
    /// [`SolverConfig::lattice_order_seed`]. The first sentence is kept because
    /// it is the only statement of what the flag was for, which is the input to
    /// [Q-M1e.10](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)'s
    /// option (a).
    ///
    /// [`print_alive`]: SolverConfig::print_alive
    pub candidate_order_seed: i64,
    pub lattice_sanity_check: bool,
    /// `"lex"` (the default, and what the baselines were recorded under) or
    /// `"score-sum"`.
    pub lattice_order: String,
    /// `None` disables the per-layer shuffle; a seed makes traversal order a
    /// deterministic permutation, which is how shuffle-invariance is probed.
    pub lattice_order_seed: Option<i64>,
    pub enable_path_nogoods: bool,
    /// **Retired at M1e2 S1e2.1a.6, and read by nothing.** The saturator
    /// mirrors every relation marked `(__symmetric__ R)`, always; off, this
    /// flag took the mirror's facts out of root's fixpoint and so changed R,
    /// which no config option may (`configuration.md` § 2.5). The field stays,
    /// at `true`, because the `.einb` container round-trips [`FIELDS`].
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

/// Every field as `(kebab-name, repr(value))`, in declaration order — what
/// `--dump-config` walks and what the KB-shape dump prints.
///
/// Booleans print as `True` / `False`, an unset seed as `None`, a string
/// through [`crate::repr::repr_str`], and a float through Rust's shortest
/// round-trip formatting (`{:?}`), so a weight prints as `1.0` or `0.5`.
pub fn rendered_fields(c: &SolverConfig) -> Vec<(&'static str, String)> {
    every_field(c)
        .into_iter()
        .filter(|(name, _)| FLAGS.iter().any(|(f, _)| f == name))
        .collect()
}

/// [`rendered_fields`] over every field, retired ones included.
fn every_field(c: &SolverConfig) -> Vec<(&'static str, String)> {
    let b = |v: bool| {
        if v {
            "True".to_string()
        } else {
            "False".to_string()
        }
    };
    let s = |v: &str| crate::repr::repr_str(v);
    vec![
        (
            "enable-pre-branch-lookahead",
            b(c.enable_pre_branch_lookahead),
        ),
        (
            "enable-lookahead-kill-cache",
            b(c.enable_lookahead_kill_cache),
        ),
        ("hypgen-scoring", s(&c.hypgen_scoring)),
        ("hypgen-rel-weight", format!("{:?}", c.hypgen_rel_weight)),
        ("hypgen-obj-weight", format!("{:?}", c.hypgen_obj_weight)),
        ("print-alive", b(c.print_alive)),
        ("warn-derived-naf", b(c.warn_derived_naf)),
        ("candidate-order-seed", c.candidate_order_seed.to_string()),
        ("lattice-sanity-check", b(c.lattice_sanity_check)),
        ("lattice-order", s(&c.lattice_order)),
        (
            "lattice-order-seed",
            c.lattice_order_seed
                .map_or("None".to_string(), |v| v.to_string()),
        ),
        ("enable-path-nogoods", b(c.enable_path_nogoods)),
        ("enable-symmetric-mirror", b(c.enable_symmetric_mirror)),
        (
            "enable-singleton-writeback",
            b(c.enable_singleton_writeback),
        ),
        ("enable-forced-positive", b(c.enable_forced_positive)),
        (
            "record-alternative-justifications",
            b(c.record_alternative_justifications),
        ),
        ("enable-fail-fast-fork", b(c.enable_fail_fast_fork)),
    ]
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
        assert_eq!((FLAGS.len(), RETIRED.len()), (7, 10));
        let mut names: Vec<&str> = FIELDS.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), FIELDS.len(), "a flag name repeats");
        for (name, _) in FIELDS {
            assert!(!name.contains('_'), "{name} should be kebab-case");
            let kept = FLAGS.iter().any(|(f, _)| f == name);
            let retired = RETIRED.iter().any(|(f, _, _)| f == name);
            assert!(
                kept != retired,
                "{name} is kept or retired, not both or neither"
            );
        }
        let every = every_field(&SolverConfig::default());
        assert_eq!(
            every.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            FIELDS.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            "the renderer and the field table drifted"
        );
        let rendered = rendered_fields(&SolverConfig::default());
        assert_eq!(
            rendered.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            FLAGS.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            "the renderer and the flag table drifted"
        );
    }
}
