# P2.2g — Names across an import: what a module's facts are in the program that imports it

**Status:** **planned 2026-11-27** on the user's word, for
[Q-M2.43](open_questions/kernel/Q-M2.43-is-a-programs-declaration-of-a-modules-relation-a-second-declaration-of-it.md),
which moves here from [P2.2e](../p2.2e_kernel_fixes/README.md): *"in your examples 2 facts are different, imported is
(m.relation m.likes m.Person m.Drink) and program's is (relation m.likes Person Drink) - I don't yet wknow how to
import `m.relation` - create a new phase for this question"*. **Not started**: no stage runs, not even the survey,
until the user says so. **Widened the same day** by the user's generic questions, filed as
[Q-M2.45](open_questions/kernel/Q-M2.45-what-is-a-module-its-names-the-order-imports-make-and-what-is-public.md):
*"every module has it's atoms, facts and rules; modules form a lattice with import (are they?); what and where we
want to be feasible/exposed/public and what hidden/private"*.
**Depends on:** [S2.2e.1](../p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md), which closed E-104 and
whose probes, [`a-modules-relation/`](../p2.2e_kernel_fixes/probes/a-modules-relation/), are where this starts.
**Changes:** Ein (`ein.rs/`, `stdlib/`, `docs/ein/`, `docs/decisions/`, the corpus), **and possibly what an import
means**. [Q-M2.16](../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md) lets M2 change the
kernel only where Ein's syntax and semantics stay as they are; what a name a module writes means is the import's
semantics, so a change here is made only on the user's ruling of this phase's question, and that ruling is the word.
EinAf follows in the same batch where a ruling reaches it.
**Blocks:** nothing planned. Whether [P2.3](../p2.3_benchmark/README.md)'s programs, which import `std.slots`, would
read otherwise under a ruling is S2.2g.1's to find.

## The question

**The user's statement**, in four messages of 2026-11-27, is kept verbatim in Q-M2.43. Its last turn:

> "in your examples 2 facts are different, imported is (m.relation m.likes m.Person m.Drink) and program's is
> (relation m.likes Person Drink) - I don't yet wknow how to import `m.relation`"

**In the user's reading every name a module writes is the module's**: its declarator, its labels and its
relation, so the module `m` declares `(m.relation m.likes m.Person m.Drink)`. A program that declares the relation
writes `(relation m.likes Person Drink)`. These are two facts of two relations, and the user does not yet know how a
program would import `m.relation`.

**Ein today does a third thing.** The probe `aliased.ein` (`(import m :as a)`, its own `(relation a.likes …)`)
loads, and `ein saturate --dump` lists:

```ein
(relation m.likes Person Drink)
(relation a.likes Person Drink)
(a.likes Ann Tea)
(m.likes Cy Milk)
```

- **Only a declared name takes the module's full name**: a relation, a rule or a macro
  ([Q-M1f.231](../../../docs/decisions/imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)).
- **`relation` is the kernel's one atom**, read as a declaration wherever it is written.
- **Every constant a module writes is a name of the program**: `Person`, `Drink`, `Cy`, `Milk`
  ([Q-M1f.137](../../../docs/decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md)).

Three decisions hold to that, each pinned by a test:
- [Q-M1f.137](../../../docs/decisions/imports/Q-M1f.137-an-imported-facts-constants-are-names.md): a module written in place answers as its import.
- [Q-M1f.249](../../../docs/decisions/imports/Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md): the answer does not depend on the import form.
- [Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md): a declaration is an ordinary fact.

**What the user's reading opens**, each a question of its own:

- **The declarator.** If a module's `relation` is `m.relation`, what makes the module's declaration one the kernel
  reads, and one the program's rules see (the user's *"how to import `m.relation`"*)? The kernel reads a declaration
  through one word of its own (`ein_core::terms::Kernel`) and reads no theory's name
  ([Q-M1f.436](../../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)).
