# Ein examples

Encoded puzzles and focused fixtures, in [ein-lang](../docs/kernel/ir/03-ein-lang/).
Run any with `ein solve <file>` (or `ein saturate <file>` for the
saturation demos); see [`docs/api/rust.md`](../docs/api/rust.md) to drive them
from another Rust program.

> The step-by-step **human Zebra walkthrough** (the M1 target trace) used to
> live here; it moved to
> [`docs/kernel/inference/zebra_walkthrough.md`](../docs/kernel/inference/zebra_walkthrough.md).

> **The stdlib conformance corpus is not here.** M1c
> [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus)
> added 45 programs — one per `std.*` rule or tight family, each stating what
> it should and should not derive — and they live in
> [`tests/stdlib/`](../tests/README.md). They are a suite rather than a set of
> things to read: each exists to break, most are three declarations and two
> facts, and nobody would learn ein-lang from them. What *is* here is the three
> `features/1{0,1,2}_expect*.ein` fixtures, which demonstrate the form they are
> written in.

## Zebra puzzle — two ontologies, one puzzle

`zebra.ein` and `zebra2.ein` are not "classic vs modern". They encode the
*same* Zebra puzzle over deliberately different vocabularies, both solve to the
same model, and the pair is kept because the comparison is the only way to see
which of the engine's reasoning power is general and which is an artefact of one
encoding.

