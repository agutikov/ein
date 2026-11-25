# M2 — the evaluation of Ein

**What this page is.** The second of the milestone's [two main ideas](../README.md#the-two-main-ideas-the-user-2026-11-25),
in the user's words (2026-11-25): *"evaluate ein.rs in this auto-frmalization application, find/highlight it's
weaknesses, strength, gaps, bugs and limits, and then go to next iteration of ein/ein.rs development with this
feedback"*. EinAf is the first application that drives the kernel as a user would, with programs nobody wrote by
hand; this page is what it finds of the kernel, kept for the iteration of Ein that follows.

**How it is kept.**

- **An entry is one finding**, under an id `E-n` that is kept and never reused, in one of the user's five kinds:
  a **strength** (what served the application as it is), a **weakness** (what works as designed and works against
  the application), a **gap** (what the application needs and Ein does not have), a **bug** (what Ein does wrong
  by its own rules), a **limit** (how far it goes: a size, a time, a class of program).
- **An entry says** what was found; the evidence, a program and a command, or the stage's record; what it cost the
  application; and its owner where it has one (a followup, a decision that makes it the design, a stage of this
  plan). An entry with no owner is the next iteration's to weigh.
- **Every stage that drives the kernel adds what it found**, and a phase shows the page at its close. A bug that
  is a wrong answer is filed in [the wrong-answer register](../../wrong_answers/README.md) as well, by its rules.
- **One file an entry**, `<topic>/E-<n>-<slug>.md`, under the topic it is about: its id and its bold sentence as
  the title; its kind, its status (open, or closed and by what; a strength stands), where it was filed and its
  owner; then the cells of its row, each a section, as they were written. This page is the index. **The ids are one
  sequence for the milestone and its phases**, and the next free id is **E-104**.
- **An entry filed by a phase lives in the phase's `evaluation/`** (every phase's folder has one) until the phase
  closes. Then, if it is open or a strength, it moves here under the same topic; a closed one goes with the phase's
  folder, where `git log` keeps it.
- **At each level's gate the page is written up and shown**, and the user says whether EinAf goes on or an
  iteration of Ein comes first ([Q-M2.15](../open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)).
  At the milestone's close what is open moves to the plan of that iteration, or to the followups.
- **A gap or a bug may be closed inside M2**, by the stage that needs it, where the change leaves Ein's syntax
  and semantics as they are: a feature (a log, a trace, a measurement, a timeout, a limit) or a bug fix, the
  user's ruling on [Q-M2.16](../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md). The entry then
  says which stage closed it. What would change the language is recorded and waits for the iteration of Ein.

**Seeded 2026-11-25, before any stage ran**: E-1 to E-32 from [the analysis](../analysis/README.md), E-33 on from
the re-plan, whose writers ran the binary on corpus programs and on small probes to check what the plan says.
They are findings of reading and of probing, on `458bf90b`, not yet of the application at work. Each cites the
section, the command or the program that has its evidence; a number was taken that day. **A probe a finding
rests on is kept**, under [`analysis/probes/`](../analysis/probes/), and each was run again before its entry was
written.

**From the first stage on, an entry is a finding of the application at work**, and lives in the folder of the
phase that filed it until the phase closes: E-62 to E-67 are
S2.1.1's, on `0effa7b0`, the first time EinAf linked
the crates and read the kernel's values; its record has the commands. E-68 to E-93 are S2.1.2's, filed at its end on `e74139cf`: the twenty findings its two halves recorded, each command re-run on that commit, and what its last half
found. E-94 to E-101 are S2.1.3's, filed at its end on
`0ab1aef0`: what building the feedback object from the library took, what the object and the binary said of the same
runs, and what the probes of the bounded call did. E-102 and E-103 are S2.1.4's,
filed at its end on `df75643c`: what was left of a deep term's cost once its printers kept no call stack, and the one
module path its fix left the binary's.

## Topics

