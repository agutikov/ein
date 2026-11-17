# Architecture

The structural map of `ein.rs`: the crates and how they stack, the path of one `ein solve` from a file to its
read-out, every module of the shipped crates and what it owns, and where each part of the language lives in code.
The contracts behind the modules are the [design pages](03-design/README.md); the build, the features and the
dependency policy are [`03-design/10-toolchain-and-layout.md`](03-design/10-toolchain-and-layout.md).

## 1. The crates

### 1.1 The system in its context

`ein` is one binary, and everything it works on is a file. It starts no other program and opens no connection: the
standard library is read from the first of three places that holds it
([`03-design/02-shared-assets.md`](03-design/02-shared-assets.md#3-resolution)), and a drawing leaves `ein` as DOT
text, which the reader hands to Graphviz.

```plantuml
@startuml
left to right direction
skinparam componentStyle rectangle

actor "user" as user
rectangle "ein\n(one binary)" as ein

file "program.ein\nand the modules it imports\nfrom its own directory" as prog
database "program.einb\na private cache of one\nloaded program" as einb
package "the standard library: the first source that answers" as std {
  folder "1. $EIN_STDLIB" as s1
  folder "2. stdlib/ of a checkout,\nfound from the executable" as s2
  artifact "3. the copy compiled\ninto ein" as s3
}
file "the answer\nthe read-out on stdout; as files:\n~--dump-answer, ~--json-summary,\n~--events, ~--trace, ~--dump-lattice" as out
file "DOT text\nein render …" as dot
rectangle "Graphviz" as gv

user --> prog : writes
user --> ein : runs a command on a file
ein --> prog : reads
ein --> std : resolves std.*
ein --> einb : ein kb save writes it
einb --> ein : read wherever a .ein is
ein --> out : writes
ein --> dot : writes
dot --> gv : the user draws it
@enduml
```

### 1.2 The eight crates

An arrow runs from a crate to a crate it names under `[dependencies]` in its `Cargo.toml`; every edge of the
manifests is drawn. The six shipped crates stack linearly up to `ein-infer` and fork there; a crate depends on the
crates below it and on nothing above. `ein-cli` takes `ein-einb` under its `einb` feature.

```dot
digraph crates {
  rankdir=TB; labelloc=t;
  node [shape=box, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=9];

  subgraph cluster_shipped {
    label="the six crates that ship"; labeljust=l; fontname="Helvetica"; fontsize=10; color=gray50;
    core   [label="ein-core\ninterning, Value, the layered KB,\nprovenance"];
    ir     [label="ein-ir\nparse, imports, macros, load"];
    infer  [label="ein-infer\ncompile, match, saturate,\nH, the routes over R, cores"];
    einb   [label="ein-einb\nthe .einb container"];
    render [label="ein-render\nDOT views, the read-out,\nthe trace, the dumps"];
    cli    [label="ein-cli\nthe ein binary"];
  }

  ir -> core;
  infer -> ir; infer -> core;
  einb -> infer; einb -> ir; einb -> core;
  render -> infer; render -> ir; render -> core;
  cli -> render; cli -> infer; cli -> ir; cli -> core;
  cli -> einb [style=dashed, label="feature einb"];
}
```

Two crates are dev-only (`publish = false`), `ein-corpus` and `ein-narration`, and neither library depends on a
crate of the workspace. No shipped crate names one under `[dependencies]`; the edges below are every
`[dev-dependencies]` edge between two crates that is not a `[dependencies]` edge already, the ones a test or a
bench build adds. Two of them run against the stack, a lower crate's tests reading a higher crate: `ein-ir`'s tests
compile and saturate with `ein-infer`, and `ein-infer`'s tests render with `ein-render`.

```dot
digraph dev_edges {
  rankdir=TB; labelloc=t;
  node [shape=box, fontname="Helvetica", fontsize=11];
  edge [style=dashed, fontname="Helvetica", fontsize=9];

  subgraph cluster_dev {
    label="dev-only crates"; labeljust=l; fontname="Helvetica"; fontsize=10; style=dashed; color=gray50;
    corpus    [label="ein-corpus\nthe corpus manifest, fixture helpers,\nthe sweeps' worker pool, the bench set"];
    narration [label="ein-narration\nwhat of a derivation is narration"];
  }
  core [label="ein-core"]; ir [label="ein-ir"]; infer [label="ein-infer"];
  einb [label="ein-einb"]; render [label="ein-render"]; cli [label="ein-cli"];

  core -> corpus; ir -> corpus; infer -> corpus; einb -> corpus; render -> corpus; cli -> corpus;
  infer -> narration; render -> narration;
  ir -> infer     [label="tests only"];
  infer -> render [label="tests only"];
  corpus -> core; corpus -> ir; corpus -> infer;
}
```

The last three edges are `ein-corpus`'s own `[dev-dependencies]`, for its bench set
(`ein-corpus/benches/engine.rs`): its library depends on none of the three.

- **`ein-core`** depends on no other crate of the workspace: interning, `Value` and `FactId`, the layered
  copy-on-write KB, provenance and its walks, the value printers, and `:why` template substitution, which the
  obligation pass needs below the renderers ([`03-design/03-data-model.md`](03-design/03-data-model.md)).
- **`ein-ir`** parses, expands macros, resolves imports and loads: it builds the KB itself rather than handing an AST
  to a separate loader, which is why it sits above the store ([`03-design/04-ir-frontend.md`](03-design/04-ir-frontend.md)).
- **`ein-infer`** is the only writer of derived facts: compile, match, saturate, the boundary, the obligation pass,
  hypothesis generation, the routes over `R`, the cores
  ([`03-design/05-matcher.md`](03-design/05-matcher.md), [`03-design/06-saturation.md`](03-design/06-saturation.md),
  [`03-design/07-search.md`](03-design/07-search.md)).
- **`ein-einb`** writes and maps the `.einb` container ([`03-design/09-binary-format.md`](03-design/09-binary-format.md)).
- **`ein-render`** owns every rendering: the DOT views, the read-out's text, the markdown trace, the answer and
  lattice dumps, the JSON summary, the hypothesis space's document and graphs.
- **`ein-cli`** orchestrates: six subcommands, `ein {oracle,render,saturate,show,solve,test}`, plus `ein kb` under
  the `einb` feature. Every command reads a program by one path, `common::read_program`, `.ein` or `.einb` alike.
  **`ein render` draws and `ein show` lists**: `render` writes DOT, `show` writes Ein text a `.ein` file could hold.

`stdlib/` is Ein *data* that `ein-ir` resolves and embeds ([`03-design/02-shared-assets.md`](03-design/02-shared-assets.md)).

## 2. One `ein solve`, from file to read-out

| step | what happens | where |
|---|---|---|
| read | the file's bytes, `.ein` text or a `.einb` container by its magic | `ein-cli/src/common.rs` (`read_program`), `ein-ir/src/read.rs` |
| parse | text to the AST arena, by the parser generated from `ein.l` and `ein.y` | `ein_ir::grammar` |
| resolve, expand | imports read, each module once, names to full names; macros rewritten | `ein_ir::imports`, `ein_ir::macros` |
| load | the AST to the KB: registries, written facts, `(written T)`, the loader's refusals | `ein_ir::from_ir` |
| judge the mechanisms | where two hypothesis mechanisms could conflict, a second load is saturated and they are read together on root's fixpoint | `ein-cli/src/common.rs` (`load_held`), `ein_infer::mechanisms` |
| compile | each rule, per activator, to a plan, memoised for the run | `ein_infer::{compile, plan, engine}` |
| saturate root | the closure and the boundary, to root's fixpoint | `ein_infer::{saturator, match_, firing, contradiction}` |
| build H | the generation ladder at root | `ein_infer::{hypgen, hrule, oblgen}` |
| check | what root owes, the static monotonicity check, forbid | `ein_infer::{obligations, monotone, forbid}` |
| answer | the question's route over `R`, each node through the node function | `ein_infer::{explore, tree, tree_classes, states, downset, region}` |
| read | `R` described once; each listed state's model and each query's projection | `ein_infer::{described, read_out, verdict}` |
| print | the read-out, and whatever artefact was asked for | `ein_render::{read_out, trace, dump}`, `ein-cli/src/{solve, modes, summary}.rs` |

The same run as the calls one crate makes into the next. `prepare` is `ein_infer::region::prepare_with`, which every
question starts with; the loop is the route's, and its body is the node function of
[`03-design/07-search.md` § 2](03-design/07-search.md#2-the-node).

```plantuml
@startuml
actor user
participant "ein-cli\nmodes::run" as cli
participant "ein-ir" as ir
participant "ein-infer\nexplore, region" as infer
participant "ein-render" as render

user -> cli : ein solve FILE
cli -> ir : read::read_program(path)
ir --> cli : Held::Text | Held::Container
cli -> ir : parse(ast, text)
ir --> cli : the top-level forms
cli -> ir : load_query_with(ast, terms, forms, dir)
note right of ir
  resolve imports, expand macros,
  relations, rules, facts, written,
  the refusals, the indexes
end note
ir --> cli : Kb
opt two mechanisms could conflict
  cli -> infer : mechanisms::check(a second load)
  infer --> cli : conflicts (refused) | notes
end
cli -> infer : read_out::run_with(kb, terms, ast, opts, events)
group prepare
  infer -> infer : Saturator::new, saturate\n(root's fixpoint, the snapshot)
  infer -> infer : hypgen::generate_config_free\n(H, sorted and deduplicated)
  infer -> infer : obligations::tally(root)
  infer -> infer : monotone::classify, forbid::check
end
loop each batch of the route
  infer -> infer : region::fan_out(sets)\nfork root, write the commitment,\nresume, saturate, tally
end
infer -> infer : Described; each listed state's model\nand each query's classes
infer --> cli : ReadOut
cli -> render : read_out::render(&ReadOut)
render --> cli : Text
cli --> user : stdout, and each artefact asked for
@enduml
```

`ein oracle` runs the same load and root, then route 2, the whole cube. `ein test` loads a file, asks it the least
question that decides its claims, and checks them (`ein-cli/src/test.rs`, `ein_infer::claim`). `ein saturate` stops
after root.

## 3. The modules

### 3.1 `ein-core`

| module | owns |
|---|---|
| `intern.rs`, `terms.rs` | the symbol table and the two intern tables, lent to workers during a fan-out |
| `value.rs`, `facts.rs` | `Value`, four bytes; the fact store, interned rows and their ids |
| `kb.rs` | the KB: immutable layers and one writable top, the indexes, the extent counts, the fork |
| `bitset.rs` | dense sets of ids |
| `prov.rs`, `walks.rs` | derivation records and their arena; the premise closure, the derivation DAG, the unsat-core walk |
| `entities.rs`, `program.rs` | the typed views of relations, rules and macros; the program's registries |
| `config.rs`, `numparse.rs` | the typed view of `(config …)`, and the coercion of its values |
| `text.rs`, `repr.rs`, `printable.rs`, `numfmt.rs` | the one printer of Ein text, the value renderer, its escapes, number formats |
| `why.rs` | `:why` template substitution |
| `counters.rs` | the work counters, compiled in by `counters` |
| `shape.rs` | the KB's shape as one text, a test instrument |

### 3.2 `ein-ir`

| module | owns |
|---|---|
| `grammar.rs`, `grammar/build_ast.rs`, `ein.l`, `ein.y` | the generated lexer and parser, and the builder its actions call |
| `ast.rs` | the AST arena and its side table of positions |
| `macros.rs` | pattern macros, a load-time rewrite of the AST |
| `imports.rs` | import resolution, full names and short names |
| `stdlib.rs` | where `std.*` comes from: the override, the checkout, the embedded copy |
| `read.rs` | the one read of a program's text from a path |
| `from_ir.rs` | the loader |
| `dump.rs` | the canonical IR printer |
| `claim.rs`, `expect.rs` | the parsed forms of a `(solve … :expect …)` region claim and a query's `:expect` |

The modules as the path a program's bytes take through them; an arrow is what one hands the next
([`03-design/04-ir-frontend.md`](03-design/04-ir-frontend.md#2-the-load-path)).

```dot
digraph ein_ir {
  rankdir=LR;
  node [shape=box, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=9];

  file   [shape=note, label="program.ein\nor program.einb"];
  read   [label="read.rs\nread_program"];
  gram   [label="grammar.rs\nein.l, ein.y (generated)\ngrammar/build_ast.rs"];
  ast    [shape=cylinder, label="ast.rs\nthe Ast arena"];
  imp    [label="imports.rs\nResolver"];
  std    [label="stdlib.rs\nSource"];
  mac    [label="macros.rs"];
  load   [label="from_ir.rs\nthe loader"];
  claims [label="claim.rs, expect.rs"];
  dump   [label="dump.rs"];
  kb     [shape=cylinder, label="ein_core::Kb\nwith its Program"];
  text   [shape=note, label="Ein text"];

  file -> read  [label="bytes"];
  read -> gram  [label="Held::Text"];
  gram -> ast   [label="nodes"];
  ast  -> imp   [label="forms"];
  imp  -> read  [label="a module's path"];
  imp  -> std   [label="std.*"];
  imp  -> load  [label="one import-free list,\nfull names"];
  load -> mac   [label="a rule's clauses,\na goal"];
  mac  -> load  [label="expanded"];
  claims -> load [label=":expect, parsed"];
  load -> kb    [label="facts, registries,\n(written T)"];
  ast  -> dump  [label="nodes"];
  dump -> text;
}
```

### 3.3 `ein-infer`

| module | owns |
|---|---|
| `compile.rs`, `plan.rs`, `engine.rs`, `shape.rs` | the per-activator compiler, the plan, the engine's compile cache and its order, every plan as one text |
| `match_.rs`, `predicates.rs` | the register matcher; `eq` and `neq` |
| `saturator.rs`, `firing.rs`, `contradiction.rs` | the two-phase loop and the agenda; a rule application; the contradiction check |
| `obligations.rs` | the obligation pass over a quiescent KB |
| `naf_deps.rs`, `strata.rs` | the static map of what reads an `absent` over what; the rule dependency graph and its strata |
| `hypgen.rs`, `hrule.rs`, `oblgen.rs`, `mechanisms.rs`, `invariant.rs`, `scope.rs`, `relation_names.rs`, `types_as_objects.rs` | the generation ladder and its rungs; the mechanisms read together; the alive-set invariant; what the universe leaves out; a relation or a type bound as an object |
| `monotone.rs`, `detector.rs`, `forbid.rs` | the static check, the detector, the refusal of a refutation resting on an extendable `absent` |
| `region.rs` | the node function, the fan-out, the cube (route 2, the oracle) |
| `explore.rs` | every question about `R` and its route; the licences |
| `downset.rs`, `states.rs`, `tree.rs`, `tree_classes.rs`, `order.rs`, `known_dead.rs`, `transversal.rs`, `unheld.rs` | route 1, the state route, the obligation tree, the tree and the classes, the tree's order, the known-dead memory, transversals, licence L9 |
| `described.rs`, `count.rs`, `read_out.rs`, `verdict.rs`, `claim.rs`, `expect.rs` | `R` described once; exact counts; the read-out's data; a query projected over a state; claims checked against `R` |
| `canon.rs`, `explain.rs` | a state's identity; the minimal explanation search |
| `structure.rs`, `structure/` | the structure of H, read from the program's rules |
| `events.rs`, `error.rs`, `build.rs`, `mt19937.rs` | the `--events` writer; why a question was not answered; what the build compiled in; the seeded generator tests draw from (no command takes a seed) |

The modules by what each computes for the next: plans for the matcher, matches for the saturator, a fixpoint for
the node function, nodes for the routes, `R` for the read-out. The static check and the cores stand beside the
line: one reads the compiled plans, the other a dead node's derivations.

```dot
digraph ein_infer {
  rankdir=TB;
  node [shape=box, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=9];

  subgraph cluster_closure {
    label="the closure: one KB to its fixpoint"; labeljust=l; fontname="Helvetica"; fontsize=10; color=gray50;
    compile [label="compile.rs, plan.rs\na rule and an activator to a Plan;\nthe run's PlanMemo"];
    engine  [label="engine.rs\nthe ordered plan list, fired"];
    match   [label="match_.rs, predicates.rs\nthe register matcher; eq, neq"];
    sat     [label="saturator.rs\nthe agenda, the closure step,\nthe NAF boundary, the mirror"];
    firing  [label="firing.rs, contradiction.rs\none application; (false) and X, (not X)"];
    obl     [label="obligations.rs\nwhat a quiescent KB owes"];
  }

  subgraph cluster_worlds {
    label="the worlds: the lattice of commitments"; labeljust=l; fontname="Helvetica"; fontsize=10; color=gray50;
    hyp     [label="hypgen.rs\nhrule.rs, oblgen.rs\nthe ladder: H"];
    region  [label="region.rs\nprepare; the node function;\nfan_out; the cube"];
    explore [label="explore.rs\na question to its route"];
    routes  [label="downset.rs, states.rs, tree.rs,\ntree_classes.rs\nknown_dead.rs, order.rs, transversal.rs"];
    desc    [label="described.rs, count.rs\nR described once"];
    readout [label="read_out.rs, verdict.rs,\nclaim.rs, expect.rs"];
  }

  static  [label="monotone.rs, detector.rs, forbid.rs\nthe static check over the compiled plans"];
  explain [label="explain.rs\na dead node's core"];
  events  [label="events.rs\nthe --events writer, the Watch"];

  compile -> engine [label="PlanId"];
  engine  -> sat    [label="plans, in order"];
  sat     -> match  [label="run_one, run_seeded, holds"];
  match   -> sat    [label="each Match"];
  sat     -> firing [label="a popped candidate"];
  firing  -> sat    [label="the derived facts"];
  sat     -> obl    [label="the fixpoint"];

  sat     -> region  [label="Snapshot of root"];
  obl     -> region  [label="Owes"];
  hyp     -> region  [label="candidates"];
  region  -> static  [label="root's plans, H's relations"];
  static  -> explore [label="monotone | flagged"];
  explore -> routes  [label="Mode"];
  routes  -> region  [label="sets to saturate"];
  region  -> routes  [label="NodeOut"];
  region  -> explain [label="a dead fork"];
  routes  -> desc;
  desc    -> readout [label="Described"];
  sat     -> events  [style=dotted];
  region  -> events  [style=dotted];
}
```

### 3.4 `ein-einb`, `ein-render`, `ein-cli`

| crate | modules |
|---|---|
| `ein-einb` | `header.rs` (the header and the section table), `sections.rs`, `tables.rs` (ids across the boundary), `meta.rs` (what a file was made from), `solutions.rs`, `wire.rs`, `cast.rs` (the audited zero-copy casts) |
| `ein-render` | `read_out.rs` (the read-out as text), `trace/` (the trace: its AST, its steps, relevance, the region trace), `dump/` (the answer, the lattice, the oracle's cube, `--json-summary`), `rules.rs`, `constraints.rs`, `ir_dot.rs`, `kb_dot.rs`, `derivation.rs`, `slice.rs`, `region_dot.rs`, `region_doc.rs` (the views), `structure.rs` (the hypothesis space written out), `grid.rs`, `models.rs`, `sentence.rs` (a fact in English), `shape.rs` |
| `ein-cli` | `cmdline.rs` (the argument surface), `common.rs`, one module per command (`solve.rs` and `modes.rs`, `oracle.rs`, `test.rs`, `render.rs`, `show.rs`, `saturate.rs`, `kb.rs`), `summary.rs`, `factdump.rs`, `printers.rs`, `version.rs`, `help_shape.rs` |

`ein-render` by what each module is handed and what it writes. A view is DOT text and the read-out is text, both
returned to the caller, and a dump is files in a directory the caller names; no module of the crate writes to
stdout, which is `ein-cli`'s.

```dot
digraph ein_render {
  rankdir=LR;
  node [shape=box, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=9];

  forms   [shape=cylinder, label="the parsed forms\n(ein_ir::Ast)"];
  kb      [shape=cylinder, label="a Kb\nwith its Terms"];
  firings [shape=cylinder, label="firings\n(ein_infer::firing)"];
  ro      [shape=cylinder, label="ReadOut\n(ein_infer::read_out)"];
  cube    [shape=cylinder, label="Region, the oracle's cube\n(ein_infer::region)"];
  hs      [shape=cylinder, label="the structure of H\n(ein_infer::structure)"];
  visits  [shape=cylinder, label="the nodes a run visited\n(ein_infer::events::Watch)"];

  prog  [label="rules.rs, constraints.rs,\nir_dot.rs"];
  kbv   [label="kb_dot.rs, derivation.rs"];
  trace [label="trace/, slice.rs,\nsentence.rs"];
  text  [label="read_out.rs\ngrid.rs, models.rs"];
  dans  [label="dump/region_answer.rs,\ndump/json.rs, dump/serialise.rs"];
  dlat  [label="dump/region_lattice.rs"];
  dreg  [label="dump/region.rs,\nregion_doc.rs, region_dot.rs"];
  struc [label="structure.rs"];
  dot   [label="builder.rs, dot_util.rs,\npalette.rs"];

  o_dot  [shape=note, label="DOT"];
  o_md   [shape=note, label="the trace, markdown"];
  o_out  [shape=note, label="the read-out's text"];
  o_ans  [shape=note, label="--dump-answer DIR,\n--json-summary"];
  o_lat  [shape=note, label="nodes.json, nodes.md"];
  o_reg  [shape=note, label="an oracle dump:\nregion.json, region.md"];
  o_hs   [shape=note, label="h-space.json,\nits read-out"];

  forms -> prog;  kb -> kbv;  firings -> trace;  ro -> trace;
  ro -> text;  ro -> dans;  visits -> dlat;  cube -> dreg;  hs -> struc;
  prog -> dot;  kbv -> dot;  trace -> dot;  dlat -> dot;  dreg -> dot;  struc -> dot;
  dot -> o_dot;
  trace -> o_md;  text -> o_out;  dans -> o_ans;  dlat -> o_lat;  dreg -> o_reg;  struc -> o_hs;
}
```

## 4. Where the language lives

| concept | parsed | loaded | compiled | run |
|---|---|---|---|---|
| a fact | `ein.y` | `from_ir` writes it, with `source` provenance, and states `(written T)` for each fact nested in it | — | read by every plan through the indexes |
| a rule | `ein.y` | `from_ir` registers it (`entities`) | `compile::compile_rule`, once per activator, to a `Plan` | the matcher finds its matches; the saturator fires them |
| `(absent …)` | `ein.y` | — | lifted to a NAF guard (`split_naf`), or a nested step | judged at the boundary, on a quiescent KB |
| `(open ?R)` | `ein.y` | an obligation rule | compiled like any rule | the obligation pass, after the fixpoint |
| an hrule | `ein.y` | registered | a plan whose match proposes | `hrule`, the ladder's first rung |
| a macro | `ein.y` | expanded away before the load (`macros`) | — | — |
| an import | `ein.y` | resolved before the load (`imports`) | — | — |
| `(query …)`, `(solve …)` | `ein.y` | registered on the program | a goal compiles under `#query` | the routes; `verdict` projects a goal over a state |
| `(config …)` | `ein.y` | `SolverConfig` | — | read by the saturator and the routes; no field changes `R` |

## 5. The closure and the worlds

Two layers meet at one seam. **The closure** is monotone: the saturator fires the positive plans to quiescence,
and nothing it derives is ever retracted (`saturator`, `match_`, `firing`). **The worlds** are the nodes of the
search: each a fork of root with a commitment written in, saturated on its own, never written back to root (`region`'s
node function, and the routes above it). Negation as failure sits on the seam: an `(absent …)` is answered only at
the boundary, against a quiescent KB, one admission a round ([`03-design/06-saturation.md`](03-design/06-saturation.md)); a
commitment can change what a guard reads only by being a different node, which the static check reads off the plans
([`03-design/07-search.md`](03-design/07-search.md)). A state's identity, `canon::state_key`, is where the two layers' results
are compared.

## 6. One artefact, one owner

A semantic artefact that exists in two places is unified, compared by a test, or annotated at both sites with the
reason it must differ. Examples in the code: the model count and the verdict are read off one `Described`, never
recomputed per surface; the event schema and its emitters are held equal by `events_reference.rs`; the printer's
list of the grammar's reserved words is the grammar's, held by a test in `ein_ir::grammar`; and the language's
`ein_core::terms::RESERVED`, the names a declaration may not bind, is a different set from the grammar's
reserved words, so it has a different name, each documented with what the other is. The test of which case holds:
would an edit to one side be wrong not to make to the other? If so it is one artefact.
