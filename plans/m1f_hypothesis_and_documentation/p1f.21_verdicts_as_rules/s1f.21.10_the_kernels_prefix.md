# S1f.21.10 — the kernel's prefix

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [F](modifications.md#f-q-m1f399-a-word-of-the-language-renamed), items 21 to 23.
**Waits on:** [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open),
the names. Planned on the user's directions, (a) with (d): the word `false` stays, and what the kernel reads goes
under one prefix.
**Writes:** the loader (`ein-ir/src/from_ir.rs`) and `ein-core/src/terms.rs`; the grammar page's table and
[`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2; their tests.

## What it does

**No word is renamed.** Items 21 and 22, a new word for `(false)`, are not run on the user's directions, and
every program that writes `(false)` stays as written. The stage is item 23: the prefix `ein.` becomes the kernel's.

1. **The census**, re-taken: no program under `examples/`, `tests/` or `stdlib/` writes a name under `ein.`
   (none on 2026-11-19, `grep -rhoE '\(ein\.[a-z.-]+' --include='*.ein' examples tests stdlib`).
2. **The prefix is reserved.** No declarator binds a name under it: a relation, a rule, an hrule, a macro, a
   macro's parameter, an import's `:as`. A refusal at load, on every command, with its section of the diagnostics
   page and its row of the grammar page's table.
3. **A name under the prefix that the kernel does not define is refused**, where today `(ein.kill)` loads as an
   ordinary fact of an undeclared relation and kills nothing: a misspelt verb must not load in silence.
   **Reading 1**, default: refused at load, with the kernel's names listed. The names themselves are defined by
   the stages that give them a meaning, [S1f.21.11](s1f.21.11_kill_and_three_conventions.md) and
   [S1f.21.12](s1f.21.12_hypotheses_as_facts.md); until one does, every name under the prefix is refused.
4. **How a program writes them.** **Reading 2**, default: in full, `(ein.kill)`, with no import line and no short
   name; they are few, and a full name says at sight that the kernel reads it. The other reading: an import form.

## The names it puts, with Q-M1f.399

The stage builds no name; it puts them, each with a default, for the two stages that do:

| what | the names | default |
|---|---|---|
| the fact that ends a saturation | `(ein.kill)`, `(ein.die)`, `(ein.saturation.kill)`, `(ein.contradiction)` | `(ein.kill)`, the leaning of [`open_facts.md` § 3](open_facts.md#3-who-kills-a-dead-node-einkill) |
| a candidate, under [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules) (b) | `(ein.ask F)`, `(ein.suppose F)`, `(ein.candidate F)`, or `(open F)` as the user wrote it | `(ein.ask F)`: the fact says *asked*, and stays true once F is decided, which *open* does not |
| an obligation's verdict | `(open ?R)` as today, or under the prefix | as today: four rules of the standard library and every program's own write it |
| a relation no commitment extends | `__closed__` as today, or `ein.closed` | as today in this stage; S1f.21.12's to move |

## What must hold

- No corpus program is refused that loaded before; the corpus sweep.
- `ein-render/tests/renaming_invariance.rs` renames no name under the prefix, as it renames no kernel name today.
- A shape the grammar newly refuses has its row in the grammar page's table (`ein-ir/tests/grammar_pages.rs`).

## Acceptance

A file that writes `(ein.kil)` or declares `(relation ein.x A …)` is refused with one line that names the prefix;
the diagnostics page has the section; one gate.

## Record

Nothing yet.
