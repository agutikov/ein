# Part 2 — `open`: what a state owes, and the hypotheses

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

**Four questions are open, 52 to 55: what your answers to items 47 to 51 leave.** Each is a folder with its page,
its programs and their runs, each has a default, and each has its section in [`ANSWERS.md`](ANSWERS.md). The
second part's two builds are planned on your rulings and can be built on these defaults.

## What is left for you

| # | question | what it asks | the defaults |
|---|---|---|---|
| [**52**](52-Q-M1f.574/README.md) | Q-M1f.574 | a hypothesis first held above root: how the program is found not monotone. Your *"right?"* is answered there: by your definition yes, and today's static check calls such a program monotone | both a site of the static check and a check in flight; an answer found so in flight stands under the warning |
| [**53**](53-Q-M1f.575/README.md) | Q-M1f.575 | what one `:assert` holds beside `(open R)` and beside `(open F)`. Your *"cant"* reads as *can* and as *can't* | `(open R)` is a rule's whole `:assert`, as today; `(open F)` stands in a conjunction like any fact |
| [**54**](54-Q-M1f.576/README.md) | Q-M1f.576 | the union's edges: what an obligation whose candidates cannot be read, and the blind enumerator, do beside a rule | the obligation writes nothing and calls no one; the blind enumerator writes only where no rule and no obligation does. Three corpus programs keep their 19, 2 and 3 candidates, against 923, 30 and 57 |
| [**55**](55-Q-M1f.577/README.md) | Q-M1f.577 | a view of a state without its `open` facts | `-P` leaves them out as it leaves out the `(not …)` |

Write each answer in [`ANSWERS.md`](ANSWERS.md); an item left blank keeps its defaults.

## The stages your rulings plan

Neither is started; each starts on your word, after part 1.

| stage | what it builds |
|---|---|
| [S1f.21.12](../../s1f.21.12_hypotheses_as_facts.md) | `open` facts: `(open R)` held where a state owes, `(open F)` a hypothesis any rule may conclude, written at each state's fixpoint and dropped at a fork; $H$ root's, the union; a rule that reads one makes its program not monotone; the nesting rule; the `open` facts part of a state. Its examples first, on the corpus, the programs whose answer the union moves among them |
| [S1f.21.18](../../s1f.21.18_the_hrule_form_removed.md) | the `hrule` form and the keyword `:hrules` removed: 119 program files written again, each answering as before, then the form deleted |

## Ruled, and out of the bundle

Each answer is in your words in [the phase's README](../../README.md) and under its question in the ledger. Each
item's page and programs are kept in the phase's folder.

| item | question | ruled | its page |
|---|---|---|---|
| 32 | Q-M1f.390 | an obligation emits `(open H)`, no justification in it; `absent` reads it as any fact | [`q-m1f.390_…`](../../q-m1f.390_what_an_obligation_emits/README.md) |
| 34 | Q-M1f.549 | `(open …)` is the context for hypotheses, *may*; an obligation is *must* | [`q-m1f.549_…`](../../q-m1f.549_hypotheses_as_facts/README.md) |
| 39 | Q-M1f.560 | (b): `std.debts` is merged. **Done**: `edbe6c3b` | in `git log` |
| 44 | Q-M1f.566 | the obligation stays as it is; `(open R)` is held as a fact and counted as before; `(open F)` over a fact is a hypothesis; the bare `(open)` is refused; a fork drops every `open` fact | [`q-m1f.566_…`](../../q-m1f.566_how_the_must_is_emitted/README.md) |
| 45 | Q-M1f.567 | the `hrule` form and the keyword `:hrules` go; an activator is a fact the program writes | [`q-m1f.567_…`](../../q-m1f.567_the_hrule_form_dropped/README.md) |
| 46 | Q-M1f.568 | a written `(open P)`, `(not (open P))`, `(open (not P))` legal; `(absent (absent P))`, `(open (open P))`, `(open (absent P))` refused | [`q-m1f.568_…`](../../q-m1f.568_absent_open_and_not_nested/README.md) |
| 47 | Q-M1f.569 | a rule may read an `open` fact, and its program is then not monotone automatically; a written `(open P)` is at root alone; every rule that concludes `(open …)` is read at every state, $H$ root's | [`q-m1f.569_…`](../../q-m1f.569_what_may_read_an_open_fact/README.md) |
| 48 | Q-M1f.570 | (b), the union: $H$ is every `(open F)` root holds | [`q-m1f.570_…`](../../q-m1f.570_override_or_union/README.md) |
| 49 | Q-M1f.571 | every default: `absent` takes a premise, the universal stays; under an ordinary relation both are ordinary names | [`q-m1f.571_…`](../../q-m1f.571_what_absent_and_open_take/README.md) |
| 50 | Q-M1f.572 | (a): the `open` facts are part of a state | [`q-m1f.572_…`](../../q-m1f.572_open_facts_shown_with_a_state/README.md) |
| 51 | Q-M1f.573 | the defaults: read at load by what stands under `open`; a decided hypothesis is no candidate; $H$ root's | [`q-m1f.573_…`](../../q-m1f.573_obligation_or_hypothesis/README.md) |

**The binary.** Every item runs on `build/review-bin/ein-head-edbe6c3b`, the gate build of master at `edbe6c3b`,
with the checkout's `stdlib/`; each item's `tools/run.sh` re-takes its `runs/`.