| topic | what it holds | here | in a phase's folder |
|---|---|---:|---:|
| [`answer/`](answer/) | R and what is said of it: the outcomes, the questions, cores, claims, counts, flagged programs | 26 | 0 |
| [`diagnostics/`](diagnostics/) | what the kernel says beside an answer or instead of one: refusals, kinds, messages, places, the summary's lines | 17 | 0 |
| [`bounds/`](bounds/) | how far and how long a run goes: termination, the clock, the caps, memory, time | 20 | 0 |
| [`library/`](library/) | Ein as a library: the API, building against it, embedding it, what its values hold | 7 | 0 |
| [`language/`](language/) | the language: relations, modules, imports, names | 4 | 0 |
| [`stdlib/`](stdlib/) | the standard library's theories, and how a program selects one | 6 | 0 |
| [`hypotheses/`](hypotheses/) | H and the hypothesis mechanisms, the blind enumerator among them | 4 | 0 |
| [`natural-language/`](natural-language/) | text: `:why`, `:source`, `:goal-text`, a value's words, a sentence for an answer | 6 | 0 |

## Strengths

| id | title | topic | status | owner |
|---|---|---|---|---|
| [E-1](answer/E-1-the-kernel-says-its-own-outcomes-and-none-is-silent.md) | The kernel says its own outcomes, and none is silent | `answer` | standing (a strength) | — |
| [E-2](answer/E-2-a-program-states-its-answer-and-the-kernel-checks-it.md) | A program states its answer and the kernel checks it | `answer` | standing (a strength) | — |
| [E-3](answer/E-3-each-question-does-the-least-work-it-needs.md) | Each question does the least work it needs | `answer` | standing (a strength) | — |
| [E-4](answer/E-4-a-reference-by-brute-force.md) | A reference by brute force | `answer` | standing (a strength) | — |
| [E-5](natural-language/E-5-a-program-explains-itself-in-its-own-words.md) | A program explains itself in its own words | `natural-language` | standing (a strength) | — |
| [E-6](hypotheses/E-6-a-sentence-with-two-readings-is-a-pair-of-hypotheses.md) | A sentence with two readings is a pair of hypotheses | `hypotheses` | standing (a strength) | — |
| [E-7](stdlib/E-7-a-theory-is-selected-by-a-fact.md) | A theory is selected by a fact | `stdlib` | standing (a strength) | — |
| [E-8](library/E-8-the-library-returns-the-answer-as-values.md) | The library returns the answer as values | `library` | standing (a strength) | — |
| [E-9](answer/E-9-the-semantics-are-written-as-mathematics.md) | The semantics are written as mathematics | `answer` | standing (a strength) | — |
| [E-10](answer/E-10-a-program-that-is-not-monotone-is-detected-before-it-is.md) | A program that is not monotone is detected before it is answered | `answer` | standing (a strength) | — |
| [E-33](stdlib/E-33-an-order-as-a-bijection-onto-named-ranks.md) | An order as a bijection onto named ranks | `stdlib` | standing (a strength) | — |
| [E-34](answer/E-34-a-question-that-looks-closed-world-needs-no-absent.md) | A question that looks closed-world needs no `absent` | `answer` | standing (a strength) | — |
| [E-35](answer/E-35-maximal-counts-the-full-answers-exactly-where-a-description.md) | `--maximal` counts the full answers exactly where a description is stopped | `answer` | standing (a strength) | — |
| [E-36](answer/E-36-the-answer-does-not-depend-on-names.md) | The answer does not depend on names | `answer` | standing (a strength) | — |
| [E-49](answer/E-49-a-dead-root-is-described-for-the-price-of-roots-saturation.md) | A dead root is described for the price of root's saturation | `answer` | standing (a strength) | — |
| [E-50](diagnostics/E-50-a-claim-over-a-relation-the-program-does-not-have-is-refused.md) | A claim over a relation the program does not have is refused at the load, with the relation named | `diagnostics` | standing (a strength) | — |
| [E-66](answer/E-66-the-seven-outcomes-are-read-off-values-and-eight-questions.md) | The seven outcomes are read off values, and eight questions agree on every corpus program | `answer` | standing (a strength) | — |
| [E-68](diagnostics/E-68-every-line-a-run-prints-is-a-value-of-its-summary.md) | Every line a run prints is a value of its summary | `diagnostics` | standing (a strength) | — |
| [E-94](library/E-94-the-library-and-the-binary-say-the-same-of-every-run-einaf.md) | The library and the binary say the same of every run EinAf compared | `library` | standing (a strength) | — |

