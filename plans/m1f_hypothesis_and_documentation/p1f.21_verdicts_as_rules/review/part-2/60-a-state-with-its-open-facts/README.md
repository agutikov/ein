# 60 · A state with its `open` facts: one program of each writer

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.3 of the stage. You ruled that a state's `open` facts are part of it, listed, drawn and
counted with its facts (Q-M1f.572 (a)), and that `-P` leaves them out (Q-M1f.577). Here, for each of the three
writers of an `(open F)`, a rule, an obligation and the blind enumerator, a state at root and one above it: the
`open` facts the pass would write there, and what `ein solve -p`, a dump and a drawing of the state would then show.

**The answer, short.** The three writers fill a state differently. **A rule** writes at every state what its match
gives there: in the drinks program the same four at every state, the facts the state supposes among them, so a state
that holds `(chosen Tea)` also holds `(open (chosen Tea))`. **An obligation** writes only where something is owed:
`(open seats)` and the facts that would pay what is owed there, and **nothing in a state of $R$**. **The blind
enumerator** writes every fact still undecided at that state: six at root, five once one of them is supposed.

## 1. How it was taken

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes every
run into `runs/` (ignored by git; mirrored under
`build/review-incoming/p1f21-s12-examples/60-a-state-with-its-open-facts/`); `runs/states.md` is the table below as
the tool wrote it, and `runs/states/` has every file of every state.

**A state is made a root.** For each state the commitment is written into the program as facts, so the state is
the root of a run of its own, and what is written there is read at that state's own fixpoint, which is where the
pass writes, a fork having dropped what root wrote. Then:

- **a rule's**, [the drinks](programs/drinks.ein): its stand-in [`drinks-asked.ein`](programs/drinks-asked.ein),
  where `guess` concludes `(question F)` for `(open F)`, saturated at the state (`ein saturate --dump`);
- **an obligation's**, [the seats](../../../../../../examples/lattice/05_owed_seats_bounded.ein): the stand-in of
  [item 62](../62-what-a-fork-drops/README.md), whose rules conclude `(question seats)` where an instance is owed
  and `(question (seats ?g ?c))` for what would pay it, saturated at the state;
- **the blind enumerator's**, [one person, one food](programs/blind.ein): `ein show hyps` at the state, every
  fact it guesses there.

**What is shown, and what is composed.** `runs/states/W-K.shown.ein` is the state's program with those facts written
in under `question`; its `ein saturate --dump` and `ein render kb` (`.shown.saturate-dump.txt`, `.shown.kb.svg`) are
today's engine on it: what a dump and a drawing of the state would hold, `question` standing for `open`. The `-p`
blocks below are **composed** (`runs/states/W-K.p.txt`): today's block for the state, read off its own run, with the
`open` facts added and the count moved. No binary prints them.

## 2. The states

| writer | state | `-p` lists today | the `open` facts the pass would write there |
|---|---|---|---|
| a rule | ∅, root | nothing | 4: `(open (chosen Milk))` `(open (chosen Tea))` `(open (served Milk))` `(open (served Tea))` |
| | {`(chosen Tea)`, `(served Milk)`}, a state of $R$ | `(chosen Tea)` `(served Milk)` | the same 4, two of them of facts the state holds |
| an obligation | ∅, root | nothing | 5: `(open seats)` and `(open (seats Ann C1))` `(open (seats Ann C2))` `(open (seats Bob C1))` `(open (seats Bob C2))` |
| | {`(seats Ann C1)`}, owes 2, outside $R$ | `(seats Ann C1)` | 4: `(open seats)` and `(open (seats Ann C2))` `(open (seats Bob C1))` `(open (seats Bob C2))` |
| | {`(seats Ann C1)`, `(seats Bob C2)`}, a state of $R$ | `(seats Ann C1)` `(seats Bob C2)` | none |
| the blind enumerator | ∅, root | nothing | 6: `(open (food Ann))` `(open (knows Ann Soup))` `(open (knows Soup Ann))` `(open (likes Ann Soup))` `(open (likes Soup Ann))` `(open (person Soup))` |
| | {`(likes Ann Soup)`}, a state of $R$ | `(likes Ann Soup)` | 5: the six but `(open (likes Ann Soup))` |

So a state of $R$ holds no `(open R)`, which is your ruling read on a state that owes nothing, and a rule's `(open F)`
stands beside F once F is supposed ([the page of Q-M1f.573](../../../q-m1f.573_obligation_or_hypothesis/README.md),
§ 2: *a state that supposes it holds both*). The obligation's and the blind enumerator's never name a fact the state
has decided, by what each writes: the payments of what is owed, and the facts still undecided.

## 3. `-p`, composed

The drinks' state {`(chosen Tea)`, `(served Milk)`}:

<!-- output -->
```text
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 6 facts):
  (chosen Tea)
  (served Milk)
  (open (chosen Milk))
  (open (chosen Tea))
  (open (served Milk))
  (open (served Tea))
```

The seats' node {`(seats Ann C1)`}, which `-p` does not print, being outside $R$; an oracle's dump of the node would:

<!-- output -->
```text
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 5 facts):
  (seats Ann C1)
  (open (seats Ann C2))
  (open (seats Bob C1))
  (open (seats Bob C2))
  (open seats)
```

The blind enumerator's state {`(likes Ann Soup)`}:

<!-- output -->
```text
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 6 facts):
  (likes Ann Soup)
  (open (food Ann))
  (open (knows Ann Soup))
  (open (knows Soup Ann))
  (open (likes Soup Ann))
  (open (person Soup))
```

`-P` prints each block without its `(open …)` lines, as it prints none of the `(not …)` (Q-M1f.577).

## 4. A dump and a drawing

`runs/states/rule-1.shown.saturate-dump.txt` lists the four `question` facts among the state's facts, and its counts
by relation count them (`question` 4). The drawing, `runs/states/rule-1.shown.kb.svg`, draws each as a fact whose
first arrow is at `question` and whose second is at the fact asked about. The two facts asked about that the state
does not hold, `(chosen Milk)` and `(served Tea)`, stand in a dashed box, the context of `question`; the two it
holds, `(chosen Tea)` and `(served Milk)`, stand outside it, held, each under an arrow from its `question` fact. Once
built the box is `open`'s and the facts are the pass's; the drawing is otherwise the same. The seats'
and the blind enumerator's states are drawn the same way (`obligation-*.shown.kb.svg`, `blind-*.shown.kb.svg`).

## 5. Your call

Nothing new is asked: this is your rulings Q-M1f.572 and Q-M1f.577 shown on a state of each writer.

- **(a)** As shown. **Default.**
- **(b)** Something here is not what you meant: say which.
