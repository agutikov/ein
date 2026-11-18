# S1f.23.4 — the kernel reads no theory's name, and the built-in symmetry goes

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word, in the worktree `build/wt-s1f234` on the branch
`s1f.23.4-no-theory-names`, beside S1f.23.1 (master) and S1f.23.2 (its own worktree); the main session merges it,
and one gate, `utils/ci_container.sh` and one nightly hold the phase's stages on the merged tree.

## The ruling

The user, 2026-11-18, verbatim:

> In general: ein.rs must expect the minimal shape of the program: rules, relations, ... Not special names like
> `is-a` `type` `instance` `symmetric` `bijection` `slot`; All those names belong to some specific theory and must
> not pollute the kernel.

and, on the main session's census of what `ein.rs` reads by a theory's name (production code, a string literal
compared or looked up) and its proposal of this stage, with the question whether the built-in `__symmetric__` is this
stage's or P1f.21's:

> Yes, let's drop also built-in symmetry.

The census, at the ruling: `grid.rs` and `models.rs`' key (`is-a`, `T`, `slot-partition`, `functional`, `injective`,
`bijective`) go with S1f.23.2; `kb_dot.rs`' and `ir_dot.rs`' `is-a` and `constraints.rs`' `type`/`instance` go with
S1f.23.1; **left for this stage**: `ein-ir/src/from_ir.rs`' `program_relations`, which makes `is-a` every program's
relation beside `eq` and `neq`; and **`__symmetric__`**, the saturator's native mirror (`ein-core/src/terms.rs`'
`SYMMETRIC` and `Kernel::symmetric`, `ein-infer/src/saturator.rs`' `SYMMETRIC` and the mirror, `ein-narration`'s
`mirror`), written by no standard-library module (`std.algebra`'s `symmetric` is a rule), only by
`examples/features/06_symmetric_native.ein` and four witnesses under `examples/ein-bugs/wrong-answers/`
(`git grep -l __symmetric__`). `__closed__`, search control like `open` and `false`, is no theory's name and stays,
P1f.21's.

## What it does

1. **First, the invariance**: every program that writes `(__symmetric__ R)` (the corpus's, and the tests' if any)
   answers the same with that fact replaced by `std.algebra`'s `symmetric` (imported, `(symmetric R)`): R, every
   mode, the oracle, the verdicts of its claims. A program where they differ is shown before anything is removed, and
   is the user's.
2. **The built-in symmetry goes**: the mirror in the saturator and wherever it is read (the agenda, the matcher, the
   narration's `mirror`, the events, the read-out), `Kernel::symmetric`, `SYMMETRIC`; `__symmetric__` is no longer
   reserved (`ein_core::text`, the blind enumerator's exclusion, `dunder_objects.rs`), an ordinary name a program may
   write and which means nothing to the kernel. Its example and the four witnesses are rewritten to `std.algebra`'s
   rule where what they show still stands, or retired with their corpus entries where they show only the mirror, each
   said in the record; their rows in `corpus/regressions.toml` follow (a fixed wrong answer of a mechanism that is
   gone is retired with it, the row's history in git).
3. **`program_relations` names no theory**: `is-a` leaves its list; `eq` and `neq`, the kernel's computed relations,
   stay. A goal or claim over `is-a` in a program that never writes `is-a` is then refused as any unknown relation
   is; the corpus programs that do that, if any, are shown.
4. **A test holds the rule**: every corpus program with every name that is not the kernel's renamed (a fixed
   bijection onto fresh names, the program's text rewritten through the parser, imports and standard-library modules
   renamed alike) gives the same answer, the same claims' verdicts and the same drawings, apart from the names. Its
   tier is the gate's where it fits the gate's budget, else the nightly's, by measurement; a program it cannot hold is
   named with why. The kernel's names are `ein-core/src/terms.rs`' `Kernel` after this stage.
5. **The pages and decisions**: `AGENTS.md`'s standing rule on `__closed__` and `__symmetric__`;
   `03-reserved-names.md`, `ein.rs/07-reserved-engine-strings.md`, the saturation's pages and algorithms where they
   state the mirror; every decision that states it (`git grep -l '__symmetric__\|native mirror' -- docs`), rewritten
   to the answer that stands. P1f.21's `open_facts.md` § 8 lists `__symmetric__` among the names left: a line there
   says it went here.

## Readings and rulings to file

The main session files the user's two rulings above, and this stage's readings (each with its default, written in
its Record), in `open_questions.md` on merge; the stage does not edit that file.

## Acceptance

- Step 1's invariance holds on every program that wrote `(__symmetric__ R)`, or what differs is the user's.
- No other answer moves: the shape golden's lines move only for the programs rewritten or retired, and for what the
  narration of a mirror wrote.
- The crate tests it touches, `cargo test --profile gate --workspace` once in the worktree when no other cargo run
  is going, `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/check_generated.py`,
  `utils/check_release_voice.py`, `utils/wrong_answers.py`, green in the worktree; the gate, `utils/ci_container.sh`
  and the nightly once, on the merged tree, by the main session.

## Record

Not started.
