# Ein — the Rust embedding API

How to drive the engine **as a library from another Rust program**: load a
`.ein` puzzle, ask a question about its answer — the region `R` — and read
what came back with its explanation.

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
between its two `page` markers. **Three** of that file's tests hold this page
to it — `the_page_quotes_this_file` diffs the code block,
`the_page_quotes_this_files_prose_too` diffs the paragraph below that names
the tests, and `the_page_and_the_file_name_the_same_tests` resolves every test
name printed here against a `fn` that exists, in both directions. The second
and third are M1e `CD-M4`: the naming paragraph sat *outside* the marked
region and had rotted there, which is the one class of drift a quote-this-file
check cannot catch. So the example compiles, runs, and produces the output
printed here — checked by `cargo test --workspace`, which is the gate. It is the
substitute for the contract suite the deferred
[S1a.9.2](../history/m1a_rust/README.md#p1a9--release) would have been, and it
is stronger on the one axis that matters: it cannot rot without the gate going
red.

## Three consumers, and what each takes

| | what it links | why not the CLI |
|---|---|---|
| [M20](../../plans/m20_gui/README.md) — the GUI | the crates, into a Tauri backend | the backend **is** the Rust process; there is no server between them |
| [M10](../../plans/m10_external_benchmarks/README.md) — external benchmarks | `ein-bench`, which shells out | a linked rival and a subprocess rival are not comparable measurements |
| [M2](../../plans/m2_nl_to_ir/README.md) — the EinAf loop | undecided ([Q25](../../plans/m2_nl_to_ir/open_questions.md#q25--what-language-is-the-frontend-written-in), taken in [S2.1.3](../../plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/s2.1.3_boundary.md)) | its loop wants *why* a load failed as data; linking gives `ParseError`'s fields for free, and for the rest M2's [S2.1.2](../../plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md) builds a versioned feedback object the CLI can write, which moves the boundary to a protocol |

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

Six crates ship and two are dev-only (`ein-corpus`, `ein-narration`, both
`publish = false`). They stack linearly up to `ein-infer` and fork there:

```text
ein-core  ← ein-ir ← ein-infer ← ein-einb    (the .einb container)
                              ← ein-render   (DOT, markdown trace, dumps,
                                              the model-set key)
                                    ← ein-cli
```

Every library crate is `#![forbid(unsafe_code)]` except `ein-einb`, which is
`deny` plus one `allow`ed module — `cast.rs`, the zero-copy reads. The one
other `unsafe` the engine ships is the binary's, not a library's:
`ein-cli/src/main.rs`, its own crate root outside `lib.rs`'s `forbid`, calls
`libc::signal(SIGPIPE, SIG_DFL)` once, as `main`'s first statement, so a
closed stdout ends `ein` by `SIGPIPE` as it ends `cat` (M1f S1f.5.25,
[`defined_behaviour.md` § 4.19](../kernel/defined_behaviour.md#419-a-closed-stdout-ends-ein-by-sigpipe--m1f-s1f525)).
A program that embeds the crates keeps its own disposition: the library
`ein_cli::run` leaves it alone, and the engine crates write nothing to stdout
(no `print!` in `ein-core`, `ein-ir`, `ein-infer`, `ein-einb` or
`ein-render`, 2026-11-10). **There is no async runtime anywhere**: the
engine is synchronous and `Send`, and `--jobs N` is a `rayon` pool built
inside a route.

## The five steps

| step | entry | crate |
|---|---|---|
| 1. arenas | `Ast::new()`, `Terms::new()` | `ein-ir`, `ein-core` |
| 2. load | `ein_ir::load_file(&mut ast, &mut terms, path)` — or `ein_ir::parse` then `ein_ir::load` | `ein-ir` |
| 3. saturate *(optional)* | `ein_infer::region::root_firings`, or `Saturator::new(&mut session)` + `.saturate(…)` | `ein-infer` |
| 4. ask | `ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)` — or `ein_infer::explore::run` for the answer without its states read | `ein-infer` |
| 5. read | `ReadOut` (`answer`, `states`, `queries`), `ein_render::trace::region::of_read_out` for the trace | `ein-infer`, `ein-render` |

### 1 — The two arenas

`Ast` owns the parsed forms; `Terms` owns the interned symbols and values
(and integers, until M1f S1f.5.4 retired the int kind). Both outlive the KB and are threaded through by `&mut`, because
loading *and* solving intern. A `Kb` is meaningless without the `Terms` it was
built against — the data model is integers
([design/03](../history/m1a_rust/design/03_data_model.md)), and a fact is a
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
strings you have to re-parse: `ParseError` carries every error recovery
reached, each with its line, column, notes and ranked repairs (since M1f
S1f.5.5's swap, when the parser generated from the grammar became the only
one), `KbLoadError` the accumulated problems. **That is the difference a binding
would not have given** — the CLI can only hand back an exit code and a line of
text, which is why M2's validator argues for linking rather than for a Python
module.

### 3 — Saturate (optional)

A route saturates root itself, so reach for this only when you want the
deductive closure *without* the search — what the rules alone derive — or, as
the example does, root's own firings for a trace.
`ein_infer::region::root_firings` saturates root in place and returns them,
and a route run on that root afterwards finds it saturated and leaves it as it
is. Below it is `Saturator`, which runs inside a `Session`, the `(kb, terms,
ast, events, memo)` bundle the engine's phases share.

> **`Saturator::is_stalled` is not a read.** It asks a question and changes
> the answer to later ones: a full enqueue pass runs first — a caller may have
> written facts to the KB outside `step`'s flow, and a stale queue would be a
> wrong answer rather than a cheap one — and that pass **advances the
> tiebreaker**, which is the FIFO position of every candidate enqueued after
> it. So a drive that probes quiescence can fire the same rules in a different
> order from one that does not. Nothing in the engine calls it; if you drive
> `step` yourself, probe at a boundary you are willing to have observed
> (M1e `CO-M6`).

### 4 — Ask

The answer is the **region `R`**: the commitments whose state is alive and
discharged
([`solution_semantics.md`](../kernel/inference/solution_semantics.md)). A
question about it is an `explore::Mode`, the two axes `ein solve` has — which
part of `R` (`Which::All`, `Minimal`, `Maximal`) and what is asked of it
(`Ask::Describe`, `Find(n)`, `Single`, `Count`) — and each is answered with the
least work it needs, by one of the routes
([`ein_infer::explore`](../../ein.rs/crates/ein-infer/src/explore.rs)). Every
route uses one node function, so no two questions can read one commitment
differently, and every answer is held to the oracle
(`ein-infer/tests/explore.rs`). **There is one answering path**: the walk,
which this page taught until M1e2 S1e2.1c.5, answered differently from
`ein solve` and was deleted.

`explore::Options::new(mode)` takes the defaults `ein solve` takes. The fields
an embedder sets:

| field | |
|---|---|
| `budget` | facts beyond root, summed over every fixpoint the run computes — `--region-budget`. A run that reaches it stops and says what it established, with `exhausted` false; it never changes an answer |
| `limit` | the largest `n` whose whole cube a program the static check flags gets — `--region-limit` |
| `jobs` | `> 1` fans the nodes out over threads, same answer. Inert without the `parallel` feature |
| `deadline` | `-T`: an `Instant`, past which the run is an error, never an answer |

`read_out::run` is `explore::run` plus every listed state read through the
node function: each state's `(model …)` and each `(query …)`'s projection
classes. `read_out::run_with` and `explore::run_with` take an `Events`
narrator: `Events::off()` is the do-nothing one, and `Events::to(sink,
level)` writes the [`--events` protocol](../kernel/inference/events.md).

### 5 — Read

`--single`'s word is `answer.verdict`: `Solution` (`R` is one state),
`Ambiguity` (two or more; `states` holds two), `Contradiction` (`R` is empty
and every node is dead) or **`Open`** (`R` is empty and every alive node
owes an obligation the program stated, M1d
[S1d.2.6](../history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus)).
`Open` is the arm an embedder is most likely to mis-file, because `R` is empty
exactly as it is for a `Contradiction`, and it means *not yet a model* where
`Contradiction` means *no model*. `None` means a budget or the deadline
stopped the run before it decided.

A described `R` is `answer.described`: its states with their commitments and
lines, its counts (`k_s`, `k_c`), `N`, root's own debts and core, and how the
run was partial if it was. The bindings come from **each state's own
fixpoint**, not from root: `queries[i].classes` are a query's projection
classes over the states listed, each with its binding rows and the rows
rendered through the query's `:goal-text`. A root whose fixpoint is dead
carries its unsat core in `described.root.core`: the *source frontier* that
forces the conflict, not the conflicting facts themselves.

For the narrative, `ein_render::trace::region::of_read_out` writes the
markdown `ein solve --trace out.md` writes: root's derivation, then each
listed state's from its first minimal commitment, then `N`.

## Worked example

```rust
use std::path::Path;

use ein_core::Terms;
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Verdict, Which};
use ein_ir::Ast;

/// Ask whether one `.ein` file's answer is one state, and report it, as a
/// downstream crate would.
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

    // 3 — root's derivation, kept for step 5's trace. The run below finds
    //     root saturated and leaves it as it is; a caller that wants no trace
    //     skips this.
    let firings = ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut Events::off())
        .map_err(|e| e.to_string())?;

    // 4 — ask. A question is a `Mode`: which part of R (all of it, the
    //     minimal line, the maximal line) and what is asked of it (describe
    //     it, `Find(n)` states, or `Single`: is it exactly one state?). The
    //     read-out answers it and reads every state it lists — each state's
    //     `(model …)` and each query's projection classes over them.
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Single,
    });
    let r =
        ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())?;

    // 5 — read. `verdict` is `None` only when a budget stopped the run before
    //     it decided, so a caller that never sets one still has to name it.
    let mut out = format!("loaded {loaded_facts} facts\n");
    match r.answer.verdict {
        Some(Verdict::Solution) => {
            // R's one state: the positive facts of the candidates' relations,
            // and the query's rows in it — `(query …)` asked of the state, not
            // of root, which would answer nothing.
            out += &format!("one state: {} facts over H\n", r.states[0].model.len());
            for class in r.queries.iter().flat_map(|q| &q.classes) {
                for row in &class.rows {
                    for (var, val) in row {
                        out += &format!("  {var} = {val}\n");
                    }
                }
            }
        }
        Some(Verdict::Ambiguity) => out += "R holds two states or more\n",
        Some(Verdict::Contradiction) => out += "R is empty: every node is dead\n",
        // Alive nodes, every one of which still owes an obligation the program
        // stated (M1d S1d.2.6). Not a model and not a refutation — a caller
        // that branches on "R is empty" alone would call it unsat.
        Some(Verdict::Open) => out += "R is empty: every alive node owes\n",
        None => out += &format!("undecided: {:?}\n", r.answer.stopped),
    }
    // `exhausted`: the states listed are all of the part of R asked about.
    // Under `Single` that is every word but `Ambiguity`, which lists two.
    out += &format!(
        "saturations: {}, R listed whole: {}\n",
        r.answer.saturations, r.answer.exhausted
    );

    // 6 — explain. The markdown `ein solve FILE --single --trace FILE.md`
    //     writes: root's derivation, then each state's, from step 3's firings.
    let md = ein_render::trace::region::of_read_out(
        &r,
        &mut kb,
        &mut terms,
        &ast,
        &firings,
        &ein_render::read_out::Options {
            command: "solve",
            argv: "",
            file: &path.to_string_lossy(),
            budget: opts.budget,
            uncut: false,
        },
        ein_render::trace::region::Shape {
            diagrams: false,
            relevant: false,
            reorder: false,
            full_kb: false,
        },
    )?;
    let steps = md.lines().filter(|l| l.starts_with("### Step ")).count();
    out += &format!("trace: {steps} steps\n");
    Ok(out)
}
```

Output, on [`examples/zebra2.ein`](../../examples/zebra2.ein):

```text
loaded 84 facts
one state: 25 facts over H
  h_water = House-1
  h_zebra = House-5
  who_water = Norwegian
  who_zebra = Japanese
saturations: 45, R listed whole: true
trace: 569 steps
```

> **This example asked the walk until 2026-11-03**, `ein_infer::solve::solve`
> with `stop_after: Some(1)`, and printed *k = 1, exhausted = false* over a
> 444-fact model. That was a second answering path beside `ein solve`'s, and
> it answered differently; M1e2
> [S1e2.1c.5](../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md)
> deleted the walk on the user's ruling of that day and rewrote the example on
> the routes `ein solve` takes. `one state` counts the state's `(model …)`,
> the positive facts over H's relations and root's own, which is what tells
> R's states apart; the trace is root's derivation and the state's.

*The Norwegian drinks water in House-1; the Japanese owns the zebra in
House-5* — the canonical Zebra answer, and the solve
[the walkthrough](../kernel/inference/zebra_walkthrough.md) annotates as the M1
target trace.

<!-- prose -->
The counts above are asserted by `the_worked_example_runs`, which takes the
`Solution` arm; `the_other_three_verdicts_are_reachable` takes
`Contradiction`, `Ambiguity` and `Open`, on three other files. So the `match`
is five arms of which **four** have run. The fifth, `None`, needs a budget no
example here sets, and it is in the page because a caller that never sets one
still has to name it.
<!-- prose -->

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
| `ein-infer/parallel` | on | `explore::Options::jobs > 1`. Off, `jobs` is accepted and inert |
| `ein-render/parallel` | on | forwards the above |
| `ein-infer/counters` | off | the work counters, compiled out entirely when off |
| `ein-cli/snmalloc` | on | the global allocator — on the **binary** only. A library that installs one makes the choice for everything that links it, which is why no engine crate has this row |

A crate that forwards a job count forwards `parallel`; the measured cost of
turning the defaults off is
[`feature_cost.md`](../history/m1a_rust/measurements/feature_cost.md).

## What is *not* the contract

The matcher, the per-rule compiler, the hypothesis generator, the
contradiction detector, the node function and the routes' private helpers are
internals. They are `pub` because the workspace is one unit and
the tests reach into them — **`pub` here is not a stability promise.** What
this page names is; everything else moves.

The version is `0.1.0` and the crates are unpublished, so there is no semver
commitment yet. What *is* stable is the layer above: the CLI's surface, the
event protocol (`ein-events/1`), the `.einb` format major, and the IR language
itself, which M1a's invariant I1 froze for the whole milestone.

## See also

- [`docs/api/README.md`](README.md) — this subtree's index
- [`docs/kernel/inference/implementation.md`](../kernel/inference/implementation.md)
  — the engine internals, module by module
- [`docs/kernel/inference/architecture_and_algorithms.md`](../kernel/inference/architecture_and_algorithms.md)
  — the language-independent algorithm view
- [`docs/kernel/inference/events.md`](../kernel/inference/events.md) — the
  `--events` protocol, for an embedder that wants the narration as data
- [`docs/install.md`](../install.md) — the binary, for when embedding is not
  what you want