- **The constants.** If `Person` written in a module is `m.Person`, a program's `Person` is another atom. That
  changes what a module's rules match, what the blind enumerator guesses, and what a module written in place
  answers (Q-M1f.137's invariance).
- **The other kernel words and the library's names**, `open`, `absent`, `false`, `not`, and `std.core`'s rule,
  which every program gets ([Q-M1f.551](../../../docs/decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)):
  module-scoped too, or not.
- **Q-M2.43's cases**, a program's declaration of a relation it lists, aliases or names fully, follow from the
  three above.

**The user's generic questions**, filed as
[Q-M2.45](open_questions/kernel/Q-M2.45-what-is-a-module-its-names-the-order-imports-make-and-what-is-public.md),
frame the phase:

> "every module has it's atoms, facts and rules; modules form a lattice with import (are they?); what and where we
> want to be feasible/exposed/public and what hidden/private"

The starting point, in Q-M2.45's file, is what the decisions in force say:
- **What a module owns**: only its declared names. Its constants are the program's, its facts and rules come into
  the program whole.
- **The order**: an import cycle is refused, so modules form a partial order. They do not form a lattice. What a
  program loads, a set of modules closed downward, does form one.
- **Public and private**: everything is public. There is no export list, no module reserves its relations, a short
  name is local to its file, and nothing is re-exported.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2g.1](s2.2g.1_every_name_a_module_writes.md) | Every name a module writes, today | a table: each kind of name a module writes against each import shape, what the loaded program holds, which decision says so, which test holds it; the order imports make, as the loader builds it; what of a module a program may write, state and see, today; and, counted by a script, the corpus programs and library modules a module-scoped constant or declarator, or a hidden name, would read otherwise. No change to Ein |
| [S2.2g.2](s2.2g.2_the_users_reading_worked_out.md) | The user's reading, worked out | the user's reading written as a page, for Q-M2.45's three questions (what a module owns, the order, public and private) and Q-M2.43's cases; for each, the options, today's among them, each worked on Q-M2.43's probes and on a library program, with what loads, what each query answers, what the kernel would read, and which decisions it supersedes; the coordinator's recommendation; **stops for the user's ruling** |
| [S2.2g.3](s2.2g.3_the_ruling_made_true.md) | The ruling made true | the decisions under `docs/decisions/`, the loader, the tests and the pages; every corpus answer that moves, with the reason; EinAf following; the gate (and the nightly). Runs only after the ruling, and only if it changes something |

### Order

S2.2g.1 needs no ruling and no model. S2.2g.2 ends at the user's ruling of Q-M2.43 and of what S2.2g.2 adds to it.
S2.2g.3 runs on that ruling alone. The phase is not in [the order before P2.3](../README.md#the-order-before-p23)
unless S2.2g.1 finds that P2.3's programs would read otherwise.

## Acceptance

- S2.2g.1's table, every cell probed, with its command and its output.
- S2.2g.2's page, its options worked on examples that run, and the user's ruling in the user's words.
- Under the ruling, if it changes Ein:
  - the decisions that say so, superseding by id what they supersede;
  - the code and the tests;
  - every corpus program answering as before, or listed with the reason it moved;
  - Q-M1f.137's and Q-M1f.249's invariances held, or rewritten by the ruling and held as rewritten.
- The gate green on the phase's last commit.

## Risks

- **Every program that imports the library reads the import's semantics.** A ruling that scopes a module's
  constants or its declarator reaches every module of `stdlib/` and every program that imports one.
- **Two invariances the corpus is held to rest on today's reading**: a module written in place answers as its
  import (Q-M1f.137, `import_facts.rs`), and the answer does not depend on the import form (Q-M1f.249,
  `import_invariance.rs`).
- **The kernel's word.** A declarator per module touches what the kernel reads as a declaration, which Ein keeps
  to the kernel's own words (Q-M1f.436).
- **The ruling may change nothing.** Today's reading is one of S2.2g.2's options, and then S2.2g.3 does not run.

## Defaults taken at the plan

Each is the plan's, recorded so the user can correct it; none is a ruling.

1. **Q-M2.43 moves here whole**, and Q-M2.45 joins it, each with its default standing: nothing changes until the
   user rules.
2. **A phase of its own, outside the order before P2.3**, unless S2.2g.1 finds otherwise.
3. **Today's reading is one of S2.2g.2's options**, so that the ruling can keep it.
4. **Nothing of Ein changes before S2.2g.3.**

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic. Ids come from the milestone's one sequence,
the next free on `master`.
