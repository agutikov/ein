# tests/

**Ein programs whose job is to fail when the engine is wrong.**

Everything here is an `.ein` file carrying an `:expect` (a query's, or since
M1e2 S1e2.1a.4 the region claim on `(solve …)`), so it states its own answer
and `ein test` turns the directory into a status code:

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

## `region/` — the region claim, one program per form

M1e2 [S1e2.1a.4](../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
Each program states its expected region on `(solve … :expect …)`, and `ein
test` checks it against R as `ein solve` describes it. That is route 1, or the
cube on a flagged program, and never the walk. The claims were rendered from
each program's computed R, the way
[`region_examples.md`](../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)'s
were.

| file | the form |
|---|---|
| `01_states.ein` | states, ∅ among them as `(model)` |
| `02_merged_single.ein` | `(merged …)` on every state, one `(model …)` from each |
| `03_merged_or.ein` | `(merged …)` with an `(or …)` of two commitments of different sizes |
| `04_owing.ein` | `(owing …)`: R empty, root owes |
| `05_empty.ein` | `(region)`: R empty, either kind |
| `06_false.ein` | `(false)`: every node dead |
| `07_cube.ein` | a flagged program, checked against the cube |

Every claim here holds. Their false twins, and the claims a route cannot
check, are `ein-infer/tests/claim_semantics.rs`. Corpus group `positive`,
runs `test` and `saturate`.

## `text/` — natural-language text

M1f [S1f.5.2](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)
(T1f.5.2.2). A string is natural-language text, written only after `:why`,
`:source` and `:goal-text`, and it is Unicode with reasonable escapes (the
user, 2026-11-06). `01_unicode.ein` writes each of the three with Cyrillic,
CJK, an emoji and every escape a string reads, `\n \t \r \" \\` and `\u{…}`,
and one `:source` with a raw newline; its claim is about facts, which the text
does not move. `ein-cli/tests/natural_language_text.rs` holds the text byte for
byte on every surface that carries it: the read-out and `answer.txt`,
`--trace`, `ein saturate --dump`, a `--dump-answer` state file, `ein render
program` and a `.einb`. Corpus group `positive`, runs `test`, `solve` and
`saturate`.

## `stdlib/` — the stdlib conformance corpus

M1c [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus).
One program per stdlib rule or tight family, each the smallest thing that
activates it and each stating what it should and should not derive.

**Why they exist.** Every other check in this repository is *relative*: the
goldens compare ein.rs to its own past. The stdlib was where that gap was
widest —
[the census](../docs/history/m1c_external_validation/stdlib_census.md)
measured **38 of 73 rules never firing** on 2026-08-23 over 400 corpus runs,
33 of them never even loaded, and 20 more held up by `examples/zebra.ein`
alone.

| dir | rules | what the programs do |
|---|---:|---|
| [`stdlib/algebra/`](stdlib/algebra/) | 40 | the copiers and the relative product; the Boolean lattice; the extensive operators and their closed-world caveat; the tag lemmas and the two Tarski equations; Schröder's negative propagation; then the seven property **checks** — once satisfied, once violated apiece — the totality pair's `forall` in both directions, and (M1d S1d.2.4) the totality pair's **obligation** duals, each once owing and once satisfied |
| [`stdlib/bijection/`](stdlib/bijection/) | 8 | the two setup fan-outs and the negative completion; domain- and range-elimination, each productive in its own file and redundant in its sibling's; the two arg typechecks violated; and `06_blind_enumeration.ein`, the one program here that leaves the enumerator **on** — since M1d S1d.2.5 by making the obligations rung *decline*, which is the only way to reach it from a program that declares one |
| [`stdlib/elim/`](stdlib/elim/) | 4 | the same inference in its *positional* formulation, plus `no-room-left` — the pair that brackets a quantifier: one exclusion short of full forces a value, exactly full refutes |
| [`stdlib/closure/`](stdlib/closure/) | 1 | `infer-closure`, and its soundness caveat **exhibited**: a program the import takes from fifty-four models to one; plus the closed-and-owing pair, which since M1d S1d.2.6 is the only place in the suite where **one fact decides a verdict word** — `02` is `Solution` and `03` is `Open — owes 1`, and what put them in scope to differ at all is one `(total-owed r is-a)` line apiece |
| [`stdlib/slots/`](stdlib/slots/) | 20 | a second activating puzzle for the module that had exactly one — the partition chain; `slot-fill` and `slot-elimination` as a matched pair, each productive where the other cannot fire; both violation duals; and the spatial family over two different position structures, one of which exists nowhere else and is what tells `slot-prune-fwd` from `slot-prune-bwd` |
| [`stdlib/typing/`](stdlib/typing/) | 4 | the reflexive-closure knob, and the `(type-hierarchy …)` knob in both directions |
| [`stdlib/digits/`](stdlib/digits/) | — | `std.digits` is a library of **facts** (M1f S1f.6.2) and has no rule to activate; see [below](#a-module-of-facts-has-no-rule-to-activate). Two programs: every table imported flat with a row of each claimed and R = {∅}; and a refutation through a lookup, whose core names the row |
| [`stdlib/macro/`](stdlib/macro/) | — | `forall` checked against its own expansion written out by hand, and `unknown`'s three states |

### The obligation pairs, and the check that is not an `:expect`

M1d [S1d.2.4](../docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
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

### A module of facts has no rule to activate

M1f [S1f.6.2](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.2_digits_and_tables.md).
`std.digits` holds the calculator's seven digit tables as plain facts, 1 664
rows, and no rule. The coverage gate asks that every stdlib *rule* be
activated by a program here, so it has nothing to ask of this module, and
`stdlib_coverage.rs` names it as rule-free, beside `std.macro`, rather than
inferring that a module with no rule is fine. The mutation sweep likewise
mutates rules and has none here.

**What holds the rows is not a program here.** A wrong row is a wrong answer
in every sum that reads it, and no `.ein` program can say what 7 + 5 is
without the table it would be checking. So the rows are held from outside the
language: [`ein-ir/tests/digit_tables.rs`](../ein.rs/crates/ein-ir/tests/digit_tables.rs)
computes every table from Rust's integers and compares, none missing and none
twice, and runs the generator's `--check`.

The two programs under [`stdlib/digits/`](stdlib/digits/) hold what only a
program can show:

| file | what it holds |
|---|---|
| `01_tables.ein` | a flat import brings a listed table's rows (S1f.6.11's fix, W88, seen from the stdlib); each goal is a lookup, constants where the table reads and variables where it gives, and its claim closes the goal, so a second row for one reading fails the file; `nz` and `adj` are claimed whole; and with `__closed__` listed the module's own closing facts leave **no candidate**, R = {∅}, claimed on the `(solve …)` form with no `:hypothesis-relations` list to help |
| `02_a_core_names_its_rows.ein` | a refutation through a lookup: root is dead and the unsat core is `(column 7 5 3)` and `(fa 7 5 0 2 1)`, the claim and the row it was checked against. The agreeing `(column 7 5 2)` is there so the rule is matched and silent once (*a refutation rule gets two files*, here two facts) |

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
an ordinary positive fact the expectation *can* close. It reads under an
`absent`, so the boundary judges it once the closure is at its fixpoint, which
is what a probe that reads the world needs: to be the last thing to read it.
(Until M1f S1f.5.15 removed `:priority` it also carried priority 500, above
everything the stdlib declared.)

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
needed. **Since M1e2's switch one form can say that a state owes**, though not
how much: `(solve … :expect (region (owing (model …))))` names an owing state,
which R, the states that are alive *and discharged*, does not hold
([`solution_semantics.md`](../docs/kernel/inference/solution_semantics.md)).
The owe counts are still `obligation_reports.rs`'s. Eight mutants died of that
word (below).

**157 of 217 on 2026-08-31, and it is an instrument now** —
[`utils/stdlib_mutants.py`](../utils/stdlib_mutants.py), M1e S1e.3.6 T6, the
review's `TE-M6`. The sweep began as **50 of 51**, hand-taken: one deliberate
defect per rule family — a dropped `neq`, an exchanged pair of `absent`
operands, a `forall` over the wrong type, a fan-out missing an activator —
injected into a copy of `stdlib/` and run past `ein test tests/`. Every one of
the five separations above was added *because* a mutant survived without it,
so the sweep is not decoration: it is what built this directory.

The two numbers are different measurements. The script is **exhaustive** over
four mechanical families where the hand sweep was one per family, and it runs
in 7 s, so it can be re-taken rather than remembered. The survivors are banked
as a *set* in [`mutation_survivors.txt`](mutation_survivors.txt) — a score
decays while the thing it guards grows, which is the same finding as `TE-M2`'s
floors — and `--check` fails on a new one **and** on a banked one that has since
been killed.

**The recorded survivor is dead.** `slot-adjacent-bwd-neg` with its two
structure operands exchanged survived from M1c to M1e: on the three-seat row of
`slots/08_spatial_adjacent.ein` the exclusion it should derive is reachable
from the other clue's chain a pass later.
[`slots/13_adjacent_bwd_neg_direction.ein`](stdlib/slots/13_adjacent_bwd_neg_direction.ein)
is the four-seat row that isolates it, and it killed two of its siblings on the
way. What it needed was the observation that the exchanged rule is **sound and
strictly weaker** — its guard becomes satisfiable only where the conclusion is
already true for another reason — so no contradiction fixture could ever have
caught it, and only a program that *needs* the derivation can.

**165 of 217 on 2026-11-03: the region claim killed eight**
(`python3 utils/stdlib_mutants.py`, M1e2
[S1e2.3.6](../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.6_the_nightly_workflow.md)).
The nightly found them two weeks after they died: its `--check` failed on
2026-11-02 and 11-03 with eight banked survivors now killed and none new, and
from 10-17 to 11-01 a step before it had failed first. All eight are mutants of
the four obligation rules, and they had survived for the reason the first
paragraph of this section gives: no fixture could say what a state owes, so a
mutant that changed only *whether* a debt is owed left every fact where it was.
The region claim says it. A mutant that stops a debt being owed puts the state
in R where the claim says R is empty, and one that invents a debt takes a
discharged state out of R where the claim lists it. The claim's fixtures came
in two commits, and the kills with them:

| survivor until 2026-10-16 | killed by | what the run says | since |
|---|---|---|---|
| `algebra.ein::total-owed::swap-premise::(?isa ?a ?A)` | `region/04_owing`, `region/05_empty`, `region/07_cube`; since the switch also `stdlib/algebra/23_total_owed` and `stdlib/closure/03_closed_and_owing` | *the claim says R is empty, and R has 1 state*; on `07_cube`, *the claim lists 3 states and R has 4* | `0f9a6656`, T1e2.1a.4.2: the region claim, with its fixtures under `region/` |
| `algebra.ein::surjective-owed::swap-premise::(?isa ?b ?B)` | `stdlib/algebra/25_surjective_owed` | *the claim says R is empty, and R has 1 state* | `fbcae3b2`, S1e2.1a.5's switch, which moved every fixture's claim about a state to `(solve … :expect (region …))` |
| `slots.ein::slot-owed-room::swap-premise::(?isa ?a ?Ta)` | `stdlib/slots/09_owed_room` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-room::swap-premise::(?sub ?Ta ?super)` | `stdlib/slots/09_owed_room` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-fill::swap-premise::(?isa ?i ?index)` | `stdlib/slots/11_owed_fill` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-room::drop-neq::(neq ?Ta ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `10_owed_room_satisfied` | *the claim lists 1 state and R is empty*: a seat now owes a seat | `fbcae3b2` |
| `slots.ein::slot-owed-fill::drop-neq::(neq ?Tv ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `12_owed_fill_satisfied` | the same, on the fill side | `fbcae3b2` |
| `slots.ein::slot-owed-fill::swap-premise::(?sub ?Tv ?super)` | the same three | *the claim lists 1 state and R is empty* | `fbcae3b2` |

The *since* column is measured, not read off the history: each commit's own
`ein`, `tests/` and `stdlib/`, the mutant applied, `ein test tests/` run. All
eight survive at `9c2815e0`, the commit before the region claim; at
`0f9a6656` and at `d09ba6a7`, the switch's parent, `total-owed`'s alone dies;
at `fbcae3b2` all eight do, killed by the files above (S1e2.3.6, 2026-11-03).
`stdlib/` has not changed since the last green nightly (`git diff 3fde4ef1
HEAD -- stdlib/` is empty), so the kills are the suite's and the engine's.
**The bank's diff is those eight lines, removed, and nothing added**: 60 → 52,
re-banked by `--bless` once each line was explained here.

**167 of 217 on 2026-11-08: *forbid* killed two**
(`python3 utils/stdlib_mutants.py`, M1f
[S1f.5.17](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.17_ci_green_again.md)).
The nightly's `--check` failed on them on 2026-11-06 and 11-07, its first two
runs after they died, and on this machine too, so the platform is not what
killed them:

| survivor until 2026-11-04 | killed by | what the run says | since |
|---|---|---|---|
| `bijection.ein::bijective-setup::swap-assert::(total ?R ?isa)` | `stdlib/bijection/06_blind_enumeration` | refused at load, exit 2: *rule 'total' [activator is-a seats]: concludes (false) from (absent …) over seats, a relation a commitment can still extend* | `597b2b7e`, T1e2.2.5.2: *forbid*'s refusal |
| `bijection.ein::bijective-setup::swap-assert::(surjective ?R ?isa)` | the same | the same, of *rule 'surjective'* | `597b2b7e` |

The swap turns the activator `(total ?R ?isa)` into `(total is-a seats)`, so
`std.algebra`'s `total` reads `seats` under `absent`. `06_blind_enumeration` is
the one program in `stdlib/bijection/` that leaves the blind enumerator on, so
`seats` is a relation a commitment can extend, and *forbid* refuses the
program. A refusal is a kill: the script counts any non-zero exit of `ein test
tests/`. The *since* column is measured as the table above measures it: `git
bisect run` over `3ee7796a..916d97ac` names `597b2b7e`. At its parent
`c887a35b` both mutants pass (66 held, exit 0), and at `597b2b7e` both are
refused. `stdlib/` and `tests/` are unchanged since `3ee7796a`, the last green
nightly's commit (S1f.5.17, 2026-11-08). **The bank's diff is those two lines,
removed, and nothing added**: 52 → 50.

**Where the remaining 50 are.** Forty-two in `slots.ein`, which has the most
parameterised, direction-sensitive rules and the fewest programs per rule; the
rest split between `algebra` (4), `bijection` (2) and `elim` (2)
(`grep -c '^slots' tests/mutation_survivors.txt`, 2026-11-08). That distribution is the
useful part of the number, and it is what a score alone never said.

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
suite has stood on its own since — **73 of 73 with no `examples/` entry
contributing** when S1c.1.4 closed on 2026-08-24, and *all of them* today,
which is not a number this page has to keep because
`stdlib_coverage.rs::every_stdlib_rule_is_activated_by_a_program` fails
the moment it stops being true.

The third file every entry here is swept by is `corpus/corpus.toml` — five runs
each but one, so this directory's share of the sweep is
`grep -c 'path *= *"tests/stdlib' corpus/corpus.toml` entries out of
`grep -c '^\[\[entry\]\]' corpus/corpus.toml`, and `wc -l
ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt` is the whole cell count.
It cost **0.72 s** of the sweep's 5.1 s on 2026-08-24.

### Re-measuring the coverage

The gate is a yes/no. The *numbers* — firings per rule, productive vs
redundant, who the sole activator is — stay with the instrument:

```sh
python3 utils/stdlib_census.py                     # the table, every corpus entry
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

**A claim that cannot be checked at the runner's own budget does not go in.**
`ein test` runs the question each claim is about, at the default
`--region-budget` since M1e2 S1e2.1a.5 (the walk's `--max-set-size 5` before);
where the run stops short of R there, `:expect` reports
[`NOT CHECKED`](../docs/kernel/defined_behaviour.md) — which is honest, is not
a pass, and **takes a failing exit code**. So the policy needs no mechanism: a
claim that does not check cannot be added without turning the gate red. It is
the failure mode of adding one, not a convention to remember.

Its cost today is **zero entries** — 59 corpus claims, 59 held, all 59 under a
search that exhausted, and 58 of them entering no commitment at all
([`closure_census.md`](../docs/history/m1d_satisfiability/closure_census.md)).
Its forward cost was the **ten** corpus entries whose claim *would* have come
back unchecked under the walk — `saturation/type-exclusivity/pets.ein` most
sharply, which the walk called `Contradiction k = 0` at `-m 5` and which has
**35 models** at `-m 10`. Under R the set is the entries a describe does not
exhaust at the default budget — the zebra family among them, and `pets.ein`
is not: its R, 11 339 states, is exhausted. Making those affordable is
P1e2.1c's speed, not this directory's.

**The corollary for a new fixture**: keep it small enough that its answer is
reached by saturation or by a couple of layers. That is what every program here
already is, and it is why the whole suite is 0.04 s.
