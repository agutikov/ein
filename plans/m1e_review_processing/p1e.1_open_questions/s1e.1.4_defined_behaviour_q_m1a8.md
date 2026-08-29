# S1e.1.4 — Q-M1a.8's real trigger: Q3

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 1 day
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) T1 — this stage
is the standard of proof's first real customer, because its likely outcome is
a **refutation** of a page the tree calls normative.
**Blocks:** [CD-H3](../p1e.2_high/s1e.2.2_code_doc_consistency.md) — the fix
there is a doc correction, an engine bug or a closure, and this stage decides
which.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q3.

## Context

[`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) exists
because `ein.py` was deleted: it is what *"whatever ein.py did"* used to
define, promoted to a normative page and checked by `cargo test`. §3.2 is the
one item the page itself flags as **a latent bug rather than a quirk**, filed
as `Q-M1a.8` and cited from the README's Known gaps. It promises that two
activators differing only in an integer argument can suppress each other's
firings, silently.

The review probed it directly — two activators `(walk edge 1)` and
`(walk edge 2)` — and got **both** firings, because `BindingKey.values`
includes the int-seeded register. So one of two things is true:

- the engine does not have the documented bug, and the page's one
  self-declared latent bug is **false**; or
- the collision exists in a narrower shape — nested-`Fact` activator
  arguments, which bind nothing and so stay out of both halves of the key —
  and every statement of it names the **wrong trigger**: *"integer
  argument"*, *"a puzzle whose rule parameters are integers can lose a
  firing"*.

Either way nothing pins the claimed suppression, in either direction, and
anyone triaging `Q-M1a.8` starts from a recipe that does not reproduce. There
is a second reason to care: `refresh_collision_risk`
([`saturator.rs:1182-1204`](../../../ein.rs/crates/ein-infer/src/saturator.rs))
spends real conservatism on the same asymmetry, so if the asymmetry is not
what §3.2 says it is, the cost is being paid against a mis-stated model.

## Acceptance

- **Both probe shapes are executed and banked as tests** — the int-argument
  shape (expected: both fire) and the nested-`Fact`-argument shape (expected:
  unknown, and that is the point). A non-reproduction that is not banked is
  not an answer.
- §3.2 is either **amended to the real trigger** with the reproducing program
  quoted, or **deleted** with the reason recorded, and `Q-M1a.8` is closed in
  the [M1a ledger](../../../docs/history/m1a_rust/open_questions.md) with a
  date and the probe's name.
- The README's Known gaps entry moves with it. Two pages state this claim and
  they do not get to disagree afterwards.
- If the nested-`Fact` shape *does* reproduce, the item stays a latent bug,
  §3.2 is rewritten to the shape that reproduces, and the fix — if any — is
  filed, not taken here. The page's job is to state behaviour correctly; it
  is not this stage's job to change the behaviour.

## Tasks

### Task T1e.1.4.1 — Read the key, then predict ✅

**Done 2026-08-29, before anything ran — and the reading was right about all
three shapes, including one the stage did not ask for.**

The key has **three** components and not two, which is the whole answer:

| component | what it keeps of the activator | built at |
|---|---|---|
| `PlanKey` — the *compile cache* key | **every** argument, stringified | `compile.rs` `plan_key` |
| `BindingKey.activator` — an interned `plan.activator_args` | the **symbol** arguments | `compile.rs` `bind_activator`'s return, `engine.rs` `intern_activator` |
| `BindingKey.values` — `regs[..plan.n_regs]` | every argument that **binds a parameter** | `compile.rs` `bind_activator`'s seed loop, `firing.rs` `BindingKey::new` |

The seed loop is the pivot, and it is one line —
`if a.as_fact().is_some() { continue; }`, ein.py's
`isinstance(a, (str, int))`:

| activator argument | in `activator`? | seeds a register → in `values`? |
|---|---|---|
| `Sym` | yes | yes |
| `Int` | **no** | **yes** |
| nested `Fact` | no | no |

So the set that reaches *neither* half of the binding key is exactly the nested
`Fact`s, and the predictions written down before the first run were:

1. **Int** — same `activator`, different `values`. Both fire; §3.2 is false.
2. **Nested `Fact`** — same `activator`, and the `Fact` seeds nothing, so both
   plans have the same `seed`, `reg_names` and `n_regs`. Equal keys, second
   application suppressed. **But nothing is lost**: `Compiler::run` hands the
   activator to `bind_activator` and to nowhere else, so the two plans are
   equal in every field of `Plan`.
3. **A third shape the stage did not name**, read off `Engine::check_layout`'s
   own doc comment: an `int` where another activator has a nested `Fact`, in
   the same position. Same `activator`, *different register layouts* — which
   is the invariant `BindingKey` documents. Predicted: a debug-build assertion,
   and in release a value-vector comparison between different variables.

All three reproduced. The third is the one that costs a derivation, and
predicting it before running is what turned it from a curiosity into the
answer: it is the shape §3.2 should have described all along.

The task as written:

Before running anything, write down what the two halves of the binding key
contain, from
[`firing.rs:219-224, 242-249`](../../../ein.rs/crates/ein-infer/src/firing.rs)
and [`compile.rs:94-101, 440-453`](../../../ein.rs/crates/ein-infer/src/compile.rs):
which activator argument kinds seed a register and therefore reach
`BindingKey.values`, and which bind nothing and therefore reach neither half.
The prediction is cheap and it is what makes the probes decisive rather than
exploratory — a probe designed after the answer is known proves less.

The expected reading, to be confirmed or corrected: a **symbol** or **int**
argument seeds a register and lands in `values`; a **nested `Fact`** pattern
in activator position binds nothing, so two activators differing only there
produce equal keys. If that is right, §3.2's mechanism is real and its
*trigger* is misnamed.

### Task T1e.1.4.2 — Two probes, both banked ✅ — three

**Done 2026-08-29.** All three are in
[`ein-infer/tests/rule_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/rule_semantics.rs),
under a new *the activator's identity — Q-M1a.8* section beside
`a_parametrised_rule_with_no_activator_is_dormant`, which is the file's other
claim about what an activator does:

