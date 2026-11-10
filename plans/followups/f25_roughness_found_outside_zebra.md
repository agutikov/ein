# F25 — Roughness found outside the zebra programs

**Filed 2026-11-11**, on the user's instruction of that day, at the start of M1f
[P1f.7](../m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md):

> I don't want to review any new findings that does not affect existing zebra problems - so if find anything -
> either fix in place if it is quick or record into followups; I accept that language and engine MAY still contain
> some impurities, roughness, it's OK, we can return to them later when have real applications and corpus of real
> Ein programs (that starts from plans/m1f_hypothesis_and_documentation/p1f.6_calculator)

**What it owns.** Every finding a stage makes outside its own scope that moves no answer of an existing zebra
program (`examples/zebra*.ein` and the files generated from them) and is not quick to fix where it was found. A
finding is a row below: what, the smallest witness, where it was found, and the answer today. Nothing here is a
question for the user, and nothing here is a stage. A finding that is a wrong answer also gets a row in
`corpus/wrong_answers.toml`, `open` and owned by this followup, when its stage can file one at no cost; otherwise
the row here is the record.

**Trigger.** A corpus of real Ein programs exists (from [P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)'s
calculator onward) and one of them meets a row; or the user asks for the list.

## The items

| id | what | witness | found by | today |
|---|---|---|---|---|
| R-1 | **`(not ?x)` concluded where the loader cannot foresee that `?x` binds a name.** S1f.7.4 refuses the shape where a name is provable at load; a binding that arrives under a commitment or through an undeclared relation is not. What the firing does with it is not decided | S1f.7.4's records | the analysis of 2026-11-11 | to be taken by S1f.7.4's T1 |
| R-2 | **An `:hrules` entry that proposes nothing at root says nothing.** An activator's argument at a parameter that names no relation is a value, which no declaration lists, so S1f.7.8's refusal cannot check it: a misspelt type gives the hrule no match, H is empty, and the only sign is the read-out's `universe` row, *no candidate*. The same for a parameterised hrule that no activator names. A read-out row per `:hrules` entry that proposes nothing at root would cover both; it is a `Row` of `ein_render::read_out` and a line of `read_out.md` | `(solve :hrules (guess (at Colour House)))` beside `(is-a Red Color)`, `ein-cli/tests/solve_lists.rs`'s `a-value-that-is-no-relation` | S1f.7.8's T1 | loads, H empty, R {∅}, no word about the entry |
| R-3 | **A bare name in `:hrules` selects nothing.** `ein.y` reads `(h1 h2)` as a list of hrules, and the engine reads only activators from it: every hrule without parameters proposes whether the list names it or not, so `:hrules (h1)` beside a declared `h2` runs both. Since S1f.7.8 a bare name must be a declared hrule, and that is all it does | two hrules `h1`, `h2` with no parameter and `(solve :hrules (h1))` | S1f.7.8's T1 | both hrules propose |
| R-4 | **`ein render rules` on a program whose every rule is imported** prints *no rule forms in FILE* and exits 1, where the program has rules and nothing is wrong with it | `(import std.algebra :symbols (bijective-properties functional))` and no rule of its own | S1f.7.8's T3, writing its pin | exit 1, *no rule forms* |
| R-5 | **A class of a goal whose `(absent …)` holds a variable nothing outside it binds prints the variable.** The read-out's `as :expect` row then suggests a claim no program can write: a variable in a claim is a parse error, and so is `(absent (and …))`. Since S1f.7.5 expands a goal's macro calls, `forall` in a goal reaches it, its expansion being `(absent (and G (absent B)))` | `(query :goal (absent (q ?x)))` on a state holding no `q`: `as :expect (model (absent (q ?x)))`; `(query :goal (forall ?x (p ?x) (is-a ?x Thing)))` with `forall` imported: `as :expect (model (absent (and (p ?x) (absent (is-a ?x Thing)))))` | S1f.7.5's T1 | the goal is answered right (*the goal holds: it binds no variable*); the suggested claim does not load |
| R-6 | **A rule whose `:match` reads a relation nothing makes can never fire, and nothing says so.** No declaration, fact or conclusion names the relation, so no state holds a fact of it. Not refused (Q-M1f.120): a library rule may read a relation its importer supplies, and the stdlib's rules read markers a program may not write. Such a relation is also a relation of the program, so a goal over it loads and has no row | `(rule r () :match (and (p ?x) (zzz ?x)) :assert (won ?x) …)` beside `(p k)`, with `(query :goal (zzz ?x))` | S1f.7.11's T1 | loads; the rule never fires; the goal has no row |
