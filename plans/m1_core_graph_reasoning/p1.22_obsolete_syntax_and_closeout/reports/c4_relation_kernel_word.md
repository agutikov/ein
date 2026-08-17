# C4 — `relation` as a kernel word: decision memo

**Stage:** [S1.22.4](../s1.22.4_relation_kernel_word.md), task T1.22.4.1
(the decision; T1.22.4.2–.5 then implement what §5 found). **Date:**
2026-08-17.
**Tree:** post-S1.22.3 (`7a6b5f8`). **Decision input:**
[S1.22.3's census](../s1.22.3_relation_signature_semantics.md), now placed
prose in [`01_grammar.md` §what the signature means](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md).

---

## 0. Decision

> **(i) — `relation` stays kernel-side.** Demotion does not remove any
> kernel *interpretation*; it only swaps the syntactic channel through
> which the kernel receives the same three structural signals, and pays
> for the swap with three load-time errors and one silently-dropped
> metadata field. The purity gain is nominal: under (ii) the kernel still
> hardcodes the string `"relation"`, merely as a reserved **fact head**
> instead of a reserved **declarator** — the same category as
> `__closed__` / `__symmetric__` / `not` / `false`. The reserved set does
> not shrink; it is renamed.

| question | answer |
|---|---|
| Which option? | **(i) keep kernel-side.** §1 |
| Does any consumer need entity-side `signature` rather than the fact? | **No** — 6 production sites, all mechanically re-keyable onto the mirror fact (§4). Feasibility was never the blocker. |
| What does (ii) cost? | 3 load-time errors → silence or saturation-time, + `:why` silently dropped. All four measured (§2). |
| Does (ii) shrink the kernel? | **No.** The three structural signals (S1.22.3 census) survive verbatim; only the channel changes (§1). |
| Is "`relation` as a **stdlib** word" available at all? | **No.** No rule in a `.ein` file can create an entity, assign `NameRef.category`, capture `:why`, or reject a duplicate at load (§1.2). |
| Is the user's item-5 ask satisfied by (i)? | **Not yet** — but its blocker is *not* the kernel word's existence. It is an arity-coupling gap, fixable additively (§5). This is the memo's real finding. |
| Anything deferred? | **Nothing** — the 4 ride-along items were implemented in this stage (T1.22.4.2–.5) rather than parked (§6). |

---

## 1. Why demotion buys nothing

### 1.1 The purity line it would claim to extend does not reach here

S1.7.23 and S1.7.24 removed the kernel **deciding things about user
semantics**: type-compatibility filtering, `_ancestor_names`,
`INHERITANCE_RELATIONS`, the `"T"` universal-top short-circuit, the
`kb.types` / `kb.instances` entity-view, symmetric-closure in three search
paths. Each deletion made the kernel *stop interpreting* a user notion.

The `relation` declarator interprets nothing. Per S1.22.3's census the
kernel reads a declaration in exactly three ways, all shape-only:

| signal | what the kernel concludes |
|---|---|
| signature non-empty | "declared domain relation" — hypothesis-eligible, `__closed__`-eligible |
| signature length 2 | the enumerator may fill its slots |
| signature atoms | these names are type-roles, not guessable objects |

**Option (ii) preserves all three.** It has to — they are what makes
hypothesis generation terminate. It changes only *where the kernel looks
them up*: `kb.relations[R].signature` becomes a scan of
`kb._facts_by_relation["relation"]`. That is a refactor of the lookup, not
a removal of a kernel concern. The kernel would still be the thing that
knows what `(relation …)` means.

### 1.2 "Not a kernel word but a stdlib word" is not an available shape

The user's framing offers `relation` as a *stdlib* word. Stdlib words in
Ein are `(rule …)` / `(macro …)` declarations in
[`ein.py/src/ein/stdlib/*.ein`](../../../../ein.py/src/ein/stdlib/) —
things the **matcher** runs. No rule can:

- create a `Relation` entity or populate `kb.relations`;
- assign `NameRef.category = "relation"` (`store._categorise_name`);
- capture a `:why` template (facts drop unknown kw-pairs — §2);
- reject a duplicate or a shadowed name *at load*, with a `loc`.

So the honest (ii) is **"kernel-read reserved fact head"**, not "stdlib
word". Ein already has that category, documented as reserved kernel
vocabulary precisely *because* the kernel reads it:
[`06_reserved_names.md` §hypothesis control](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
(`__closed__`, `__symmetric__`) and §⊥ primitives (`not`, `false`). Moving
`relation` from the declarator table into that table is a lateral move.

### 1.3 What (i) already concedes

Everything reflective the user asked for is **already true**: declarations
*are* facts (the loader mirrors them), and `:match (relation ?R ?A ?B)` is
the working idiom in `std.bijection` / `std.algebra` / `std.typing` /
zebra2. (i) is not "the kernel keeps the schema to itself" — it is "the
kernel keeps a *validated* schema channel **and** publishes it as data".
Userspace loses nothing by the declarator's existence; it gains a
load-time contract.

---

## 2. What demotion costs — measured

Four probes on this tree (`load(parse(src))`, PyPy-independent):

| probe | today | under (ii) |
|---|---|---|
| `(relation eq Person Person)` | `LOAD ERROR: relation 'eq' shadows a reserved kernel name` | stored as a fact; `eq` enters `kb.relations` by fact-head vivification, colliding with the computed predicate |
| `(relation r A B)` + `(relation r C D)` | `LOAD ERROR: duplicate relation 'r'` | **two** mirror facts, both stored (`Fact` identity is `(rel, args)`, so they do not dedup) — the kernel's signature lookup becomes ambiguous in exactly the place the three structural signals are read |
| `(relation r)` | `LOAD ERROR: (relation) needs name + signature` | legal unary fact (see §5 — this is the one case where (ii) *adds* something) |
| `(relation likes P D :why "{?1} likes {?2}")` | template captured on the entity (`rel.why`) | **silently dropped** — `_fact_args` discards unrecognised kw-pairs; the raw IR keeps the pair but nothing reads it. `ein solve`'s rendered-answer column degrades to raw s-expressions with no error |

The duplicate case is the serious one. It is not merely an ergonomic
regression: today a conflicting re-declaration is a hard load error, and
under (ii) it becomes an ambiguity feeding `hypgen._raw_candidates` /
`_candidate_objects` — i.e. it perturbs the search's candidate set with no
diagnostic. Trading a `loc`-bearing load error for a silent search
perturbation is the wrong direction for a reasoner whose whole value
proposition is an auditable derivation.

The `:why` case is the cheapest to fix (a `(why R "…")` carrier fact) and
the clearest illustration of the shape of (ii): every dropped kernel
behaviour has to be re-added as a **new convention the kernel also reads**.

---

## 3. The residue, re-verified against HEAD

The stage brief's checklist, confirmed line-by-line. Retained as the entry
point if (ii) is ever revisited.

| residue | site (verified) | status |
|---|---|---|
| malformed-form check | `kb/from_ir.py:165` | no rule-space analogue |
| reserved-name shadow check | `kb/from_ir.py:172-174` | no rule-space analogue |
| duplicate-declaration check | `kb/from_ir.py:176-177` | partially expressible as a ⊥-rule for *conflicting* sigs; identical re-declaration would dedup, conflicting ones would not (§2) |
| `:why` capture | `kb/from_ir.py:182-183` | needs a new carrier |
| mirror fact emission | `kb/from_ir.py:188-194` | **already the fact channel** — unchanged under either option |
| `declared` + `signature` on the entity | `kb/entities.py:91-93` | re-keyable onto the fact (§4) |
| `NameRef.category` assignment | `kb/store.py:589` (`KERNEL_META_RELATIONS`, `entities.py:51`) | derivable from fact-head occurrence, as auto-vivified heads already are (`entities.py:80-89`) |
| grammar production | `ir/grammar.lark:97` | folds into `generic_fact` |

---

## 4. Consumers of entity-side `signature` (the grep the task asked for)

`grep -rn '\.signature' ein.py/src ein.py/tests utils` — **7 hits, 6
production**:

| site | read | fact-keyable? |
|---|---|---|
| `inference/hypgen.py:251` | non-empty | yes |
| `inference/hypgen.py:267` | slot count | yes |
| `inference/hypgen.py:293` | length == 2 | yes |
| `inference/hypgen.py:539` | atom set | yes |
| `inference/closed.py:84` | non-empty | yes |
| `cli/saturate.py:388` | display | yes |
| `tests/kb/test_store.py:112` | assertion | n/a |

No consumer needs the *entity*. One mechanical caveat: hypgen iterates
`kb.relations.values()`, so a fact-keyed rewrite needs a name→signature
index built per call or maintained incrementally — cheap, but it is added
machinery, not removed machinery. **Feasibility was never the argument
against (ii); value was.**

---

## 5. The real finding — reflection over declarations is arity-coupled

The user's item 5 asks for a specific capability:

> Some other rules then could check if argument is name of relation by
> `:match (relation ?R)`

**That does not work today, and the memo's §1 reasoning does not fix it.**
Measured — three rules against two declarations, one unary, one binary:

```lisp
(relation adult Person)              ;; unary declaration — legal, loads
(relation likes Person Drink)        ;; binary
(rule sees-binary () :match (relation ?R ?A ?B) :assert (is-binary-decl ?R))
(rule sees-unary  () :match (relation ?R ?A)    :assert (is-unary-decl  ?R))
(rule sees-any    () :match (relation ?R)       :assert (is-rel         ?R))
```

| rule | fires on |
|---|---|
| `sees-binary` | `likes` **only** |
| `sees-unary` | `adult` **only** |
| `sees-any` | **nothing** — no arity-1 `(relation …)` fact is ever stored |

Three consequences, none of which the kernel-word decision touches:

1. **`std.bijection` / `std.algebra` / `std.typing` are binary-only.** All
   of them spell the idiom `(relation ?R ?A ?B)`, so a legal unary (or
   ternary) declaration is *silently invisible* to the whole typecheck /
   converse / hierarchy stack. S1.22.3 documented that unary declarations
   are legal; this is the other half of that fact.
2. **There is no generic "is `?R` a relation" predicate.** A rule must
   already know the arity it is looking for. Property-tag relations
   (auto-vivified, no declaration) are unreachable by any of these
   patterns.
3. **The user's spelling is blocked *by* the kernel word.** `(relation r)`
   *parses* — it falls through `relation_decl` to `generic_fact` — and is
   then rejected by the loader's relation branch. So the fact channel
   already accepts the shape; only the declarator's validation refuses it.

**The fix is additive, not a demotion.** Emit a companion arity-1 fact
`(relation R)` alongside the arity-N mirror, giving userspace exactly the
predicate the user asked for while the declarator keeps its validation.
Shipped as **T1.22.4.2**; the two design points this memo left open were
settled there — the fact is emitted for **declared** relations only (so
`(relation ?R)` means *declared relation*, and nothing must be emitted
mid-saturation when a rule vivifies a head), and `hypgen`'s candidate set
is unperturbed because the new fact's single arg is a relation-category
name, which `_candidate_objects` already excludes.

That the strongest result of a stage titled "decide the kernel word" is an
*additive* change on the other side of the question is itself the argument
for (i): the friction the user hit was never the declarator.

---

## 6. Implemented, not parked

> **Superseded 2026-08-17.** This section originally parked four items as
> a `plans/followups/` entry, per P1.22's out-of-scope rule. The user
> ruled otherwise the same day — decide **and** implement — so all four
> became tasks of this stage and shipped. The out-of-scope rule carries a
> scoped exception ([P1.22 README](../README.md)); the followup file was
> deleted rather than created. **The §0 decision is unaffected**: nothing
> below demotes `relation`, and T1.22.4.2 is the additive move §5 argues
> for.

| task | item | source |
|---|---|---|
| [T1.22.4.2](../s1.22.4_relation_kernel_word.md) | Generic relation reflection — the companion `(relation R)` fact | §5, the user's item 5 |
| T1.22.4.3 | Unary hypothesis targets — hypgen's arity-2 cut | stage ride-along |
| T1.22.4.4 | Bare untyped declaration `(relation R)` | stage ride-along |
| T1.22.4.5 | Unary rendering in the compact DOT view | stage ride-along |

T1.22.99.1 therefore inherits **no** new backlog from this stage.

---

## 7. Corrections to the stage brief

- **§Sub-items, "Bare untyped declaration `(relation R)` — currently a
  grammar error (`SYMBOL+` requires a type atom)".** It is a **load**
  error, not a parse error: `(relation r)` fails to match `relation_decl`,
  falls to `generic_fact`, and is rejected by `_ingest_relation`
  (`from_ir.py:165`). The distinction matters — it means the *fact* channel
  already accepts the shape, so T1.22.4.4 is a loader decision, not a
  grammar change alone (§5).
- Everything else in the brief's residue table and origin framing verified
  correct against HEAD; the line references had not drifted.
