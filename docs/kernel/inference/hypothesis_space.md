# The hypothesis space and its structure

A program's **hypothesis space** is its hypothesis set H, the candidates a
commitment is made of, together with the structure of that set. This page
defines the structure: what H looks like from inside: which candidates
rule each other out, which of that the rules alone decide, which candidates
answer one question, where exactly one of them must hold, and which questions
bear on each other. The engine reads all of it from the program's own lattice.
**Nobody declares it**: no object below is read off a relation's name, its
arguments or a declaration of it.

One module computes it, [`ein_infer::structure`](../../../ein.rs/crates/ein-infer/src/structure.rs),
and one command shows it, `ein render h-space FILE`
([§ How to see it](#how-to-see-it)). **`ein solve` computes none of it**, and
no route reads it: the structure is a set of statements about nodes, it moves
no answer, and it is not free ([§ What it costs](#what-it-costs)).

The answer it is the structure *of* is the region R,
[`solution_semantics.md`](solution_semantics.md). Every object here is a
statement about nodes of the lattice R is a region of: root's fixpoint with a
commitment written in and saturated, by the node function every route and the
oracle share. None is a second semantics.

---

## The objects

H is the universe: the candidates the generator proposes at root's fixpoint, in
rendered-text order ([`solution_semantics.md` § 1.4](solution_semantics.md)).
A candidate is named by its text, and its place in H is its place in that
order, so nothing below depends on the order ids were interned in.

### One candidate, and two: the pair layer

What the nodes of one candidate and of two decide at root.

| object | read from | definition |
|---|---|---|
| **at root** | root's fixpoint | root already holds the candidate: committing it commits nothing |
| **dead alone** | the node `{a}` | it is dead |
| **implies** | the node `{a}`, alive | `b` is in its closure: committing `a` brings `b` |
| **equivalent** | two alive single nodes | one closure |
| **excludes** | the node `{a, b}`, each alive alone, neither implying the other | it is dead |
| **interacts** | the same node, alive | its fixpoint holds a fact that neither `{a}`'s nor `{b}`'s does |

The excluded pairs are the members of N, the minimal dead sets, that hold two
candidates. `ein-infer/tests/pair_layer.rs` holds the two to each other on
every corpus program whose describe exhausts.

**A candidate's mark** is what root says of it alone: *at root*, *dead alone*,
*alive alone*, or *unread* where the layer stopped before its singles. A
candidate alive alone is **open**.

On a program the static check calls monotone, two licences decide a pair
without its node. *Dead is upward-closed* (L1): a pair with a member dead
alone is dead, and is **not** an exclusion, since its dead member says all
there is to say. *A commitment's state is its closure's* (L3): a pair one of
whose members implies the other is that member's node. Under a dead root every
candidate is dead alone and no pair is read.

### The theory's share, and the puzzle's

At root, a pair two declarations exclude and a pair two clues exclude are the
same thing, a dead node. They are told apart by reading the same universe a
second time in **the small KB**: the program with the puzzle's statements
taken out, every rule, declaration and import kept. A pair dead there is **of
the theory**. A pair root excludes that is alive there is **of the puzzle**.

The small KB's pair layer is over all of H. A candidate dead alone at root is
alive alone in a small KB that drops what killed it, and has its row and its
column there like any other. So the theory may exclude more pairs than root
lists as exclusions: the rest have a member dead alone at root.

**Which statements are the puzzle's** nothing declares, and the engine reads
it off the candidates (`structure::small_kb`). Of the facts a file writes, the
small KB drops:

- a fact of a relation a candidate is of, stated or denied: a given;
- **where the program's own hrules or obligations built H**: every other fact
  that names one of the names the candidates' relations hold, but for a fact
  of a relation that rung reads, which says what the universe is made of and
  is kept;
- **where the blind enumerator built H**: only a fact that names a candidates'
  relation together with such a name, or two such names.

A fact that names none of them is kept: a declaration that names a relation
and no object speaks of the relation whole. `(written T)` follows the facts it
is read in. The rule reads no relation by what it is called and no `:source`.
`ein-infer/tests/theory_share.rs` holds it by what must be true of any right
one: a declared bijection's rows and columns are exactly the theory's pairs,
two programs one clue apart have one theory, one puzzle in two encodings has
one theory, and the small KB derives no more than the program.

**The support of a death** is the second reading of the same question, and
asks no definition. The node of a pair root excludes is dead by a derivation,
and its support grounds that derivation out on the facts the program states. A
pair whose support lies inside the small KB is the theory's; otherwise the
support names the statements it is the puzzle's by. It is read only where
asked (`--supports`), and costs its node a second saturation and a search.

### The group, and the cover

**A group is a maximal set of two or more candidates that pairwise exclude by
the theory alone**: a maximal clique of the graph whose vertices are all the
candidates of H and whose edges are the pairs the small KB excludes. A state
holds **at most one** member of a group.

- **The cover.** Groups overlap: a candidate of a bijection is in a row and in
  a column. So the groups are a cover of the candidates they hold, and no
  partition.
- **In no group.** A candidate no pair of the theory holds is in no group, and
  no group says anything of it. A candidate the theory itself kills alone is
  among them.
- **An open member** of a group is a member alive alone at root. A candidate
  dead alone at root is a member like any other.
- **The graph is the theory's and not root's.** Once the clues have decided
  part of a puzzle, some candidate outside a row excludes every member of it,
  and the row is a clique of root's graph that is no longer maximal. The
  maximal cliques of root's own exclusion graph are kept beside the groups, as
  what the clues add, and are not called groups.

What *at most one* rests on: a group's pairs are dead in the small KB, and
dead at root wherever the small KB derives no more than the program. The pairs
of which that fails are listed (`cover.pairs_alive_at_root`), and the list is
empty where the small KB's rule fits the program.

### Exactly one: owed groups, and grids

A group says *at most one*. *Exactly one* needs a *must*, and the engine has
one source of it, an **obligation**: a state that owes an instance is not in R
until a fact witnesses it. L5's table reads, at root, the candidates that can
witness each instance root owes, where its premise holds of the instance: the
instance is then **licensed**, and those candidates are its **alternatives**.
Read against the cover:

| object | what it is | what a state of R holds |
|---|---|---|
| **owed group** | a group whose members are exactly the alternatives of a licensed instance | exactly one of its open members |
| **owed set that is no group** | a licensed instance whose alternatives do not pairwise exclude | at least one |
| **group nothing owes** | a group with no such instance | at most one, and nothing the engine reads says a member must hold |
| **unlicensed instance** | an instance L5 does not license: no alternatives, only the reason | — |

An unlicensed instance's witnesses are read all the same: which is a
candidate, which root denies, and for each the candidates whose own node holds
it, at root and in the small KB. Where the candidates that bring a witness by
the theory alone are exactly a group's members, and every witness root does not
deny is so brought, the instance is **about** that group. **The group is not
owed by it**: nothing is concluded from a witness a candidate brings.

**A grid** is found from the groups and their members alone: a connected
component of the theory's exclusion graph in which every candidate is in
exactly two groups, the groups are two families no two groups of one of which
share a member, and two groups share at most one member. A member is one cell,
between its row and its column. One relation's candidates may be several grids,
and a component that is no grid says why.

**A bijection** is a grid with as many groups in one family as in the other,
every one of them owed. **Nothing is concluded from a count**: where one family
is owed and the other is not, the grid says how many groups of each are owed
and stops there.

Nothing is synthesised. No rule, obligation or closure is added, and where the
*must* is missing the structure says what is missing, not what would fill it.

### Coupling: questions, edges, ties and debts

A group says which candidates answer one question. The coupling says which
questions bear on each other.

- **A question** is a group, *which of these, if any*, or a candidate in no
  group, *this one, or not*.
- **An edge** is a set of two or more questions that bear on each other: a
  hyperedge of the questions' hypergraph. It says which of two kinds it is, or
  both.
- **A tie** is of the first kind: some conflict or some derivation needs a
  member of each of the edge's questions.
- **A debt** is of the second: an instance root owes whose payers lie in two
  questions or more. A state of R holds one of its payers, so R is no product
  across them. One whose payers are all in one question is no edge: it is that
  question's own, which is what makes an owed group *exactly one*. **Whether L5
  licenses the instance does not decide it**, and every debt says which it is
  (`licensed`). A licensed instance's payers are its alternatives. An
  unlicensed one's are *the payers read*: the candidates whose own node holds
  one of its witnesses.

**Two things are read.** From root's pair layer, the pairs of two candidates
alive alone and in no question together that root excludes, of which one
implies the other, or that interact. And **the whole node**: root with every
candidate alive alone committed, one saturation, read with every rule
application that reached it.

**The two readings of the whole node.** A derived fact's label is the union of
the classes of questions its ways need, so on a program whose derivations go
round, a grid of bijections for one, the node's edges come to one edge of
every question the clues reach. So two sets of edges are kept, off the same
node: **the whole node's**, each derived premise followed back to the
candidates, and **the edges one application back**, the finer ones the first
absorbs. Neither replaces the other.

**The two sets of components** are of the candidates alive alone. Two are
together where they share a question or an edge holds a question of each.

- **Over every edge**: these are what **R is the product of its parts over**.
- **Over the conflicts' and the derivations' edges alone**, the debts' left
  out: what no debt joins. Where a debt's payers lie in two of them, R is no
  product over those.

`ein-infer/tests/coupling.rs` holds both to a describe wherever one exhausts:
every member of N lies inside one edge of a conflict or a derivation, and R is
the product over the first set.

---

## The domain

**The structure is of H as root builds it, on a program the static check calls
monotone.** There a pair dead at root is dead at every node above it, and a
closure read at root is a subset of every closure above it, so what the pair
layer says of root's nodes holds of the lattice.

**On a flagged program** neither licence holds. Every pair is saturated, each
statement is still true of the node it was read from, and **nothing above root
follows**. The structure says so in its own words, an object each
(`structure::FLAGGED`, `COVER_FLAGGED`, `COUPLING_FLAGGED`), no table of what
root owes is read, and so no group is owed.

**Which rung built H** is part of every structure (`rung`): `hrules`, the
program's own; `obligations`, what would discharge what root owes; `blind`, the
combinatorial enumerator; `declined`, an obligation the ladder declined, which
falls to the blind enumerator; `stuck` and `no-search`, where nothing is
proposed. The small KB's rule has one branch for the first two and another for
the blind enumerator.

**H does not grow, of debts**: on a monotone program a node that owes an
instance root did not is refused by the obligation tree (`tree.rs`,
`check_owed`). Of an hrule's proposals nothing enforces it: an hrule whose
`:match` reads a relation a commitment can change would propose more at a node
than at root. The structure is of H as root builds it either way.

**The programs the small KB's rule does not fit.** It reads the puzzle off the
candidates, and four kinds of program defeat that reading. On each the
structure is still a set of true statements; what it does not do is separate
the theory from the puzzle the way a reader would.

| class | what happens |
|---|---|
| **a clue written as a rule** | the small KB keeps every rule, so what such a rule says is the theory's. `theory.rules_naming_objects` lists the rules whose text names one of the candidates' objects |
| **a clue that names none of the candidates' objects** | kept, as a declaration is: a gate between two wires no candidate holds |
| **the blind enumerator's universe** | H holds facts of every open relation over every name, so a type's name and a relation's are candidates' objects, and the statements that type an object go with the clues. A cover there can be all of candidates nobody would propose: on `examples/branching/06_lookahead_on.ein` every member of every group is dead alone at root, so its groups are true and say nothing of the search (`ein-cli/tests/h_space_view.rs`) |
| **a table a module states over the candidates' objects** | dropped with the clues. Nothing in a loaded fact says which file wrote it |

And one more can show in the structure itself: a flagged program whose small
KB is monotone (`cover.small_monotone` true beside `monotone` false). The
small KB's layer is then licensed where root's is not, and it is still no more
than a set of statements about another program's nodes.

A smaller program derives no more, unless a rule reads the absence of a
statement it dropped. So what is dead in the small KB and alive at root
(`theory.against_root`), and what the small KB's fixpoint holds that root's
does not (`theory.small_beyond_root`), are both empty where the rule fits a
program and name the program where it does not.

---

## What it costs

In the read-out's units, saturations and facts beyond root:

| object | what it costs |
|---|---|
| root's pair layer | root's node, a saturation a candidate, and one for each pair the two licences leave |
| the small KB's pair layer | the same over the same candidates, in the small KB, facts counted beyond its own root |
| the cover, *exactly one*, the grids | no saturation: a clique enumeration, and two passes of root's matcher |
| the coupling | one saturation, the whole node. The edges one application back are read off the same node |
| a support, where asked | its pair's node saturated a second time, and a search |

`ein render h-space` prints what one program's cost was, in its `work` row
and under `cost` in the document. On the declared control the two layers and
the whole node are in [the read-out below](#the-read-out), and a describe of
the same program is `ein solve examples/zebra2.ein`, whose own `work` row
stands beside them: there the structure costs several times the answer. **No
route pays for a structure**, and this page claims the comparison of no other
program: each has its two rows.

**Over the corpus**, the sweep that draws every structure prints what reading
them cost. Over the 188 corpus programs that search: root's layers 49 017
saturations and 1 389 870 facts beyond root, the small KBs' 82 591 saturations
and 513 859 facts beyond their own roots, and 173 whole nodes, one a program
with a candidate alive alone.

```sh
EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-render \
    --test dot_wellformed -- graphviz_accepts_the_structure --nocapture
```

The test prints those figures and asserts none of them: they move with the
corpus. Program by program they are the census's,

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example pair_layer -- \
    --theory --routes > rows.jsonl
```

which writes one JSON line a program, with root's layer's `saturations`, the
small KB's under `theory.small.saturations`, and `searches`, whether the
program's describe saturates more than root.

**The budget.** Each pair layer is read within `--region-budget`, facts beyond
its own root summed over the fixpoints it computes, the routes' budget in the
routes' unit and by default the routes' default
(`ein_infer::downset::DEFAULT_BUDGET`). Root's layer and the small KB's are
each read at it. The whole node is charged to neither: it is not read where
its facts pass the budget.

**The bounds** are constants of the module and no lever. No `(config …)`
field, no option and no environment name reaches one, and each says what it
cut.

| constant | bounds |
|---|---|
| `CLIQUE_STEPS` | the calls of the clique enumeration for one graph |
| `NODE_APPLICATIONS` | the rule applications of the whole node's saturation |
| `ROUNDS` | the rounds a label is settled in |
| `WAYS` | the ways one application is read |
| `DESCRIPTIONS` | the descriptions of one way |

```sh
grep -n 'pub const \(CLIQUE_STEPS\|NODE_APPLICATIONS\|ROUNDS\|WAYS\|DESCRIPTIONS\)' \
    ein.rs/crates/ein-infer/src/structure.rs ein.rs/crates/ein-infer/src/structure/coupling.rs
```

---

## How to see it

```sh
ein render h-space FILE [--view VIEW] [--dump DIR] [--supports]
                        [--region-budget FACTS] [-j JOBS] [--no-require-why]
```

`FILE` is a `.ein` or a `.einb`, read as every command reads a program. A
program the loader refuses is refused, exit 1, with the loader's line. **Every
structure exits 0**, whole or not.

| option | what it does |
|---|---|
| `--view` | what stdout holds. `summary`, the default, the read-out; `exclusion`, `cover`, `coupling` or `one-step`, one graph as DOT; `json`, the document |
| `--dump DIR` | also writes `h-space.json` and the four graphs to `DIR`. stdout and stderr are the same with it. The directory is made before the work: a path that cannot be made is exit 1 |
| `--supports` | also reads the support of every pair root excludes. The groups, the grids and the components are the same with it |
| `--region-budget` | each pair layer's budget |
| `-j`, `--jobs` | threads for each step's nodes. The same structure, byte for byte |
| `--no-require-why` | loads a program that carries no `:why` ([`configuration.md` § 3.4](../configuration.md#34-the-option-that-decides-loading)) |

None of them changes an answer: [`configuration.md` § 3.3](../configuration.md#33-what-an-option-can-do-to-the-answer).
The view draws no SVG, as no `ein render` view does: a graph is DOT for
Graphviz to lay out.

### The read-out

<!-- generated: ein render h-space examples/zebra2.ein -->
```text
h-space      examples/zebra2.ein
H            56 candidates · 21 dead alone · 0 at root · built by the rung `hrules`
pairs        1 540 · 408 saturated · 327 excluded at root · 66 interacting · 234 implications · 9 equivalence classes
theory       128 pairs excluded by the theory alone · of root's 327, 45 the theory's and 282 the puzzle's · the small KB drops 18 of 66 stated facts and kills 0 candidates alone
supports     not asked: --supports names the statements each pair of the puzzle rests on
cover        36 groups · 0 candidates in no group · 5 components of the theory's graph
exactly one  36 owed groups · 0 groups nothing owes (0 an unlicensed instance is about) · 0 owed sets that are no group · 0 instances L5 does not license, of the 36 root owes
grids        5 grids · 5 bijections · 0 components that are no grid
coupling     36 questions · 535 ties · 393 edges (1 of three questions or more, 0 a debt's alone) · 495 edges one application back · 0 debts that are edges (0 unlicensed)
components   1 over every edge · 1 over the conflicts' and the derivations' alone · 0 debts across those
work         root's layer 465 saturations · 68 385 facts beyond root; the small KB's 1 597 saturations · 32 086 facts beyond its own root; the whole node 1 saturation · 182 facts beyond root; supports 0 saturations
graph        exclusion.dot · 56 nodes · 410 lines
graph        cover.dot · 92 nodes · 112 lines
graph        coupling.dot · 37 nodes · 419 lines
graph        coupling_one_step.dot · 125 nodes · 729 lines
whole        yes: nothing was stopped or cut at --region-budget 1048576
```
<!-- /generated -->

The block is the binary's output, and
`ein-cli/tests/h_space_view.rs` fails when it is not: edit nothing in it by
hand.

How to read two of its rows. **`theory`**: the theory excludes its pairs over
all of H, and root's own layer lists as exclusions only those of them whose
two members are each alive alone at root; the rest have a member dead alone
there. So the first number is not the third, and the exclusion graph draws the
first and the puzzle's together. **`exactly one`** and **`grids`**:
`zebra2.ein` states five relations bijective, and the structure finds five
grids, each a bijection, every group owed, with no name read to find them.

Three more rows appear where they have something to say: `domain`, a flagged
program's sentences; `says`, what is true of the program that makes its
structure small or empty, no candidate, a dead root, no candidate alive alone,
no table of what root owes; and `unread`, below.

### A structure that is not whole

**A structure a budget or a bound cut is never shown as a whole one.**
Everything stopped, cut or left unread is a sentence, and each sentence is
said three times: a row `unread` of the read-out under `whole  NO`, a line of
the document's `unread` beside `whole: false`, and a `note:` on stderr. Each
graph's title says how many there are. The lists hold what was read, and no
more.

What can be unread: a pair layer that stopped, with the singles and the pairs
it left; a small KB with no layer, and so no cover and no coupling; a clique
enumeration its bound cut; supports the search did not read, or cut; the whole
node, where it passed a bound; a way or a description past its bound; labels
that did not settle; and the two readings of the next paragraph.

**A layer read whole can leave something unread.** Which candidates' own nodes
hold each witness of an owed instance is read after a layer, through its
singles, and is refused once the layer has spent its budget. A layer's last
step may carry it past the budget, so the layer can end with every pair read
and that reading refused. An unlicensed instance then has no payers read: at
root it is no edge of the coupling, and in the small KB it is about no group.
The structure says `whole: false` there, with the sentence that names the
reading. `ein-cli/tests/h_space_view.rs` holds this on `examples/zebra.ein`,
one fact under what root's layer spends.

**Asking for supports can make a whole structure not whole.** A support's
search has caps of its own, and one it stops at is cut: a real frontier of the
death, of which a smaller one may exist, so it may name a statement the death
does not need. `ein render h-space examples/zebra2.ein --supports` reads all
327 supports and says 7 of them were cut, under `whole  NO`, where the same
program without the option is whole. What the option moves there is what it
adds: each excluded pair gains its support, and the ties and the edges gain the
statements those supports name. The candidates, the pair layer, the cover,
*exactly one*, the questions, both sets of components, and which questions each
edge holds are the same with it and without.

At the default budget, with no support asked, no corpus program that searches
has anything unread: `ein-render/tests/dot_wellformed.rs`, which reads the
structure of each, fails naming one that has.

### The graphs

Four undirected graphs, one a file under `--dump`.

| view | file | what it draws |
|---|---|---|
| `exclusion` | `exclusion.dot` | the pairs that exclude, in two strokes. A pair of the theory is a solid line, dotted where a member is dead alone at root. A pair of the puzzle is a dashed line, labelled under `--supports` with the statements its support names: `s7` is place 7 of `theory.stated`. A box is filled by its component of the theory's graph; a candidate dead alone at root is dashed and grey; one in no pair is not drawn |
| `cover` | `cover.dot` | a group a box, a candidate an oval joined to each group it is in, each component a cluster under what it is: a grid, a bijection, or why it is no grid. In a grid a member stands between its row and its column. An owed group has a double border and names the rule that owes it; a group an unlicensed instance is about is dashed |
| `coupling` | `coupling.dot` | the questions' hypergraph over the whole node's edges. A question is a box, an edge of two questions a line, an edge of more a point joined to each. A tie's edge is labelled with how many ties and ways it covers; a debt's alone is dashed and named for its debts |
| `one-step` | `coupling_one_step.dot` | the same over the edges read one application back |

**A question is drawn under its place**, `q3`, beside its name. The name is the
pattern of its members' texts, `(color-loc Green *)`, and is no identity: two
questions of one program can share one, as the kinds of attribute at one house
of `examples/zebra.ein` do.

**A large graph is written and not promised a layout.** The document says how
many nodes and lines each graph holds (`graphs`), so a reader knows before
asking Graphviz. `ein-render/tests/dot_wellformed.rs` puts every graph of
every corpus program that searches to `dot`, and names, graph by graph, the
ones past its bound of lines that it does not lay out.

**No picture is held to its bytes.** A layout is Graphviz's, and nothing here
pins one engine's or one version's. What is held is the DOT: its bytes in any
id space, and that `dot` draws it.

### `h-space.json`

Schema `ein-h-space/1`. One document, every object. A candidate is its text
wherever it is named; a group, a question, a component, a grid, an instance
and a debt is its place in its own list, which follows the universe's order.
**The document holds no clock and no job count**: it is a function of the
program and of what was asked.

**The header.**

| key | what it holds |
|---|---|
| `schema`, `program` | `ein-h-space/1`; the program as the command was given it |
| `asked` | `region_budget` and `supports` |
| `n`, `rung`, `monotone`, `root_dead` | how many candidates; which rung built H; the static verdict; whether root's own fixpoint derives `(false)` |
| `whole`, `unread` | whether nothing was stopped, cut or left unread; the sentences where something was |
| `says` | what is true of the program that makes its structure small or empty |
| `domain` | `pair_layer`, `cover`, `coupling`: each `null`, or on a flagged program the sentence that says what its statements are about |
| `cost` | `root_layer` (`saturations`, `facts_beyond_root`); `small_kb_layer` (`saturations`, `facts_beyond_its_own_root`); `supports`; `cover` (`clique_calls`, `root_clique_calls`, `bound`); `exactly_one`; `whole_node` (`saturations`, `facts_beyond_root`, `applications`, `steps`, `contradictions`, `rounds`, `merged`); and `saturations`, their sum |
| `graphs` | for each graph: `graph`, `file`, `nodes`, `lines` |

**`candidates`: H listed.** One object a candidate, in the universe's order.

| key | what it holds |
|---|---|
| `candidate` | its text |
| `at_root` | its mark |
| `killed_by_the_theory_alone` | dead alone in the small KB |
| `implies`, `implied_by`, `equivalent_to` | candidates, by text |
| `pairs_the_theory_excludes`, `pairs_the_puzzle_excludes` | how many pairs of each stroke it is in |
| `groups`, `in_an_owed_group` | the groups it is in, by place; whether one of them is owed |
| `questions` | the questions that hold it, by place |
| `component`, `conflict_component` | its place in each set of components; `null` for a candidate not alive alone |

**`pairs`: root's pair layer.** `theory.small_kb_layer` is the same object of
the small KB.

| key | what it holds |
|---|---|
| `n`, `candidates` | H |
| `monotone`, `domain`, `root_dead`, `licences` | the verdict; a flagged program's sentence; a dead root; the licences that decided a pair without its node |
| `complete`, `stopped`, `singles_left` | whether every single and every pair the licences leave was read; why it stopped (`why`, and `spent` and `projected` at a budget, `n` and `limit` at a limit); the singles it left |
| `pairs` | `all`; `dead_member` and `implied`, decided by a licence; `saturated`; `left` |
| `work` | `saturations`, `facts` |
| `at_root`, `dead_alone` | candidates |
| `implies` | pairs, the first implying the second |
| `equivalent` | the classes |
| `excluded`, `interacting` | pairs |
| `against_l1`, `against_l3` | on a flagged program, the pairs each licence would have read wrongly; `against_l3` each a `pair` with whether it is `dead` |

**`theory`: the theory's share.**

| key | what it holds |
|---|---|
| `small_kb` | `kept` and `dropped`, counts of the facts files write; `by_clause`, each `clause` of the rule with whether it is `kept` and its `facts`; `error`, where the small KB has no layer |
| `stated` | every fact the program holds before saturation: `fact`, `source`, `origin` (a fact, a declaration, the loader's `written`), `kept`, the `clause` that said so, `read_absent` |
| `theory_dead_alone`, `theory_excluded` | what the theory kills and excludes alone |
| `against_root`, `small_beyond_root` | `singles` and `pairs` dead in the small KB and alive at root; facts the small KB's fixpoint holds that root's does not |
| `root_excluded` | each pair root excludes: `pair`, `of_the_theory`, `support` |
| `rules_naming_objects` | the rules whose text names one of the candidates' objects |
| `small_kb_layer` | the small KB's pair layer |

A `support` is `null` where none was asked. Otherwise: `read`; `status`, what
the search established; `dropped` and `kept`, the stated facts it names by
what the small KB does with each; `commitments`, the members of the pair it
rests on; `other`; `rules`, those whose firing ends the derivation; and
`check`, which this command never asks and is always `null` here.

**`cover`.**

| key | what it holds |
|---|---|
| `n`, `complete`, `monotone`, `small_monotone`, `domain` | as the layer's; `small_monotone` is the small KB's verdict |
| `stopped` | the small KB's layer's stop |
| `bound`, `steps`, `cut` | the enumeration's bound, its calls, and what the bound cut: `pairs` inside no group listed, `candidates` of such a pair in none |
| `candidates` | each `candidate` with `at_root`, `killed_by_the_theory_alone`, `groups` |
| `groups` | each its `members` and its `open` members |
| `in_no_group`, `components` | candidates; the components of the theory's exclusion graph |
| `pairs_alive_at_root` | the theory's pairs whose node is alive at root; `null` where root's pairs were not all read |
| `root_cliques` | the maximal cliques of root's own graph: `cliques`, `steps`, `cut` |

**`exactly_one`.**

| key | what it holds |
|---|---|
| `n`, `domain` | as the cover's |
| `root_owes`, `no_table` | how many instances root owes; why no table is read, where none is |
| `singles_read`, `theory_read` | whether the witnesses' holders were read, at root and in the small KB |
| `instances` | L5's table, below |
| `groups` | each `group` with its `members`, its `open` members, `owed_by`, `an_unlicensed_instance_is_about_it`, and its `grid` (`grid`, `family`) |
| `owed_groups`, `groups_nothing_owes` | groups, by place |
| `owed_sets_not_a_group`, `unlicensed` | instances, by place |
| `grids` | each `grid` with its `component`, `members`, `open`, `cells`, `relations`, two `families` (`groups`, `owed`, `an_unlicensed_instance_is_about`), `kind`, `bijection` |
| `components_that_are_no_grid` | each `component` with `members`, `groups`, `owed`, `relations`, `why`, and `detail` (`most`, or `members_in_one_group`) |

An instance: `instance`, `rule`, `standing`, `licensed`, `witnesses_denied_at_root`
and `witnesses`, each a `fact` with what it `is`, `held_at_root_by` and
`held_in_the_theory_by`, or `unknown` with why none is known. A licensed one
adds `alternatives`, and by its standing `group`, `inside`, or
`pairs_the_theory_does_not_exclude` with `holds_groups`. An unlicensed one adds
`why`, `pay_by_the_theory_alone`, `about_group`, and where its witnesses'
holders were read `witnesses_no_candidate_and_not_denied` and
`of_them_brought_by_a_candidate_at_root`.

A grid's `relations` are read off its members' texts, for the reader. No grid
is found by them.

**`coupling`.**

| key | what it holds |
|---|---|
| `reading`, `complete`, `domain` | the reading taken, the whole node's; whether it was read whole; a flagged program's sentence |
| `pairs_read`, `node_stopped`, `cut` | whether root's pairs were all read; why the whole node was not, where it was not; what the bounds cut (`ways`, `descriptions`, `unsettled`) |
| `questions` | each what it `is`, its `name`, how many members are `open`, and its `members` with each one's mark |
| `ties` | each `pair` with its `kind` and, under `--supports`, the statements it is `stated` by |
| `edges`, `edges_one_step_back` | the two readings' edges, below |
| `components`, `conflict_components` | the two sets |
| `debts` | each `rule`, `instance`, `licensed`, `payers`, `questions` and `question_ids` |
| `debts_across_conflict_components` | debts, by place |
| `unlicensed_instances`, `committed` | how many instances L5 does not license; the candidates committed in the whole node |

An edge: `questions`, by name, and `question_ids`, by place, which is what
tells two questions of one name apart; `kind`, *a conflict or a derivation*,
*a debt*, or both; `pairs` (`excludes`, `implies`, `interacts`) and `node`
(`conflicts`, `derivations`, `implications`), how much of each it covers;
`debts`, the instances it is the edge of; and `stated`, the puzzle's
statements it rests on with how many of what it covers name each.

**`declared`: what the program declares, beside.** Nothing of the structure is
read from it.

| key | what it holds |
|---|---|
| `mechanisms` | the hypothesis mechanisms at root's fixpoint: `hrules`, `active_hrule_plans`, `obligation_rules`, `obligation_plans`, counts; `proposed`, `owable`, `closed`, `listed`, `excluded`, relations; `no_search`. Or `error` |
| `relations` | for each relation a candidate is of, in the universe's order: `relation`, `candidates`, `stated_of_it`, `an_active_hrule_proposes_it`, `an_obligation_can_owe_it`, `closed`, `listed`, `excluded`, `candidates_in_no_group`, `grids`, `bijections` |

`stated_of_it` is the facts the program states, the small KB keeps, and that
name the relation: a statement of the relation whole. It is found by the
relation's own name among a fact's arguments and by no other name, so the
engine does not know which of them means *bijective*. A reader does: on
`examples/zebra2.ein` each of the five relations has `(bijective R)` there,
beside its one grid, a bijection.

---

## What each reading does not tell

- **The pair layer does not see a conflict of three.**
  `examples/lattice/02_genuine_3set_death.ein` needs all three of its
  candidates for its `(false)`: no pair excludes, no group forms, and its one
  edge is the coupling's.
- **The theory's share is one rule's reading**, with the classes of
  [§ The domain](#the-domain) that it does not fit.
- **A support is an explanation**, as a core is: sufficient, not least, and it
  may vary with rule order. It names what one derivation rests on.
- **A group nothing owes says *at most one* and no more.** A count that would
  suggest *exactly one* concludes nothing.
- **An unlicensed debt's edge may be too small.** Its payers are only those a
  single candidate's node shows: a witness that needs two candidates together
  is not seen. Nothing follows from such an edge beyond *do not take these
  questions for independent*.
- **The coupling may tie more than any state does.** A way through a derived
  fact does not know which members of a question the fact was derived from.
  And it is coarse where a fact has ways of two kinds, which is what the edges
  one application back are kept for.
- **A tie is of root as the clues leave it.** Where root has applied them, the
  pairs it excludes are their joint consequences, and a tie is no one
  statement's.
- **The owner's coupling does not itself say when an unlicensed instance's
  payers were not read.** The view says it, from what was read of the
  witnesses ([§ A structure that is not whole](#a-structure-that-is-not-whole)).
- **On a flagged program nothing above the nodes read follows.**

---

## What holds this page

| claim | held by |
|---|---|
| each object, on its controls and over the corpus | `ein-infer/tests/pair_layer.rs`, `theory_share.rs`, `cover.rs`, `exactly_one.rs`, `coupling.rs` |
| the structure, the document and the graphs are the same in any id space and under every import form | `ein-infer/tests/structure_invariance.rs` |
| the document is the owner's reading; the door, the dump, a stopped structure, a refused reading, a flagged program, a program with no candidate; the read-out above | `ein-cli/tests/h_space_view.rs` |
| Graphviz draws every graph, and every corpus structure is whole at the default budget | `ein-render/tests/dot_wellformed.rs` |
| the census and the view spell one object one way | both call `ein_render::structure`'s lists |
