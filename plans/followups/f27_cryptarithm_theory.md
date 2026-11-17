# F27 — A cryptarithm theory, `std.math.crypt`

**Filed 2026-11-17** by M1f S1f.20.8, the stage of its documentation phase that wrote the decisions,
on the user's ruling at S1f.20.2's review (item 11: *"std.math.crypt (P1f.6's #42) goes to a followup"*), so that
the decision outlives P1f.6's folder, which S1f.20.9 deleted.

**The decision, taken and not built.** P1f.6's question 42, *A small cryptarithm theory in the stdlib?*, was
answered **yes** (the user, in
`questions.md`: *"Yes, again not
std.crypt, maybe std.math.arith.crypt"*), and the replies' index placed it as **`std.math.crypt`, in the layout,
later** (`replies_2026-11-12.md`,
row 42). The user fixed its id at **Q-M1f.337** with the map of local numbers (S1f.20.2's review, item 7). Being
unbuilt, it has no file under `docs/decisions/`: a decision file states what holds, and nothing of this holds yet.
The decision census carried it as `P1f.6#42`, `as Q-M1f.337`, flagged `pending`, until it was retired (S1f.20.9);
this followup is its record now. When the theory ships, its decision file is written as
`docs/decisions/stdlib/math/Q-M1f.337-<slug>.md`.

**What it is.** The column rules of an addition with the digit tables read in every direction, written once, so a
cryptarithm states only its letters and its columns; SEND + MORE = MONEY its example. The question's source is
[`08-word-problems.md`](../../docs/ein/user/02-guide/08-word-problems.md#cryptarithms) § 3 and its
question 7: the same puzzle as a plain search takes 17 747 search nodes, with the tables read backwards as
propagation 639, and a human a dozen deductions. *"A cryptarithm theory in the stdlib, with those rules written
once, would be to arithmetic puzzles what `std.bijection`'s elimination rules are to the zebra."* What the corpus
holds today: [`examples/beyond_arith/send_more_money.ein`](../../examples/beyond_arith/send_more_money.ein) (its
generated twin `send_more_money_written.ein`), [`cryptarithm.ein`](../../examples/beyond_arith/cryptarithm.ein)
(I + BB = ILL, whose rule `add-column` is the program's own) and
[`tables_backwards.ein`](../../examples/beyond_arith/tables_backwards.ein).

**Why not now.** The reply put it after two things, both of which have run: `ein test` deciding a claim by the
least question (P1f.6's question 39, S1f.6.27, done: a claim of one state is asked `--single`'s question), and the
structure of the hypothesis space (question 40, P1f.10, closed), whose finding E-29 is that the order the owed
instances are branched on, which a cryptarithm's declaration order sets, decides whether its search ends. No plan
has taken it since; P1f.6's close names it as the one reply with no stage.

**Trigger.** Any of: a stage that adds a module under `stdlib/math/`; a corpus program that writes its own
column rules for a cryptarithm (as `cryptarithm.ein` does); a benchmark of puzzle families beside the zebra
([F13](f13_puzzles_beyond_zebra/ideas.md), [M10](../m10_external_benchmarks/README.md)); or a search-order stage,
for which a cryptarithm in two declaration orders is the cheapest witness.

**Connections.** The calculator's layout and rules, [AGENTS.md](../../AGENTS.md) § Where things live (`stdlib/math/`)
and § Edits that go together (a module's closing rule, its program under `tests/stdlib/math/`, a row of `OPS`);
[M1h](../m1h_beyond_word_problems/README.md), the other puzzle and problem families after the calculator.
