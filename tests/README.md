# tests/

**Ein programs whose job is to fail when the engine is wrong.**

Everything here is an `.ein` file carrying an `:expect`, so it states its own
answer and `ein test` turns the directory into a status code:

```sh
ein test tests/                       # 0 if every claim holds, 1 if one is false
ein test tests/stdlib/slots/ -v       # one line per query, with the verdict and k
```

**This is not `examples/`.** That directory is a set of things to read and run
— two Zebra encodings, per-feature demonstrations, the negative fixtures the
diagnostics are pinned against — and a reader goes there to find out what
ein-lang looks like. These are tests: each one exists to break, most of them
are three declarations and two facts, and nobody would learn the language from
them. The split is the reason `tests/` is a root of its own rather than
`examples/stdlib/`.

They are corpus entries all the same. `corpus/corpus.toml` has one row per file
(group `stdlib`, five runs each) and `ein-corpus`'s completeness check walks
this directory beside the other two, so a test file with no entry fails the
gate — the sweep would otherwise never run it, which reads as coverage nobody
has.

## `stdlib/` — the stdlib conformance corpus

M1c [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus).
One program per stdlib rule or tight family, each the smallest thing that
activates it and each stating what it should and should not derive.

**Why they exist.** Every other check in this repository is *relative*: the
goldens compare ein.rs to its own past, and since
[P1a.10](../docs/history/m1a_rust/README.md#p1a10--one-implementation) there is
no second engine to compare it to at all. The stdlib was where that gap was
widest —
[the census](../docs/history/m1c_external_validation/stdlib_census.md)
measured **38 of 73 rules never firing** in 400 corpus runs, 33 of them never
even loaded, and 20 more held up by `examples/zebra.ein` alone.

| dir | rules | what the programs do |
|---|---:|---|
| [`stdlib/algebra/`](stdlib/algebra/) | 40 | the copiers and the relative product; the Boolean lattice; the extensive operators and their closed-world caveat; the tag lemmas and the two Tarski equations; Schröder's negative propagation; then the seven property **checks** — once satisfied, once violated apiece — the totality pair's `forall` in both directions, and (M1d S1d.2.4) the totality pair's **obligation** duals, each once owing and once satisfied |
| [`stdlib/bijection/`](stdlib/bijection/) | 8 | the two setup fan-outs and the negative completion; domain- and range-elimination, each productive in its own file and redundant in its sibling's; the two arg typechecks violated; and `06_blind_enumeration.ein`, the one program here that leaves the enumerator **on** — since M1d S1d.2.5 by making the obligations rung *decline*, which is the only way to reach it from a program that declares one |
| [`stdlib/elim/`](stdlib/elim/) | 4 | the same inference in its *positional* formulation, plus `no-room-left` — the pair that brackets a quantifier: one exclusion short of full forces a value, exactly full refutes |
| [`stdlib/closure/`](stdlib/closure/) | 1 | `infer-closure`, and its soundness caveat **exhibited**: a program the import takes from fifty-four models to one; plus the closed-and-owing pair, which since M1d S1d.2.6 is the only place in the suite where **one fact decides a verdict word** — `02` is `Solution` and `03` is `Open — owes 1`, and what put them in scope to differ at all is one `(total-owed r is-a)` line apiece |
| [`stdlib/slots/`](stdlib/slots/) | 20 | a second activating puzzle for the module that had exactly one — the partition chain; `slot-fill` and `slot-elimination` as a matched pair, each productive where the other cannot fire; both violation duals; and the spatial family over two different position structures, one of which exists nowhere else and is what tells `slot-prune-fwd` from `slot-prune-bwd` |
| [`stdlib/typing/`](stdlib/typing/) | 4 | the reflexive-closure knob, and the `(type-hierarchy …)` knob in both directions |
| [`stdlib/macro/`](stdlib/macro/) | — | `forall` checked against its own expansion written out by hand, and `unknown`'s three states |

### The obligation pairs, and the check that is not an `:expect`

M1d [S1d.2.4](../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md)
added four rules that assert the verdict atom `open` — `total-owed`,
`surjective-owed`, `slot-owed-room`, `slot-owed-fill` — and eight programs,
one firing and one satisfied per rule. They are written the way everything
else here is, with one exception that is worth knowing before writing a
ninth.

**`:expect` cannot state an owe claim, and will not learn to.** Its three
forms are all assertions about *facts*, and an `open` conclusion is by
construction never a fact: it is a tally on the search-lattice node, because a
stored one would survive its own discharge in a fork that paid it. So each of
the eight carries an ordinary `:expect (model …)` — which is what satisfies
the coverage gate and what pins the *state* the debt is read from — and the
count itself is asserted in
[`ein-infer/tests/obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
in-process, the same shape S1c.1.5 used. Growing the grammar a word for an
open verdict is a verdict-vocabulary change and is S1d.2.6's to take.

**The satisfied half is the half that matters.** The coverage gate is happy
with the firing file alone. What it cannot show is that the report ever
*stops*: discharge **is** the guard — there is no second query that could
disagree with the rule's own `absent` — so "this file reports nothing" is the
whole claim that the guard is right.

**`06_blind_enumeration.ein` is not a rule's test.** It exists because the two
new activator facts per declaration are *stored* facts, and the question "can
a blind run start guessing about them" had to be a fixture rather than an
argument. It found a bug: an obligation rule's name was categorising as an
*object*, so the enumerator proposed `(seats total-owed C1)` — the name of a
rule as a puzzle value. It is the only program in this directory that leaves
the enumerator on, and it names only the hierarchy relations in
`:no-hypothesis`, because a file that scoped the activators out would be
testing its own scoping line.

**And since M1d S1d.2.5 it has to *earn* the enumerator**, which is worth
knowing before editing it. Hypothesis generation is a ladder now — the puzzle's
hrules, else what the state owes, else the blind enumeration — so a program
declaring `(bijective seats)` no longer reaches the third rung at all, and this
file's whole check went vacuous the day the ladder landed. It gets it back with
a second obligation, `sorted-somewhere`, which owes `is-a` — exactly the
relation `(total-owed seats is-a)`'s guard *scans*. The domain contract's C4
declines a branch whose candidate set could grow underneath it, and a declined
obligation hands the whole call to the blind enumerator. So the file is also
the corpus's one `mode=declined` fixture, and the obligation it added is
*satisfied* on purpose: the decline is static, per activator, and must not
depend on the KB happening to owe something.

### Five things about how they are written

**Naming a relation closes it.** An expectation lists a relation's *complete*
extent, so a rule that derived one fact too many fails the file. That is what
makes a "must not fire" case checkable at all for the 33 rules that carry no
guard: their only *must not* is scope, and scope is a claim about a whole
relation. Where a rule needs a relation to stay untouched, the fixture gives it
one authored edge and closes it there —
[Q-M1c.6](../docs/history/m1c_external_validation/open_questions.md#q-m1c6--how-does-an-expectation-say-a-relation-is-empty)
is why an *empty* relation cannot be named.

**`(unknown …)` is how a fixture says a negative was *not* invented.** Stored
negatives are deliberately not closed by an expectation, so listing the
exclusions that exist says nothing about the ones that do not. Ten programs
carry a four-line `probe-undecided` rule whose body is `std.macro`'s `(unknown
P)` — true exactly when a fact is in neither store — turning the absence into
an ordinary positive fact the expectation *can* close. Its priority is 500,
above everything the stdlib declares, because a probe that reads the world has
to be the last thing to read it.

**A refutation rule gets two files.** "It fires and the answer is ⊥" is one
test and it is not the one that finds bugs; the other is a program where the
rule is loaded, activated and *satisfied*, so that a guard admitting too much
turns an ordinary model into a contradiction. `algebra/08_checks_satisfied.ein`
is that file for seven rules at once, and each of the seven has a `_violated`
sibling.

**Where two rules reach one verdict, separate them by activation.** An
expectation is made of facts and cannot say which rule produced them — the
`route` residue
[Q-M1c.2](../docs/history/m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)
parks. On a fully excluded row `no-room-left` refutes and `domain-elimination`
forces a value against a stored negative, and both end in ⊥; the fixture picks
one by declaring only its activator. Five files do this — `elim/02`,
`elim/03`, `elim/04`, `slots/05`, `slots/06` — and each says so in its header,
because a reader who does not know why `slot-partition-setup` is absent will
put it back.

### What a mutation sweep says about them

**A verdict word is not what an expectation claims.** All three `:expect`
forms are assertions about **facts** (`01_grammar.md` § Query), so no fixture
here says `Solution`, `Open` or `Contradiction` — it says what the answer's
fact set is. M1d S1d.2.6 is the proof that this was the right shape: twelve of
these programs changed verdict word that day, from `Solution` to `Open`, and
**not one `(model …)` claim moved**, because the facts an open state reached
are the facts it reached. What a fixture cannot state it asserts in-process
instead — the owe counts in
[`obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
the rung modes in `obligation_rung.rs` — which is S1c.1.5's shape and the
reason growing `:expect` a word for the verdict was deferred rather than
needed.

**50 of 51.** One deliberate defect per rule family — a dropped `neq`, an
exchanged pair of `absent` operands, a `forall` over the wrong type, a fan-out
missing an activator — injected into a copy of `stdlib/` and run past
`ein test tests/`. Every one of the five separations above was added *because*
a mutant survived without it.

The survivor is `slot-adjacent-bwd-neg` with its two structure operands
exchanged: on the three-seat row of `slots/08_spatial_adjacent.ein` the
exclusion it should derive is reachable from the *other* clue's chain, and
isolating it needs a fourth seat and enough slack to keep the two chains from
meeting — a bigger puzzle than the acceptance allows a fixture to be. It is
recorded here rather than papered over, because a mutation score with an
unnamed survivor is a slogan.

### What holds this directory up

Two claims about it are **in `cargo test`**, since M1c
[S1c.1.5](../docs/history/m1c_external_validation/README.md#s1c15--in-the-gate)
— `ein-infer/tests/stdlib_coverage.rs`, 0.04 s for the whole sweep:

| the claim | what fails without it |
|---|---|
| **every stdlib rule is activated by a program *here*** | a rule added to `stdlib/` with no test. It is named, with its module, so the failure says which directory the missing file goes in |
| **every program here states an expectation** | a fixture whose `:expect` was lost in a refactor, which would then load, run and pass forever |

The first is deliberately scoped to this directory and not to the corpus. The
corpus-wide version is weaker in exactly the way that matters: a rule that
happened to fire somewhere inside `examples/zebra.ein` would pass it with **no
test written**, which is the state
[the census](../docs/history/m1c_external_validation/stdlib_census.md)
found 20 rules in. Scoping it also found the one rule this suite did not run —
`transitive`, whose fixture was a two-cycle where the `(neq ?a ?c)` guard
refuses every match. `algebra/21_transitive.ein` grew a three-chain, and the
suite now stands on its own: **73 of 73, no `examples/` entry contributing.**

The third file every entry here is swept by is `corpus/corpus.toml` — five
runs each, 225 of the sweep's 889 cells, **0.72 s** of its 5.1 s.

### Re-measuring the coverage

The gate is a yes/no. The *numbers* — firings per rule, productive vs
redundant, who the sole activator is — stay with the instrument:

```sh
python3 utils/stdlib_census.py                     # the table, all 180 entries
python3 utils/stdlib_census.py -k tests/stdlib     # this directory's own contribution
python3 utils/stdlib_census.py --check             # exit 1 while any rule is at zero
```

It parses `stdlib/*.ein` for rule heads, then sweeps every corpus entry under
every declared `solve` / `saturate` / `test` run with `--events` and counts
`fire` by rule. A firing counts for a module only when the file does not
declare that name itself and the module is in its import closure, so a puzzle's
inline `symmetric` is never credited to `std.algebra`'s. That attribution rule
is the one the cargo test re-implements, and the two have to stay one rule.

## What may be claimed here — M1d S1d.4.3

**A claim that cannot be checked at the runner's own depth does not go in.**
`ein test` exhausts and it exhausts at `--max-set-size 5`; where the frontier
is still non-empty there, `:expect` reports
[`NOT CHECKED`](../docs/kernel/defined_behaviour.md) — which is honest, is not
a pass, and **takes a failing exit code**. So the policy needs no mechanism: a
claim that does not check cannot be added without turning the gate red. It is
the failure mode of adding one, not a convention to remember.

Its cost today is **zero entries** — 59 corpus claims, 59 held, all 59 under a
search that exhausted, and 58 of them entering no commitment at all
([`closure_census.md`](../plans/m1d_satisfiability/p1d.4_model_set_closure/closure_census.md)).
Its forward cost is the **ten** corpus entries whose claim *would* come back
unchecked if one were written — `saturation/type-exclusivity/pets.ein` most
sharply, which reports `Contradiction k = 0` at `-m 5` and has **35 models** at
`-m 10`. None of the ten carries a claim, and none gets one until the search
that would check it is affordable, which is
[P1d.10](../plans/m1d_satisfiability/p1d.10_exhaustive_search/README.md)'s
subject rather than this directory's.

**The corollary for a new fixture**: keep it small enough that its answer is
reached by saturation or by a couple of layers. That is what the 56 programs
here already are, and it is why the whole suite is 0.04 s.
