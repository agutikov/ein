# S1k.10.1, its examples — drawings from the internet and drawn here, read as Ein knowledge bases

**Phase:** [P1k.10](README.md) · **Stage:** S1k.10.1, the examples part · **Status:** done 2026-11-18; the design
part of S1k.10.1 is after it.

The user's task, 2026-11-18, verbatim:

> Find some examples from internet or draw yourself, e.g. state machine, an algorithms, class diagram, C4 diagrams,
> https://graphviz.org/gallery/ ... whatever; and try convert them into Ein KB, record what succeded, what failed and
> why.

and, during the stage, the user's rulings, verbatim:

> p1f.30 - each generated ein is loaded, but no need to saturate it, because it has no rules; undirected arcs can be
> replaced with 2 directed, main requirement are labels, but for experiment we can allow default labels - options
> are a single one or all different.

**In short.** 28 drawings: 25 read by reading 1 (17 downloaded, 8 drawn here) and 3 of Ein's own drawings read by
reading 2. **Read strictly, reading 1 loads 2 of the 25**: the turnstile and Ein's crate graph, the two drawn
clean. Every one of the 17 downloaded drawings fails, 13 for an unlabelled arrow (744 arrows) and 12 for a label that
is no name. **With the labels repaired and a default label for an unlabelled arrow, all 25 load, every root alive,
and every two-argument fact comes back from `ein render kb` as its arrow** (1,438 of 1,438). What does not come back
as drawn is a fact of an atom alone (an octagon labelled `(Dog)`, 10 facts) and every label the repair renamed.
**Reading 2 reads Ein's own drawings back to their knowledge base**, but for a fact the renderer leaves out, and only
if a loop is read the right way, which today's drawing does not say. Of the two default-label options, **one label
for every unlabelled arrow** is what the evidence favours ([below](#unlabelled-arrows-a-single-default-or-all-different)).

## What was run

- **The prototype**, [`s1k.10.1_examples/graph2ein.py`](s1k.10.1_examples/graph2ein.py), throwaway: `read1`, `read2`
  and `survey`. Graphviz parses (`dot -Tjson0`); `gvpr`'s `ishtml` says which node labels are HTML, which the JSON
  does not. `python3 s1k.10.1_examples/graph2ein.py survey` re-takes every number on this page from the downloads in
  `build/p1f30-external/` (gitignored, one directory a file, its URL in `SOURCE_URL` beside it) and the drawings in
  [`s1k.10.1_examples/drawn/`](s1k.10.1_examples/drawn/). It writes
  [`s1k.10.1_examples/results.json`](s1k.10.1_examples/results.json) (a downloaded drawing's counts, none of its text)
  and the full record to `build/p1f30-external/results.json`.
- **The binary**: `build/review-bin/ein-head-b5fb8d70`. Each generated program is **loaded and not saturated**
  (`ein solve FILE --no-search`, as ruled) and drawn back (`ein render kb FILE`). Only Ein's two original programs,
  which have rules, are saturated (`ein saturate --dump`), to get the knowledge base their drawing is of.
- **No relation is declared.** A relation nothing declares needs no `:why`, so no program needs `--no-require-why`,
  and the blind enumerator guesses no fact of it, so R is {∅} in every program. A probe that declared one relation
  of two labels, `(relation owns P T)`, made a bare `ein solve` of a ten-fact program describe R as the whole cube of
  130 candidates, and the declaration's two facts appear in `ein render kb` (an octagon `(relation)` and a loop
  `owns -[relation]-> owns`). Reading 2 writes declarations without `:why` and loads with `--no-require-why`
  ([Q-M1f.62](../../../docs/decisions/cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md)): the drawing
  has no `:why`.
- **The name check is held to the binary.** Every single-token label the survey met, and every name the repairs
  wrote, 1,394 words, was loaded as `(r W x)` and as `(W a b)` by the binary: the prototype's check and the binary
  agree on all 2,788.
- **Three runs per drawing, and a variant where one shows a repair.** *strict*: the rules as written, every
  failure reported, a program written only when nothing is refused. *(a)* and *(b)*: the label repairs below, and
  an unlabelled arrow given `edge` (a) or `e1`, `e2`, … in written order (b). A *variant* adds one repair of a
  loss (clusters, ports, `dir`, renaming, a third argument, the sentence kept, the colour as the relation) on the
  drawings that have it. Undirected arcs are two facts in every run (ruled).

**The label repairs (a, b).** A node's label: an HTML label's first line of text, split at `|` where the shape is a
record; a record label the node's id; an empty label the id; a label of several lines its first line; then, if it
is still no name, its **slug** (folded to ASCII, every run of other characters one `-`). An arrow's label: an
`xlabel` where there is no `label`; then the slug of the whole label, or, where nothing folds to ASCII, the
**escape** (percent-encoding every byte that is no name character, `%` itself among them: `%` is a name character,
so the escape is reversible). A kernel word gets a trailing `_`. Two nodes that come to one name each become
`name@id`.

## The examples

*strict*: what the strict reading did (refused: the number of refusals). *(a)*, *(b)*: facts written, and relations.
*Round trip*: of the program's two-argument facts, how many `ein render kb` draws as their arrow, and of the input
drawing's arrows, how many come back with the same three labels. Programs of this repo's drawings are in
[`s1k.10.1_examples/out/`](s1k.10.1_examples/out/); a downloaded drawing's stay in `build/p1f30-external/`.

| # | example | kind | source | nodes / arrows | strict | (a) facts, relations | (b) facts, relations | round trip (a) | what failed or was lost |
|---|---|---|---|---|---|---|---|---|---|
| 1 | [turnstile-fsm](s1k.10.1_examples/drawn/turnstile-fsm.gv) | state machine | drawn here | 3 / 5 | **loaded as is** | 5, 3 | 5, 3 | 5/5; verbatim 5/5 | nothing: the clean case |
| 2 | gallery-fsm | state machine (LR(0)) | https://graphviz.org/Gallery/directed/fsm.gv.txt | 9 / 14 | refused (14) | 14, 8 | 14, 8 | 14/14; verbatim 0/14 | every arrow label has parentheses: `SS(B)` → `SS-B` |
| 3 | pressure-valve | state machine | https://danielk.developer.irf.se/phd-course-software-development/tutorials/diagrams/ (its third DOT block) | 5 / 5 | refused (7) | 5, 5 | 5, 5 | 5/5; verbatim 0/5 | labels with spaces (`Low flow`, `Request recived`); two point nodes of the empty label |
| 4 | [euclid-flowchart](s1k.10.1_examples/drawn/euclid-flowchart.gv) | algorithm flowchart | drawn here | 8 / 9 | refused (11) | 9, 3 | 9, 7 | 9/9; verbatim 0/9 | statements as labels (`a := a - b` → `a-=-a-b`); 5 unlabelled steps |
| 5 | gallery-cluster | processes in frames | https://graphviz.org/Gallery/directed/cluster.gv.txt | 10 / 13 | refused (13) | 13, 1 | 13, 13 | 13/13; verbatim 0/13 | every arrow unlabelled; 2 frames lost (kept by `+clusters`: 21 facts) |
| 6 | gallery-UML_Class_diagram | UML class diagram (HTML) | https://graphviz.org/Gallery/directed/UML_Class_diagram.gv.txt | 9 / 11 | refused (16) | 11, 6 | 11, 11 | 11/11; verbatim 0/11 | 7 HTML labels; 8 arrows unlabelled, 2 of them with an `xlabel`; `dir=back` on 8; 6 arrow ends at table cells (ports) |
| 7 | [uml-classes](s1k.10.1_examples/drawn/uml-classes.gv) | UML class diagram (records) | drawn here | 5 / 5 | refused (7) | 5, 4 | 5, 5 | 5/5; verbatim 0/5 | record labels; inheritance unlabelled (its meaning is `arrowhead=empty`); multiplicities in `headlabel`/`taillabel` lost |
| 8 | gallery-datastruct | data structure | https://graphviz.org/Gallery/directed/datastruct.gv.txt | 13 / 17 | refused (30) | 17, 1 | 17, 17 | 17/17; verbatim 0/16 | record labels, 3 pairs of identical records; 34 arrow ends at record fields (kept by `+ports`: 42 facts) |
| 9 | [c4-context](s1k.10.1_examples/drawn/c4-context.gv) | C4 system context | drawn here, after the C4 model's banking example | 4 / 4 | refused (8) | 4, 4 | 4, 4 | 4/4; verbatim 0/4 | name, `[type]` and description in one label; arrows are sentences; `+source` keeps the sentence and the round trip loses every arrow (0/4) |
| 10 | c4-container | C4 container | https://danielk.developer.irf.se/phd-course-software-development/tutorials/diagrams/ (its first DOT block) | 6 / 7 | refused (13) | 7, 7 | 7, 7 | 7/7; verbatim 0/7 | HTML tables as labels; sentences with `[SSH]`, `[JSON/HTTPS]` |
| 11 | [ein-crates](s1k.10.1_examples/drawn/ein-crates.gv) | dependency graph | drawn here, from each crate's `Cargo.toml` | 8 / 14 | **loaded as is** | 16, 3 | 16, 3 | 14/14, 2 `(X)` as octagons; verbatim 14/14 | the two dev-only crates are no arrow's ends: `(ein-corpus)`; 2 frames lost |
| 12 | gallery-go-package | dependency graph | https://graphviz.org/Gallery/directed/go-package.gv.txt | 20 / 49 | refused (49) | 49, 1 | 49, 49 | 49/49; verbatim 0/49 | every arrow unlabelled; the labels `regexp/syntax`, `unicode/utf8` are names |
| 13 | gallery-unix | version history | https://graphviz.org/Gallery/directed/unix.gv.txt | 41 / 49 | refused (80) | 49, 1 | 49, 49 | 49/49; verbatim 0/49 | 31 labels with a space (`5th Edition`); every arrow unlabelled |
| 14 | gallery-pprof | call graph | https://graphviz.org/Gallery/directed/pprof.gv.txt | 21 / 28 | refused (49) | 29, 22 | 29, 22 | 28/28, 1 `(X)`; verbatim 0/28 | every arrow label a number (` 393`): 28 arrows, 21 relations; the legend is a node no arrow touches |
| 15 | [family-tree](s1k.10.1_examples/drawn/family-tree.gv) | family tree | drawn here | 6 / 7 | refused (1) | 8, 4 | 8, 4 | 7/7, 1 `(X)`; verbatim 1/7 | `Zoë`; two people called John, one atom unless repaired |
| 16 | gallery-lion_share | pedigree | https://graphviz.org/Gallery/directed/lion_share.gv.txt | 47 / 51 | refused (68) | 51, 1 | 51, 51 | 51/51; verbatim 0/46 | 17 marriage nodes of the empty label; every arrow unlabelled and drawn `dir=none` |
| 17 | gallery-kennedyanc | family tree | https://graphviz.org/Gallery/directed/kennedyanc.gv.txt | 14 / 22 | refused (36) | 44, 1 | 44, 22 | 44/44; verbatim 0/22 | undirected; the relation is the colour (parent, spouse); first lines collide: two `John Fitzgerald Kennedy` |
| 18 | gallery-ER | ER diagram | https://graphviz.org/Gallery/neato/ER.gv.txt | 12 / 12 | refused (6) | 24, 4 | 24, 9 | 24/24; verbatim 6/12 | undirected; three attribute nodes all labelled `name`; cardinalities `1`, `n`, `m` as labels |
| 19 | gallery-traffic_lights | Petri net | https://graphviz.org/Gallery/neato/traffic_lights.gv.txt | 14 / 16 | refused (16) | 16, 1 | 16, 16 | 16/16; verbatim 0/16 | every arc unlabelled; transitions are nodes with arrows out |
| 20 | gallery-networkmap_twopi | network map | https://graphviz.org/Gallery/twopi/networkmap_twopi.gv.txt | 426 / 425 | refused (426) | 850, 1 | 850, 425 | 850/850; verbatim 0/421 | undirected, every arc unlabelled; IP addresses are names; 5 IPs on two nodes each; its image files not downloaded |
| 21 | gallery-happiness | mind map | https://graphviz.org/Gallery/twopi/happiness.gv.txt | 78 / 77 | refused (78) | 155, 2 | 155, 78 | 154/154, 1 `(X)`; verbatim 0/77 | undirected, unlabelled; a credit line `© 2020-2022 …` as a node no arrow touches |
| 22 | gallery-fdpclust | undirected, nested frames | https://graphviz.org/Gallery/undirected/fdpclust.gv.txt | 7 / 6 | refused (9) | 12, 1 | 12, 6 | 12/12; verbatim 0/6 | 2 arcs end at a frame (`e -- clusterB`), which no reading 1 node is |
| 23 | gallery-git | concepts and commands | https://graphviz.org/Gallery/directed/git.gv.txt | 8 / 10 | refused (17) | 10, 7 | 10, 10 | 10/10; verbatim 0/10 | HTML labels; commands as arrow labels (`git commit`); ports |
| 24 | [kernel-words](s1k.10.1_examples/drawn/kernel-words.gv) | Ein's kernel words | drawn here | 42 / 23 | refused (25) | 26, 22 | 26, 22 | 23/23, 3 `(X)`; verbatim 1/23 | every kernel word refused or changed ([below](#10-kernel-words-as-labels)) |
| 25 | [edge-cases](s1k.10.1_examples/drawn/edge-cases.gv) | every case to decide | drawn here | 25 / 18 | refused (13) | 19, 19 | 19, 19 | 17/17, 2 `(X)`; verbatim 9/17 | one of each kind below |
| 26 | [three-houses](s1k.10.1_examples/ein/three-houses.kb.dot) | Ein's drawing | [`three-houses.ein`](s1k.10.1_examples/ein/three-houses.ein), the running example | 21 / 35 | reading 1 refused (35) | reading 2: 25 facts | | KB 25 of 26 | `(transitive left-of)` is not drawn |
| 27 | [common-belief](s1k.10.1_examples/ein/common-belief.kb.dot) | Ein's drawing | [`01_common_belief.ein`](../../../examples/beliefs/01_common_belief.ein) | 21 / 32 | reading 1 refused (26) | reading 2: 21 facts | | KB 21 of 21 | nothing; the rules and `:why` are no part of a KB |
| 28 | [turnstile-kb](s1k.10.1_examples/ein/turnstile-kb.kb.dot) | Ein's drawing | row 1's program | 3 / 5 | reading 1: 0 refused | reading 2: 5 facts | | KB 3 of 5 | its two loops are facts of two arguments, which reading 2 takes for one |

Every (a), (b) and variant program loads with its root alive. A drawing's verbatim count can be smaller than its
arrows (datastruct 16 of 17, lion_share 46 of 51, the network map 421 of 425): arrows with the same three labels are
one triple.

## The round trip

**Reading 1 and back.** Every two-argument fact is drawn as its arrow, with the relation as its label: 1,438 of
1,438 over the 25 drawings under (a), and the same under (b) and in every variant but `+source`. What does not come back
as drawn:

- **A fact of an atom alone**, `(Dog)`, is drawn as an octagon labelled `(Dog)` with no arrow, and reading 1 refuses
  that label (parentheses): 10 facts in 6 drawings. The node came in as `Dog` and comes back as `(Dog)`.
- **A renamed label** comes back renamed: only the turnstile and the crate graph came back with every arrow's three
  labels as drawn; ER 6 of 12, edge-cases 9 of 17, family-tree 1 of 7 and kernel-words 1 of 23 (the arrows whose
  three labels were names); the other 19 drawings 0.
- **A `:source` sentence is written into the arrow's label** after the relation: `(Sends-e-mail-using
  Internet-Banking-System E-mail-System :source "Sends e-mail using")` is drawn with the label `Sends-e-mail-using
  Sends e-mail using`, which is no name, so `c4-context.a+source` comes back with 0 of its 4 arrows.
- **A fact of three arguments is an octagon** with arrows `#1`, `#2`, `#3`, which reading 1 refuses: in
  `edge-cases.a+numbered` the two parallel `calls` arrows, written `(calls a b 1)` and `(calls a b 2)`, come back as
  two octagons, and its other 16 facts of two arguments as their arrows.
- **A fact another fact points at is an octagon** too: written by hand,
  [`c4-context.nested.ein`](s1k.10.1_examples/out/c4-context/c4-context.nested.ein) keeps C4's `[XML/HTTPS]` as
  `(technology (Gets-account-information-from-and-makes-payments-using Internet-Banking-System
  Mainframe-Banking-System) XML/HTTPS)`, which Ein holds and the drawing cannot write (DOT has no arrow at an
  arrow), and `ein render kb` then draws the arrow's fact as an octagon, which only reading 2 reads.
- **`is-a` is drawn its own way** (dashed, hollow head, its object a box): its label is still `is-a`, and it reads
  back.

**Reading 2 and back**, on Ein's drawings ([section 11](#11-reading-2-on-eins-own-drawings)).

## Unlabelled arrows: a single default, or all different

16 of the 25 drawings have unlabelled arrows, 753 of them, and in 10 every arrow is unlabelled. Run both ways
(`python3 s1k.10.1_examples/graph2ein.py survey`, the `same KB reversed` column; the rule below by hand):

| | (a) one label, `edge` | (b) each its own, `e1`, `e2`, … |
|---|---|---|
| facts | 1,448 over the 25 | 1,448: the same, since no two unlabelled arrows join one pair in one direction in any drawing (0 of 753), so (a) merges none |
| relations | one more per drawing (the go-package's 49 arrows: 1) | one per arrow (49; the network map's 425 arcs: 425) |
| round trip | every arrow back | every arrow back, each relation its own colour |
| the drawing's arrows written in reverse order | **the same KB in 16 of 16** | a different KB in 15 of 16: `e1` is the first unlabelled arrow written. The one equal is edge-cases, with one default label |
| a rule over the unlabelled arrows | one rule, `:match (edge ?x ?y)`: a reachability rule over the go-package's KB loads and answers | refused at load: *rule 'reach-step': its :match writes (?r ?x ?y) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds*; it takes a parameter and 49 activators |
| what a reader can tell apart | which pairs an unlabelled arrow joins, and that it is unlabelled; not one unlabelled arrow from another of the same ends (none in these drawings) | each arrow from every other, by a name the file's order gave it, which says nothing a second drawing shares |

Neither keeps what the drawing meant where the meaning is elsewhere: Kennedy's 22 arcs are four relations told by
colour (parent, spouse), and under (a) all are `edge`, under (b) 22 relations; the variant
`gallery-kennedyanc.repaired+colour` reads the colour as the relation and gets 4.

**The evidence favours (a)**: the same facts in every drawing, a KB that does not depend on the order the arrows are
written in, and one relation a rule can read. (b) keeps parallel unlabelled arrows apart, and none of the 753 had a
parallel.

## The failure kinds

Counts are of the strict reading over the 25 drawings reading 1 reads (`survey`'s last table); *refused* stops the
strict reading, *lost* is read and not kept.

### 1. A label that is no name — refused, 211 labels in 17 drawings

| why | labels | drawings | a case | slug | escape |
|---|---|---|---|---|---|
| a space | 84 | 9 | `5th Edition` (unix) | `5th-Edition` | `5th%20Edition` |
| several lines | 44 | 5 | `deflate\n62 (14.76%)\nof 384 (91.43%)` (pprof); C4's name, `[type]` and description | first line, `deflate` | |
| empty | 21 | 4 | lion_share's 17 marriage nodes, `label=""` | the id, `marr0001` | |
| HTML | 19 | 4 | `<b>«interface» I/O</b> \| + property …` (UML) | first field, `interface-I/O` | |
| a record | 18 | 2 | `<f0> 0x10ba8\| <f1>` (datastruct) | the id, `node0` | |
| punctuation | 17 | 2 | `SS(B)`, `S($end)` (fsm); `?x`, `:why`, `a;b` (edge-cases) | `SS-B`, `S-$end`, `x`, `why`, `a-b` | `SS%28B%29` |
| not ASCII | 7 | 3 | `Zoë`, `Zürich`, `Москва`, `© 2020-2022 …` | `Zoe`, `Zurich`, nothing (the id `e`) | `Zo%C3%AB`, `%D0%9C%D0%BE…` |
| a module's full name | 1 | 1 | `std.macro.forall` (edge-cases) | `std-macro-forall` | |

**What is a name, more than a reader expects**: `'*'`, `'='`, `[unknown]`, `regexp/syntax`, `S-$end`, `<=`, `->`,
`1`, `109.217.106.212.`, `3.43322790286038071e-06`, `{x}`: quotes, brackets, braces, slashes, `<`, `>`, `=`, `$`, `@`
and `%` are name characters, so most punctuation survives the slug and the escape is all name characters.

**A dotted label loads or not by what lies beside the program.** A name `a.b` is the full name of a module's name
where `a` is a module the program could import, and is refused unless it is imported
([Q-M1f.238](../../../docs/decisions/imports/Q-M1f.238-a-full-name-of-a-module-no-import-brings-is.md)): *'a.b' is
a full name of a, which no import of the program brings*. `std.macro.forall` is refused anywhere; `a.b` loaded in an
empty directory and was refused once a file `a.ein` lay beside the program. So `2.8-BSD`, `PWB-1.0` and the IP
addresses load here and would not beside a file `2.ein`, `PWB-1.ein` or `109.ein`. The prototype checks the standard
library and the files beside the program.

**The slug collided once in 25**, and not by itself: Kennedy's `John Fitzgerald Kennedy\nb. 29.5.1917 …` and
`John Fitzgerald Kennedy\nb. 25.11.1960 …` differ, and their first lines do not; the repair made them
`John-Fitzgerald-Kennedy@I1` and `@I6`.

### 2. Two nodes of one label — lost, 12 groups in 6 drawings

One atom ([Q-M1f.380](../../../docs/decisions/language/Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)),
so their arrows meet. **In 5 of the 6 drawings the nodes are different things**: ER's three attributes `name` (of a
course, an institute, a student) become one atom; lion_share's 17 marriages one; datastruct's records, in 3 groups
with the same fields; the pressure valve's entry and exit points (both `""`); the family tree's two Johns. **In one
they are one thing**: the network map draws 5 routers twice, once on each route that reaches them (nodes `533` and
`538` are `195.66.224.76`), where one atom is right. The repair, `label@id`, keeps them apart: `name@name0`,
`John@john1`.

### 3. Parallel arrows of one label — lost, 2 arrows in 1 drawing

A knowledge base is a set, so `a -[calls]-> b` drawn twice is one fact. None of the 17 downloaded drawings has two
parallel arrows of one label; edge-cases has them because it was drawn to. The repair that keeps the count,
`(calls a b 1)` and `(calls a b 2)`, makes facts of three arguments, which the fold cannot draw: they come back as
two octagons (`edge-cases.a+numbered`).

### 4. A node's label that is an arrow's label — lost, 4 in 2 drawings

One atom, the relation, which Ein allows. None of the downloaded drawings has one; kernel-words and edge-cases do by
design (`n_calls [label="calls"]` beside `a -[calls]-> b`). In Ein's own drawings it is the point: the atom `owns` is
a node with the arrow `owns -[relation]-> owns` and the label of the arrows `Norwegian -[owns]-> Dog`. The repair,
`calls@n_calls`, keeps them apart (`edge-cases.a+rename`).

### 5. An undirected graph — ruled: two facts

5 undirected drawings, 542 arcs, 1,084 facts. Right where the relation is symmetric: the network map's links,
Kennedy's 4 marriages. Where the author chose `graph` for the layout, it makes a directed relation symmetric:
Kennedy's other 18 arcs are parent and child, written parent first (`I8 -- I1`); the mind map's arcs are topic and
subtopic; ER's are entity and attribute. `dir=none` in a digraph is the same picture with a direction written:
lion_share's 51 descent lines, `"001" -> "marr0001" [dir=none]`. The strict reading and (a), (b) read a digraph's
arrow as written; the variant `+dir` reads `dir=none` and `dir=both` as two facts and `dir=back` reversed, and doubles
lion_share to 102 facts.

### 6. Frames — lost, 10 frames in 6 drawings; an arc to a frame refused, 2 arcs in 1

A frame (`subgraph cluster_…`) has no fact under reading 1. The variant `+clusters` writes
`(in-cluster a0 process-1)` for each member and `(in-cluster clusterC clusterA)` for a frame in a frame: 8 facts for
the gallery's two processes, 3 for C4's enterprise boundary, 8 for the crate graph. fdp draws an arc to a frame,
`e -- clusterB`, whose end is no node: refused strictly, and written to the frame's atom by (a) and (b).

### 7. Ports and record fields — lost, 44 arrow ends in 3 drawings

`System_1:ss1 -> Subsystem_1` reads as an arrow from `System_1`, and which cell it leaves is lost; compass points
(`l:n -> m:s`) are layout. The variant `+ports` makes a cell an atom, `node0/f0`, with `(port node0 node0/f0)`, and
the arrow between cells: datastruct's 17 arrows become 42 facts, a linked structure Ein can follow.

### 8. Arrow attributes beyond the label — lost

| attribute | arrows | drawings | where it carries meaning |
|---|---|---|---|
| `color` | 563 | 6 | Kennedy: the relation (parent, spouse); pprof: the weight again |
| `penwidth`, `weight` | 94, 74 | 4, 5 | pprof: the sample count, already the label |
| `style` | 56 | 8 | UML: dashed is a dependency or a realisation |
| `dir=none`, `dir=back`, `dir=both` | 55, 12, 2 | 5, 3, 2 | UML: `dir=back` puts the hollow head at the parent; lion_share: no head on a directed line |
| `arrowhead`, `arrowtail` | 19, 10 | 3, 2 | UML: `empty` is inheritance, `diamond` composition, `odiamond` aggregation, and the only mark of it |
| `headlabel`, `taillabel` | 3, 3 | 2 | UML: the multiplicities `0..*`, `1` |
| `xlabel` | 4 | 3 | UML: `inheritance`, `composition`, the only label those arrows have; read as the label by the repair |
| `tooltip`, `labeltooltip`, `len`, `minlen`, `constraint` | 28, 28, 6, 1, 1 | | none |

### 9. Labels that are sentences, or more than one fact — refused as no name, 13 in 4 drawings; numbers, 30 in 2

C4's arrows are sentences (`Views account balances, and makes payments using`), some with a fact about the arrow
after them (`[XML/HTTPS]`); git's are commands (`git add ...`, `git reset` on one arrow: two facts). The slug makes a
relation of the sentence, `Views-account-balances-and-makes-payments-using`; `+source` keeps the sentence as the
fact's `:source` and loses the round trip (above); a nested fact keeps the technology and draws the arrow as an
octagon (above). pprof's labels are sample counts: 28 arrows, 21 relations, each of one to four facts (`(393 gzip
_start)`), a number where a relation should be. ER's `1`, `n`, `m` are cardinalities, names, and relations of
nothing but the cardinality.

### 10. Kernel words as labels

None of the 17 downloaded drawings writes one. What the binary does with each, probed one line at a time
(`(w a b)`, `(r w b)`, `(w)`):

| word | an arrow's label, `(w a b)` | a node's label, `(r w b)` | a node no arrow touches, `(w)` |
|---|---|---|---|
| `not`, `and`, `or`, `neq`, `rule`, `hrule`, `query`, `macro`, `import`, `open`, `_` | refused at the parse | refused at the parse | refused at the parse |
| `solve`, `config`, `trace` | refused at the parse | refused at the parse | **loads as an empty form: no fact, the node gone** |
| `program` | refused (a saved file's head) | a name | **loads as an empty saved program: no fact** |
| `module`, `commit` | refused (a saved file's head) | a name | refused |
| `relation` | **a declaration**, which wants a `:why` | a name | refused |
| `eq` | **loads, and root is dead**: `(eq a b)` is a check, false | a name | refused |
| `false` | refused (*false takes no argument*) | a name | **`(false)`: root is dead** |
| `absent`, `region`, `merged`, `owing` | refused | a name | refused |
| `step`, `true`, `model`, `is-a`, `__closed__`, `__symmetric__` | a fact | a name | a fact |

The kernel-words drawing (row 24) meets 25 of them; the repair, a trailing `_` (`not_`, `false_`, `__`), loads.

### 11. Reading 2, on Ein's own drawings

Reading 1 refuses Ein's drawings: three-houses 35 times, common-belief 26 (an octagon's label `(relation)`, an
arrow's `#1`, an arrow label that carries its source sentence or `by pet-follows-owner`). Reading 2, as drawn today
(an octagon `(rel)` a fact, its arrows `#k` its arguments, any other arrow a fact of two arguments whose label's first
word is the relation, a frame named `… ctx` holding facts not held), with a loop read as a fact of one argument:

