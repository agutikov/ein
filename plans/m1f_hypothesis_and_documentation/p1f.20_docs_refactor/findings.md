# P1f.20 — what the phase found outside its stages

**Phase:** [P1f.20](README.md). Opened 2026-11-16.

What a stage of this phase finds outside its own scope: kept in the phase and shown to the user at its next
stop (the first is [S1f.20.2](s1f.20.2_the_disposition_table.md)'s review), not fixed by the stage that found
it. A wrong answer among them would be a row of [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml)
before anything fixed it; none below is one.

| id | what | witness, and the command | found by | owner, by default |
|---|---|---|---|---|
| L-1 | **A bare path outside link syntax is read by neither check** in a `.md` page or a Rust comment: `doc_audit.py --links --tracked` reads a page's links and a Rust comment's links, and a bare path only in a `.ein` comment and `corpus.toml`. Five name nothing today outside `plans/` and `docs/history/`: `docs/kernel/README.md:44` (`docs/ir.md`), `examples/README.md:43`, `:47`, `:53`, `utils/README.md:85`. The crates hold 53 such paths, one of them gone on purpose (`ein-infer/tests/refutation_under_absent.rs:47`); `plans/` 98 that name nothing, `docs/history/` 11 | [S1f.20.1's record § Findings outside the stage](s1f.20.1_the_two_checks.md) has the list and the grep | S1f.20.1 | the moves of S1f.20.3 to S1f.20.5 grep for them with each page they move; S1f.20.6 fixes the five |
| L-2 | ~~**A `.ein` string carries the development**, and neither check reads strings~~. **Withdrawn by the coordinator, 2026-11-16**: both witnesses S1f.20.1 named are comments, which the release-voice check reads already (`stdlib/elim.ein:3` is a `;;;` line quoting `"M1-blocking stdlib"`; the date in `examples/lattice/synth/three_components.ein:30` is inside its `;;; region-spec:` comment). No `:why`, `:source` or `:goal-text` string carries an id or a date today. What remains true: the check does not read strings, so one written later would pass | `grep -rnE ':(why\|source\|goal-text) +"[^"]*(20[0-9]{2}-[0-9]{2}\|M1\|S1[a-z]*\.)' --include='*.ein' stdlib examples tests` prints nothing | S1f.20.1 | none; S1f.20.9 widens the check, and may read strings |
| L-3 | **A false positive in the release-voice baseline**: `stdlib/math/decimal-arith.ein:621`, `(multiples M1 … M9)`, is a list of variable names, counted as milestone ids | `python3 utils/check_release_voice.py --list stdlib/math/decimal-arith.ein` | S1f.20.1 | S1f.20.6: escape or reword |
| L-4 | **`utils/README.md`'s count is loose**: it says all but eight scripts drive `ein.rs`, but `check_hashmap_iteration.py`, `stdlib_manifest.py` and `check_test_scratch.py` do not drive it either | `grep -n 'eight' utils/README.md` | S1f.20.1 | S1f.20.5, which rewrites the scripts' catalogue into the tree |
