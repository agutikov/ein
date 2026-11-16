# The matcher

How `ein-infer` finds every way a rule's `:match` holds in a KB: a rule is compiled, per activator, to a plan of
steps over registers ([`plan.rs`](../../../../ein.rs/crates/ein-infer/src/plan.rs),
[`compile.rs`](../../../../ein.rs/crates/ein-infer/src/compile.rs)), and the register matcher
([`match_.rs`](../../../../ein.rs/crates/ein-infer/src/match_.rs)) runs the plan against the store
([`data-model.md`](data-model.md)), handing each match to a callback. The saturator is its caller
([`saturation.md`](saturation.md)).

## 1. The shape it is built against

A matcher over a dynamically typed binding set pays three structural costs: a copy of the binding set per bound
variable, per candidate fact and per level of the join; type dispatch at every unification; and a fresh frame and
step list per step entered. The store makes each avoidable: a value is a `u32` and a fact an interned row of them
([`data-model.md`](data-model.md)). So variables live in a **register file**, binding is a register write and a push
onto a **trail**, unification is a `u32` comparison, and a match is handed to a callback instead of being
materialised: the matcher allocates nothing per bound variable, per candidate fact or per level of the join, which
[`ein-infer/tests/match_alloc.rs`](../../../../ein.rs/crates/ein-infer/tests/match_alloc.rs) counts.

## 2. Compile: from a rule to a plan

A rule compiles **once per activator** ([Q29](../../../decisions/engine/Q29-a-rule-compiles-once-per-activator.md)):
the activator is the fact that activated the rule (a property fact such as `(symmetric co-located)` for a rule
parameterised by a relation), and its arguments bind the rule's parameters, so the plan is specialised to them. The
plan's key is the rule and **every argument of its activator fact**, as the KB holds them
(`compile::plan_key`, `PlanKey`; [Q-M1f.43](../../../decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md)).
`compile::activators_for` lists a rule's activators in the KB.

A `Plan` holds:

- **disjuncts**: the first is the rule's `:match`; a top-level `(or …)` adds one per extra alternative, each run from a
  fresh seed, so a caller sees every disjunct's matches with no rule split;
- per disjunct, **steps** (`plan::Step`): `Rel`, a relation lookup (`Scan` when it shares no variable with an
  earlier premise, `Join` when it does; the unifier treats both alike), `Guard`, a built-in predicate (`eq`, `neq`)
  over the current bindings, and `Absent`, a **nested** `(absent …)` such as a `forall` desugars to;
- per disjunct, its **NAF guards** (`NafGuard`): every top-level `(absent …)`, lifted out of the steps by
  `split_naf`, which the saturator checks at the boundary rather than during the match;
- the **assert templates**, the conclusion to instantiate per match.

A step's argument positions are **slots** (`plan::Slot`): `Reg`, a variable, bound on first sight and compared after;
`Const`, a name, or a parameter the activator bound; `Nested`, a sub-pattern matched against a fact-valued argument.
Registers are numbered at compile time, one per distinct variable of a disjunct, the names kept beside them for
rendering. A rule binds **at most 256 variables** (`plan::MAX_REGS`, `ein_ir::MAX_MATCH_VARIABLES`), counted and
refused by the loader, so the register file is a fixed array
([Q-M1f.73](../../../decisions/engine/Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md),
[Q-M1f.173](../../../decisions/engine/Q-M1f.173-the-register-file-stays-256-wide.md);
[`compile_limits.rs`](../../../../ein.rs/crates/ein-infer/tests/compile_limits.rs)).

**The probe** (`plan::Probe`) is computed at compile time: for each relation step, the slots that may narrow its
candidates, a constant or a register bound by an earlier step, and, inside a nested slot, the position within the
nested fact (`(not (?R ?b ?i))` with `?b` bound probes position 0 of the nested fact). Which registers are bound at a
step is static, because the bind order is; whether a bound register holds a nested fact is not, so the runtime walks
the pre-filtered list and takes the first slot that keys.

**Compile errors** are a library caller's, worded in Ein's terms (`compile::CompileError`,
[Q-M1f.75](../../../decisions/engine/Q-M1f.75-the-compilers-errors-are-a-library-callers.md)); an error the loader
already refuses is an invariant of the compiler, not a message a program can reach
([Q-M1f.84](../../../decisions/engine/Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md)).

## 3. Compile once per run

A plan is a pure function of its key, so compiling it twice is waste. `compile::PlanMemo` is the run-wide
`(rule, activator) → PlanId` memo: every node of a search reads the plans root compiled, and a node compiles only a
pair it has not seen. What a node does **not** share is the order its engine meets the plans in, which the
saturator's enqueue pass walks: each `Engine` keeps its own ordered list of `PlanId`s, and the memo holds only the
plans. The memo is append-only. A compile is narrated as a `compile` event on a miss alone ([`../events.md`](../events.md)).
`Engine::compile_for` is the entry point ([`engine.rs`](../../../../ein.rs/crates/ein-infer/src/engine.rs)), and the
KB's count of rule applications, a version the compiler skips its walk on when nothing changed, is the store's
([`data-model.md`](data-model.md)).

