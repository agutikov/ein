# Ein — the Rust embedding API

How to drive the engine **as a library from another Rust program**: load a
`.ein` puzzle, solve it, and read the verdict with its explanation.

> **Audience: embedders.** This page is the *programmatic* contract — the
> crates you add to a `Cargo.toml` and the functions you call. If you want to
> **author puzzles** in the S-expression language, read
> [`docs/kernel/`](../kernel/) (the grammar, the kernel API, the stdlib). If
> you want the **engine internals**, read
> [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md).
> If you want to **run** ein rather than embed it,
> [`docs/install.md`](../install.md) and `ein --help`.

**This page's example is a test.** The code below is the region of
[`ein.rs/crates/ein-cli/tests/embedding.rs`](../../ein.rs/crates/ein-cli/tests/embedding.rs)
between its two `page` markers, and one of that file's three tests compares
the two texts. So the example compiles, runs, and produces the output printed
here — checked by `cargo test --workspace`, which is the gate. It is the
substitute for the contract suite the deferred
[S1a.9.2](../../plans/m1a_rust/p1a.9_release/README.md) would have been, and it
is stronger on the one axis that matters: it cannot rot without the gate going
red. The five Python pages beside it went stale precisely because nothing ran
them.

## Three consumers, and what each takes

| | what it links | why not the CLI |
|---|---|---|
| [M20](../../plans/m20_gui/README.md) — the GUI | the crates, into a Tauri backend | the backend **is** the Rust process; there is no server between them |
| [M1c](../../plans/m1c_external_validation/README.md) — external benchmarks | `ein-bench`, which shells out | a linked rival and a subprocess rival are not comparable measurements |
| [M2](../../plans/m2_nl_to_ir/README.md) — the NL frontend | undecided | its validator wants *why* a load failed as data, which linking gives for free and the CLI cannot ([Q-M2.14](../../plans/m2_nl_to_ir/open_questions.md)) |

## Adding the dependency

The crates are **not published to crates.io** and this milestone does not
claim a name there. Depend on them by path, from a checkout:

```toml
[dependencies]
ein-core   = { path = "../ein/ein.rs/crates/ein-core" }
ein-ir     = { path = "../ein/ein.rs/crates/ein-ir" }
ein-infer  = { path = "../ein/ein.rs/crates/ein-infer" }
ein-render = { path = "../ein/ein.rs/crates/ein-render" }   # optional
ein-einb   = { path = "../ein/ein.rs/crates/ein-einb" }     # optional
```

Six crates ship and two are dev-only (`ein-corpus`, `ein-parity`, both
`publish = false`). They stack linearly up to `ein-infer` and fork there:

```text
ein-core  ← ein-ir ← ein-infer ← ein-einb    (the .einb container)
                              ← ein-render   (DOT, markdown trace, dumps)
                                    ← ein-cli
```

Every crate is `#![forbid(unsafe_code)]` except `ein-einb`, which is
`deny` plus one `allow`ed module — `cast.rs`, the zero-copy reads, and the
only `unsafe` in the repository. **There is no async runtime anywhere**: the
engine is synchronous and `Send`, and `--jobs N` is a `rayon` pool built
inside `solve`.

## The five steps

| step | entry | crate |
|---|---|---|
| 1. arenas | `Ast::new()`, `Terms::new()` | `ein-ir`, `ein-core` |
| 2. load | `ein_ir::load_file(&mut ast, &mut terms, path)` — or `ein_ir::parse` then `ein_ir::load` | `ein-ir` |
| 3. saturate *(optional)* | `Saturator::new(&mut session)` + `.saturate(…)` | `ein-infer` |
| 4. solve | `ein_infer::solve::solve(&mut kb, &mut terms, &ast, &mut events, dumper, &opts)` | `ein-infer` |
| 5. read | `Answer` / `Verdict` / `Solution`, `goal_bindings`, `linearize` + `render_markdown` | `ein-infer`, `ein-render` |

### 1 — The two arenas

`Ast` owns the parsed forms; `Terms` owns the interned symbols, integers and
values. Both outlive the KB and are threaded through by `&mut`, because
loading *and* solving intern. A `Kb` is meaningless without the `Terms` it was
built against — the data model is integers
([design/03](../../plans/m1a_rust/design/03_data_model.md)), and a fact is a
row of `Value`s whose text lives in the interner.

### 2 — Load

`ein_ir::load_file` is parse + macro expansion + `(import std.*)` resolution +
KB construction, in one call, resolving file-relative imports against the
file's own directory. Where `std.*` comes from is
[`ein_ir::stdlib`](../../ein.rs/crates/ein-ir/src/stdlib.rs)'s three steps —
`$EIN_STDLIB`, a `stdlib/` found by walking up from the executable, then the
copy compiled in — and `ein --version` prints which one answered.

