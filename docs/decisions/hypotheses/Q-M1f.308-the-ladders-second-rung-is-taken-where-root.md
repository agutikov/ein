# Q-M1f.308 — An obligation activated at root writes what would pay what root owes

Where root's activators compile a plan of an obligation rule (one with no parameters always), the obligation writes
at root `(open R)` for each relation an instance is owed of and `(open F)` for each fact that would pay one, and
$H$ holds those beside what a rule that concludes a hypothesis writes
([Q-M1f.570](Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)). `(solve :no-search)` proposes none. An
obligation rule the program loads and nothing activates writes nothing, so a module imported for another of its
names leaves the program to the blind enumerator. A root that owes nothing under an activated obligation proposes
nothing from it, and is complete because it is discharged.

**Why.** Where a program states an obligation, a state is judged by whether it is paid, and that judgement needs a
writer that proposes nothing once nothing is owed. Whether a program states one is a matter of what root activates:
every import brings its module whole, and importing one name must not change what is supposed.

**Held by.** `ein-cli/tests/import_invariance.rs`; `ein-infer/tests/obligation_rung.rs`; `ein_infer::hypgen`;
[Solution semantics § 1.4](../../ein/reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe).
