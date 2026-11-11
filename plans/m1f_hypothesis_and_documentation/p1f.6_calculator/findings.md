# P1f.6 — what the calculator found in the engine

**Phase:** [P1f.6](README.md). Opened 2026-11-11.

The findings this phase's stages make about the engine or the language, outside their own scope. They are kept
**in the phase**, on the user's ruling of 2026-11-11:

> my ruling "plans/followups/f25_roughness_found_outside_zebra.md" was about only one phase p1f.7, so revert F25
> updates from p1f.6 and pull them back into current phase

So [F25](../../followups/f25_roughness_found_outside_zebra.md) holds P1f.7's rows, R-1 to R-8, and nothing of
this phase's. The first seven rows below were filed there as R-9 to R-15 on 2026-11-11 and moved here the same day; E-8 is
S1f.6.11's.
The calculator is the first corpus of real Ein programs, so a finding one of its programs meets is this phase's
to settle: each row is shown to the user with the stage that found it, and is fixed by a stage of this phase,
ruled as it stands, or given an owner, before the phase closes. A wrong answer among them is a row of
`corpus/wrong_answers.toml` before anything fixes it.

| id | what | witness | found by | today | state, 2026-11-11 after the re-plan |
|---|---|---|---|---|---|
| E-1 | **`__closed__` is proposed as an object.** Where the blind enumerator builds H for a declared relation and the program writes a `(__closed__ …)` fact of another, the dunder is among the candidate objects, so `(total __closed__)` is a candidate beside `(total 0)` to `(total 9)` | a program that imports the calculator prototype, which closes its own relations, and declares `(relation total Number)` without closing it: 11 candidates, k_s 2 048 (`build/s1f.6/prototype/findings/`, [design.md § Findings](design.md#findings)) | S1f.6.1's prototype | loads; `(total __closed__)` is in H (2026-11-11) | open, shown, no owner yet. The user ruled the finding behind it (a program's own relations beside the theory) left as it is; this row is the roughness, not that |
| E-2 | **A nested fact with the wrong count of arguments for its declared relation loads, and is answered by silence.** `(relation num9 …)` declares nine arguments; a request holding `(num9 <eight digits>)` loads, matches no rule, and derives nothing: exit 0 and no word | `(add (num9 0 0 1 2 3 0 7 8) (num9 0 0 0 0 0 4 5 6 7))` beside the prototype's theory (`build/s1f.6/prototype/findings/`, [design.md § Findings](design.md#findings)) | S1f.6.1's prototype | loads; no `sum-of`, no flag, exit 0 (2026-11-11) | [S1f.6.12](s1f.6.12_malformed_numbers.md): its investigation decides whether the loader holds a nested fact to its declared arity |
| E-3 | **The fact index does not key on a nested-fact value**, so a join on an argument that is a nested fact scans its relation (`ein-infer/src/match_.rs`, `probe_key`), and a batch of requests over nested numbers costs quadratically | 4 000 nine-digit sums: 142 ms on `num9` with one wide rule, 7.3 s on the nested little-endian theory (`build/s1f.6/prototype/q9_scale.py`, 2026-11-11; [design.md § Round 2](design.md)) | S1f.6.1's round 2 | right answers, quadratic time | open, shown, no owner yet as an engine row. The theory avoids it by rule: [S1f.6.3](s1f.6.3_addition.md)'s test holds that no rule of `std.arith` joins two facts on a number; [S1f.6.13](s1f.6.13_num9.md) meets it in `num9`'s division |
| E-4 | **A `--dump-answer` state file holding a fact nested past the parser's limit of 1 000 does not reload.** A rule can derive a fact deeper than any program can write | 999 nines + 1 on the nested little-endian theory, a number of 1 000 digits ([design.md § Round 2](design.md)) | S1f.6.1's round 2 | the answer is right; its state file is refused on reload | [S1f.6.9](s1f.6.9_width.md), which puts the choice to the user |
| E-5 | **`--trace`'s *Derives* line names only a firing's first conclusion** | a rule concluding `(division-by-zero …)` and `(false)` (`build/s1f.6/prototype/le/f/`; [design.md § Round 2](design.md)) | S1f.6.1's round 2 | the second conclusion is stored and not named on that line | open, shown, no owner yet. [S1f.6.6](s1f.6.6_division.md) derives the flag and `(false)` by two rules, so its trace does not depend on it |
| E-6 | **A fact stated twice keeps its first statement and silently loses the later `:source`** | the import probes under `build/s1f.6/prototype/g/imp/` ([design.md § Round 2](design.md)); F24's neighbour | S1f.6.1's round 2 | loads, no word | **stated 2026-11-11** by [S1f.6.11](s1f.6.11_importing_facts.md) (its point (e), [Q-M1f.139](../open_questions.md#q-m1f139--the-same-fact-from-two-places), `defined_behaviour.md` § 4.25 and `01_grammar.md` § Import), not ruled there; the question is [F24](../../followups/f24_written_provenance.md)'s |
| E-7 | **The blind enumerator proposes as objects the relation names a rule asserts as values** | the import probes' `liquid`, `thing` (`build/s1f.6/prototype/g/imp/`; [design.md § Round 2](design.md)); E-1's neighbour | S1f.6.1's round 2 | loads; the names are candidate objects | open, shown, no owner yet; E-1's neighbour |
| E-8 | **A qualified import brings a module's `(query …)`, `(solve …)` and `(config …)`.** A bare or `:as` import keeps every form of the module, so the importer is asked the module's queries, solved under the module's lattice and configured by the module's flags; two imports of a module that holds a `(solve …)` are refused, *2 (solve …) forms — a program states its lattice once* | a module holding `(query :goal (q ?x))`, `(solve :hypothesis-relations ())` and `(config :record-alternative-justifications false)`, imported `:as n`, and again `:as k` ([S1f.6.11 § T1 record](s1f.6.11_importing_facts.md#t1-record-2026-11-11)) | S1f.6.11's T1 | loads; `ein solve` answers the module's query as `(n.q ?x)`, and `--dump-config` shows the module's flag (2026-11-11). A flat import brings none of the three, before and after S1f.6.11 | open, shown, no owner yet. The witnesses' two modules each hold a `(solve …)` and are imported flat |

**On E-3, from S1f.6.1's round 3** ([design.md § Round 3](design.md#round-3-2026-11-11-several-representations-at-once)):
a theory can avoid it. A rule whose premises share no nested value is never matched by a scan, and
`build/s1f.6/prototype/r3/k.ein` computes the five operations over nested numbers with such rules alone, each
call carrying a continuation tag: 4 000 nine-digit divisions in 2.2 s, where the theory of round 2 does not
finish 1 000 in 45 s (`python3 r3_table.py`, 2026-11-11). The same table shows E-3 on fixed-width numbers too:
`num9`'s division joins its state with the divisor's multiples on the divisor, 193 ms for 250 divisions and
8.1 s for 4 000. The row stays open: a join on a nested value is the natural way to write a rule.

The flat import that drops a module's facts is not a row here: it is [`design.md`](design.md)'s finding 1, with
a fix specification of its own in § Round 2, **fixed on 2026-11-11 by [S1f.6.11](s1f.6.11_importing_facts.md)**
(W88, four register rows).
