# Q-M1f.306 — The blind enumerator's objects are the program's names, never a kernel dunder

The blind enumerator's objects are the names index at root's fixpoint, which holds every fact's head and arguments,
and every name of the program ([Q-M1e.21](Q-M1e.21-a-rule-brings-in-no-name.md)), less the names a relation
signature declares as a type, the reserved words, and every name the registry calls a relation or a rule. Of what
remains:

- **a kernel dunder is never an object**: `__closed__` and `__symmetric__` are left out by name, where a rule derives
  the fact one heads and where a program writes one as a value, which stays legal and keeps it a name of the program.
  One closure stated by a fact or by a rule has one H and one R;
- **a relation's name that only a rule's conclusion heads is an object**: the candidates stay in H, every question
  prints a `relation names` row under `universe`, and the summary carries `root.hypgen.relation_names_as_objects`.
  Declaring the relation takes its name out;
- **what an import brings is the program's**: a name only an imported module writes is an object as the main file's
  are, whatever the import form.

**Why.** A dunder is the kernel's word for a relation's closure or symmetry, nothing a puzzle is about, and as an
object it gave one program stated two ways two answers. A derived relation's name is an object for the reason a type
is: the program left it undeclared, and the row tells it how to say otherwise. Every import brings its module whole,
and the answer does not depend on the import form.

**Held by.** `ein-infer/tests/dunder_objects.rs`: `a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers`,
`a_dunder_written_as_a_value_is_no_object`, `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value`;
`ein-cli/tests/relation_names_row.rs`: `a_relations_name_bound_as_an_object_is_one_sentence_on_every_surface`;
`ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form`;
`ein_infer::hypgen::candidate_objects`.
