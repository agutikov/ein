# The grammars

**A grammar holds a model's output to Ein**: a GBNF grammar in the request's sampler chain, so that a token the
grammar does not allow is never generated ([the model's runtime § 6](05-model.md#6-a-grammar-and-where-it-begins)).
It is the filter's mechanism: where it is on, the model's output parses. EinAf has two kinds: **the whole grammar**,
a file generated from Ein's two grammar files, and **a grammar a step** of the formalization
([the contract § 3](04-contract.md#3-the-steps)), written at request time from what the step is given. The code is
[`einaf/src/grammar/`](../../einaf/src/grammar/mod.rs); the inventory of the grammars and of every hand-written rule
is [`einaf/grammars/README.md`](../../einaf/grammars/README.md).

| section | what |
|---|---|
| [1. The whole grammar](#1-the-whole-grammar) | generated from `ein.l` and `ein.y`; what is translated and what is written by hand; the two roots, and the characters a model writes |
| [2. Held to the parser](#2-held-to-the-parser) | the corpus through EinAf's acceptor, the grammar's own texts through the parser, llama.cpp's reading |
| [3. A grammar a step](#3-a-grammar-a-step) | what each step's grammar allows, what it is rendered on, what its vocabulary closes |
| [4. Where a grammar begins, and how an output ends](#4-where-a-grammar-begins-and-how-an-output-ends) | from the first token, or after the thinking; the end of a turn |
| [5. How a token is held to a grammar](#5-how-a-token-is-held-to-a-grammar) | every candidate, or the chain's choice first |
| [6. The commands](#6-the-commands) | `einaf grammar` |
| [7. Held by](#7-held-by) | the tests |

## 1. The whole grammar

**`einaf/grammars/ein.gbnf` is generated** by `einaf grammar write` from Ein's two grammar files,
[`ein.l`](../../ein.rs/crates/ein-ir/src/ein.l) and [`ein.y`](../../ein.rs/crates/ein-ir/src/ein.y), read through
grmtools' public readers as Ein's grammar page is written from them
([the grammar page](../ein/user/04-reference/01-grammar.md)): `lrlex` for the lexer's rules, each regular expression
read by `regex-syntax` and translated node for node (a literal a literal, `|` an alternation, `*` `+` `?` and `{m,n}`
their GBNF, a class the class of the same characters); `cfgrammar` for the phrase rules, each production an
alternative and a list, which `ein.y` writes as a rule that names itself first, the repetition it derives. The file
says that it is generated, and [`MANIFEST.sha256`](../../einaf/grammars/MANIFEST.sha256) names it by its digest.

**What GBNF cannot take from either is written by hand**, a named rule each, because GBNF is one grammar over
characters and Ein's lexer is a pass of its own that takes the longest match, and of two of one length the earlier
rule ([the grammar page § 1](../ein/user/04-reference/01-grammar.md#1-lexical-grammar)):

- **a name is a run of the name characters that no earlier rule takes whole**: `and`, `relation`, `_` and the
  engine's writeback names are tokens of their own, so `tok-name` is the run less those words, a tree of rules over
  their prefixes, and `nota`, `and-1` and `relations` are names; `?_` likewise of a variable;
- **a token that is not a paren ends at a separator or a paren**: between two such tokens a separator is required,
  between a paren and anything it is not, and after a string a separator does not begin with `#`, which the lexer
  would take into the string's token;
- **a line comment runs to the end of its line**: it is followed by a line break, or ends the text;
- **the tokens no production uses are left out**: `_`, `?_`, an unknown keyword, the writeback names, and the three
  catch-alls, so a text that holds one is refused where it stands.

Each rule of `ein.y` is compiled into a rule a state of its own, so that the separator before every token is chosen
by the kinds of the two tokens it stands between: a paren, a string, or another word. A rule that begins or ends with
more than one kind of token is written into the rules that use it, and the one such rule that is recursive,
`Template`, is written once a kind (`template-w`, `template-p`).

**Two roots.** `program` takes every text Ein's parser takes: every form `ein.y`'s `Program` takes, between
separators, Ein's language exactly. **`root` takes what the contract admits** ([the contract § 1.4](04-contract.md#14-what-a-program-may-not-hold)):
`relation`, `rule`, `import`, `solve`, `query`, a fact, a stored negation and a written `(open F)`, and not
`config`, `trace`, a macro or a saved file's wrapper; it is the root of a whole program written in one call, and
**it takes them in the characters a model writes**:

- **a name and a variable** are runs of letters, digits and `_ . * + = -` (`einaf::grammar::whole::ROOT_NAME_CHARS`),
  less Ein's words of those characters, where Ein's names may also hold ``$ % ^ < > ! & @ ~ / ' ` [ ] { }``;
- **no block comment**, only line comments;
- **no `<`, `>`, `|` or backtick in a string or a line comment** (`einaf::grammar::EXCLUDED`).

So none of the chat template's markers (`</think>`, `<tool_call>`, `<|im_end|>`), each of `<`, `>` and `|`, and no
fence, of backticks, can stand anywhere in a program written under `root`: neither as a name, where Ein's language
takes `</think>` as one, nor in a comment or a string. llama.cpp's grammar sampler reads a special token by its text,
so a grammar that takes the text `</think>` takes the token `</think>` too. The root's rules are `ein.y`'s compiled a
second time over these lexical rules, each named `m-…` (`m-relation`, `m-tok-name`); `program`'s are Ein's, and the
two share `space`. A request for a whole program carries `root` and the rules it reaches alone
(`einaf grammar render whole`).

## 2. Held to the parser

**The grammar is held to Ein's parser in both directions, with no model**, by EinAf's tests (§ 7):

1. **The corpus through EinAf's acceptor.** Every `.ein` file under `examples/`, `stdlib/` and `tests/` is accepted
   by the whole grammar from `program` exactly when Ein's parser, reached through `einaf::kernel::parse`, reports no
   error. **From `root`**, a corpus file the parser takes, and every program of the contract, is accepted exactly when
   it holds no form the contract does not admit, no block comment and no character the root leaves out, as written
   and with its comments emptied (the hand-written programs write code in backticks in their comments). Two refusals are not a context-free grammar's to state and are named where they occur: a text nested past
   1 000 parentheses ([§ 1.7 of the diagnostics](../ein/user/04-reference/07-diagnostics.md#17-a-text-nested-past-1-000-parentheses-is-refused-before-it-is-parsed))
   and a byte that is not UTF-8. And every shape of the parser's own banked table of the shapes it refuses is refused
   by the grammar. **EinAf's acceptor** is `einaf::grammar::gbnf::Compiled`: the grammar's text read as llama.cpp's
   parser reads it, a repetition expanded into the rules it stands for, and run as llama.cpp's grammar sampler runs
   it, over characters, a set of stacks each advanced to the characters it can take.
2. **The grammar's own texts through the parser.** A sampler walks the grammar's rules from a stated seed and writes
   texts, an alternative that begins with a name, a variable or white space weighted above the rest so that the texts
   hold names beside the words no name is, a position word in each place, tokens glued to parens and comments; every
   text parses, and every text of `root` holds none of the characters it leaves out.
3. **llama.cpp's reading**, by hand, with the model: `einaf grammar llama` (the `llama` feature) loads the model's
   vocabulary alone, tokenises each file, and feeds its tokens one by one to llama.cpp's own grammar sampler made from
   the same text, each token offered alone and refused where the sampler sets its logit to minus infinity, the end of
   turn offered after the last; a file is read alike where the sampler refuses the token that holds the character
   EinAf's acceptor refuses, or takes every token and the end where the acceptor accepts. With `--special` a text is
   tokenised with the model's special tokens parsed, so that `</think>` is offered as the one token the model
   generates, not as its characters.

## 3. A grammar a step

**Each step of the formalization has a grammar of its own**, the whole grammar restricted to the forms the step
writes, in the order it writes them ([the contract § 3.1](04-contract.md#31-each-step)), written at request time by
`einaf::grammar::steps::render` and named by its text's SHA-256:

| step | what its grammar allows, in order | rendered on |
|---|---|---|
| **1** | statements (facts and stored negations with `:source`) and queries, the answer query, with `:goal-text`, exactly once among them; then `relation` forms, `:why` required, its references `{?2}` to `{?n+2}` for `n` labels, and memberships, `:source` where there is one | the input's sentences, so that a `:source` is one of them verbatim; or nothing |
| **2** | the ontology, `relation` forms and facts; `import` forms of the catalogue's modules, listing names each declares but those step 1's program declares itself, which a list would bind twice; the catalogue's activating facts, each at its count, and the marker of a missing property; no `rule` | the catalogue; step 1's program |
| **2b**, step 2's second call | `rule` forms, each after a marker, each pattern and conclusion of a relation the program or a module it imports declares, at a count it is declared at, over the program's names and variables | the catalogue; the program of the steps before |
| **3** | the activating facts of the modules the program imports, at their counts, over its names; a marker before each `rule` form, closed as step 2b's; the hypothesis mechanism: `(solve :no-search)`, `(solve :hypothesis-relations (…))` of the program's relations, a written `(open F)` | the catalogue; the program of the steps before |

**The catalogue** a grammar is rendered on, `einaf::grammar::steps::Catalogue`, is either of two. **A run of the
steps renders its grammars on the catalogue's entries** ([the steps § 1](07-steps.md#1-the-catalogue)): the modules
the entries import, the names they list, the facts they select with at their counts, the relations a module reads
that no module declares (`slot-partition`), and the names a program writes as values (the calculator's orders, the
digits); `einaf catalogue --grammar` lists it. **The library's**, `Catalogue::library`, read off its texts, is every
module of `std` but `std.core`, the names it declares (which `:symbols` may list), its rules of one parameter or more
with their counts (a fact of whose name activates them), and the relations its rules read that no module declares;
`einaf grammar catalogue` lists it, and the grammars' own tests render on it.

**What a step's vocabulary closes.** Step 1 writes its facts before their declarations, so it closes none of them. Step
2 declares relations in the same output as the facts of them, so its ontology is open, and its imports and activating
facts are the catalogue's, an import listing no name step 1 declared. Steps 2b and 3 are rendered on the program of the steps before them: a fact, a pattern or
a conclusion they write is of a relation that program or a module it imports declares, at a declared count, and its
names are the program's. No grammar holds an argument to a label: a label is no type
([Q-M1f.372](../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)).

**The layout is the model's.** A paren hugs what it holds; a separator between two tokens is a run of up to 24 spaces,
or a line break and an indentation as long; forms are a line break or a blank line apart; a comment is a line of its
own between forms, before the first and after the last; a name is a run of letters, digits, `_` and `-` that is no
word of Ein's; a string holds no `"`, `\` or line break, and a template's braces only its references; **a string, a
comment and the marker hold no `<`, `>`, `|` or backtick**, as under the whole grammar's `root`, but a sentence of the
input quoted verbatim as step 1's `:source`, which is the input's. **The marker**
of a missing property is the comment line `;; missing: ` and the property's words, which steps 2b and 3 write directly
above each rule and step 2 writes as any comment, as the steps by hand do ([the contract § 7](04-contract.md#7-the-steps-by-hand)). A program the kernel reads is the same with its comments or
without them.

## 4. Where a grammar begins, and how an output ends

**Thinking off, from the first token** (`{"at": "first"}`): the grammar holds the whole output. **A grammar begins
where the program begins**: where the prompt asks for the program in one fenced block, the grammar a request carries
is the grammar with a root added, `fenced ::= "```" [a-z]* "\n" root "\n" "\n"? "```"`, so that the model's first
token is the fence it means to write, the program is held inside it, and the turn ends at the closing fence. A grammar
that leaves the fence out refuses the model's first choice, and the model's next can be the program the prompt shows
nearest, not the one it was asked for.

**Thinking on, after the thinking**: a grammar from the first token takes the thinking too
([the model's runtime § 6](05-model.md#6-a-grammar-and-where-it-begins)), so the grammar a request carries is the
grammar with a root added, `after-thinking ::= "</think>" "\n"? "\n"? root`, begun lazily at the model's closing tag,
the token `</think>` (`{"at": "token", "text": "</think>"}`): llama.cpp feeds a lazy grammar the trigger's own text, so
the trigger is the grammar's first text, and the white space after it is bounded. A pattern trigger,
`[\s\S]*?(</think>)[\s\S]*`, begins it at the same token, but is matched against the whole thinking at every token
while the model thinks, which costs a long thinking its speed; the token is matched against nothing. `einaf::grammar::request_grammar` writes
each frame, `einaf::grammar::Frame`'s `fence` and `after_thinking`, alone or together (`after-thinking` then holds
`fenced`).

**The end.** The root of a step's grammar is the step's part in synthesis order, inside the fence where there is
one, so that the end of the turn is allowed only after a whole part (and its closing fence): step 1 after its ontology, with its one answer query written; steps 2, 2b and 3
after any whole form of theirs, and step 3, which may have nothing to add, before its first too. The model ends its turn where the grammar allows the end of turn and the model takes it.
Reaching the token budget is the runtime's stop `max-tokens`, a truncated output, never a refusal of the grammar.

## 5. How a token is held to a grammar

A request's grammar carries `check` ([the model's runtime § 3](05-model.md#3-a-request-and-a-generation)):

| `check` | what the runtime does at each token | its cost |
|---|---|---|
| `every-token` (absent) | the grammar applied to every candidate, then the chain chooses | the grammar's work on every token of the vocabulary, at every token |
| `chosen-first` | **the chain chooses, and its choice alone is offered to the grammar**; where the grammar refuses it, the candidates are read again, the grammar applied to all and the chain chooses again, as llama.cpp's own sampler does in its server | the grammar's work on one token, but where it refuses the choice; while a lazy grammar has not begun, every token as above |

**Under greedy decoding the two choose the same tokens**: the most likely token the grammar allows. The envelope's
`grammar` holds the check, the grammar's `build_ms`, the wall time of its sampler's making, and, under
`chosen-first`, `chosen_again`, the tokens whose first choice the grammar refused. `request_grammar` writes
`chosen-first`.

## 6. The commands

```sh
einaf grammar write [--check]                          # einaf/grammars/ein.gbnf and its manifest, or exit 1 where they differ
einaf grammar accept [--grammar G] [--root R] FILE...  # EinAf's acceptor: each file accepted, or refused at a line and column
einaf grammar sample [--grammar G] [--root R] [--seed N] [--count N]   # texts the grammar allows
einaf grammar render whole|1|2|2b|3 [--program P] [--text T] [--fence] [--after-thinking] [--request]   # a grammar in its frame (`whole`: `root` and its rules), or the one a request carries
einaf grammar catalogue                                # the catalogue the steps' grammars are rendered on
einaf grammar llama --model M [--grammar G] [--root R] [--list L] [--special] FILE...   # llama.cpp's reading beside EinAf's (`llama`); --special: a marker one token
```

Without `--grammar`, `accept`, `sample` and `llama` read the whole grammar from its root `program`; `--root root`
reads it from the model's root. `render 1` is
rendered on `--text`'s sentences; `render 2`, `render 2b` and `render 3` on `--program`, the program of the steps
before.

## 7. Held by

[`einaf/tests/grammar.rs`](../../einaf/tests/grammar.rs), in EinAf's gate step, with no model:

| test | what it holds |
|---|---|
| `the_grammar_file_is_what_einaf_grammar_write_writes` | the file and the manifest are what the command writes from `ein.l` and `ein.y` |
| `the_inventory_names_every_hand_written_rule` | the inventory's table has a row for each hand-written rule, and the counts of the words no name is, in Ein's characters and in the model's |
| `the_grammar_accepts_a_corpus_file_exactly_when_the_parser_does` | § 2's first test, every corpus file; `--nocapture` prints the counts and its time |
| `every_shape_of_the_kernels_table_is_refused_by_the_grammar` | every row of `ein-ir/tests/golden/grammar_table.txt` refused by the parser and by the grammar |
| `every_text_sampled_from_the_whole_grammar_parses` | § 2's second test, 2 000 texts from a stated seed, half from each root, and that they reach the hand-written rules; those of `root` hold none of the characters it leaves out |
| `the_root_refuses_the_chat_templates_markers_the_fence_and_a_block_comment` | texts Ein's parser and `program` take that `root` refuses: `(p </think>)`, `(p <tool_call>)`, a fence, `/` in a name, a block comment, `<`, `|` or a backtick in a `:why`, a marker in a comment; and each step's comment and marker refusing a marker |
| `the_root_accepts_the_contracts_programs_and_the_corpus_in_its_characters` | § 2's first test from `root`: every corpus file the parser takes and every program of the contract, as written and with its comments emptied, accepted exactly when it holds none of what `root` leaves out; `--nocapture` names each file refused for a character of a name or a string, and each contract program refused as written |
| `each_steps_grammar_accepts_the_steps_written_by_hand` | each step's grammar, rendered for each text of [the steps by hand](04-contract.md#7-the-steps-by-hand), accepts what that step wrote, a marker before each rule of step 3; step 2b's the rules step 3 wrote, each after a marker |
| `every_text_sampled_from_a_steps_grammar_parses_and_is_of_its_step` | texts sampled from each step's grammar, for each of those texts, parse, are of the forms the step writes in its order, and hold no `<`, `>`, `|` or backtick but in a sentence of the input |

The unit tests of [`gbnf.rs`](../../einaf/src/grammar/gbnf.rs) and [`whole.rs`](../../einaf/src/grammar/whole.rs)
hold the reader, the acceptor, the sampler and a few texts of the whole grammar. llama.cpp's reading (§ 2's third
test) and every generation under a grammar are measurements by hand, not tests: the gate has no model.
