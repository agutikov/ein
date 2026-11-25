# Q-M2.7 — What language is the frontend written in?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"Rust"*.

**Read as:** the harness and the loop are Rust and reach the engine as crates, `ein-ir` to load, `ein-infer` to
solve, `ein-render` to explain ([`04-api.md`](../../../../docs/ein/ein.rs/04-api.md)): the answer as values (`ReadOut`,
`Verdict`, root's debts and core), a parse error with every error, its notes and its repairs, and
`ein_ir::dump_canonical`, which no command prints and five tasks of P2.2 and P2.4 key on. This was Q25 here.
The stage that was to decide it, *the boundary*, is gone from [P2.1](../../p2.1_kernel_as_instrumentation/README.md),
and no stage file writes a Python path.

**What it left to the re-plan, and what the re-plan took** (each a default, the user's to correct):

- **Four load-side errors are `String` newtypes** (`KbLoadError`, `LoadError`, `MacroError`, `CompileError`), so a
  Rust loop reads strings there until they carry a kind and a place: the same ground as
  [Q-M2.9](../kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md), and S2.1.2's first task.
- **Where the package sits**: a Cargo package of its own under `einaf/`
  ([Q-M2.8](Q-M2.8-where-does-einaf-live.md)) that depends on the engine's crates by path, outside the engine's
  workspace, so that nothing under `ein.rs/` names it
  ([S2.1.1](../../p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md)).
- **The benchmark's generators are Rust too**, in the same package, so that EinAf has one toolchain; the ruling
  names the frontend, and this goes one step past it.

**Kept from the entry, because it is cheap to write down and expensive to rediscover:** PyO3 is not a boundary
([Q-M1a.23](../../../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)); there is no socket
and no server, the engine ships as a library and a CLI; and `ollama` is no runtime where a grammar is the
mechanism, which the host checks confirmed on two versions ([Q-M2.14](../model/Q-M2.14-what-runs-the-model.md)).
