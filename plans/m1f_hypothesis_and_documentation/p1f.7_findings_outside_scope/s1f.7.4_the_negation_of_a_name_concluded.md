# S1f.7.4 — A conclusion `(not ?x)` whose variable binds a name

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 15. **Done 2026-11-11**: W84, seven rows filed and fixed; refused
at load wherever the program shows that the variable can hold a name, and the firing builds no negation of a name
([the records](#t1-record-2026-11-11)).
**Depends on:** nothing.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **A conclusion `(not ?x)` whose variable binds a name stores the negation of a name.** `(rule r () :match (a
> ?x) :assert (not ?x) …)` stores `(not k :rule r)`, which no program can write (`(not k)` is a parse error as a
> fact and as a conclusion). The grammar takes `(not Var)` as a conclusion, for a variable bound to a fact.

So the store holds a fact that the language cannot state. It reaches every surface that prints a state, and a
`--dump-answer` state file that holds it does not reload.

## What is open

The grammar takes `(not ?x)` because `?x` may be bound to a fact, and whether it is depends on the data. Three
directions:

- **A refusal at load** wherever the program shows that `?x` can bind a name, read from the positions that hold
  names in the way S1f.5.18 and S1f.5.9 read relation names (`refuse_derived_relation_activators`,
  `refuse_eq_at_another_arity`). This agrees with how the phase refused shapes that no run can answer, but it
  refuses a program in which only some bindings are names.
- **A false conclusion**: the firing that would conclude the negation of a name concludes `(false)` instead, as
  S1f.5.7 treats a false `(eq …)` conclusion (G-21, Q-M1e2.26).
- **A stop** with Ein's line where the firing meets a name. This breaks *exit non-zero only on error* unless the
  program is wrong, and it is answered only where a firing meets it.

T1 measures where the corpus and the stdlib write `(not ?x)` and what each variable binds. The choice is a Q-M1f
row. The first direction is the default if the census shows no program in which a variable binds both a name and a
fact.

## Tasks

### T1f.7.4.1 — confirmation, and the census

Re-take the finding on every command, including `--dump-answer` with its reload and a saved container. Census every
`(not ?x)` conclusion in the corpus, the stdlib and the Rust suites' inline programs, and what its variable can
bind.

### T1f.7.4.2 — the rows

One witness per way the engine answers wrongly: the stored `(not k)`, the state file that does not reload, and any
claim that holds of it. Each is filed `open` and held to today's engine.

### T1f.7.4.3 — the fix

The chosen direction. The rows are flipped. A test holds every witness on every command.

### T1f.7.4.4 — the pages

`02_patterns.md` (what `(not ?x)` concludes), `defined_behaviour.md`, and the stage's records.

## Acceptance

- No run stores the negation of a name, and every command answers a program that would store one the same way.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves, or
  each that moves is named and ruled.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.4 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `55aadd0b` (`ein saturate --dump`, `ein solve`, `ein oracle`, `ein kb save`
and the container's run, the `render` views). The class, over two guests of whom Ann has paid, `(paid Ann)`:

| the program writes | at `55aadd0b` |
|---|---|
| `(rule deny () :match (paid ?g) :assert (not ?g))` | `(not Ann :rule deny)` stored; R {∅}; the state file `ein solve --dump-answer` writes holds `(not Ann)` and does not reload, *unexpected \`Ann\`: a name* |
| the same as an hrule | H is `(not Ann)`, and R is two states the read-out lists as two `(model)` lines; `ein oracle` the same |
| `(deny Bob)` beside `(rule deny (?g) :match () :assert (not ?g))` | `(not Bob)` stored |
| `(macro neg (?a) (not ?a))` and `:assert (neg ?g)` | `(not Ann)` stored; `(neg Ann)`, the name written, is refused since S1f.5.24 |
| `:match (neq ?g Ann)`, the guard alone | `(not Bob)`, `(not Guest)` and `(not paid)` stored, the guard binding over the program's names |
| `(seen ?g)` bound where another rule derives `(seen ?g)` of each guest | `(not Ann)` and `(not Bob)` stored |
| a declared `seen` with no fact, `(solve :hypothesis-relations (seen))` | R four states, three of them storing a negated guest |
| `(rule deny (?R) :match (?R ?g) :assert (not ?g))` beside `(deny paid)` | `(not Ann)` stored |
| `(claim (paid Bob)) (claim Ann)` and `(claim ?c)` to `(not ?c)` | `(not (paid Bob))` and `(not Ann)` stored |
| `:assert (said (not ?g))`, `:assert (not (not ?g))` | parse errors: the grammar takes `(not Var)` only as a whole conclusion |

A container saved from each answers as its text.

**The census.** `(not ?x)` is concluded in three corpus programs,
`examples/saturation/hypothesis-contradiction/{coloc,next-to,right-of}-disproved.ein`, each `:match (and
(hypothesis ?h) (contradiction-under ?h))`, where both relations are undeclared and every written fact of either
holds a nested fact. The stdlib concludes none (`std.macro`'s `unknown` reads `(not ?P)` in a premise). The Rust
suites conclude it in `value_rule.rs`'s `the-negation-of-a-bound-fact` and in `from_ir`'s unit lists, each over a
fact. One register witness writes it, `eq-value-negation-concluded.ein`, refused at the parse since S1f.7.10. No
program binds one variable to both a name and a fact (`git ls-files '*.ein' | xargs grep -n '(not ?'`,
2026-11-11), so the stage's default stands: refuse at load.

**What a refusal can see** is Q-M1f.127, and what the firing does with the rest Q-M1f.128, both defaulted.

## T2 record, 2026-11-11

Seven witnesses under `examples/ein-bugs/wrong-answers/`, each with a corpus entry and a row of W84:
`not-name-concluded`, `-hrule-concluded`, `-parameter`, `-macro`, `-guard`, `-derived` and `-under-commitment`. Each
one's right answer is a refusal. Filed `open` with a `[case.region]` each and held to the unchanged engine:
`ein-corpus`, `ein-cli`'s `wrong_answers` and `ein-infer`'s `register_region` green on the sources at `55aadd0b`
with the seven rows open, 283 fixed, 20 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-11).

**A program whose name only a run meets has no row.** A row's witness is a corpus program, and
`answer_round_trip.rs` holds every corpus program's `--dump-answer` state files to reload. Such a program reaches
the name through a derived activator, which its state file writes as a fact, so the loader reads it there and
refuses the file (F25's R-1). The three programs of that kind are in `negated_names.rs`, written to a scratch
directory.

Goldens named before blessing: `corpus_exits.txt` (two lines a witness) and `corpus_shapes.md5` (the witnesses'
lines).

## T3 record, 2026-11-11

Two halves.

**The loader** (`ein_ir::from_ir`'s `refuse_negated_names`, after the macros are expanded). A position, a relation
and an argument's index, holds a name where the program shows one: a written fact at any depth; a `(relation …)`
declaration; a rule's or an hrule's conclusion that writes a name or a variable that can hold one, to a fixpoint; an
activator, a fact or a `(solve :hrules …)` entry; and, where no hrule is declared and nothing writes the closure
marker, every argument of a declared relation the `(solve …)` lists leave among the hypotheses. A pattern or a
conclusion whose relation is a parameter is read as each relation a written activator names there. A conclusion
`(not ?x)` is refused where one disjunct of the `:match` binds `?x` only at such positions, or only by a guard. The
lines, as printed:

```
kb load error: examples/ein-bugs/wrong-answers/not-name-concluded.ein:29:45: rule 'deny': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where the fact (paid Ann) holds Ann — not takes a pattern (relation argument …) or a variable bound to a fact: it is the stored negation of one fact, and a name is no fact; negation as failure is (absent …)
… hrule 'suppose': its :assert writes (not ?g), and ?g can be a name: (paid ?g) binds ?g where the fact (paid Ann) holds Ann — …
… ?g can be a name: ?g is its parameter, which the fact (deny Bob) binds to Bob — …
… ?g can be a name: (neq ?g Ann) binds ?g over the program's names, nothing else binding it — …
… ?g can be a name: (seen ?g) binds ?g where rule 'greet' writes (seen ?g) — …
… ?g can be a name: (seen ?g) binds ?g where a commitment can hold a name, seen being a declared relation the hypotheses are taken from — …
```

**The firing** (`ein_infer::firing::build_fact`). A template that would hold `(not <name>)` builds no fact, so
`build_conclusion` concludes nothing for it, the saturator stores the firing's other conclusions, the lookahead
reads none, and `Hrules::candidates` proposes none. It is the one place a conclusion is built, so it holds on every
command and in both builds. What reaches it is a binding through the relation of a parameter whose activator a rule
derives through a variable; F25's R-1 says what is left.

W84's seven rows flipped to `fixed`, with W85's three: 293 fixed, 13 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). `[case.fixed]` carries the null id in the fixing commit
and the hash in the next.

A sweep of the 641 `.ein` files tracked at `55aadd0b`, the binary there against the new one, `ein saturate --dump`'s
stored facts and exit and `ein solve`'s whole output (2026-11-11), moved nothing: every file prints the same bytes,
the zebra programs and the three `hypothesis-contradiction` programs among them. The sweep covers S1f.7.2's change
too.

**Fixtures and tests moved, each named**: `value_rule.rs`'s `the-negation-of-a-bound-fact` holds its declared
relation `h` out of the hypotheses, `(solve :no-hypothesis (h))`, since a commitment's `(h a)` would bind the
negated variable to a name; `from_ir`'s unit test `a_macros_expansion_is_held_to_the_grammars_shapes` listed
`:match (a ?x) :assert (not ?x)` and `:assert (neg ?x)` as loading beside `(a k)`, and lists each over `(h (p k))`
since; `corpus_exits.txt` and `corpus_shapes.md5` gain the ten witnesses' lines. No fixture's program was
rewritten, and the three `hypothesis-contradiction` programs are unmoved.

**The pin**: `ein-cli/tests/negated_names.rs`, eighteen refused shapes and the seven witnesses on twenty
commands, ten programs that keep loading and the three corpus programs' stored negations, and three programs whose
name only a run meets. On the binary at `55aadd0b` every refused shape loads, and each of those three programs
stores `(not Ann)` (T1's probes).

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.23 (new, with S1f.7.2; § 4.16's *what is not refused* restated), `02_patterns.md` (the
primitives table's `not` row and a paragraph on what `(not ?x)` concludes), `01_grammar.md` (a paragraph beside
the conclusions' other rules), `examples/README.md`, `AGENTS.md`, F25's
R-1, the phase README and this file. The grammar is unmoved, so `00_ebnf.md` is not re-blessed.
