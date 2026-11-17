# Q-M1f.75 — The compiler's errors are a library caller's, in Ein's words

Four of the compiler's refusals are reached by no program, and by a direct caller of `compile_rule` with a tree or
an activator the loader would not have let through: an activator whose argument count is not the rule's parameter
count, a relation head its activator binds to a fact, a relation variable compiled with no activator to bind it,
and a guard's argument that is a form holding a variable, `(eq ?x (f ?y))`, which the loader refuses in a rule and
in a query's goal in the same words ([Q-M1f.30](../language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)).
Each stays a `CompileError` for that caller, its line in Ein's words: the activator written as a fact, the
parameters as the rule writes them, and a form's place as `<file>:<line>:<col>`, the spelling `ein_ir::loc_repr`
gives every line that places one. The arms the loader covers are invariants instead
([Q-M1f.84](Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md)).

**Why.** The call is one the library's API allows, so its refusal is a diagnostic the caller can read, not a
panic; and a refusal, whoever reaches it, speaks the language the program was written in.

**Held by.** `ein_infer::compile::CompileError` and `ein_ir::loc_repr`; `ein-infer/tests/compile_semantics.rs`:
`an_arity_mismatched_activator_is_refused_and_never_reaches_the_compiler`,
`every_compile_error_says_what_its_expected_file_says`.
