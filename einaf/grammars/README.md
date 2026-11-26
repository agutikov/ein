# The grammars

EinAf's GBNF grammars, which hold a model's output to Ein where a grammar is on
([the grammars' page](../../docs/einaf/06-grammars.md)). One is a file, generated; the others are written at
request time by a function of `einaf`, and a record names each by its text's SHA-256, never by a path.

| grammar | what it allows | used by | parameterised on | where |
|---|---|---|---|---|
| **the whole grammar**, root `program` | every text Ein's parser takes, every form `ein.y`'s `Program` takes, with Ein's lexer's separators and comments; held to the parser on the corpus | the acceptance test of the grammar against the parser | nothing: generated from `ein.l` and `ein.y` | [`ein.gbnf`](ein.gbnf), `einaf grammar write` |
| **the whole grammar**, root `root` | the forms the contract admits, in any order: `relation`, `rule`, `import`, `solve`, `query`, a fact, a stored negation, a written `(open F)`; no `config`, `trace`, macro or saved file's wrapper; **in the characters a model writes**: a name or a variable of letters, digits and `_ . * + = -`, no block comment, no `<`, `>`, `|` or backtick in a string or a comment | a whole program in one call: the rules `root` reaches, `einaf grammar render whole` | nothing | [`ein.gbnf`](ein.gbnf), its rules named `m-…` and `root-forms…`, and `space` |
| **step 1** | statements (facts and stored negations with `:source`) and queries, exactly one with `:goal-text`; then `relation` forms with `:why` and memberships | the formalization's step 1 | the input's sentences (a `:source` is one of them verbatim), or nothing | `einaf::grammar::steps`, `einaf grammar render 1` |
| **step 2** | `relation` forms and facts (the ontology); `import` forms of the catalogue's modules with names each declares but those step 1's program declares; the catalogue's activating facts at their counts, and the marker of a missing property; no `rule` | step 2's first call | the catalogue; step 1's program | `einaf grammar render 2` |
| **step 2b** | `rule` forms, each after a marker, every pattern and conclusion of a relation the program or a module it imports declares, at a count it is declared at | step 2's second call, on a marker | the catalogue; the program of the steps before | `einaf grammar render 2b` |
| **step 3** | the activating facts of the modules the program imports, over its names; a marker before each `rule` form, closed as step 2b's; the hypothesis mechanism, `(solve :no-search)`, `(solve :hypothesis-relations (…))`, written `(open F)` | step 3 | the catalogue; the program of the steps before | `einaf grammar render 3` |

Every grammar but the whole one is written in the model's layout: a paren hugs what it holds, a separator between two
tokens is a run of up to 24 spaces or a line break and an indentation as long, forms are a line break or a blank line
apart, a comment a line of its own between forms (the marker, `;; missing: …`, one of them), a name of letters,
digits, `_` and `-`, a string with no `"`, `\` or line break. **No grammar a model writes under, the whole one's `root`
and every step's, lets it write `<`, `>`, `|` or a backtick but inside a sentence of the input** quoted as step 1's
`:source` (`einaf::grammar::EXCLUDED`): no name holds one, nor `/`, and no string or comment, so that none of the chat
template's markers (`</think>`, `<tool_call>`) and no fence can stand in a program. **Each is carried in a frame**
(`einaf::grammar::request_grammar`): inside the fenced block a prompt asks for, the root `fenced` added, `"```"`, a
word, a line break, the grammar's root, one or two line breaks, `"```"`; and after the model's thinking, the root
`after-thinking` added, `"</think>"`, at most two line breaks, and the root, begun at the model's token `</think>`.

## The hand-written rules of the whole grammar

The phrase grammar is `ein.y`'s rules compiled, each production an alternative and a list the repetition it derives;
the lexical grammar is `ein.l`'s rules translated node for node of the regular expression. What GBNF cannot take from
either, since Ein's lexer is a pass of its own that takes the longest match and the earlier rule on a tie, is written
by hand, a named rule each (`einaf::grammar::whole::HAND_WRITTEN`, which a test holds to this table):

| rule | stands for | what it does |
|---|---|---|
| `tok-name` | `ein.l`'s `NAME`, and every rule before it whose text is all name characters: the 25 words, `UNDERSCORE`, `WRITEBACK` | a name is a run of the name characters that no earlier rule takes whole: the run less those 29 words, written as a tree of rules over their prefixes (`tok-name-N`, and `tok-name-rest` past them) |
| `tok-var` | `ein.l`'s `VAR`, and `VARUNDERSCORE` before it | a variable is `?` and a run of the name characters, less `?_` |
| `gap`, `gap-word`, `gap-text`, `gap-after-text` | `ein.l`'s skip rule, the start condition `AFTER_WORD`, the catch-alls `BAD` and `BADTEXT` | a token that is not a paren ends at a separator or a paren: between two such tokens a separator is required, between a paren and anything it is not; after a string a separator does not begin with `#`, which `BADTEXT` would take into the string's token |
| `comment-line-ended`, `gap-end` | the skip rule's `;[^\n]*` | a line comment runs to the end of its line: it is followed by a line break, or ends the text |
| `program`, `root` | `ein.y`'s `Program`, `TopForm` and `Form` | a text is a separator, its forms with separators between them, and a separator; `root` takes the forms the contract admits in the characters a model writes, over the rules named `m-…`, `program` every form `Program` takes |
| `m-tok-name`, `m-tok-var` | `ein.l`'s `NAME` and `VAR`, under `root` | a name the model writes is a run of letters, digits and `_ . * + = -` less the 26 words of those characters, a tree of rules over their prefixes (`m-tok-name-N`, `m-tok-name-rest`): no writeback name can begin, so none is left out; a variable is `?` and the same run, less `?_` |
| `m-tok-text`, `m-comment-line` | `ein.l`'s `TEXT` and the skip rule's `;[^\n]*`, under `root` | a string or a line comment the model writes is Ein's, less `<`, `>`, `|` and the backtick, the characters of the chat template's markers and of the fence |
| `m-gap`, `m-gap-word`, `m-gap-end`, `m-comment-line-ended` | `ein.l`'s skip rule and `AFTER_WORD`, under `root` | the separators of `gap`, `gap-word`, `comment-line-ended` and `gap-end` with no block comment: white space and line comments only, so that a separator after a string is the one after a word |
| (left out) | `ein.l`'s `UNDERSCORE`, `VARUNDERSCORE`, `WRITEBACK`, `KEYWORD`, `BAD`, `BADTEXT`, `BADESCAPE` (`ein.y`'s `%expect-unused`) | the tokens no production uses are in no rule: a text that holds one is refused where it stands |
| the separators between tokens | `ein.y`'s productions | each rule of `ein.y` is compiled into a rule a state, the separator between two tokens chosen by their kinds: a paren, a string, or another word |

What is translated and not written by hand: the 25 words and the 20 keywords, each its text; `TEXT`, its escapes
and `\u{…}`'s six ranges (`tok-text`); the skip rule's three pieces (`space`, `comment-line`, `comment-block`); and
every rule of `ein.y`, a rule of the grammar each but those written into the rules that use them (a list, and a rule
that begins or ends with more than one kind of token), with `template-w` and `template-p` the one rule written once a
kind, `Template`, which is recursive through such rules. **`ein.y`'s rules are compiled twice**: once for `program`,
over Ein's lexical rules and separators, and once for `root`, each rule named `m-…` (`m-relation`, `m-head-name`),
over the rules of the three rows above; the two share `space`.

## The manifest

[`MANIFEST.sha256`](MANIFEST.sha256): the SHA-256 of each generated file and its path, as
`stdlib/MANIFEST.sha256` is the library's. `einaf grammar write` writes both, `einaf grammar write --check` writes
nothing and exits 1 where either is not what it would write, and EinAf's tests hold both
(`einaf/tests/grammar.rs`, `the_grammar_file_is_what_einaf_grammar_write_writes`).