| test | claim | result |
|---|---|---|
| `activators_differing_only_by_an_int_argument_both_fire` | `(tag edge 1)` and `(tag edge 2)` over one edge | **both fire** — `(tagged A 1)` *and* `(tagged A 2)`, and the two firings bind `?n` to 1 and to 2 |
| `activators_differing_only_by_a_nested_fact_argument_share_one_binding_key` | one program, four cells, only the *kind* of the second argument varying | one `Fact` **1** firing · two `Fact`s **1** · two symbols **2** · two ints **2** — and the same one derived fact in all four |
| `an_int_beside_a_nested_fact_in_one_position_loses_a_derivation` | an `int` and a nested `Fact` in one position | **a derivation is lost**, and a debug build asserts instead |

The middle row is the shape of the answer. It is one fixture run four ways, so
the firing count is attributable to the argument *kind* and to nothing else —
and it says in one table both that §3.2's trigger is not a trigger (the two-int
cell fires twice) and that the real collision is at the argument kind nobody
had named.

The assertions are on firings and on derived facts rather than on a rendered
table, per the task. `rule_semantics.rs`'s harness already collects every
`Firing` the saturator emits, redundant ones included, which is the same object
`--events` prints as `fire`; the stage used `--events` for the exploration and
banked the in-process form, because a test that shells out to a binary is a
test nobody runs.

**Why the third probe is not a corpus fixture.** `cargo test` builds with
`debug_assertions`, and under them the program trips
`Engine::check_layout` — *two plans share (rule, activator) and disagree on
their register layout* — before it can answer. A `.ein` file for it would need
a `corpus.toml` entry, and the entry would panic the gate. So the reproducer
lives inline and the test carries **both** profiles' behaviour:
`#[cfg_attr(debug_assertions, should_panic(…))]`, with the release assertions
below it. Verified in both — `cargo test -p ein-infer --test rule_semantics`
and the same with `--release`, 14 passed each.

The task as written:

