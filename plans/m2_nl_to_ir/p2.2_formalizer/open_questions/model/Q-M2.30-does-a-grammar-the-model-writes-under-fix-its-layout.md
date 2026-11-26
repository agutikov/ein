# Q-M2.30 — Does a grammar the model writes under fix its layout?

**Status:** open

**Raised 2026-11-26 by [S2.2.3](../../s2.2.3_gbnf.md#amended-on-the-users-word)**, amended on the user's word, from the
user's question on the whole grammar's size (*"why it is so HUGE?"*).

**The question.** The whole grammar's `root` takes Ein's separators: any run of white space and line comments where a
separator is allowed, chosen by the kinds of the two tokens it stands between (a paren, a string, another word).
Each step's grammar takes the model's layout: a run of up to 24 spaces or a line break and an indentation as long,
forms one or two line breaks apart, a comment a line of its own. A grammar could instead fix the layout, one space
between two words, none beside a paren, a line break between two forms and no comment, and be smaller by what its
separators cost.

**The default taken: no fixed layout.** The amendment narrowed what a model may write (the characters of a name, a
string, a comment) and left the separators as they were.

**What a fixed layout would save**, measured on the root, not adopted (`einaf::grammar::whole`'s `Flavor::Model`, its
`sp` one space and its `sp0` none, the forms a line break apart, no comment; written and read by EinAf's acceptor once,
on `23d582aa`): **31 199 bytes and 376 rules, against the narrowed root's 34 631 bytes (its rules but their comments)
and 383 rules**; as llama.cpp's parser reads them, 400 rules and 4 702 elements against 411 and 5 091. About a tenth:
the root is `ein.y`'s rules compiled into a rule a state (each state is there whatever its separator) and the name
tree that leaves out Ein's 26 words, and neither depends on the layout.

**What the default costs.** A tenth of the root's bytes, and nothing a call measures: the root's sampler is built in
0.38 ms (the amendment's test 3, the median over the corpus), and under `chosen-first` a grammar costs a generation
what it refuses, not its size.

**What a fixed layout would cost.** **A grammar that refuses the model's own layout moves it off its own program**: a
grammar that left out the fence the prompt asked for refused the model's first token, and the model wrote another
text's program ([L-M2.14](../../findings/model/L-M2.14-a-grammar-that-leaves-out-the-fence-a-prompt-asks-for-derails-the-model.md));
under step 1's grammar three of six runs parted from the model's own output at a second blank line before a comment
header ([the report § 4.2](../../grammars/README.md#42-the-second-plan-inside-the-fence)). The model writes its
programs over many lines, indented, with section headers in comments; one space and no comment refuses most of that.

**The alternatives.** A fixed layout for the whole grammar's `root` alone, which a program in one call is written
under; or the steps' bounded layout (runs of up to 24 spaces, a comment a line of its own) for the root too, between
Ein's and a fixed one.
