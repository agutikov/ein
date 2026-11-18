# Q-M1f.306 — The guessable atoms are the program's names, never a kernel dunder

The atoms the blind enumerator guesses about, the **guessable atoms**, are the names index at root's fixpoint, which
holds every fact's relation and arguments, and every name of the program
([Q-M1e.21](Q-M1e.21-a-rule-brings-in-no-name.md)), less the names a declaration writes as a label, the reserved
words, and every name the registry calls a relation or a rule. Of what remains:

- **the kernel's dunder is never guessable**: `__closed__` is left out by name, where a rule derives a fact of it
  and where a program writes it as an argument, which stays legal and keeps it a name of the program. One closure
  stated by a fact or by a rule has one H and one R;
- **a relation's name that only a rule's conclusion writes is guessable**: the candidates stay in H, every question
  prints a `relation names` row under `universe`, and the summary carries `root.hypgen.relation_names_as_objects`,
  *object* being the read-out's word for a guessable atom. Declaring the relation takes its name out;
- **what an import brings is the program's**: a name only an imported module writes is guessable as the main file's
  are, whatever the import form.

**Why.** The dunder is the kernel's word for a relation's closure, nothing a puzzle is about, and guessed about it
gave one program stated two ways two answers. A derived relation's name is guessable for the reason an atom
on the right of an `is-a` is: the program left it undeclared, and the row tells it how to say otherwise. Every
import brings its module whole, and the answer does not depend on the import form.

**Held by.** `ein-infer/tests/dunder_objects.rs`: `a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers`,
`a_dunder_written_as_a_value_is_no_object`, `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value`;
`ein-cli/tests/relation_names_row.rs`: `a_relations_name_bound_as_an_object_is_one_sentence_on_every_surface`;
`ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form`;
`ein_infer::hypgen::candidate_objects`.
