# P1e.5 — Documentation, and other

**Estimate:** ~1 week for the first two stages; **~7 weeks** with the three
added on 2026-08-28 ([S1e.5.5](s1e.5.5_nl_required.md) 4 d,
[S1e.5.6](s1e.5.6_rule_priority.md) 6 d,
[S1e.5.20](s1e.5.20_docs_refactor.md) 23 d). The phase stays open-ended by
design (see [§ Scope](#scope)).
**Depends on:** nothing to start. [S1e.5.2](#stages) depends on
[Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
being ratified, which it is; S1e.5.6 depends **hard** on
[S1e.1.3](../p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md) and
S1e.5.20 **hard** on P1e.2 S1e.2.2 and P1e.3 S1e.3.7 / S1e.3.8.
**Blocks:** nothing.
**Source:** the user's instruction of 2026-08-28 — *"we need a comprehensive
config options reference document in `docs/kernel/` — make it a first stage in
the new phase p1e.5 for ein documentation only."* Three further notes of the
same day added S1e.5.5, S1e.5.6 and S1e.5.20, and the directory was renamed
`p1e.5_documentation_and_other` because **S1e.5.6 is engine work**: it removes
a language keyword and replaces it with a static analysis. The phase name is
now literal.

---

## Scope

**This phase writes documentation that does not exist.** It is not the doc
repair work — that is [P1e.2](../p1e.2_high/README.md)
[S1e.2.2](../p1e.2_high/s1e.2.2_code_doc_consistency.md) (triage
`docs/kernel` page by page) and [P1e.3](../p1e.3_medium/README.md)
[S1e.3.7](../p1e.3_medium/s1e.3.7_code_doc_consistency.md) /
[S1e.3.8](../p1e.3_medium/s1e.3.8_documentation.md) (fix what drifted). Those
process findings. This phase adds pages the review's absence-of-findings
could not have flagged, because *"absence of findings there is absence of
evidence"* ([Q9](../review/open-questions.md)) applies to documentation too.

One stage is specified. The others are named as candidates and left for the
user to shape — writing four speculative stage files would be the same
mistake as writing prose counts nobody generates.

## Why a config reference is the first one

Three facts, checked at `7731848`:

- **17 `(config …)` flags ship**, in
  [`config.rs`](../../../ein.rs/crates/ein-core/src/config.rs)'s `FIELDS`, and
  the only page that mentions the form at all is
  [`01_grammar.md:58`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md),
  which gives the shape `(config [:flag v]*)` and enumerates nothing.
  `grep -n 'enable-\|hypgen-\|lattice-order'` over
  [`features.md`](../../../docs/kernel/inference/features.md) — the page that
  is cited as *the measurement* for the lookahead — returns **nothing**. Two
  flags are described in `01_grammar.md`'s prose in passing; fifteen are
  documented only by a Rust doc comment, and four of those have none.
- **The `EIN_*` environment variables are not enumerable.** A grep finds
  twenty-four names; some are process environment read by the shipped binary
  (`EIN_STDLIB`, `EIN_TRAVERSAL`, `EIN_OBLIGATION_CHOICE`), some are read only
  by tests (`EIN_BLESS`, `EIN_ID_SEEDS`), some by scripts (`EIN_BIN`), and at
  least two are **Python local variables that merely look like env vars**
  (`EIN_RS` in `utils/e2e_baseline.py` and `utils/feature_matrix.py`). Nobody
  can currently answer *what does `ein` read from the environment* without
  doing this exercise, and [AGENTS.md](../../../AGENTS.md)'s gate block lists
  six of them as if that were the set.
- **The one column that matters is missing everywhere.** *Does this knob
  change the answer?* [F4 Q40](../../followups/f4_cross_cutting.md) says at
  least one does — `enable-pre-branch-lookahead` decides a verdict — and
  [Q7](../review/open-questions.md) says at least one is undefined (`-n 0`).
  A reference that lists flags without that column would be a table, not a
  contract.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e.5.1](s1e.5.1_config_reference.md) | The configuration reference | 3 d | one page in `docs/kernel/` covering the 17 flags, the live `EIN_*` set and the CLI options that shadow them, with a **does it change the answer** column — and a test that fails when the flag list drifts from it |
| S1e.5.2 | What a solution is, and what a model is | **done 2026-08-28, ahead of the phase** | [`docs/kernel/inference/solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) — hypothesis / L1 / commitment / entering / integrated / solution / model / owes / exhausted, plus §6 on where the engine's `complete()` differs from the definition; ten new [glossary](../../../docs/kernel/glossary.md) entries under *Search and answers*; indexed from `inference/README.md` |
| *S1e.5.3* | *The read-out reference* | *proposed* | one page for every number `ein solve` prints — `k`, `solution_nodes`, `exhausted`, `owes`, `layers_explored` — and which of them is a count of *models* and which of *what the search did*. [SE-M1](../README.md#the-findings) and [AR-M2](../README.md#the-findings) are findings about exactly this confusion |
| [S1e.5.5](s1e.5.5_nl_required.md) | Every statement convertible to NL | 4 d | the register census — `zebra2`'s unique model is **444 facts and 25 of them (5 %) have a template**; fact-level rendering from **provenance** rather than from the relation; one page on the four registers; two mechanical checks in `cargo test` |
| [S1e.5.6](s1e.5.6_rule_priority.md) | Remove `:priority`; derive the order from the rules | 6 d | the control experiment banked (**137 of 139 corpus entries identical** with every `:priority` stripped from the stdlib and the corpus); the rule dependency graph and its strata; the static non-stratifiability diagnostic `01_grammar.md` has called future work since S1.7.4; 353 occurrences gone or a written refusal |
| [S1e.5.20](s1e.5.20_docs_refactor.md) | `docs/ein/`: the tree a released system would have | 23 d | `docs/history/` dissolved rather than deleted; `docs/ein/{user,reasoning,ein.rs}` as three perspectives; **≈1 989 history-referencing lines** across ~330 files resolved; and the two checks — a link checker and a release-voice check — that make it finishable and keep it finished |

*Proposed* means the stage is named and not specified. Say which you want and
it gets a file.

**S1e.5.5, S1e.5.6 and S1e.5.20 came from the user's notes of 2026-08-28** and
were written up on the 28th. Two of them are not *"documentation ein does not
have"*: S1e.5.6 is engine work, and S1e.5.20 rewrites documentation ein
already has. S1e.5.20 alone is **a fifth of M1e's whole estimate for work that
closes no finding**, and its § How to cut it carries the seven-stage split it
should probably become; taking the split is the user's call.

**S1e.5.2 was taken first and out of order**, on 2026-08-28, because
[Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
forbids a ruling that lives only in a plan file and
[Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
was decided the same day. It cites `defined_behaviour.md` and
`architecture_and_algorithms.md` by **anchor text rather than by section
number**, per this phase's third risk, so P1e.2's triage can renumber them
without breaking it.

## Acceptance

- Every page this phase adds is **pinned by something that runs**. The repo's
  rule, learned the expensive way and stated in
  [Q-M1e.4](../open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all):
  *every count a test pins is exactly right; every count only prose states has
  drifted.* A reference page is a list of names and defaults — the most
  drift-prone shape there is — so it ships with its diff test or it does not
  ship.
- Every page states its **date and its commit**, as the census documents do.
- No page duplicates a `docs/history/` page's subject. Where the material
  exists as history with a banner, the new page **links** rather than restates
  ([Q-M1e.3](../open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s
  rule).
- `RUSTDOCFLAGS="-D warnings" cargo doc` and `./run_tests.sh` green — the
  first because five of the phase's likely edits are doc comments, and the
  gate's own history is that nothing had ever run rustdoc until M1c S1c.1.5
  and it found nineteen defects on the first try.

## Risks

- **A reference page is a fourth hand-maintained copy.**
  `FIELDS` ↔ `rendered_fields` is already a pair, and it is a pair the code
  *diffs with a test*
  ([`config.rs`](../../../ein.rs/crates/ein-core/src/config.rs)
  `every_field_has_a_flag_and_no_flag_is_orphaned`). A prose table would be a
  third copy with no diff — [AR-M1](../README.md#the-findings) exactly. The
  stage's design constraint is therefore *generate or diff*, and
  [`docs/api/rust.md`](../../../docs/api/rust.md)'s marker-guarded region is
  the precedent that already works.
- **Documenting a knob blesses it.** Four of the seventeen flags are probes
  (`candidate-order-seed`, `lattice-order-seed`, `lattice-sanity-check`,
  `print-alive`) and one is a mode that returns a constant
  (`hypgen-scoring: most-constrained`, see
  [S1e.1b.5](../p1e.1b_hypothesis_structure/s1e.1b.5_ordering.md)). Writing
  them up as features makes them contract. The page needs a **stability
  column** as much as an answer column.
- **It overlaps [S1e.3.7](../p1e.3_medium/s1e.3.7_code_doc_consistency.md).**
  `defined_behaviour.md` is being amended in P1e.1 (Q3) and triaged in P1e.2;
  a new reference that cites its section numbers will cite the old ones.
  Sequence this phase **after** P1e.2 S1e.2.2, or cite by anchor text rather
  than by number.

## Connections

- [`docs/kernel/README.md`](../../../docs/kernel/README.md) — the tree this
  phase adds to, and the orientation page a new file has to be reachable from.
- [`docs/install.md`](../../../docs/install.md) — where `$EIN_STDLIB` is
  already documented, and the page the config reference must not contradict.
- [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)
  — the thirteen diagnostics and error strings. A rejected `(config …)` flag
  is one of them, and the reference points there rather than restating it.
- [`AGENTS.md`](../../../AGENTS.md) § Running the gate — the six environment
  variables an agent is currently told about, which S1e.5.1 either confirms as
  the whole live set or corrects.
