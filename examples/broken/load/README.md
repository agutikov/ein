# Load-negative fixtures

Files that **parse** and then fail to **load**. Each `<name>.ein` sits beside a `<name>.expected` holding the
exact message it produces. Every fixture here is refused by every command that reads a program, the four views
that draw one (`render program`, `rules`, `rule`, `constraints`) and `kb save` among them.

Sibling `../*.ein` are the *parse*-negative fixtures: they never reach the loader. Everything here does, but
`reserved_open_direct.ein`, which three fixtures here import by path (`reserved_open_{aliased,qualified,symbols}`),
so it stays beside them although the grammar refuses `(macro open …)` at the parse; those four `.expected` are a
parse error, read where the module is.

## What is here

- **The `:expect` refusals.** `:expect` lets a query state its own answer, so a `(query …)` can carry a *test*,
  and a test that silently checks nothing is worse than no test at all. Each of these is a way to write one:

  | fixture | what it would have checked |
  |---|---|
  | `expect_unknown_relation.ein` | the extent of a relation that does not exist, which is empty for ever |
  | `expect_omits_the_goal.ein` | something other than what the query asked |
  | `query_over_unknown.ein` | an `:over` that names none of `all`, `minimal`, `maximal` |
  | `query_class_twice.ein` | one class listed twice |
  | `region_merged_unlisted.ein` | a `(merged …)` whose state the region does not list |
  | `region_owing_with_states.ein` | an `(owing …)`, which says R is empty, beside states |

  [`07-diagnostics.md` § 2.1](../../../docs/ein/user/04-reference/07-diagnostics.md#21-a-querys-a-solves-and-a-claims-refusals)
  states each refusal.
- **An `(or …)` written as a premise of an `and`**, `nested_or.ein`: the loader refuses it in one line naming the
  rule, the disjunction and what it is a premise of. An `(or …)` is a whole `:match`, each of its premises one way
  it holds.
- **A malformed primitive a macro's body writes**, `eq_in_a_macro_body.ein`: the loader holds a rule's clauses,
  after macro expansion, to the shapes the grammar gives a written rule, so `(eq ?x)` written in a macro's body is
  refused where the macro is called, the line placed at the macro's body; a macro no rule calls still loads
  ([Q-M1f.67](../../../docs/decisions/language/Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)).
- **`:ein`, a rule's word to the engine**
  ([Q-M1f.399](../../../docs/decisions/language/Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md)): the
  grammar takes one name after it, and the loader holds the rest, each line listing the engine's words.
  `ein_word_unknown` writes `:ein dead`, a word that is none of the engine's; `ein_beside_assert` writes it beside
  an `:assert`, where a rule states one or the other.
- **No declarator binds `open`**
  ([Q-M1f.66](../../../docs/decisions/language/Q-M1f.66-open-is-a-name-and-the-kernel-reads.md)): it is a
  name wherever a name goes, and the kernel's word at the head of a written fact, a premise or a conclusion.
  `reserved_open_symbols`' `:symbols (open)` reads the module, and the four `reserved_open_*` keep the module's parse
  error, with its note, *open takes one argument: …*.
- **What a state owes is a rule's whole `:assert`**
  ([Q-M1f.575](../../../docs/decisions/language/Q-M1f.575-open-r-is-a-rules-whole-assert-and-open-f-stands.md)):
  `open_mixed_assert` writes `(open ?R)` beside a fact, refused where the `(open ?R)` is written;
  `open_compound_witness`, whose `(open ?R)` resolves to no single witness, starts its line the same way.
- **A list that disagrees with root's `open` facts**
  ([Q-M1f.578](../../../docs/decisions/hypotheses/Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)):
  `no_hypothesis_beside_open` names in `:no-hypothesis` `likes`, a relation root holds an `open` fact of,
  `(open likes)`, which the obligation `total-owed` writes.
- **A relation variable no parameter binds**, `unbound_head.ein`, `(?R ?x)` with `?R` no parameter: the loader
  refuses it where the rule is written, so it never reaches the compiler. A relation parameter an activator binds to
  a fact is no refusal: the program loads under a warning, and the activator activates nothing
  ([`07-diagnostics.md` § 2.9](../../../docs/ein/user/04-reference/07-diagnostics.md#29-a-rule-shape-that-reads-nothing);
  [`features/19_derived_activator.ein`](../../features/19_derived_activator.ein)).
- **Imports**: a module not found, a relative path not found, a bare `std`, `:as` and `:symbols` together, a short
  name listed that the module does not provide, two definitions of one short name, and a cycle.
  `import_cycle.ein` and `import_cycle_b.ein` import each other. Both are fixtures: the reported chain is the
  resolution stack, so entering the cycle from either side is a different message.
- **Duplicates and missing parts**: a rule declared twice under one name, a rule that concludes a hypothesis among
  them (`hrule_duplicate_name.ein`), a macro twice, two `(solve …)`
  forms, a rule with no `:match` or no `:assert`, the two half-specified declarators (`rule_half_declarators.ein`,
  a node-kind probe too), a macro called with the wrong count, a `std` macro used without its import, and a
  derivation cycle in written provenance (`derivation_cycle.ein`, `:using ((r c d))`).

## Format

```
<name>.ein          the fixture — a parseable file the loader rejects
<name>.expected     the exact message, one line, with placeholders
```

Placeholders — the only machine-specific text any message contains:

| placeholder | expands to |
|---|---|
| `{FILE}` | the fixture's own path, as handed to the loader |
| `{DIR}` | its directory — the base for file-relative imports |
| `{STDLIB}` | the resolved stdlib root |

Consumers:
[`ein-ir/tests/load_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/load_semantics.rs)
(every fixture, byte for byte), and the corpus sweep
([`ein-cli/tests/corpus_cli.rs`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)),
which checks that each fixture is refused through the CLI as well as through
the loader.

## What is *not* here

Three loader messages have no fixture, all for structural reasons rather than
oversight. Recorded so the gap is a decision and not a hole:

| message | why no fixture |
|---|---|
| a malformed `(macro …)`, `(rule …)` or `(import …)`, a fact with a non-atom head, an unexpected top-level form | **the grammar refuses each shape at the parse**: `(macro m)`, `(rule r)`, `(import)`, `(?R a b)` are parse errors. The loader reads that shape off the tree with an `expect` naming the production of [`ein.y`](../../../ein.rs/crates/ein-ir/src/ein.y) that states it, so a caller that hand-builds an AST (`ein_ir::load` is public API) gets a panic naming the grammar rather than a load error, and each shape is a row of `ein-ir/tests/grammar_pages.rs`' table. |
| `non-rule form where a rule was expected` · `unresolved (import …) — internal error` | **unreachable by construction.** The top-level router only sends `rule`-headed forms to the rule ingester, and `resolve_imports` consumes every import before the router runs. Each is an internal-invariant assertion. |
| `(import M) — file-relative import needs a base directory` | **not expressible as a file.** It fires only when `base_dir` is `None`, i.e. `ein_ir::load(…, None)` over parsed source, and loading a *file* always supplies one, on every command. It is an inline unit test in [`ein-ir/tests/kb_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/kb_semantics.rs). |

A message about a top-level form ends `at None`: the form is built without a source position
([diagnostics § 1.5](../../../docs/ein/user/04-reference/07-diagnostics.md#15-loader-messages-about-a-top-level-form-end-in-at-none),
[Q-M1a.6](../../../docs/decisions/diagnostics/Q-M1a.6-a-loader-message-about-a-top-level-form-names-no.md)). A
message about a form inside another names the file, the line and the column, as `macro_arity_mismatch`'s does, raised
during expansion on a nested node, and as the lines that start `{FILE}:<line>:<col>:` do.

A reserved name used as a declarator's name is refused at the parse, so its fixtures, one per declarator namespace,
are in `../`.

## Growth rule

Any load error found outside this set becomes a fixture in the same commit that
fixes it. The set only grows.
