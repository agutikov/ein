# Q-M2.37 — Where do a library's descriptions of its rules live?

**Status:** open

**Raised 2026-11-27 by the user**, written up in
[the analysis of P2.2, § 12](../../p2.2_formalizer/report/ANALYSIS.md#12-what-a-library-says-of-its-rules-to-whoever-selects-them),
and filed at the phase's close the same day. The user's words:

> Probably stdlib, or other lib the LLM can import, should carry NL description/documentation of existing rules
> when it selects what to use.
> Because, if I take my own understanding:
>     symmetric - is quite simple
>     includes, compose, converse - is harder to understand
>     top, meet - absolute ambiguity
>     slot-partition, slot-spatial - may look simple, but at the same time exact semantic is not obvious

**The question.** A selector is told today what a rule *derives*: the rule's `:why`, a sentence of one firing; the
reference page; and EinAf's catalogue, by hand. None says what a selecting fact is *for* or what its arguments
need, and a library that is not the standard one has no description at all
([E-128](../../evaluation/stdlib/E-128-a-library-cannot-say-what-a-selecting-fact-is-for-or-what-it-needs.md),
[L-M2.30](../../findings/model/L-M2.30-modules-shown-with-no-description-were-imported-by-no-run.md)). Where does a
description of a rule for its use live, so that whoever selects reads it and something keeps it true?

| the place | what it takes | what it costs |
|---|---|---|
| the module's comments, read by EinAf | nothing of Ein | words nothing checks, and no form for an argument's role |
| a declaration of the selecting fact in the module, its labels the roles, its `:why` the sentence | no new syntax | a declaration is a fact others read: the blind enumerator guesses at every declared relation and `std.contradiction` activates from every declaration |
| a described program beside the module, one a selectable fact or idiom, in the library's tree | EinAf's catalogue entries moved to the library, as programs the gate runs; a directory given beside the library read the same way | a place and a layout to choose in Ein's tree |
| a keyword of the language on `rule` and `macro`, beside `:why` | the description a part of the form | a change to Ein's syntax, which M2 does not make |

**The default that stands**: EinAf's catalogue, by hand, under `einaf/catalogue/`; a module given beside the
library shown by its forms.

**The proposal**: the third, with the first's words where they serve; the fourth written into the evaluation for
the iteration of Ein that follows. Until the user rules, [S2.2d.3](../../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md)
writes the descriptions in EinAf's own tree, where they are today, in a layout a library could take over.

**What bears on it.** It is Ein's to settle and not EinAf's: the library is Ein's
([Q-M2.16](Q-M2.16-what-of-the-kernel-may-change-inside-m2.md) lets a library change that leaves the syntax and the
semantics as they are). [Q-M2.23](Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md): for `top` and
`meet` a description is the lesser repair, and one selecting fact a verb the greater.
