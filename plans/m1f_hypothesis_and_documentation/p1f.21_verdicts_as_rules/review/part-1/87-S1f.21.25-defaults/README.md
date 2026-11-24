# 87 · The defaults S1f.21.25 took

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.25_an_activator_that_names_no_relation_in_silence.md#record)

S1f.21.25 is built (`a117e80a`, `e802c611`, `02bcddc3`): your ruling on item 82, an activator that binds a relation
parameter to a fact activates nothing **and nothing says so**. These are the five choices it made that your ruling
did not. Each is built; a word from you changes it.

## What you should know first

- **The three lines are gone**: the load's for an activator a rule could derive, the load's for one the program
  writes, the run's for one a state holds. Over the corpus, 933 programs under `ein solve`, `ein saturate` and
  `ein show hyps`, old binary against new: **no exit code and no stdout moved**; the 15 runs that printed a line
  (5 programs × 3 commands) print none, and no run prints any `warning:` line. The gate is green on `02bcddc3`.
- **What stays**: no plan is compiled for such an activator; the reading of which positions hold a relation's name,
  for Q-M1f.46's refusal and the relations of the program (Q-M1f.522); fixtures 19 and 20, still flagged not
  monotone by the static check.
- **Item 82's programs**, run on master's release build: `zebra2-about-1.ein`, both `imply1-into-a-rule*.ein` and
  step 6 exit 0 with nothing on stderr and the same answers; step 6 still derives `(symmetric (H1 H2))`, which still
  activates nothing. (The programs are in `git log`, at `0f7190b9`.)

```sh
ein.rs/target/release/ein solve examples/features/19_derived_activator.ein 2>&1 >/dev/null   # the WARNING line alone: flagged, no other line
ein.rs/target/release/ein saturate examples/features/19_derived_activator.ein --dump | grep -E '\(r \(a b\)|^;;   r '   # the activator held, no plan of r
```

## The five

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.690](../../../../open_questions.md#q-m1f690--the-loads-warnings-and-the-clis-channel-go-with-the-lines): the load's warnings and the CLI's channel | `Program::warnings`, which held only these lines, goes, with `ein-cli`'s channel for them; what printed them now prints the load's `note:` lines alone (`say_notes`). Q-M1f.580 now speaks of every line a load or a run prints | keep the field and the channel, empty, for a later warning |
| 2 | [Q-M1f.691](../../../../open_questions.md#q-m1f691--q-m1f1-keeps-its-file-and-its-slug): Q-M1f.1's file | it keeps its file and slug, its title rewritten to start as the old one: *A rule that could derive an activator binding a relation parameter to a fact loads, and such an activator activates nothing in silence* | a title led by the activator, *An activator that binds a relation parameter to a fact activates nothing, and nothing says so*, in a file of that slug, every link moved |
| 3 | [Q-M1f.692](../../../../open_questions.md#q-m1f692--what-the-test-of-the-sixteen-shapes-holds-now): the test that read the lines | it keeps its sixteen shapes, each loading, and holds the positions through the relations of the program: a name passed where a relation's name is held is a relation of the program | the shapes alone, loading |
| 4 | [Q-M1f.693](../../../../open_questions.md#q-m1f693--the-fixtures-headers-keep-their-line-counts): the fixtures' headers | rewritten in as many lines as they had, so no position in them moves and no golden is re-banked | any length |
| 5 | [Q-M1f.694](../../../../open_questions.md#q-m1f694--what-cites-no-line-stays): what cites no line | stays: the diagnostics page's § 2.9 keeps its paragraph on the positions, which two other sections read, retitled *Which positions hold relation names*; the reasoning pages and five decisions that cite no line, unchanged | — |

## Also found

[V-101 to V-105](../../../findings.md#v-101--a-header-still-says-the-keyword-pair-in-a-guard-is-a-load-error): a
header that calls the keyword-pair refusal a load error (V-101), a test comment citing Q-M1f.1 for what it does not
decide (V-103) and a function left `pub` with one caller (V-104), each small and left for the next batch of code;
whether the calculator's closing rules may now name `std.math.positional`'s rules, open and not tried (V-102); the
ledger's entry for Q-M1f.1, fixed in the filing (V-105).

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 87. Blank keeps the five as built.
