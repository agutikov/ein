# `nl2smt/` — Zebra sentences through a parser to SMT

A proof of concept, M1j: the Zebra
puzzle's clues as English lines go in, and a solver's answer comes out.

```
sentences, a line each  →  a parser  →  its converter  →  SMT-LIB code  →  a solver  →  the solution
                           NLTK         nltk2smt.rkt
                           link-parser  lg2smt.rkt
            └──────────────────────── nl2smt.py runs all of it ───────────────────────┘
```

It is one direction, it knows this puzzle's words and sentence shapes and no
others, and it is small on purpose. What makes it more than a printed text is
that the SMT code is a function of the sentences: a line added, removed, moved
or reworded changes the code, and so the solution. **The code holds what the
lines say and nothing else of the puzzle**: a thing is in it only where a line
names it, and the houses are counted only where a line counts them
([§ The SMT code](#the-smt-code)). Nothing here is wired into the engine: no
crate depends on it and the gate does not run it. It is to
[M2](../plans/m2_nl_to_ir/README.md), natural language to a formal statement
by a model in a loop, what [`c/`](../c/README.md) is to the engine: the same
problem by other means.

| file | what it is |
|---|---|
| [`nl2smt.py`](nl2smt.py) | the wrapper: it runs the others, times them and prints the result. It reads no English and writes no SMT |
| [`english.cfg`](english.cfg) | the context-free grammar NLTK's parser is given: phrase structure, and the vocabulary's words by part of speech |
| [`nltk2smt.rkt`](nltk2smt.rkt) | the converter for NLTK, in Racket: how a tree is read |
| [`lg2smt.rkt`](lg2smt.rkt) | the converter for link grammar, in Racket: how a linkage is read |
| [`script.rkt`](script.rkt) | what the two converters share: the vocabulary, what a sentence means, and the SMT-LIB script written from the meanings |
| [`vocabulary.rktd`](vocabulary.rktd) | every word the pipeline accepts, and what kind of word it is; the wrapper and the converters read it |
| [`puzzles/`](puzzles/) | the input files: the puzzle, and the edits of it the tests run |
| [`test_pipeline.py`](test_pipeline.py) | the tests |
| [`smt/`](smt/) | three hand-written SMT-LIB encodings from 2021, two of them of this puzzle |
| [`prototype/`](prototype/) | the 2021 prototype's two parsing scripts |

`nlp/` and `smt/` were two directories at the repository's root until M1j put
them here. [§ The hand-written encodings](#the-hand-written-encodings) and
[§ The prototype's scripts](#the-prototypes-scripts) are what their two READMEs
said.

## Running it

```sh
python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt
```

```
parser: nltk 3.10.3 · 0.116 s
converter: nltk2smt.rkt · 0.825 s in 2 runs
solver: cvc5 (This is cvc5 version 1.3.0 [git tag 1.3.0 branch HEAD]) · 0.051 s in 2 runs
1 solution, and no other

  ╭────────┬───────────┬───────────────┬────────────┬──────────────┬─────────────╮
  │ house  │ 1         │ 2             │ 3          │ 4            │ 5           │
  ├────────┼───────────┼───────────────┼────────────┼──────────────┼─────────────┤
  │ nation │ Norwegian │ Ukrainian     │ Englishman │ Spaniard     │ Japanese    │
  │ colour │ yellow    │ blue          │ red        │ ivory        │ green       │
  │ pet    │ fox       │ horse         │ snails     │ dog          │ zebra       │
  │ drink  │ water     │ tea           │ milk       │ orange juice │ coffee      │
  │ smoke  │ Kools     │ Chesterfields │ Old Gold   │ Lucky Strike │ Parliaments │
  ╰────────┴───────────┴───────────────┴────────────┴──────────────┴─────────────╯
  Who drinks water?    Norwegian
  Who owns the zebra?  Japanese
```

[`puzzles/zebra.txt`](puzzles/zebra.txt) is the fifteen clues and the two
questions as `prototype/xxx-link.py` has listed them since 2021, a line a
sentence, none reworded. A blank line or one that starts with `#` is skipped.

**The first three lines are the three steps**, each with the tool that did it
and the time it took: the parser, once; the converter and the solver, once for
each solution looked for and once more, to learn that there is no other. The
times are of this machine and this run. The table has a column for each house
that holds something and a row for each kind of thing the lines name; a cell
is blank where the lines name nothing of that kind in that house.
[rich](https://github.com/Textualize/rich) draws it, with the header and the
kinds in bold on a terminal, and without `rich` it is plain columns.

| option | |
|---|---|
| `--parser nltk\|link-grammar` | the parser, and with it the converter; without it, the first of the two that is installed. The output's first line names it |
| `--solver cvc5\|z3\|cvc4` | the solver; without it, the first of the three that is installed. The output's third line names it |
| `--solutions N` | look for up to N solutions. The default is 2, which is enough to say whether there is one only |
| `--show parse` | print what the parser gave for each sentence, NLTK's trees or link-parser's linkages, under a first line that names the parser, and stop |
| `--show readings` | print what the converter reads each tree or linkage of each sentence as, or why it reads nothing, and stop |
| `--show smt` | print the SMT-LIB code, and stop |
| `-` for the file | the sentences on stdin |

Under `--show` the steps that ran are timed on stderr, so that stdout is the
stage's output and nothing else.

An answer exits 0, *no solution* included. A sentence the pipeline does not
take exits 1 with one line on stderr that names the line:

```
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-word.txt
nl2smt: line 2: the word "cat" is not in the vocabulary (vocabulary.rktd)
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-shape.txt
nl2smt: line 2: cannot read "The Spaniard drinks the dog.": "drinks" wants a drink, which "dog" is not (its first tree; none of the 1 is read)
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-parse.txt
nl2smt: line 2: cannot read "The dog the the lives.": no tree, the parser stops at "the", word 3
```

## What it needs

A parser, Racket and a solver, and `rich` for the table. The versions are the
ones this was run with on 2026-11-19.

| tool | version | |
|---|---|---|
| NLTK | 3.10.3 (`python3 -c 'import nltk; print(nltk.__version__)'`) | **the default parser.** Python's `nltk` module: Arch's `python-nltk` with `python-defusedxml`, or `pip install nltk` in a virtual environment. **None of NLTK's data is needed**, no corpus, tagger or model: the chart parser is code, and its grammar is [`english.cfg`](english.cfg) |
| `link-parser` | 5.13.0, English dictionary 5.12.4 (`link-parser --version`) | the other parser, Arch's `link-grammar` package. Its command line is used, since the package has no bindings for this Python. Either parser is enough |
| `racket` | 9.3 [cs] (`racket -v`) | both converters; the user chose it on 2026-11-13. Nothing from Racket's package manager |
| a solver | `cvc5` 1.3.0, `z3` 4.16.0, `cvc4` 1.8 | any one of the three; all three were run and agree |
| `python3` | 3.14.7 | the standard library, and `nltk` for the default parser |
| rich | 15.0.0 | Python's `rich` module, Arch's `python-rich`: it draws the table. Optional: without it the table is plain columns |

It was run with Arch's `python-nltk` 3.10.3 and, before that was installed,
with the same version from `pip` in a virtual environment. `import nltk` and
the chart parser load two modules beside the standard library's, `regex` and
`defusedxml`, and work with NLTK's other dependencies made unimportable
(`click`, `joblib`, `tqdm`, `cloudpickle`). Arch's `python-nltk` names
`python-regex` as a dependency and not `python-defusedxml`, which is why the
row above names both. Arch's `nltk-data`, 5.4 GB installed, is not read: its
grammars are of other domains or of teaching size, and the grammar that can
be read off its Penn Treebank sample gives one tree a sentence, the wrong one
for a question and for the two longest clues. With no `nltk` module the
wrapper takes `link-parser`, and says so on its first line.

Neither of the two submodules the old directories carried is needed, and
neither is re-added: see the two notes below.

## The vocabulary

[`vocabulary.rktd`](vocabulary.rktd) is the whole of it, a list of forms
`(kind tag word ...)`:

- **`value`**: the puzzle's things by kind, five nations, colours, pets, drinks
  and brands. A name of several words is a list, `(Old Gold)`.
- **`verb`**: *owns*, *kept*, *drinks*, *drunk*, *smokes*, *smoked*, *lives*,
  each with the kind of thing it is said of.
- **`noun`**: *house*, *smoker* and *man*, each with the kind of value that
  names one: the *red* house, the *Kools* smoker.
- **`position`** (*first*, *middle*), **`number`** (*five*, and *four* and
  *six*), **`asks`** (*who*, which asks for a nation), and **`grammar`**, the
  few words the sentence shapes are made of.

`nl2smt.py` reads the words: a sentence with a word that is not in the file is
refused, with the word and the line, before a parser sees it. The converters
read the kinds and the tags: which constant a word is, what a verb takes, what
*who* asks for. `four` and `six` are the two words that are not the puzzle's.
They are there so that *There are five houses.* is a line that can be changed.

The vocabulary says what a word **means** and not what part of speech it is,
which link grammar's own dictionary knows and NLTK has to be told:
[`english.cfg`](english.cfg) writes each word once more, as a noun, an
adjective, a verb, a participle. A word of the vocabulary that the grammar
lacks is refused by name under NLTK, with both files named.

## From a sentence to an assertion

### A parser gives several parses, and the first that is read is taken

Neither parser gives a sentence one parse. Link grammar gives every *linkage*
it finds, a set of labelled links between the words, ranked by cost; NLTK's
chart parser gives every *tree* the grammar allows. A converter walks a
sentence's parses in the parser's order and takes **the first whose shape it
knows**. The rule is deterministic, and the comment above each assertion in
the SMT code says which parse it was.

The two converters read different things and say the same: what a parse means
is one of three forms, which [`script.rkt`](script.rkt) defines and writes the
script from, `(houses 5)`, `(fact (= drink-coffee colour-green))`, `(ask nation
(= who drink-water))`. `--show readings` prints them. For every input file the
two parsers end in the same SMT code, line for line under its comments, and
`test_8` holds that.

### Under NLTK: a grammar, and its trees

NLTK is a toolkit and not a parser of English: without a downloaded model it
parses with the grammar it is given. The grammar here is
[`english.cfg`](english.cfg), eight lines of phrase structure and the
vocabulary's words by part of speech, in the format `nltk.CFG.fromstring`
reads:

```
S -> NP VP
S -> Ex Be NP
S -> Wh VP
NP -> Det Nom | Nom
Nom -> N | Adj Nom | Num Nom | N Nom | Nom PP | Nom RC
RC -> Rel VP | RelAdv S
VP -> V | V NP | Be Part | Be PP | VP PP
PP -> P NP | Adv PP
```

It says what a phrase is made of and nothing of what it means: *The Spaniard
drinks the dog.* has a tree, and it is the converter that refuses it. The
wrapper gives NLTK's Earley chart parser (`nltk.parse.EarleyChartParser`) the
sentence's words, each in the grammar's spelling and without the full stop,
and prints every tree as NLTK prints one. That text is an S-expression, which
is what [`nltk2smt.rkt`](nltk2smt.rkt) reads. For the puzzle (2026-11-19,
NLTK 3.10.3):

| line | sentence | trees | read | taken |
|---:|---|---:|---:|---:|
| 1 | There are five houses. | 1 | 1 | 1 |
| 2 | The Englishman lives in the red house. | 1 | 1 | 1 |
| 3 | The Spaniard owns the dog. | 1 | 1 | 1 |
| 4 | Coffee is drunk in the green house. | 1 | 1 | 1 |
| 5 | The Ukrainian drinks tea. | 1 | 1 | 1 |
| 6 | The green house is immediately to the right of the ivory house. | 2 | 1 | **2** |
| 7 | The Old Gold smoker owns snails. | 1 | 1 | 1 |
| 8 | Kools are smoked in the yellow house. | 1 | 1 | 1 |
| 9 | Milk is drunk in the middle house. | 1 | 1 | 1 |
| 10 | The Norwegian lives in the first house. | 1 | 1 | 1 |
| 11 | The man who smokes Chesterfields lives in the house next to the man with the fox. | 5 | 1 | **4** |
| 12 | Kools are smoked in the house next to the house where the horse is kept. | 3 | 1 | **2** |
| 13 | The Lucky Strike smoker drinks orange juice. | 1 | 1 | 1 |
| 14 | The Japanese smokes Parliaments. | 1 | 1 | 1 |
| 15 | The Norwegian lives next to the blue house. | 1 | 1 | 1 |
| 16 | Who drinks water? | 1 | 1 | 1 |
| 17 | Who owns the zebra? | 1 | 1 | 1 |

```sh
python3 nl2smt/nl2smt.py --show smt nl2smt/puzzles/zebra.txt | grep '^; line'     # trees, taken
python3 nl2smt/nl2smt.py --show readings nl2smt/puzzles/zebra.txt | awk '/^line/ {if (s) print n, "of", m, "|", s; s=$0; n=0; m=0; next} {m++; if ($0 !~ /not read/) n++} END {print n, "of", m, "|", s}'     # read
```

A sentence has more than one tree where a phrase can attach in more than one
place, and of those trees one is read: the others leave a house or a man with
nothing to say which one it is, or with two things that do. **A sentence with
no tree is refused with the word the parser stops at**: an edge of an Earley
chart ends only where the words before it can begin a sentence of the grammar,
so the furthest edge is how far the sentence could be read.

**A worked example.** *The green house is immediately to the right of the
ivory house.* has two trees (`python3 nl2smt/nl2smt.py --show parse
nl2smt/puzzles/zebra.txt`). The first hangs *of the ivory house* on the verb:

```
(S
  (NP (Det the) (Nom (Adj green) (Nom (N house))))
  (VP
    (VP (Be is) (PP (Adv immediately) (PP (P to) (NP (Det the) (Nom (N right))))))
    (PP (P of) (NP (Det the) (Nom (Adj ivory) (Nom (N house)))))))
```

The converter finds *to the right* with nothing after *right*, and does not
read it: `not read, what follows right: 0 found`. The second hangs it on
*right*, and is the one taken:

```
(S
  (NP (Det the) (Nom (Adj green) (Nom (N house))))
  (VP
    (Be is)
    (PP
      (Adv immediately)
      (PP
        (P to)
        (NP
          (Det the)
          (Nom (Nom (N right)) (PP (P of) (NP (Det the) (Nom (Adj ivory) (Nom (N house)))))))))))
```

1. `(S NP VP)` is a statement. The subject's noun is *house*, a noun the
   vocabulary says a `colour` names, and *green* stands before it:
   `colour-green`.
2. `(VP Be PP)`: *is*, with a place.
3. `(PP Adv PP)` with *immediately*, and under it `(PP (P to) NP)` whose noun
   is *right* with `(PP (P of) NP)` after it: the house immediately to the
   right of that noun phrase's.
4. That one's noun is *house* again, with *ivory* before it: `colour-ivory`.
5. The subject's house and the house it is put in:

```smt
; line 6, tree 2 of 2: The green house is immediately to the right of the ivory house.
(assert (order colour-ivory colour-green))
```

**The shapes it knows.** A part of speech holds its word or, for a name of
several, its words: `(N house)`, `(N Old Gold)`, `(P next to)`.

| shape | in the tree | example, and what it becomes |
|---|---|---|
| a subject and what is said of it | `(S NP VP)` | every statement |
| the verb's object | `(VP V NP)` | *The Spaniard owns the dog.* `(= nation-Spaniard pet-dog)` |
| a place after the verb | `(VP VP PP)`, with `(PP (P in) NP)` | *The Englishman lives in the red house.* `(= nation-Englishman colour-red)` |
| the passive | `(VP Be Part)`, and the `PP` after it | *Kools are smoked in the yellow house.* `(= smoke-Kools colour-yellow)` |
| a noun named by a value | `(Nom Adj Nom)`, `(Nom N Nom)` | the *red* house, the *Old Gold* smoker |
| the first, the middle house | `(Nom (Adj first) Nom)` | *Milk is drunk in the middle house.* `(= drink-milk 3)` |
| next to | `(PP (P next to) NP)` | *The Norwegian lives next to the blue house.* `(near nation-Norwegian colour-blue)` |
| to the right of | `(VP Be PP)`; `(PP (P to) NP)` whose noun is *right* with `(PP (P of) NP)` after it; `(PP Adv PP)` for *immediately* | `(order colour-ivory colour-green)`; without *immediately*, `(< colour-ivory colour-green)` |
| the man who … | `(Nom Nom (RC Rel VP))` | *the man who smokes Chesterfields* is `smoke-Chesterfields` |
| the man with … | `(Nom Nom (PP (P with) NP))` | *the man with the fox* is `pet-fox` |
| the house next to … | `(Nom Nom (PP (P next to) NP))` | clue 11: `(near smoke-Chesterfields pet-fox)` |
| the house where … | `(Nom Nom (RC RelAdv S))` | clue 12: `(near smoke-Kools pet-horse)` |
| There are N houses. | `(S Ex Be NP)`, `(Nom Num Nom)` | `(define-fun houses () Int 5)` |
| Who …? | `(S Wh VP)` | a `get-value` over the nations the lines name |

A noun is read with what stands beside it, and exactly one thing has to: *the
house* alone is no house, and a house with two phrases after it is none
either. A verb is held to its kind: *The Spaniard drinks the dog.* is refused.

### Under link grammar: linkages

`link-parser` prints the cheapest linkage, and the next for every empty line
that follows the sentence on stdin, so the wrapper sends each sentence with 99
empty lines after it:

```sh
link-parser en -postscript=1 -graphics=0 -echo=1
```

For the puzzle (2026-11-13, link-parser 5.13.0):

| line | sentence | linkages shown | read | taken |
|---:|---|---:|---:|---:|
| 1 | There are five houses. | 8 | 2 | 1 |
| 2 | The Englishman lives in the red house. | 4 | 4 | 1 |
| 3 | The Spaniard owns the dog. | 2 | 2 | 1 |
| 4 | Coffee is drunk in the green house. | 8 | 4 | **2** |
| 5 | The Ukrainian drinks tea. | 8 | 4 | 1 |
| 6 | The green house is immediately to the right of the ivory house. | 30 | 10 | 1 |
| 7 | The Old Gold smoker owns snails. | 2 | 2 | 1 |
| 8 | Kools are smoked in the yellow house. | 2 | 2 | 1 |
| 9 | Milk is drunk in the middle house. | 8 | 4 | **2** |
| 10 | The Norwegian lives in the first house. | 12 | 12 | 1 |
| 11 | The man who smokes Chesterfields lives in the house next to the man with the fox. | 40 | 8 | 1 |
| 12 | Kools are smoked in the house next to the house where the horse is kept. | 40 | 8 | 1 |
| 13 | The Lucky Strike smoker drinks orange juice. | 100 | 9 | 1 |
| 14 | The Japanese smokes Parliaments. | 2 | 2 | 1 |
| 15 | The Norwegian lives next to the blue house. | 8 | 4 | 1 |
| 16 | Who drinks water? | 2 | 2 | 1 |
| 17 | Who owns the zebra? | 2 | 2 | 1 |

The two commands above give it, with `--parser link-grammar`. Line 13 has 129
linkages by the parser's count, and the converter is shown the first 100. **No
sentence of the puzzle has two parses that are read differently**, under
either parser: every linkage, and every tree, of a sentence that is read gives
the same assertion, and `test_6` holds that. The linkages that are not read
are the ones that attach a phrase elsewhere, so that a house or a man is left
with nothing to say which one it is, and the ones below.

**A worked example.** *Coffee is drunk in the green house.* has eight
linkages, and the parser draws the first two so (`printf 'Coffee is drunk in
the green house.\n\n' | link-parser en`). The cheapest reads *drunk* as an
adjective, as in *the sailor is drunk*, the `Pa` link:

```
    +--------------------------Xp--------------------------+
    |                               +--------Js-------+    |
    +------->WV------->+            |   +----Ds**x----+    |
    +--->Wd----+---Ss--+--Pa-+--MVp-+   |     +---A---+    |
    |          |       |     |      |   |     |       |    |
LEFT-WALL coffee.n-u is.v drunk.a in.r the green.a house.n .
```

The converter knows no adjective after *is*, so it does not read that one. The
second is the passive, the `Pv` link, and it is the one taken:

```
    +--------------------------Xp--------------------------+
    |                               +--------Js-------+    |
    +---------->WV---------->+      |   +----Ds**x----+    |
    +--->Wd----+---Ss--+--Pv-+--MVp-+   |     +---A---+    |
    |          |       |     |      |   |     |       |    |
LEFT-WALL coffee.n-u is.v drunk.v in.r the green.a house.n .
```

What the converter is handed is the same linkage in the parser's postscript
form, the words by position and each link as *left right height (label)*:

```
[(LEFT-WALL)(coffee.n-u)(is.v)(drunk.v)(in.r)(the)(green.a)(house.n)(.)]
[[0 8 0 (Xp)][0 3 0 (WV)][0 1 0 (Wd)][1 2 0 (Ss)][2 3 0 (Pv)][3 4 48 (MVp)][4 7 1 (Js)]
[5 7 1 (Ds**x)][6 7 2 (A)]]
```

and it reads the links, not the string:

1. `W` from the wall leads to the subject, word 1, *coffee*: a `drink` in the
   vocabulary, so the constant `drink-coffee`.
2. `S` from the subject leads to its verb, *is*, and `Pv` from *is* to
   *drunk*: a passive. The vocabulary says *drunk* is said of a `drink`, which
   coffee is.
3. `MV` from *drunk* leads to *in*, and `J` from *in* to its noun, *house*.
4. `A` into *house* comes from *green*, a `colour`, which is the kind of value
   the vocabulary says names a house: `colour-green`.
5. The subject's house and the house it is put in are one:

```smt
; line 4, linkage 2 of 8: Coffee is drunk in the green house.
(assert (= drink-coffee colour-green))
```

**The shapes it knows.** A label is a type and then subscripts, so `Ss*s` is
an `S` link and `MVp` an `MV`. These are all the shapes
[`lg2smt.rkt`](lg2smt.rkt) reads, and they compose: *the man who lives next to
the blue house drinks water* is read with no rule of its own.

| shape | links followed | example, and what it becomes |
|---|---|---|
| a subject and its verb | `W` wall to subject, `S` subject to verb | every statement |
| the verb's object | `O` | *The Spaniard owns the dog.* `(= nation-Spaniard pet-dog)` |
| a place after the verb | `MV` verb to preposition, `J` preposition to noun | *The Englishman lives in the red house.* `(= nation-Englishman colour-red)` |
| the passive | `Pv` from *is* or *are* to the participle, then its `MV` | *Kools are smoked in the yellow house.* `(= smoke-Kools colour-yellow)` |
| a noun named by a value | `A` or `AN` into the noun | the *red* house, the *Old Gold* smoker |
| the first, the middle house | `L` from the noun's determiner to *first*; `AN` from *middle* | *Milk is drunk in the middle house.* `(= drink-milk 3)` |
| next to | the preposition *to* with *next* linked before it | *The Norwegian lives next to the blue house.* `(near nation-Norwegian colour-blue)` |
| to the right of | `Pp` from *is* to *to*, `J` to *right*, `M` to *of*, `J` to the noun; `EB` to *immediately* | *The green house is immediately to the right of the ivory house.* `(order colour-ivory colour-green)`; without *immediately*, `(< colour-ivory colour-green)` |
| the man who … | `B` from the noun to the clause's verb, read as a verb | *the man who smokes Chesterfields* is `smoke-Chesterfields` |
| the man with … | `M` from the noun to *with*, `J` | *the man with the fox* is `pet-fox` |
| the house next to … | `M` from the noun to *to*, as next to | clue 11: `(near smoke-Chesterfields pet-fox)` |
| the house where … | `M` to *where*, `C` to the clause's subject, its `S` and `Pv` | clue 12: `(near smoke-Kools pet-horse)` |
| There are N houses. | `W` to *there*, `PF`, `SI` to *houses*, `D` from the number | `(define-fun houses () Int 5)` |
| Who …? | `W` to *who*; the rest as a statement | a `get-value` over the nations the lines name |

A preposition that a noun is linked to by `M` belongs to that noun and is not
read as the verb's place, which is what tells *lives in [the house next to the
man]* from *lives [in the house] [next to the man]*.

**Where that conversion does not follow the links:**

- **A name of several words is joined by position**, from the vocabulary,
  before any link is read: *Old Gold*, *Lucky Strike*, *orange juice*. The
  parser has no complete linkage of a sentence with *Old Gold* inside it: it
  leaves *Old* linked to nothing. An unlinked word that is no part of such a
  name refuses the sentence.
- **The words of a shape are matched as words**: *in*, *with*, *to*, *right*,
  *where*, *is*, *are*, *there*, *houses*. *next to* is the word *next* before
  *to* with a link between them, whatever its label, since the parser names an
  idiom's link by a generated label (`_IBJH` in 5.13.0).

## The SMT code

`--show smt` prints it. It has the shape
[`smt/einstain-problem.smt`](smt/einstain-problem.smt) has by hand: a thing is
an integer constant whose value is the number of its house.

| part | where it comes from |
|---|---|
| `set-option`, `set-logic`, the functions `order` and `near` | [`script.rkt`](script.rkt)'s fixed text, the same two functions as the hand-written file's |
| `(define-fun houses () Int 5)` | the line *There are five houses.* **Without such a line the houses are not counted**: a house's number is 1 or more, and *the middle house* is refused, since no line says which one it is |
| a `declare-fun` and `(<= 1 x houses)` for each thing **a line names**, `(<= 1 x)` where no line counts the houses | the sentences, questions among them. The vocabulary gives the constant its name |
| one `distinct` for the things of a kind that are named, where they are two or more | the vocabulary's kinds. **It is the one thing in the code that no line says**: that two nations, or two colours, are in two houses is what a kind is |
| one `assert` for each statement, in the order of the lines | the sentences |
| `(check-sat)` and a `get-value` of every constant declared | fixed |
| an `echo` and a `get-value` for each question | the sentences: *Who drinks water?* asks the solver for `(= nation-X drink-water)` of each nation **the lines name** |
| `(assert (not (and (= x 1) ...)))` | a solution already found, passed back to the converter |

The names are the vocabulary's: `nation-Englishman`, `smoke-Old-Gold`. The
wrapper prints the table from them and from nothing else.

**Only what the lines say.** [`puzzles/new-sentences.txt`](puzzles/new-sentences.txt)
is seven lines that name twelve things and count no houses, and its code
declares those twelve and no count:

```
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/new-sentences.txt
parser: nltk 3.10.3 · 0.105 s
converter: nltk2smt.rkt · 1.208 s in 3 runs
solver: cvc5 (This is cvc5 version 1.3.0 [git tag 1.3.0 branch HEAD]) · 0.019 s in 3 runs
more than 2 solutions; the first 2 (--solutions N looks for more)

  ╭────────┬──────────┬──────────┬───────┬────────┬───────┬──────╮
  │ house  │ 1        │ 2        │ 3     │ 4      │ 5     │ 6    │
  ├────────┼──────────┼──────────┼───────┼────────┼───────┼──────┤
  │ nation │ Spaniard │ Japanese │       │        │       │      │
  │ colour │          │ red      │ green │ yellow │ ivory │ blue │
  │ pet    │          │ zebra    │ dog   │        │       │      │
  │ drink  │ water    │          │       │        │       │ tea  │
  │ smoke  │          │          │ Kools │        │       │      │
  ╰────────┴──────────┴──────────┴───────┴────────┴───────┴──────╯
  Who lives in the green house?  nobody the sentences name

  ╭────────┬───────┬──────────┬──────────┬───────┬──────╮
  │ house  │ 1     │ 2        │ 3        │ 4     │ 5    │
  ├────────┼───────┼──────────┼──────────┼───────┼──────┤
  │ nation │       │ Spaniard │ Japanese │       │      │
  │ colour │ red   │ green    │ yellow   │ ivory │ blue │
  │ pet    │ dog   │          │ zebra    │       │      │
  │ drink  │ water │          │          │       │ tea  │
  │ smoke  │ Kools │          │          │       │      │
  ╰────────┴───────┴──────────┴──────────┴───────┴──────╯
  Who lives in the green house?  Spaniard
```

Each table is one solution of many, the solver's choice among them: the two
answer the question differently, and nothing in the output says what all the
solutions have in common.

The puzzle without clue 15 has no line that names the blue house, so its 32
solutions leave one house with no colour. Until 2026-11-19 the script declared
every value of the vocabulary, whether a line named it or not, and took the
number of houses from the vocabulary where no line counted them, so that seven
lines came out as a table of five houses and twenty-five things, most of it
the solver's free choice.

**How it says whether a solution is the only one.** After a model, the wrapper
passes it back to the converter as an argument, the converter writes the same
script with that model ruled out, and the solver is asked again. One solution
and then `unsat` is *1 solution, and no other*. The wrapper stops after
`--solutions N`, so the puzzle without clue 15 answers *more than 2 solutions*
by default and *32 solutions, and no other* under `--solutions 40`
(`python3 nl2smt/nl2smt.py --solutions 40 nl2smt/puzzles/zebra-minus-15.txt`).
That is a converter run and a solver run for each solution, 33 of each there,
and it is the converter's that take the time: Racket is started for each.

## The tests

```sh
python3 nl2smt/test_pipeline.py
```

Plain `unittest`, each test on an input file under [`puzzles/`](puzzles/):
**nine tests for each parser that is installed, and one across them**. They
are skipped, with what is missing named, where no parser, `racket` or every
solver is installed. On 2026-11-19 all 19 passed with both parsers, and the
10 of link grammar alone with no `nltk` module.

| test | input | what it shows |
|---|---|---|
| 1 | `zebra.txt` | one solution and no other, the table above; the Norwegian drinks water and the Japanese owns the zebra. The clue assertions are the same set as `smt/einstain-problem.smt`'s |
| 2 | `zebra-minus-15.txt`, clue 15 removed | more than one solution. Against the hand-written twin `smt/einstain-problem-minus-15.smt`, two comparisons: the same set of clue assertions once the constants are renamed and the two sides of `=` and `near` are put in no order, and then the same 32 solutions, model for model, each without its blue house, which no line of the file names |
| 3 | `zebra-reversed.txt`, `zebra-shuffled.txt` | the assertions come in the order of the lines, each sentence's assertion what it was; the solution is the same |
| 4 | `zebra-swapped.txt`, *Norwegian* and *Ukrainian* exchanged | still one solution, and it is the other one: the Ukrainian is in the first house and drinks water, the Norwegian in the second drinks tea. The hand-written file with the two constants exchanged gives the same model |
| 5 | the five above, `zebra-contradiction.txt`, `new-sentences.txt` | no two of the seven have the same SMT code. Six sentences that are in no version of the puzzle become the right assertions, and a seventh the right question, asked of the two nations the file names. That file's code holds the twelve things its lines name, no other and no count of the houses, and so do its solutions. The puzzle with *The Norwegian drinks milk.* added has no solution |
| 5 | `refused-word.txt` | *cat* is refused by name, with its line |
| 5 | `refused-parse.txt`, `refused-shape.txt` | a sentence the parser cannot parse, and one the converter cannot read, are refused with the line and the reason: NLTK has no tree and names the word it stops at, link-parser links a word to nothing |
| 6 | `zebra.txt` | every linkage, and every tree, of a sentence that is read is read alike |
| 7 | `zebra.txt` | every installed solver gives the same solution |
| 8 | the seven of test 5 | every installed parser ends in the same SMT code, line for line under its comments |

The comparison with the hand-written files needs a table of names, since the
two were written apart: `house-white` there is `colour-ivory` here, `nation-england`
is `nation-Englishman`, `drink-tee` is `drink-tea`. It is in the test, and it
is the only place the hand-written names appear. The framing differs in text
and not in meaning (pairwise `!=` there, `distinct` here), which is why the
second comparison is of the solutions.

The shuffled file is the puzzle's lines under Python's
`random.Random(15).shuffle`.

## What it does not do

- **No general English.** A word outside the vocabulary is refused; a sentence
  in a shape that is not in the tables above is refused, however ordinary:
  *The green house is the first house.*, or *and* between two clues.
- **Under NLTK, no parser of English at all.** Link grammar parses the
  sentences with a dictionary and a grammar of its own, and the converter
  picks among what it gives. NLTK parses with [`english.cfg`](english.cfg),
  which was written for these sentences: there the limit on the sentence
  shapes is in the grammar as well as in the converter.
- **No second puzzle, and no second direction.** Nothing goes from a model
  back to sentences: the table is the constants' names.
- **No understanding past the kind of a verb's object.** *The dog owns the
  Spaniard.* is refused, and *The Spaniard lives with the dog.* is read as the
  two being in one house.
- **Nothing links a solver into Ein.** M3 stays dropped, and no Ein program is
  written; [`examples/zebra.ein`](../examples/zebra.ein) is the same puzzle
  for the engine, written by hand.
- **It decides nothing for M2.** [S2.6.4](../plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
  measures whether link-grammar output helps a model formalize. This shows
  only that the installed parsers give usable structure for these sentences.
- **It is not in the gate.** Its tests need a parser, Racket and a solver
  where they run.

## Quirks of the tools, worked around here

- **`link-parser` leaves *Old* unlinked** in *the Old Gold smoker*, and
  reports no complete linkage. See above: names are joined by position. Under
  NLTK a name of several words is one rule of the grammar, `N -> 'Old' 'Gold'`.
- **`link-parser`'s cheapest linkage of *X is drunk in …* is the adjective.**
  The converter passes over it. The grammar NLTK is given has *drunk* as a
  participle only.
- **A link's third number, a height for drawing, is not always set** in
  5.13.0's postscript output, and differs from run to run. It is not read. It
  does mean `--show parse` is not the same text twice under link grammar.
- **Past its `limit` of 1000 linkages the parser samples.** No sentence here
  comes near it.
- **NLTK's trees have no cost**: they come in the order the chart gives them,
  and that order is the parser's own. The converter is shown the first 100.
- **NLTK prints a tree's words bare**, so a full stop in a tree would not be
  read back as an S-expression. The wrapper leaves the sentence's last mark
  out of what it gives the parser.
- **`z3` prints an `echo` without its quotes, `cvc5` and `cvc4` with them.**
  The wrapper's reader drops quotes.
- **Racket is started once for each solution**, and loads from source, two
  modules now. The second line of the output is what that costs.

## The hand-written encodings

[`smt/`](smt/) holds what the Zebra puzzle and 4-queens look like when written
by hand for a solver: `einstain-problem.smt`,
`einstain-problem-minus-15.smt` and `4-queens.smt`, from 2021. The two Zebra
files are what the pipeline's output is compared with.
[M10](../plans/m10_external_benchmarks/README.md) counts the three as three of
its benchmark corpus's encodings, already written, so their content does not
change. **M3 (SMT integration) was dropped 2026-08-18**: Ein has no solver
back-end and none is planned; see [`plans/README.md`](../plans/README.md)
§ Roadmap.

> **The `CVC4` submodule was deinitialised at M1a
> S1a.10.5.**
> It pointed at `CVC4/CVC4` at version 1.8 (2021), at `smt/CVC4`, and was never
> built by anything here; what it cost was a `git clone --recurse-submodules`
> fetching a large repository for a dropped milestone. Nothing that *reads*
> these files needed it — M10's benchmark uses **CVC5**, which is a different
> program with a different name, and the pipeline here runs whichever solver
> is installed. Re-add it if some later stage actually wants 1.8's source:
>
> ```sh
> git submodule add https://github.com/CVC4/CVC4.git nl2smt/CVC4
> ```

External-tech notes on solvers live in
[`docs/lib/02-solvers-csp-sat-smt.md`](../docs/lib/02-solvers-csp-sat-smt.md).

- CVC4 user manual: <http://cvc4.cs.stanford.edu/wiki/User_Manual>
- CVC4 source: <https://github.com/CVC4/CVC4>

## The prototype's scripts

[`prototype/`](prototype/) holds two throwaway scripts from the 2021
prototype's look at dependency parsing. `xxx-link.py` is the *linkgrammar
script*: it imports the `linkgrammar` Python bindings, which are not installed
here, holds the Zebra puzzle's sentences as text and prints each one's
linkage. `nl2smt.py` is what replaces it, and
[`puzzles/zebra.txt`](puzzles/zebra.txt) is its sentence list. `xxx.py` is the
same sentences, in English and in Russian, under NLTK, beside the NLTK book's
toy grammar and `nltk.ChartParser`; [`english.cfg`](english.cfg) is a grammar
for those sentences. Neither is run by
anything. They are kept because
[M2 S2.6.4](../plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
names them as its starting point: a *measured* A/B on whether feeding
link-grammar output to the LLM improves the formalization. It was a phase of
its own (P2.5) until the 2026-08-23 reshape of M2 around
[`EinAf.md`](../plans/m2_nl_to_ir/EinAf.md) made it one arm of the
representation ablation; nothing about the experiment changed.

> **The `link-grammar` submodule was deinitialised at M1a
> S1a.10.5.**
> It pointed at `opencog/link-grammar`, at `nlp/link-grammar`, and was never
> checked out, so what it cost was a `git clone --recurse-submodules` fetching
> it for an experiment that has not run. The pipeline here did not need it:
> the system package's `link-parser` was enough. S2.6.4 re-adds it in one
> command if it wants the source or the bindings — and its possible outcome is
> "deprecate the submodule", so registering it now would pre-empt the decision
> the stage exists to take:
>
> ```sh
> git submodule add https://github.com/opencog/link-grammar.git nl2smt/link-grammar
> ```

## Reading

Parsing and grammar:

- <https://www.nltk.org/book/ch08.html>
- <https://nlp.stanford.edu/software/lex-parser.shtml>
- <http://www.link.cs.cmu.edu/link/>
- <https://www.abisource.com/projects/link-grammar/#download>
- <https://www.abisource.com/projects/link-grammar/api/index.html#sents>
- <https://www.abisource.com/projects/link-grammar/dict/index.html>
- <https://www.abisource.com/projects/link-grammar/dict/summarize-links.html>
- <https://github.com/opencog/link-grammar>
- <https://github.com/opencog/relex>
- <https://wiki.opencog.org/w/RelEx_Dependency_Relationship_Extractor>
- <https://github.com/opencog>
- <https://wiki.opencog.org/w/Real_World_Reasoning>
- <https://en.wikipedia.org/wiki/Category:Grammar_frameworks>
- <https://ru.wikipedia.org/wiki/%D0%93%D1%80%D0%B0%D0%BC%D0%BC%D0%B0%D1%82%D0%B8%D0%BA%D0%B0_%D1%81%D0%BE%D1%81%D1%82%D0%B0%D0%B2%D0%BB%D1%8F%D1%8E%D1%89%D0%B8%D1%85>

Semantic networks and WordNet:

- <http://people.duke.edu/~mccann/mwb/15semnet.htm>
- <https://en.wikipedia.org/wiki/Semantic_network#Examples>
- <https://en.wikipedia.org/wiki/WordNet#Licensed_vs._Open_WordNets>
- <http://trimc-nlp.blogspot.com/2015/06/python-nltk-and-wordnet.html>
- <https://en.wiktionary.org/wiki/Wiktionary:Semantic_relations>
- <https://wordnet.princeton.edu/documentation>
- <https://medium.com/parrot-prediction/dive-into-wordnet-with-nltk-b313c480e788>
- <https://en.wikipedia.org/wiki/MultiNet>
- <http://www.jfsowa.com/pubs/semnet.htm>

Other:

- <http://graphit-lang.org/getting-started>
