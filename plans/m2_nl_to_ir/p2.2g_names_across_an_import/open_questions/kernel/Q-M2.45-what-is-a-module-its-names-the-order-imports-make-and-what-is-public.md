# Q-M2.45 — What is a module: its atoms, facts and rules; the order imports make; and what of it is public?

**Status:** open; P2.2g's, beside [Q-M2.43](Q-M2.43-is-a-programs-declaration-of-a-modules-relation-a-second-declaration-of-it.md),
which it frames

**Raised 2026-11-27 by the user**, of [P2.2g](../../README.md):

> "plans/m2_nl_to_ir/p2.2g_names_across_an_import - Generic questions - every module has it's atoms, facts and
> rules; modules form a lattice with import (are they?); what and where we want to be feasible/exposed/public and
> what hidden/private"

## What Ein says today, as the phase's starting point

The coordinator's reading of the decisions in force. [S2.2g.1](../../s2.2g.1_every_name_a_module_writes.md) probes
each point before anything is weighed.

**1. A module's atoms, facts and rules.**

| | whose today | the decision |
|---|---|---|
| a name the module declares (a relation, a rule, a macro) | **the module's**: one full name, `m.likes`, whoever imports it and by whatever path | [Q-M1f.231](../../../../../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md) |
| a constant it writes (`Cy`, `Person`), and a relation it states facts of and does not declare | **the program's**: a name like the ones the main file writes, which the blind enumerator guesses over | [Q-M1f.137](../../../../../docs/decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md), [Q-M1f.250](../../../../../docs/decisions/imports/Q-M1f.250-a-listed-name-a-module-states-facts-of-names.md) |
| the declarator `relation` and the kernel's other words | **the kernel's**, one atom in every file | `ein_core::terms::Kernel`; [Q-M1f.436](../../../../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md) |
| its facts | **the program's facts**, once however many imports bring them | [Q-M1f.143](../../../../../docs/decisions/imports/Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md) |
| its rules and declarations | **in the program, whole**: an import selects nothing | [Q-M1f.247](../../../../../docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md) |

So a module today is closer to a text spliced in with its declared names renamed than to a unit with atoms of its
own. A module of facts written in place answers as its import, and Ein holds that by a test (Q-M1f.137's,
`import_facts.rs`).

**2. The order imports make.** An import cycle is refused at load (`07-diagnostics.md`'s row, *import cycle: a -> b ->
a*), so the import relation is a directed acyclic graph, and *imports, directly or not* is a partial order on
modules. **It is not a lattice of modules**:
- two modules need have no least module above both, nor a greatest below both;
- `std.core` comes into every program, after its forms, and not as a module every other one imports
  ([Q-M1f.551](../../../../../docs/decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)).

**What a program loads is a lattice**: the modules it imports and everything below them. That is a set closed
downward in the order, and those sets form a lattice under union and intersection (a distributive one): the union
is the program that imports both, and the intersection is what two programs share. Whether Ein should say more than
that, for instance give the order a bottom every module sits on, is the question's.

**3. Public and private: today everything is public.**
- A module has no export list and hides no name: every declared name has a full name any importer may write
  ([Q-M1f.232](../../../../../docs/decisions/imports/Q-M1f.232-a-module-has-no-export-list.md)).
- A module cannot keep a program from stating facts of its relations, its circuit's included
  ([Q-M1f.319](../../../../../docs/decisions/imports/Q-M1f.319-no-module-reserves-its-names.md)).
- A short name is local to its file, and nothing is re-exported
  ([Q-M1f.231](../../../../../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)).
- A full name may be written only of a module an import brings
  ([Q-M1f.238](../../../../../docs/decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md)).

Q-M1f.232 says why: *what a theory promises to a program, and what is its own circuit, is its documentation's to
say*.

## The question

- What should a module own: its atoms, its facts and its rules, and its declarator (the user's `m.relation`)?
- Is the order imports make one Ein states, and with what structure? A partial order, a lattice, one bottom?
- What of a module should be exposed to the program that imports it, and what hidden? Which names a program may
  write, which facts it may state, and which rules see the program's facts?

Q-M2.43's cases, a program's declaration of a relation it lists, aliases or names fully, are instances of all three.

## What it bears on

- **Every module of `stdlib/`.** The calculator's circuit relations are the case Q-M1f.319 names: a program may
  state an answer nobody computed.
- **The invariances the corpus is held to**: a module written in place answers as its import (Q-M1f.137), and the
  answer does not depend on the import form
  ([Q-M1f.249](../../../../../docs/decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).
- **The model.** A theory whose circuit is hidden is a smaller vocabulary for a model to write in, and the catalogue
  EinAf shows the model is a module's public face.

## The default

Nothing changes until the user rules. [S2.2g.2](../../s2.2g.2_the_users_reading_worked_out.md) works this question
with Q-M2.43, today's answers among its options.
