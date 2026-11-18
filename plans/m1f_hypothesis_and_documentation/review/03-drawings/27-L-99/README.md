# L-99 — three drawings of a derivation keep a fact's text in a node

## 1. The question

The rule *every object is drawn once* (Q-M1f.395) ends with *"No node is labelled with a fact's text"*: a fact
under an arrow is drawn as its own octagon with numbered arrows to its arguments. Three drawings of a
**derivation** do not do that: each node is a fact of the derivation, drawn once, labelled with the fact's text,
with no arrows to its arguments. They are (1) the **slice** of a region trace (`ein solve --trace`, an inline
`dot` block per step), (2) the **derivation graph** (`derivation_dag_to_dot`, the library's), and (3) the graph of a
**written `(trace …)`** in its `dag` view. Q-M1f.395 itself excepts (1) and (2) as *not drawings of facts with
their arrows*. Should the rule be read of these too, so that their facts are drawn as objects?

Recorded: [`../../findings.md`](../../../findings.md#the-engines-renderers), row **L-99** (P1f.22's N-28); its default
under [§ Defaults nested facts took](../../../findings.md#defaults-nested-facts-took).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-b5fb8d70`. Two programs:

- [`programs/believe.ein`](programs/believe.ein), written for this review: a given fact that points at another, and
  a rule that unpacks it, so the derivation has a fact under an arrow.

  ```lisp
  (relation owns Person Pet :why "{?1} owns {?2}")
  (claims Carol (owns Bob Dog) :source "c")
  (rule believe () :match (claims ?p (owns ?x ?y)) :assert (owns ?x ?y) :why "{?p} says {?x} owns {?y}, and is believed")
  (solve :no-search)
  ```
- [`programs/trace-steps.ein`](programs/trace-steps.ein), a copy of `examples/syntax/trace-steps.ein`, the corpus's
  written `(trace …)` with steps that chain.

### (1) The slice of a trace — `ein solve FILE --trace OUT.md`

[`runs/believe.trace.md`](runs/believe.trace.md) (read-out: [`runs/believe.solve.read-out.txt`](runs/believe.solve.read-out.txt)).
Its one step's `dot` block, cut out: [`renders/believe.trace.block1-step1.dot`](renders/believe.trace.block1-step1.dot) /
[`.svg`](renders/believe.trace.block1-step1.svg).

<!-- output -->
```text
  "f_e7f0a8c099" [label="claims(Carol, owns(Bob, Dog))", shape=box, style=rounded];
  "f_5b3adbf9bf" [label="owns(Bob, Dog)", shape=box, style=rounded, penwidth=2];
  "fire0_believe" [shape=box, style="rounded,bold", color="#9467bd", fontcolor="#9467bd", label="believe\nCarol says Bob owns Dog, and is believed"];
```

![the slice: a box labelled claims(Carol, owns(Bob, Dog)) into the rule believe, into a bold box owns(Bob, Dog)](renders/believe.trace.block1-step1.svg)

Two nodes carry a fact's text. And the one object `(owns Bob Dog)` appears twice: as its own node (the derived
fact, bold) and inside the premise's label.

With `-F` the trace also draws each traced state's whole KB, as `ein render kb` does
([`runs/believe.trace-full-kb.md`](runs/believe.trace-full-kb.md); block 2:
[`renders/believe.trace-full-kb.block2-state.svg`](renders/believe.trace-full-kb.block2-state.svg), `fdp`): there
`(owns Bob Dog)` is one octagon with arrows `#1`, `#2`, entered by `claims c` — the object drawing, beside the text
drawing of the same step. (Its label `(owns)\nby believe` is L-96.) The crate's golden of a slice,
`ein-render/tests/golden/dot/slice_render_slice.dot`, copied:
[`renders/trace-slice.crate-golden.slice_render_slice.svg`](renders/trace-slice.crate-golden.slice_render_slice.svg).

### (2) The derivation graph — no command

`derivation_dag_to_dot` (`ein-render/src/derivation.rs`) is called by nothing in `ein-cli`: only by its test,
`ein-render/tests/derivation_dot.rs`, against its golden (read, not run: a `grep` of the crates). So no `ein`
command draws it. The golden is copied:
[`renders/derivation-graph.crate-golden.kb_provenance_dag.dot`](renders/derivation-graph.crate-golden.kb_provenance_dag.dot) /
[`.svg`](renders/derivation-graph.crate-golden.kb_provenance_dag.svg):

<!-- output -->
```text
  f_0b8a036bc4 [shape=box, label="(q a c)\n[rule: triangle]"];
  f_69510aab53 [shape=ellipse, label="(p a b)\n[(1)]"];
  f_b34b2c73fa [shape=ellipse, label="(p b c)\n[(2)]"];
```

### (3) The graph of a written `(trace …)`

The library has three views of a `(trace …)` form (`ir_dot::TraceView`: `per-step`, `aggregate`, `dag`).
`ein render program` draws `per-step` only; it writes **no** fact's text — a node per step and premise:
[`runs/trace-steps.render-program.dot`](runs/trace-steps.render-program.dot),
[`renders/trace-steps.render-program.trace-per-step.svg`](renders/trace-steps.render-program.trace-per-step.svg).

The `dag` view, the one with text nodes, is reached by no command either; the shape golden digests it
(`dot[ir-trace-dag]`). Its DOT for `trace-steps.ein`,
[`renders/trace-steps.trace-dag.library-view.dot`](renders/trace-steps.trace-dag.library-view.dot) /
[`.svg`](renders/trace-steps.trace-dag.library-view.svg), was **written by hand from the code**
(`render_trace_dag`, `builder.rs`) and is **byte-identical to the library's**: its md5 is the golden's line
`711e556435f48363  12L  examples/syntax/trace-steps.ein::dot[ir-trace-dag]`
([`runs/trace-steps.digest-check.txt`](runs/trace-steps.digest-check.txt)).

<!-- output -->
```text
  "(lives-in N H1)" [shape=box, style=bold, label="(lives-in N H1)"];
  "(color-loc Blue H2)" [shape=box, style=bold, label="(color-loc Blue H2)"];
  "(final X)" [shape=box, style=bold, label="(final X)"];
```

The crate's two goldens of a written trace, copied:
[`renders/written-trace.crate-golden.ir_render_trace_a.svg`](renders/written-trace.crate-golden.ir_render_trace_a.svg) (per-step),
[`renders/written-trace.crate-golden.ir_render_trace_dag.svg`](renders/written-trace.crate-golden.ir_render_trace_dag.svg) (dag).

## 3. The options

### None (the default): the views of a derivation are as they are

A node is a fact of the derivation, labelled with its text; Q-M1f.395's last paragraph already says so of (1) and
(2).

### Read *no node is labelled with a fact's text* of these too

Each fact node becomes the fact as its object: an octagon with its relation's name and a numbered arrow to each
argument; a fact that is an argument of another is the same octagon. **Mocks**, each the DOT above with the fact
nodes so replaced and the derivation's arrows kept (argument arrows grey):

- the slice: [`renders/believe.trace.block1-step1.objects.MOCK.dot`](renders/believe.trace.block1-step1.objects.MOCK.dot) /
  [`.svg`](renders/believe.trace.block1-step1.objects.MOCK.svg). `(owns Bob Dog)` is then **one** octagon,
  entered by the rule's bold arrow and by the premise's arrow 2: what the step derived is visibly what the claim
  was about.

  ![mock: claims an octagon with arrow 1 to Carol and arrow 2 to the octagon owns, which the rule believe also enters; owns has arrows to Bob and Dog](renders/believe.trace.block1-step1.objects.MOCK.svg)

- the written trace's `dag` view:
  [`renders/trace-steps.trace-dag.objects.MOCK.dot`](renders/trace-steps.trace-dag.objects.MOCK.dot) /
  [`.svg`](renders/trace-steps.trace-dag.objects.MOCK.svg).
- the derivation graph:
  [`renders/derivation-graph.crate-golden.objects.MOCK.dot`](renders/derivation-graph.crate-golden.objects.MOCK.dot) /
  [`.svg`](renders/derivation-graph.crate-golden.objects.MOCK.svg) (what the label's second line said, the rule or
  the source, kept as an `xlabel`).

Side by side: [`renders/slice.today-above-mock-below.png`](renders/slice.today-above-mock-below.png);
[`renders/written-trace-and-derivation-graph.today-and-mocks.png`](renders/written-trace-and-derivation-graph.today-and-mocks.png)
(per-step, dag today, dag mock, derivation graph today, its mock, left to right).

What it costs: a slice gets an atom node per argument, so a step over many facts grows (the slice is drawn *"only
that cone … never the whole KB"*, 12-rendering.md); and a derivation's facts are read from the text now, at a
glance, where the mock makes them be read off arrows. What it gains: one object one node, the drawing's rule
everywhere, and a derived fact that is the argument of a premise shown as one node.

## 4. The default

**None** — *"The views of a derivation (the slice, the derivation graph, a written trace's graph) are as they were
(L-99)"*, [§ Defaults nested facts took](../../../findings.md#defaults-nested-facts-took); the row: *"by default none:
unless no node is labelled with a fact's text is read of these too"*. Q-M1f.395 scopes the rule to the four writers
that draw facts with their arrows, and names the slice and the derivation graph as outside it.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`programs/believe.ein`](programs/believe.ein) | a small program with a derivation, written for this review |
| [`programs/trace-steps.ein`](programs/trace-steps.ein) | copy of `examples/syntax/trace-steps.ein` |
| [`runs/believe.trace.md`](runs/believe.trace.md) | `ein solve --trace`, the region trace with its slice |
| [`runs/believe.trace-full-kb.md`](runs/believe.trace-full-kb.md) | the same with `-F`, each state's KB appended |
| [`runs/believe.solve.read-out.txt`](runs/believe.solve.read-out.txt) | the run's read-out |
| [`runs/trace-steps.render-program.dot`](runs/trace-steps.render-program.dot) | `ein render program` of the written trace (per-step view) |
| [`runs/trace-steps.digest-check.txt`](runs/trace-steps.digest-check.txt) | the dag view's DOT against the shape golden's digest |
| [`renders/believe.trace.block1-step1.dot`](renders/believe.trace.block1-step1.dot) / [`.svg`](renders/believe.trace.block1-step1.svg) | the slice, cut from the trace |
| [`renders/believe.trace.block1-step1.objects.MOCK.dot`](renders/believe.trace.block1-step1.objects.MOCK.dot) / [`.svg`](renders/believe.trace.block1-step1.objects.MOCK.svg) | mock: the slice with facts as objects |
| [`renders/believe.trace-full-kb.block2-state.dot`](renders/believe.trace-full-kb.block2-state.dot) / [`.svg`](renders/believe.trace-full-kb.block2-state.svg) | the `-F` KB snapshot, for contrast |
| [`renders/trace-steps.render-program.trace-per-step.dot`](renders/trace-steps.render-program.trace-per-step.dot) / [`.svg`](renders/trace-steps.render-program.trace-per-step.svg) | the per-step view, drawn |
| [`renders/trace-steps.trace-dag.library-view.dot`](renders/trace-steps.trace-dag.library-view.dot) / [`.svg`](renders/trace-steps.trace-dag.library-view.svg) | the dag view, by hand from the code, checked by digest |
| [`renders/trace-steps.trace-dag.objects.MOCK.dot`](renders/trace-steps.trace-dag.objects.MOCK.dot) / [`.svg`](renders/trace-steps.trace-dag.objects.MOCK.svg) | mock: the dag view with facts as objects |
| [`renders/derivation-graph.crate-golden.kb_provenance_dag.dot`](renders/derivation-graph.crate-golden.kb_provenance_dag.dot) / [`.svg`](renders/derivation-graph.crate-golden.kb_provenance_dag.svg) | the derivation graph's golden, copied |
| [`renders/derivation-graph.crate-golden.objects.MOCK.dot`](renders/derivation-graph.crate-golden.objects.MOCK.dot) / [`.svg`](renders/derivation-graph.crate-golden.objects.MOCK.svg) | mock: it with facts as objects |
| [`renders/trace-slice.crate-golden.slice_render_slice.dot`](renders/trace-slice.crate-golden.slice_render_slice.dot) / [`.svg`](renders/trace-slice.crate-golden.slice_render_slice.svg) | the slice's golden, copied |
| [`renders/written-trace.crate-golden.ir_render_trace_a.dot`](renders/written-trace.crate-golden.ir_render_trace_a.dot) / [`.svg`](renders/written-trace.crate-golden.ir_render_trace_a.svg) | the per-step view's golden, copied |
| [`renders/written-trace.crate-golden.ir_render_trace_dag.dot`](renders/written-trace.crate-golden.ir_render_trace_dag.dot) / [`.svg`](renders/written-trace.crate-golden.ir_render_trace_dag.svg) | the dag view's golden, copied |
| [`renders/slice.today-above-mock-below.png`](renders/slice.today-above-mock-below.png) | the slice and its mock |
| [`renders/written-trace-and-derivation-graph.today-and-mocks.png`](renders/written-trace-and-derivation-graph.today-and-mocks.png) | the written trace's two views, the derivation graph, and their mocks |

## 6. Re-run

From the repo root, `E=build/review-bin/ein-head-b5fb8d70`, `D=plans/m1f_hypothesis_and_documentation/review/03-drawings/27-L-99`:

```sh
$E solve $D/programs/believe.ein --trace $D/runs/believe.trace.md > $D/runs/believe.solve.read-out.txt
$E solve $D/programs/believe.ein --trace $D/runs/believe.trace-full-kb.md -F > /dev/null
$E render program $D/programs/trace-steps.ein > $D/runs/trace-steps.render-program.dot
# cut each ```dot block of a trace to its own file
python3 -c "import re,sys; [open(f'{sys.argv[2]}.block{i}-{re.match(r\"digraph (\S+)\",b).group(1)}.dot','w').write(b) for i,b in enumerate(re.findall(r'\`\`\`dot\n(.*?)\`\`\`',open(sys.argv[1]).read(),re.S),1)]" $D/runs/believe.trace.md $D/renders/believe.trace
for f in ein-render/tests/golden/dot/{kb_provenance_dag,ir_render_trace_a,ir_render_trace_dag,slice_render_slice}.dot; do ls ein.rs/crates/$f; done   # the goldens copied
for f in $D/renders/*.dot; do case $f in *state.dot) fdp -Tsvg $f > ${f%.dot}.svg;; *) dot -Tsvg $f > ${f%.dot}.svg;; esac; done
grep 'trace-steps.ein::dot\[ir-trace-dag' ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5
head -c -1 $D/renders/trace-steps.trace-dag.library-view.dot | md5sum
```

## 7. Found while building

- **Two of the three views have no `ein` command.** The derivation graph (`derivation_dag_to_dot`) is called only by
  its crate test, and the `dag` and `aggregate` views of a written trace only by the library and its tests (the
  shape golden digests `dag`); `ein render program` draws `per-step`, which has no fact text. Q-M1e.20 says *"No
  renderer the engine builds is reachable from Rust alone."* Read, not run (a `grep` of `ein-cli/src`). So of the
  row's three, only the slice is something a user of `ein` sees.
- **Q-M1f.395 says the slice and the derivation graph draw each fact "as one node that carries its sentence".**
  The nodes carry the fact's **text** — `claims(Carol, owns(Bob, Dog))`, `(q a c)` — not its sentence; the sentence
  (*Bob owns Dog*) is in the trace's markdown, and the rule node carries the rule's `:why`. The decision does not
  name the third view, the written trace's `dag`.
- In the slice, the object `(owns Bob Dog)` is both a node and part of another node's text.
- The per-step view of `trace-steps.ein` labels `s3` and `s4` `step: s3`, `step: s4`, but `s1` and `s2` plain
  rectangles `s1`, `s2`: a step that is a later step's premise is re-declared as a premise, which overrides its
  label (`Builder::node`). Out of this row's scope, noted only.
- The ledger says `trace-steps.ein` is *"the one corpus program whose view of the last kind has such a node"*; that
  view is `dot[ir-trace-dag]`, which no command prints.

## 8. Your call

- **None** (default): the three views keep a fact's text in a node; the rule is the four fact-drawing writers'.
- **Read the rule of these too**: each fact of a derivation drawn as its object (the mocks); worth it mostly for
  the slice, the one a user sees.
