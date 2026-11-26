# M17 — Formalizing a text that does not fit into the model's context

**Status:** created 2026-11-27 on the user's instruction ([below](#the-instruction)); **not started**. This page
reads the instruction against what exists and lists what would have to be decided. Nothing is designed, nothing has
a default, and it carries no estimate (the user's ruling of 2026-11-08).
**Depends on:** [M14](../m14_ein_server/README.md), the Ein server, into which the chapters are loaded; and on
formalizing one text that fits, which is [M2](../m2_nl_to_ir/README.md)'s (EinAf's steps, `einaf from-text`). M2's
P2.2 has not met its minimal goal: the Zebra puzzle, one shot, prints *"The statements contradict each other."*
([its report](../m2_nl_to_ir/p2.2_formalizer/report/README.md)).
**Where it sits** against M2 and the milestones after M1f is not ruled; it starts only on the user's word.

## The instruction

The user, 2026-11-27, verbatim:

> M20 - How Ein can help to formalize text that does not fit into model's context?
> - split text into pages, chapters
> - read and formalize separate chapters and load into the Ein server
> - try merge, find conflicts
> - summarize previous chapters to improve formalization of next
> - similar to attention window

M20 is the GUI's ([M20](../m20_gui/README.md)); asked, the user gave this milestone **M17**. The same message
created M14, the server, *"useful for Ein MCP and M20"*, that is for M15 and this milestone.

## Each line, against what exists

- **The context today.** EinAf runs `qwen3.5:4b` locally with a context of 65 536 tokens and a budget of 12 288
  generated tokens a call (P2.2's bounds). A text is formalized in three steps, facts, then ontology, then theory
  ([`docs/einaf/07-steps.md`](../../docs/einaf/07-steps.md)). Step 2's grammar is rendered from step 1's program
  (`einaf grammar render 2 --program …`), so a program written so far already stands in for what came before it,
  inside one text.
- **Split the text into pages, chapters.** By the text's own structure, or by its tokens. The nearest material in
  the plans is M1h's school text, *Winnie-the-Pooh*, whose Chapter II
  [P1h.10](../m1h_beyond_word_problems/p1h.10_a_school_text/README.md) takes. The book's ten chapters are a long
  text whose chapters are each one text.
- **Read and formalize separate chapters, and load them into the Ein server.** Each chapter formalized as one text
  is today. New: a chapter's program held by [M14](../m14_ein_server/README.md), as a program of its own or added to
  one that grows.
- **Try merge, find conflicts.** A merge is the union of the chapters' forms. In Ein an object's identity is its
  name or its tuple
  ([Q-M1f.380](../../docs/decisions/language/Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)): two
  chapters that write one name mean one object, and two names for one thing are two objects until a rule says
  otherwise. So the vocabulary across chapters is the merge's first problem. P2.2 found the model choosing its
  vocabulary text by text, and its readings going wrong in silence
  ([L-M2.18](../m2_nl_to_ir/p2.2_formalizer/findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)).
  A relation written at another count loads in silence as another relation
  ([Q-M1f.447](../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)). A conflict is
  what Ein already reports: the merged program's root dead, its core naming the facts and the rules
  ([cores](../../docs/ein/reasoning/02-kb-evolution/09-cores.md)), or no commitment alive.
- **Summarize previous chapters to improve the formalization of the next.** The summary may be the model's prose,
  or Ein's: the names declared so far with their `:why`, the facts the next chapter may need (a slice,
  [M1h P1h.2](../m1h_beyond_word_problems/p1h.2_slicing/README.md)), what is still open (H, what root owes), read as
  English ([M1p](../m1p_ein_to_natural_language/README.md)). And the next chapter's grammar can be rendered from
  the merged program, as a step's is from what the step is given
  ([`docs/einaf/06-grammars.md`](../../docs/einaf/06-grammars.md)), so the model writes in the vocabulary the
  earlier chapters fixed.
- **Similar to an attention window.** The context is a window over the text, and the knowledge base is what is
  kept of the part already read; what the next window is given of it is what is chosen to bring back.

## What would have to be decided

**None of these is decided, and none has a default.**

- **Which texts and which questions**: a book, a contract, requirements
  ([P1h.20](../m1h_beyond_word_problems/p1h.20_requirements_verification/README.md)), a long puzzle, several
  documents; questions over the whole text or over one chapter; their gold answers.
- **What "help" is measured against**: the model alone with the text cut to its context, or with its own summary;
  a text that does fit, run both whole and in chapters, as the control.
- **The split**: by the text's structure or by tokens; whether windows overlap; a chapter's size against the
  context.
- **One program that grows, or a program a chapter merged later**; chapters in reading order or each alone.
- **The vocabulary across chapters**: given before the first chapter, grown chapter by chapter (the next grammar
  rendered from the merged declarations), or aligned after (two names for one thing found and joined by a rule, by
  the model or by the user).
- **What a conflict is, and what is done with it.** One the text has (a story contradicts itself, or changes over
  time: [P1h.1](../m1h_beyond_word_problems/p1h.1_temporal_and_causal_theories/README.md); what a character says or
  believes: [P1h.5](../m1h_beyond_word_problems/p1h.5_contexts_in_a_text/README.md)) against one the formalization
  made; who repairs it (the model given the core, as in EinAf's loop, or the chapter formalized again).
- **What the summary is**: the model's prose, Ein's text of the knowledge base, the declarations, a slice; its
  size; how it is chosen for the next chapter.
- **Whose it is**: EinAf's, an application of Ein with a model in M2's tree, or a project of its own; and against
  M2's phases, the loop ([P2.4](../m2_nl_to_ir/p2.4_loop/README.md)) and scaling
  ([P2.7](../m2_nl_to_ir/p2.7_failure_scaling_generalization/README.md)).

## What it must not assume

- **That the merged program's answer is more than Ein's.** It is R of the merged program; a flagged program claims
  nothing.
- **That one name means one thing across chapters**, or that the model keeps its vocabulary from one call to the
  next.
- **That a summary is true of the text.** A summary the model writes is a claim of the model's.
- **That Ein has numbers.** A chapter's quantities are the calculator's theory, computed by saturation.

## Connections

- [M14](../m14_ein_server/README.md), the server the chapters are loaded into and merged in.
- [M2](../m2_nl_to_ir/README.md), EinAf: the formalizer of one text, its grammars and its feedback; P2.2's
  [report](../m2_nl_to_ir/p2.2_formalizer/report/README.md) and its [analysis](../m2_nl_to_ir/p2.2_formalizer/report/ANALYSIS.md).
- [M15](../m15_mcp_server/README.md), an MCP server: a model calling Ein, chapter by chapter.
- [M1h](../m1h_beyond_word_problems/README.md): the school text, slicing, contexts in a text, temporal theories.
- [M1p](../m1p_ein_to_natural_language/README.md), Ein to natural language: a knowledge base read back as text, a
  summary Ein writes.
