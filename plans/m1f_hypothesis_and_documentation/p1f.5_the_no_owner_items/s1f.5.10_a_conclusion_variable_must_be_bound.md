# S1f.5.10 — A conclusion's variable must be bound, and is refused at load where it is not

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.10**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** half a day.
- An hour to sweep the corpus and list every move.
- Two hours for the loader's check and its line.
- An hour for the fixture, `defined_behaviour.md` and the pages.

**Status:** specified 2026-11-07 on the user's ruling of that day on
[F13](README.md#f13-a-conclusions-variable-that-nothing-binds).
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
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
only when it fires ([S1f.5.5](s1f.5.5_the_grammar.md#questions-it-has-to-settle)).

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