## 4. Execute: registers and a trail

The matcher walks a disjunct's steps depth first. At a relation step it draws candidates (§ 6), unifies each against
the step's slots, and on success descends to the next step; on exhaustion it unwinds the trail to the step's mark,
writing the unbound marker back into each register, and returns to the step before. Unification of a slot with a
value is: `Const`, `u32` equality; `Reg`, bind if unbound else `u32` equality; `Nested`, the value must be a fact
and the sub-pattern must unify with that fact's row, one level of descent per level of nesting. A walk that reaches
the end of the steps emits a `Match` to the callback, which reads its bindings, premises and guards in place; the
saturator's enqueue runs inside the callback, so a match that is a duplicate costs only its key.

The walk is recursive over the borrowed step slice, with the candidate iterator on the stack: no frame allocates.

### 4.1 Three orders the matcher owes

The matcher's result sequence is part of the engine's contract, because the firing order is the trace
([`determinism-and-order.md`](determinism-and-order.md)). It owes three orders exactly:

- **matches**, in the order the nested steps produce them;
- **bindings**, in the order the matcher first bound each variable, because a derivation's bindings are printed in
  that order: the trail is that order by construction, seed bindings first;
- **premises**, in step order, including through a seeded run (§ 5), whose contract is that a derivation found from a
  seed is identical to one found by a full run: the seeded fact is written at its step's position.

## 5. Entry points

| entry | what it runs | used by |
|---|---|---|
| `Matcher::run` | every match of every disjunct | the saturator's enqueue pass, a query's goal |
| `run_guarded` | the same, with each match's disjunct's NAF guards | the saturator's full match |
| `run_seeded`, `run_seeded_guarded` | every match in which one new fact plays a step whose relation it has, seeding **each** such step in turn, since the fact may play either role in `(R ?a ?b) ∧ (R ?b ?c)` | the saturator's delta |
| `run_one` | the first match | a probe that needs one |
| `holds` | whether a guard's sub-plan has a match | the boundary's guard checks |
| `universal_support` | the facts each nested `(absent …)` of a guard matched | a derivation through a `forall` ([Q-M1e2.15](../../../decisions/answer/Q-M1e2.15-a-derivation-through-a-forall-rests-on-each-fact.md)) |

A query's `:goal` compiles to a plan under the made-up rule name `#query`
([`../reserved-engine-strings.md`](../reserved-engine-strings.md)) and runs through `run`.

**A guard's arguments resolve at run time**: a `Guard` step holds its arguments as written, resolved against the
binding environment, seeds included, when it is evaluated. That is why `split_naf` starts every guard's scope from
the rule's parameters, so a guard inside an `(absent …)` sees the parameters the activator bound.

## 6. Candidates

A relation step's candidates come from the KB's indexes ([`data-model.md`](data-model.md)):

- **a fully bound premise** (every slot a constant or a bound register, at any depth) denotes exactly one argument
  tuple, and the store interns propositions, so one interned lookup answers it: the one fact or none
  (`Matcher::ground_args`). A nested pattern naming a proposition that was never interned emits nothing without a
  lookup at all;
- **a probed premise** draws from the index bucket of its first keyed slot, a direct argument or a position inside a
  nested argument;
- **otherwise**, the relation's whole extent.

Each is a narrowing of the extent scan, never a different answer: unification re-checks **every** slot of every
candidate, whatever selected it, and the fully bound case is that argument taken to its limit, where the re-check is
the whole test. `match_.rs`'s `narrowing_never_changes_the_match_sequence` runs the matcher with and without its
probes over randomised insertion orders and requires the same sequence.

## 7. What is legal, and what is not

An optimisation is legal here only if it leaves the three orders of § 4.1 as they are:

- **legal**: registers and the trail, `u32` equality, the probe and the ground lookup (§ 6), a binding key of a fixed
  width hashed with FxHash, a per-step cached extent;
- **not legal**: **reordering the join**, cost-based or first-fail, which changes the order matches are produced and
  so the firing order and the trace; a worst-case-optimal join, for the same reason; **deduplicating matches inside
  the matcher**, which the saturator's own sets do, observably; and **skipping the re-check** of a slot a probe
  narrowed on, which is what makes narrowing behaviour-preserving.

The matcher keeps no intermediate join result between runs: each run starts from the KB, and a fork shares the KB's
indexes rather than a materialised join.

The work counters (`ein_core::Counters`, behind the `counters` feature) name the matcher's work: `walk`, `plan_run`,
`candidates`, `unify_slot`, `scan_bucket`, `scan_extent`, `scan_ground`, and their `_guard` twins, which split off
what the boundary's guard checks cost.
