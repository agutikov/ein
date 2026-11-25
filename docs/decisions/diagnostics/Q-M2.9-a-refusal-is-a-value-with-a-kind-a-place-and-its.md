# Q-M2.9 — A refusal is a value with a kind, a place and its line

**Every refusal Ein makes of a program is a value of the library, not only a line.** A refusal at the parse or at
load is a list of findings, and each finding has three parts:

- its **kind**, one short name, which says which refusal it is;
- its **place**, the file, the line and the column, where the line names one, and none where it does not: a line
  that ends `at None` has no place ([Q-M1a.6](Q-M1a.6-a-loader-message-about-a-top-level-form-names-no.md)), and
  no position is invented for it;
- its **message**, the line as `ein` prints it, byte for byte, its place in front where the line carries one.

What a command prints is made from the findings and is what it printed before they were values: a load's lines
`; `-joined behind `kb load error: `, a parse's errors one under another with their notes and repairs. No message,
exit code or stream moves for it, and no answer.

**The vocabulary is the reference's.** A refusal's kind is a section of
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md), named beside the section that states it, one
kind a section: `parse` for the parse's report whatever its note says, `not-ascii`, `nesting-limit`, `unreadable`,
`claim`, `obligation`, and on to `derivation-cycle`. A line two sections list takes the kind of the section that
states the check that makes it: a goal's refusal by a check a rule's `:match` gets keeps that check's kind. A
section that states no refusal has no kind, and a refusal no section stated was given one, so that every refusal a
program can meet has a section; a `.einb` container a build cannot read, by its magic, its version, its digest or a
section, is a file that cannot be read, `unreadable`. The lines a load and a run write beside an answer have kinds
too, in [`08-read-out.md` § 10](../../ein/user/04-reference/08-read-out.md#10-stderr): the static check's warning
(`not-monotone`), a partial run (`partial`), a claim not checked (`claim-not-checked`) and one refuted
(`claim-failed`), the clock's abort (`deadline`), a container written under other inputs (`stale-container`), and
the hypothesis mechanisms' notes (`mechanisms`, the kind of their refusal); a file the run was asked to write and
could not is § 2.4's `artefact`. Each diagnostic has a **severity**: `error` where the exit code says so (a
refusal, a refuted claim, the clock's abort) and for an artefact's failure, whose line says `error:` and whose run
keeps its exit code; `warning` and `note` where no answer and no exit code moves.

**The mechanisms' judgement is the library's.** Two of the program's hypothesis mechanisms that disagree with the
`open` facts root holds refuse it
([Q-M1f.245](../hypotheses/Q-M1f.245-the-hypothesis-mechanisms-are-judged-at-load-on.md)); the judgement reads
root's fixpoint, which the parse and the load do not compute, so it is a step after the load,
`ein_infer::mechanisms::judge_loaded`, which every command takes and a program that links Ein takes alike, its
conflicts and its notes findings of the kind `mechanisms`.

**What the machine surfaces carry.**

- **The run summary**, `--json-summary` of `ein solve`, `ein oracle` and `ein test`, ends with one key,
  `diagnostics`: every line the run printed on stderr, as a value, `{kind, severity, message, place}`, in the
  order stderr printed it, the load's notes last as stderr prints them, and `[]` where the run said nothing
  ([`08-read-out.md` § 11.3](../../ein/user/04-reference/08-read-out.md#113-diagnostics-what-the-run-said-as-values)).
- **It is written wherever a run gets far enough to know where to write it.** A program refused at the parse, at
  the load, by the judgement or at a saved file's node, and a run the clock ended, write a summary with no answer:
  `region`, `verdict`, `queries`, `claim`, `work`, `monotonicity`, `leftover` and `root` `null`, `asked` the question
  where the command chose one, `config` the program's where it loaded, and `diagnostics` why. A run that ends in an
  engine's error, which no diagnostic of Ein's names, and `ein oracle`'s refusal of a cube past its limit write
  none. `--dump-answer` writes nothing for a run with no answer.
- **`ein test --json-report`'s row in `error` carries its `diagnostics`**: a file that did not load, its findings;
  a claim's row of a file whose run the clock ended, the deadline's; `[]` where an engine's error ended it. Every
  other row's are `null`.
- **A core's sentences stand beside it**: `region.core_sources`, for each fact of `region.core`, in its order, the
  `:source` it was written with or `null`, and `null` where the core is.

Streams and exit codes do not move: an artefact option still changes neither stream nor the exit code. The keys
are added under `ein-summary/2` and `ein-test-report/6`: the page defines the summary's number by its one shape,
every key on every run and `null` where it does not apply, which one more key on every run keeps. No option is
added for them.

**Why.** A program that calls Ein cannot tell one refusal from another but by reading text meant for a person, and
text is the part a change of wording moves. The kinds are the refusals Ein already states, one a section of its
reference, so the vocabulary is the language's and grows only where the language does: a kind is not a class of
mistake Ein has no notion of. The message stays the normative line and the place stays inside it, because the line
is what a person reads and what the reference states; the place beside it is the same position as a value, so a
caller reads it without parsing the line. A line that names no place gets none here, since giving one would move
every line it is part of. A summary written for a run with no answer is the one artefact that says why to a caller
that reads files, and a summary's absence said nothing a caller could tell from a usage error.

**Held by.** `ein-core/tests/diagnostic_kinds.rs`: `the_pages_and_the_code_name_the_same_kinds` (every kind of
`ein_core::Kind` is named on one of the two pages, and every name there is a kind), `one_kind_is_one_section` and
`each_kind_is_documented_at_its_section`; the unit tests of `ein_core::diagnostic`; `ein-ir/src/macros.rs`'
`arity_is_counted_over_every_argument` and `a_self_expanding_macro_hits_the_depth_cap`, a finding's kind and its
place; every `.expected` fixture under `examples/broken/`, which holds each refusal's printed line where its
findings are values (`ein-ir/tests/load_semantics.rs`, `imports_semantics.rs`, `grammar_decisions.rs`); and the
worked example of [`04-api.md`](../../ein/ein.rs/04-api.md#3-worked-example), which reads a refusal by its kind
(`ein-cli/tests/embedding.rs`). The surfaces: `ein-cli/tests/summary_diagnostics.rs`, every refused fixture under
`examples/broken/` a summary with no answer whose findings are of kinds the page names and hold its `.expected`,
every corpus program's summary what its stderr printed, root alone, the clock's summary under `ein solve`, `ein
oracle` and `ein test`, a report row in `error`, and the page's table of a diagnostic's fields;
`ein-cli/tests/corpus_cli.rs`, every declared `solve` run of the corpus held to the same, kind by kind
(`tests/said/mod.rs`); `ein-cli/tests/summary_properties.rs`'
`the_diagnostics_and_a_cores_sources_have_one_shape_on_every_arm`; `ein-cli/tests/bounded_saturation.rs`, the clock's
summary and the cap's note; `ein-cli/tests/read_out_reference.rs`, the owners table naming `diagnostics` and the
fields table `region.core_sources`. The pages: [`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md),
each section's `**Kind:**`, [`08-read-out.md` § 10 and § 11.3](../../ein/user/04-reference/08-read-out.md#10-stderr),
[`05-cli.md`](../../ein/user/04-reference/05-cli.md) and
[`10-answer-dump.md`](../../ein/user/04-reference/10-answer-dump.md).
