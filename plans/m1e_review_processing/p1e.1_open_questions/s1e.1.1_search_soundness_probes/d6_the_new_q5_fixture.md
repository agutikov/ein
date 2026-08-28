# D6 — Decided: `examples/branching/15` + `16`, stating today's answer

> **Decided 2026-08-28, as recommended.** The fixture is a **pair** —
> `examples/branching/15_lookahead_two_step_on.ein` and
> `16_lookahead_two_step_off.ein`, differing in one `(config …)` line the way
> `06`/`07` do — each carrying an `:expect` that states **what the engine
> answers today**, so the stage that fixes
> [Q-M1e.8](../../open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)
> has to move the golden. Both goldens grow; nothing existing moves.
>
> **A pair rather than one file, and that is now forced rather than
> preferred.** An `:expect` is one claim per `(query …)`, and the point of this
> fixture is that the two configurations *answer differently* — so one file
> could state only one of the two answers, and the other side would go
> unpinned. `06`/`07` split for the same reason.
>
> **And the target the moved golden will state is now known.**
> [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
> was ruled on the same day and selects fix **(ii)**, re-saturate and
> re-check: under it every record-site probe answers what `-L` answers. So the
> `:expect` written today is a claim with a dated successor, not an open bet.

**Touches:** [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
step 3.

## Why a new fixture at all

The stage's Q5 pair was `lattice/02` and `branching/06`. Reconnaissance
removed the second:

- **42 varying slots**, smallest determining set 8, `C(42,8) = 118 030 185` —
  `--models key` declines and prints the models instead;
- 20 of its 22 models bind `?h` to `Color` or `House`, because the blind rung
  is untyped and `(is-a Color T)` makes the type an object
  ([D8](d8_branching06_untyped_models.md));
- **neither side exhausts** — both hit the depth-5 cap, so k=22 against k=0 is
  two lower bounds rather than two answers, and the comparison terminates in
  [Q-M1d.6](../../../../docs/history/m1d_satisfiability/open_questions.md)
  which this stage is told not to touch.

`lattice/02` is decisive on its own — both sides exhaust — but it is a
*three-way* conflict, so it exercises the lookahead only incidentally. The new
fixture is the one that isolates the lever with both sides complete.

## What it has to satisfy

1. **Both configurations exhaust** at the default `-m 5`. This is the
   requirement `branching/06` fails.
2. **The verdicts differ** between lookahead on and off.
3. **Its solution set is derivable in a paragraph**, from the program text,
   under [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model).
4. **Typed**, so no model contains a type atom as an argument.
5. It carries an `:expect` stating the correct side.

A shape that satisfies all five: two or three hypotheses, a refutation rule
that needs **two firings** to reach `(false)` for one candidate, so the
one-step lookahead cannot see it and `complete` under-reports on **both**
sides — which is the finding, made visible in one file.

## Where it goes

`examples/branching/`, beside `06_lookahead_on.ein` and
`07_lookahead_off.ein`, which are exactly this A/B and whose headers already
carry the measurement. `examples/` is *things to read*, and this is a file
whose header explains a semantics question — the same job `06`'s does.

**Not** `tests/stdlib/` (that directory is one program per stdlib rule) and
**not** `examples/broken/` (the program loads and solves fine; what is at
issue is the answer).

Naming, following the directory's convention:
**`15_lookahead_two_step_on.ein`** and **`16_lookahead_two_step_off.ein`** —
`14_lookahead_unjudgeable.ein` is the current last.

Two files rather than one, decided: the pair *is* the point, an `:expect` is
one claim per query, and the two configurations answer differently — so a
single file leaves one side of the comparison unpinned. Two entries, one
`(config …)` line apart, with a header that diffs them, exactly as `06`/`07`
do.

## What it pins, and what moves

| artefact | change |
|---|---|
| `corpus/corpus.toml` | one new `[[entry]]` (or two), with `runs` and, if either run costs ≥ 1 s together, a measured `cost_ms` — the [completeness check](../../../../corpus/README.md) fails without it |
| `examples/README.md` | one catalogue line per file |
| `ein-cli/tests/golden/corpus_exits.txt` | grows one row per declared run |
| `ein-render/tests/golden/corpus_shapes.md5` | grows one entry |

Both goldens **grow**; nothing existing moves. An addition named in advance in
a stage file is a step, not a stop — the milestone's rule. `EIN_BLESS=1` is
how they are re-banked, and the stage says so before it runs it.

## The `:expect` it carries

Under Q-M1e.6 the correct side is the one whose recorded state is **maximal**.
Where both sides under-report — which is the shape this fixture is built to
show — `:expect` states the *right* answer and the fixture **fails** until
[Q-M1e.8](../../open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)'s
fix lands.

That is a deliberate choice and it needs saying out loud, because a red
fixture in the corpus is not this repo's habit. The alternative is to state
what the engine does today and re-bless later, which is what
`tests/stdlib/closure/02|03` did — *"banked so the stage that fixes it has to
move the golden"*, and it was cashed at S1d.2.6. **Take the same route:**
state today's answer, and let [D3](d3_q_m1e8_file_or_take.md)'s fix move it.
