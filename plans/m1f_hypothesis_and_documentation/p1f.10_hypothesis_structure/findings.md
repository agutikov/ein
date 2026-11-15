# P1f.10 — what the phase's stages found outside their scope

**Phase:** [P1f.10](README.md). Opened 2026-11-15.

A stage keeps what it meets outside its own scope here, under a heading of its own, and fixes none of it. Each
row is shown to the user at the phase's stop or its close, and is then fixed by a stage, ruled as it stands, or
given an owner. A wrong answer among them is a row of `corpus/wrong_answers.toml` before anything fixes it.

## From S1f.10.9

Met by [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md#record-2026-11-15-tasks-1-to-3)'s
tasks 1 to 3, 2026-11-15, on the release binary of `d7050ccf`. The witnesses are under the untracked
`build/s1f.10.9/h9/` and verbatim below; `build/s1f.10.9/run.sh`, which the stage's record carries, runs them.
None is fixed, and no register row is filed: the stage was told to file none.

### H9-1 — A reserved word a program writes as a value is a name of the program and never an object

The reverse of E-1: a name the blind enumerator does **not** take. `hypgen::candidate_objects` drops seven
reserved words, `not false and or absent eq neq`, and every name `Program::categorise` calls a relation, the
kernel's `relation`, `rule` and `written` among them, whether or not the program writes the word as a value.
Five of them a program can write as a value today, and each loads: `eq`, `absent`, `false`, `relation`,
`written` (`(order eq)` and its four twins, probed; `neq`, `not`, `and`, `or` and `open` are refused at the
parse). The shipped stdlib writes one: `std.math.digits`'s table `cl` holds the orders `lt`, `eq` and `gt` as
values, `(cl eq 0 0 eq)`.

`h9/eq_as_value.ein`:

```
;; S1f.10.9, H9-1: a reserved word a program writes as a value is a name of
;; the program and never an object. std.math.digits writes `eq` so, in the
;; rows of its table `cl`.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`h9/eq_as_value_written.ein`:

```
;; S1f.10.9, H9-1's twin: eq_as_value.ein with the fact the enumerator does
;; not propose written by the program. It loads and root is alive.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(pick eq)
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`ein oracle` on the first: H is `(pick gt)` and `(pick lt)`, 2 candidates, R 4 states, none holding `(pick
eq)`. On the twin: the same 2 candidates, R 4 states, every one holding `(pick eq)`, so the state the first
program never reaches is alive and owes nothing. The same on the shipped stdlib: the stage's witness
`e1/shipped_decimal_arith.ein` proposes `(total lt)` and `(total gt)` and not `(total eq)`.

**How it reads against the rulings.** `eq` in `(order eq)` is a constant in an argument position of a fact, so
it is a name of the program by Q-M1e2.9 (a), and `invariant::program_names` lists it.
[`solution_semantics.md` § 1.2](../../../docs/kernel/inference/solution_semantics.md) says *every name is in
the candidate list* and *a list that left one out was an engine bug* (W4, W15). Read so, this is a wrong
answer of that family. Read the other way, a reserved word is never an object, as a type a signature declares
is not, and § 1.2 has an exception it does not state. **Not filed as a register row**: which reading holds is
the user's, and it is the same question as Q-M1f.306's option (b) seen from the other side, whether a kernel
name a program writes as a value is an object. Not counted over the corpus.

**Today:** open, to be shown with Q-M1f.306. No owner.

### H9-2 — A written dunder fact registers the dunder as an auto-vivified relation of the program

The loader makes a registry entry for the head of every fact a file writes that no `(relation …)` declares
(`ein-ir/src/from_ir.rs`, *Undeclared relations are auto-created open-world*), and leaves a built-in predicate
out, *so vivifying one would put a phantom entry in the registry*. It does not leave the kernel's dunders out:
`ein saturate FILE --dump` on a program that writes `(__closed__ digit)` prints *Relations — auto-vivified
(1)*, `__closed__`, and on the same program with the closure derived by a rule prints none (the stage's
`e1/closed_by_fact.ein` and `e1/closed_by_rule.ein`). So whether the registry holds `__closed__` or
`__symmetric__` depends on how a closure or a symmetry came to hold, and every reader of the registry sees a
kernel name as a relation of the program in one case and not in the other.

The blind enumerator is one such reader, and what it does with the difference is E-1 and Q-M1f.306. **The
other readers were not examined**: the `relation` facts the loader states, `scope.rs`'s *outside* list,
`invariant::Universe`, the render views and the `--dump` listing itself. No wrong answer is known from them.

**Today:** open. It bears on where a fix under Q-M1f.306 (b) is made: one in the enumerator leaves this
difference in place.

### H9-3 — E-1 and E-7 are absent from the corpus's answers by the rung twelve programs take

The census of S1f.10.9 finds no corpus program with a dunder among its candidates and one with a relation's
name. On the 174 programs whose H an hrule, the obligations rung or `(solve :no-search)` builds, the blind
enumerator is not asked; were it (`hypgen::generate_blind` at root, which is not H and decides nothing), it
would name `__closed__` on 8 programs, 458 candidates, and a derived relation's name on 4, 200 candidates
(`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census`, the `alone`
columns, 2026-11-15):

| programs | rung today | the blind enumerator alone would name |
|---|---|---|
| `examples/binary_logic/{equivalent,knights,not_equivalent,satisfiable,unsatisfiable}.ein`, `stdlib/bool/inputs.ein`, `tests/stdlib/bool/inputs/{01_in_drives,02_required}.ein` | obligations | `__closed__` |
| `examples/domain_elim/{ab,b_branch,b_only}.ein` | an hrule | `surjective`, `total` |
| `examples/features/09_adjacent_via_same_house.ein` | obligations | `adjacent-via-bwd`, `adjacent-via-fwd` |

**Today:** not a wrong answer and nothing to fix. It is a dependency between two of the phase's questions:
[S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) asks which rung a program takes, and a ruling there
that sends a program to the blind enumerator makes Q-M1f.306's (a) to (c) bear on programs they do not bear on
today. The census is re-taken by the same command after such a ruling.

## From S1f.10.6

Met by [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md#record-2026-11-15-tasks-1-to-3)'s tasks 1 to 3,
2026-11-15, on the release binary of `14063474`. The programs are under the untracked `build/s1f.10.6/` and
verbatim below and in the stage's record; `build/s1f.10.6/run.sh`, which the record carries, runs them. None is
fixed, and no register row is filed: the stage was told to file none.

### H6-1 — An hrule that reads what a commitment can change proposes at a node what it did not at root, and nothing flags it

Q-M1e.11's loss without an obligation derived at a node. H is the generator's list at root's fixpoint, and an
hrule is matched there and nowhere else. Where an hrule's `:match` reads a relation another hrule proposes, the
facts it would propose once a commitment holds one are never candidates.

`hr/reads_h.ein`:

```
;; S1f.10.6, T1, the hrule rung with no derived activator: both debts are
;; stated, there is no rule `arm`, and the hrule that proposes a drink reads
;; `likes`, which the other hrule proposes.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`hr/reads_h_stated.ein`:

```
;; S1f.10.6, T1: hr/reads_h.ein with the fact its second hrule reads stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(likes Ann Soup)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`ein solve` and `ein oracle` on the first: H is `(likes Ann Soup)`, one candidate; the node that holds it owes
*drinks owes Ann a Drink*; **R is empty, every alive node owing**. On the second, which states the fact the
second hrule reads: H is `(drinks Ann Tea)` and R one state. `owed_census --iterate` on the first: the
generator at `{(likes Ann Soup)}` proposes `(drinks Ann Tea)`, and R over the two candidates is one state,
`{(drinks Ann Tea) (likes Ann Soup)}`, which `ein oracle` answers on the copy that states both
(`price/reads_h_r2.ein`).

**Nothing detects it.** No rule concludes an activator and the static check does not read an hrule's plan, so
the program is *monotone*: `ein solve` takes the state route and `--single` the obligation tree with L5. Root
owes both instances, so `check_owed` has nothing to refuse. The read-out's `outside` row is silent: no
saturation rule writes `drinks`, no query reads it, and an hrule asserts it.

**The corpus** (`owed_census`, 2026-11-15, the command in the stage's record). Four programs have an active
hrule whose `:match` reads a relation in `D`: `examples/ein-bugs/wrong-answers/hrule-reads-unknown.ein`
(`paid`), and `examples/ein-bugs/wrong-answers/past-limit-lines.ein`,
`examples/lattice/04_obligation_trigger_hole.ein` and `tests/region/07_cube.ein` (`is-a`). On none does a node
propose a fact outside H: three by the cube at the oracle's default limit, the second at `--limit 17`. Of the
88 programs on the hrule rung the cube read 61, and **no corpus answer is known to be concerned**.

**How it reads against the rulings.**
[`solution_semantics.md` § 1.4](../../../docs/kernel/inference/solution_semantics.md) defines a candidate as
*a fact the generator proposes at root*, so R is right by the definition. The same section holds that *H must
contain every fact a model needs* (ruling (h)), and that where it does not *either the program is wrong … or
the engine is*: which of the two an hrule conditioned on a guess is, is the user's. **Not filed as a register
row.**

**What it bears on.**

- **Q-M1f.307.** Answer (A) read over nodes (the record's reading 2) gives the twin's model here. Read
  statically from activators (reading 1) it does not reach this program, and (B), (C) and (D) as worded do
  not either: no activator is derived, and the program is monotone.
- **[README § The domain](README.md#the-domain)'s second paragraph**, *H does not grow*, is enforced for
  debts on a monotone program and for nothing an hrule proposes. Track S reads H as root builds it either way;
  what is not enforced is that no node would propose more.

**Today:** open, to be shown with Q-M1f.307. No owner.

### H6-2 — Under (B), Q-M1f.306 bears on two more programs, and marks nothing on them

[H9-3](#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take)'s dependency,
met from this side. Q-M1f.307's answer (B) sends `oblig-derived-activator.ein` and its `-declared` control from
the obligations rung to the blind enumerator, so Q-M1f.306's options (a) to (c) would bear on them. The
stage's stand-ins for that H, `build/s1f.10.6/blind/derived.ein` and `blind/declared.ein`, hold twelve
candidates each, `likes` and `drinks` over `Ann`, `Soup` and `Tea`, and `object_census` marks none of them: no
kernel dunder, no relation's name, no name only an import writes (`cargo run --release --manifest-path
ein.rs/Cargo.toml -p ein-infer --example object_census -- build/s1f.10.6/blind/derived.ein
build/s1f.10.6/blind/declared.ein`, 2026-11-15).

**Today:** not a wrong answer and nothing to fix. A ruling of (B) re-takes S1f.10.9's census by its own
command, as H9-3 says of S1f.10.15's.

## From S1f.10.15

Met by [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-15-tasks-1-to-3)'s tasks 1 to
3, 2026-11-15, on the release binary of `c6251db3`. The programs are under the untracked `build/s1f.10.15/`
and verbatim in the stage's record; `build/s1f.10.15/run.sh`, which the record carries, runs them. None is
fixed, and no register row is filed: the stage was told to file none.

### H15-1 — Under (b), Q-M1f.306 bears on one more corpus program, and under (c) on two

[H9-3](#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take)'s dependency,
measured. Q-M1f.308's options (b) and (c) send programs from the obligations rung to the blind enumerator,
and Q-M1f.306's question is then asked of them. Of H9-3's twelve programs (b) moves one and (c) two; the
other ten keep their rung under every option, seven owing at root and three under an hrule
(`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example rung_census -- --jobs 8
--check`, the columns `K1` to `any`, and `object_census` on the two files, 2026-11-15):

| program | moved by | new candidates | marked | what the marks name |
|---|---|---:|---:|---|
| `examples/features/09_adjacent_via_same_house.ein` | (b), (c) | 82 | 44, `R1` | `adjacent-via-bwd`, `adjacent-via-fwd`, a derived relation's name: E-7 |
| `stdlib/bool/inputs.ein` | (c) | 342 | 340: 36 `K1`, 336 `IN` | `__closed__`: E-1; the sixteen functions `std.bool.gates` writes: E-54 |

On the first, root is dead, so every candidate is dead alone and R is empty with the marked ones or without.
On the second, R goes from {∅} to an unexhausted region over the 342. No other corpus program an option moves
gains a candidate.

**Today:** not a wrong answer and nothing to fix. A ruling of (b) or (c) re-takes S1f.10.9's census by its
own command, as H9-3 says.

### H15-2 — Under (b), a program that loads today is refused: *forbid*'s premise depends on the rung

*Forbid* refuses a program whose refutation could rest on an `absent` over a relation a commitment can still
extend, and which relations those are is read off H. So what a loaded and unactivated obligation rule takes
off the blind enumerator, it also makes legal to read closed-world. `w/connex_lt_alive.ein`, the re-plan's
witness with its third pair stated:

```
;; S1f.10.15, T1: the re-plan's witness with the third pair stated, so root
;; is alive.
(import std.algebra :symbols (connex))
(relation lt Node Node :why "{?1} is less than {?2}")
(instance A Node) (instance B Node) (instance C Node)
(connex lt instance Node)
(lt A B) (lt B C) (lt A C)
(query :goal (lt ?x ?y))
```

Today: it loads, H is empty, R is {∅}, and `connex` is read over `lt` as stated. With the rung chosen by an
obligation activated, the blind enumerator proposes `(lt B A)`, `(lt C A)` and `(lt C B)`, and the program is
refused at load, exit 1: *rule 'connex' [activator lt instance Node]: concludes (false) from (absent …) over
lt, a relation a commitment can still extend … close the relation, (solve :no-hypothesis (lt))* (`ein solve`
on the stage's copy `b/connex_lt_alive.ein`, and `rung_census`'s `refusals` column, which reads
`forbid::check` over the blind list in process). The witness as described, with two pairs stated and a dead
root, is refused the same way.

**The corpus**: none. Its two programs that activate `connex`,
`tests/stdlib/algebra/08_checks_satisfied.ein` and `12_connex_violated.ein`, name `lt` in a `:no-hypothesis`
list; `rung_census` finds no program (b) or (c) moves that a load would then refuse.

**How it reads against the rulings.** Neither answer is wrong: today no commitment extends `lt`, so the
closed-world reading is the legal one by *forbid*'s own rule, and under (b) the refusal is that rule applied
to another H. What it shows is that an import decides today whether a program's `absent` is closed-world.

**Today:** open, to be shown with Q-M1f.308. No owner. Under (b), task 4's list of moved answers has to hold
programs that stop loading, of which the corpus has none.

### H15-3 — The corpus was written for the third rung: one header and 35 lists say so

`examples/features/09_adjacent_via_same_house.ein` imports `std.algebra` for `symmetric`, `transitive` and
`includes` and `std.bijection` for its two typechecks, activates no obligation, and has been on the second
rung, with no candidate, since every import brings its module whole. Its header still describes the third:
*"Argument typing, so the blind hypothesis generator stays inside the declared signatures"*, and of its
`:hypothesis-relations` list, *"if the guard comes back, root is consistent and the engine goes looking …
Four candidates is enough to report a verdict and stop."* Its corpus note says *with the guard back it
answers Ambiguity k=4*. With the guard put back (`w/features09_guard_back.ein`, the stage's `sed` over the
tracked file): today H is empty and R is {∅}, one state; on the third rung H is 82 candidates, 78 dead alone
and the four the header counts alive, and R is 9 states. The fixture still fails on the regression, a dead
root turned alive, and no longer shows what it says it would.

And 35 programs an unactivated rule holds on the second rung, 32 of them under `tests/stdlib/`, carry a
`:no-hypothesis` list that names every relation the blind enumerator would fill. On the second rung with
nothing owed the list filters nothing, and nothing notes it: the `note:` for a list that filters nothing is
written for a list beside an hrule.

**Today:** not a wrong answer. Under Q-M1f.308 (b) the header and the lists are true again as written; under
(a) the header and the corpus note are stale and task 4 restates them.

### H15-4 — The ladder is stated in five places, and three of them read as option (c)

Two say what the engine does.
[`06_reserved_names.md` § Hypothesis / query control](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md#hypothesis--query-control),
since S1f.6.34 corrected it: *a loaded obligation rule that owes nothing at root proposes nothing, and the
blind enumerator does not run*. And `oblgen.rs`'s header: *the middle row is about what the program declares,
not about what this state happens to owe*.

Three do not. [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) § 1.4 and
`AGENTS.md` say *the program's own `(hrule …)` if it declares one, else the facts that would discharge what
root owes, else the blind combinatorial enumerator*, and `hypgen.rs`'s header *obligation-driven while the
state owes something it may branch on, otherwise blind combinatorial*. Read as written, they send a program
that owes nothing to the blind enumerator, which is Q-M1f.308's option (c) and what the engine does not do:
`w/o_satisfied.ein` owes nothing and has no candidate.

**Today:** a documentation finding, no answer concerned. Whatever Q-M1f.308 rules, the sentence is restated
in all five; task 4 names `06_reserved_names.md` and the two module headers.

### H15-5 — From the second rung, the only way to the blind enumerator is an obligation nothing owes

`(solve :no-search)` is how a program says *judge root alone*. A program that imports `std.algebra`,
`std.bijection`, `std.closure`, `std.typing`, `std.slots` or `std.bool.inputs` and wants its own open
relations guessed about has no declaration for that: an hrule is the first rung and not the third, and the
two lists only take relations out. What reaches the third rung is the second rung declining, which a program
can cause by writing an obligation rule with a bare `(open)` that no state owes.
[`tests/stdlib/bijection/06_blind_enumeration.ein`](../../../tests/stdlib/bijection/06_blind_enumeration.ein)
does so on purpose, by `sorted-somewhere`, *"the file has to earn it back"*, and the stage's copies for
options (b) and (c) are made the same way, with one rule:

```
(rule s15-off () :match (and (s15-on) (absent (s15-paid))) :assert (open) :why "never owed: a bare (open) takes the obligations rung to the blind enumerator")
```

On each of the stage's programs the copy's H is the blind enumerator's list at the original's root, and its R
is the cube `rung_census` computes over that list in process.

**Today:** not a wrong answer. It bears on Q-M1f.308 (a): kept as it is, the ladder has a declared way down
to root alone and none across to the third rung.