- **three-houses**: 25 facts, loads, its drawing is the original's in every node and arrow but style and annotation
  (a fact's node is named by a hash of the fact, so the ids match), and its knowledge base is the original root's but
  for **`(transitive left-of)`, which `ein render kb` does not draw** (a rule's activator). Its four `:source`
  sentences come back from the arrow labels; a derived fact comes back given (only its comment says by which rule).
- **common-belief**: 21 facts, the knowledge base exactly: the nested `(likes Dan Tea)` under `believes`, the frame
  `not ctx`, the held `(not (likes Dan Fish))`, `(__closed__ person)`, the declarations.
- **turnstile-kb**, Ein's drawing of row 1's program: wrong, `(push Locked)` for `(push Locked Locked)`.

**A loop is two facts in today's drawing**: `(r x)` and `(r x x)` are both `x -[r]-> x`. Read as two arguments,
three-houses and common-belief are refused at load (*the fact (vacant House-2 House-2) gives vacant 2 arguments, and
(relation vacant House) declares 1*), and the turnstile is right. Every loop in Ein's two programs is a fact of one
argument (15), and every loop in the 25 drawings reading 1 reads is an arrow of two ends (5: the two state machines'
and edge-cases' `retries`), so neither reading is right for both, and only a declaration notices. **A stored negation no fact points at is not drawn at all**: a probe holding
`(not (u b a))` drew nothing for it. The canonical form (S1f.23.1) draws both as the graph has them.

## What the examples say about the README's questions

- **A label that is no name**: 17 of the 25 drawings have one, and 22 have one or an unlabelled arrow, 17 of 17
  downloaded among them.
  Refusing it would refuse nearly every drawing; the slug made every label a readable name, collided once (and then
  only because a label was cut to its first line), and the escape covers what folds to nothing. **Default the
  evidence suggests: rename by the slug, the escape where the slug is empty, and report each renamed label with its
  line on stderr; a module's full name is renamed too.**
- **Two nodes of one label**: 5 of 6 drawings meant two things. **Default: keep them apart as `label@id`, with a
  note**; the one drawing where one atom was right (the network map's routers) can say so by its labels.
- **A node with arrows out under reading 1**: an atom, and right in 23 of 25 drawings. In two it is a fact drawn as
  a node, the Levi pattern: lion_share's 17 marriages (two parents in, the children out) and the Petri net's 6
  transitions; and in Ein's drawings an octagon is one by definition. Ein's drawing tells itself apart: an octagon
  whose arrows are labelled `#1`, `#2`, …, which reading 1 refuses. **Default: an atom under reading 1, and reading
  2 where the drawing has an octagon with `#k` arrows.**
- **Undirected graphs**: ruled, two facts. The evidence adds that a `graph` is often a directed relation drawn without
  heads (Kennedy, the mind map, ER) and its arcs are written in the relation's order; reading `dir` in a digraph
  turned lion_share's directed lines into two facts each. **Default: a digraph's arrow as written, whatever its
  `dir`, with a note.**
- **Frames**: lost in 6 of 25, and kept as facts by `+clusters` without trouble; an arc to a frame (fdp) has no node.
  **Default: a frame is layout under both readings, as reading 2 says; `in-cluster` facts on request; an arc to a
  frame refused with its line.**
- **Unlabelled arrows**: **(a), one default label.**
- **Declarations**: none under reading 1; a declaration wants a `:why` the drawing has none of, and makes its
  relation guessable.

## What the conversion exposed about Ein

- **No name holds a character that is not ASCII**, so `Zoë`, `São Paulo` and every Cyrillic label lose their text
  or become `%D0%9C…`. Natural-language text has one place, a `:source`, and today's renderer writes it into the
  arrow's label, where it is no name.
- **A dotted name's meaning depends on the files beside the program** (above).
- **A node labelled `false` with no arrow kills the knowledge base; one labelled `solve`, `config`, `trace` or
  `program` disappears without a word.** `(eq a b)` written as a fact is a false check, and root is dead.
- **A rule cannot range over relations**: `(?r ?x ?y)` needs a parameter bound by an activator, so a KB of many
  relations (option b, pprof's counts) is one a rule cannot read as a whole.
- **Today's fold is not invertible**: `(r x)` and `(r x x)` draw alike; a held negation no fact points at, and a
  rule's activator, are not drawn; a derived fact's rule is only a word in its label. So a test that a drawing reads
  back to its knowledge base fails on today's drawing for these and holds on the three drawings here otherwise.
- **`dot -Tjson` is a laid-out drawing**: it needs the images an HTML label names (the network map's were not
  downloaded; the prototype strips its `<IMG>` cells), and it does not say which labels are HTML (gvpr does).