For a caller that already has the text, `ein_ir::parse(&mut ast, text,
filename)` returns the top-level forms and `ein_ir::load(&mut ast, &mut terms,
&forms, base_dir)` builds the KB. Both errors are structured values, not
strings you have to re-parse: `ParseError` carries the location,
`KbLoadError` the accumulated problems. **That is the difference a binding
would not have given** — the CLI can only hand back an exit code and a line of
text, which is why M2's validator argues for linking rather than for a Python
module.

### 3 — Saturate (optional)

`solve` saturates internally, so reach for `Saturator` only when you want the
deductive closure *without* the hypothesis search — what the monotonic rules
alone derive. It runs inside a `Session`, which is the `(kb, terms, ast,
events, memo)` bundle the engine's phases share.

### 4 — Solve

One entry, and **the verdict is read from the result rather than chosen**:
`k` distinct solution nodes → `Solution` (k = 1) / `Ambiguity` (k > 1) /
`Contradiction` (k = 0). There is no `mode` argument.

`SolveOptions` is a plain struct with a `Default`, so `..Default::default()`
is the idiom. The fields an embedder actually sets:

| field | |
|---|---|
| `stop_after` | `Some(1)` is the fast path — stop at the first complete, consistent model, `stats.exhausted == false`, so `k = 1` reads as *a* model rather than a certified-unique one. `None` exhausts the lattice |
| `max_set_size` | commitment-set depth, default 5 |
| `max_time`, `max_enterings` | budgets. What happens when one trips is `on_budget` |
| `store_lattice` | keep the proof, which step 5's trace needs and nothing else |
| `jobs` | `> 1` fans each lattice layer out over threads. **Same verdict, same models, same counters** — 20 712 corpus cells at `--jobs {2,4,8,16}` move nothing ([P1a.7](../../plans/m1a_rust/p1a.7_parallelism/README.md)). Inert without the `parallel` feature |
| `config` | the `SolverConfig` knobs; `None` takes the puzzle's own `(config …)` block |

`Events::off()` is the do-nothing narrator; `Events::to(sink, level)` writes
the [`--events` protocol](../kernel/inference/events.md) instead. `NoDumper`
is the do-nothing lattice dumper.

### 5 — Read

`solve` returns `Solved { answer, proof, stats, jobs }`. **`Answer::Aborted`
sits outside `Verdict` on purpose**, so a caller that never sets a budget
still has to name it once and exhaustive verdict handling stays exhaustive —
`solution_nodes == 0` under an abort means *unexplored*, not *proven
unsatisfiable*.

The bindings come from the **solution's own KB**, not from root:
`goal_bindings(&ast, &mut terms, &model.kb, None)` projects the puzzle's
`(query :goal …)` over the model, and `Some(node)` projects a different
question. A `Contradiction`'s `unsat_core` is the *source frontier* that
forces the conflict, not the conflicting facts themselves.

For the narrative, `ein_render::linearize` turns a `Solved` (with its proof)
into a `Trace`, and `render_markdown` renders it — the same document
`ein solve --trace out.md` writes.

## Worked example

