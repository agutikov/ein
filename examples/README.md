# Ein examples

Encoded puzzles and focused fixtures, in [ein-lang](../docs/kernel/ir/03-ein-lang/).
Run any with `ein solve <file>` (or `ein saturate <file>` for the
saturation demos); see [`docs/api/rust.md`](../docs/api/rust.md) to drive them
from another **Rust** program. *This line said "from Python" until M1e
`CD-M6`, and `import ein` has not worked since M1a
[S1a.10.5](../docs/history/m1a_rust/README.md#s1a105--the-removal) —
[`docs/api/README.md` § There is no Python module](../docs/api/README.md) is
the standing statement.*

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
[S1e.1.6](../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.6_coverage_gaps.md),
the review's `Q8`; it was `acceptance/test_zebra_two_ontologies.py` until that
directory was deleted with `ein.py/`, and the pointer outlived it by six days:

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
> [`DO-M2`](../plans/m1e_review_processing/p1e.3_medium/s1e.3.8_documentation.md)
> cites for the dangling class): putting it there would mean creating
> `docs/history/m1_core/`, and
> [Q-M1e.3](../plans/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)'s
> rule is that *a page is moved **into** a milestone record, never made into
> one* — M1 is the one shipped milestone with no entry, by the decision
> [`docs/history/README.md`](../docs/history/README.md) records: *"what
> survived its plan tree went to `docs/kernel/inference/` and
> `plans/followups/` at P1.22, and the rest is in git history."* So the two
> halves of C2 that are still read went where they are read — its §5(ii)
> anchoring argument into
> [`stdlib/README.md`](../stdlib/README.md), and its §0 measurements are
> superseded by the table above, whose footnote already carries C2's own
> ~21 s / ~9 s as the PyPy figures they were. What is left in git is the
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
and **28.2 ms**, verdict, `k` and `exhausted` unchanged. These were ~21 s and
~9 s under PyPy before the port.)*

`zebra2.ein` remains the **primary M1 acceptance target** (it also carries the
Ambiguity and Contradiction task-class variants below);
`zebra.ein` is the independent second reading.

