# P1f.6 — what the calculator found in the engine

**Phase:** [P1f.6](README.md). Opened 2026-11-11.

The findings this phase's stages make about the engine or the language, outside their own scope. They are kept
**in the phase**, on the user's ruling of 2026-11-11:

> my ruling "plans/followups/f25_roughness_found_outside_zebra.md" was about only one phase p1f.7, so revert F25
> updates from p1f.6 and pull them back into current phase

So [F25](../../followups/f25_roughness_found_outside_zebra.md) holds P1f.7's rows, R-1 to R-8, and nothing of
this phase's. The seven rows below were filed there as R-9 to R-15 on 2026-11-11 and moved here the same day.
The calculator is the first corpus of real Ein programs, so a finding one of its programs meets is this phase's
to settle: each row is shown to the user with the stage that found it, and is fixed by a stage of this phase,
ruled as it stands, or given an owner, before the phase closes. A wrong answer among them is a row of
`corpus/wrong_answers.toml` before anything fixes it.

| id | what | witness | found by | today | state |
|---|---|---|---|---|---|
| E-1 | **`__closed__` is proposed as an object.** Where the blind enumerator builds H for a declared relation and the program writes a `(__closed__ …)` fact of another, the dunder is among the candidate objects, so `(total __closed__)` is a candidate beside `(total 0)` to `(total 9)` | a program that imports the calculator prototype, which closes its own relations, and declares `(relation total Number)` without closing it: 11 candidates, k_s 2 048 (`build/s1f.6/prototype/findings/`, [design.md § Findings](design.md#findings)) | S1f.6.1's prototype | loads; `(total __closed__)` is in H (2026-11-11) | open |
| E-2 | **A nested fact with the wrong count of arguments for its declared relation loads, and is answered by silence.** `(relation num9 …)` declares nine arguments; a request holding `(num9 <eight digits>)` loads, matches no rule, and derives nothing: exit 0 and no word | `(add (num9 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))` beside the prototype's theory (`build/s1f.6/prototype/findings/`, [design.md § Findings](design.md#findings)) | S1f.6.1's prototype | loads; no `sum-of`, no flag, exit 0 (2026-11-11) | open |
| E-3 | **The fact index does not key on a nested-fact value**, so a join on an argument that is a nested fact scans its relation (`ein-infer/src/match_.rs`, `probe_key`), and a batch of requests over nested numbers costs quadratically | 4 000 nine-digit sums: 142 ms on `num9` with one wide rule, 7.3 s on the nested little-endian theory (`build/s1f.6/prototype/q9_scale.py`, 2026-11-11; [design.md § Round 2](design.md)) | S1f.6.1's round 2 | right answers, quadratic time | open |
| E-4 | **A `--dump-answer` state file holding a fact nested past the parser's limit of 1 000 does not reload.** A rule can derive a fact deeper than any program can write | 999 nines + 1 on the nested little-endian theory, a number of 1 000 digits ([design.md § Round 2](design.md)) | S1f.6.1's round 2 | the answer is right; its state file is refused on reload | open |
| E-5 | **`--trace`'s *Derives* line names only a firing's first conclusion** | a rule concluding `(division-by-zero …)` and `(false)` (`build/s1f.6/prototype/le/f/`; [design.md § Round 2](design.md)) | S1f.6.1's round 2 | the second conclusion is stored and not named on that line | open |
| E-6 | **A fact stated twice keeps its first statement and silently loses the later `:source`** | the import probes under `build/s1f.6/prototype/g/imp/` ([design.md § Round 2](design.md)); F24's neighbour | S1f.6.1's round 2 | loads, no word | open |
| E-7 | **The blind enumerator proposes as objects the relation names a rule asserts as values** | the import probes' `liquid`, `thing` (`build/s1f.6/prototype/g/imp/`; [design.md § Round 2](design.md)); E-1's neighbour | S1f.6.1's round 2 | loads; the names are candidate objects | open |

The flat import that drops a module's facts is not a row here: it is [`design.md`](design.md)'s finding 1, with
a fix specification of its own in § Round 2.