```rust
use std::path::Path;

use ein_core::Terms;
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict, goal_bindings};
use ein_ir::Ast;
use ein_render::{LinearizeOpts, Mode, linearize, render_markdown};

/// Solve one `.ein` file and report the model, as a downstream crate would.
fn run(path: &Path) -> Result<String, String> {
    // 1 — the two arenas. `Ast` owns the parsed forms, `Terms` the interned
    //     symbols and values; both outlive the KB and are passed by `&mut`
    //     everywhere, because loading and solving both intern.
    let mut ast = Ast::new();
    let mut terms = Terms::new();

    // 2 — load: parse, expand macros, resolve `(import std.*)` against the
    //     file's directory, and build the KB. `ein_ir::parse` + `ein_ir::load`
    //     is the same thing in two steps, for a caller that has the text.
    let mut kb = ein_ir::load_file(&mut ast, &mut terms, path).map_err(|e| e.to_string())?;
    let loaded_facts = kb.n_facts();

    // 3 — solve. One entry, and the verdict is *read* from the result rather
    //     than chosen: `stop_after: Some(1)` is the fast path, `None`
    //     exhausts the lattice and certifies unique / ambiguous / unsat.
    //     `store_lattice` is what step 5 needs and nothing else.
    let opts = SolveOptions {
        stop_after: Some(1),
        store_lattice: true,
        ..SolveOptions::default()
    };
    let solved: Solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut Events::off(),
        &mut NoDumper,
        &opts,
    )
    .map_err(|e| e.to_string())?;

    // 4 — read. `Answer::Aborted` is deliberately outside `Verdict`, so a
    //     caller that never sets a budget still has to name it once.
    let mut out = format!("loaded {loaded_facts} facts\n");
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(model)) => {
            // The model is the solution's **own** KB — root plus what the
            // winning commitment derived. Projecting the goal over `kb` would
            // ask the question of the unsolved root and answer nothing.
            out += &format!("model: {} facts\n", model.kb.n_facts());
            for row in goal_bindings(&ast, &mut terms, &model.kb, None) {
                for (var, val) in row {
                    out += &format!("  {} = {}\n", terms.sym(var), terms.display(val));
                }
            }
        }
        Answer::Verdict(Verdict::Ambiguity(models)) => {
            out += &format!("{} distinct models\n", models.len());
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            out += &format!("unsat, core of {} facts\n", unsat_core.len());
        }
        Answer::Aborted { reason } => out += &format!("aborted: {reason}\n"),
    }
    out += &format!(
        "k = {}, exhausted = {}\n",
        solved.stats.solution_nodes, solved.stats.exhausted
    );

    // 5 — explain. The markdown derivation `--trace` writes, from the lattice
    //     proof step 3 asked for.
    let trace = linearize(&ast, &terms, &kb, &solved, LinearizeOpts::new());
    let md = render_markdown(&trace, Mode::Engine, false);
    out += &format!("trace: {} steps\n", trace.steps.len());
    let _ = md;
    Ok(out)
}
```

Output, on [`examples/zebra2.ein`](../../examples/zebra2.ein):

```text
loaded 84 facts
model: 434 facts
  h_water = House-1
  who_water = Norwegian
  h_zebra = House-5
  who_zebra = Japanese
k = 1, exhausted = false
trace: 244 steps
```

*The Norwegian drinks water in House-1; the Japanese owns the zebra in
House-5* — the canonical Zebra answer, and the solve
[the walkthrough](../kernel/inference/zebra_walkthrough.md) annotates as the M1
target trace. The three counts are asserted by
`the_worked_example_runs`; the other two verdict arms are exercised by
`the_other_two_verdicts_are_reachable`, so the `match` is not three arms of
which one has ever run.

## Caching a loaded KB — `.einb`

`ein-einb` writes a loaded (optionally saturated) KB as a binary container and
maps it back without re-parsing:

```text
ein_einb::save(&path, &ast, &terms, &kb, &opts)   →  a .einb file
ein_einb::open(&path, &mut terms, &opts)          →  Opened { ast, kb, .. }
ein_einb::is_einb(&bytes)                         →  dispatch on the magic
```

**It is a private cache format, never an interchange one.** Anything crossing
a tool boundary is `.ein` text or the event protocol's JSON. A reader refuses
a newer `FORMAT_MAJOR` rather than guessing, and `ein --version` prints the
major.minor this build reads.

## Features

| feature | default | |
|---|---|---|
| `ein-infer/parallel` | on | `SolveOptions::jobs > 1`. Off, `jobs` is accepted and inert |
| `ein-render/parallel` | on | forwards the above; `ein-render`'s solve helpers take a job count |
| `ein-infer/counters` | off | the work counters, compiled out entirely when off |
| `ein-cli/snmalloc` | on | the global allocator — on the **binary** only. A library that installs one makes the choice for everything that links it, which is why no engine crate has this row |

A crate that forwards a job count forwards `parallel`; the measured cost of
turning the defaults off is
[`feature_cost.md`](../../plans/m1a_rust/p1a.9_release/feature_cost.md).

## What is *not* the contract

The matcher, the per-rule compiler, the hypothesis generator, the
contradiction detector, the learned no-goods and the lattice driver's private
helpers are internals. They are `pub` because the workspace is one unit and
the tests reach into them — **`pub` here is not a stability promise.** What
this page names is; everything else moves.

The version is `0.1.0` and the crates are unpublished, so there is no semver
commitment yet. What *is* stable is the layer above: the CLI's surface, the
event protocol (`ein-events/1`), the `.einb` format major, and the IR language
itself, which M1a's invariant I1 froze for the whole port.

## See also

- [`docs/api/README.md`](README.md) — this subtree's index, and why five of
  its pages are history
- [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md)
  — the engine internals, module by module
- [`docs/kernel/inference/architecture_and_algorithms.md`](../kernel/inference/architecture_and_algorithms.md)
  — the language-independent algorithm view
- [`docs/kernel/inference/events.md`](../kernel/inference/events.md) — the
  `--events` protocol, for an embedder that wants the narration as data
- [`docs/install.md`](../install.md) — the binary, for when embedding is not
  what you want