## Weaknesses

| id | title | topic | status | owner |
|---|---|---|---|---|
| [E-11](stdlib/E-11-a-theory-selected-under-a-name-the-file-does-not-list-is.md) | A theory selected under a name the file does not list is inert, with no message | `stdlib` | open | none |
| [E-12](language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md) | An undeclared relation, and a form of a declared relation at another count, load in silence | `language` | open | the design: [Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md), [Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md) |
| [E-13](hypotheses/E-13-a-program-that-names-no-hypothesis-mechanism-is-answered-by.md) | A program that names no hypothesis mechanism is answered by the blind enumerator | `hypotheses` | open | none |
| [E-14](answer/E-14-the-kernels-word-is-not-the-problems-class.md) | The kernel's word is not the problem's class | `answer` | open | [F26](../../followups/f26_consistency_and_completeness.md) |
| [E-15](answer/E-15-one-solution-has-two-readings.md) | *One solution* has two readings | `answer` | open | [Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) |
| [E-16](answer/E-16-a-core-is-an-explanation.md) | A core is an explanation | `answer` | open | [F22](../../followups/f22_exact_core_past_the_cap.md) |
| [E-17](bounds/E-17-what-a-run-costs-depends-on-the-question-asked-of-it.md) | What a run costs depends on the question asked of it | `bounds` | open | none |
| [E-18](stdlib/E-18-a-fact-beside-its-negation-is-a-contradiction-only-where-the.md) | A fact beside its negation is a contradiction only where the program imports `std.contradiction` | `stdlib` | open | the design: [Q-M1f.557](../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md) |
| [E-19](bounds/E-19-events-multiplies-the-wall-clock-three-to-six-times.md) | `--events` multiplies the wall clock three to six times | `bounds` | open | none |
| [E-37](answer/E-37-open-over-an-empty-r-does-not-tell-a-problem-with-no.md) | `Open` over an empty R does not tell a problem with no solution from a formalization that is unfinished | `answer` | open | [F26](../../followups/f26_consistency_and_completeness.md) |
| [E-38](stdlib/E-38-transitive-r-with-irreflexive-r-does-not-refute-a-cycle.md) | `(transitive R)` with `(irreflexive R)` does not refute a cycle | `stdlib` | open | none |
| [E-39](hypotheses/E-39-the-answer-does-not-say-who-wrote-the-candidates.md) | The answer does not say who wrote the candidates | `hypotheses` | open | none |
| [E-40](bounds/E-40-a-question-about-a-line-can-cost-more-than-the-description.md) | A question about a line can cost more than the description of R | `bounds` | open | none |
| [E-41](answer/E-41-a-lower-bound-is-written-as-a-plain-number.md) | A lower bound is written as a plain number | `answer` | open | none |
| [E-51](natural-language/E-51-a-core-holds-facts-that-have-no-sentence.md) | A core holds facts that have no sentence | `natural-language` | open | [F22](../../followups/f22_exact_core_past_the_cap.md), [F24](../../followups/f24_written_provenance.md) |
| [E-52](diagnostics/E-52-a-parse-errors-repairs-are-in-the-parsers-words.md) | A parse error's repairs are in the parser's words | `diagnostics` | open | none |
| [E-53](answer/E-53-a-flagged-program-still-prints-a-verdict-word-under-single.md) | A flagged program still prints a verdict word under `--single` | `answer` | open | the design: [Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md) |
| [E-54](answer/E-54-a-fact-written-with-rule-or-using-leaves-every-core.md) | A fact written with `:rule` or `:using` leaves every core | `answer` | open | F24 |
| [E-60](hypotheses/E-60-the-answer-turns-on-one-line-that-states-the-lattice.md) | The answer turns on one line that states the lattice | `hypotheses` | open | none |
| [E-69](diagnostics/E-69-one-line-two-sections.md) | One line, two sections | `diagnostics` | open | none |
| [E-70](diagnostics/E-70-one-walk-makes-the-lines-of-three-sections-split-by-the.md) | One walk makes the lines of three sections, split by the form's position | `diagnostics` | open | none |
| [E-71](diagnostics/E-71-a-lines-place-is-not-always-in-front.md) | A line's place is not always in front | `diagnostics` | open | none |
| [E-72](diagnostics/E-72-a-parse-error-inside-a-refusal-may-be-a-modules.md) | A parse error inside a refusal may be a module's | `diagnostics` | open | none |
| [E-85](bounds/E-85-the-cap-counts-firings-an-engine-count.md) | The cap counts firings, an engine count | `bounds` | open | none |
| [E-86](bounds/E-86-ein-test-asks-a-cut-file-twice.md) | `ein test` asks a cut file twice | `bounds` | open | none |
| [E-87](bounds/E-87-region-stopped-why-has-two-values-no-summary-reaches.md) | `region.stopped.why` has two values no summary reaches | `bounds` | open | none |
| [E-92](diagnostics/E-92-a-schemas-number-is-written-into-an-options-help.md) | A schema's number is written into an option's help | `diagnostics` | open | the user's to rule; the default is [S2.1.2](../defaults/p2.1.md)'s record; [Q-M2.19](../open_questions/kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md) |
| [E-93](diagnostics/E-93-a-fixture-banks-one-finding-of-two.md) | A fixture banks one finding of two | `diagnostics` | open | none |
| [E-103](diagnostics/E-103-a-standard-library-module-is-named-by-the-path-the-binary-found.md) | A standard library module is named by the path the binary found it under | `diagnostics` | open | none |

