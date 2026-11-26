# M14 — Ein server

**Status:** created 2026-11-27 on the user's instruction ([below](#the-instruction)); **not started**. The
instruction names what the server can do; this page reads each line against the engine as it ships and lists what
would have to be decided. Nothing is designed, nothing has a default, and it carries no estimate (the user's ruling
of 2026-11-08).
**Depends on:** the engine that ships, [`ein.rs/`](../../ein.rs/), driven as a library
([`docs/ein/ein.rs/04-api.md`](../../docs/ein/ein.rs/04-api.md)).
**Feeds:** [M15](../m15_mcp_server/README.md), an Ein MCP server, and [M17](../m17_long_text_formalization/README.md),
texts longer than a model's context, both named by the instruction; and Ein as a tool a model calls, M2's
[Q-M2.13](../m2_nl_to_ir/open_questions/einaf/Q-M2.13-who-calls-the-kernel.md).
**Where it sits** against M2 and the milestones after M1f is not ruled; it starts only on the user's word.

## The instruction

The user, 2026-11-27, verbatim:

> M14 - Ein server
> Ein in a form of server
> reusable rule libs already loaded
> one programs as a database
> can try saturation,
> can serach for R with limits or iterate over $H$ step by step
> can query different program's found properties
> can query different parts of separate states
> of course can query found models
> Everything that can be useful for Ein MCP and M20

*M20* in the last line is the milestone the same message created for texts that do not fit into a model's context.
M20 is the GUI's ([M20](../m20_gui/README.md)), and the user gave that milestone **M17**.

## Each line, against what ships

