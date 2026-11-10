# Load-negative fixtures

Files that **parse** and then fail to **load**. Each `<name>.ein` sits beside a
`<name>.expected` holding the exact `KbLoadError` message it produces.

Sibling `../*.ein` are the *parse*-negative fixtures — they never reach the
loader. Everything here does, but `reserved_open_direct.ein`, which three
fixtures here import by path (`reserved_open_{aliased,qualified,symbols}`), so
it stays beside them although the grammar refuses `(macro open …)` at the parse
since M1f S1f.5.5's swap; the four `.expected` are that parse error.

**`nested_or` moved here from `../compile/` at M1f S1f.5.21** (W62, 2026-11-10):
the loader refuses an `(or …)` written as a premise since, one line naming the
rule, the disjunction and what it is a premise of, where the compiler refused
it in the Python IR's spelling. Every fixture here is refused by every command
that reads a program since the same stage, the four views that draw one
(`render program`, `rules`, `rule`, `constraints`) among them, which parsed
and never loaded a program until then (W65), so twelve of these fixtures'
`render rules` cells exit 1 where they drew (`corpus_exits.txt`).

**`eq_in_a_macro_body` moved here from `../compile/` at M1f S1f.5.24** (W66,
2026-11-10): the loader holds a rule's clauses, after macro expansion, to the
shapes the grammar gives a written rule, so `(eq ?x)` written in a macro's body
is refused where the macro is called, one line placed at the macro's body,
where the compiler refused it only where it compiled the rule and the views and
`kb save` accepted it; a macro no rule calls still loads (the user's ruling on
Q-M1f.67). It was the last compile refusal a program reached.

