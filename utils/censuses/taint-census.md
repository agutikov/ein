# The taint census — 2026-11-25

**S1e2.1b.4's measurement** (the stage), re-taken,
the evidence for [F21](../../plans/m1n_non_monotone_programs/f21_non_monotone_programs_decided.md)'s
D6, *may a route learn from a refutation that rests on an `absent`?* It
reads every dead node of every flagged cube the oracle reaches, and what its
death rests on. It decides nothing.

**What holds today.** A flagged program loads, every command answers it by the
cube under a `WARNING` that it is not monotone, and nothing is claimed of its
answer
([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)):
no program is refused for an `absent`. *Forbid*'s refusal, which the stage's
evidence fed (S1e2.2.5's F1 to F3: what counts as extendable, the `via`
class, where the refusal runs), is gone, so those questions have no subject.
No route learns on a flagged program: it is answered by route 2, the cube,
with no licence. Whether one may is D6, F21's.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census                 # § 1–5 and § 7, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census -- --written    # the programs in shared/written.rs, 0.6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example taint_census -- \
    plans/m1n_non_monotone_programs/f21_probes/*.ein                                                                       # S1e2.4.1's probes: none loads today
```

Every command takes `--json` for one row per program. Taken 2026-11-25 at
`213e8a92`, release build, stdlib = the checkout, 16 jobs on a 32-thread
machine, each command through `utils/bench_env.sh --cores 0-31` on AC.
Every number on this page is from those commands unless it names another.

**What it selects.** `site_census`'s selection (933 files, 748 of which load
and saturate) and the programs the static check flags: **59**, 52 with a cube
within the limit (656 nodes in all) and 7 past it. The witnesses of the
regressions W8 left, `stale-naf-fork` and its `-warn` twin, are among the 52.
**S1e2.4.1's 14 probes are not re-taken**: each is written in a form the
parser refuses today (`hrule`, `:priority`, no `:why`), and none loads. The
two programs in
[`examples/shared/written.rs`](../../ein.rs/crates/ein-infer/examples/shared/written.rs)
get their own run.

**What it reads.** [`taint_census`](../../ein.rs/crates/ein-infer/examples/taint_census.rs)
computes the cube with `region::compute` and reads each dead node's fixpoint
through `region::Nodes`, the node function every route shares. The **taint**
is [`examples/shared/taint.rs`](../../ein.rs/crates/ein-infer/examples/shared/taint.rs)'s
walk, wider than the oracle's own `region::Node::absent_premises`, which
follows the first contradiction's witness, the positive side of a pair:

- **every contradiction** of the fixpoint, and **both sides** of a pair, each
  through every fact's primary derivation, root's facts included;
- the **activator** of every plan a fact was derived by, which no derivation
  records (§ 3).

Each `absent` premise is classified by three questions:

- is its relation **static** (outside D), or in D?
- is it read **directly** by the rule that concluded the contradiction, or
  **via** a fact further down?
- is it recorded on a fact **root** holds, and reached **through an
  activator**?

A node is **untainted** when some contradiction of it rests on no premise in
D: what option (b) would learn from. Two readings of *extendable* are taken
side by side: **in D** (the static check's) and **eligible**
(`hypgen::eligible_relations`, S1e.2.3's).

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 52, 656 nodes |
| dead nodes | **175** |
| … untainted | **138**: `three_components` 96, `d3-unsat-core` 12, `oblig-fresh-name-two-declared` 8, `alive-empty-interlayer` 4, and 18 on 11 other `wrong-answers` witnesses |
| … untainted, with an alive superset (a counterexample to (b)) | **1**: `activator-chain-node-writes`'s `{(p a)}` (§ 3) |
| … tainted | **37**, 13 of them with an alive superset: 13 of the 14 dead holes |
| dead holes | 14, 13 tainted |
| `absent` premises in D, over the 37 | 42 direct, **13 via**, 0 static |
| … recorded on a fact root holds | 40, on 24 of the 37 |
| … reached through an activator | **10**, on 10 of the 37 |
| dead nodes where *in D* and *eligible* disagree | **12**, all `d3-unsat-core`'s |
| dead nodes where the full walk and the oracle's `absent_premises` disagree | **10**: `activator-all-int`, `-nested`, `-symbol` 2 each, and `activator-chain-0`, `-1`, `-2`, `-node-absent` 1 each |
| what (b) would let a route skip | **116** of the 656 nodes: 88 of `three_components`' 128, 10 of `d3-unsat-core`'s 32, 7 of `oblig-fresh-name-two-declared`'s 16, 3 of `alive-empty-interlayer`'s 8, and 1 on each of 8 more, `activator-chain-node-writes` among them |
| dead by fork and not from scratch | 17: W8's `{(q one)}` and its twin's, the one candidate's node of `activator-all-*`, 12 of `rung-connex-import`'s 16 |
| S1e2.4.1's probes | not re-taken: none loads |
| `written.rs` | 2 cubes, 12 nodes: 2 dead holes, both tainted by the full walk. **On `ACTIVATOR_UNDER_ABSENT` the oracle's own reading and provenance alone call its hole untainted** (§ 3) |

## 2. The dead nodes, by program

Every flagged cube within the limit with a dead node. The other 23 have none.

| program | n | dead | untainted | tainted | … with an alive superset | dead holes | skippable by (b) | its premises |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `ein-bugs/activator-all-int`, `-nested`, `-symbol` | 1 | 2 | 0 | 2 | 0 | 0 | 0 | `(absent (checked …))`, **via**, **at root**, **through an activator**: root is dead, `alarm`'s `(false)` on an activator `flag` derives under the `absent` |
| `ein-bugs/alive-empty-interlayer` | 3 | 4 | 4 | 0 | 0 | 0 | 3 | none: `kill` refutes `(p Z)` on positive premises; the site `totality` never fires |
| [`ein-bugs/d3-unsat-core`](../../examples/ein-bugs/d3-unsat-core.ein) | 5 | 12 | 12 | 0 | 0 | 0 | 10 | `(absent (not (r1 o3 o2)))` and its mirror, direct: static *in D*, eligible (§ 3) |
| [`ein-bugs/naf-upward-closure`](../../examples/ein-bugs/naf-upward-closure.ein) | 2 | 1 | 0 | 1 | 1 | 1 | 0 | `(absent (q A))`, direct (`bad`) |
| `wrong-answers/naf-nowb` | 2 | 1 | 0 | 1 | 1 | 1 | 0 | the same |
| `wrong-answers/activator-chain-0`, `-1`, `-2`, `-node-absent` | 2 | 1 | 0 | 1 | 1 | 1 | 0 | `(absent (p2 a))`, **via**, **through an activator**: a chain of activators from an `absent` ends in a refutation compiled at the node |
| `wrong-answers/activator-chain-param-absent` | 2 | 1 | 0 | 1 | 1 | 1 | 0 | `(absent (p2 a))`, **via**, recorded on a derivation |
| `wrong-answers/activator-chain-node-writes` | 2 | 1 | **1** | 0 | **1** | 1 | 1 | `(absent (x a))`, direct, **static** *in D*: `x` is written only by a plan a node compiles (§ 3) |
| `wrong-answers/activator-chain-no-absent`, `core-activator-committed`, `core-activator-guard-only` | 2 | 2 | 2 | 0 | 0 | 0 | 1 | none |
| the eight `oblig-*` witnesses with a dead node | 1–4 | 1–8 | all | 0 | 0 | 0 | 0–7 | none in D |
| `wrong-answers/rung-connex-import` | 4 | 16 | 0 | 16 | 0 | 0 | 0 | `(absent (lt A C))` beside `(absent (lt C A))`, direct, **at root**: `std.algebra`'s `connex` refutes root |
| [`wrong-answers/stale-naf-fork`](../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein), `-warn` | 1 | 1 | 0 | 1 | 0 | 0 | 0 | `(absent (q one))`, **via** `(p one)`, which **root** derived (`derive-p`) |
| [`lattice/synth/three_components`](../../examples/lattice/synth/three_components.ein) | 7 | 99 | 96 | 3 | 3 | 3 | **88** | the 3 holes: `(absent (h B))` or `(absent (h D))`, direct (`dead-3`, `dead-7`); the other 96 rest on none |
| [`lattice/synth/trigger_hole`](../../examples/lattice/synth/trigger_hole.ein) | 5 | 3 | 0 | 3 | 3 | 3 | 0 | `(absent (h B))`, `(absent (h E))`, direct (`dead-1`, `dead-2`) |

**Every death with an alive superset is tainted but one**, on the corpus and
the written programs, under the full walk: `activator-chain-node-writes`'s
`{(p a)}` (§ 3). Every other untainted death is dead in every superset. That
was (b)'s soundness condition, observed on 2026-11-02 and held by a test
since removed (§ 6); the corpus has gained a program that breaks it.

**Tainted is not the same as not upward-closed.** 24 of the 37 tainted deaths
have no alive superset: `rung-connex-import`'s 16, `activator-all-*`'s 6 and
W8's two. (b) is conservative on them, as the static check is on the flagged
programs with no hole.

## 3. The readings of *tainted*, side by side

| reading | what it reads | counterexamples to (b): an untainted death with an alive superset |
|---|---|---|
| **the full walk**, in D | every contradiction, both sides, every derivation, every plan's activator | **1** on the corpus (`activator-chain-node-writes`), 0 written |
| **provenance alone**, in D | the same without the activators: (b) as the stage states it, *read off provenance* | **5** on the corpus (`activator-chain-0`, `-1`, `-2`, `-node-absent`, `-node-writes`), **1** written |
| **the oracle's field** | `region::Node::absent_premises`: the first contradiction's witness, its derivation | **5** on the corpus, the same, **1** written |
| **eligible** instead of D | the full walk, S1e.2.3's reading of *extendable* | it disagrees with *in D* on `d3-unsat-core`'s 12 dead nodes, none with an alive superset |

**`ACTIVATOR_UNDER_ABSENT`**, written by S1e2.1b.3
([hole census § 2](hole-census.md#2-by-cause-and-kind)):

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
**Four corpus programs now show the same**: the `activator-chain` witnesses
`-0`, `-1`, `-2` and `-node-absent`, whose refutation is compiled at a node
from an activator a chain derives under an `absent` (`mk`'s, or `q`'s in
`-node-absent`).

**`activator-chain-node-writes` breaks the full walk too.** `chk` concludes
`(false)` from `(p ?y)` while `(x ?y)` is absent, and only `q2`'s plan writes
`x`, compiled from the activator `(q2 a)` that `mk2` derives from the
candidate `(p2 a)`. Root compiles no plan of `q2`, so `x` is outside D, which
is closed over the plans root compiled: the walk calls the premise *static*
and `{(p a)}` untainted, and `{(p a), (p2 a)}` above it is alive. The static
check flags the program for `q2`'s activator site alone. A taint read *in D*
misses a relation a commitment extends through a plan a node compiles.

**Both sides of a pair, and every contradiction.** On `d3-unsat-core` the
deaths are pairs (`fire-1` derives `r1` under the `absent`, `fire-0` its
negation), and the `absent` reads a stored negation, `(not (r1 o3 o2))`,
which *in D* does not hold and *eligible* does: so *in D* calls its 12 deaths
untainted, none with an alive superset, and *eligible* tainted. The argument
for reading both sides stands (a `(not X)` derived under an `absent` beside a
given `X` would be missed by the oracle's field), and no program here shows
it.

## 4. What (b) buys over (a)

Under (a), what ships, no route learns on a flagged program: route 2 is the
cube, every node visited. Under (b) a route may skip every proper superset of
an untainted death. Within the limit that is **116 of the 656** corpus nodes,
88 of them on `three_components` (n = 7). One of the 116 is a node of R:
`activator-chain-node-writes`'s `{(p a), (p2 a)}`, which (b) read *in D* would
skip (§ 3). On the other cubes (b) skips nothing, because their deaths are all
tainted or there are none.

Past the limit, where it would matter, the census cannot count: (b)'s value
there is what a search on the flagged programs past the limit (F21's D7)
would gain, and none is built. Of the 7, `features/01` has the one activator
site, a root plan that concludes a positive fact, and `lattice/06`'s one site
is a trigger. None is measured.

## 5. W8's `p-and-q-clash`, fork and scratch

[`stale-naf-fork.ein`](../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein):
`derive-p` derives `(p one)` at root under `(absent (q one))`, and
`p-and-q-clash` refutes `(p one)` beside `(q one)`.

| | `{(q one)}` | its taint |
|---|---|---|
| **fork** (A), today's node | dead, refuted by `p-and-q-clash` | `(absent (q one))`, **via** `(p one)`, **on a root fact**: tainted. No superset (n = 1) |
| **scratch** (B) | in R: `derive-p` never fires once `(q one)` is given ([`node_census.md` § 2.1](node-census.md#21-the-corpus)), so there is no refutation to taint | — |
| (C) over (A) | the third outcome: dead, its death resting on a stale fact | as fork |

So under fork the refutation is tainted, and (b) would not learn from it;
under scratch it does not exist. A premise recorded on a fact root holds is
the corpus's on 24 tainted deaths today: W8's pair, `rung-connex-import`'s 16
and `activator-all-*`'s 6, the latter two with a dead root. Under fork such a
premise cannot make a death non-upward-closed, because a fork keeps root's
facts in every node; the walk counts it anyway, which is conservative.

## 6. The check, as a test

`ein-infer/tests/region.rs::an_untainted_death_is_dead_in_every_superset`,
banked by T1e2.1b.4.2, held (b)'s soundness condition per commit: on every
flagged corpus and register cube within the limit, no untainted dead node had
an alive superset, and every dead hole was tainted. **It is gone**: removed
with every assertion that held a flagged program's answer (Q-M1f.564: no suite
validates what a non-monotone program answers), so

```sh
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-infer --test region an_untainted_death_is_dead_in_every_superset
```

runs 0 tests (3.8 s, the build). **It would fail today** on
`activator-chain-node-writes` (§ 3), as its activator half did by hand on
`ACTIVATOR_UNDER_ABSENT` when it was banked. What it held is now this census
alone, a measurement that fails nothing.

## 7. The refutation a node compiles, for F3

A rule with parameters is compiled once per activator fact, so a node can
compile a plan root never had, when a commitment derives an activator. The
stage read where a check of the refutation would see it, for *forbid*'s
placement (S1e2.2.5's F3); the refusal is gone, and the measurement stands as
what the static check's sites see. Two places: **at load**, on root's plans
after root's saturation, where the static check runs; and **at each node's
compile**, on every plan a node compiles that root did not:

- **at load**: the refuting plans at root that read an `absent` over D at odd
  depth;
- **at each node's compile**: on every node of every cube within the limit,
  each rule with parameters, each activator fact the node holds and root does
  not, the plan compiled (`compile::compile_rule`), kept when it refutes, with
  whether it reads an `absent` over D and whether its activator's own
  derivation rests on one.

| | corpus: flagged programs | within the limit: nodes that compile a refutation | `ACTIVATOR_REFUTES` | `ACTIVATOR_UNDER_ABSENT` |
|---|---|---|---|---|
| activator sites | **21**, one a program: 20 within the limit (`activator-all-*`, the seven `activator-chain` witnesses, `core-activator-*`, `derived-activator` and `-never`, `oblig-derived-activator` and `-declared`, `features/19` and `/20`, `lattice/09` and `/12`) and `features/01` past it (`unknown-feeling`, a root plan that concludes a positive fact) | — | `watch` | `watch` |
| … whose rule has a refuting plan at root | **0** | — | 0 | 0 |
| refutations a node compiles | — | **7**, on 7 programs: `activator-chain-0`, `-1`, `-2`, `-no-absent`, `-node-absent`, `core-activator-committed`, `core-activator-guard-only` | `watch` for `(watch q)`, first at `{(p A)}`: reads `(absent (q ?x))` over D | `watch` for `(watch r)`, first at `{(p A)}`: reads no `absent`; its activator rests on `act`'s `(absent (q ?x))` |
| … reading an `absent` over D | — | **0** | yes | no |
| … whose activator rests on one | — | 2: `activator-chain-0` (`p2 ← mk`), `-node-absent` (`p2 ← q`) | no | yes |
| **seen at load** | **8** programs refute at root over an `absent` in D: `branching/13`, `/14`, `alive-empty-interlayer`, `naf-upward-closure`, `naf-nowb`, `rung-connex-import`, `three_components`, `trigger_hole` | — | **no**: root has no plan of `watch` | **no** |
| **seen at each node's compile** | the same 8: their plans exist at root | 0 more over D | **yes**, at the first node holding `(p A)` | **no**: the plan reads no `absent`. It is `via`, through the activator |

**On the corpus the two places see the same programs at an `absent` over D.**
No refutation a node compiles reads one: the seven read none, and two of them
rest on one through their activator, as `ACTIVATOR_UNDER_ABSENT` does. Such a
death is seen by neither place; the static check flags each program by its
activator site, and the taint's walk through the activator (§ 3) is what
reaches the `absent`. Past the limit the census reads the static half only:
`features/01`'s activator site is the one there, and its plan at root
concludes a positive fact.

## 8. Where it bears

- [F21](../../plans/m1n_non_monotone_programs/f21_non_monotone_programs_decided.md)'s
  D6: the census, with the stage's recommendation, (b) with the activator
  walk, and today's counterexample to it read *in D*
  (`activator-chain-node-writes`, § 3): (b) needs D followed through the plans
  a node compiles, or its walk to read such a premise as extendable.
- Q-M1e2.5, the decision the stage's dated note served, is reversed by
  [Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md):
  nothing is refused, nothing is claimed of a flagged program's answer, and no
  test holds (b)'s condition.
- [`08-monotonicity.md`](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md)
  states what holds of a flagged program today; no claim there rests on this
  census.
