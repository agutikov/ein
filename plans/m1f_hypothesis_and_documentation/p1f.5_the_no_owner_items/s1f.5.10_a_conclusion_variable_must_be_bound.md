# S1f.5.10 — A conclusion's variable must be bound, and is refused at load where it is not

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.10**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-07 on the user's ruling of that day on
[F13](README.md#f13-a-conclusions-variable-that-nothing-binds). The user ruled on 2026-11-08 that P1f.5 runs, and on
2026-11-09 *"just continue with default decisions till the end of the phase"*.
**Running since 2026-11-09**, in lane 2's worktree (`p1f5-lane2`), after S1f.5.7 (`2eb483e0`): T1f.5.10.1
([its record](#t1f5101-record-2026-11-09)), every move listed, W57's four rows and W58 filed, five questions taken by
default (Q-M1f.36 to Q-M1f.40) (`6fbcf64b`); T1f.5.10.2 ([its record](#t1f5102-record-2026-11-09)), the refusal, W57
fixed (`bef85fe0`); T1f.5.10.3 ([its record](#t1f5103-record-2026-11-09)), the pages.
**Depends on:** [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md), whose `(eq ?y ?y)` is one of the two
spellings the refusal names, and which makes a guard bind. It lands before
[S1f.5.9](s1f.5.9_an_empty_match_fires.md), so that a rule with `:match ()` and an unbound conclusion
variable is refused at load rather than stopping the run at root.

## The ruling

The agent asked whether `(rule r () :match (p ?x) :assert (q ?y))` should be refused or read as *every name
is a `q`*. The user, 2026-11-07: *"looks nonsence at first, but logically it means: if exist a fact of
relation p then all objects are associated with relation q; it seems logically correct, but what is it's
value for us? Can it be rewritten with forall more explicit?"*

The agent's answer: its value is almost nil, since "every name" holds relation and type names, and what is
meant is every object of a type, which a premise already says. Ein's `forall` is a test in a `:match` and
cannot conclude a universal. The meaning stays writable explicitly, with a typed premise or, after S1f.5.7,
`(eq ?y ?y)`. So refusing the implicit form forbids nothing that can be meant and catches a likely typo. The
user: *"yes, file F13 with the refusal"*.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary.

| the program | today |
|---|---|
| `(p a)`, `(rule r () :match (p ?x) :assert (q ?y))` | loads; `ein solve` stops at the first firing, exit 1: `unbound var ?y in :assert — bindings: {'x': 'a'}`; `ein test`: an error, exit 2 |
| the same rule over `(relation s Node)`, `:match (s ?x)`, no `s` fact | loads, root never fires it, and the run stops at a node whose commitment holds a candidate `(s a)`, with the same line |
| [`examples/ein-bugs/unbound-assert-var.ein`](../../../examples/ein-bugs/unbound-assert-var.ein): `:match (or (r1 ?v1 ?v3) (r0 ?v0 ?v2)) :assert (not (r1 ?v0 ?v1))` | loads; stops at the first firing through an arm that does not bind `?v0` or `?v1`. `ein saturate` prints `KeyError: "unbound var ?v1 in :assert — bindings: {…}"`, the line [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) defines |

`firing.rs`' `FireError::UnboundVar` says *"The matcher guarantees no unbound var reaches an `:assert`
template, so this is an invariant violation"*. The rows above show the guarantee does not hold: the loader
never checks it, and the run finds it at whichever node first fires the rule.

## What it changes

**The check, at load.** Every variable of a rule's `:assert` must be bound on every way the `:match` can
hold:
- by a parameter the activator binds;
- by a positive pattern;
- or, after S1f.5.7, by an `eq` or `neq` guard, whose free variable ranges over the program's names.

A variable seen only inside an `(absent …)`, a `(not …)` matched as negated or a `forall` is not bound
outside it. In an `(or …)`, the variable must be bound in every arm. An hrule's `:assert` is held to the same
rule.

**The line.** One per variable, in Ein's spelling, naming the rule and the variable and the two explicit
spellings: *rule 'r': its :assert writes ?y, which no premise or parameter binds; to conclude it of every
object of a type, bind it with a premise such as (is-a ?y T), or of every name of the program with
(eq ?y ?y)*. Every command that loads the program exits 1, and `ein test` exits 2.

**What goes.** `FireError::UnboundVar` and its `KeyError: "unbound var …"` line, which nothing can then reach.
`defined_behaviour.md` § 4's row is replaced by the load line.

**The fixture.** `unbound-assert-var.ein` keeps its program, and its runs move from a stop at the first firing
to a refusal at load, with the corpus entry's note and golden restated. No register row records it, since it
is a regression fixture, not a wrong answer.

**What it closes.** F13, and the unbound-variable part of G-14, a bad conclusion that loads and is found
only when it fires ([S1f.5.5](s1f.5.5_the_grammar.md#questions-it-has-to-settle)). G-14's other load-time
part, ruled (b) on 2026-11-08 ([Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused)),
a derived activator binding a relation parameter to a fact, is
[S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)'s, which runs after this stage.

## The method

### Task T1f.5.10.1 — every move listed

The check prototyped, and the corpus swept with every command that loads. A program refused that loads today
is listed with its rule. The fuzzer's corpus, `corpus/fuzz_findings/`, is swept too, since the fuzzer found
the first such input.

### Task T1f.5.10.2 — the refusal

The loader's check, the line, and `FireError::UnboundVar` retired. A test in `ein-ir` holds the line for each
shape: a free variable, an `or` arm, a variable seen only under `absent`, and an hrule. `fuzz_properties` gains
the property that no firing meets an unbound variable.

### Task T1f.5.10.3 — the pages

`defined_behaviour.md` § 4, `01_grammar.md`'s rule section (every conclusion variable is bound), and the
inventory's F13.

## Acceptance

- A rule or hrule whose `:assert` names a variable that some way of satisfying its `:match` leaves unbound is
  refused at load, one line per variable, naming both explicit spellings.
- `(rule r () :match (and (p ?x) (is-a ?y House)) :assert (q ?y))` loads and answers as today.
- No run stops on an unbound conclusion variable: `FireError::UnboundVar` is gone.
- `./run_tests.sh` is green.

## T1f.5.10.1 record (2026-11-09)

**Run on the user's instruction of 2026-11-09**, *"just continue with default decisions till the end of the phase"*:
every decision a reviewer would call one is taken by default and is a row of [`open_questions.md`](../open_questions.md),
Q-M1f.36 to Q-M1f.40. The stage ran in lane 2's worktree, on `p1f5-lane2` at `2eb483e0`, after S1f.5.7, so the generated
parser is the only one and a guard binds a free variable over the program's names.

**The case, re-taken** on `2eb483e0`'s release and debug builds alike (`build/s1f.5.10/case`, `build/s1f.5.10/matrix.sh`,
every command that loads a program: `saturate --dump`, `solve`, `solve --minimal`, `oracle`, `test`):

| the program | `2eb483e0` |
|---|---|
| `(p a)`, `(rule r () :match (p ?x) :assert (q ?y))` | loads; root fires `r` and every command stops, exit 1, `unbound var ?y in :assert — bindings: {'x': 'a'}` (`saturate`: `KeyError: "…"`); `ein test` with a claim: an `ERROR`, exit 2 |
| the same rule over `(s ?x)`, no `s` fact | loads; `saturate` and `solve --minimal` answer, exit 0, R's minimal line {∅}; `solve` and `oracle` stop at a commitment holding a candidate `(s a)`, exit 1, the same line |
| `unbound-assert-var.ein` | loads; every command stops at root, exit 1, `unbound var ?v1 in :assert — bindings: {'v0': 'o2', 'v2': 'o1'}` |

So the case stands, and its second row is worse than the stage file says: two commands answer the program two others
stop on, which the user's rule that no two commands may disagree forbids.

**The shapes** (`build/s1f.5.10/shapes`, 22 programs). A variable read only inside an `(absent …)`, an `unknown` or a
`forall`, nested in a conclusion's argument, or left unbound by one arm of an `(or …)` stops the run where a firing
reaches it, as the first row does. **Three answer instead**: an hrule whose template writes one proposes nothing, in
silence (`hrule::Hrules::candidates` skips a template that builds no fact), H empty and R {∅}; and a rule that never
fires loads and answers, whether its `:match` is `()` (F12), its activator is never written, or it reads an `(absent
…)` that never holds. **What loads and must**: a variable a parameter binds; one a relation premise binds at any depth
of its arguments, a stored `(not P)` among them; one an `eq` or `neq` guard binds (S1f.5.7), `(and (p ?x) (eq ?y ?y))`;
and the acceptance's `(and (p ?x) (is-a ?y House))`. An obligation's `(open ?y)` with `?y` no parameter is refused
already (`defined_behaviour.md` § 4.2), and a nested `(or …)` stops compilation, a `CompileError`, before and after.
**One shape still reaches the stop with the check in place**: a value parameter its activator binds to a nested fact,
read only by the conclusion, `(rule act (?y) :match (p ?x) :assert (hit ?y))` under `(act (f a))`. The compiler skips
a nested fact when it seeds a parameter (*a nested Fact does not bind*), W33's cause, so `?y` reaches the conclusion
unbound. A loader cannot see it, a parameter being bound by its activator, and the user's ruling of 2026-11-08 on
S1f.5.5's census makes the program answer `(hit (f a))`, which is [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)'s
fix.

**The prototype** (`build/s1f.5.10/prototype.patch`): `ein_ir::from_ir`'s `refuse_unbound_conclusions`, called after
the loader's other rule-shape refusals, reads a rule's `:match` as the compiler does, one way per disjunct of a top-level
`(or …)`, and refuses each `:assert` variable some way leaves unbound. Every program of the five roots, 533 files
(`find examples tests stdlib corpus plans -name '*.ein'`, `corpus/fuzz_findings/`'s ten among them), went through
`2eb483e0`'s release build and the prototype's: `solve --json-summary -H --dump-answer`, `saturate --dump`, `test` and
`oracle`, the exit, stdout and stderr of each, the summary and every file the dump wrote compared, timings masked
(`build/s1f.5.10/xcheck/run_all.sh`, 92 s). **531 are the same** and two move:

| program | what moves |
|---|---|
| `examples/ein-bugs/unbound-assert-var.ein` | the stage's fixture: every command's stop at root → a refusal at load, two lines, `?v0` unbound where the `:match` holds through `(r1 ?v1 ?v3)` and `?v1` where it holds through `(r0 ?v0 ?v2)`; exit codes unmoved (1, `ein test` 2) |
| `examples/syntax/rule-nary-and-or.ein` | a node-kind fixture whose `:assert`, `(quad ?d ?c ?b ?a)`, writes `?c` and `?d`, which its second disjunct, `(?rel ?a ?b)`, leaves unbound; no activator is written, so it answered R {∅}, exit 0, and is refused at load (Q-M1f.39: rewritten by T1f.5.10.2, its answer unmoved) |

No other corpus program, stdlib rule, fuzz finding or plan probe writes a conclusion variable its premises leave unbound.

**The rows** ([Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row),
[Q-M1f.37](../open_questions.md#q-m1f37--fireerrorunboundvars-last-path-w33s-cause)). The stage file says its fixture
needs no row, being a regression fixture. The register reads a run that stops and a run refused at load as two answers
(`Now::against`: *an error is no answer at all, so it is never the right one*, W18's stops filed as rows), so each way
the engine answers such a program today is a row whose `right` is the refusal; filed `open`, owned by S1f.5.10, each
witness under `examples/ein-bugs/wrong-answers/` with its hand derivation and a corpus entry, each `wrong` and `now`
taken at `2eb483e0`, release and debug alike:

| row | witness | `now`, today | `right` |
|---|---|---|---|
| `W57/assert-var-stops` | `assert-var-stops.ein`: root fires `r` | every command stops, exit 1, the unbound line | refused at load |
| `W57/assert-var-under-commitment` | `assert-var-under-commitment.ein`: only a commitment fires `r` | describe and `oracle` stop; `saturate` and `--minimal` answer, exit 0 (its `wrong`; the corpus entry banks all three runs) | refused at load on every command |
| `W57/hrule-assert-var` | `hrule-assert-var.ein`: an hrule's template | H empty, R {∅}, exit 0 | refused at load |
| `W57/assert-var-never-fires` | `assert-var-never-fires.ein`: `:match ()` | R {∅}, exit 0 | refused at load, before S1f.5.9 makes `()` fire |

The fixture stays no row: its program is `W57/assert-var-stops`'s shape through an `(or …)` arm. **Found outside the
case, filed**: `W58/parameter-nested-assert`, `parameter-nested-assert.ein`, the program above, `now` the stop at root,
`right` root holding `(hit (f a))` (its twin, the rule written with `(hit (f a))` in its conclusion, answers so),
`open` and owned by S1f.5.18, which binds the parameter and with it retires `FireError::UnboundVar`, whose last path
this is. Until then `FireError::UnboundVar` stays, and the acceptance item *`FireError::UnboundVar` is gone* is
S1f.5.18's (Q-M1f.37, taken by default). The register then reads **140 fixed, 24 open and 1 not wrong** (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**The line** ([Q-M1f.38](../open_questions.md#q-m1f38--the-refusals-line-its-separator-and-a-disjunction)) is the
stage file's in every word but its separator: the loader joins several lines with `; ` on one stderr line, so the
finding and the advice are separated by ` — `, as every loader line's are; and where some disjuncts of a top-level `(or
…)` bind the variable, it names those that do not, *… which no premise or parameter binds where its :match holds
through (r1 ?v1 ?v3) — …*.

**The fuzz property** ([Q-M1f.40](../open_questions.md#q-m1f40--where-the-fuzz-property-lives-and-what-it-reads)) is
`fuzz_properties`', over every plan the compiler builds for a generated program the loader accepts: each conclusion
reads only registers each disjunct binds, a parameter excepted until S1f.5.18. `ein-ir` takes `ein-infer` as a
dev-dependency for it, as `ein-infer` takes `ein-render`.

**What moves by banking**, with no engine change: `corpus_exits.txt` gains the five witnesses' eleven cells, the
commitment witness's three among them (`solve` 1, `solve --minimal` 0, `saturate` 0), and `corpus_shapes.md5` 205 digests,
41 per witness, none of any other program moved; `examples/README.md` names the witnesses (154 → 159 programs under
`ein-bugs/wrong-answers/`); and `ein-cli/tests/region_modes.rs`' `one_verdict_on_every_corpus_program`, whose list of
the programs whose read-out errs gains `assert-var-under-commitment.ein`, whose `--monotone` answers where its read-out
stops at a commitment's firing, W57's disagreement seen from one more pair of runs, until T1f.5.10.2 refuses it. The
prototype is set aside as `build/s1f.5.10/prototype.patch`; T1f.5.10.2 lands it.

## T1f.5.10.2 record (2026-11-09)

**The refusal** (`ein_ir::from_ir`'s `refuse_unbound_conclusions`, the prototype of T1f.5.10.1): after the loader's
rule-shape refusals (`refuse_no_fact_holds`) and before the verdict atom's (`validate_open`), it reads a rule's or an
hrule's `:match` as the compiler does, one way per disjunct of a top-level `(or …)`. A way binds the rule's parameters,
every variable of a relation premise at any depth of its arguments (a stored `(not P)` among them), and every variable
of an `eq` or `neq` guard; an `(absent …)` binds nothing outside it, so neither does a `forall` or an `unknown`, which
expand to one; `()` binds nothing; a nested `(or …)` binds what each of its arms binds, and the compiler refuses it
anyway (a `CompileError`, before and after). Each variable the `:assert` writes, at any depth and in the order written,
that some way leaves unbound is one line, `; `-joined with the loader's others (Q-M1f.38):

```text
rule 'r': its :assert writes ?y, which no premise or parameter binds — to conclude it of every object of a type,
bind it with a premise such as (is-a ?y T), or of every name of the program with (eq ?y ?y)
rule 'fire-3': its :assert writes ?v0, which no premise or parameter binds where its :match holds through (r1 ?v1 ?v3)
— …
```

An `(open …)` conclusion names a relation, which `validate_open` already checks (`defined_behaviour.md` § 4.2), so it
is not read here. Every command that loads the program refuses it with the line, exit 1 (`ein test`: 2): `saturate`,
every `solve` mode, `oracle`, `test`, `render lattice`, `render kb` and `kb save`.

**What stays** (Q-M1f.37): `FireError::UnboundVar` and its `KeyError: "unbound var …"` line, which W58's shape, a
parameter its activator binds to a nested fact read only by a conclusion, still reaches until S1f.5.18 binds the
parameter. Its doc says so, as do `firing`'s unit test (a parameter no activator bound, where it wrote a variable no
premise bound), `hrule.rs`' comment (no hrule reaches it: the loader refuses the template, and a `:hrules` activator
binds names only) and `ein-cli`'s `common::saturate_error_line`.

**The tests.**
- `ein-ir`'s `from_ir::tests::a_conclusion_variable_nothing_binds_is_refused_at_load`: the line for each shape the stage
  file names, a free variable, an `(or …)` arm, a variable seen only under an `(absent …)` and an hrule, and a guard's
  under an `(absent …)`, a nested conclusion's, an empty `:match`'s, two in one conclusion (two lines, in order) and a
  `forall`'s; and eleven rules that bind and load, a parameter, a nested premise, a stored `(not P)`, `eq`, `neq` and
  `(not (eq …))`, the acceptance's `(and (p ?x) (is-a ?y House))`, an `(or …)` whose arms both bind, and an hrule.
- `ein-cli`'s `cli_semantics::a_conclusion_variable_nothing_binds_is_refused_at_load`: W57's four witnesses and the
  fixture on the eight commands that load, the exit and the line, with no `KeyError`, `bindings: {` or `unbound var`
  on stderr; a program whose typed premise, `eq` guard and parameter each bind, which loads and concludes; and W58's
  stop, unmoved, S1f.5.18's.
- `ein-ir`'s `fuzz_properties::no_firing_meets_an_unbound_variable`, the stage file's property (Q-M1f.40): for every
  generated program the loader accepts, every plan the compiler builds for its rules and hrules reads, in each
  conclusion, only registers each disjunct binds, a parameter excepted until S1f.5.18. `ein-ir` takes `ein-infer` as a
  dev-dependency for it. The per-commit stream loads 425 programs of its 2 000 and refuses 3 with the line (`cargo test
  -p ein-ir --test fuzz_properties no_firing -- --nocapture`, 2026-11-09); at 20 000, a release build, 4 189 and 15, in
  26.9 s, which the parse-only property takes too (26.98 s), the parse being the cost.
- `ein-infer`'s `rule_semantics::the_forall_bound_variable_does_not_escape_the_guard`: a head that names the
  `forall`'s `?q` is refused at load, naming `?q` and not `?p`, where its first firing stopped the run.

**Moved**, every one a row, the fixture or Q-M1f.39's rewrite: W57's four rows `fixed`, each `now` the refusal and no
region; `examples/ein-bugs/unbound-assert-var.ein` refused at load with two lines, its program kept, its header and
corpus note restated; `examples/syntax/rule-nary-and-or.ein` rewritten, its `:assert` `(quad ?b ?a ?b ?a)`, its answer
R {∅} as before (`ein solve` on the old program with `2eb483e0`'s build and on the new with this one); and
`region_modes`' list of programs whose read-out errs back to its two. Against `2eb483e0`'s build every program of the
five roots but the fixture and W57's four witnesses is the same (`build/s1f.5.10/xcheck`, 538 files, 533 the same,
93 s, 2026-11-09), W58's witness among the same. Banked: `corpus_exits.txt` six cells 0 → 1, W57's three answering
witnesses'; `corpus_shapes.md5` 152 digests removed and 47 added, the five refused programs each losing the 21
renderings a loaded program draws and changing 7, and `rule-nary-and-or`'s 12 IR and rule views, none of any other
program. The register reads **144 fixed, 20 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-09).

## T1f.5.10.3 record (2026-11-09)

**The pages.** No current kernel page describes a conclusion's variable that nothing binds as loading, or the run's stop
as what every such program meets:

- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) § 4's row says what still reaches the `KeyError`
  line, a parameter its activator binds to a nested fact (W58, until S1f.5.18), and a new
  [§ 4.13](../../../docs/kernel/defined_behaviour.md#413-a-conclusions-variable-nothing-binds--m1f-s1f510) states the
  refusal: the line and its disjunction form, what binds, what it replaced (W57), and what pins it.
- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)'s *Conclusion forms in `:assert`*: every variable
  a conclusion writes is bound, on every way the `:match` can hold, an hrule's too, and what the refusal names.
- The inventory's [F13](../../m1e2_correctness/inventory_2026-11-06_no_owner.md) and
  [G-14](../../m1e2_correctness/inventory_2026-11-06.md) say what did it, G-14's derived activator left to S1f.5.18;
  and Q-M1e2.26's F13 paragraph ([`open_questions.md`](../../m1e2_correctness/open_questions.md#q-m1e226--are-eq-and-neq-relations))
  says it is built.
- [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)'s stage file gains W58: its seed fixes W58 with W33, and it
  retires `FireError::UnboundVar`, its line, and `fuzz_properties`' parameter exception, the acceptance item S1f.5.10
  moved to it (Q-M1f.37).

W57's four `[case.fixed]` blocks carry `bef85fe0`, T1f.5.10.2's, where that commit wrote the null id. The phase README's
F13 and stage row, `AGENTS.md` and the live read-out page are the merge's, which T1f.5.10.4's § For the merge lists.