## Gaps

| id | title | topic | status | owner |
|---|---|---|---|---|
| [E-20](diagnostics/E-20-no-machine-surface-carries-a-diagnostic.md) | No machine surface carries a diagnostic | `diagnostics` | closed by S2.1.2 (`e74139cf`) | S2.1.2 |
| [E-21](diagnostics/E-21-four-load-side-errors-are-strings.md) | Four load-side errors are strings | `diagnostics` | closed by S2.1.2 (`62a8ee45`) | S2.1.2 |
| [E-22](natural-language/E-22-the-answers-line-is-in-no-key-of-ein-summary-2.md) | The answer's line is in no key of `ein-summary/2` | `natural-language` | open | none; [M1p](../../m1p_ein_to_natural_language/README.md)'s ground |
| [E-23](natural-language/E-23-a-value-has-no-text-of-its-own.md) | A value has no text of its own | `natural-language` | open | EinAf's formatters ([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)) meanwhile; M1p's P1p.1 |
| [E-24](answer/E-24-nothing-computes-the-difference-of-two-states.md) | Nothing computes the difference of two states | `answer` | open | none |
| [E-25](answer/E-25-a-whole-state-is-in-no-json-key.md) | A whole state is in no JSON key | `answer` | open | none |
| [E-26](bounds/E-26-no-surface-carries-a-time-or-a-memory-figure.md) | No surface carries a time or a memory figure | `bounds` | open | none |
| [E-27](stdlib/E-27-no-one-catalogue-says-how-each-library-property-is-spelled.md) | No one catalogue says how each library property is spelled to be active | `stdlib` | open | [S2.2.4](../p2.2_formalizer/s2.2.4_passes.md) builds one for the prompt |
| [E-28](diagnostics/E-28-no-analysis-of-a-programs-shape.md) | No analysis of a program's shape | `diagnostics` | open | none |
| [E-42](natural-language/E-42-a-negative-answer-has-no-sentence.md) | A negative answer has no sentence | `natural-language` | open | none; [M1p](../../m1p_ein_to_natural_language/README.md)'s ground |
| [E-43](answer/E-43-no-surface-says-how-deep-or-how-large-a-derivation-is.md) | No surface says how deep or how large a derivation is | `answer` | open | none |
| [E-44](answer/E-44-the-sizes-of-r-are-a-descriptions-keys-only.md) | The sizes of R are a description's keys only | `answer` | open | none |
| [E-55](answer/E-55-a-claim-that-names-an-atom-the-program-does-not-have-fails.md) | A claim that names an atom the program does not have fails like a wrong answer | `answer` | open | none |
| [E-56](library/E-56-a-rows-value-reaches-a-caller-of-the-library-as-text.md) | A row's value reaches a caller of the library as text | `library` | open | none; [M1p](../../m1p_ein_to_natural_language/README.md)'s ground |
| [E-57](bounds/E-57-the-clocks-abort-hands-back-nothing-of-what-was-established.md) | The clock's abort hands back nothing of what was established | `bounds` | open | none |
| [E-58](diagnostics/E-58-the-rules-dependency-graph-covers-saturation-rules-only.md) | The rules' dependency graph covers saturation rules only | `diagnostics` | open | none |
| [E-73](diagnostics/E-73-a-refusal-about-a-top-level-form-has-no-place.md) | A refusal about a top-level form has no place | `diagnostics` | open | [F23](../../followups/f23_diagnostics_in_eins_words.md)'s D-2 and D-22 |
| [E-75](diagnostics/E-75-two-refusals-only-a-library-caller-meets-which-no-page.md) | Two refusals only a library caller meets, which no page states | `diagnostics` | open | none |
| [E-82](bounds/E-82-a-run-the-cap-stops-hands-back-nothing-it-established.md) | A run the cap stops hands back nothing it established | `bounds` | open | none |
| [E-89](diagnostics/E-89-a-line-with-no-kind-stays-off-every-summary.md) | A line with no kind stays off every summary | `diagnostics` | open | none |
| [E-91](natural-language/E-91-a-cores-sentences-stand-beside-roots-core-alone.md) | A core's sentences stand beside root's core alone | `natural-language` | open | none |
| [E-95](library/E-95-no-library-value-says-which-kernel-it-is.md) | No library value says which kernel it is | `library` | open | none |
| [E-96](library/E-96-the-clocks-abort-is-the-binarys-line-not-a-library-value.md) | The clock's abort is the binary's line, not a library value | `library` | open | none |
| [E-97](library/E-97-a-runs-diagnostics-are-a-value-only-of-the-rendered-read-out.md) | A run's diagnostics are a value only of the rendered read-out, and their order is the binary's | `library` | open | none |
| [E-100](bounds/E-100-nothing-bounds-a-calls-memory-in-process.md) | Nothing bounds a call's memory in process | `bounds` | open | none |