| file | description |
|------|-------------|
| [`zebra2-hints.ein`](zebra2-hints.ein) | `zebra2` with solution hints injected (S1.5a.11 diagnostic) |
| [`zebra2-minus-15.ein`](zebra2-minus-15.ein) | `zebra2` with condition (15) removed — a reduced, under-determined variant |
| [`zebra2-obligations.ein`](zebra2-obligations.ein) | `zebra2` with the `(hrule guess …)` and its `:hrules` clause **removed** — the theory alone drives the search (M1d S1d.2.5's obligations rung) |
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
ein-cli/tests/cli_semantics.rs
ein-cli/tests/corpus_cli.rs
ein-cli/tests/einb_cli.rs
ein-cli/tests/embedding.rs
ein-cli/tests/leftover_probe.rs
ein-cli/tests/summary_properties.rs
ein-corpus/benches/engine.rs
ein-einb/tests/corruption.rs
ein-einb/tests/invalidation.rs
ein-einb/tests/roundtrip.rs
ein-infer/tests/acceptance.rs
ein-infer/tests/explain_semantics.rs
ein-infer/tests/layer_census.rs
ein-infer/tests/naf_semantics.rs
ein-infer/tests/obligation_reports.rs
ein-infer/tests/obligation_rung.rs
ein-infer/tests/obligation_rung_control.rs
ein-infer/tests/search_invariants.rs
ein-infer/tests/search_semantics.rs
ein-infer/tests/stdlib_coverage.rs
ein-infer/tests/tree_traversal.rs
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
4. **Goldens — and three of them may never be re-blessed.**
   `ein-ir/tests/golden/from_ein_py/zebra.golden` and `zebra2.golden`, and
   `ein-render/tests/golden/from_ein_py/kb_zebra_unified.dot`, are the last
   independent provenance in the repo: bytes `ein.py` signed off on before it
   was deleted. **An edit to `zebra.ein` or `zebra2.ein` that changes the
   *parse* spends them**, and no test says so at the moment it happens — which
   is the sentence this section exists for. A `;`-comment does not: comments
   are lexer trivia and reach no golden, which is why the headers those two
   files gained at M1e S1e.4.5 moved nothing.

## Feature fixtures (per engine capability)

| dir | what it exercises |
|-----|-------------------|
| [`features/`](features/) | language features: `not`/`absent`, `*` in identifiers, `forall`, `open`, stdlib domain-elimination, the `__symmetric__` kernel mirror, the unstratifiable `p ← absent q; q ← absent p` (which pins that the NAF boundary admits **one** candidate per round — a batch would derive both), two `(or …)` disjuncts with different guards and one binding key (the only fixture where a parked candidate is already fired when the boundary reaches it), and an `adjacent-via` constraint satisfied from the *same* house — the 2026-08-20 `disjunctive-prune` regression. **`10_expect.ein` and its two siblings are the ones that are not like the others**: each carries an `:expect` and so states its own answer, which makes `ein solve` on it a test rather than a demonstration (M1c [S1c.1.2](../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)) — one per verdict, `10` being `k = 1`, `11_expect_ambiguity.ein` `k > 1` and `12_expect_false.ein` `k = 0`. A fourth joined them at M1e [S1e.3.1](../plans/m1e_review_processing/p1e.3_medium/s1e.3.1_correctness.md): **`13_mixed_solution_and_open.ein`** is the `Solution` arm's *mixed* regime — one discharged model beside one open state, so `verdict.k` is 1 where `stats.solution_nodes` is 2 — which `finalise` defined and no program had reached, and which the table used to render as `solutions (k) 2` beside `verdict Solution`. **`ein test examples/features/` runs exactly those four** and never enters the search on the other nine, `04_open.ein` included ([S1c.1.3](../docs/history/m1c_external_validation/README.md#s1c13--ein-test)) |
| [`branching/`](branching/) | the hypothesis loop: saturate-only, dead/alive branches, multi-level, lookahead on/off, kill-cache on/off, `hrule`, hypothesis-relation whitelist, typed blind solve, the lookahead's NAF world and its unjudgeable guard (P1.21 R4 / S1.21.8 D3); and `15`+`16`, M1e's on/off pair where **both halves under-report** — `Solution k=1` against `Contradiction k=0`, both exhausted, and the hand-derived answer is k=2 |
| [`saturation/`](saturation/) | per-rule saturation demos by family — symmetric, transitive, `implies`, square fwd/bwd/unique, type-exclusivity, hypothesis-contradiction (see [`saturation/README.md`](saturation/README.md)) |
| [`lattice/`](lattice/) | commitment-lattice search: subset-pruned, genuine 3-set death, state-hash collision |
| [`domain_elim/`](domain_elim/) | domain-elimination vs hypothesis measurement fixtures (see [`domain_elim/README.md`](domain_elim/README.md)) |
| [`syntax/`](syntax/) | seventeen **node-kind probes** — one per shape the renderers draw and no puzzle contains: the two `=` arities, an arity-0 relation, every argument kind, stored negation over four shapes, the `is-a` subject positions, `(relation …)` at every arity as a nested value, a full `(query …)`, a chained `(trace …)` and an empty one, `(config …)`, the three S1.7c.4 wrapper heads as ordinary facts, five rule shapes (nested `absent`, `forall`/`not`/`eq`, top-level `or`, disjoint guard sets), and the constraint-scope markers. The eighteenth — the two half-specified declarators — is `broken/load/rule_half_declarators.ein`, because it is a load-negative as well as a probe. Moved out of `ein-render/tests/dot_parity.rs` by M1a S1a.10.2, where they were diffed against ein.py under eight parse views; as corpus entries the manifest digests them under **every** view and every op |

## Diagnostics & negative fixtures

| dir | what it holds |
|-----|---------------|
| [`ein-bugs/`](ein-bugs/) | contradiction / bug-repro puzzles (`zebra2-bad.ein` — injected-fact contradiction; `mixed-type-hypothesis.ein` and `nested-fact-hypothesis.ein` — the two shapes of the `sorted(alive)` crash in `apriori.layer_1`, recorded rather than repaired: M1a Q-M1a.4 / D2, the second needing no mixed types at all; `unbound-relation-head.ein` and `unbound-assert-var.ein` — `(?R ?x)` with `?R` unbound, and an `(or …)` whose arms bind different variables so one reaches the `:assert` unbound: two crash-parity cells, the second of which ein.rs had to learn to spell `KeyError:` for; `query-goal-free-head.ein` and `query-goal-free-head-unsat.ein` — `(query :goal (?R Rex Animal))`, two lines, a program ein.py rejected in its table renderer and ein.rs ran, plus the `Contradiction` arm that exits **0** because no solution block is rendered and so the goal is never compiled; `int-goal-binding.ein` and `fact-goal-binding.ein` — the two goal-binding shapes the M1a S1a.6.6 fuzzer found and both engines were **fixed** for, an integer binding that must stay a JSON number and a nested-fact binding that used to crash `json.dumps`; and three M1e witnesses that the engine **records a state its own rules refute** — `alive-empty-phase1.ein`, `alive-empty-interlayer.ein` and `complete-records-stale.ein`, one per `record_node` caller, each answering `Solution`/`Ambiguity` on a model that re-saturates to `Contradiction`. The first two carry an `:expect` **stating today's answer**, banked to break when the fix lands; and the M1e pair `alt-cap-core.ein` / `alt-cap-core-reordered.ein`, one `:priority` apart, which report a **3-fact** and a **2-fact** unsat core for the same facts and the same rules — `MAX_ALT_JUSTIFICATIONS = 32` retains by premise count where the explanation search minimises frontier size, so a full list refuses the smaller explanation: S1e.1.3 / Q-M1e.15; and `naf-upward-closure.ein` — the twenty lines on which **`dead` is not upward-closed under `absent`**, `{(p A)}` dying while `{(p A), (q A)}` would live, so the maximal state is never generated. Five of the six shipped configurations answer it wrongly and all six say `exhausted = true`. It carries `(config :warn-derived-naf true)`, which makes S1e.2.3's `RefutationUnderAbsentWarning` part of its recorded output, and its `:expect` states today's answer and is meant to break: M1e Q-M1e.9, fix filed as F18; and its M1e S1e.3.3 sibling, the pair `alive-set-fresh-name.ein` / `alive-set-fresh-name-declared.ein`, which differ by **one fact** — `(seen Z)`, over a relation nothing else mentions — and by the whole answer: the first says `k = 0, exhausted = true`, *No solution*, where `{(q A Z), (q B Z)}` is a model, because a rule names `Z` and no fact does, so the name enters only inside a fork while the lattice enumerates subsets of the `alive` set taken at root; the second names `Z` in the ontology and answers `Solution k = 1` over exactly that model. The **M1 alive-set invariant** (`ST-M1`, Q-M1e.21) measured rather than argued, and neither file contains an `(absent …)`, so nothing is confounded with Q-M1e.9) |
| [`broken/`](broken/) | curated **parse-failure** fixtures; each expects a `file:line:col` error (bare top-level atom, keyword-as-value, rule missing params, unclosed paren) |
| [`broken/load/`](broken/load/) | curated **load-failure** fixtures — files that parse and then fail `KnowledgeBase.from_ir`; each carries the exact `KBLoadError` message in a `.expected` beside it (see [`broken/load/README.md`](broken/load/README.md)) |
| [`broken/compile/`](broken/compile/) | curated **compile-failure** fixtures — files that parse and load and then hit one of the four `CompileError`s S1.22.0 turned from a silent `return []` into an error; each carries the exact message in a `.expected` beside it (see [`broken/compile/README.md`](broken/compile/README.md)). `activator_arity.ein` is the odd one: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
