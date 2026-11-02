# The taint census — 2026-11-02

**S1e2.1b.4's measurement** ([the stage](s1e2.1b.4_refutation_resting_on_absent.md)),
the evidence for [F21](../../followups/f21_non_monotone_programs_decided.md)'s
D6, *may a route learn from a refutation that rests on an `absent`?*, and for
[S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)'s F1–F3. It
reads every dead node of every flagged cube the oracle reaches, and what its
death rests on. It decides nothing: *forbid* stands as S1e2.4.1 ruled it, and
until D6 is decided no route learns on a flagged program.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census                 # § 1–5 and § 7, 0.2 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census -- --written    # the programs in shared/written.rs
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census -- \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein                               # S1e2.4.1's probes
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-infer --test region an_untainted_death_is_dead_in_every_superset   # § 6
```

Every command takes `--json` for one row per program. Taken 2026-11-02 at
`b6a3045b` with the example T1e2.1b.4.1 adds, release build, stdlib = the
checkout, 16 jobs on a 32-thread machine. Every number on this page is from
those commands unless it names another.

**What it selects.** `site_census`'s selection (271 files, 208 of which load
and saturate) and the programs the static check flags: **26**, 18 with a cube
within the limit (256 nodes) and 8 past it. The register's W3 and W8
witnesses are among the 18. S1e2.4.1's probes and the two programs in
[`examples/shared/written.rs`](../../../ein.rs/crates/ein-infer/examples/shared/written.rs)
get their own runs.

**What it reads.** [`taint_census`](../../../ein.rs/crates/ein-infer/examples/taint_census.rs)
computes the cube with `region::compute` and reads each dead node's fixpoint
through `region::Nodes`, the node function every route shares. The **taint**
is [`examples/shared/taint.rs`](../../../ein.rs/crates/ein-infer/examples/shared/taint.rs)'s
walk, wider than the oracle's own `region::Node::absent_premises`, which
follows the first contradiction's witness, the positive side of a pair:

- **every contradiction** of the fixpoint, and **both sides** of a pair, each
  through every fact's primary derivation, root's facts included;
- the **activator** of every plan a fact was derived by, which no derivation
  records (§ 3).

Each `absent` premise is classified by three questions:

- is its relation **static** (outside D), or in D?
- is it read **directly** by the rule that concluded the contradiction, or
  **via** a fact further down? These are S1e2.4.1's `direct` and `via` sets,
  per node;
- is it recorded on a fact **root** holds, and reached **through an
  activator**?

A node is **untainted** when some contradiction of it rests on no premise in
D: what option (b) would learn from. Two readings of *extendable* are taken
side by side: **in D** (the static check's) and **eligible**
(`hypgen::eligible_relations`, S1e.2.3's), S1e2.2.5's F1 (b) and (a).

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 18, 256 nodes |
| dead nodes | **122** |
| … untainted | **100**: `three_components` 96, `alive-empty-interlayer` 4 |
| … untainted, with an alive superset (a counterexample to (b)) | **0** |
| … tainted | **22**, 8 of them with an alive superset: the 8 dead holes |
| dead holes | 8, every one tainted |
| `absent` premises in D, over the 22 | 22 direct, **2 via**, 0 static |
| … recorded on a fact root holds | 2: `stale-naf-fork`'s and its twin's |
| … reached through an activator | 0 |
| dead nodes where *in D* and *eligible* disagree | **0** |
| dead nodes where the full walk and the oracle's `absent_premises` disagree | **0** |
| what (b) would let a route skip | **91** of the 256 nodes: 88 of `three_components`' 128, 3 of `alive-empty-interlayer`'s 8 |
| dead by fork and not from scratch | 2: W8's `{(q one)}` and its twin's |
| S1e2.4.1's flagged probes | 5, 174 nodes: 97 dead, 96 untainted with no alive superset, 1 tainted (`via_hole`'s hole, *via*) |
| `written.rs` | 2 cubes, 12 nodes: 2 dead holes, both tainted by the full walk. **On `ACTIVATOR_UNDER_ABSENT` the oracle's own reading and provenance alone call its hole untainted** (§ 3) |

## 2. The dead nodes, by program

Every flagged cube within the limit with a dead node. The other 10 have none.

| program | n | dead | untainted | tainted | … with an alive superset | dead holes | skippable by (b) | its premises |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | 12 | 0 | 12 | 0 | 0 | 0 | `(absent (not (r1 o3 o2)))` and its mirror, direct: `fire-1` derives the positive side of the pair under it. 16 contradictions over the 12 nodes |
| [`ein-bugs/alive-empty-interlayer.ein`](../../../examples/ein-bugs/alive-empty-interlayer.ein) | 3 | 4 | 4 | 0 | 0 | 0 | 3 | none: `kill` refutes `(p Z)` on positive premises; the site `totality` never fires |
| [`ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | 1 | 0 | 1 | 1 | 1 | 0 | `(absent (q A))`, direct (`bad`) |
| [`ein-bugs/wrong-answers/naf-nowb.ein`](../../../examples/ein-bugs/wrong-answers/naf-nowb.ein) | 2 | 1 | 0 | 1 | 1 | 1 | 0 | the same |
| [`ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | 1 | 0 | 1 | 0 | 0 | 0 | `(absent (q one))`, **via** `(p one)`, which **root** derived (`derive-p`) |
| [`ein-bugs/wrong-answers/stale-naf-fork-warn.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein) | 1 | 1 | 0 | 1 | 0 | 0 | 0 | the same |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | 7 | 99 | 96 | 3 | 3 | 3 | **88** | the 3 holes: `(absent (h B))` or `(absent (h D))`, direct (`dead-3`, `dead-7`); the other 96 rest on none |
| [`lattice/synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein) | 5 | 3 | 0 | 3 | 3 | 3 | 0 | `(absent (h B))`, `(absent (h E))`, direct (`dead-1`, `dead-2`) |

**Every death with an alive superset is tainted**, on the corpus, the probes
and the written programs, under the full walk. Every untainted death is dead
in every superset: `three_components`' 96 and `alive-empty-interlayer`'s 4.
That is (b)'s soundness condition, observed; T1e2.1b.4.2 makes it a test.

**Tainted is not the same as not upward-closed.** 14 of the 22 tainted deaths
have no alive superset: `d3-unsat-core`'s 12 and W8's two. (b) is
conservative on them, as the static check is on its eight.

## 3. The readings of *tainted*, side by side

| reading | what it reads | counterexamples to (b): an untainted death with an alive superset |
|---|---|---|
| **the full walk**, in D | every contradiction, both sides, every derivation, every plan's activator | **0** on the corpus, 0 on the probes, 0 written |
| **provenance alone**, in D | the same without the activators: (b) as the stage states it, *read off provenance* | 0 on the corpus, 0 on the probes, **1** written |
| **the oracle's field** | `region::Node::absent_premises`: the first contradiction's witness, its derivation | 0 on the corpus, 0 on the probes, **1** written |
| **eligible** instead of D | the full walk, S1e.2.3's reading of *extendable* | 0: it agrees with *in D* on every dead node of every run |

**The one counterexample is `ACTIVATOR_UNDER_ABSENT`**, written by S1e2.1b.3
([hole census § 2](hole_census.md#2-by-cause-and-kind)):

```lisp
(rule act () :match (and (p ?x) (absent (q ?x))) :assert (watch r))
(rule watch (?R) :match (?R ?x) :assert (false))
```

`{(p A), (r A)}` is dead: `act` derives the activator `(watch r)` under
`(absent (q A))`, the node compiles `watch` for `r`, and `(r A)` refutes.
`{(p A), (q A), (r A)}` is alive, because `act` does not fire there and no plan
refutes. The refutation's derivation is `(false)` ← `watch` ← `(r A)`, a
commitment, and it records no `absent`: a derivation does not record the
activator its plan was compiled from (`ein_core::Prov` has no field for it).
So a route that read the taint off provenance would learn `{(p A), (r A)}`
dead and skip a node of R. The full walk follows the plan's activator to
`act`'s premise and calls the death tainted, *via*, through an activator.

**Both sides of a pair, and every contradiction, add nothing on the corpus.**
The full walk and the oracle's field agree on all 122 dead nodes, including
`d3-unsat-core`'s, which die of a pair (`fire-1` derives `r1` under the
`absent`, `fire-0` its negation) and where the `absent` is on the positive
side the oracle reads. The argument for reading both sides stands (a `(not
X)` derived under an `absent` beside a given `X` would be missed by the
oracle's field), and no program here shows it.

## 4. What (b) buys over (a)

Under (a), what ships, no route learns on a flagged program: route 2 is the
cube, every node visited. Under (b) a route may skip every proper superset of
an untainted death. Within the limit that is **91 of the 256** corpus nodes,
88 of them on `three_components` (n = 7), and 88 of the 174 probe nodes, on
`three_components_owe`. On the other cubes (b) skips nothing, because their
deaths are all tainted or there are none.

Past the limit, where it would matter, the census cannot count: (b)'s value
there is what a search on the flagged programs past the limit (F21's D7)
would gain, and none is built. Of the 8, `features/05` and `/12` have rules
that refute on an `absent` over D at relation grain (S1e2.4.1's `direct`
set), and `lattice/06`'s one site is a trigger. None is measured.

## 5. W8's `p-and-q-clash`, fork and scratch

[`stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein):
`derive-p` derives `(p one)` at root under `(absent (q one))`, and
`p-and-q-clash` refutes `(p one)` beside `(q one)`.

