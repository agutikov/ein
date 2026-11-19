# S1f.21.17 — a warning where the loader refused for what a rule could derive; `std.core` by import and by `$EIN_STDLIB`

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's answers to review items 38 and 41
([the rulings](README.md#the-rulings-of-the-answers-file)); **not started**: it starts on the user's word.
**Waits on:** for task 1, **the user's word that the reading below is theirs**; for tasks 2 and 3, nothing. In
order it is the next stage of part 1, before [S1f.21.11](s1f.21.11_kill_and_three_conventions.md);
[S1f.21.16](s1f.21.16_hrules_lists_activators.md), which stood before it, is dropped.
**Writes:** the loader's check on an activator a rule could derive and the compiler's reading of such an
activator; the loader's reading of `(import std.core)` and of `$EIN_STDLIB`; the diagnostics' page, the page of
`std.core`, the install page; the decisions it amends; `AGENTS.md` where it states either.

## The answers

The user, 2026-11-19, in part 1's answers file, verbatim.

- Item 38 ([Q-M1f.559](../open_questions.md#q-m1f559--the-elimination-and-the-module-every-program-gets)): *"forbid becomes a warning, fix :hrulse activators syntax"*
- Item 41 ([Q-M1f.563](../open_questions.md#q-m1f563--what-a-module-every-program-gets-brings)): *"import std.core works like duplicated import; $EIN_STDLIB allows to replace std.core is in $EIN_STDLIB there is no std.core - print error about std.core not found in $EIN_STDLIB; There is still only a single user of ein - me, so I don't care about backward compatibility. Forget about it until I explicitely ask. Containers not load - delete and create new."*

The second half of item 38's was [S1f.21.16](s1f.21.16_hrules_lists_activators.md)'s, which is dropped: the user's
later ruling removes the clause it spelled (Q-M1f.567). The rest is here.

## Task 1 — the refusal for what a rule could derive becomes a warning

**The reading, the coordinator's.** *Forbid* names, everywhere else, the refusal of a refutation under an `absent`,
which the user removed the same day ([Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes),
S1f.21.15). Item 38 is about another refusal: at load, of a program in which a rule *could* derive an activator
that puts a fact where a relation's name goes, whether or not any run derives one. The answer stands under that
item, and stood there when the user was told that the two differ and asked which was meant; so it is read as that
item's option (d): **this refusal becomes a warning, and the program loads**. It reverses
[Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b), the user's of 2026-11-08. **Nothing of
this task is built before the user has said the reading is theirs.**

What the check is, and where each step of it was decided, is in review item 38's page (in `git log`, last at
`bedbfbd4`) and in the ledger's Q-M1f.559.

1. **What it refuses today**, by the binary: five corpus programs, with the words *a relation is named by a symbol,
   not a fact* (`ein show hyps` over `examples/` and `tests/`, on `946ee788`):
   `examples/broken/load/derived_activator.ein` and `derived_activator_never.ein`, and under
   `examples/ein-bugs/wrong-answers/`, `derived-activator.ein`, `derived-activator-never.ein` and
   `nested-fact-activator.ein`. Each is shown loading, with what it answers, before anything is banked.
2. **At load**: each line the refusal printed is printed as a `warning:` on stderr, and the program loads.
3. **At run, where a state does derive such an activator.** No fact has a fact under its first arrow, so a pattern
   whose relation is a fact matches nothing, and a conclusion whose relation is a fact is no object of the
   knowledge base. **Reading 1**, default: the activator is held as the fact it is and **activates nothing**, no
   plan compiled for it, and a `warning:` names the rule and the activator where a state derives it. The other
   reading: the run stops there with the compiler's line, as it did before the check was at load.
4. **What expected the refusal**: the two fixtures under `examples/broken/load/` with their expected lines, the
   three witnesses' rows of `corpus/regressions.toml`, the diagnostics' section, the grammar page's table if it has
   the shape, and the decisions Q-M1f.1, Q-M1f.41 and Q-M1f.42, each rewritten in place to what is true: what the
   warning reads, and that nothing is refused.

## Task 2 — `(import std.core)` is an import like any other

Today it is refused: *std.core is the module every program gets, with no import line: no import names it*. The
user: *"import std.core works like duplicated import"*. A second import of a module a program already has loads
today in silence, one copy, in the same form or another (`(import std.negation)` twice; `(import std.negation)`
beside `(import std.negation :as n)`: each run on `946ee788`).

1. The refusal goes. `(import std.core)`, in a program or a module, in any of the three forms, binds the module's
   names as an import does and brings no second copy.
2. **Reading 2**, default: where the module stands is not moved by an import of it: after the program's forms and
   its imports, as [Q-M1f.551](../../../docs/decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)
   has it, so a program lists the same rules in the same order with the line and without.
3. `ein-cli/tests/import_invariance.rs` holds a program to its answer with the line added.

## Task 3 — `$EIN_STDLIB` may replace `std.core`

The user: *"$EIN_STDLIB allows to replace std.core is in $EIN_STDLIB there is no std.core - print error about
std.core not found in $EIN_STDLIB"*. It is the first answer to how the module is customised, which Q-M1f.551 left
for later. Today, on `946ee788`, with a copy of the library under `$EIN_STDLIB`:

| the copy | today |
|---|---|
| its `core.ein` edited, the manifest left as it was | loads; the edited module is every program's |
| no `core.ein` | *kb load error: std.core, the module every program gets, cannot be read: (import std.core) — module not found at …/core.ein (None)* |
| a `core.ein` and no `MANIFEST.sha256` | *… $EIN_STDLIB names …, which has no MANIFEST.sha256 — a directory is the stdlib only if it carries the marker …* |

1. **The error where the library `$EIN_STDLIB` names has no `std.core`** says so in those words: that `std.core`
   was not found in `$EIN_STDLIB`, with the directory.
2. **Reading 3**, default: the manifest stays what identifies a directory as the library, so the third row stays
   a refusal, with its own words; the other reading takes any directory that has a `core.ein`.
3. The page of `std.core`, the install page's section on `$EIN_STDLIB` and the configuration reference say that
   the module is replaced this way, with a program run under a library whose `std.core` is another.

## Not built: backward compatibility

The user: *"There is still only a single user of ein - me, so I don't care about backward compatibility. Forget
about it until I explicitely ask. Containers not load - delete and create new."* A saved file or a container of an
older format does not load, and nothing is added to read one. It is a rule of the phase from here on
([the README](README.md#rules-of-the-phase)).

## What must hold

- **Every answer** of a program that loaded before: the corpus sweep, each mode against the oracle.
- A program task 1 newly loads is shown with its answer and what warns, and nothing is claimed of it past what
  the run shows.
- A refusal that goes takes its section of
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) with it, in the same commit; the
  standing rules of `AGENTS.md` that state either refusal are rewritten in place.

## Acceptance

`(import std.core)` loads; a library with no `core.ein` under `$EIN_STDLIB` gives the error in the user's words;
and, once the reading is the user's, the five programs of task 1.1 load under a warning. The gate on the stage's
last commit.

## Record

Nothing yet.
