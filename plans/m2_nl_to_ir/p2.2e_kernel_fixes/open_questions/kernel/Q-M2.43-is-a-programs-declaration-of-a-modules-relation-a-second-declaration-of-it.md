# Q-M2.43 — Is a program's declaration of a relation it takes from a module a second declaration of the module's relation?

**Status:** open

**Raised 2026-11-27 by [S2.2e.1](../../s2.2e.1_declared_again_under_an_import.md)**, on the user's statement of how a
declaration meets an import, passed to the stage by the coordinator while it ran. For two of the statement's cases
the code and a decision in force say otherwise; the stage stopped at the evidence and changed neither, as the
coordinator asked.

**The user's words**, verbatim, the first message and its correction:

> "A relation declared again, under an import - at first it has different global name - so it is different
> relation, but if it is imported with  :symbols then it is the same relation and in this case it is just 2
> different relation declarations - different ordinary facts of relation `relation`. If imported without :symbols,
> without alias, rules from program code does not see relation fact from module, because it has different name and
> thus it is different atom. If imported with local alias - then both program rules and module rules see the same
> objects by the same atom."

> "in the third case a.likes is M.likes, but if program declares it's own (relation likes ...) it is different,
> obviously because 'likes' != 'a.likes'"

> "2 relation declarations is also possible because they are different facts, that may be used by different rules -
> this is Ein's semantic. We would not change Ein sematic to help dumb 4B LLM write programs, we will do it another
> way."

The coordinator reads [Q-M1f.447](../../../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)
as covering every case, a program's declaration under a module's atom among them. Q-M1f.234 and the code's reading
of an alias say otherwise for two of them; the question is which holds.

## What Ein does today

The probes are [`../../probes/a-modules-relation/`](../../probes/a-modules-relation/): a module `m` declares
`(relation likes Person Drink …)`, a rule `mfan` over it and the fact `(likes Cy Milk)`; each program writes a fact
of its own, `(likes Ann Tea)` in the spelling it uses. Run with `ein solve` on the stage's commit.

| the program | the user's statement | what Ein does | what says so |
|---|---|---|---|
| `(import m)`, and its own `(relation likes …)` | two relations, `likes` and `m.likes` | **the same**: it loads, and neither file's rule reads the other's facts, at the module's count or another | [Q-M1f.231](../../../../../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md), [Q-M1f.236](../../../../../docs/decisions/imports/Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md); held by `declared_counts.rs`, `under_a_bare_import_or_an_alias_a_program_and_a_module_declare_two_relations` |
| `(import m :as a)`, and its own `(relation likes …)` | two relations, `likes` and `a.likes` | **the same** | the same test |
| `(import m :symbols (likes mfan))`, and `(relation likes …)` ([`listed.ein`](../../probes/a-modules-relation/listed.ein)) | one relation, two declarations, which load | **refused**, exit 1: `listed.ein:3:9: 'likes' is bound to two full names in the program, likes (its own declaration) and m.likes — one short name names one declaration in a file: list it from one module, or write the other by its full name` | [Q-M1f.234](../../../../../docs/decisions/imports/Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md): *"A short name bound to two full names, by two imports that list it or by an import that lists it and the file's own declaration of it, is refused at load"* |
| `(import m :as a)`, and `(relation a.likes …)` ([`aliased.ein`](../../probes/a-modules-relation/aliased.ein)) | `a.likes` is `m.likes`: one relation, two declarations | **loads, and the declaration makes `a.likes` a relation of the program's own**, in silence: `(a.mfan ?a)` answers `Cy` alone, the module's rule not reading the program's `(a.likes Ann Tea)`, and `(a.likes ?a ?b)` answers the program's fact alone; the module's relation prints as `m.likes`. With no declaration ([`aliased-undeclared.ein`](../../probes/a-modules-relation/aliased-undeclared.ein)) the program's fact is the module's relation: `(a.mfan ?a)` answers `Ann` and `Cy` | no decision says it: [Q-M1f.239](../../../../../docs/decisions/imports/Q-M1f.239-a-program-declares-no-name-under-the-full-name.md) refuses a declaration under a module's full name and calls a dotted name whose prefix is no module's a name like another; an alias is no module's name |
| `(import m)`, and `(relation m.likes …)` ([`full-name.ein`](../../probes/a-modules-relation/full-name.ein)) | not in the statement; the coordinator: record it and change nothing | **refused**, exit 1: `full-name.ein:4:11: the program declares 'm.likes', a full name of m, which the program imports — a declaration's full name is its own file's, and a full name of m is one m declares (Q-M1f.239)` | Q-M1f.239 |

A program's two declarations of its own relation, beside an import of anything, are two declarations of one
relation and load: that was E-104, which the stage fixed, and it is the statement's simplest instance.

## The question

Does the user's statement supersede Q-M1f.234 for a relation's declaration, so that a file's declaration of a short
name it lists from a module is a second declaration of the module's relation, one more fact of `relation` over the
module's atom; and does it settle the alias, so that a file's declaration of `a.likes` under `(import m :as a)` is a
second declaration of `m.likes` and no relation of the file's own? And the full name: is `(relation m.likes …)`
under a bare import a second declaration too, which would supersede Q-M1f.239, or does it stay refused?

**What a yes would take**, not done: the name binding of `ein-ir/src/imports.rs` (`tables` and `name_map`), where a
file's own declaration of a name binds it to the file's full name before a listed name or an alias is read; a
rewritten Q-M1f.234, a relation's declaration excepted (and Q-M1f.239, if the full name goes with it), Q-M1f.447
being the semantics they would then agree with;
`07-diagnostics.md` § 2.27; a test of the three forms, the selective and the aliased answering alike, every reader
of a declaration reading each. A rule or a macro written under a module's name stays refused, the user's *"rules
with the same name are load errors"*, by another line than today's.

**The default**: nothing changes until the user rules. Q-M1f.234 and Q-M1f.239 stand, a declaration under an alias
stays the file's own relation, and the stage holds the two cases that agree with the statement.
