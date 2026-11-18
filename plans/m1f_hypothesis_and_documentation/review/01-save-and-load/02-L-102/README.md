# L-102 — the writer of a fact with its English, which no command uses

## 1. The question

`ein_render::dump::serialise` holds the code that wrote a state file's facts, one per line, each with a comment in
English: `provenance_comment` (the comment: `; source "…"`, `; rule r: <reason>`, `; hypothesis N`, then
` — <statement>`), `fact_line` (a fact and its comment), `kb_to_ein_text` (a whole KB so) and `facts_to_ein_text`
(chosen facts so). Since save/load, a saved file holds no fact (Q-M1f.400, Q-M1f.401), so no command of the binary
calls any of them. The one caller left is a test: the corpus shape digest's `states` op
(`ein-render/tests/corpus_ops`), which prints each state with this writer so that the golden digests it. The call is
whether a surface writes a state's facts in English again (then the writer has a caller), or the code goes and the
digest prints the facts its own way. It goes with [`Q-M1f.414`](../01-Q-M1f.414/README.md).

Recorded: [`findings.md`](../../../findings.md#dead-code-and-comments), row **L-102** (§ Dead code, and comments).

## 2. What is so today

**The code**, as it stands at HEAD: [`runs/code.serialise.rs.today.txt`](runs/code.serialise.rs.today.txt)
(`ein-render/src/dump/serialise.rs` lines 1 to 110, numbered). Since the earlier binary only its doc comments moved
([`runs/code.serialise.rs.before-vs-today.diff`](runs/code.serialise.rs.before-vs-today.diff)), and
`ein-render/src/sentence.rs`, where the English comes from, is the same bytes
([`runs/code.sentence.rs.before-vs-today.diff`](runs/code.sentence.rs.before-vs-today.diff), empty).

**Every caller**, `grep -rn 'facts_to_ein_text\|kb_to_ein_text\|fact_line\|provenance_comment' ein.rs/crates`
([`runs/grep.callers.today.txt`](runs/grep.callers.today.txt)) (output, the lines that are calls or uses):

```text
ein.rs/crates/ein-render/src/dump/mod.rs:17:pub use serialise::kb_to_ein_text;
ein.rs/crates/ein-render/src/dump/serialise.rs:47:    match provenance_comment(kb, terms, f) {
ein.rs/crates/ein-render/src/dump/serialise.rs:90:        .push(fact_line(kb, terms, f));
ein.rs/crates/ein-render/src/dump/serialise.rs:106:        text.push_str(&fact_line(kb, terms, f));
ein.rs/crates/ein-render/tests/corpus_ops/mod.rs:660:                        ein_render::dump::serialise::facts_to_ein_text(kb, terms, &state)
```

The rest of the grep's lines are the definitions, `kb_dot.rs`' unrelated `emit_fact_line`, and a doc comment in
`ein-ir/tests/kb_semantics.rs`. So: `provenance_comment` and `fact_line` are called only inside the module;
`kb_to_ein_text` is re-exported and called by nobody; `facts_to_ein_text` is called by the `states` op alone. In the
earlier tree the binary called it, from the state file writer `region_answer.rs:711`
([`runs/grep.callers.before-a8d595eb.txt`](runs/grep.callers.before-a8d595eb.txt); the writer,
[`runs/code.region_answer.state_file.before-a8d595eb.txt`](runs/code.region_answer.state_file.before-a8d595eb.txt)).

**What the earlier binary printed with it**: every fact of a state but the program's `relation` facts, sorted, after
`;; ── the state`. On [`programs/walks.ein`](programs/walks.ein) (the program of
[`01-Q-M1f.414`](../01-Q-M1f.414/README.md)):
[`runs/walks.dump-answer.before-ein-a8d595eb/models/02.ein`](runs/walks.dump-answer.before-ein-a8d595eb/models/02.ein)
(output, its last lines):

```text
(busy Alice) ; hypothesis 0 — Alice is busy
(dog Rex) ; Rex is a dog
(owns Alice Rex) ; source "sentence 3" — Alice owns Rex
(tired Alice) ; rule tire: Alice is busy and walks Rex, so Alice is tired — Alice is tired
(walks Alice Rex) ; rule walk: Alice owns the dog Rex, so Alice walks Rex — Alice walks Rex
```

Today's binary writes the same state as the program and `(commit (busy Alice))`
([`runs/walks.dump-answer.today/models/02.ein`](runs/walks.dump-answer.today/models/02.ein)).

**What the shape digest's `states` op prints with it.** The op ([`runs/code.corpus_ops.states-op.today.txt`](runs/code.corpus_ops.states-op.today.txt))
saves every state as the binary does, takes the `(commit …)` tail of each saved file, and appends the state the
node function reaches, written by `facts_to_ein_text`: `--- NN.ein`, `(commit`, the tail, the facts. The op runs only
inside `cargo test`, which this review does not run, so its text is **reconstructed** here by
[`tools/state_op.py`](tools/state_op.py) from two runs — today's saved files for the tails, the earlier binary's state
files for the facts (same filter, same sort, same writer, same node function, `sentence.rs` unchanged) — and the
reconstruction is **checked against the golden**: digested as the golden digests (md5's first 16 hex digits and the
line count), it gives the line banked in `ein-render/tests/golden/corpus_shapes.md5` on all five corpus programs
tried, among them `examples/zebra.ein` (611 lines) ([`runs/states-op.golden-check.txt`](runs/states-op.golden-check.txt)).
On `walks.ein`: [`runs/walks.states-op.RECONSTRUCTED.txt`](runs/walks.states-op.RECONSTRUCTED.txt) (output, the
second state):

```text
--- 02.ein
(commit
  (busy Alice))
(busy Alice) ; hypothesis 0 — Alice is busy
(dog Rex) ; Rex is a dog
(owns Alice Rex) ; source "sentence 3" — Alice owns Rex
(person Alice) ; source "sentence 1" — Alice is a person
(person Bob) ; source "sentence 2" — Bob is a person
(tired Alice) ; rule tire: Alice is busy and walks Rex, so Alice is tired — Alice is tired
(walks Alice Rex) ; rule walk: Alice owns the dog Rex, so Alice walks Rex — Alice walks Rex
```

On the corpus program `tests/saved/02_a_choice_with_a_fact_inside_it.ein` ([`programs/`](programs/02_a_choice_with_a_fact_inside_it.ein)):
[`runs/02_a_choice_with_a_fact_inside_it.states-op.RECONSTRUCTED.txt`](runs/02_a_choice_with_a_fact_inside_it.states-op.RECONSTRUCTED.txt),
the golden's `0849931ad5096687 9L`. So the golden's 671 `::states` lines still digest the English the state files
carried; no file and no command shows it.

Before save/load the op read those comments from the state files themselves
([`runs/code.corpus_ops.states-op.before-a8d595eb.txt`](runs/code.corpus_ops.states-op.before-a8d595eb.txt));
today it calls the writer to print them.

## 3. The options

**(1) A surface writes a state's facts in English again** — Q-M1f.414's (b), `ein saturate --dump`, or (c), `-p`.
The writer gets a caller in the binary, as `fact_line` (each fact with its comment). What it would print is mocked in
[`01-Q-M1f.414`](../01-Q-M1f.414/README.md#3-the-options). The `states` op can stay as it is.

**(2) The code goes**, the four functions and the `pub use`, and the `states` op prints the facts its own way. A code
change in `serialise.rs` and `corpus_ops`, and the golden's 671 `::states` lines re-banked (`EIN_BLESS=1`). Two
mocks of what the op would print, hand-edited from the reconstruction (made by
[`tools/mock_states_op.py`](tools/mock_states_op.py)):

- **(2a) each fact alone**, the comment dropped:
  [`runs/walks.states-op.MOCK.option-2a-fact-alone.txt`](runs/walks.states-op.MOCK.option-2a-fact-alone.txt)
  ([diff](runs/walks.states-op.MOCK.option-2a-fact-alone.diff)). The digest then holds a state's facts and nothing of
  where each came from.
- **(2b) each fact with its origin, as `ein saturate --dump` prints it**, the English dropped:
  [`runs/walks.states-op.MOCK.option-2b-fact-and-origin.txt`](runs/walks.states-op.MOCK.option-2b-fact-and-origin.txt)
  ([diff](runs/walks.states-op.MOCK.option-2b-fact-and-origin.diff)) (output):

  ```text
  --- 02.ein
  (commit
    (busy Alice))
  (busy Alice :hypothesis 0)
  (dog Rex)
  (owns Alice Rex :source "sentence 3")
  (person Alice :source "sentence 1")
  (person Bob :source "sentence 2")
  (tired Alice :rule tire)
  (walks Alice Rex :rule walk)
  ```

  The digest keeps each fact's origin, which the round trip holds a saved file's load to; it stops holding the
  English. What still digests English is the `region-trace` op, which the golden digests whole: a reason and a
  derived fact's statement, not a given's statement.

## 4. The default

None of its own: the row is **the user's call**, with Q-M1f.414. Under Q-M1f.414's default, (a), the row's own words
are *"else the code goes, the digest printing the facts its own way"*; that has not been done, and the code is there
today.

## 5. Files

- [`programs/walks.ein`](programs/walks.ein) — the program of `01-Q-M1f.414`, copied.
- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copied from `tests/saved/`.
- [`runs/grep.callers.today.txt`](runs/grep.callers.today.txt) — every caller at HEAD.
- [`runs/grep.callers.before-a8d595eb.txt`](runs/grep.callers.before-a8d595eb.txt) — every caller in the earlier tree.
- [`runs/code.serialise.rs.today.txt`](runs/code.serialise.rs.today.txt) — the four functions, numbered.
- [`runs/code.serialise.rs.before-vs-today.diff`](runs/code.serialise.rs.before-vs-today.diff) — `serialise.rs` since the earlier tree: doc comments.
- [`runs/code.sentence.rs.before-vs-today.diff`](runs/code.sentence.rs.before-vs-today.diff) — `sentence.rs` since then: empty.
- [`runs/code.corpus_ops.states-op.today.txt`](runs/code.corpus_ops.states-op.today.txt) — the `states` op at HEAD.
- [`runs/code.corpus_ops.states-op.before-a8d595eb.txt`](runs/code.corpus_ops.states-op.before-a8d595eb.txt) — the `states` op in the earlier tree.
- [`runs/code.region_answer.state_file.before-a8d595eb.txt`](runs/code.region_answer.state_file.before-a8d595eb.txt) — the earlier state file writer, the binary's call.
- [`runs/walks.dump-answer.before-ein-a8d595eb/`](runs/walks.dump-answer.before-ein-a8d595eb/), [`runs/walks.dump-answer.today/`](runs/walks.dump-answer.today/) — `--dump-answer` of `walks.ein`, each binary.
- [`runs/02_a_choice_with_a_fact_inside_it.dump-answer.before-ein-a8d595eb/`](runs/02_a_choice_with_a_fact_inside_it.dump-answer.before-ein-a8d595eb/), [`…today/`](runs/02_a_choice_with_a_fact_inside_it.dump-answer.today/) — the same for the corpus program.
- [`runs/walks.states-op.RECONSTRUCTED.txt`](runs/walks.states-op.RECONSTRUCTED.txt), [`runs/02_a_choice_with_a_fact_inside_it.states-op.RECONSTRUCTED.txt`](runs/02_a_choice_with_a_fact_inside_it.states-op.RECONSTRUCTED.txt) — the op's text, reconstructed.
- [`runs/states-op.golden-check.txt`](runs/states-op.golden-check.txt) — the reconstruction digested against the golden, five programs.
- [`runs/walks.states-op.MOCK.option-2a-fact-alone.txt`](runs/walks.states-op.MOCK.option-2a-fact-alone.txt) / [`.diff`](runs/walks.states-op.MOCK.option-2a-fact-alone.diff) — **MOCK** of (2a).
- [`runs/walks.states-op.MOCK.option-2b-fact-and-origin.txt`](runs/walks.states-op.MOCK.option-2b-fact-and-origin.txt) / [`.diff`](runs/walks.states-op.MOCK.option-2b-fact-and-origin.diff) — **MOCK** of (2b).
- [`tools/state_op.py`](tools/state_op.py) — the reconstruction, and its digest with `--digest`.
- [`tools/mock_states_op.py`](tools/mock_states_op.py) — the two mocks.

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/02-L-102
H=build/review-bin/ein-head-b5fb8d70
A=build/review-bin/ein-a8d595eb; EA=$PWD/build/wt-rev-a8d/stdlib
grep -rn 'facts_to_ein_text\|kb_to_ein_text\|fact_line\|provenance_comment' ein.rs/crates --include='*.rs'
(cd build/wt-rev-a8d && grep -rn 'facts_to_ein_text\|kb_to_ein_text\|fact_line\|provenance_comment' ein.rs/crates --include='*.rs')
for p in walks 02_a_choice_with_a_fact_inside_it; do
  $H solve $D/programs/$p.ein --dump-answer $D/runs/$p.dump-answer.today
  EIN_STDLIB=$EA $A solve $D/programs/$p.ein --dump-answer $D/runs/$p.dump-answer.before-ein-a8d595eb
  python3 -I $D/tools/state_op.py $D/runs/$p.dump-answer.today $D/runs/$p.dump-answer.before-ein-a8d595eb > $D/runs/$p.states-op.RECONSTRUCTED.txt
done
# the golden check, one program (the same for the others, with their corpus path):
python3 -I $D/tools/state_op.py $D/runs/02_a_choice_with_a_fact_inside_it.dump-answer.today \
  $D/runs/02_a_choice_with_a_fact_inside_it.dump-answer.before-ein-a8d595eb --digest tests/saved/02_a_choice_with_a_fact_inside_it.ein
grep '  tests/saved/02_a_choice_with_a_fact_inside_it.ein::states$' ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5
python3 -I $D/tools/mock_states_op.py $D/runs/walks.states-op.RECONSTRUCTED.txt $D/runs/walks.states-op
```

## 7. Found while building

- **The same module's `firing_to_json` and `fact_summary` have no caller either**, at HEAD or in the earlier tree:
  `grep -rn 'firing_to_json\|fact_summary' ein.rs --include='*.rs'` finds only their definitions and
  `fact_summary`'s call inside `firing_to_json`. Both are `pub`, so no warning names them. Not in the ledger.
- **`ein-render/src/dump/mod.rs`' module doc still says** *"the KB-to-`.ein` text every state file is written in
  ([`serialise`])"*: no state file is written in it now.
- The op's text for a root state reads `(commit` with nothing after it (a root's saved file has no `(commit …)`, and
  the op prints the line regardless). It is a digest's input, read by no one; it changes no verdict.

## 8. Your call

- **(1)** a surface writes a state's facts in English again (Q-M1f.414 (b) or (c)): the writer gets its caller.
- **(2)** the code goes and the `states` op prints the facts its own way: **(2a)** the fact alone, or **(2b)** the
  fact and its origin as `ein saturate --dump` prints it; 671 golden lines re-banked either way.
- No default of its own; under Q-M1f.414's default (a) the row says (2).
