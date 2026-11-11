# Natural language — what a program says in English, and where it is printed

**Status: current** — M1f [S1f.5.14](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md),
2026-11-10. The texts a program writes ([§ 1](#1-the-four-texts)), the rule that every declaration carries one
([§ 2](#2-the-mandate-every-declaration-carries-a-why)), how a fact reads as English from its provenance
([§ 3](#3-a-fact-in-english-the-fallback-ladder)), the four registers the census splits the answer into
([§ 4](#4-the-four-registers-and-the-census)), the forms a template has nowhere to live on
([§ 5](#5-where-a-template-has-nowhere-to-live)), and the line between the domain's words and the engine's
([§ 6](#6-the-domain-and-the-engine-whose-words-are-whose)). What holds it is [§ 7](#7-what-holds-this-page).

The user's note that started it, 2026-08-28: *"make `:why` or other →NL conversion string/template mandatory, so
every fact, match, every Ein statement can be converted to NL."* The return leg, NL back to Ein, is
[M2](../../../../plans/m2_nl_to_ir/README.md)'s, not this page's.

## 1. The four texts

A string is natural-language text, written only after `:why`, `:source` and `:goal-text`
([`01_grammar.md`](01_grammar.md), M1f S1f.5.2), Unicode with the escapes `\n \t \r \" \\ \u{…}`. A template fills
`{?ref}` from bindings ([`ein_core::render_why`](../../../../ein.rs/crates/ein-core/src/why.rs)) and leaves a
reference nothing binds as written, which is why [§ 7](#7-what-holds-this-page)'s first check exists.

| text | on | fills | printed by |
|---|---|---|---|
| `:why` | `(relation R …)` | **positional**: `{?1}` the fact's first argument, `{?2}` its second, … — the fact's **statement** | a `--dump-answer` state file, after each fact of R; `--trace`, after each derived fact of R (*Derives `…` — …*) |
| `:why` | `(rule …)`, `(hrule …)` | the firing's variables: a parameter, or a variable a positive premise binds — a derived fact's **reason** | `--trace`'s step line, the DOT slice's rule node, an obligation's owe line, a state file after each fact the rule derived first |
| `:why` | `(macro …)` | a parameter | nothing ([§ 5](#5-where-a-template-has-nowhere-to-live)); it documents the pattern |
| `:goal-text` | `(query …)` | the goal's variables, per row | the read-out's projection class (`“…”`) |
| `:source` | a fact the program states | — | a state file after the fact; `--trace`'s premises line |

`ein-cli/tests/natural_language_text.rs` holds each text byte for byte on every surface that prints it, with Cyrillic,
CJK, an emoji and every escape.

## 2. The mandate: every declaration carries a `:why`

**A `(relation …)`, `(rule …)`, `(hrule …)` and `(macro …)` without `:why "…"` is refused at load**, one line per form
([`defined_behaviour.md` § 4.10](../../defined_behaviour.md#410-a-form-with-no-why--m1f-t1f5146-t1f5147)). The grammar keeps `:why` optional; the
loader decides, through [`ein_ir::LoadOptions::require_why`](../../../../ein.rs/crates/ein-ir/src/from_ir.rs), **true by
default since T1f.5.14.7** (2026-11-10), once every program the corpus, the stdlib and the Rust suites load carried
one; `--no-require-why` on the seven commands that load a program loads one without
([`configuration.md` § 3.4](../../configuration.md#34-the-option-that-decides-loading)), as `--require-why`, default
off, required one from T1f.5.14.6 until then. It decides whether a program loads and never what a loaded program answers (the user,
2026-11-08: *"yes, the flag only decides loading"*). `:goal-text` and `:source` stay optional: a query's answer reads
through its classes, and only a given has a sentence to quote.

This is rung (d) of the four the stage weighed, *a load error*, on the user's ruling of 2026-11-08 (*":why is
optional in grammar and loader has a flag that makes it required, start with default false so not break everything,
then after other stages finished swith to default true and fix remaining ein code"*), where the stage had
recommended a census and a gate scoped to the stdlib and refused (d) in writing
([§ T4](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#task-t1f5144--the-mandate-and-where-it-binds)).
That argument is the mandate's risk: **100 % coverage bought with `:why "rule fired"` is worse than an honest
s-expression.** So a `:why` says what its form says, in English:

- a relation's, the sentence one fact of it reads as: `(relation drink-loc Drink House :why "{?1} is drunk in {?2}")`;
- a rule's, what it concludes and from what: `:why "{?a} knows {?b}, so {?b} knows {?a}"`;
- a refutation's (`:assert (false)`), what cannot be; an obligation's (`:assert (open …)`), what is still owed;
- an hrule's, the guess it proposes: `:why "guess that {?x} is in box {?b}"`;
- a made-up name (`p`, `r1`) is said as what it is, `"p holds of {?1}"`, and never given a meaning it does not have.

[`stdlib/`](../../../../stdlib/) is the house style, and lived by the rule before it was one: 77 of 77 rules
then (`ein-infer/tests/stdlib_coverage.rs`). T1f.5.14.7 migrated the corpus, the Rust suites' inline programs and the doc
pages' code blocks, every `:why` written by reading the program (the stage file's
[§ T1f.5.14.7 record](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5147-record-2026-11-10)),
and `utils/nl_census.py --check` exits 1 while any program the loader reads lacks one.

## 3. A fact in English: the fallback ladder

A fact has two texts, from two places
([`ein_render::sentence`](../../../../ein.rs/crates/ein-render/src/sentence.rs), T1f.5.14.2):

- its **statement**, what it says: its relation's `:why` with its arguments;
- its **reason**, why it holds, read off its **first recorded justification** ([`Kb::primary`](../../../../ein.rs/crates/ein-core/src/kb.rs)).

The reason by the justification's provenance kind, `ProvKind`'s four
([`prov.rs`](../../../../ein.rs/crates/ein-core/src/prov.rs)):

| provenance | the reason | the state file writes |
|---|---|---|
| `source`, with a sentence | the program's `:source`, quoted | `; source "…" — <statement>` |
| `source`, with none (a given) | none | `; <statement>` |
| `rule` | the rule's `:why`, filled with that firing's bindings | `; rule r: <reason> — <statement>` |
| `hypothesis` | none: the commitment, which the engine names | `; hypothesis N — <statement>` |
| `rejected` | — | never in a state |

**The ladder.** A fact reads from the program's texts where it has one: its `:source`, its rule's `:why`, its
relation's `:why`. Where it has none it reads as its s-expression, `(R a b)`, which is true, and **never as invented
prose**: there is no built-in relation-to-verb vocabulary. The census counts a fact with a text as *rendered* and one
with none as a *fallback* ([§ 4](#4-the-four-registers-and-the-census)).

**First recorded.** A fact can carry up to `MAX_ALT_JUSTIFICATIONS` more justifications, and which arrived first can
move with rule order. A reason is an explanation, as a core is: the user's ruling of 2026-11-08 (*"plain FIFO, core
is an explanation, F22 makes it canonical"*) covers both, and F22 makes them canonical. Within a run the order is
deterministic, and `ein-render/tests/id_order_invariance.rs` and `jobs_invariance.rs` hold the state files (the sweep's
`states` op) to it.

A state file from `ein solve examples/zebra.ein --dump-answer DIR`, `DIR/models/01.ein`, held to a run by
`ein-cli/tests/natural_language_page.rs`:

<!-- state-file: examples/zebra.ein -->
```lisp
(co-located Englishman Red) ; source "condition (2)" — Englishman and Red are in the same house
(co-located House-1 Water) ; rule slot-fill: every Drink except Water is excluded for slot House-1 ⟹ Water is at House-1. — House-1 and Water are in the same house
(instance House-5 House) ; House-5 is an instance of House
(next-to House-2 House-1) ; rule includes: next-to includes right-of: from House-2 →right-of→ House-1, derive House-2 →next-to→ House-1. — House-2's house is next to House-1's
```
<!-- /state-file -->

**What is not rendered.** The read-out's `(model …)` blocks and its `as :expect` are Ein text a claim can be pasted
from, so they stay s-expressions, as `ein solve`'s stdout and `answer.txt` do; a state file reloads to its facts
because the English is a comment (`ein-cli/tests/answer_round_trip.rs`), a line break in it a comment line of its
own. `--trace` writes markdown, so there a text's line break is a hard break and the line after it stays in its block,
a rule's `:why` in its quote (M1f S1f.5.28, [Q-M1f.109](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f109--a-texts-line-break-in-the-traces-markdown);
`ein-cli/tests/natural_language_text.rs`): written as it stands, markdown read it as a soft break and joined the lines.

**What it costs.** Nothing on a bare `ein solve`, which renders no fact. On `--dump-answer` and `--trace` over the
zebra family the rendering is within the runs' spread: [§ T1f.5.14.2 record](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5142-record-2026-11-10)
has the table (`utils/bench_env.sh`, 2026-11-10).

## 4. The four registers, and the census

[`utils/nl_census.py`](../../../../utils/nl_census.py) solves every corpus entry with a `solve` run as `ein solve FILE
--dump-answer DIR` and reads every fact of every state file, the whole state (root's facts too), with the program the
file re-writes above it. Each fact's relation is in one register:

| register | the relation is | who can template it |
|---|---|---|
| **puzzle** | declared by the program or a corpus module it imports, or a rule of theirs (an activator) | the program |
| **stdlib** | a `std.*` rule's name (an activator, `(bijective color-loc)`), or a relation a `std.*` rule asserts | nobody: no form declares it, and a fact of it renders by its reason |
| **kernel** | a name the engine reserves, `ein_core::RESERVED`: `not`, `relation`, `__closed__`, … | the engine, which does not ([§ 6](#6-the-domain-and-the-engine-whose-words-are-whose)) |
| **undeclared** | written or derived, never declared, and no rule's | nobody, until the program declares it |

It prints, per entry and for the corpus, the facts whose relation carries a `:why` (*statement*), whose first
justification has a text (*reason*), either (*rendered*), and the fallbacks by register. The register split is the
deliverable: a total says how much, the split says who could fix it.

On `examples/zebra2.ein`'s one state, 426 facts: before the migration, 5.9 % had a statement, 88.5 % a reason, and
88.7 % rendered, the 48 fallbacks the givens of `is-a`, `bijective`, `symmetric`, `transitive`, `includes` and the two
hierarchies (`python3 utils/nl_census.py --slow -k zebra2.ein`, 2026-11-10, at `b7108e3a`). **After it, 41.5 % have a
statement, 88.5 % a reason, and 97.4 % render** (the same command at `9c06414f`): the 11 fallbacks left are the given
activators of `std.*` rules, `(bijective …)`, `(includes …)`, `(symmetric next-to)`, `(transitive is-a*)` and the two
hierarchies, which no form can carry a `:why` for (§ 5). Over the corpus, 239 359 facts of 211 entries, 44.9 %
rendered before and 94.4 % after, and 104 of the 203 entries with a fact render every one, where 20 did. The census,
every root and register, before and after, is the stage file's
[§ T1f.5.14.1](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5141-record-2026-11-10)
and [§ T1f.5.14.7](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5147-record-2026-11-10)
records.

## 5. Where a template has nowhere to live

Taken by default on the user's instruction of 2026-11-09 (T1f.5.14.3), each a defaulted question:

- **A macro's `:why`** ([Q-M1f.59](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f59--what-a-macros-why-renders)):
  required like the others since S1f.5.5's swap gave a macro one, and rendered nowhere. A macro is expanded at load;
  a fact's justification names the rule whose `:match` used it, and that rule's `:why` says what the pattern meant
  there. The macro's `:why` documents the pattern for a reader of the program, and a state file re-writes it with
  the program.
- **An auto-vivified relation** ([Q-M1f.60](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f60--a-relation-nobody-declares)):
  a relation created for an open-world head has no declaration to carry a `:why`, and the stdlib cannot declare the
  puzzle's relations. A fact of one that a rule derived reads by its reason; one the program states falls back to its
  s-expression, which the census counts as `stdlib` or `undeclared`. A program that wants it read declares it.
- **A goal's conjuncts** ([Q-M1f.61](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f61--a-template-per-goal-conjunct)):
  `:goal-text` is one string for the whole query, and a template per conjunct would be a language change. None is
  made: each conjunct's fact reads by its relation's `:why` where a surface prints it.

## 6. The domain and the engine: whose words are whose

**Every text a program writes is the domain's**: the puzzle's `:why`s, `:source`s and `:goal-text`s, and the
stdlib's, which is a program. The engine fills them and never edits them. **The words around them are the
engine's**: the read-out's labels ([`read_out.md`](../../inference/read_out.md)), the trace's headings and its
*Derives* and *Premises*, the provenance words a state file writes (`source`, `rule`, `hypothesis`) and its separators
(`: `, ` — `), and the verdict and protocol words [`reserved_engine_strings.md`](../../inference/reserved_engine_strings.md)
registers. The engine owns no domain word: it has no sentence for a relation that has none, and a `kernel` relation,
`not` or `relation`, reads as its s-expression. A stage that adds a word to an output adds it on the engine's side,
in `read_out.md` or `reserved_engine_strings.md`, and never in a renderer's vocabulary for the domain.

## 7. What holds this page

- `ein-ir/tests/template_wellformedness.rs` (T1f.5.14.5): every `{?ref}` in a corpus or stdlib `:why` names something
  its form can fill — a relation's a position within its signature, a rule's or an hrule's a variable it binds (a
  parameter, or a positive premise's, macros expanded), a macro's a parameter — and no `{x}` is a reference with its
  `?` missing. When it was written it found 66 over 893 templates (2026-11-10), each fixed in the same commit.
- `no_placeholder_reaches_an_output` (T1f.5.14.5, `ein-cli/tests/answer_round_trip.rs`): no line of `answer.txt` or
  of a state file's facts, over every corpus program the per-commit tier dumps, carries `{?`; `corpus_cli.rs` holds
  every `--trace` run's file to the same.
- `ein-cli/tests/natural_language_text.rs` (§ 1, § 3), `ein-ir/tests/load_semantics.rs`' and
  `ein-cli/tests/config_reference.rs`' `require_why_*` tests (§ 2), and the corpus sweep's `states` op (§ 3).
- `ein-cli/tests/natural_language_page.rs`: § 3's block is lines of the state file its marker's run writes, § 4's
  four registers are the census's, the census's `kernel` is `ein_core::RESERVED`, and every question the page cites
  is a heading of M1f's `open_questions.md`.
