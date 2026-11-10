# F23 — Diagnostics in Ein's own words

**Filed 2026-11-05**, from M1e2's [inventory of 2026-11-05](../m1e2_correctness/inventory_2026-11-05.md) § A.3, on the user's ruling of that day ([Q-M1e2.16](../m1e2_correctness/open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items) (4): *"parked in a followup"*).

**What it owns.** The diagnostics and spellings that M1e2 found and did not fix, and two labels the ruling named no owner for (D-12, D-13). None of them changes an answer: each is a line an error path prints, a label, or a page or record that no longer says what holds. They are kept here so that whoever next touches an error path finds them in one place.

**Trigger.** Any of these:
- a release, which shows these lines to a user;
- M5's presentation work;
- a stage that touches the loader's or the compiler's error paths;
- a user hitting one.

## The items

Re-taken on the inventory's snapshot build at `cdcc3994`, 2026-11-05, unless the row says otherwise.

| id | what | source | re-taken |
|---|---|---|---|
| D-1 | **Python spellings, which differ between commands.** `ein solve nope.ein` prints `FileNotFoundError: [Errno 2] …` where `ein saturate` prints `error: nope.ein not found`. `ein saturate` on `unbound-assert-var.ein` prints `KeyError: "unbound var ?v1 …"`, where `solve` prints the message alone. `solve` and `saturate` prefix `ein.inference.compile.CompileError:` and `oracle` does not. The `CompileError` lines carry the Python IR's repr, `` `SForm(head=Var(name='R'), args=(…))` ``. [`defined_behaviour.md` § 4](../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) lists the three. **The `CompileError` half was taken by M1f [S1f.5.21](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md)** (2026-11-10): no command prints the class or the IR's repr, the refusals a program reached are the loader's (§ 4.15), and the macro body's arity line is one line in Ein's words on every command that compiles (§ 4.3). `KeyError` went at M1f S1f.5.18. **`FileNotFoundError` stays here**: it is the read path's (`ein-cli`'s `common::read_program`), not the compiler's, and on 2026-11-10 `solve`, `saturate` and `oracle` print `FileNotFoundError: [Errno 2] No such file or directory: 'nope.ein'` where `ein test` prints `error: nope.ein: No such file or directory (os error 2)` (S1f.5.21's T1, Q-M1f.75) | [S1e2.2.18 § Notes](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md#notes) | `FileNotFoundError` live; the `CompileError` half fixed |
| D-2 | **Loader messages end `at None`, or `at Loc(file=…, line=…, col=…)`.** This is defined behaviour (§ 1.5), but a Python spelling. **The `Loc(…)` half was taken by M1f S1f.5.21** (Q-M1f.75): `ein_ir::loc_repr`, which the compiler's arity line printed too, prints `<file>:<line>:<col>` since, so `macro_arity_mismatch.expected` reads `… at {FILE}:6:20`. `at None` stays here: a top-level form carries no position | [Q-M1a.6](../../docs/history/m1a_rust/open_questions.md) | `at None` live (`examples/broken/load/`); `Loc(…)` fixed |
| D-3 | **`ein test` on a refuted `(solve … :expect (region …))` exits 1 with an empty stderr**, where `ein solve` prints `(solve :expect) FAILED — the claim lists 1 state and R has 2` | [S1e2.2.15 § Notes](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md#notes) | live |
| D-4 | **The `[activator …]` label prints symbols only**, so a refusal on an all-int activator names no activator | [S1e2.2.16 § Notes](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.16_activator_text_key.md#notes) | live (`activator-all-int.ein`) |
| D-5 | **The plan shape's `GUARD` line prints `String(value='b')`** for the symbol `b` | S1e2.2.18 § Notes | library and `corpus_shapes.md5` only |
| D-6 | **`ein saturate --max-steps 3` prints `running with max_steps=3`** and `last='includes'` | [S1e2.3.7 § Notes](../m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.7_four_lines_that_misname.md#notes) | live |
| D-7 | **`ein.inference.saturator.SaturatorStepLimitError`** and a `{:?}` firing at `common.rs:254` and `saturator.rs:632` | S1e2.3.7 § Notes | unreachable from the binary |
| D-8 | **An hrule's template that builds no fact proposes nothing, and nothing says so.** `hrule.rs` reads `Err(_) => continue`; only a range is refused at load | S1e2.2.18 § Notes | live (`:assert foo` loads with no diagnostic) |
| D-9 | **An unused rule parameter is accepted silently** | [S1e2.2.12 § Notes](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md#notes) | not run; a reading of the loader |
| D-10 | **`ein render rules` exits 1 on a file with no rule** (*"no rule forms in …"*). This is contested against the user's *exit non-zero only on error*: a file with no rule may be an answer, not an error | M1e2's review | live |
| D-11 | **Pages and records that no longer say what holds**, outside `plans/m1e2_correctness/`, which the close moves or deletes. The list is below | the inventory § A.3 | by reading |
| D-12 | **The `types` warning reads `is-a` by name only**, so a type relation with another name (`instance`) is never read. The inventory's S-10, placed here by default: Q-M1e2.16 (3) named no owner for it, and it is a warning, not an answer | [S1e2.2.10 § Notes](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md#notes) | latent: the ten `instance` programs all have H = ∅ |
| D-13 | **`corpus_shapes.md5`'s `hyp` rows digest `hypgen::generate`**, the walk's ladder with the lookahead, the kill cache and `emit_closed`, which no route and no command runs. The inventory's S-14, placed here by default as D-12 is: a test golden, not a user surface | [S1e2.3.1](../m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md) § What applying the ruling measured | `ein-infer/src/shape.rs` |

What D-11 covers:
- `configuration.md:407–411` says a deadline stop reports a partial answer, then that a `-T` abort prints none. `read_out.md` § 8.1 lists `stopped.why: deadline`. Re-taken: `-T 0` exits 2 and prints no read-out.
- `solution_semantics.md` § 3 says *"No surface computes projection classes yet"*, though § 5.1 and every read-out print them.
- `monotonicity.md` § 7.9 quotes *"the flagged sites break nothing here"* in the present tense.
- `region.rs`'s module doc speaks of `alive₀`, the cascade and the lookahead.
- `implementation.md` names `--dump-region`.
- Five obligation fixtures' headers say `:expect` cannot state what a state owes.
- `lever_invariance.rs`'s module doc.
- `release.yml` says *"two slow entries"* and *"everything nightly"*.
- `plans/README.md` says *"31 probe programs"*.
- F4's *Q40* section still calls a fixed bug live.
- M1e's Q-M1e.8 index row still says open.
- Global Q28 says `()` *"parses as a no-op atom"*. At top level it is a parse error.

The milestone's close fixes any D-11 page it touches on its way, and strikes it here.
