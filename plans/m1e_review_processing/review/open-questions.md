# Open questions

Issues that could not be resolved from repository evidence within this review. None is promoted to a finding.

## Q1 — Shared no-goods across concurrent workers: is the determinism argument airtight?

- **Unclear:** The `Nogoods` store is shared by `Arc<RwLock<…>>` across forks; under `--jobs N` a clause learned while a layer is fanned out could in principle prune another worker's candidate depending on scheduling. The invariance evidence is strong (20 712 cells, byte-identical verbose streams, `jobs_does_not_move_the_answer_or_a_counter`), and commits replay in candidate order on the committing thread — but this review did not reconstruct the structural argument that mid-flight reads cannot differ between runs.
- **Why it matters:** `--jobs N is the same computation` is a headline claim.
- **Evidence creating the ambiguity:** `ein-core/src/kb.rs:369-408` (Nogoods, snapshot copying the shared Arc); `solve.rs` fan-out (:1789-1801).
- **Resolves it:** A written argument (or a test that injects a mid-layer clause from another thread and shows the commit-order replay masks it).

## Q2 — Does the MAX_ALT_JUSTIFICATIONS=32 cap ever change which unsat core is reported?

- **Unclear:** `alts` is capped at 32, sorted shortest-premises-first, shortest evicts longest. The unsat-core search walks recorded derivations ("smallest source frontier … searched across every recorded derivation"). If eviction removes the derivation whose frontier is smallest, the reported core could be larger than the recorded-derivation minimum the docs promise.
- **Why it matters:** The core is a user-facing explanation artifact and an M2 feedback signal.
- **Evidence:** `kb.rs:42-49, 1456-1535`; `explain.rs` (smallest_contradiction_frontier); docs claim in README ("smallest set of given facts … not a subset-minimal MUS").
- **Resolves it:** A fixture with >32 alternative justifications where the shortest-frontier derivation is evicted, or an argument that shortest-premises-first retention makes eviction harmless to the frontier search.

## Q3 — What is the true trigger shape of Q-M1a.8, if any?

- **Unclear:** defined_behaviour.md §3.2's claimed int-argument activator collision did not reproduce (see `review/code-doc-consistency/high.md`). Nested-Fact activator args (which bind nothing) are the remaining candidate shape; untested.
- **Resolves it:** One decisive probe per shape, banked as a test either way, and Q-M1a.8 amended or closed.

## Q4 — Can the inter-layer alive-∅ path record a false model?

- **Unclear:** Whether a program encoding totality as a saturation `(false)` rule (rather than an obligation) can make root recorded as a model whose falsity only a fork would derive (see `review/correctness/medium.md`).
- **Resolves it:** A constructed fixture, or a written invariant argument beside `solve.rs:1528-1551`.

## Q5 — Q40: which side of the lookahead verdict flip is correct, and is the wrong one golden-pinned?

- **Unclear:** With `enable_pre_branch_lookahead` off, `branching/06` and `lattice/02` change from Solution/Ambiguity to Contradiction; README's Known gaps records that "one of the two configurations is wrong today". This review could not establish which, nor whether the current verdicts are pinned by corpus goldens such that fixing the semantics requires a deliberate re-bless.
- **Why it matters:** A performance lever currently decides what a complete model is — the project's own phrasing.
- **Resolves it:** A hand-derivation of the two fixtures' true model sets (they are small), then a golden audit.

## Q6 — Is the tree traversal's inner-node rung flip actually constructible?

- **Unclear:** The fragility in `review/correctness/high.md`(c) requires an obligation activator fact derived under a hypothesis. Whether the current stdlib + loader rules make that expressible (activators are ordinary facts, so a rule can derive one) was argued but not demonstrated.
- **Resolves it:** A probe program with a rule deriving `(total ?R …)`-style activator facts inside a fork under `EIN_TRAVERSAL=tree`, checked for missed models against the lattice.

## Q7 — What does `-n 0` mean?

- **Unclear:** `SolveOptions{stop_after: Some(0)}` semantics (see `review/error-handling/low.md`); likely ein.py parity, but no test, doc, or comment states it.
- **Resolves it:** A ruling: refuse or define + pin.

## Q8 — Does anything still pin zebra.ein and zebra2.ein to the same model?

- **Unclear:** The claim's named owner was a deleted Python acceptance file (`examples/README.md:27`). The acceptance_cli tests solve both files, but whether any current test compares the two *models to each other* was not established.
- **Resolves it:** Grep/read of acceptance tests; if absent, one cross-encoding assertion.

## Q9 — Was the unverified remainder of the review surface clean?

- **Unclear:** The dedicated deep-finder/adversarial-verification stage of this review (per-dimension finders for algorithms, invariants, adversarial CLI/solver probing, plus a verification pass over every finding) was aborted by an external session limit before returning results (see `review/summary.md` § Method). Areas with **no dedicated pass**: algorithmic complexity/pathology analysis, `ein-einb/src/cast.rs` unsafe audit beyond its stated invariants, hands-on fuzz-style probing of parser/CLI edges, micro-CSP ground-truth verdict checks, and adversarial verification of every reading-pass finding.
- **Why it matters:** Absence of findings in those areas is absence of evidence, not evidence of absence.
- **Resolves it:** Re-running the aborted pass; the findings here stand on the reading pass plus the three binary-verified reproductions.

## Q10 — Release matrix

- **Unclear:** The macOS/Windows/aarch64 release legs have never executed (release.yml's own admission); their correctness is untested until a first tag. Recorded under `review/tests/low.md`; kept here because no repository evidence can resolve it.
