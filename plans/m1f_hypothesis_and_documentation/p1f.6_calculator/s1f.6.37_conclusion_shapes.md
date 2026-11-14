# S1f.6.37 — What a rule may conclude: `and` inside `and` (E-63)

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-14, on the user's word of that day; the fix ruled the same day (flatten). Not started. **An engine stage**, small.
**Depends on:** nothing. Runs after [S1f.6.17](s1f.6.17_binary_logic.md)'s build (which holds `corpus/` and the
goldens while it runs) and **before [S1f.6.16](s1f.6.16_binary_arithmetic.md)'s build**, whose connecting call
(Q-M1f.266, (iii)) then sits in each base's closing rule rather than in a rule of its own.

## The instruction

E-63 was met by S1f.6.16's design ([§ The design](s1f.6.16_binary_arithmetic.md)): a macro call that expands to
an `(and …)`, written inside a closing rule's `(and …)` conclusion, activated nothing, in silence. The user,
2026-11-14:

> E-63 is a separate stage, The rule can assert some arbitrary complext fact built from simple facts with `and`
> and `or` - how engine handles this?

## What the engine does today

Probed 2026-11-14 on a release binary of that day (`ein saturate`, `ein solve -f`), one rule `:match ()` each,
over declared `p`, `q`, `r`, `h`:

| the rule's `:assert` | today |
|---|---|
| `(p a)` | stores the fact |
| `(not (q a))` | stores the negation (a stored negative, not closed) |
| `(and (p a) (not (q a)))` | stores each conjunct |
| **`(and (r a) (and (p a) (q a)))`** | **loads; stores `(r a)` and one fact headed `and`; `(p a)` and `(q a)` never** — E-63, a wrong answer: `(query :goal (p ?x) :expect (model (p a)))` fails (witness `build/s1f.6.16/design/and_in_and.ein`) |
| `(false)` | refutes the state |
| `(eq a a)`, `(eq a b)` | a check: the first holds and stores nothing; the second kills the state (root dead) |
| `(or (p a) (q a))`, `(and (p a) (or (q a) (r a)))` | refused at parse, *unexpected `or`*; the loader carries its own refusal too (*"an (or …) is no conclusion … write one rule per disjunct"*) |
| `(h (and (p a) (q a)))`, `(h (or …))`, `(not (and …))` | refused at parse: a value is a name or an atom (AGENTS.md § The language), so no fact holds a formula |
| `()`, `(open)`, `(open ?R)` | nothing; an obligation |

So a rule concludes **a conjunction of literals** (atoms and stored negations), `(false)`, a check, or an
obligation, and nothing else: no disjunction and no compound fact. **Why E-63 slips through:** the grammar's
`Conclusion ::= '(' 'and' Conclusions ')'` nests, and the loader's check of a conclusion
(`ein-ir/src/from_ir.rs`, the `conclusion` walk) recurses through every level; the compiler splits only the top
one (`assert_conjuncts`, `ein-infer/src/compile.rs`), so an inner `(and …)` becomes a fact template headed
`and`. Premises flatten at any depth (the same file's `premise`), conclusions do not. Macro expansion makes the
shape without anyone writing it: a macro whose body is an `(and …)`, called as one conjunct.

**Where "one of" lives instead.** A rule asserts no disjunction; *A or B* is said to the hypothesis layer, an
hrule's proposals or an obligation `(open R)` and the candidates that discharge it, and each alternative is a
state of R ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)). A formula as an
object, `(And X Y)` written with ordinary names, is M1g's
([S1g.3.3](../../m1g_logic/p1g.3_proof_systems/s1g.3.3_natural_deduction.md)): to the engine it is an atom like
any other.

## Tasks

1. **T1f.6.37.1 — the register row first:** W-next `open` in `corpus/wrong_answers.toml`, the witness under
   `examples/ein-bugs/wrong-answers/`, its corpus entry, in a commit of its own.
2. **T1f.6.37.2 — the inventory of every reader of a conclusion** before the fix: the compiler's plan, the
   loader's checks (`from_ir.rs`), `ein_infer::forbid`, `ein_infer::mechanisms`, `invariant.rs`'s names, the
   obligation reader, the NL rendering of `:why`, the rule view and the IR printer (`ein-render`), `dump.rs`,
   `rule_census.py`. Each must read the same list of conjuncts.
3. **T1f.6.37.3 — the fix: flatten at any depth**, as premises are, through one function every reader of T2
   calls; the grammar unchanged. **Ruled by the user, 2026-11-14:** *"ok, flatten is fine, continue"* (the
   alternative, refusing a nested `and` at parse, is not taken). Flattening keeps the macro call of S1f.6.16
   legal inside a closing rule.
4. **T1f.6.37.4 — every row of the table pinned:** a test per shape (the grammar page's table for the parse
   refusals, `defined_behaviour.md` § 4 for the load refusals, a solve test for each stored shape), and the
   grammar page and `01_grammar.md` saying what a conclusion is in one sentence.
5. **T1f.6.37.5 — the sweep and the flip:** a census of the corpus's conclusions by shape (no tracked program
   writes a nested `and` today, so no answer may move: verified on every tracked program, old binary against
   new); W-next flipped `fixed`; the record and the phase README's row.

## Acceptance

- `(and (r a) (and (p a) (q a)))` stores `(r a)`, `(p a)` and `(q a)`, and no fact is ever headed `and`.
- Every reader of a conclusion reads the same conjuncts; a macro expanding to an `(and …)` inside an `(and …)`
  conclusion activates what it names.
- No tracked program's answer moves.

## What it must not do

- **Not add disjunctive conclusions** or compound facts: *one of* stays the hypothesis layer's.
- **Not change S1f.6.16's design** beyond letting its call sit in the closing rule.

## The standing rules

A wrong answer is a register row before its fix. Findings go to [`findings.md`](findings.md), questions to
[`open_questions.md`](../open_questions.md), each under the next free id. The gate runs once over the batch
(the user's cadence of 2026-11-13).