**Twenty-one moved to `../` at M1f S1f.5.5's swap** (T1f.5.5.4, 2026-11-09,
Q-M1f.2 (a)), each refused at the parse since, its `.expected` the parse
report: `config_bad_value`, `config_retired_flag`, `config_unknown_flag`,
`empty_absent`, `expect_is_a_pattern`, `expect_unknown_keyword`,
`hrule_reserved_name`, `import_alias_not_a_name`, `import_symbols_empty`,
`import_symbols_not_a_list`, `macro_reserved_name`, `open_arity`,
`open_in_match`, `open_mixed_assert`, `query_lattice_keyword`,
`region_top_level`, `relation_malformed`, `relation_needs_a_name`,
`relation_reserved_name`, `rule_reserved_name` and `solve_unknown_keyword`.
`relation_parameter_int` retired (S1f.5.4's Q2 (a)): `3` is a name since, and
names the relation `3`. `derivation_cycle` was rewritten, `:using ((r c d))`, a
list of facts (the paper grammar's default 2), and still reaches the loader's
cycle line. What the sections below say of a moved fixture is its record.

## The `:expect` refusals — M1c S1c.1.2

Four of the fixtures here are newer than the rest and share a reason. `:expect`
lets a query state its own answer, so a `(query …)` can now carry a *test* —
and a test that silently checks nothing is worse than no test at all. Each of
these four is a way to write one:

| fixture | what it would have checked |
|---|---|
| `expect_unknown_keyword.ein` | nothing — `:expct` parsed, loaded and was ignored |
| `expect_unknown_relation.ein` | the extent of a relation that does not exist, which is empty for ever |
| `expect_omits_the_goal.ein` | something other than what the query asked |
| `expect_is_a_pattern.ein` | whatever the engine derived — a `?var` matches it |

They are also the first loader messages in the repo that name no exception
class, which is what [`defined_behaviour.md`
§4.1](../../../docs/kernel/defined_behaviour.md) is about.

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

`import_cycle.ein` and `import_cycle_b.ein` import each other. Both are
fixtures: the reported chain is the resolution stack, so entering the cycle
from either side is a different message.

## What is *not* here

Three loader messages have no fixture, all for structural reasons rather than
oversight. Recorded so the gap is a decision and not a hole:

| message | why no fixture |
|---|---|
| `(macro) needs name + params + body` · `malformed (macro …)` · `(rule) needs name + params` · `malformed (rule …)` · `malformed (import …) — missing module name` · `fact with non-atom head` · `unexpected top-level form` | **deleted at M1f S1f.5.5's T5** (T1f.5.5.5, 2026-11-09). `(macro m)`, `(rule r)`, `(import)`, `(?R a b)` are all parse errors, the grammar guaranteeing the shape these checks re-verified; they were defence in depth for a caller that hand-builds an AST (`ein_ir::load` is public API). Since T5 the loader reads that shape off the tree with an `expect` naming the production of [`ein.y`](../../../ein.rs/crates/ein-ir/src/ein.y) that states it, so such a caller gets a panic naming the grammar rather than a load error, and each shape is a row of `ein-ir/tests/grammar_pages.rs`' table. |
| `non-rule form where a rule was expected` · `unresolved (import …) — internal error` | **unreachable by construction.** The top-level router only sends `rule`/`hrule`-headed forms to the rule ingester, and `resolve_imports` consumes every import before the router runs. Each is an internal-invariant assertion. |
| `(import M) — file-relative import needs a base directory` | **not expressible as a file.** It fires only when `base_dir` is `None`, i.e. `ein_ir::load(…, None)` over parsed source — and loading a *file* always supplies one, on every command since M1f S1f.5.5's T1f.5.5.13 (2026-11-09): until then `ein saturate` loaded a file with none, and refused a file-relative import with this line (W41, `examples/ein-bugs/wrong-answers/saturate-import.ein`). It is an inline unit test in [`ein-ir/tests/kb_semantics.rs`](../../../ein.rs/crates/ein-ir/tests/kb_semantics.rs). |

Almost every message ends `at None`. The loader interpolates the form's
`loc`, and the AST lowerer synthesises the heads of `relation` / `rule` /
`macro` / `import` forms without one, so top-level forms carry no position.
The two exceptions are `macro_arity_mismatch`, raised during expansion on a
nested (located) node, and `relation_parameter_int` (M1e2 S1e2.2.4), which
positions its message at the activator's integer for the same reason. It is a real usability bug, and
[`defined_behaviour.md` §1.5](../../../docs/kernel/defined_behaviour.md)
states `at None` as the behaviour until it is fixed.

Reserved-name shadowing has one fixture per declarator namespace
(`relation` / `rule` / `hrule` / `macro`) rather than one per reserved word:
the message differs by namespace, so the fixtures cover the message set, and
the sweep over `{absent, false, eq, relation}` stays a parametrized unit test
where it belongs.

## A relation named by a number — M1e2 S1e2.2.4

`relation_parameter_int.ein` is Q-M1e.21's residue. A rule parameter that names
a relation, `(?R ?a ?b)`, bound by its activator to an integer would have the
compiler intern the integer's text as a relation's name: a name the program
does not have, which no rule's text predicts and so no candidate can hold. The
user ruled on 2026-10-16 that rules cannot assert new names, and the loader
refuses the activator where it is written down — a fact, as here, or a literal
in an `:assert`. One a rule derives through a variable was refused by the
compiler, and had no fixture here because it loaded, until M1f S1f.5.18 refused
it at load (below; `ein-infer/tests/compile_semantics.rs`;
[`defined_behaviour.md` § 4.6](../../../docs/kernel/defined_behaviour.md#46-a-relation-named-by-a-number--m1e2-s1e224)).

A nested fact in that place is refused the same way since M1e2 S1e2.2.24
(W24, Q-M1e2.18 (iv)): it binds no parameter, so the relation it would name
is none (*… binds it to the fact (a b) — a relation is named by a symbol, not
a fact*). Its witness is
[`ein-bugs/wrong-answers/nested-fact-activator.ein`](../../ein-bugs/wrong-answers/nested-fact-activator.ein),
and the derived one was the compiler's, beside the integer's, until M1f
S1f.5.18 (below).

## A derived activator that binds a relation parameter to a fact — M1f S1f.5.18

`derived_activator.ein` and `derived_activator_never.ein` are S1f.5.5's two
`g14` probes: a rule, `mk`, derives `r`'s activator `(r ?f)` from `item`'s
argument, and `r`'s parameter names a relation. `item` is declared, so its
argument holds any value, a fact among them, and the loader refuses the program
whether or not a fact puts one there (the user's ruling of 2026-11-08 on
Q-M1f.1 (b)), one line naming the deriving rule, the activated rule and the
parameter. Until S1f.5.18 the first stopped where `r`'s plan was compiled, the
compiler's `CompileError` the section above names, and the second answered, 64
states; each is a row of the register, W59
([`defined_behaviour.md` § 4.9](../../../docs/kernel/defined_behaviour.md#49-a-rule-shape-that-reads-nothing--m1e2-s1e2224)).
A marker's argument, `(bijective ?R)`, passes a relation name on and loads: the
loader reads what the program does with it (Q-M1f.41).

## Two from `../compile/` — M1e2 S1e2.2.24

`empty_absent.ein`, `(absent ?x)`, and `unbound_head.ein`, `(?R ?x)` with `?R`
no parameter, were compile-negative fixtures (S1a.3.1) until the loader began
refusing a premise that is not a form and a relation variable no parameter
binds (W24, the user's ruling on Q-M1e2.18 (v), 2026-11-05). Each `.expected`
is its load line, where the compiler's `CompileError` in the Python IR's
spelling stood; the run's exit, 1, did not move.

## Growth rule

Any load error found outside this set becomes a fixture in the same commit that
fixes it. The set only grows.
