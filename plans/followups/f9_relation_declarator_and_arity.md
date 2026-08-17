# F9 — Relation declarator: reflection + arity

Four parked engine items about the `(relation …)` declarator and relation
**arity**. They share one root: Ein's kernel is arity-2-shaped in places
where its *graph model* is not, and its schema channel publishes less than
userspace can consume.

Parked by [S1.22.4](../m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/s1.22.4_relation_kernel_word.md)
(P1.22 forbids engine behaviour change; see
[C4 §6](../m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/reports/c4_relation_kernel_word.md)).
The kernel-word question itself is **decided and closed** — `relation`
stays kernel-side, C4 §1 — so nothing below is blocked on it. Each item is
independent; F9.1 is the one with a user asking for it.

## Trigger

Surfaces when any of:

- a puzzle declares a **non-binary** relation and expects `std.bijection` /
  `std.algebra` / `std.typing` to see it (they will not — F9.1);
- a rule needs "is `?R` a relation?" without knowing `?R`'s arity (F9.1);
- a puzzle wants the solver to *guess* a unary fact (F9.2);
- the DOT view of a property-tag-heavy KB needs to read well (F9.4).

## F9.1 — Generic relation reflection: the companion `(relation R)` fact

**The gap, measured** ([C4 §5](../m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/reports/c4_relation_kernel_word.md)):
reflection over declarations is **arity-coupled**. The loader mirrors
`(relation likes Person Drink)` as a 3-arg fact and
`(relation adult Person)` as a 2-arg fact, so:

| rule pattern | matches |
|---|---|
| `(relation ?R ?A ?B)` | binary declarations only |
| `(relation ?R ?A)` | unary declarations only |
| `(relation ?R)` | **nothing** — no arity-1 mirror is ever stored |

Consequences: the entire `std.*` typecheck / converse / hierarchy stack
spells `(relation ?R ?A ?B)` and is therefore silently **binary-only**; and
auto-vivified relations (the property tags — no declaration, so no mirror
fact at all) are invisible to every one of these patterns.

**Origin — user, 2026-08-17** (root-`TODO.md`, item 5; full block in
[S1.22.3 §Origin](../m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/s1.22.3_relation_signature_semantics.md)):

> Some other rules then could check if argument is name of relation by
> `:match (relation ?R)`

**Sketch.** Emit a companion arity-1 fact `(relation R)` for every relation
node, alongside the existing arity-N mirror. Open design points:

- **Declared vs auto-vivified.** Should a property tag's carrier relation
  get one? If yes, `(relation R)` means "R is a relation node"; if the two
  need distinguishing, a second head (`(declared-relation R)`) is the
  cheaper split than overloading arity.
- **`hypgen` interaction.** `_candidate_objects` subtracts signature atoms
  from the guessable pool; a new arity-1 `relation` fact adds no signature
  atoms, but it does add a fact whose head the kernel already special-cases
  (`KERNEL_META_RELATIONS`) — check the candidate set is unperturbed.
- **Does the stdlib migrate?** `std.bijection`'s `typecheck-setup` etc.
  could keep `(relation ?R ?A ?B)` (they genuinely need the two sorts); the
  new fact serves the *membership* question, not the *signature* question.
  Migrating them is a separate call.

Additive and small; it does not touch the declarator's validation.

## F9.2 — Unary hypothesis targets

`hypgen._fill_slot` returns immediately unless the signature has length 2
(`inference/hypgen.py:293`), so a declared unary relation loads, stores
facts and saturates normally but is **never a guess target**. This is an M1
cut for tractability, not a design position — recorded as such in
[`01_grammar.md` §relation declarator](../../docs/kernel/ir/03-ein-lang/01_grammar.md)
and [`03_ein_model.md` §5.1](../../docs/kernel/ir/01-ein-graph/03_ein_model.md).

Lifting it means deciding what a unary hypothesis *costs*: the candidate
count is |objects| per relation rather than |objects|², so it is cheap —
the question is whether any puzzle needs "guess that `adult(Jack)`" as a
branch point, and how it interacts with `emit_closed` (a unary relation no
rule produces is auto-closed today, which already suppresses the guess).

## F9.3 — Bare untyped declaration `(relation R)`

`(relation r)` **parses** — it falls through `relation_decl` (which
requires `SYMBOL SYMBOL+`) to `generic_fact` — and is then rejected by the
loader's relation branch (`from_ir.py:165`, *"(relation) needs name +
signature"*). So this is a **loader** decision, not a grammar change.

Legalising it collides with the kernel's signature-**presence** keying: an
empty signature currently means "not a declared domain relation" (skipped
by `hypgen._raw_candidates` and by `emit_closed`), which is exactly how
property tags are told apart from domain relations
([S1.22.3 census Q2](../m1_core_graph_reasoning/p1.22_obsolete_syntax_and_closeout/s1.22.3_relation_signature_semantics.md)).
A bare declaration would be a third state — *declared but not a hypothesis
target* — needing its own flag. Today's answer is to write the don't-care
atom: `(relation R T)`.

Note the overlap with F9.1: if `(relation R)` becomes the generic
membership fact, it cannot simultaneously be a bare declaration form.
**Decide F9.1 first.**

## F9.4 — Unary rendering in the compact DOT view

`ir/to_dot.py` collapses a fact into a labelled arrow only when
`len(positional) == 2`; everything else — including every unary property
tag — falls through to the Levi-bipartite path (an octagon node with one
incident slot-edge). That is *correct*, and the Levi view needs nothing.
The open question is only whether the **compact** view wants a dedicated
unary convention (a tag badge on the relation node, say) rather than
dropping to an octagon mid-diagram, since a property-tag-heavy KB renders
as a field of one-armed octagons.

The "arrow that starts at an object and ends nowhere" shape the user
sketched is the thing the Levi encoding makes unnecessary — see
[`03_ein_model.md` §5.1](../../docs/kernel/ir/01-ein-graph/03_ein_model.md).

## Connections

- [F5](f5_rules_as_data.md) — rules matching/generating rules; F9.1's
  companion fact is the same "publish the kernel's own structure as
  matchable data" move, one level down.
- [F7](f7_rule_induction.md) — its *relation-var arity* axis counts
  relation **variables** in a rule body; F9 is about the arity of the
  relations themselves. Orthogonal, but a rule library that induces
  activators will hit F9.1's binary-only ceiling.
- [F8](f8_FCA_RCA_odis_tptp/ideas.md) — already frames property tags as
  *unary relation properties* `P(R)` over relation-objects, the same
  predicate-as-subset reading as `03_ein_model.md` §5.1.
- [`docs/kernel/ir/03-ein-lang/08_self_describing.md`](../../docs/kernel/ir/03-ein-lang/08_self_describing.md) —
  what userspace already builds on the mirror fact.
