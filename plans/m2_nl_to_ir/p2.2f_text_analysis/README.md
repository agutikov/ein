# P2.2f — The text analysed by trained networks before it is formalized: AMR first

**Status:** **planned 2026-11-27** on the user's word, *"Ok, add new phase for investigation of this approach,
especially AMR"*; **started the same day** on the user's word, *"start p2.2f in worktree"*, and, asked whether
the stage's agent or the user makes the environment, *"a, let the agent create build/nlp and start p2.2f"*. Its
stages run one by one, each in a subagent in a git worktree of its own whose branch is merged into `master` when
the stage ends, beside [P2.2e](../p2.2e_kernel_fixes/README.md), which runs in the main checkout. S2.2f.1 and
S2.2f.2 run; **S2.2f.2a** (AMR and spaCy read by rules), added on the user's word, runs no model and waits for nothing but the user's word; **S2.2f.3 waits** for [S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md),
whose phase is not started, so the phase stops there for the user, or at S2.2f.1 if no tool passes the check.
**Ids reserved for it while it runs beside P2.2e**, so that two branches never take one id: L-M2.40 to L-M2.49,
Q-M2.50 to Q-M2.59, E-140 to E-149; the milestone's sequence on `master` goes on below them.
**S2.2f.1 is done (2026-11-27)**: three AMR parsers and two UD parses run here within 4 GB of the card each and are
right on 50 to 56 of the check's 68 cells, where the model's step 1 was right on 43 at best; they go on, and so does
the phase, to S2.2f.2 ([the check's report](check/README.md), [the stage's record](s2.2f.1_the_check.md#record)).
**S2.2f.2 is done (2026-11-27)**: the parsers that passed, recorded for the six texts under `einaf/tests/fixtures/analysis/`, are written for step 1 as text (PENMAN, the default, or a node a line) and as Ein facts the kernel loads and draws, and EinAf gives one to step 1 (`--analysis`), its tests replaying them with no parser and no model ([the stage's record](s2.2f.2_the_analysis_written.md#record), [the page](../../../docs/einaf/09-analysis.md)); the phase stops for the user at S2.2f.3, which waits for S2.2c.1. The environment is `build/nlp/` of the main checkout, made by [`tools/setup.sh`](tools/setup.sh).
**Depends on:** [P2.2](../p2.2_formalizer/README.md), closed: the steps by hand of six texts
([`docs/einaf/04-contract.md` § 7](../../../docs/einaf/04-contract.md#7-the-steps-by-hand), `einaf/contract/`), which
are the reference a reading is checked against, and step 1 with the model, whose failures are what this phase asks
about ([S2.2.2's record](../p2.2_formalizer/s2.2.2_model_runtime.md#record), T2.2.2.8;
[S2.2.4's runs](../p2.2_formalizer/steps/README.md)). For its runs with the model,
[S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md), the stop on a loop.
**Informs:** [P2.2d](../p2.2d_formalizer_corrected/README.md), what step 1 is given;
[P2.2b](../p2.2b_guided_thinking/README.md), the same question asked of the model's own thinking;
[S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md), whose input-side arm, link grammar until
[Q11](../open_questions/formalizer/Q11-link-grammar-value.md)'s ruling, is what this phase finds feasible, if
anything.
**Changes:** EinAf (`einaf/`, `docs/einaf/`) and the plans; Ein only where a finding of [the evaluation](../evaluation/README.md)
needs a fix, and none is planned.
**Rulings it rests on:** [Q11](../open_questions/formalizer/Q11-link-grammar-value.md), the same day: no link grammar
and no strictly pre-defined grammar; the model is `qwen3.5:4b`
([Q-M2.5](../open_questions/model/Q-M2.5-local-model-choice.md)); EinAf is Rust
([Q-M2.7](../open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)).

## The user's words (2026-11-27)

The question:

> A different question about additional steps in formalization. link-grammar and NLTK needs some grammars
> packages. Can LLM do the syntactic/semantic analisys instead? Producing some markup of the sentence and some
> explanations about what every word in a sentence means and what other words it is connected and how. Or maybe
> there are some ready-to-use special neural-netorks-powered tools for that (like special NNs in speach
> recognition, sythesis, silence detection etc.)

And, on the coordinator's answer (the model can write such an analysis, but it is not independent of the misreading
it would correct; trained parsers exist at every layer, and AMR is the nearest to what a formalization needs):

> Ok, add new phase for investigation of this approach, especially AMR; everything that fits 8GB VRAM is ok, for
> now speed is not main question for EinAF, main question is feasibility. Q11 - link-grammar will not be used, any
> strictly pre-defined grammars will not be used. make new phase for that with check check as forst stage

## Goal

**Is it feasible, on this host, to give the formalizer a trained network's analysis of the text, AMR first, and
does that analysis carry what step 1 gets wrong?** Feasible means four things, each a yes or a no with its
evidence: the tools install and run here within 8 GB of the GPU's memory; they read the six texts right where the
model read them wrong; EinAf can hand what they write to the model, as text and as Ein facts; and step 1 given it
does better on those cases than step 1 without it. **Speed is not a question here** (the user): a tool that is
slow and right is feasible.

## What P2.2 found that this phase starts from

- **Step 1 fails on the reading of the sentence, not on Ein.** Thinking off, six of six replies load and one
  matches the hand: on the Zebra puzzle no `:source`, *Old Gold* split into two arguments, the passives made
  active; on the seating, placements no sentence states; on the word problems, a number taken out of its fact or
  written as a name ([S2.2.2's record](../p2.2_formalizer/s2.2.2_model_runtime.md#record), T2.2.2.8).
- **A polarity and a kind go wrong with nothing to see them**
  ([L-M2.18](../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)), and the
  model's Zebra program has seven independent defects, of which the kernel names two
  ([L-M2.24](../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md)).
- **An active and a passive verb are one relation read both ways** (the user, at S2.2.0a: `std.algebra`'s
  `converse`), and the model writes the passive as a second, active relation.

## Why AMR first

An Abstract Meaning Representation is a sentence's meaning as a rooted graph: concepts (a verb by its sense, a
noun, a name) and the roles between them (`:ARG0`, `:ARG1`, `:location`, `:polarity -`). Against the failures above:
**negation is a node of its own**; **the active and the passive of one verb are one frame with the same roles**;
a multi-word name is one `name` node; a number is a value of its concept; coreference inside a sentence is one
node reached twice. Its usual notation, PENMAN, is an S-expression, near Ein's own facts
([M1k's P1k.20](../../m1k_notations_as_facts/p1k.20_s_expressions_as_facts/README.md) asks how such notations
read as facts). What it does not give: coreference across sentences, the alignment of a node to the words it
came from (an aligner adds it), and anything the puzzle's genre does that its training text did not
([`docs/lib/10-nlp-semantic-parsing.md`](../../../docs/lib/10-nlp-semantic-parsing.md) has the layers).

**Beside it**, the analyses S2.2f.1 surveys: a Universal Dependencies parse (part of speech, lemma, a head and a
relation for each word), coreference across sentences, and word senses with their glosses. **Not candidates**, on
Q11's ruling: link grammar, NLTK's grammars, and any hand-written grammar of English.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2f.1](s2.2f.1_the_check.md) | **The check**: what trained parsers read right where step 1 went wrong | an environment on this host and each tool's peak GPU memory; AMR from at least two parsers and the analyses beside it on the six texts; a table, failure class by sentence, of what each reads right against the hand's step 1 and the model's; a verdict per tool. No model of the formalizer runs |
| [S2.2f.2](s2.2f.2_the_analysis_written.md) | The analysis written for the formalizer | what passed the check, written two ways, as text for a prompt and as Ein facts the kernel loads, each node with the sentence it came from; recorded analyses EinAf's tests replay with no parser |
| [S2.2f.2a](s2.2f.2a_parses_to_step1_by_rules.md) | Step 1's facts read from the parses by Ein rules, with no model: AMR and spaCy | added 2026-11-27 on the user's word (*"make a separate stage for it, before S2.2f.3"*), S2.2f.3's arm 5 taken out, and widened to spaCy's dependency parse (*"not only AMR, also try to convert SpaCy output into S-forms and do the same"*): spaCy's parse written as S-forms; for each parse, rules over EinAf's facts naming only that parse's closed vocabulary, held to it by a check; a concept as an argument and as a relation, the second through `defrel` requests EinAf turns into declarations; the six texts' programs against the hand's step 1 on the failure classes |
| [S2.2f.3](s2.2f.3_step1_with_the_analysis.md) | Step 1 given the analysis | step 1 on the six texts under each arm (none, AMR as text, AMR as facts, the best analysis beside it, the model's own analysis first; the rules' arm is S2.2f.2a's), held to the hand's step 1 on the failure classes; the Zebra puzzle one shot under the best arm |
| [S2.2f.4](s2.2f.4_verdict.md) | The verdict, and what goes on | feasible or not, arm by arm, with its evidence; what S2.6.4 measures as its input-side arm, and what P2.2d and P2.2b take; the phase report |

### Order

**S2.2f.1 runs at any time**: it needs no model of the formalizer and no stage of another phase. **If it finds no
tool that runs here and reads the texts better than the model did, the phase stops there and reports**, and the
user decides whether it goes on. S2.2f.2 follows S2.2f.1's verdict; S2.2f.2a follows S2.2f.2 and needs no model; S2.2f.3 follows S2.2f.2 and S2.2c.1,
and reads S2.2f.2a's table beside its arms; S2.2f.4
closes the phase.

**Where it stands in the milestone's order.** It is not a prerequisite of [P2.3](../p2.3_benchmark/README.md):
nothing of the benchmark waits for it, and its result reaches the formalizer through P2.2d and S2.6.4. When it
runs is the user's ([the order before P2.3](../README.md#the-order-before-p23)).

### The environment

The parsers are Python with PyTorch and models from a hub: **none is on this host** (`link-parser` and NLTK are,
and are not used). The host's rule is that nothing is written outside the project, `/tmp` and `~/.claude*`, so:

- one environment, under the gitignored `build/nlp/`, made by a committed script of the phase's: a Python the
  packages have wheels for (the system's is 3.14; `uv`, on the host, can fetch another, into `build/nlp/` with
  `UV_PYTHON_INSTALL_DIR` and `UV_CACHE_DIR`), the packages, and **every cache pointed there**: pip's, Hugging
  Face's (`HF_HOME`), torch's (`TORCH_HOME`), Stanza's resources, spaCy's and the AMR parsers' models. Nothing
  under `~`;
- where the session's permissions refuse a download, the stage gives the user the commands and waits;
- **the GPU is shared**: a parser and the model run one at a time, never together; the user's ollama service may
  hold memory, and the stage reads `nvidia-smi` before each tool;
- what the environment and the models are is a field of every record: the package versions, each model's name
  and hash.

### What is committed

As the milestone's rule says ([§ What is committed](../README.md#what-is-committed)): the setup script, the
scripts that run the parsers and compare, the plans of runs, the tables, the record and the report are committed;
the environment, the models, the parsers' raw outputs and the model's generations are not, and are written under
the phase's `runs/` or `out/`. **An analysis that a test of EinAf replays is a test's input** and is committed under
`einaf/tests/fixtures/`, as the model's recorded replies are.

## Acceptance

- S2.2f.1's table, for each of the six texts and each failure class, says what each tool reads right, against the
  hand's step 1 and the model's; each tool's install, its version and its peak GPU memory are recorded.
- Where a tool passed: the analysis is written as text and as Ein facts, the facts load under `ein solve
  --no-search`, and EinAf's tests replay the recorded analyses with no Python and no GPU.
- S2.2f.2a's rules, over AMR and over spaCy's parse, each held to its parse's closed vocabulary by a check, give a step-1 program for each of the six texts,
  read against the hand's step 1 on the failure classes.
- S2.2f.3's arms are run on the six texts, every run recorded, and each read against the hand's step 1 on the
  failure classes; the Zebra puzzle one shot under the best arm, its answer and its outcome.
- The verdict: feasible or not, arm by arm; what S2.6.4 measures; what Ein showed of itself, in the evaluation.
- The gate is green on the phase's last commit that changes EinAf; a stage that changes no code gets the link
  check.

## Risks

- **Packages that do not install.** AMR parsers pin old versions of PyTorch and of the transformers library, and the
  host's Python is new. The check records what fails and why, tries another parser or another Python before giving
  up on a tool, and says which.
- **The genre.** Parsers are trained on news and web text. A Zebra clue (*the man who smokes Chesterfields lives in
  the house next to the man with the fox*) has a relative clause inside a spatial relation; the check reads what the
  parser does with it and does not assume.
- **The alignment.** An AMR node does not say which words it came from; an aligner does, and its errors are its
  own. A node with no sentence cannot carry a `:source`, so the stage says how many nodes are aligned.
- **A mapping by rules must not become a grammar of English.** S2.2f.2a, which reads AMR into step 1's facts with
  no model, is kept on the user's condition (2026-11-27): *"it is an option, if rules relies on finite amount of terms
  that AMR uses, not rely on NL words"*. Its rules match only AMR's own closed vocabulary (its roles, `:ARG0` to
  `:ARG5`, `:polarity`, `:name` and `:op1`…, `:quant`, `:location`, and its fixed special concepts such as `name`,
  `and`, `have-rel-role-91`), and carry a concept such as `drink-01` or a name's words through as data, never
  matching an English word. A rule that would have to name a word is not written, and the case is counted as one
  the parse alone does not reach.
- **The analysis may mislead.** A parse that is wrong is followed; the check counts what each tool reads wrong, not
  only what it reads right.
- **Scope.** The loop, repair and the benchmark are other phases'. This phase measures feasibility on the texts
  P2.2 has, with small numbers and every run recorded; no rate here is a result of the milestone.

## Defaults taken at the plan

Each is the plan's, recorded so that the user can correct it; none is a ruling.

1. **A phase of its own, named among the `P2.2x`**, the formalizer's, and not a prerequisite of P2.3.
2. **AMR first**, a Universal Dependencies parse, coreference across sentences and word senses beside it; at least
   two AMR parsers, since one parser's failure is not the representation's.
3. **The texts are the six with steps by hand** (Zebra, seating, `apples`, `more`, `together`, `youngest`), whose
   step 1 is the reference; S2.2f.3 adds S2.2.4's sixteen where an arm passes on the six.
4. **The analysis reaches the model two ways**, as text in the prompt and as Ein facts, and both are measured. (The
   link-grammar arm's fragment was plain text and never Ein; a graph reads as facts, so here both are tried.)
5. **The model's own analysis is an arm**: the same markup written by `qwen3.5:4b` before step 1, which answers the
   user's first question (*can the LLM do the analysis instead?*) beside the trained tools.
6. **AMR read into step 1's facts by rules, with no model, is an arm**, on the user's condition (2026-11-27): its
   rules rely on AMR's finite vocabulary and on no word of English (§ Risks). *Was a default; the user's word
   since.*
7. **The parsers run in Python outside EinAf**, and EinAf reads what they wrote from a file; EinAf's gate needs no
   Python and no GPU.
8. **The model runs as P2.2's did**: `qwen3.5:4b`, thinking off, greedy, the stop on a loop; every plan of
   generations committed before its first generation.
9. **Feasibility, not significance**: no pre-registered threshold; S2.6.4 measures an arm that passes, under its
   own rule.

## Connections

- [Q11](../open_questions/formalizer/Q11-link-grammar-value.md), ruled the same day, and
  [S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md), re-aimed to it.
- [P2.2b](../p2.2b_guided_thinking/README.md): the model decomposing the text in its thinking; this phase asks
  the same of tools outside the model.
- [`nl2smt/`](../../../nl2smt/README.md), outside Ein and EinAf, parses the Zebra puzzle with NLTK or link
  grammar; it stays as it is, and nothing here uses it.
- [M1k's P1k.20](../../m1k_notations_as_facts/p1k.20_s_expressions_as_facts/README.md): S-expressions as Ein
  facts, of which PENMAN is one.

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic, ids the milestone's next free.
