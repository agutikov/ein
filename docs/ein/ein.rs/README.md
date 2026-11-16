# `ein.rs`: the engine as built

The concrete implementation: the Rust workspace under [`ein.rs/`](../../../ein.rs/), its crates and modules, the
design contracts the code cites as its specification, the engine's own read-outs, and how to drive it as a library.
What a program means and what the answer is are stated without reference to this code elsewhere in `docs/ein/`;
these pages say how `ein.rs` computes it.

| page | what it holds |
|---|---|
| [`architecture.md`](architecture.md) | the crates and how they stack, one `ein solve` from file to read-out, every module and what it owns, where each part of the language lives in code |
| [`algorithms.md`](algorithms.md) | the algorithm `ein.rs` uses for each operation, where that places it, and the levers that change how and never what |
| [`design/`](design/README.md) | **the design contracts**: determinism and order, shared assets, the data model, the IR frontend, the matcher, saturation, the search, parallelism, the binary format, the toolchain and layout |
| [`api.md`](api.md) | **embedding the crates**: the five steps from a file to an answer, a worked example that is a test, the `.einb` cache, the features, what is and is not the contract |
| [`events.md`](events.md) | the `--events` protocol, `ein-events/1`: every kind, its payload, how two streams are compared |
| [`lattice-dump.md`](lattice-dump.md) | `--dump-lattice`: every node a run visited, as `nodes.json` and `nodes.md` |
| [`reserved-engine-strings.md`](reserved-engine-strings.md) | the engine's internal vocabulary: its reserved names, the names the loader makes up, the dunders' effects |
