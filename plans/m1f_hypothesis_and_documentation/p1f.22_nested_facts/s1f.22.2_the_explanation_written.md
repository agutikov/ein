# S1f.22.2 — the explanation, written

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **not started**. It waits for the user's look at the frame
([`contexts.md` § 14](contexts.md#14-the-frame-tried-twice)) and for nothing else.
**Writes:** pages of `docs/ein/` and their diagrams, decisions under `docs/decisions/`, and this file's § Record.
No crate, no standard library, no corpus program: what the rulings need built is S1f.22.3's.

## What it writes from

The draft is [`analysis.md` § 3](analysis.md#3-the-explanation-drafted). The rulings that shape it, each in
[`open_questions.md`](../open_questions.md):

| ruling | what it gives the pages |
|---|---|
| Q-M1f.394 | **the words**: a fact is *held*, which is *in the global context*, and/or *in a context*, a fact's (local) one; a negation is a context; *object* for every node of the graph; *value* only for the object a variable of a rule is bound to; *term* a word of the syntax; no *relational node* |
| Q-M1f.396 | **where**: sections of `01-static-ir/01-knowledge-base.md` and `02-rewriting-rules.md`, a sentence each on the pages of `02-kb-evolution/`; the user's example for the cases and the two rewritings, the page's `says` kept |
| Q-M1f.398 | **the drawing**: a fact is filled where it is held and inside a dashed frame where it is in a context; a figure with no fact under an arrow is not changed |
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

Not started.
