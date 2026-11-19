# S1f.21.16 — the `:hrules` clause lists activators as a rule's are written

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's ruling; **not started**: it starts on the user's word.
**Waits on:** no decision: [Q-M1f.565](../open_questions.md#q-m1f565--the-hrules-clause-lists-activators-as-a-rules-are-written).
In order it runs after [S1f.21.15](s1f.21.15_forbid_becomes_a_warning.md), which moves the same goldens, and before
[S1f.21.11](s1f.21.11_kill_and_three_conventions.md).
**Writes:** the grammar (`ein-ir/src/ein.y`, the grammar page's generated regions and diagrams) and the loader's
reading of the clause; every `.ein` that writes it; the pages and the tests that quote it; the generator of the
zebra variants; the editor's grammar.

## The ruling

The user, 2026-11-19, verbatim: *":hrules argument looks bad. How do we activate rules normally? (rule R (?args) ....)  then somewhere (R args) right? So why :hrules look different? it has to be (solve :hrules ((guess chosen Drink) (guess served Drink)))"*

**The clause is a list of activators, each a form headed by its hrule**, as a rule's activator is a fact headed by
its rule. What the clause does, and that it is `solve`'s, is not changed.

```lisp
(solve
  :hrules ((guess color-loc  Color       House)
           (guess nation-loc Nationality House)
           (guess drink-loc  Drink       House)
           (guess smoke-loc  Cigarette   House)
           (guess pet-loc    Pet         House)))
```

is `examples/zebra2.ein`'s clause, which today reads `:hrules (guess (color-loc Color House) (nation-loc Nationality
House) …)`. `examples/lattice/13_witness_of_two.ein`'s, three hrules with no parameter, `:hrules (order cook shop)`,
becomes `:hrules ((order) (cook) (shop))`.

## What it does

1. **The grammar**: `:hrules (` *activator* … `)`, an activator `(` *name* *argument* … `)`; `()` lists none.
   **Reading 1**, default: the grouped form is refused at the parse, with a line that shows the same clause in the
   new spelling, as a retired spelling is; it is not read as a second way to write the clause.
2. **The loader**: each activator is held as today's are, its hrule one the program declares, its count the
   hrule's count of parameters, an argument at a parameter that names a relation a relation of the program. **A
   rule named in the clause heads its form and is no argument**, so the reading that counted an hrule from its
   second naming is gone ([V-13](findings.md#v-13--the-check-on-a-parameters-conclusion-counts-an-hrule-from-its-second-naming)).
3. **Every program that writes the clause**, re-spelled by a script and read back: 52 files under `examples/`, 4
   under `tests/`, 1 in `stdlib/` (`grep -rl -- ':hrules'`, on `946ee788`). The four zebra variants are written by
   their generator, which is changed with them.
4. **The pages and the tests**: `02-syntax.md`, the grammar page by its generator, the guide's chapters that show
   the Zebra program's `solve`, every page that quotes a clause (32 files under `docs/`), the decisions
   Q-M1f.115 and Q-M1f.116 reworded to the form, `AGENTS.md` where it writes the clause; the inline programs of the
   crates' tests (39 files); the scripts that write or read a clause.
5. **What prints a clause** prints the new form: a saved file's `(solve …)`, `ein render program`'s drawing of it.

## What must hold

- **Every answer**: for every program re-spelled, the summary's `region`, `queries` and root facts are what they
  were, and `ein show hyps` lists the same candidates; taken old binary on the old file against new on the new.
- **The four programs that were refused beside `std.negation`** (`examples/lattice/02`, `03`, `15`, `16`, each with
  the module imported in a scratch copy) load, and answer as they do without the import.
- The goldens move where a clause is echoed, the `load` and `ir` lines of the re-spelled programs and the
  drawings of their `solve`; no read-out, core or state line moves.
- A shape the grammar newly takes or refuses has its row in the grammar page's table and its line in the
  diagnostics.

## Acceptance

No file of the project writes the grouped form but the one that shows its refusal; `c-neg-twice.ein` of
[review item 38](review/part-1/38-Q-M1f.559/README.md), re-spelled, loads; the gate on the stage's last commit.

## Record

Nothing yet.