1. **Ein in a form of server.** Today a question is a process (`ein solve`, `ein saturate`, `ein show`, `ein
   render`, `ein test`, `ein oracle`), or a Rust program links the crates and runs the five steps of
   [`04-api.md` § 2](../../docs/ein/ein.rs/04-api.md#2-the-five-steps); no process keeps a program between two
   questions. Server mode was dropped from M1a on 2026-08-18, and
   [Q-M1a.23](../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md) names when a binding
   is added: *"state that must persist across many small queries in one process"*. This milestone is that
   consumer.
2. **Reusable rule libraries already loaded.** Today every import brings its module whole, one copy per program,
   parsed and loaded with it; the standard library is embedded in the binary, and `.einb` caches a loaded knowledge
   base ([`04-api.md` § 4](../../docs/ein/ein.rs/04-api.md#4-caching-a-loaded-kb-einb)). New: a module loaded once
   and shared by every program that imports it. A program's names are interned in its own arena (`Terms`), so what
   can be shared is a question of the arenas.
3. **One program as a database.** Read here as: a loaded program held by the server under a name, its knowledge
   base at root and the states found, asked again with no load. The pattern exists inside the tests: a corpus sweep
   loads a program once and runs every question on a copy (`corpus_ops::Loads`; `Ast`, `Terms` and `Kb` are
   `Clone`). New: a program kept across calls; and whether it may change after load (facts added, as M17's
   chapters are), which no command allows today.
4. **Can try saturation.** Today `ein saturate`, root's fixpoint, bounded by `--max-steps` and `-T`. New: a
   saturation asked of a held program, and of one with facts added since, from where the last one stopped.
5. **Can search for R with limits.** Today a bare `ein solve` describes R, and `--single`, `-n N`, `--count`,
   `--minimal`, `--maximal` and `--no-search` each do the least work their question needs, under
   `--region-budget`, `--region-limit`, `--max-steps` and `-T`
   ([`06-config.md`](../../docs/ein/user/04-reference/06-config.md)). New: the same questions of a held program.
6. **Or iterate over H step by step.** Today `ein show hyps` lists H, `ein render h-space` draws its structure
   ([`03-hypotheses-and-the-answer.md`](../../docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md)),
   and `--dump-lattice DIR` writes every node a run visited; the walk the search over R replaced is gone with its
   flags. New: the client commits a hypothesis, sees the state (saturated, alive or dead with its core, what it
   owes), then commits another or goes back: the lattice of
   [`05-hypothesis-space.md`](../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md) walked by the
   client, one commitment at a time. A state is a copy-on-write layer over root, so holding several is cheap in
   principle.
7. **Can query different programs' found properties.** Today a run reports the static check's verdict (monotone, or
   flagged), H's structure (exclusions, groups, what is exactly one, what couples the groups; `h-space.json`), R's
   counts and the summary `ein-summary/2`. New: each asked of a held program, and compared across programs.
8. **Can query different parts of separate states.** Today a state is printed whole (`--dump-answer DIR`, each
   state loadable, [`10-answer-dump.md`](../../docs/ein/user/04-reference/10-answer-dump.md)). New: a pattern asked
   of one state's facts, several states side by side, what one holds and another does not.
9. **Of course can query found models.** Today R's members are read through `--dump-answer`, the read-out and the
   summary, each a run. New: kept after the search, and asked by pattern with no search again.
10. **Everything useful for Ein MCP and M17.** [M15](../m15_mcp_server/README.md)'s open list (which commands are
    tools, what a tool takes and returns, what a call that does not end does) is this server seen from a model.
    M17 loads a text's chapters into the server, merges them and asks for the conflicts.

## What would have to be decided

**None of these is decided, and none has a default.**

- **Whose it is.** Ein's, a crate of `ein.rs/` beside `ein-cli`; or a project over Ein, as EinAf is, linking the
  crates. A client of EinAf's is one Ein may not name.
- **The protocol and the transport**: JSON-RPC over stdio, HTTP, a socket, or MCP's own. That decides whether
  M15 is a face of this server, or this server a library M15 wraps.
- **What a program is in the server**: its name, its lifetime, whether it may change after load (a fact, a module,
  a rule added), and what a change does to what was found before it. A held R is the R of the program as it was.
- **What is shared between programs**: a module's compiled rules, the standard library, the interning arena, and
  what sharing does to a state printing the main file's spelling of a name.
- **The questions and the shapes of their answers**: which of the CLI's questions, with which options, and what
  each returns. The machine surfaces that exist are `ein-summary/2`, the event protocol
  ([`05-events.md`](../../docs/ein/ein.rs/05-events.md)) and `ein test --json-report`; a new one is a contract with
  a page and a test.
- **The language of a query over a state**: Ein's own, a `:goal` held to what a rule's `:match` is held to, or
  another.
- **What a step of H is**: one hypothesis committed, or one node of a route; what the client sees at each; whether
  going back is a step; how a walk the client drives agrees with the routes.
- **Limits, cancellation and memory**: the region budget (2²⁰ facts by default), the cap on a saturation's firings,
  a clock, a question that does not end, many programs held at once; what the server says it established when a
  limit cuts.
- **Concurrency**: the engine is synchronous and `Send`, and `--jobs N` is a pool inside a route; several clients,
  several programs, one question a program at a time or not.
- **Persistence**: whether held programs and what was found survive a restart.
- **Where it lives**, how it is tested, and how it is released.

## What it must not assume

- **That an answer changes with the surface.** No two commands may disagree, and every mode is held to `ein
  oracle`: what the server answers of a held program is what the command line answers of the same program loaded
  afresh.
- **That a server option may change R.** No config option, lever or flag changes R; one of the server's is a lever.
- **That a held program is asked more than its question.** Each question does the least work it needs.
- **That a flagged program's answer is claimed.** The server says what every command says of it, in capitals.
- **That the kernel reads a theory's name.** A query reads relations as the program names them.

## Connections

- [M15](../m15_mcp_server/README.md), an MCP server: a model's face of what this server holds.
- [M17](../m17_long_text_formalization/README.md), texts longer than a model's context: chapters loaded into this
  server, merged, their conflicts asked for.
- [M2](../m2_nl_to_ir/README.md), EinAf: its one door to the kernel, `einaf::kernel::call`, loads a program a call;
  Ein is a tool the model may call ([Q-M2.13](../m2_nl_to_ir/open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)).
- [M20](../m20_gui/README.md), the GUI: links the crates directly, and shows what a held program would hold.
- [M1h P1h.2](../m1h_beyond_word_problems/p1h.2_slicing/README.md), slicing: what of a program an answer used, a
  part of a state asked for.
- [Q-M1a.23](../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md): no binding, and when
  one is added.
- The reasoning it would serve: [finding R](../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md), [the
  hypothesis space](../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md),
  [cores](../../docs/ein/reasoning/02-kb-evolution/09-cores.md).
