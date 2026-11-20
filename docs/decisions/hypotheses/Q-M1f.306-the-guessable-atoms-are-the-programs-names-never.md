# Q-M1f.306 — The guessable atoms are the program's names, never a kernel dunder

The atoms the blind enumerator guesses about, the **guessable atoms**, are the names index at root's fixpoint, which
holds every fact's relation and arguments, and every name of the program
([Q-M1e.21](Q-M1e.21-a-rule-brings-in-no-name.md)), less the names a declaration writes as a label, the reserved
words and the kernel's words `absent` and `open`, which a program may write as names, every relation of the program and every rule's name, and the relation of every fact root's fixpoint holds.
Of what remains:

- **the kernel's dunder is never guessable**: `__closed__` is left out by name, where a rule derives a fact of it
  and where a program writes it as an argument, which stays legal and keeps it a name of the program. One closure
  stated by a fact or by a rule has one H and one R;
- **no relation is guessable**, whichever way the program writes it: a relation only a rule's conclusion writes,
  or one only an activator gives a parameter, is a relation of the program as a declared one is
  ([Q-M1f.522](../language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md));
- **what an import brings is the program's**: a name only an imported module writes is guessable as the main file's
  are, whatever the import form.

**Why.** The dunder is the kernel's word for a relation's closure, nothing a puzzle is about, and guessed about it
gave one program stated two ways two answers. A relation is the atom first in a tuple, however the program writes
it, and no thing a candidate is about. Every import brings its module whole, and the answer does not depend on the
import form.

**Held by.** `ein-infer/tests/dunder_objects.rs`: `a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers`,
`a_dunder_written_as_a_value_is_no_object`, `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value`;
`ein_infer::hypgen`'s unit test `candidate_objects_hold_no_relation`;
`ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form`;
`ein_infer::hypgen::candidate_objects`.