**What pins that they agree cell by cell** — M1e
[S1e.1.6](../docs/history/m1e_review_processing/README.md#s1e16--what-nothing-pins-q8-q9-q10),
the review's `Q8`:

| test | what it holds |
|---|---|
| `ein-infer/tests/acceptance.rs::the_generic_link_encoding_is_the_unique_solution` | `zebra.ein`'s model places all 25 `GRID` cells through `co-located`, at `k = 1` and `exhausted` |
| `…::both_ontologies_reach_the_same_model` | `zebra2.ein`'s model is **exactly** those 25 cells, read through the five `*-loc` projections |

`GRID` is the *published* answer, vocabulary-independent, and both encodings
are compared against **it** rather than against each other — which is the
stronger claim of the two, because two encodings can agree on a wrong model.
**The design comparison is `C2`, and it is in git history** — the S1.22.1a
report `c2_zebra_ein_gap.md`, which lived under `plans/m1_core_graph_reasoning/`
and went when that tree was deleted at P1.22. Read it with

```sh
git show ff1d6c5^:plans/m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/reports/c2_zebra_ein_gap.md
```

> **Why not `docs/history/`** (M1e `CD-M6`, and the decision
> [`DO-M2`](../docs/history/m1e_review_processing/README.md#s1e38--documentation-medium)
> cites for the dangling class): putting it there would mean creating
> `docs/history/m1_core/`, and
> [Q-M1e.3](../docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s
> rule is that *a page is moved **into** a milestone record, never made into
> one* — M1 is the one shipped milestone with no entry, by the decision
> [`docs/history/README.md`](../docs/history/README.md) records: *"what
> survived its plan tree went to `docs/kernel/inference/` and
> `plans/followups/` at P1.22, and the rest is in git history."* So the two
> halves of C2 that are still read went where they are read — its §5(ii)
> anchoring argument into
> [`stdlib/README.md`](../stdlib/README.md), and its §0 measurements are
> superseded by the table above. What is left in git is the
> *reasoning*: §3 weighs the four ways the property could have been stated and
> says why three were rejected, §5 prices the four things that made the
> encoding fast. A bare **C2** with no link and no location was the third
> state, and it is the one this replaces.

| | [`zebra.ein`](zebra.ein) | [`zebra2.ein`](zebra2.ein) |
|---|---|---|
| attribute link | **one generic** `co-located` equivalence over all 30 values, whose classes *are* the houses | **five typed** projections (`color-loc : Color → House`, …) |
| membership | split: `(instance V T)` + `(type Sub Super)` | unified `is-a` |
| a cross-attribute clue | an ordinary fact: `(co-located Englishman Red)` | a 4-ary activator: `(co-located nation-loc Englishman color-loc Red)` |
| a spatial clue | an ordinary fact: `(right-of Green Ivory)` | a 5-ary activator: `(adjacent-via right-of color-loc Ivory color-loc Green)` |
| the property that drives it | type-scoped: `(slot-partition co-located instance type Attribute House)` + one `(slot-spatial …)` per spatial relation — `std.slots` | per-relation: `(bijective color-loc)` ×5 — `std.bijection` |
| rules defined in the file | 0 (all imported) | `grep -c '^(rule ' <file>` |
| `solve --exhaustive` | Solution, k=1, exhausted — **46.9 ms** | Solution, k=1, exhausted — **31.1 ms** |

*(End-to-end, release build, one pinned P-core —
[scaling.md §1](../docs/history/m1a_rust/measurements/scaling.md), taken
2026-08-20. **Re-taken 2026-09-01** on the same box with
`utils/bench_env.sh` and `ein solve <file> -e -t`, median of five: **44.1 ms**
and **28.2 ms**, verdict, `k` and `exhausted` unchanged. Both rows are the
walk's `-e`, which no command runs since M1e2 S1e2.1a.5; `ein solve <file>
--single` decides the same uniqueness in 3.8 s on `zebra2.ein` (2026-10-18).)*

`zebra2.ein` remains the **primary M1 acceptance target** (it also carries the
Ambiguity and Contradiction task-class variants below);
`zebra.ein` is the independent second reading.

| file | description |
|------|-------------|
| [`zebra2-hints.ein`](zebra2-hints.ein) | `zebra2` with solution hints injected (S1.5a.11 diagnostic) |
| [`zebra2-minus-15.ein`](zebra2-minus-15.ein) | `zebra2` with condition (15) removed — a reduced, under-determined variant |
| [`zebra2-obligations.ein`](zebra2-obligations.ein) | `zebra2` with the `(hrule guess …)` and its `(solve … :hrules …)` form **removed** — the theory alone drives the search (M1d S1d.2.5's obligations rung) |
| [`zebra2-minus-15-obligations.ein`](zebra2-minus-15-obligations.ein) | both at once: under-determined *and* hypothesis-rule-free — 32 models, found the way the hrule path finds them |
| [`gen_zebra2_variants.py`](gen_zebra2_variants.py) | generator for the four `zebra2` variants — clue-dropped, clue-added and hrule-free. `--check` is what makes "nothing else changed" a test |

### What an edit to these two files fans out into

`zebra.ein` and `zebra2.ein` are **world anchors**: tests across the workspace
hard-code facts about them — counts, model cells, `k`, `enterings` — and their
docs say so, but only `embedding.rs` said what it costs. This is the list a
reviewer changing either file needs, and it is here rather than in the puzzle
headers because it moved twice in the two days after M1e S1e.1.6 first measured
it. **Both puzzle files point at this section**; nothing else has to be kept in
sync by hand.

Four rings, outermost first:

1. **Four generated files.** `zebra2.ein` is copied whole by
   [`gen_zebra2_variants.py`](gen_zebra2_variants.py) into
   `zebra2-minus-15.ein`, `ein-bugs/zebra2-bad.ein`, `zebra2-obligations.ein`
   and `zebra2-minus-15-obligations.ein`. Its `--check` runs inside `cargo
   test` (`cli_semantics`), so **an edit to `zebra2.ein` that is not
   regenerated turns the local gate red** — including an edit that is only a
   comment.
2. **Test files that name one of these paths**, re-derived by
   `world_anchors::the_anchor_list_is_the_greps_own_answer` and banked below.
   Edit the test, run it, paste — or `EIN_BLESS=1` writes it.

<!-- generated: grep -rl 'examples/zebra' ein.rs/crates/*/tests/*.rs ein.rs/crates/*/benches/*.rs -->
```text
ein-cli/tests/acceptance_cli.rs
ein-cli/tests/answer_dump.rs
ein-cli/tests/cli_semantics.rs
ein-cli/tests/corpus_cli.rs
ein-cli/tests/einb_cli.rs
ein-cli/tests/embedding.rs
ein-cli/tests/models_grid.rs
ein-cli/tests/region_dump.rs
ein-cli/tests/render_doors.rs
ein-corpus/benches/engine.rs
ein-einb/tests/corruption.rs
ein-einb/tests/invalidation.rs
ein-einb/tests/roundtrip.rs
ein-infer/tests/acceptance.rs
ein-infer/tests/explain_semantics.rs
ein-infer/tests/naf_semantics.rs
ein-infer/tests/obligation_reports.rs
ein-infer/tests/obligation_rung.rs
ein-infer/tests/region.rs
ein-infer/tests/search_semantics.rs
ein-infer/tests/stdlib_coverage.rs
ein-infer/tests/worker_view.rs
ein-ir/tests/dump_goldens.rs
ein-ir/tests/imports_semantics.rs
ein-ir/tests/kb_semantics.rs
ein-render/tests/golden_dot.rs
ein-render/tests/idea08_acceptance.rs
ein-render/tests/presentation_semantics.rs
```
<!-- /generated -->

3. **Outside the crates**, which no test can re-derive: `docs/api/rust.md`
   (through `embedding.rs`'s marked region), the `utils/` scripts that name a
   zebra path (`grep -l 'examples/zebra' utils/*`), and `corpus/corpus.toml`.
4. **Goldens.** `ein-ir/tests/golden/zebra.golden` and `zebra2.golden`, and
   `ein-render/tests/golden/kb_zebra_unified.dot`, are byte goldens of the two
   puzzles. **An edit to `zebra.ein` or `zebra2.ein` that changes the *parse*
   moves them**, and `EIN_BLESS=1` re-banks them like any other golden — read
   the diff first. A `;`-comment does not: comments are lexer trivia and reach
   no golden, which is why the headers those two files gained at M1e S1e.4.5
   moved nothing.

## Feature fixtures (per engine capability)

| dir | what it exercises |
|-----|-------------------|
| [`features/`](features/) | language features: `not`/`absent`, `*` in identifiers, `forall`, `open`, stdlib domain-elimination (closing `is-a` since M1e2 S1e2.2.5, as `12` closes `instance`, so neither refutation rests on an `absent` a commitment can fill), the `__symmetric__` kernel mirror, the unstratifiable `p ← absent q; q ← absent p` (which pins that the NAF boundary admits **one** candidate per round — a batch would derive both), two `(or …)` disjuncts with different guards and one binding key (the only fixture where a parked candidate is already fired when the boundary reaches it), and an `adjacent-via` constraint satisfied from the *same* house — the 2026-08-20 `disjunctive-prune` regression. **`10_expect.ein` and its two siblings are the ones that are not like the others**: each carries an `:expect` and so states its own answer, which makes `ein solve` on it a test rather than a demonstration (M1c [S1c.1.2](../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)) — one per verdict, `10` being `k = 1`, `11_expect_ambiguity.ein` `k > 1` and `12_expect_false.ein` `k = 0`. A fourth joined them at M1e [S1e.3.1](../docs/history/m1e_review_processing/README.md#s1e31--correctness-medium): **`13_mixed_solution_and_open.ein`** is the `Solution` arm's *mixed* regime — one discharged model beside one open state, so `verdict.k` is 1 where `stats.solution_nodes` is 2 — which `finalise` defined and no program had reached, and which the table used to render as `solutions (k) 2` beside `verdict Solution`. **`ein test examples/features/` runs exactly those four** and never enters the search on the other nine, `04_open.ein` included ([S1c.1.3](../docs/history/m1c_external_validation/README.md#s1c13--ein-test)) |
| [`branching/`](branching/) | the hypothesis loop: saturate-only, dead/alive branches, multi-level, lookahead on/off, kill-cache on/off, `hrule`, hypothesis-relation whitelist, typed blind solve, the lookahead's NAF world and its unjudgeable guard (P1.21 R4 / S1.21.8 D3), refusal fixtures since M1e2 S1e2.2.5 (*forbid*: `13`'s rule can never fire and is refused by its shape, `14`'s `absent` is at depth 3); and `15`+`16`, M1e's on/off pair where **both halves under-report** — `Solution k=1` against `Contradiction k=0`, both exhausted, and the hand-derived answer is k=2 |
| [`saturation/`](saturation/) | per-rule saturation demos by family — symmetric, transitive, `implies`, square fwd/bwd/unique, type-exclusivity, hypothesis-contradiction (see [`saturation/README.md`](saturation/README.md)) |
| [`lattice/`](lattice/) | commitment-lattice search: subset-pruned, genuine 3-set death, state-hash collision; and since M1e2 S1e2.1a.1 two **region** fixtures — `04_obligation_trigger_hole.ein`, a hole in the solution region with no `absent` in any saturation rule (an obligation's trigger grows with the commitment), and `05_owed_seats_bounded.ein`, `owed-seats` with both upper bounds, whose region is two isolated points; and since S1e2.1a.8 **`synth/`**, programs **written for a chosen region** by [`utils/region_synth.py`](../utils/region_synth.py) — three components with holes, a trivial maximal line, one model per component, merged states, a hole of each kind — each carrying its target in its header, which `ein-infer/tests/region.rs` holds the oracle to node for node. Generated: regenerate, never edit — but for `three_components.ein` and `trigger_hole.ein`, frozen since M1e2 S1e2.2.5 as W3's witnesses: their dead holes rest on an `absent` the hrule can fill, which *forbid* refuses, and the generator now writes such a hole owing; and two **seating** problems the user asked for on 2026-10-16 — `06_friends_tables.ein`, two groups of friends (2, 3) at two tables (2, 3 seats) with one guest avoiding the air-conditioner, where a group that does not fit goes elsewhere (19 candidates; the obligation is the group's, so the program is not monotone: 21 isolated solutions and 166 holes), its monotone twin `06_friends_tables_everyone_sits.ein` (everyone must sit: 8 solutions, the table choice forced), and `07_two_pairs_two_tables.ein`, where it is free: 8 components in 2 projection classes; and since M1e2 S1e2.1b.6 one fixture per **cause** of a non-monotone program, each the program's only site — `08_absent_witness_hole.ein` (an `absent` that takes a witness away, concluding no refutation), `04` (an obligation's trigger) and `09_activator_hole.ein` (a rule derives an activator, so a commitment compiles a plan root never had: `04`'s cube by another mechanism, and the corpus's first exercised activator site) — plus `synth/maximal_not_top.ein`, obligations only with a maximal line that is not every candidate, and `synth/local_not_global.ein`, where across its two holes the one-fact test calls ∅ maximal and {A,B} minimal ([`docs/kernel/inference/monotonicity.md`](../docs/kernel/inference/monotonicity.md)); and since M1e2 S1e2.1b.7 the **FIFO gap** — `10_fifo_gap.ein`, two `absent` rules at one priority on a stratified program, where `{a, b}` built at once holds a stale `p` and built by way of `{a}` does not (the state depends on the path, every node is in R), and `11_fifo_gap_static.ein`, the same race over a relation the static check calls static: a *monotone* program whose node built from scratch holds a stale fact the fork does not; and since M1e2 S1e2.2.17 `12_activator_chain_hole.ein`, `09`'s cube one activator further from root (opening the kitchen derives `(serves K1)`, whose plan derives the `total-owed` activator), on which the status line named no site until that stage: the attribution read the obligation's own activator site, and `total-owed` is none, its activator relation written only by a plan root never compiled; it follows the activator to the plan that derived it since, and names `serves` |
| [`domain_elim/`](domain_elim/) | domain-elimination vs hypothesis measurement fixtures (see [`domain_elim/README.md`](domain_elim/README.md)) |
| [`syntax/`](syntax/) | seventeen **node-kind probes** — one per shape the renderers draw and no puzzle contains: the two `=` arities, an arity-0 relation, every argument kind, stored negation over four shapes, the `is-a` subject positions, `(relation …)` at every arity as a nested value, a full `(query …)`, a chained `(trace …)` and an empty one, `(config …)`, the three S1.7c.4 wrapper heads as ordinary facts, five rule shapes (nested `absent`, `forall`/`not`/`eq`, top-level `or`, disjoint guard sets), and the constraint-scope markers. The eighteenth — the two half-specified declarators — is `broken/load/rule_half_declarators.ein`, because it is a load-negative as well as a probe. As corpus entries the manifest digests them under **every** view and every op |

## Diagnostics & negative fixtures

| dir | what it holds |
|-----|---------------|
| [`ein-bugs/`](ein-bugs/) | contradiction / bug-repro puzzles (`zebra2-bad.ein` — injected-fact contradiction; `mixed-type-hypothesis.ein` and `nested-fact-hypothesis.ein` — hypothesis candidates the layer-1 sort of `alive` has to compare across value kinds (an int against a symbol) and between two nested facts, which `Value`'s total order answers, the second needing no mixed types at all; `unbound-relation-head.ein` and `unbound-assert-var.ein` — `(?R ?x)` with `?R` unbound, and an `(or …)` whose arms bind different variables so one reaches the `:assert` unbound: two refusals at exit 1, the second spelled `KeyError: "…"`; `query-goal-free-head.ein` and `query-goal-free-head-unsat.ein` — `(query :goal (?R Rex Animal))`, two lines, a program the engine ran until M1a S1a.6.6 and now refuses where it renders a model, plus the `Contradiction` arm that exits **0** because no solution block is rendered and so the goal is never compiled; `int-goal-binding.ein` and `fact-goal-binding.ein` — the two goal-binding shapes the M1a S1a.6.6 fuzzer found, an integer binding that must stay a JSON number and a nested-fact binding that renders as an s-expression string; and three M1e witnesses that the engine **recorded a state its own rules refute** — `alive-empty-phase1.ein`, `alive-empty-interlayer.ein` and `complete-records-stale.ein`, one per `record_node` caller, each answering `Solution`/`Ambiguity` on a model that re-saturated to `Contradiction`. **Fixed by M1e S1e.3.1 (`CO-M1`)**, and each `:expect` now states the right answer (`(false)`, `(false)`, `(model (p A) (p B))`); and the M1e pair `alt-cap-core.ein` / `alt-cap-core-reordered.ein`, one `:priority` apart, which report a **3-fact** and a **2-fact** unsat core for the same facts and the same rules — `MAX_ALT_JUSTIFICATIONS = 32` retains by premise count where the explanation search minimises frontier size, so a full list refuses the smaller explanation: S1e.1.3 / Q-M1e.15; and `naf-upward-closure.ein` — the twenty lines on which **`dead` is not upward-closed under `absent`**, `{(p A)}` dying while `{(p A), (q A)}` would live, so the walk never generated the maximal state: M1e Q-M1e.9. **Refused since M1e2 S1e2.2.5** (*forbid*, ruled 2026-11-02): every command that takes the universe exits 1 naming `bad` and the forms that say what was meant, and it is W3's witness of the refusal, as `alive-empty-interlayer.ein`'s `totality` is W1's; and its M1e S1e.3.3 sibling, the pair `alive-set-fresh-name.ein` / `alive-set-fresh-name-declared.ein`, which differ by **one fact** — `(seen Z)`, over a relation nothing else mentions — and, until M1e2 S1e2.2.4, by the whole answer: the first said `k = 0, exhausted = true`, *No solution*, and then R = {∅}, where `{(q A Z), (q B Z)}` is the maximal state, because a rule names `Z` and no fact does, and the candidate objects were the KB's names; the second names `Z` in a fact and reached that state. A rule's constant is a name of the program (Q-M1e.21 (b)), the candidates are seeded with it, and the two describe one R. The **M1 alive-set invariant** (`ST-M1`, Q-M1e.21) measured rather than argued, and neither file contains an `(absent …)`, so nothing is confounded with Q-M1e.9; and `kwpair-below-the-filter.ein` — four lines, `trace` mutated to `tracex` by the fuzzer, on which `render constraints` and `render program` exited 101 on a keyword pair below an undeclared form, until M1e2 S1e2.3.2 made `value_label` label one `:key value` (the user's ruling (a) of 2026-11-03); and two M1e2 S1e2.3.4 pins of surfaces that are gone, `d3-goal-row-order.ein` — one model that satisfies the goal twice, whose printed row moved under a permuted interner until the read-out printed every row sorted — and `nested-core.ein` — an unsat core holding a nested fact, once printed as a struct repr and now `(not (p A))`; and M1e2 W7's family (S1e2.2.7, Q-M1e.16), `defined_behaviour.md` § 3.2's program cut four ways: `binding-key-int-only.ein` and `binding-key-nested-only.ein` keep one of its two activators each, an `int` and a nested fact in one position, and are the controls, whose `noted` facts the pair with both activators, `binding-key-both.ein` and `binding-key-swapped.ein`, reach in either order since S1e2.2.7 gave a rule application's identity the activator's every argument; until then the first order lost `(noted 1 3)` in a release build and a debug build aborted on both, which is why the pair was banked under `corpus/fuzz_findings/` until the fix promoted it; and M1e2 W11's three witnesses, promoted from `corpus/fuzz_findings/` by S1e2.2.12 (2026-11-04): `unsat-core-id-space.ein` and `d3-unsat-core.ein`, the engine fuzzer's two finds whose walk core moved with the interner (the second refused since S1e2.2.5), and `fork-delta-order.ein`, written for the mechanism the second visit found, a dead node's four-fact core that moved with the interner until a resumed fork's delta took the order its commitment was written in; and M1e2 W16's family (S1e2.2.16, 2026-11-04): `activator-text-int-only.ein` and `activator-text-string-only.ein`, the controls of the order pair whose activators are the int `1` and the string `"1"`, which keyed one compile plan, so the second's two `noted` facts were lost, and on which a debug build aborted, so the pair, `activator-text-int-and-string.ein` and `activator-text-string-then-int.ein`, was banked under `corpus/fuzz_findings/` until S1e2.2.16's fix promoted it, the key being the activator's values since; and W16's second shape, `activator-all-int.ein` and `activator-all-nested.ein`, an activator whose every argument is an int or a nested fact, which *forbid* read as no activator, so a refutation it refuses on the symbol control, `activator-all-symbol.ein`, loaded until the fix. **Those of them that still answer wrongly are rows of [`corpus/wrong_answers.toml`](../corpus/wrong_answers.toml)**, M1e2's register, with the line below) |
| [`ein-bugs/wrong-answers/`](ein-bugs/wrong-answers/) | **witnesses of known wrong answers** (M1e2 S1e2.1.1), one hundred and ten programs of a few lines each that were probes under `plans/m1e2_correctness/probes/` and are banked because a register row cannot point into a plan directory. What each one answers and should answer is [`corpus/wrong_answers.toml`](../corpus/wrong_answers.toml)'s, not this line's: the `forced-*` family (W1 at layer 0 and W2, the forced-positive cascade refuting a consistent program), `owed-two-step` / `tree-dead` (W1 turning an `Open` into a `Contradiction`), `fresh-name-{default,nospawn}` (W4 separated from W1), `naf-nowb` (W3's lever row), `owed-seats{,-renamed}` (W5), `uncovered{,-knows}` and `arity3` (W6), `stale-naf-fork{,-warn}` and `f07-fpoff` (W8; the first two refused since M1e2 S1e2.2.5 through F2's *via*), `match-only{,-declared}` and `negative-only{,-declared}` (W15, M1e2 S1e2.2.14: a name written only in a `:match`, or only inside a nested fact, and each witness's control), `oblig-fresh-name{,-two}{,-declared}`, `oblig-derived-domain{,-declared}` and `oblig-stdlib-total-owed{,-declared}` (W14, M1e2 S1e2.2.15: an obligation's domain value that only a commitment derives, which the obligations rung never proposes, and each witness's control), `alt-cap-shared` (W10's second witness, M1e2 S1e2.2.11: a saving between two facts behind a full alternatives list, deferred to F22), and the `forbid-chain-*` family (W17, M1e2 S1e2.2.17: a refutation resting on an `absent` through an activator chain that first appears at a node, which *forbid* missed and refuses since the stage's fix — `-1` and `-2`, the chain one and two links past D, and `-node-absent`, `-param-absent` and `-node-writes`, the same mechanism on the `absent`'s side; `-0`, one link shorter and refused since S1e2.2.5, and `-no-absent`, which loads with no hole, are the controls), and the `str-in-*` family (W18, M1e2 S1e2.2.18: a string written in a rule, which the rule compiler did not lower as the loader lowers it in a fact until the stage's fix — `-match`, a premise that never matched, `-assert`, a conclusion that stopped the run, `-guard`, an `eq` that never held and a `neq` that always did, `-absent`, an `absent` that always held, and `-hrule`, an hrule that proposed nothing), and the `core-activator-*` family (W19, M1e2 S1e2.2.19: a dead node's core that left out the activator its refutation rests on until the stage's fix — `-committed`, the commitment is the activator, and `-guard-only`, a plan that matches no fact, so the core was empty; `forbid-chain-no-absent` is the witness, and `-given` and `-root-derived`, an activator the program states and one root derives, are the boundaries of Q-M1e2.12's ruling, (c), whose cores did not move), and the `wild-in-*` and `range-in-match` family (W20, M1e2 S1e2.2.20: a wildcard `_` or a range in a rule's pattern, which the rule compiler lowered as a slot that matched nothing until the stage's fix — `wild-in-match`, a premise that never matched, `wild-in-absent`, an `absent` that always held, and `wild-in-hrule`, an hrule that proposed nothing, each answering its `-twin`'s R since, `_` written as a variable the rule reads nowhere else; `range-in-match`, a range that loaded in silence, and `wild-in-head`, a head `_` that stopped compilation in a Python spelling, both refused at load since; `wild-in-head-twin` writes the head out as the disjunction over the program's relations), and the `pair-core-root-*` family (W21, M1e2 S1e2.2.21: a pair contradiction's core that named its positive side alone — `-given`, the positive side stated, and `-derived`, derived at root, so the core names none of the commitment; `-negative`, the negative side stated, is the boundary, a core that names the commitment and leaves out the stated side; and `forall-core-false` and `forall-core-elim`, the same defect through a `forall`, a core that left out the facts its universal read, until the stage's fix), and the `kw-*` family (W22, M1e2 S1e2.2.22: a name that starts with a keyword, which the parser read as the keyword and the rest of the name until the stage's fix — `kw-neq-in-assert`, `kw-or-in-assert` and `kw-and-in-assert`, a rule's conclusion; `kw-and-in-match` and `kw-neq-in-match`, a premise read as a conjunction and as a guard; `kw-neq-in-absent`; `kw-neq-in-fact`, a nested argument, and `kw-not-fact` and `kw-relation-fact`, a fact read as a negative and as a declaration; `kw-neq-in-goal`, `kw-or-in-hrule`, `kw-or-in-obligation`, `kw-and-in-expect` and `kw-or-in-solve`; each with its `-twin`, the name's first letter doubled, whose answer is the witness's right one), and the `core-search-cut` family (W23, M1e2 S1e2.2.23: a core search cut at its budget of 64 environments per fact, whose core was printed with nothing to say so until the stage's fix, and says so since — `core-search-cut`, the witness, whose root core is ten facts where eight suffice; `-control`, those eight alone, whose root core is inside the budget; `-5`, the shape with five pairs, under it), and seven programs of W24 (M1e2 S1e2.2.24: a rule shape that read nothing, in silence, until the stage's fix refused each at load — `wild-in-guard`, `_` as a guard's argument, which compared as a raw IR node; `bare-premise`, a premise that is not a form, which the rule compiler dropped; `nested-unbound-head`, a pattern inside a pattern whose relation no parameter binds, which matched nothing; `form-in-guard`, `kwpair-in-guard` and `guard-before-binder`, a form, a keyword pair and a variable no earlier premise binds as a guard's argument; `nested-fact-activator`, a relation parameter its activator binds to a nested fact), and the witnesses of the rows S1e2.2.25 filed `open` and deferred to their owners (M1e2 S1e2.2.25, the inventory of 2026-11-05's owned wrong answers — `f07-rules-swapped`, W8's, `features/07` with its two rules in the other order, whose R is the mirror of the original's; `guard-only-second` and its control `guard-only-first`, W25's, a premise-less derivation from an activator a node derived that is never recorded as an alternative; `closure-shared-given` and its `-control`, W10's, the closure estimate counting a shared given twice; `oblig-derived-activator` and its `-declared` control, W26's, an obligation whose activator a node derives, whose discharge is never a candidate), and `past-limit-lines` (W27, M1e2 S1e2.2.26: the smallest program the static check flags past the default `--region-limit`, 17 candidates and R three states, on which `--minimal` and `--maximal` labelled every member met with their line, where describe certifies {∅} minimal and says the maximal line is not known), and the `claim-int-name` family (W28, M1e2 S1e2.3.8: a claim and a query's projection classes compared as printed text, in which the int `1` and the name `"1"` were one — `claim-int-name`, a query's `:expect` about the int over a state holding the name, `-solve`, the same through the solve form, and `classes-int-name`, a goal answered by both, whose two classes are four; each witness's claim is false and stays as written). Every header carries the hand derivation of the right answer |
| [`broken/`](broken/) | curated **parse-failure** fixtures; each expects a `file:line:col` error (bare top-level atom, keyword-as-value, rule missing params, unclosed paren) |
| [`broken/load/`](broken/load/) | curated **load-failure** fixtures — files that parse and then fail the loader (`ein_ir::load`); each carries the exact `KbLoadError` message in a `.expected` beside it (see [`broken/load/README.md`](broken/load/README.md)) |
| [`broken/compile/`](broken/compile/) | curated **compile-failure** fixtures — files that parse and load and then hit a `CompileError` S1.22.0 or M1e S1e.2.1 turned from a silent misreading into an error; each carries the exact message in a `.expected` beside it (see [`broken/compile/README.md`](broken/compile/README.md)). Two of S1.22.0's four, `empty_absent` and `unbound_head`, moved to `broken/load/` at M1e2 S1e2.2.24, when the loader began refusing both. `activator_arity.ein` is the odd one: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