1. **Int arguments** — the review's shape, re-run and turned into a test:
   two activators differing only in an int argument, both must fire, asserted
   on the `fire` event stream rather than on a rendered table (the stream is
   where the firing is, and `--events` is already the instrument
   `stdlib_census.py` uses).
2. **Nested-`Fact` arguments** — two activators differing only in a nested
   fact pattern. Assert what the reading predicts; if the prediction is
   wrong, the test records the real behaviour and the reading in T1e.1.4.1
   gets corrected, not the test.

Both go under `ein-infer/tests/` beside the firing tests, named for the
claim (`activators_differing_only_by_int_argument_both_fire`), not for the
question id — a test named `q_m1a8` is unreadable the day the question
closes.

### Task T1e.1.4.3 — Amend or delete §3.2, and close Q-M1a.8 ✅

**Done 2026-08-29, and the outcome table needed a fourth row.** The three it
has assume the int probe and the nested-`Fact` probe between them settle it.
They do not: the int shape refutes §3.2, the nested-`Fact` shape collides *and
loses nothing*, and the bug is in neither of them. What landed:

| probe | result | consequence |
|---|---|---|
| int / int | both fire | §3.2's stated trigger is **false** |
| `Fact` / `Fact` | one firing, one plan's worth of conclusions | a real collision, and **harmless** — the plans are identical |
| **int / `Fact`** | a derivation is lost, silently | §3.2 **rewritten to this**, and it is the latent bug the page claims to have |

So §3.2 is **amended, not deleted** — the page keeps its one latent bug and its
count of thirteen is unchanged, which is why `CLAUDE.md`'s sentence quoting it
did not have to move. `Q-M1a.8` is **closed as stated** in
[the M1a ledger](../../../docs/history/m1a_rust/open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)
— dated, naming both probes, and recording that the misreading is ein.py's own:
`_binding_key`'s third component is `frozenset(bindings.items())` and ein.py's
bind loop is `isinstance(a, (str, int))`, so the integer was in the key there
too. The live half is
[Q-M1e.16](../open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one),
with the fixture, the three candidate fixes and no owner.

**Ten places, where the acceptance names two.** It names the page and the
README; a grep for the trigger found the same sentence in five source comments
and two history pages, and one of the five was false about something else as
well:

