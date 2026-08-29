# P1f.5 — Documentation, and other

**Estimate:** ~7 weeks — 3 stages, **33 days**
([S1f.5.5](s1f.5.5_nl_required.md) 4 d,
[S1f.5.6](s1f.5.6_rule_priority.md) 6 d,
[S1f.5.20](s1f.5.20_docs_refactor.md) 23 d). The phase stays open-ended by
design (see [§ Scope](#scope)).
**Depends on:** nothing to start. S1f.5.6 depends **hard** on
[S1e.1.3](../../m1e_review_processing/p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md), which
is **done** (2026-08-29); S1f.5.20 depends **hard** on M1e's P1e.2 S1e.2.2 and
P1e.3 S1e.3.7 / S1e.3.8, none of which has run — so this phase's largest stage
waits on a milestone it is no longer part of.
**Blocks:** nothing.
**Was P1e.5**, in [M1e](../../m1e_review_processing/README.md), until 2026-08-29. Two of its six stages
had already shipped and **stayed there** with the ids they shipped under; what
moved is the four that had not.
**Source:** the user's instruction of 2026-08-28 — *"we need a comprehensive
config options reference document in `docs/kernel/` — make it a first stage in
the new phase p1e.5 for ein documentation only."* Three further notes of the
same day added S1f.5.5, S1f.5.6 and S1f.5.20, and the directory was renamed
`…_documentation_and_other` because **S1f.5.6 is engine work**: it removes a
language keyword and replaces it with a static analysis. The phase name is now
literal. The instruction says *"the new phase p1e.5"* because that is what it
was; the number moved on 2026-08-29 and the quote is not edited to match.

---

## Scope

**This phase writes documentation that does not exist.** It is not the doc
repair work — that is [P1e.2](../../m1e_review_processing/p1e.2_high/README.md)
[S1e.2.2](../../m1e_review_processing/p1e.2_high/s1e.2.2_code_doc_consistency.md) (triage
`docs/kernel` page by page) and [P1e.3](../../m1e_review_processing/p1e.3_medium/README.md)
[S1e.3.7](../../m1e_review_processing/p1e.3_medium/s1e.3.7_code_doc_consistency.md) /
[S1e.3.8](../../m1e_review_processing/p1e.3_medium/s1e.3.8_documentation.md) (fix what drifted). Those
process findings. This phase adds pages the review's absence-of-findings
could not have flagged, because *"absence of findings there is absence of
evidence"* ([Q9](../../m1e_review_processing/review/open-questions.md)) applies to documentation too.

Three stages are specified and one is a named candidate, left for the user to
shape — writing a speculative stage file would be the same mistake as writing
prose counts nobody generates.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| *S1f.5.3* | *The read-out reference* | *proposed* | one page for every number `ein solve` prints — `k`, `solution_nodes`, `exhausted`, `owes`, `layers_explored` — and which of them is a count of *models* and which of *what the search did*. [SE-M1](../../m1e_review_processing/README.md#the-findings) and [AR-M2](../../m1e_review_processing/README.md#the-findings) are findings about exactly this confusion |
| [S1f.5.5](s1f.5.5_nl_required.md) | Every statement convertible to NL | 4 d | the register census — `zebra2`'s unique model is **444 facts and 25 of them (5 %) have a template**; fact-level rendering from **provenance** rather than from the relation; one page on the four registers; two mechanical checks in `cargo test` |
| [S1f.5.6](s1f.5.6_rule_priority.md) | Remove `:priority`; derive the order from the rules | 6 d | the control experiment banked (**137 of 139 corpus entries identical** with every `:priority` stripped from the stdlib and the corpus); the rule dependency graph and its strata; the static non-stratifiability diagnostic `01_grammar.md` has called future work since S1.7.4; 353 occurrences gone or a written refusal |
| [S1f.5.20](s1f.5.20_docs_refactor.md) | `docs/ein/`: the tree a released system would have | 23 d | `docs/history/` dissolved rather than deleted; `docs/ein/{user,reasoning,ein.rs}` as three perspectives; **≈1 989 history-referencing lines** across ~330 files resolved; and the two checks — a link checker and a release-voice check — that make it finishable and keep it finished |

*Proposed* means the stage is named and not specified. Say which you want and
it gets a file.

**Two stages are missing from that table on purpose: they shipped, and they
stayed in M1e.** `S1e.5.1` (the configuration reference) and `S1e.5.2` (what a
solution is, and what a model is) both landed on 2026-08-28, under M1e's
numbering, and five places in the tree cite `S1e.5.1` as the stage that shipped
[`configuration.md`](../../../docs/kernel/configuration.md) — `AGENTS.md`, the
page itself, `ein-core/src/config.rs` and `ein-cli/tests/config_reference.rs`.
Renumbering them would have made those citations false, so
[M1e's P1e.5](../../m1e_review_processing/p1e.5_documentation_and_other/README.md) keeps them and this
phase starts at the first stage that had not run.

**S1f.5.5, S1f.5.6 and S1f.5.20 came from the user's notes of 2026-08-28** and
were written up on the 28th. Two of them are not *"documentation ein does not
have"*: S1f.5.6 is engine work, and S1f.5.20 rewrites documentation ein
already has. S1f.5.20 alone is **23 of this phase's 33 days**, and it was a
fifth of M1e's whole estimate for work that closed none of its findings — which
is a large part of why it is no longer in M1e. Its § How to cut it carries the
seven-stage split it should probably become; taking the split is the user's
call.

**What those two shipped, and why it still matters here.** S1e.5.1 corrected
five of this file's own reconnaissance numbers — see its
[§ What shipped](../../m1e_review_processing/p1e.5_documentation_and_other/s1e.5.1_config_reference.md#what-shipped--2026-08-28-at-4a47aa3).
The two most consequential are that **`enable-symmetric-mirror` changes the
answer** (`features.md`'s `1.0×` is a claim about `zebra2`, where the mirror
has a rule fallback; the one fixture that reaches the mirror derives 0 facts
instead of 3 without it) and that two of the four flags the reconnaissance
called *probes* are not probes but **inert**. That reconnaissance stayed with
M1e — [§ Why a config reference was the first one](../../m1e_review_processing/p1e.5_documentation_and_other/README.md#why-a-config-reference-was-the-first-one)
— because the stage that corrected it is there too, and a record of a
correction needs the thing it corrected beside it.

S1e.5.2 cites `defined_behaviour.md` and `architecture_and_algorithms.md` by
**anchor text rather than by section number**, per this phase's third risk, so
M1e's P1e.2 triage can renumber them without breaking it — which is a
constraint this phase inherits and S1f.5.20 has to keep.

## Acceptance

- Every page this phase adds is **pinned by something that runs**. The repo's
  rule, learned the expensive way and stated in
  [Q-M1e.4](../../m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all):
  *every count a test pins is exactly right; every count only prose states has
  drifted.* A reference page is a list of names and defaults — the most
  drift-prone shape there is — so it ships with its diff test or it does not
  ship.
- Every page states its **date and its commit**, as the census documents do.
- No page duplicates a `docs/history/` page's subject. Where the material
  exists as history with a banner, the new page **links** rather than restates
  ([Q-M1e.3](../../m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s
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
  third copy with no diff — [AR-M1](../../m1e_review_processing/README.md#the-findings) exactly. The
  stage's design constraint is therefore *generate or diff*, and
  [`docs/api/rust.md`](../../../docs/api/rust.md)'s marker-guarded region is
  the precedent that already works.
- **Documenting a knob blesses it.** Four of the seventeen flags are probes
  (`candidate-order-seed`, `lattice-order-seed`, `lattice-sanity-check`,
  `print-alive`) and one is a mode that returns a constant
  (`hypgen-scoring: most-constrained`, see
  [S1f.10.5](../p1f.10_hypothesis_structure/s1f.10.5_ordering.md)). Writing
  them up as features makes them contract. The page needs a **stability
  column** as much as an answer column.
- **It overlaps [S1e.3.7](../../m1e_review_processing/p1e.3_medium/s1e.3.7_code_doc_consistency.md).**
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
