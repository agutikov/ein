# L-M2.15 — The grammar a model wrote under admitted the chat template's markers and the fence, as names and in comments and strings

**Status:** closed by `23d582aa`: the grammars a model writes under (the whole grammar's `root`, every step's) leave
out `<`, `>`, `|` and the backtick but in a sentence of the input, and a name `/`
**Found:** by the user's question on S2.2.3's grammar, 2026-11-26 (*"if prompt and ein grammar has overlapping
symbols, e.g. '<' '|' '>' maybe it make sense to remove them from Ein GBNF?"*); confirmed by the coordinator and by
the amendment's test 3
**Owner:** S2.2.3, amended on the user's word ([its record](../../s2.2.3_gbnf.md#amended-on-the-users-word)); the
prompts' reference, [S2.2.4](../../s2.2.4_passes.md)

## What

**Ein's name is a run of `A-Za-z0-9 _ . - $ % ^ * + = < > ! & @ ~ / ' ` [ ] { }`** (`ein.l`), so `</think>`,
`<tool_call>` and ```` ``` ```` are names, and S2.2.3's whole grammar took them where a name stands, from `root` as
from `program`: `(p </think> <tool_call>)` and ```` (p ``` x) ```` were accepted by `einaf grammar accept --root root`
and load in Ein. Its line comments and strings took any character but a line break (a string, but `"` and `\`), and
each step's comments, marker and strings likewise. **llama.cpp's grammar sampler reads a special token by its text**
(`llama_grammar_apply_impl` matches the token's piece, a special token's written out), so the grammar did not only
take the characters: on the model's own token ids it took the token `</think>` (248069) and `<tool_call>` (248058)
as a name, in a comment and in a `:why`, each offered alone after `(p ` (`einaf grammar llama --special`). Under the
fenced frame, three backticks at a line's start were both a name and the closing fence. **Under a grammar, then, the
model could write the chat template's own markers into a program**, which the runtime's parsers of a reply (the
thinking's end, a tool call) read as markers, and a fence the frame reads as the program's end.

## Evidence

[The amendment's records](../../grammars/narrowed/README.md): `llama/special.jsonl`, the six texts of `special/` under
the root before the narrowing (`98e6e0a3`'s file) and after it, with the special tokens parsed: before, every text
taken, its special token among the ids; after, every text refused at that token's index. No program the model wrote in
S2.2.2's and S2.2.3's records held a marker or a fence where the grammar now refuses one
([`models.md`](../../grammars/narrowed/models.md)): the overlap was a door the model had not used.

## What it costs

Nothing of Ein: `program` is Ein's language as it is. What a model may write under a grammar is narrower than Ein:
a name of letters, digits and `_ . * + = -` (every name of the library; one example's `$x` and two strings with a
backtick in the corpus are outside it), no block comment, and no `<`, `>`, `|` or backtick in a string or a comment,
where the model and the hand-written programs write code in backticks (the first experiment's comments, 3 of the
model's 60 loaded programs; 9 of the contract's 21 programs as written). Under greedy decoding a grammar acts only at
the tokens it refuses, so the model's comments change where it writes a backtick. **The prompts' reference**
(`einaf/prompts/ein-excerpt.v1.md` § 1) still tells the model a name may hold `< > ! & @ ~ /` and the rest; a prompt
that asks for a program under a grammar says the characters the grammar takes, which is S2.2.4's.