| | `{(q one)}` | its taint |
|---|---|---|
| **fork** (A), today's node | dead, refuted by `p-and-q-clash` | `(absent (q one))`, **via** `(p one)`, **on a root fact**: tainted. No superset (n = 1) |
| **scratch** (B) | in R: `derive-p` never fires once `(q one)` is given ([`node_census.md` § 2.1](node_census.md#21-the-corpus)), so there is no refutation to taint | — |
| (C) over (A) | the third outcome: dead, its death resting on a stale fact | as fork |

So under fork the refutation is tainted, and (b) would not learn from it;
under scratch it does not exist. It is the census's only premise recorded on a
fact root holds, with its `-warn` twin. Under fork such a premise cannot make
a death non-upward-closed, because a fork keeps root's facts in every node;
the walk counts it anyway, which is conservative.

## 6. The check, as a test

[`ein-infer/tests/region.rs`](../../../ein.rs/crates/ein-infer/tests/region.rs)`::an_untainted_death_is_dead_in_every_superset`,
banked by T1e2.1b.4.2, holds (b)'s soundness condition per commit:

```sh
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-infer --test region an_untainted_death_is_dead_in_every_superset
```

- On every flagged corpus and register cube within the limit, and on the two
  programs in `written.rs`, **no untainted dead node has an alive superset**,
  and **every dead hole is tainted**. The taint is `shared/taint.rs`'s full
  walk, the census's. Monotone programs are skipped by the static check before
  their cube is enumerated.
- **It is not vacuous**: it reads 20 flagged cubes, 124 dead nodes and 100
  untainted deaths (2026-11-02), and asserts at least 20, and at least 90
  untainted below the dead.
- **It pins the counterexample**: on `ACTIVATOR_UNDER_ABSENT` the oracle's
  `absent_premises` for the dead hole `{(p A), (r A)}` is empty, and the walk
  without activators calls it untainted beside an alive superset.
- **It has teeth.** With the walk's activator half switched off, it fails on
  `ACTIVATOR_UNDER_ABSENT`: *`{(p A), (r A)}` is dead on no `absent` over D,
  and a superset is alive* (run 2026-11-02 and reverted).

It holds whatever D6 decides: it is a property of the oracle's provenance and
of the walk, and it fails the day (b) as the census reads it stops being sound
on a cube the corpus has.

## 7. The refutation a node compiles, for F3

*Forbid* is a refusal of `(absent P)` ⟹ `(false)` or `(not …)` over a relation
a commitment can extend. S1e2.2.5's F3 asks where it runs. A rule with
parameters is compiled once per activator fact, so a node can compile a plan
root never had, when a commitment derives an activator. Two placements are
on the table: **at load**, on root's plans after root's saturation, where the
static check runs; and **at each node's compile**, on every plan a node
compiles that root did not. The census reads both:

- **at load**: the refuting plans at root that read an `absent` over D at odd
  depth;
- **at each node's compile**: on every node of every cube within the limit,
  each rule with parameters, each activator fact the node holds and root does
  not, the plan compiled (`compile::compile_rule`), kept when it refutes, with
  whether it reads an `absent` over D and whether its activator's own
  derivation rests on one.

| | corpus: flagged programs | within the limit: nodes that compile a refutation | `ACTIVATOR_REFUTES` | `ACTIVATOR_UNDER_ABSENT` |
|---|---|---|---|---|
| activator sites | **2** programs: `features/01` (`unknown-feeling`, a root plan that concludes a positive fact) and `lattice/09` (`total-owed`, an obligation) | — | `watch` | `watch` |
| … whose rule concludes a refutation | **0** | — | 1 | 1 |
| refutations a node compiles | — | **0**, on all 18 cubes and the 5 flagged probes | `watch` for `(watch q)`, first at `{(p A)}`: reads `(absent (q ?x))` over D | `watch` for `(watch r)`, first at `{(p A)}`: reads no `absent`; its activator rests on `act`'s `(absent (q ?x))` |
| **caught at load** | **9** programs, the 13 rules of S1e2.4.1's `direct` set (`features/05` and `/12` among them, past the limit) | — | **no**: root has no plan of `watch` | **no** |
| **caught at each node's compile** | the same 9: their plans exist at root | 0 more | **yes**, at the first node holding `(p A)` | **no**: the plan reads no `absent`. It is `via`, through the activator |

**So on the corpus the two placements catch the same programs.** No corpus
program has an activator in D whose rule refutes, and no node of a flagged
corpus cube within the limit compiles a refutation. The difference is the
written programs:

- `ACTIVATOR_REFUTES` is caught at a node's compile and not at load. A check
  at a node's compile runs where a route visits, so it is a refusal during a
  run rather than before it, and past the limit it sees only the nodes the run
  reached. A static check that reads every rule with parameters whose
  activator relation is in D, and refuses one that concludes a refutation
  from a body reading `(absent (?R …))` with `?R` a parameter, would catch it
  at load; on the corpus it would read the two activator sites above and
  refuse neither. That third placement is drafted here, not measured beyond
  those two.
- `ACTIVATOR_UNDER_ABSENT` is caught by neither. Its refutation reads no
  `absent`; the `absent` is in its activator's derivation. *Forbid* reaches it
  only if F2's `via` follows a plan's activator as a premise, or if F21's D1
  (E) refuses `act`, an `absent` site that concludes a positive fact.

Past the limit the census reads the static half only: `features/01`'s
activator site is the one there, and its plan at root concludes a positive
fact.

## 8. Handed on

- [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here):
  a dated note, the census and the test, and that learning before the refusal
  ships is F21's D6.
- [S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md#the-evidence-from-s1e21b4-2026-11-02):
  F1 per dead node (§ 3), the `via` deaths for F2 (§ 2, § 5), and the
  compiled refutation for F3 (§ 7), each with its numbers, and that
  `ACTIVATOR_UNDER_ABSENT` escapes both of F3's placements.
- [F21](../../followups/f21_non_monotone_programs_decided.md)'s D6: the census
  and the test, linked, with the agent's recommendation, (b) with the
  activator walk; and a fifth item under *what forbid does not cover*.
- [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md): § 3's
  *learned nogoods* row, § 7.4, § 7.7, § 7.8 and § 8 cite the census and the
  test. The taxonomy's refutation rows are S1e2.1b.7's.
