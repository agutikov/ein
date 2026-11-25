# Decisions: the engine

Decisions on `ein.rs` as built: the parser, the compiler's invariants, the store and its provenance, the `.einb` container, the agenda, and determinism.

- [Q2](Q2-ein-hands-no-problem-to-an-external-solver.md) — Ein hands no problem to an external solver
- [Q18](Q18-provenance-is-per-fact-an-or-of-derivations-over.md) — Provenance is per fact, an OR of derivations over premises
- [Q29](Q29-a-rule-compiles-once-per-activator.md) — A rule compiles once per activator
- [Q-M1a.7](Q-M1a.7-a-job-count-moves-nothing-a-run-prints.md) — A job count moves nothing a run prints
- [Q-M1a.18](Q-M1a.18-a-node-resumes-roots-saturation.md) — A node resumes root's saturation
- [Q-M1a.23](Q-M1a.23-ein-has-no-binding-for-another-language.md) — Ein has no binding for another language
- [Q-M1e.14](Q-M1e.14-a-jobs-sweep-compares-what-a-job-count-reaches.md) — A `--jobs` sweep compares what a job count reaches
- [Q-M1e.16](Q-M1e.16-a-firings-identity-keys-on-every-activator.md) — A firing's identity keys on every activator argument
- [Q-M1e.23](Q-M1e.23-a-routes-workers-share-nothing-they-write.md) — A route's workers share nothing they write
- [Q-M1e2.25](Q-M1e2.25-the-parser-is-generated-from-the-grammar.md) — The parser is generated from the grammar, and its recovery is deterministic
- [Q-M1f.8](Q-M1f.8-the-lexer-is-lrlexs-unpatched.md) — The lexer is lrlex's, unpatched
- [Q-M1f.9](Q-M1f.9-a-shape-the-grammar-refuses-is-a-loader.md) — A shape the grammar refuses is a loader invariant
- [Q-M1f.10](Q-M1f.10-a-einb-file-holds-no-integer-table.md) — A `.einb` file holds no integer table
- [Q-M1f.14](Q-M1f.14-a-keyword-pair-is-only-ever-a-forms-clause.md) — A keyword pair is only ever a form's clause
- [Q-M1f.40](Q-M1f.40-no-firing-meets-an-unbound-variable.md) — No firing meets an unbound variable
- [Q-M1f.43](Q-M1f.43-a-plans-activator-is-every-argument.md) — A plan's activator is every argument of its activator fact
- [Q-M1f.73](Q-M1f.73-a-rule-binds-at-most-256-variables-counted.md) — A rule binds at most 256 variables, counted by the loader
- [Q-M1f.75](Q-M1f.75-the-compilers-errors-are-a-library-callers.md) — The compiler's errors are a library caller's, in Ein's words
- [Q-M1f.84](Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md) — A compiler arm the loader covers is an invariant
- [Q-M1f.110](Q-M1f.110-a-container-saved-again-is-the-same-container.md) — A container saved again is the same container
- [Q-M1f.129](Q-M1f.129-a-rule-that-concludes-a-hypothesis-and-reads-not.md) — A rule that concludes a hypothesis and reads `not` reads the state it was asked about
- [Q-M1f.173](Q-M1f.173-the-register-file-stays-256-wide.md) — The register file stays 256 wide
- [Q-M1f.185](Q-M1f.185-a-state-deeper-than-the-parsers-limit.md) — A state deeper than the parser's limit is answered, saved and loaded again
- [Q-M1f.413](Q-M1f.413-a-nodes-container-opens-as-the-node.md) — A node's container opens as the node
- [Q-M1f.313](Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md) — The agenda fires first what leads to `(false)`
- [Q-M1f.436](Q-M1f.436-the-kernel-reads-no-theorys-name.md) — The kernel reads no theory's name
- [Q-M1f.503](Q-M1f.503-a-nodes-container-is-a-cache-it-compares.md) — A node's container is a cache: it compares the standard library, where its saved file compares nothing
- [Q-M1f.505](Q-M1f.505-ein-kb-save-saturate-of-a-node-banks-roots.md) — `ein kb save --saturate` of a node banks root's fixpoint, and the open commits the node
- [Q-M1f.589](Q-M1f.589-the-static-check-reads-not-as-any-relation.md) — The static check reads `not` as any relation
- [Q-M1f.644](Q-M1f.644-a-debug-build-checks-the-compile-walks-shortcut.md) — A debug build checks the compile walk's shortcut by the rules' facts counted by relation
- [Q-M1f.655](Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md) — Open facts are written at root and where a state is used
- [Q-M1f.659](Q-M1f.659-a-rule-whose-whole-match-is-a-pair-is-matched.md) — A rule whose whole match is a pair is matched by one probe, never seeded
- [Q-M1f.661](Q-M1f.661-a-nested-facts-relation-is-a-key.md) — A nested fact's relation is a key of the participation index
- [Q-M1f.674](Q-M1f.674-a-resumed-nodes-new-facts-are-read-off-its-own.md) — A resumed node's new facts are read off its own layers, above root's mark
- [Q-M1f.696](Q-M1f.696-a-search-keys-a-node-by-its-facts-beyond-roots.md) — A search keys a node by its facts beyond root's, and a state compared with one built another way by all of them
