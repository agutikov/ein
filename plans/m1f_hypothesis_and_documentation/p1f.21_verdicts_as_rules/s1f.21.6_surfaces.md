# S1f.21.6 — the surfaces

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [A](modifications.md#a-surfaces-no-answer-and-no-word-of-the-language-moves), items 1 to 5.
**Waits on:** nothing. It is the first stage to run.
**Writes:** the printers and their strings under `ein.rs/`; the pages that describe them,
[`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md),
[`09-read-out-examples.md`](../../../docs/ein/user/04-reference/09-read-out-examples.md) (written by the binary) and
[`10-answer-dump.md`](../../../docs/ein/user/04-reference/10-answer-dump.md); the goldens that hold a line it
moves; for item 5, this folder's own pages.

## What it does

No answer and no word of the language moves: each task changes what a surface prints, or a number in the plan.

1. **`-p` prints a fact of no argument as it is written** ([V-3](findings.md#v-3---p-prints-a-fact-of-no-argument-with-a-space-inside)).
   `ein solve -p` and `-P` print `(alive )` where the same run's `stale` row, `ein saturate --dump` and `ein oracle
   --dump --states` print `(alive)`. One printer for a fact, and a test that the surfaces agree on a fact of no
   argument.
2. **The saturated dump lists every kind of rule** ([V-4](findings.md#v-4--the-saturated-dump-lists-no-obligation-rule)).
   `;; Rules (N)` is the agenda's rules; an obligation rule and a hypothesis rule are in no section. **Reading 1**,
   default: a section each, `;; Obligation rules (N)` and `;; Hypothesis rules (N)`, in the dump's form for a rule,
   and the first heading says it is the agenda's. The other reading: the heading's words alone.
3. **A dead state by a pair is not said to derive `(false)`** ([V-2](findings.md#v-2--the-surfaces-say-derives-false-of-a-state-that-holds-none)).
   Four places: the read-out's `empty` row, `10-answer-dump.md`'s `root.dead` row, the legend of `ein render
   region`, the comment on `DEAD` in `ein-infer/src/region.rs`. **Reading 2**, default: *holds a contradiction*,
   true of a held `(false)` and of a pair today, and of a held `(ein.kill)` after
   [S1f.21.11](s1f.21.11_kill_and_three_conventions.md). The other reading: left until that stage makes the words
   literally true.
4. **An owing node below a non-empty $R$ has no saved file** ([item 32 § 7](review/32-Q-M1f.390/README.md#7-found-while-building)).
   Shown on a program and put. **Reading 3**, default: not built, and entered in [`findings.md`](findings.md) as a
   finding of its own; the other reading: `--dump-answer` writes the owing frontier whether $R$ is empty or not.
5. **The plan's own numbers**, no project file: the five corrections
   [`modifications.md`](modifications.md#a-surfaces-no-answer-and-no-word-of-the-language-moves) lists under its
   item 5, each in the page it is of, with the command that re-takes it.

## What must hold

- **No answer moves**: no field of `region` in any corpus program's summary, by the corpus sweep through the binary.
- A change to what `ein solve` prints changes `08-read-out.md` in the same commit, and the examples page is written
  again by the binary (`ein-cli/tests/read_out_reference.rs`).
- A golden that holds a moved line is re-banked and its diff read: only the lines of tasks 1 to 3.
- `ein render region`'s legend is a DOT writer's: `utils/ci_container.sh` is part of done.

## Acceptance

Each of V-2, V-3 and V-4 no longer shows on the run its finding gives, and the finding says so; the three readings
are recorded with what was taken; one gate on the stage's last commit.

## Record

Nothing yet.
