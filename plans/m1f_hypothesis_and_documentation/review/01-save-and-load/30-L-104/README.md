# L-104 — `ein saturate --dump` prints `:hypothesis 0` for every committed fact

## 1. The question

`ein saturate --dump FILE` prints the saturated KB grouped by origin, each fact with where it came from:
`:source "…"` for a given with a source, `:rule r` for a derived fact, and `:hypothesis N` for a fact a commitment
put there. Since save/load, a saved file's `(commit …)` is loaded as the node's commitment, so a dump of a saved file
prints `:hypothesis N` after each choice. **N is always 0**: two, three committed facts of one state all read
`:hypothesis 0`. Before save/load the state file wrote the same number in a comment, `; hypothesis 0`, on every
committed fact. The call: what `:hypothesis N` names, or whether the dump prints it at all.

Recorded: [`findings.md`](../../../findings.md#the-language-and-the-engine), row **L-104** (§ The language and the
engine).

## 2. What is so today

**Today's dump of the corpus's saved node**, `tests/saved/04_a_saved_node.ein`
([`programs/04_a_saved_node.ein`](programs/04_a_saved_node.ein)): [`runs/04_a_saved_node.saturate-dump.today.txt`](runs/04_a_saved_node.saturate-dump.today.txt) (output)

```text
;; ── DERIVED (2 facts) ──
  (flagged a :rule flag)
  (note (bad a) :hypothesis 0)
```

The ledger's witness reproduces.

**A saved file with two and three commitments.** [`programs/busy.ein`](programs/busy.ein), written for this review:
three people, an hrule that supposes each busy, a rule `pair` that reads two commitments; no refutation, so R is the
whole cube of three candidates, eight states ([`runs/busy.solve.today.txt`](runs/busy.solve.today.txt)). Saved by
`--dump-answer` ([`runs/busy.dump-answer.today/`](runs/busy.dump-answer.today/)); the state {(busy Bob), (busy Carol)}
is `models/07.ein`, which ends `(commit (busy Bob) (busy Carol))`. Its dump,
[`runs/busy.models-07.saturate-dump.today.txt`](runs/busy.models-07.saturate-dump.today.txt) (output):

```text
;; ── DERIVED (4 facts) ──
;;   both-busy (2)
  (both-busy Bob Carol :rule pair)
  (both-busy Carol Bob :rule pair)
;;   busy (2)
  (busy Bob :hypothesis 0)
  (busy Carol :hypothesis 0)
```

And `models/05.ein` ({Alice, Bob}), `models/08.ein` ({Alice, Bob, Carol}):
[`runs/busy.models-05.saturate-dump.today.txt`](runs/busy.models-05.saturate-dump.today.txt),
[`runs/busy.models-08.saturate-dump.today.txt`](runs/busy.models-08.saturate-dump.today.txt). Every `:hypothesis`
line of the four dumps: [`runs/today.saturate-dump.hypothesis.grep.txt`](runs/today.saturate-dump.hypothesis.grep.txt),
all `0`.

**The earlier binary's state files**, the same program
([`runs/busy.dump-answer.before-ein-a8d595eb/`](runs/busy.dump-answer.before-ein-a8d595eb/)): every committed fact
`; hypothesis 0` ([`runs/busy.before-ein-a8d595eb.hypothesis-comments.grep.txt`](runs/busy.before-ein-a8d595eb.hypothesis-comments.grep.txt),
twelve lines, all `0`) (output, `models/08.ein`):

```text
(busy Alice) ; hypothesis 0 — Alice is busy
(busy Bob) ; hypothesis 0 — Bob is busy
(busy Carol) ; hypothesis 0 — Carol is busy
```

