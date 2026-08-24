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
| [`stdlib/algebra/`](stdlib/algebra/) | 38 | the copiers and the relative product; the Boolean lattice; the extensive operators and their closed-world caveat; the tag lemmas and the two Tarski equations; Schröder's negative propagation; then the seven property **checks** — once satisfied, once violated apiece — and the totality pair's `forall` in both directions |
| [`stdlib/bijection/`](stdlib/bijection/) | 8 | the two setup fan-outs and the negative completion; domain- and range-elimination, each productive in its own file and redundant in its sibling's; the two arg typechecks violated |
| [`stdlib/elim/`](stdlib/elim/) | 4 | the same inference in its *positional* formulation, plus `no-room-left` — the pair that brackets a quantifier: one exclusion short of full forces a value, exactly full refutes |
| [`stdlib/closure/`](stdlib/closure/) | 1 | `infer-closure`, and its soundness caveat **exhibited**: a program the import takes from fifty-four models to one |
| [`stdlib/slots/`](stdlib/slots/) | 18 | a second activating puzzle for the module that had exactly one — the partition chain; `slot-fill` and `slot-elimination` as a matched pair, each productive where the other cannot fire; both violation duals; and the spatial family over two different position structures, one of which exists nowhere else and is what tells `slot-prune-fwd` from `slot-prune-bwd` |
| [`stdlib/typing/`](stdlib/typing/) | 4 | the reflexive-closure knob, and the `(type-hierarchy …)` knob in both directions |
| [`stdlib/macro/`](stdlib/macro/) | — | `forall` checked against its own expansion written out by hand, and `open`'s three states |

### Four things about how they are written

**Naming a relation closes it.** An expectation lists a relation's *complete*
extent, so a rule that derived one fact too many fails the file. That is what
makes a "must not fire" case checkable at all for the 33 rules that carry no
guard: their only *must not* is scope, and scope is a claim about a whole
relation. Where a rule needs a relation to stay untouched, the fixture gives it
one authored edge and closes it there —
[Q-M1c.6](../docs/history/m1c_external_validation/open_questions.md#q-m1c6--how-does-an-expectation-say-a-relation-is-empty)
is why an *empty* relation cannot be named.

**`(open …)` is how a fixture says a negative was *not* invented.** Stored
negatives are deliberately not closed by an expectation, so listing the
exclusions that exist says nothing about the ones that do not. Ten programs
carry a four-line `probe-undecided` rule whose body is `std.macro`'s `(open
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
