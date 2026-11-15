# S1f.10.6 — A debt root was not told of

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** re-planned 2026-11-15. Not started. **Its question is put at the phase's one stop**
([S1f.10.12](s1f.10.12_the_structure_shown.md)), and nothing changes before the user replies.
**Depends on:** nothing in the phase.
**Owns:** [Q-M1e.11](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis),
*what happens to an obligation derived under a hypothesis*, and the register row
**`W26/oblig-derived-activator`**, `open`, reviewed `deferred`, with this stage as its `owner`
([`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml)).
**Was:** *obligations derived under a hypothesis, the review's `Q6`*: a probe of the walk's tree, whose rung
mode could flip at a node. The tree and its guard were deleted on 2026-11-03. The question was never about the
tree; M1e2's S1e2.2.25 restated it over R and banked the witness, and this stage starts there.

## The question, over R

H is built at root. An obligation's activator is an ordinary fact, so a rule can derive one at a node, and the
facts that would pay the new debt were never candidates.

**The witness**,
[`examples/ein-bugs/wrong-answers/oblig-derived-activator.ein`](../../../examples/ein-bugs/wrong-answers/oblig-derived-activator.ein):
`std.algebra`'s `total-owed` over `likes`, stated, and over `drinks`, whose activator a rule derives from any
`likes` fact. Today (`ein solve` and `ein oracle`, the release binary at `dbedc535`, 2026-11-15): H is the one
candidate `(likes Ann Soup)`; the node that holds it owes *drinks owes Ann a Drink*; nothing pays it; **R is
empty, every alive node owing**. The control, `oblig-derived-activator-declared.ein`, states the activator: H
is two candidates and R one state, `{(drinks Ann Tea), (likes Ann Soup)}`.

**What the engine already says of it.** The static check flags the program: *total-owed: its activator
relation is one a commitment can change*, one site, so it takes the cube and no licence. And on a monotone
program a node that owes an instance root did not **refuses the run** (`tree.rs`, `check_owed`). So the shape
is detected in both classes. What is not decided is the answer.

## The four answers

Q-M1e.11's table, restated over R; the register row carries (A) as its `right` and the other three as
alternatives.

| | what | consequence |
|---|---|---|
| **A** | **H holds what a node's debt would propose.** The universe is closed under the debts a commitment can bring: what would pay an obligation whose activator a node derives is a candidate | the control's R. The universe stops being *root's generator list*: it needs either a static reading of which activators a commitment can derive (`monotone.rs` computes the relations a commitment can change, and follows activators) with the guard's scan run for them, or an iteration over nodes. Its cost and its exactness are T2's to measure |
| **B** | **decline**: such a program's H is the blind enumerator's, as Q-M1e2.10 (a) took for W14 | a larger R that holds the control's state and more |
| **C** | **refuse at load**: every obligation activator must hold at root | a diagnostic instead of an answer, the shape of *forbid*. The static check already finds the site |
| **D** | **the answer stands, and the read-out says why**: the debt's payment is outside H | the row becomes `not-wrong`, with a line in the read-out |

## Tasks

1. **T1f.10.6.1 — the witness and the control re-taken** on the binary as it stands, each read-out kept under
   `build/`, and the same pair written for the hrule rung and the blind rung, to show whether the question is
   the obligations rung's alone.
2. **T1f.10.6.2 — the census.** Over every corpus program: does a rule conclude an activator of an obligation
   rule from a relation a commitment can change (static, from `monotone.rs`'s sites; the `site_census`
   example); and of those within the oracle's reach, does a node owe an instance root did not (by the cube).
   Expected: the witness and few others. And (A) priced on them: the universe each of its two readings gives,
   and R over it by `ein oracle` on a copy that states what the reading would add.
3. **T1f.10.6.3 — the question**, filed under the next free id in
   [`open_questions.md`](../open_questions.md) with the census, and put at the phase's stop. **Default: (A)
   where T2 shows a static reading that gives the control's R on the witness and moves no other corpus
   program's R; otherwise (C).** Both name a wrong answer today; (B) trades it for a different H, and (D)
   keeps an empty R where a model exists. The default is not acted on before the user replies.
4. **T1f.10.6.4 — as ruled.** The fix, with the row flipped in the fixing commit and the control's
   expectation unchanged; under (A), the read-out's `universe` row says what was added and
   [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) § 1.2 is restated; under
   (C), a `defined_behaviour.md` § 4 section and a `broken/load/` fixture; under (D), the row's status and the
   read-out's line with `read_out.md`. Q-M1e.11 recorded as ruled, in the user's words.
   [`events.md`](../../../docs/kernel/inference/events.md)'s pointer to this stage restated to say what the
   engine does.

## Acceptance

- The witness, the control and the census are on the stage's record, each with its command.
- The user has ruled; Q-M1e.11 is closed; W26 is `fixed` or `not-wrong`, and the register has no row left
  whose owner is a stage of this phase.
- **[README § The domain](README.md#the-domain)'s second paragraph is true of the engine after the ruling**,
  and is restated if the ruling changes what H is.
- If an answer moved, it moved by the register row, and the phase's README names this stage as an exception
  to *no answer changes*.

## What it must not do

- **Not change H before the ruling.**
- **Not decide what a node is on a non-monotone program.** That is
  [F21](../../followups/f21_non_monotone_programs_decided.md)'s; this stage's question is what H is, and it
  says where the two meet.
- **Not weaken `check_owed`.** Under any ruling, a monotone program on which a node owes what root did not is
  either inside the new H or still refused.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, and the fix flips the row in the fixing commit. Config
never changes an answer. The gate is `./run_tests.sh`, once on the last commit of the batch.
