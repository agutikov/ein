# S1f.22.2 — the explanation, written

**Phase:** [P1f.22](README.md)
**Status:** **done 2026-11-17**: the pages, the diagrams and the decisions are written (§ Record). What the
rulings need built is the build stages', S1f.22.3 to S1f.22.5.
**Writes:** pages of `docs/ein/` and their diagrams, decisions under `docs/decisions/`, and this file's § Record.
No crate, no standard library, no corpus program: what the rulings need built is S1f.22.3's.

## What it writes from

The draft is [`analysis.md` § 3](analysis.md#3-the-explanation-drafted). The rulings that shape it, each in
[`open_questions.md`](../open_questions.md):

| ruling | what it gives the pages |
|---|---|
| Q-M1f.394 | **the words**: a fact is *held*, which is *in the global context*, and/or *in a context*, a fact's (local) one; a negation is a context; *object* for every node of the graph; *value* only for the object a variable of a rule is bound to; *term* a word of the syntax; no *relational node* |
| Q-M1f.396 | **where**: sections of `01-static-ir/01-knowledge-base.md` and `02-rewriting-rules.md`, a sentence each on the pages of `02-kb-evolution/`; the user's example for the cases and the two rewritings, the page's `says` kept |
| Q-M1f.398 | **the drawing**: a fact is filled where it is held, and inside a dashed frame labelled with the context's relation, `think ctx`, where it is in a context; one frame names two relations where facts of both point at a fact; frames stand side by side; a figure with no fact under an arrow is not changed |
| Q-M1f.397 | **the reserved words**: mathematics on the reasoning's pages; on the user's and the engine's, the surface between the kernel and a theory in Ein |
| Q-M1f.393 (a) | **what a rule can spell**: it unpacks by a fact's count, the relation a parameter where wanted; a variable is no literal and no premise |
| *all rules are global* ([`contexts.md` § 3 to § 5](contexts.md#3-rules-are-global-what-a-premise-reads-and-what-a-wrapped-premise-reads)) | a premise reads the global context; a context's rule is the same patterns wrapped, by hand, by a macro, or with the context a parameter; `(false)` in a context is a held fact of the context's relation |

## What it does not write

A page states what the engine does, and two rulings are of things not built. Their sentences stay as they stand
and change in the commit of S1f.22.3 that builds them:

- **Q-M1f.391 (c)**: *"$\lnot\lnot g$ is no fact"* on the knowledge-base page, the shape of $\mathsf{not}$, the
  value rule on the user's pages, the grammar.
- **Q-M1f.395**: the row *a fact under an arrow of another: one node labelled with the fact's text* in the
  knowledge-base page's table of the labelled-arrow form, and `12-rendering.md`.

Q-M1f.392 is ruled a bug; the answer-dump page changes with its fix, in S1f.22.3.

## Tasks

1. **Take the survey's § 2 again.** Every statement [`analysis.md` § 2](analysis.md#2-what-the-pages-and-the-decisions-say-now)
   quotes is read on the page as it stands before a sentence is written from it ([N-16](findings.md)).
2. **The knowledge-base page.** The legend of *How the graph is drawn* gains the frame of a context. § *A fact about a
   relation, a fact about a fact, a negation, falsity* gains the two contexts, $\operatorname{ctx}_K$, the three
   cases and their three drawings (draft § 3.1). After § *A fact, its negation, both, or neither*: a context gives
   a fact no truth, `not` as the one context the definitions read, and what is the same and what differs (draft
   § 3.2, § 3.3; [`contexts.md` § 11](contexts.md#11-not-as-a-context) for what `not` is and is not). § *Nesting*:
   the sentence on $\operatorname{sub}$ against $\operatorname{ctx}$.
3. **The rewriting-rules page.** A section on packing and unpacking after § *A conclusion, and the result of
   rewriting* (draft § 3.4), with the four drawings; the sentence on held sets in § *As a span*; the table of what
   a rule can spell; the premise the page's dependency sentence means (N-14, with
   `03-rule-evolution/02-analysis-of-rules.md`).
4. **Every figure that has a fact under an arrow, with the frame.** Under `docs/ein/reasoning/`, each source that
   draws the graph and has a fact that a fact points at: that fact inside the dashed frame, in a knowledge base
   and in a pattern alike, as in the trial. The others keep their sources. Drawn by `utils/render_diagrams.py`. A
   figure drawn with labelled arrows is not one of these.
5. **The pages of `02-kb-evolution/`**: a sentence each, as [`analysis.md` § 3.6](analysis.md#36-where-each-part-goes)
   lists (saturation, hypothesis generation, the solution's semantics with N-10, the cores).
6. **The user's pages.** `03-facts.md` § *A fact inside a fact* and `05-rules.md`: the two contexts, that rules
   are global and a context's rule is wrapped, the macro, `(false)` in a context, each shown as Ein with what it
   answers, and as a graph, in words and in mathematics where Q-M1f.371 asks for the four. The glossary's *Value*.
   The findings that are a sentence: N-6 (the pages' half), N-8, N-11, N-12.
7. **The engine's pages**: N-9's sentence in `03-design/03-data-model.md` and its *relational node*; what
   `08-reasoning-in-code/01-static-ir.md` says of the two reasoning pages' new sections (the code, the tests, the
   commands), with the programs of the analysis's § 1 that are worth keeping named as what S1f.22.3 adds.
8. **The decisions**: a file each for Q-M1f.393, .394, .396, .397 and .398, the final answer only.

## Acceptance

- Every command a page shows was run on the gate build and prints what the page says.
- `python3 utils/doc_audit.py --links --check --tracked`, `python3 utils/check_doc_blocks.py`,
  `python3 utils/render_diagrams.py --check`, `python3 utils/check_release_voice.py` and
  `python3 utils/check_generated.py` exit 0.
- The crate tests that read pages pass (`cargo test --profile gate --workspace`, once, on the stage's last commit).
- What the stage finds outside its scope is in [`findings.md`](findings.md).

## Record

### Record (2026-11-17)

**Commits**, on master:

| commit | what |
|---|---|
| `9eda1a04` | the reasoning's pages and diagrams, the notation table, and the five decisions |
| `729094b8` | the user's pages and their two diagrams |
| `27baf9f2` | the engine's two pages |

**What was done, by task.**

1. **The survey's § 2 taken again.** Each page was read as it stands before a sentence was written on it. Stale in
   the analysis beside N-16's: the sentence N-10 names is no longer on `06-solution-semantics.md`.
2. **The knowledge-base page**: the legend gains the frame of a context; a new section *The global context, and
   the context of a fact* ($\operatorname{ctx}_K$, the three cases, three drawings on the user's example, how the
   two properties are drawn); a new section *A context gives a fact no truth* (the proposition, a negation beside
   a context drawn, `not` as the one context read and no mirror, what is read only where held, the table of what
   is the same and what differs); `written` as a context the reading makes; the sentence on $\operatorname{sub}$
   against $\operatorname{ctx}$. The notation table has $\operatorname{ctx}_K$.
3. **The rules page**: a new section *Packing and unpacking*: a premise reads the global context; a rule for a
   context is the same rule wrapped (the page's own rule, and the same for what somebody says), the context a
   parameter; falsity in a context; packing and unpacking with their matches and results drawn; the proposition;
   the table of what a rule can spell. The sentence on held sets in *As a span*. N-14: the dependency graph's
   sentence is now of a conclusion's literal $\lnot x$, which writes every vertex of a stored negation, and says
   that no premise is of that kind.
4. **The frame.** One existing figure of the graph had a fact under an arrow, `01-what-an-arrow-points-at`, and is
   redrawn with its frame, *not, says ctx*; every other source of the graph is unchanged. New: `01-context-held`,
   `01-context-only`, `01-context-both`, `01-context-negation`, `02-pack-match`, `02-pack-after`,
   `02-unpack-match`, `02-unpack-after`. All drawn by `utils/render_diagrams.py`.
5. **`02-kb-evolution/`**: a paragraph on saturation (a fact is held because given or concluded, never because of
   its context; `written` is the same in every knowledge base a saturation reaches), two sentences on hypothesis
   generation (an hrule's candidate may point at a fact; the enumerator's objects are atoms), one on the solution's
   semantics (a state is a set of held facts), one on the cores (a record is of a held fact and names held facts).
6. **The user's pages**: `03-facts.md` § *A fact inside a fact* (the two contexts in Ein with the answer, as a
   graph, in words, in mathematics; what a context does not do; `(false)` as an argument, N-11; an hrule's
   conclusion and `written`, N-8); `05-rules.md` § *Rules and contexts* (global and wrapped, the macro, the
   context a parameter, `(false)` in a context and `forall` over it, what a rule must spell); the glossary's
   *Value* and a new *Held, global context, context*; `03-reserved-names.md`'s opening (the surface, Q-M1f.397);
   N-6 on `12-query.md`, `02-syntax.md` and `07-diagnostics.md` § 2.21; N-12 in § 2.30. Two drawings by hand,
   `03-facts-in-contexts` and `05-rule-in-a-context`.
7. **The engine's pages**: `08-reasoning-in-code/01-static-ir.md` (the drawings' rows, where the contexts live,
   what holds each claim and which have no test of their own, a program and two commands that show a fact in a
   context, the reserved words' row); `03-design/03-data-model.md` (N-9's sentence, and *relational node* gone).
8. **The decisions**: `language/Q-M1f.393`, `language/Q-M1f.394`, `docs/Q-M1f.396`, `docs/Q-M1f.397`,
   `docs/Q-M1f.398`, each in its folder's index.

**Run for the pages**, on `ein.rs/target/gate/ein`, every program a page shows and every claim of the new
sections: the three cases and the two questions (`u1`), a denial and `(false)` inside a claim (`u1b`), the global
rule beside its wrapped twin (`u2`, `w1`), the macro (`u3`), the context a parameter with `(false)` and `forall`
(`u4`, `u4b`, `w4`, `w4b`), unpacking with the relation a parameter (`w2`), `(not ?z)` (`w3`), a fact held and
said (`w5`), a nested declaration and activator (`g1`, `c3`), the engine page's program (`c1`, `c2`), the four
refusals of a variable as a premise or a conclusion, an hrule's conclusion and `written` (`m1h`), a core and a
`written` core (`f11`, `f12`), the candidates (`h1`, `f7`). The programs are in the session's scratch directory
and are not kept; those worth a corpus program are below.

**The checks**, on `27baf9f2`:

| check | result |
|---|---|
| `python3 utils/doc_audit.py --links --check --tracked` | exit 0 |
| `python3 utils/check_doc_blocks.py` | exit 0 |
| `python3 utils/render_diagrams.py --check` | exit 0 |
| `python3 utils/check_release_voice.py` | exit 0 |
| `EIN_BIN=ein.rs/target/gate/ein python3 utils/check_generated.py` | exit 0. Without `EIN_BIN` it exits 1 here: its default binary, `ein.rs/target/release/ein`, is not built on this machine |
| `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace` | run twice. The first run exited 101 at one test, `ein-einb/tests/roundtrip.rs`: `a_saturated_zebra2_is_under_80_kb_and_opens_cold_under_6_ms_or_20_with_assertions`, a timing budget (*cold open took 22.893 ms against a 20 ms dev budget*; 26.9 ms run alone), on a machine on battery with a load of 6; cargo stopped there, 71 test binaries in. The second, with `--no-fail-fast`: **exit 0, 160 test binaries, every one `ok`**, that test among them. The stage changed no crate |

**What a build stage should add as corpus programs** (S1f.22.3 is the one that adds witnesses), since the pages' claims of this stage that have no test of their
own are held by nothing but the runs above (`08-reasoning-in-code/01-static-ir.md` says so of each): the three
cases with their two questions; a global rule beside its wrapped twin, neither reading the other's facts; the
context a parameter concluding `(false)` in the context, alive, and `forall` over it, dead; a declaration, an
activator and `(false)` under an arrow doing nothing. `examples/beliefs/` is where they belong.

**Not done, or left uncertain.**

- Nothing of Q-M1f.391, Q-M1f.392 or Q-M1f.395 is on a page: *"$\lnot\lnot g$ is no fact"*, the labelled-arrow
  table's row for a fact under an arrow, `12-rendering.md` and `10-answer-dump.md` are as they were.
- `utils/ci_container.sh` was not run. The new sources use Graphviz clusters, which the runner's build of Graphviz
  lays out differently from this machine's 16.0.0; `check_doc_blocks.py`'s rule 1 will lay them out there.
- The engine's write edge for a conclusion `(not ?x)` was read in the code and not run (N-21).
- The findings' rows that are decisions' sentences or the engine's messages are left (N-20).
- New findings: N-17 to N-21 in [`findings.md`](findings.md).
