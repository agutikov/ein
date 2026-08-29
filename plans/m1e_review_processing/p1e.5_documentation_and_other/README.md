# P1e.5 — Documentation, and other

**Estimate:** — · **Shipped 2026-08-28**, both stages, in one day.
**Depends on:** nothing. S1e.5.2 depended on
[Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
being ratified, which it was, the same day.
**Blocks:** nothing.
**Source:** the user's instruction of 2026-08-28 — *"we need a comprehensive
config options reference document in `docs/kernel/` — make it a first stage in
the new phase p1e.5 for ein documentation only."*

---

## This phase is what it delivered, and the rest of it left

**On 2026-08-29 four of this phase's six stages moved to
[M1f](../../m1f_hypothesis_and_documentation/README.md) as
[P1f.5](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/README.md)**
— on the user's instruction, and along the line M1e's own phase table had
already drawn: this phase and P1f.10 were marked **† not review processing**,
*"additive and may be cut whole"*. They were not cut; they were given an
M-number.

**Two stages did not move, because they had already run.** `S1e.5.1` and
`S1e.5.2` shipped under M1e's numbering on 2026-08-28, and five places in the
tree cite `S1e.5.1` as the stage that shipped
[`configuration.md`](../../../docs/kernel/configuration.md) —
[`AGENTS.md`](../../../AGENTS.md) (×2), the page itself (×2),
[`config.rs`](../../../ein.rs/crates/ein-core/src/config.rs) and
`ein-cli/tests/config_reference.rs`. Renumbering a shipped stage to match a
directory it moved into would have made all five false, so the phase keeps what
it delivered and M1f starts at the first stage that had not run.

## What shipped

| ID | title | ends with |
|---|---|---|
| [S1e.5.1](s1e.5.1_config_reference.md) | The configuration reference | **done 2026-08-28** — [`docs/kernel/configuration.md`](../../../docs/kernel/configuration.md): the 17 flags with default · what it changes · **answer?** · stability, the **52** CLI options (the golden's count, not the plan's 50) and the **six** that shadow a flag (not eight), the `EIN_*` census in four classes of which the last holds **nine** names that are not environment variables. Pinned by `ein-cli/tests/config_reference.rs`, six tests: the defaults block **is** `--dump-config`'s output, `EIN_BLESS=1` re-banks it. Found and filed: [Q-M1e.10](../open_questions.md#q-m1e10--two-config--flags-are-inert), `print-alive` and `candidate-order-seed` read by **no code path** |
| S1e.5.2 | What a solution is, and what a model is | **done 2026-08-28, ahead of the phase** — [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md): hypothesis / L1 / commitment / entering / integrated / solution / model / owes / exhausted, plus §6 on where the engine's `complete()` differs from the definition; ten new [glossary](../../../docs/kernel/glossary.md) entries under *Search and answers*; indexed from `inference/README.md` |

**S1e.5.2 was taken first and out of order**, because
[Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
forbids a ruling that lives only in a plan file and
[Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
was decided the same day. It has no stage file: it was a table row here and a
page in the tree, which is the whole of its record. It cites
`defined_behaviour.md` and `architecture_and_algorithms.md` by **anchor text
rather than by section number**, so P1e.2's triage can renumber them without
breaking it.

## What left

| went as | title | est. |
|---|---|---:|
| [S1f.5.3](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/README.md#stages) | *The read-out reference* — proposed, no file | — |
| [S1f.5.5](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.5_nl_required.md) | Every statement convertible to NL | 4 d |
| [S1f.5.6](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md) | Remove `:priority`; derive the order from the rules | 6 d |
| [S1f.5.20](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.20_docs_refactor.md) | `docs/ein/`: the tree a released system would have | 23 d |

**33 days left M1e with them**, none of which closed one of the 63 findings —
which is why the split is not a loss to this milestone. What M1e keeps of that
work is the dependency in the other direction: `S1f.5.20` cannot start until
M1e's `S1e.2.2`, `S1e.3.7` and `S1e.3.8` have run.

## Why a config reference was the first one

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
  [Q7](../review/open-questions.md) said at least one is undefined (`-n 0`) —
  **answered 2026-08-29**: it was `-n 1`, and it is refused now
  ([S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md)).
  A reference that lists flags without that column would be a table, not a
  contract.

**Three of those bullets were corrected by the stage they justified**, which is
the entry worth keeping them for: see
[S1e.5.1 § What shipped](s1e.5.1_config_reference.md#what-shipped--2026-08-28-at-4a47aa3).
The two that matter most are that **`enable-symmetric-mirror` changes the
answer** — `features.md`'s `1.0×` is a claim about `zebra2`, where the mirror
has a rule fallback, and the one fixture that reaches it derives 0 facts
instead of 3 without it — and that two of the four flags called *probes* above
are not probes but **inert**
([Q-M1e.10](../open_questions.md#q-m1e10--two-config--flags-are-inert)).

## Acceptance, and the risks that went with the four stages

These were the phase's, and P1f.5 inherits them unchanged.

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
  [S1f.10.5](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.5_ordering.md)). Writing
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