| where | what moved |
|---|---|
| [`defined_behaviour.md` §3.2](../../../docs/kernel/defined_behaviour.md) | retitled to *nested-`Fact`*, the three-key table, the reproducer and its two transcripts, the corpus measurement, and why it is not a fixture |
| [`README.md`](../../../README.md) | Known gaps bullet **and** the capability table's cell — both now cite `Q-M1e.16` |
| [`firing.rs`](../../../ein.rs/crates/ein-infer/src/firing.rs) `BindingKey` | the false trigger, **and** *"the invariant this leans on"*, which said the layout invariant holds. It does not; the doc now says so and names what enforces it |
| [`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) `PlanKey` | the false trigger |
| [`plan.rs`](../../../ein.rs/crates/ein-infer/src/plan.rs) `Plan::activator_args` | the false trigger |
| [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) `refresh_collision_risk` | the false trigger, and its *"neither holds anywhere in the corpus"* given the stronger reason it actually has |
| [`engine.rs`](../../../ein.rs/crates/ein-infer/src/engine.rs) `check_layout` | *"which is a shape no rule application has"* — **false**, and it is the premise the whole invariant rested on |
| [`design/05`](../../../docs/history/m1a_rust/design/05_matcher.md) §6, [`m1a_rust/README.md`](../../../docs/history/m1a_rust/README.md) § What outlived | history: not rewritten, given a dated correction each |

`engine.rs`'s is the one worth naming twice. The invariant `BindingKey` leans
on was *argued* — a register layout is a function of the rule and of which
parameters the activator bound — and the argument was correct; what was wrong
was the sentence that dismissed its counterexample as a shape nothing has.
That is [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)
Rule 2 exactly: the premise was written down, was believed, was load-bearing,
and nothing but a `debug_assert` was enforcing it.

The task as written:

Then the paperwork, which is the point of the stage:

| probe result | §3.2 becomes | `Q-M1a.8` |
|---|---|---|
| int fires both, nested collides | rewritten: the trigger is a nested-`Fact` activator argument, with the program that shows it | stays open, correctly stated, with a fixture |
| int fires both, nested also fires both | **deleted** — the page has no latent bug; the two probes become the tests that say so | closed as **not a bug**, dated, naming both probes |
| int collides (the review's probe was wrong) | unchanged in substance; the probe that failed to reproduce is recorded with why | stays open, and the review's finding is `refuted` |

Whichever row lands, `defined_behaviour.md`'s own count of what it states
moves with it, and the README's Known gaps sentence is edited in the same
commit. The milestone has a whole finding about counts that drift because two
copies of one claim were not edited together
([DO-M1](../README.md#the-findings)); this is the first chance not to do it
again.

## Notes

The third row is the one worth keeping in mind. The review reproduced its
probe against the release binary and the reproduction is credible — but this
milestone's rule cuts both ways, and a finding that a page is wrong is itself
a claim until a banked probe holds it. Running the int shape again is fifteen
minutes and it is the difference between *the review said so* and *the tree
says so*.

The third row did not land: the review's probe reproduced exactly, and the
finding stands. What the Notes did not anticipate is that the *first* row is
only half true — the nested-`Fact` collision is real and is not the bug, and
the bug needed a shape neither the review nor this stage had named before
T1e.1.4.1 read `check_layout`'s doc comment.

## What landed

| | |
|---|---|
| the probes | three, in `ein-infer/tests/rule_semantics.rs` § *the activator's identity — Q-M1a.8* — int, nested `Fact`, and the mixed pair |
| the answer | §3.2's trigger is **not** the integer. An `int` binds its parameter and reaches the key's third component; a nested `Fact` reaches none of the three and collides **harmlessly**; an `int` **beside** a nested `Fact` in one position puts two register layouts in one identity and loses a derivation |
| the page | [`defined_behaviour.md` §3.2](../../../docs/kernel/defined_behaviour.md) retitled and rewritten, with the seventeen-line reproducer, both transcripts, and the corpus number |
| the ledger | [`Q-M1a.8`](../../../docs/history/m1a_rust/open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args) **closed 2026-08-29 as stated**, naming both probes — and recording that the misreading was ein.py's too |
| filed | [Q-M1e.16](../open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) — the live bug, three candidate fixes priced, **owner unassigned** |
| the wording | **ten places**: the page, the README twice, five source comments (`firing` · `compile` · `plan` · `saturator` · `engine`) and two history pages, the last two given a dated correction rather than a rewrite |
| the measurement | every plan compiled by `ein solve -m 2` over all **204** corpus `.ein` files binds against **153 143** activator arguments, and **every one is a symbol** — **0** ints, **0** nested `Fact`s, **0** `(rule, activator)` spaces holding more than one plan |
| not changed | the engine, `defined_behaviour.md`'s count of thirteen, `CLAUDE.md`, and no corpus answer |

**No golden moves**, and that is checkable rather than hoped for: the corpus
measurement is the reason. Every activator argument under `examples/`,
`stdlib/` and `tests/` is a symbol, so none of the three shapes occurs, and
the only files this stage adds are three `#[test]`s over inline source.

**What the corpus number is worth beyond this stage.** It was taken twice —
once over `ein saturate` (3 296 arguments, the 158 files that saturate) and
then again over `ein solve -m 2`, which is the one quoted, because a fork
compiles plans of its own against activators a rule *derived* and the first
sweep could not see them. Both answers are the same answer. It is the premise
`saturator.rs`'s `refresh_collision_risk` spends conservatism on — *neither
holds anywhere in the corpus* — and it now has a stronger reason than the one
written there: not "the asymmetry is never exercised" but "no program has a
non-symbol activator argument at all". It is also what makes
[Q-M1e.16](../open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one)'s
first candidate fix free: with every argument a symbol, the filtered and
unfiltered activator lists are the same list, so widening `ActivatorId` moves
no output. Nothing enforces the number — it is a measurement, not a check, and
a program that broke it would be caught by `check_layout` only in a debug
build.
