# F6 — Engine reads + modifies its own harness code

**Theme owner:** the user.
**Trigger:** the most distant of the self-modification followups; not
on any near-term horizon.

## What this is

The most ambitious self-modification rung. F2 modifies the *grammar*
(syntactic); F5 modifies the *rule library* (semantic, declarative);
**F6 modifies the engine itself** — the Python code under
`src/ein/`. A rule's RHS can produce a *patch* to a Python file;
the harness applies it, reloads the module, and continues running.

Variants this could take:

- **Code suggestion** — the engine emits *proposed* patches to its
  own source as a side-channel output; a human reviews + applies.
  Minimal risk; useful as a refactoring suggester or as a
  "self-improvement log" for research.
- **Sandboxed self-patching** — the engine writes patches to a *copy*
  of itself, runs the test suite against the copy, and reports
  whether the patch is a Pareto improvement (correctness preserved,
  some measurable property — speed, trace length — improved). If so,
  the patch is offered to the human; if not, discarded.
- **Online self-patching** — the engine modifies its own running
  process via Python hot-reloading or via writing files + spawning a
  new instance. This is the maximal-risk form and is *not* on any
  realistic Ein timeline.

## Why deferred indefinitely

The mechanism (Python AST manipulation, importlib reloading,
sandboxing) is well-understood. The *unsolved* parts are:

- **The bridge from the IR to source-code patches.** A rule's RHS
  currently produces *facts*; producing a Python AST patch needs an
  intermediate "IR-as-AST" mapping that doesn't exist.
- **The test-or-revert harness.** Sandboxed self-patching needs a
  trusted oracle (the existing pytest suite, plus regression
  benchmarks). M1 ships 300+ tests, which is enough for a
  *correctness preserved* check; the *improvement* check needs
  property metrics not yet defined.
- **Audit trail.** Every self-applied patch needs a recoverable
  history, preferably as commits on a sandbox branch. Git
  integration into the engine is not a research priority.
- **The composition problem.** F6 + F5 + F2 together compose into a
  system that can mutate its grammar, its rules, *and* its harness.
  Containing the resulting feedback loop is its own design problem
  — the *aligned superintelligence* problem in miniature, on a tiny
  formal substrate.

## What promotion would look like

Only as part of the shared `m_followups_self_modifying/` milestone
(see [F2](f2_self_modifying_language.md), [F5](f5_rules_as_data.md)).
F6 is the *outermost ring* — implemented last, gated by F2 + F5
working, with an explicit research bound:

- **PF6.1** — IR → Python patch bridge. A small DSL for "add this
  method to this class", "rename this parameter", "insert this
  decorator" — *not* arbitrary Python edits.
- **PF6.2** — sandbox runner. Take a patch, apply to a worktree,
  run the test suite, gather property metrics, return a verdict.
- **PF6.3** — code-suggestion mode (the *non-risky* form): emit
  patches to stdout, never apply automatically. Useful as a
  refactoring suggester for `ein/` itself.
- **PF6.4** — gated self-application. Apply only with explicit
  human approval per patch; commit to a sandbox branch.
- **PF6.5** — research bound: *not* online self-modification within
  a running process; the milestone explicitly excludes hot-reload.

## Risks

- **The classical risks of self-modifying code** — undebuggable
  divergence, "bricked" engine state, audit-trail loss. Sandbox +
  test-or-revert + human-gated application contain these.
- **Unproductive recursion.** The engine might propose patches that
  pass the test suite but degrade in ways no metric catches (trace
  readability, code clarity). The "improvement" criterion is the
  hard part.
- **Scope creep into general code synthesis.** The line between
  "engine modifies itself" and "engine writes new programs" blurs
  fast. F6 explicitly does *not* aim at the latter; the codebase
  under modification is `src/ein/` only.

## Prior art / connections

- *Self-improving software* literature: classic AI dreams (Lenat's
  Eurisko), modern LLM-as-compiler experiments. Most are unfunded
  research; few have a controlled-domain success story.
- F2 + F5 (the syntactic + semantic prerequisites). F6 doesn't make
  sense without them.
- [`docs/ideas/01-self-modifying-constraint-language.md`](../ideas/01-self-modifying-constraint-language.md)
  — F2 lineage; F6 is the maximal extension of that thread.
- [`docs/ideas/10-generic-self-modification.md`](../ideas/10-generic-self-modification.md)
  — the umbrella view of the three followups.
- [`docs/lib/01-llm-constrained-generation.md`](../../docs/lib/01-llm-constrained-generation.md)
  — constrained-generation infrastructure F6 might emit through.
