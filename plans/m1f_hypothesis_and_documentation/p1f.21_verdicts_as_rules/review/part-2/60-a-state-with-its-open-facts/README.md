# 60 · A state with its `open` facts: one program of each writer

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.3 of the stage. You ruled that a state's `open` facts are part of it, listed, drawn and
counted with its facts (Q-M1f.572 (a)), and that `-P` leaves them out (Q-M1f.577). Here, for each of the three
writers of an `(open F)`, a rule, an obligation and the blind enumerator, a state at root and one above it: the
`open` facts the pass would write there, and what `ein solve -p`, a dump and a drawing of the state would then show.
[On the built engine](#on-the-built-engine), at the foot of the page, reads the same seven states off the final
binary, beside what is composed here.

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

## On the built engine

**How it was taken.** The final binary, the release build of `7b72bd84` (`build/review-bin/ein-final-7b72bd84`),
with the checkout's `stdlib/`; its `--version` is `runs/built/version.txt`. `tools/run-built.sh` re-takes every run
into `runs/built/` (ignored by git; the stand-in's `runs/` stay as they were), and `tools/states-built.py` reads the
seven states (`runs/built/states.md`, every file under `runs/built/states/`). The programs, in the spelling the
built engine reads:

- **a rule's**: [`drinks.built.ein`](programs/drinks.built.ein), `guess` concluding `(open (?R ?x))` and activated
  by two facts. It is `drinks.ein` built, and `drinks-asked.ein` built as well (`question` read as `open`, its
  `ask` gone, since the kernel supposes each `(open F)` root holds itself), so one file stands for both. The two
  stand-ins are refused at the parse (`runs/built/drinks.solve.txt`, `runs/built/drinks-asked.solve.txt`);
- **an obligation's**: [`05_owed_seats_bounded.ein`](../../../../../../examples/lattice/05_owed_seats_bounded.ein)
  as it is. Item 62's stand-in, built, is that program: its rules concluded under `question` what the pass now
  writes;
- **the blind enumerator's**: [`blind.ein`](programs/blind.ein) as it is: it writes no `hrule`, `:hrules` or
  `question`, so it is its own built spelling.

For each: the read-out, `-p`, `-p --in-flight`, `-P`, `ein show hyps`, `ein saturate --dump`, the oracle with every
node's state and `-p`, with and without `--in-flight`, the events and the summary, `--dump-lattice` and
`--dump-answer` (with `-p`, so every state's block is kept uncut), and the knowledge base, the lattice (as `ein
solve` visits it, and in full) and the region drawn.

**Each state read three ways** (`runs/built/states.md`): the block `ein solve -p` prints for it, where it is a state
of $R$; the oracle's `--states` record of the node; and, as the stand-in's tool did, the state **made a root**, its
commitment written into the program as facts, now with the real `open` (`ein saturate --dump`, `ein show hyps`,
`ein render kb`).

| writer | state | the page (composed on edbe6c3b) | `-p` on the built engine | made a root, built |
|---|---|---|---|---|
| a rule | ∅, root | 4 `open` facts | state 1 of 12: the same 4, and nothing else (`runs/built/drinks.built.solve-p.txt`) | the same 4 (`runs/built/states/rule-0.root.saturate-dump.txt`) |
| | {`(chosen Tea)`, `(served Milk)`} | the same 4 | state 8 of 12, 6 facts: the same 4 beside the two (`runs/built/drinks.built.solve-p.txt`) | the same 4 (`runs/built/states/rule-1.root.saturate-dump.txt`) |
| an obligation | ∅, root | 5: `(open seats)` and 4 | not printed: root owes 4 instances and is not in $R$ (`runs/built/seats.solve.txt`); root's dump holds the 5 (`runs/built/seats.saturate-dump.txt`) | it is the program itself |
| | {`(seats Ann C1)`}, owes 2, outside $R$ | 4: `(open seats)` `(open (seats Ann C2))` `(open (seats Bob C1))` `(open (seats Bob C2))` | not printed: outside $R$ | the same 4 (`runs/built/states/obligation-1.root.saturate-dump.txt`) |
| | {`(seats Ann C1)`, `(seats Bob C2)`}, a state of $R$ | none | state 1 of 2: none (`runs/built/seats.solve-p.txt`) | none (`runs/built/states/obligation-2.root.saturate-dump.txt`) |
| the blind enumerator | ∅, root | 6 | state 1 of 64: the same 6 (`runs/built/blind.solve-p.txt`) | the same 6 (`runs/built/states/blind-0.root.saturate-dump.txt`) |
| | {`(likes Ann Soup)`} | 5: the six but `(open (likes Ann Soup))` | state 5 of 64: the same 5 (`runs/built/blind.dump-answer/forms/print-final-state.txt`; stdout's 100 lines end after state 1) | the same 5 (`runs/built/states/blind-1.root.saturate-dump.txt`) |

Every `open` fact the page composed is the built engine's, at each of the seven states.

**Which states print their `open` facts, and which do not.**

- **Every state of $R$ prints them**, under `ein solve -p` (and `ein oracle -p`): the drinks' 12 each list the
  rule's 4, the seats' 2 list none, owing nothing, and the blind enumerator's 64 each list the guesses still
  undecided there. Root is printed among them where it is a state of $R$ (the drinks, the blind enumerator), so
  root's `open` facts are in its block; `-P` leaves them out (`runs/built/drinks.built.solve-P.txt`: state 8 is
  `(chosen Tea)` `(served Milk)`, 2 facts), as Q-M1f.577 says.
- **A state outside $R$ prints none, on any command.** The seats' root owes, so `-p` does not print it; its `open`
  facts are read off `ein saturate --dump`, `ein show hyps` and `ein render kb`, which are root's. The node
  {`(seats Ann C1)`} is printed by nothing: the oracle's `--states` keeps a node's facts *beyond root's*, and every
  `open` fact that node holds is one root holds too, so its record is `(seats Ann C1)` alone
  (`runs/built/states/obligation-1.oracle-node.json`); § 3's second block, *an oracle's dump of the node would*,
  is not what the dump shows. Only the state made a root shows the four. That record lists a node's `open` facts
  only where root lacks them, so it cannot show the one root holds and the node has dropped, `(open (seats Ann
  C1))`, though Q-M1f.572 has every listing of a state list its `open` facts (a finding of this stage, in its
  record).
- **The oracle's `--states` lists no `open` fact at any of the seven**, for the same reason: here every `open` fact
  a state holds is root's too (`runs/built/states/*.oracle-node.json`).
- **`--in-flight` shows nothing more.** It writes every state's `open` facts, but a state nothing prints is still
  printed by nothing: `-p`, the oracle's read-out and its `region.json` are the same with and without it, the
  command line apart, for all three programs (`runs/built/*.in-flight.diff.txt`).

**§ 3's three blocks, as `-p` prints them.** The drinks' state is § 3's block line for line but in the order
`-p` sorts a state's facts, the commitment no longer first: `(chosen Tea)`, the four `(open …)`, `(served Milk)`,
6 facts. The blind enumerator's is § 3's block exactly, 6 facts. The seats' is not printed (above). The seats'
state of $R$ prints 6 facts where the stand-in's `-p` printed 2 (`runs/seats.solve-p.txt`): the two seats and four
`std.contradiction.clash-…` facts, root's derivations, the library's contradiction rules activated for the two
relations the program declares; none is an `open` fact.

**§ 4, a dump and a drawing, built.** The state made a root counts its `open` facts by relation, `open` 4
(`runs/built/states/rule-1.root.saturate-dump.txt`), where the stand-in counted `question` 4. Drawn from the
state's saved file (`ein render kb` of `--dump-answer`'s `models/08.ein`, `runs/built/states/rule-1.state.kb.svg`):
the four `open` facts, each *by guess*; a dashed frame labelled `open ctx` holding the two facts asked about that
the state does not hold, `(chosen Milk)` and `(served Tea)`; and `(chosen Tea)` and `(served Milk)` outside it,
each under an arrow from its `open` fact, drawn dashed as a fact beyond the program is. That is § 4's drawing with
`open` for `question`. The seats' state of $R$ draws no `open` fact and no frame
(`runs/built/states/obligation-2.state.kb.svg`); the blind enumerator's draws 5 and a frame of 5
(`runs/built/states/blind-1.state.kb.svg`); root's drawings, `runs/built/*.kb.svg`, draw root's 4, 5 and 6.

| | the stand-in, on edbe6c3b | the built engine | why they differ |
|---|---|---|---|
| the drinks: states of $R$ | 12 (`runs/drinks.solve.txt`) | 12 (`runs/built/drinks.built.solve.txt`) | — |
| the seats: states of $R$; root | 2; owes 4 (`runs/seats.solve.txt`) | 2; owes 4 (`runs/built/seats.solve.txt`) | — |
| the blind enumerator: states of $R$; candidates | 64; 6 (`runs/blind.solve.txt`) | 64; 6, $H$ from `blind` (`runs/built/blind.show-hyps.txt`) | — |
| `open` facts at the seven states | composed (`runs/states.md`) | the same at each (`runs/built/states.md`) | — |
| `-p` of the seats' node {`(seats Ann C1)`} | composed: 5 facts, *an oracle's dump of the node would* | printed by no command; the oracle's record is `(seats Ann C1)` alone | the oracle keeps a node's facts beyond root's, and root holds the four |
| `-p` of the seats' state of $R$ | 2 facts | 6 facts, none `open` | the library's contradiction rules, activated at root, not this page's |