And for the program `tests/saved/04` saves ([`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein),
the same program, one blank line aside): `(note (bad a)) ; hypothesis 0 — a note says (bad a)`
([`runs/02_a_choice.dump-answer.before-ein-a8d595eb/models/02.ein`](runs/02_a_choice.dump-answer.before-ein-a8d595eb/models/02.ein)).
The earlier binary's `ein saturate --dump` of its own state file printed no `:hypothesis` at all: the file stated the
state's facts, and they loaded as background facts
([`runs/busy.models-08.saturate-dump.before-ein-a8d595eb.txt`](runs/busy.models-08.saturate-dump.before-ein-a8d595eb.txt)).
So `:hypothesis N` in a dump is new with save/load; the 0 is the comment's.

**The code** ([`runs/code-lines.grep.txt`](runs/code-lines.grep.txt)):

- the dump prints the field at `ein-cli/src/saturate.rs:429`,
  `ProvKind::Hypothesis => format!("({base} :hypothesis {})", branch(p))`, where `branch(p)` (line 435) prints the
  provenance's `branch`, or `None` where it has none;
- the state file's comment read it at `ein-render/src/sentence.rs:123`,
  `ProvKind::Hypothesis => (Origin::Hypothesis(p.branch.unwrap_or(0)), None)`, where a missing branch would also read 0;
- the node function writes every committed fact's provenance as `Prov::from_hypothesis(0, None)`, branch `Some(0)`,
  at `ein-infer/src/region.rs:1744`, `:1846` and `:2037` (and four census examples). So the 0 is written, not a
  missing value.

**The same field elsewhere**: a refuted claim explains a fact the state holds and the model does not list by its
provenance (`ein-infer/src/expect.rs:83`, `hypothesised in branch {b}`). On
[`programs/walks-claim.ein`](programs/walks-claim.ein) (the program of `01-Q-M1f.414` with its claim made wrong on
purpose), `ein test` prints (output, [`runs/walks-claim.test.today.txt`](runs/walks-claim.test.today.txt), exit 1):

```text
      the state holds (busy Bob), which the model does not list
        hypothesised in branch 0
```

**What H looks like**: `ein show hyps` lists the candidates in the universe's order, with no number, for the program
and for its saved node alike (Q-M1f.415 (a)): (busy Alice), (busy Bob), (busy Carol)
([`runs/busy.show-hyps.today.stdout.txt`](runs/busy.show-hyps.today.stdout.txt),
[`runs/busy.models-08.show-hyps.today.stdout.txt`](runs/busy.models-08.show-hyps.today.stdout.txt)); `ein solve -H` of a
saved node names its choices on the `commit` row ([`runs/busy.models-08.solve-H.today.txt`](runs/busy.models-08.solve-H.today.txt)).

**What pins today's text**: `ein-cli/tests/saved_files.rs:179` asserts the dump contains `(note (bad a) :hypothesis 0)`,
and `docs/ein/user/04-reference/10-answer-dump.md` (line 158) quotes it (read, not run). Any option but (a) edits both.

## 3. The options

Each mock is today's dump of `models/07.ein`, {(busy Bob), (busy Carol)}, with the two `:hypothesis` lines edited
(by a `sed` in [`rerun.sh`](rerun.sh)) and nothing else; each has its diff. This state separates (b) from (c): Bob
and Carol are H's second and third candidates, and the commit's first and second.

**(a) As it is**: `:hypothesis 0` on every committed fact; the number names nothing.
[`runs/busy.models-07.saturate-dump.today.txt`](runs/busy.models-07.saturate-dump.today.txt).

**(b) N names the candidate: its place in H**, in the universe's order as `ein show hyps` lists it, from 1. MOCK:
[`runs/busy.models-07.saturate-dump.MOCK.option-b-candidate-of-H.txt`](runs/busy.models-07.saturate-dump.MOCK.option-b-candidate-of-H.txt)
([diff](runs/busy.models-07.saturate-dump.MOCK.option-b-candidate-of-H.diff)):

```text
  (busy Bob :hypothesis 2)
  (busy Carol :hypothesis 3)
```

The same fact gets the same number in every state of the program and in its saved files; the node function would
have to carry the candidate's index into the provenance it writes. The claim's line would read *hypothesised,
candidate 2 of H*.

**(c) N names the fact's place in the file's `(commit …)`**, from 1. MOCK:
[`runs/busy.models-07.saturate-dump.MOCK.option-c-place-in-commit.txt`](runs/busy.models-07.saturate-dump.MOCK.option-c-place-in-commit.txt)
([diff](runs/busy.models-07.saturate-dump.MOCK.option-c-place-in-commit.diff)):

```text
  (busy Bob :hypothesis 1)
  (busy Carol :hypothesis 2)
```

Local to the file: (busy Bob) is 1 here and 2 in `models/05.ein`, {Alice, Bob}. A run of a program, where no file
lists a commit, would number by the state's `from {…}` instead.

**(d) No number**: the dump prints the origin's word alone; the mock writes `:commit`, the saved file's own form's
name. MOCK:
[`runs/busy.models-07.saturate-dump.MOCK.option-d-no-number.txt`](runs/busy.models-07.saturate-dump.MOCK.option-d-no-number.txt)
([diff](runs/busy.models-07.saturate-dump.MOCK.option-d-no-number.diff)):

```text
  (busy Bob :commit)
  (busy Carol :commit)
```

`:hypothesis` with no value is the other spelling of the same choice. The claim's line would read *committed*.

## 4. The default

None: the row is **the user's call** (*what `:hypothesis N` names, or whether the dump prints it*). Today it is (a).

## 5. Files

- [`rerun.sh`](rerun.sh) — every run of this page, the mocks made again from today's dump.
- [`programs/04_a_saved_node.ein`](programs/04_a_saved_node.ein) — copied from `tests/saved/`.
- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copied from `tests/saved/`: the program `04` saves.
- [`programs/busy.ein`](programs/busy.ein) — written for this review: states of two and three commitments.
- [`programs/walks-claim.ein`](programs/walks-claim.ein) — written for this review: a refuted claim that prints the branch.
- [`runs/04_a_saved_node.saturate-dump.today.txt`](runs/04_a_saved_node.saturate-dump.today.txt) — `ein saturate --dump` of the corpus's saved node.
- [`runs/busy.solve.today.txt`](runs/busy.solve.today.txt) — the read-out of `busy.ein`.
- [`runs/busy.dump-answer.today/`](runs/busy.dump-answer.today/) — today's saved files, eight.
- [`runs/busy.models-05.saturate-dump.today.txt`](runs/busy.models-05.saturate-dump.today.txt), [`-07`](runs/busy.models-07.saturate-dump.today.txt), [`-08`](runs/busy.models-08.saturate-dump.today.txt) — `ein saturate --dump` of three saved files.
- [`runs/today.saturate-dump.hypothesis.grep.txt`](runs/today.saturate-dump.hypothesis.grep.txt) — every `:hypothesis` line of those dumps.
- [`runs/busy.dump-answer.before-ein-a8d595eb/`](runs/busy.dump-answer.before-ein-a8d595eb/) — the earlier binary's state files.
- [`runs/busy.before-ein-a8d595eb.hypothesis-comments.grep.txt`](runs/busy.before-ein-a8d595eb.hypothesis-comments.grep.txt) — every `; hypothesis` comment in them.
- [`runs/busy.models-08.saturate-dump.before-ein-a8d595eb.txt`](runs/busy.models-08.saturate-dump.before-ein-a8d595eb.txt) — the earlier binary's dump of its own state file: no `:hypothesis`.
- [`runs/02_a_choice.dump-answer.before-ein-a8d595eb/`](runs/02_a_choice.dump-answer.before-ein-a8d595eb/) — the earlier state files of the program `04` saves.
- [`runs/busy.show-hyps.today.stdout.txt`](runs/busy.show-hyps.today.stdout.txt) / [`.stderr.txt`](runs/busy.show-hyps.today.stderr.txt), [`runs/busy.models-08.show-hyps.today.stdout.txt`](runs/busy.models-08.show-hyps.today.stdout.txt) / [`.stderr.txt`](runs/busy.models-08.show-hyps.today.stderr.txt) — `ein show hyps`, program and saved node.
- [`runs/busy.models-08.solve-H.today.txt`](runs/busy.models-08.solve-H.today.txt) — `ein solve -H` of the saved node.
- [`runs/walks-claim.test.today.txt`](runs/walks-claim.test.today.txt) — `ein test` of the refuted claim, with its exit code.
- [`runs/code-lines.grep.txt`](runs/code-lines.grep.txt) — the code lines: `sentence.rs`, `saturate.rs`, `expect.rs`, every `Prov::from_hypothesis(` call, `Prov::from_hypothesis` itself.
- `runs/busy.models-07.saturate-dump.MOCK.option-{b-candidate-of-H,c-place-in-commit,d-no-number}.{txt,diff}` — **MOCKS** of (b), (c), (d).

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/30-L-104/rerun.sh`: every
command below, run from this folder, the greps, the code lines and the three mocks with their diffs. The core
commands:

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/30-L-104
H=build/review-bin/ein-head-17085b27
A=build/review-bin/ein-a8d595eb; EA=$PWD/build/wt-rev-a8d/stdlib
$H saturate --dump $D/programs/04_a_saved_node.ein > $D/runs/04_a_saved_node.saturate-dump.today.txt
$H solve $D/programs/busy.ein > $D/runs/busy.solve.today.txt
$H solve $D/programs/busy.ein --dump-answer $D/runs/busy.dump-answer.today
for i in 05 07 08; do $H saturate --dump $D/runs/busy.dump-answer.today/models/$i.ein > $D/runs/busy.models-$i.saturate-dump.today.txt; done
EIN_STDLIB=$EA $A solve $D/programs/busy.ein --dump-answer $D/runs/busy.dump-answer.before-ein-a8d595eb
EIN_STDLIB=$EA $A saturate --dump $D/runs/busy.dump-answer.before-ein-a8d595eb/models/08.ein > $D/runs/busy.models-08.saturate-dump.before-ein-a8d595eb.txt
EIN_STDLIB=$EA $A solve $D/programs/02_a_choice_with_a_fact_inside_it.ein --dump-answer $D/runs/02_a_choice.dump-answer.before-ein-a8d595eb
$H show hyps $D/programs/busy.ein
$H show hyps $D/runs/busy.dump-answer.today/models/08.ein
$H solve $D/runs/busy.dump-answer.today/models/08.ein -H
$H test $D/programs/walks-claim.ein
grep -rn 'Prov::from_hypothesis(' ein.rs/crates --include='*.rs'
```

## 7. Found while building

- **The node function writes the 0; it does not leave the branch unset.** The ledger says the number is
  `sentence.rs`' `p.branch.unwrap_or(0)` *"which the node function leaves unset"*. `ein saturate --dump` does not read
  `sentence.rs`: it prints `branch(p)` (`saturate.rs:429`, `:435`), which would print `None` for an unset branch; it
  prints `0` because `region.rs` writes `Prov::from_hypothesis(0, None)`, branch `Some(0)`, for every committed fact.
  The effect the ledger states holds; its cause is the written 0.
- **A refuted claim prints the same field**, `hypothesised in branch 0` (`expect.rs:83`), which the ledger does not
  name; whatever is chosen for the dump bears on that line too.
- The ledger's *157 907 lines, all 0* is over the corpus's state files of the earlier format; it was not re-taken.
- `10-answer-dump.md` says the dump prints each fact *"in the order held; on the state above it is root's `(item a)`,
  then `(note (bad a) :hypothesis 0)` and `(flagged a :rule flag)`"*; the dump prints them grouped by origin and then
  by relation, `(flagged a :rule flag)` before `(note (bad a) :hypothesis 0)`
  ([`runs/04_a_saved_node.saturate-dump.today.txt`](runs/04_a_saved_node.saturate-dump.today.txt);
  `saturate.rs`' `facts_section`, *"insertion order preserved inside each group"*).
- **Re-taken with `ein-head-17085b27`** (the first look was `ein-head-b5fb8d70`): nothing moved. Every dump,
  `:hypothesis 0` line, show-hyps list, claim line and code line (`saturate.rs:429`, `region.rs:1744`, `:1846`,
  `:2037`, `expect.rs:83`, `sentence.rs:123`) is as it was.

## 8. Your call

- **(a)** as it is: `:hypothesis 0` on every committed fact. Today's state.
- **(b)** N is the candidate's place in H, as `ein show hyps` lists it: stable across a program's states.
- **(c)** N is the fact's place in the file's `(commit …)`: local to one file.
- **(d)** no number: the origin's word alone (`:commit`, or `:hypothesis` bare).
- No default. Every option but (a) changes `saved_files.rs:179` and `10-answer-dump.md`, and the claim's
  `hypothesised in branch 0` line with it.