## Bugs

None is seeded: the analysis ran the binary on corpus programs and met no answer that was wrong by Ein's own
rules. The wrong answers known before this milestone are [the register](../../wrong_answers/register.toml)'s. None
below is a wrong answer.

None here: the bugs filed so far, E-62, E-65, E-74, E-76, E-79, E-88, were P2.1's, each closed in the phase, and went
with its folder at its close ([below](#in-a-phases-folder)).

## Limits

| id | title | topic | status | owner |
|---|---|---|---|---|
| [E-29](bounds/E-29-ein-oracle-refuses-above-16-candidates.md) | `ein oracle` refuses above 16 candidates | `bounds` | open | — |
| [E-30](bounds/E-30-a-description-stops-at-the-default-budget-of-2-20-facts.md) | A description stops at the default budget of 2²⁰ facts | `bounds` | open | — |
| [E-31](answer/E-31-of-a-flagged-program-nothing-is-claimed.md) | Of a flagged program nothing is claimed | `answer` | open | — |
| [E-32](language/E-32-names-are-ascii-and-there-are-no-numbers.md) | Names are ASCII, and there are no numbers | `language` | open | — |
| [E-45](bounds/E-45-nothing-in-a-solve-stops-a-saturation-that-has-no-fixpoint.md) | Nothing in a solve stops a saturation that has no fixpoint | `bounds` | closed by S2.1.2, T2.1.2.5b (`83b82833`) | S2.1.2 |
| [E-46](answer/E-46-whether-a-closed-world-rule-is-flagged-depends-on-what-is.md) | Whether a closed-world rule is flagged depends on what is hypothesised beside it | `answer` | open | — |
| [E-47](language/E-47-a-module-is-found-from-the-programs-directory-and-is-refused.md) | A module is found from the program's directory, and `..` is refused | `language` | open | — |
| [E-48](answer/E-48-an-ambiguous-answer-is-not-short.md) | An ambiguous answer is not short | `answer` | open | — |
| [E-59](language/E-59-a-module-comes-whole.md) | A module comes whole | `language` | open | — |
| [E-61](bounds/E-61-the-ein-binary-does-not-start-under-a-cap-on-its-address.md) | The `ein` binary does not start under a cap on its address space | `bounds` | open | — |
| [E-63](library/E-63-a-program-outside-the-engines-workspace-builds-the-crates-a.md) | A program outside the engine's workspace builds the crates a second time | `library` | open | — |
| [E-80](bounds/E-80-events-grows-quadratically-on-a-term-that-deepens-at-every.md) | `--events` grows quadratically on a term that deepens at every firing | `bounds` | open | — |
| [E-81](bounds/E-81-the-default-cap-costs-about-a-gigabyte-on-a-runaway-program.md) | The default cap costs about a gigabyte on a runaway program | `bounds` | open | — |
| [E-83](bounds/E-83-a-program-with-no-fixpoint-cannot-be-a-corpus-program.md) | A program with no fixpoint cannot be a corpus program | `bounds` | open | — |
| [E-84](bounds/E-84-four-readers-of-root-take-no-bound.md) | Four readers of root take no bound | `bounds` | open | — |
| [E-98](bounds/E-98-the-clock-is-not-read-where-nothing-fires-and-the-cap-counts.md) | The clock is not read where nothing fires, and the cap counts only firings | `bounds` | open | — |
| [E-102](bounds/E-102-a-deep-fact-is-rendered-and-drawn-in-time-that-grows-with-the.md) | A deep fact is rendered and drawn in time that grows with the square of its depth, after the run and outside the clock | `bounds` | open | — |

## In a phase's folder

One line a phase whose `evaluation/` holds entries, so that every id is found from here.

- **E-62 to E-103 were P2.1's**: E-62 to E-67 filed by S2.1.1, E-68 to E-93 by S2.1.2, E-94 to E-101 by S2.1.3, E-102
  and E-103 by S2.1.4. At the phase's close, 2026-11-26, the twenty-nine open or standing moved here under their topics
  (the strengths E-66, E-68 and E-94; the weaknesses E-69 to E-72, E-85 to E-87, E-92, E-93 and E-103; the gaps E-73,
  E-75, E-82, E-89, E-91, E-95 to E-97 and E-100; the limits E-63, E-80, E-81, E-83, E-84, E-98 and E-102), and the
  thirteen it closed went with its folder (`git log --diff-filter=D -- plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation`):
  E-62, E-64, E-65, E-67, E-74, E-76 to E-79, E-88, E-90, E-99 (as far as *faster than the square*, the rest E-102)
  and E-101.
- P2.2 to P2.10: none yet.
