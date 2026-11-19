# S1f.21.10 — the engine's key, `:ein inconsistent`

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 as *the kernel's prefix*; **re-planned the same day on the user's two rulings and
unblocked on the user's word** (*"Yes, :ein inconsistent"*, *"unblock S1f.21.10"*). **Not started**: it waits on
no decision, and starts on the user's word. The file keeps its first name; a stage's id and file are not moved.
**Group:** [F](modifications.md#f-q-m1f399-a-word-of-the-language-renamed), in its ruled form, with items 24 and 25
of [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), which the
rulings answer.
**Waits on:** no ruling. [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open)
and [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein)
are ruled. **In order, it runs after [S1f.21.6](s1f.21.6_surfaces.md) and
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md) are merged**: it moves the read-out's and the dump's
goldens the first re-banks, and the standard library's manifest the second rewrites.
**Writes:** the grammar (`ein-ir/src/ein.l`, `ein.y`, and the grammar page's generated regions) and the loader;
`ein-core`'s rule; `ein-infer/src/` (`saturator.rs`, `contradiction.rs`, `compile.rs`, `explain.rs`, `events.rs`,
`forbid.rs`, `monotone.rs`); `stdlib/`, one module; the pages that state a rule's form and a state's death;
`AGENTS.md` § The language; the decisions of the two rulings; the goldens.

## The rulings it builds

The user, 2026-11-19, on Q-M1f.399: *"no need to reserver `ein.` prefix; make (false) an stdlib convention; add
rules detecting contradictions infereing (false) and then - we can make another key, not :assert, but special
engine control key `:ein false` which argument is not a fact, it can remain even the same word `false` but see
plans/m1g_logic/p1g.40_proof_theory/ideas2.md and maybe propose better name here"*; and on the word proposed,
Q-M1f.550: *"Yes, :ein inconsistent"*.

So: **a rule states what it concludes under `:assert`, facts, or under `:ein`, a word addressed to the engine.**
The word is no fact: no knowledge base holds it, no rule matches it, it stands under no arrow. There is one word,
`inconsistent`: where a rule that states it fires, the saturation ends and the state is dead.

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

`(false)` is then an ordinary proposition of the standard library's, inferred by rules, held or under an arrow,
and that one rule is what makes a state that holds it dead. No word of the language is renamed and no prefix is
reserved (items 21 to 23 are not built).

## What it builds

1. **The key.** The grammar's rule takes `:ein` *word* where it takes `:assert`. The loader holds the word to the
   engine's words, of which there is one, and refuses another with a line that lists them, so a misspelt word does
   not load in silence.
2. **What the engine does with it.** A firing of a rule that states `:ein inconsistent` ends the node's saturation
   and makes the state dead, and writes no fact. The state's core and its `refuted by` are taken from that firing's
   premises.
3. **The rule from `(false)`**, in a module of the standard library every program gets. After it the kernel reads
   `(false)` by its name nowhere to judge a state: `contradiction.rs`' direct shape goes. Its other shape, a fact
   beside its stored negation, stays the kernel's until [S1f.21.11](s1f.21.11_kill_and_three_conventions.md).
4. **What read *leads to `(false)`* reads *leads to a rule with the key***: the agenda's first half
   ([Q-M1f.313](../../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)), *forbid*
   and the static check, each through the module's rule, so that each decides what it decides today.
5. **The surfaces**: `ein saturate --dump` lists the module's rule as any rule; a `fire` event of a rule with the
   key says so; the trace shows the firing that ended the state.

## The readings, each with the default the stage takes

The syntax is one key on one rule of the standard library, shown above and accepted; no corpus program is
rewritten. So the stage does not stop for an examples look: it takes these defaults, records each, and they are
shown at its end. **It stops only where a default cannot be built, or an answer or a count below moves.**

1. **`:assert` or `:ein`, never both** on one rule: the answer says *"another key, not :assert"*.
2. **The module every program gets**: a module of the standard library, `std.core`, loaded into every program with
   no import line, its rule shown and counted as any rule, not replaceable in this phase. **This is the one default
   that adds a notion to the language**, a module no file imports; it is taken because the other reading, a program
   imports the convention, leaves every program that writes `(false)` and imports nothing unable to refute:
   `(false)` is on 692 lines of 284 files. Shown to the user at the stage's end as the reading to look at first.
3. **A program's own rule may state the key**, from whatever it matches, with no `(false)` between; *forbid* reads
   such a rule as it reads one that concludes `(false)` today.
4. **`refuted by` names the program's rules**: the module's rule is left out of an explanation, so no read-out row
   moves. A core is the frontier of the firing's premises, which is the frontier of `(false)` as today.
5. **A rule with the key fires before any other the agenda holds**, so a route's node stops where it stops today:
   after the firing that concluded `(false)`, with one firing more, the key's.
6. **`:expect (false)`**, a claim's word for an empty $R$, and the grammar's reading of `false` are not this
   stage's: the reserved words are S1f.21.11's (item 29).

## What must hold

- **Every answer**: $R$ of every corpus program by the corpus sweep, each mode against the oracle
  (`ein-infer/tests/explore.rs`), `ein-cli/tests/lever_invariance.rs`, `import_invariance.rs`,
  `ein-infer/tests/agenda.rs`, `first_contradiction.rs`.
- **Every count a read-out prints**: `work`'s saturations and facts beyond root, the cores, `refuted by`. A
  read-out line of the shape golden that moves is a finding, shown, not re-banked in silence. What may move: the
  firing streams (one firing more a dead node), the saturated dump's rule lists, the events.
- **The register**: every open row answers as it does (`python3 utils/wrong_answers.py`), and
  `corpus/regressions.toml` stays right.
- **A state dead by a pair** answers as today, untouched.
- The grammar page's regions and its diagrams are written by their generator (`EIN_BLESS=1`), the new shape and
  the two refusals have their rows in its table (`ein-ir/tests/grammar_pages.rs`) and their sections of
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2.
- The module is a file of `stdlib/`: its program under `tests/stdlib/`, its region on a rules page, the manifest.
- The standing rules it rewrites, in place and in the same commits: `AGENTS.md` § The language (*a rule concludes
  a conjunction of literals*, *the agenda fires first what leads to `(false)`*), Q-M1f.313's decision; and the two
  rulings as decisions under `docs/decisions/`.
- The `.einb` container holds a rule's key as it holds its conclusion; `ein solve x.einb` is `ein solve x.ein`.
- A large change to the engine: the gate and the nightly on its last commit; `utils/ci_container.sh` if a DOT
  writer moves (a rule's drawing gains the key).

## Acceptance

`grep` finds no read of `false` by name in a decision about a state in `ein-infer/src/`; the readers that are
left are named (the grammar, a claim, the pair's other half); a file whose rule states `:ein inconsistent` over its
own premises is dead where they hold and alive where they do not; a file that writes `:ein dead` is refused with
the engine's words listed.

## Record

Nothing yet.
