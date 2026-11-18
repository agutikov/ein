# S1f.23.7 — a slice of the knowledge base defined by a query, and drawn

**Phase:** [P1f.23](README.md)
**Status:** planned 2026-11-18 on the user's answer to Q-M1f.426; task 1, the examples, started the same day on the user's word (*"go ahead"*), and **accepted 2026-11-19** (*"S1f.23.7 - I accept"*): its ten open points ruled as their defaults, [Q-M1f.523 to .532](../open_questions.md#q-m1f523--the-slice-by-a-query-its-definition). **Task 2, the build, done 2026-11-19** (`519fadd2`), after S1f.23.10's merge (Q-M1f.522); its Record below. R-41 stays open.

## The user's words

On Q-M1f.426, what the constraint view draws (the answers' file, `e4c0eaf6`), verbatim:

> So, it turns out that we have commands that render part of KB. I'm not against it - (a). Small addition - would be
> nice to formalize this KB slice with something, e.g. ein query or rule that selects all nodes where relation is
> argument (is it correct definition?), and together with `constraints` allow to render arbitrary query.

## What it asks

- **The constraint view's slice, stated in Ein**: a query (or a rule) whose answers are exactly the facts the view
  draws. The user's definition, *every fact with a relation among its arguments*, is close to the view's today
  (Q-M1f.426 (a): every un-annotated fact of a relation the program does not declare, the declarations among them) and
  not the same: a fact with a relation as an argument that the program gives with a `:source` is in the first and not
  the second, and an undeclared table of plain objects (a digit table) in the second and not the first. The stage
  shows both on the corpus and puts the definition to the user.
- **Any query's slice drawn**: `ein render kb` (or a sibling) given a goal draws the facts that answer it, with what
  they point at, in the canonical form; the constraint view becomes one such query. Q-M1f.431's query part of the
  program view, M20's *arbitrary query on a KB state* and M1h's P1h.6 (an open fact as a question) read the same.

## Stages

1. **Examples** (stops for the user): the two definitions side by side on the corpus's programs, counted; the slice
   of each as `ein render kb` would draw it, mocked from today's drawing; how a goal is given (a flag, a form in the
   program, a saved state with a query).
2. **The build**, on the user's rulings.

## Record

- Task 1 done, `ef2010ce`, waiting for the user's look: [`s1f.23.7_examples.md`](s1f.23.7_examples.md), its programs, scripts and outputs in [`s1f.23.7_examples/`](s1f.23.7_examples/).
- Its three findings outside the stage, filed in [the phase's findings](findings.md) as R-39 to R-41, each open: an
  `(or …)` goal's class prints the `or` form (open point 10); the constraint view draws a written check, `(eq a w)`, as a
  fact (open point 9); a rule taking a fact's relation as its parameter is refused when activated with `symmetric`
  (Q-M1f.1, § 4.5).
- Task 1 accepted by the user, 2026-11-19, verbatim: *"S1f.23.7 - I accept"*. The ten open points of
  [§ 6](s1f.23.7_examples.md#6-open-points-for-the-user) are Q-M1f.523 to .532, each its default. Task 2 builds
  them: the slice in the engine, `ein render kb --goal '(…)'` (of root or a saved state, `--query N` beside it), and
  `ein render constraints` as the slice of Q-M1f.523 drawn by the KB view's code; with it R-40 (a written check left
  out, Q-M1f.531) and R-39 (an `(or …)` goal's class lists the facts of the disjuncts that hold, Q-M1f.532). R-41
  (a relation-parameter rule activated with `symmetric` refused, Q-M1f.1) stays open: no default was taken for it.
- **Task 2, the build, done**: `519fadd2` (code, tests, pages, decisions), and this Record (plans only). Run on the
  user's word for the phase (*"Go with all defaults to the end of this phase p1f.23 and stop"*); no answer of a corpus
  program moved, so nothing stopped it.

### Task 2 — what was built, ruling by ruling

- **The slice, Q-M1f.523 (with .524 to .526)**: `ein_infer::slice::about_relations(kb, terms)`, every fact the
  knowledge base holds, in its order, with an atom of `ein_core::Program::relations_of` directly under an argument
  arrow. `relations_of` is new: the loader keeps the list a goal and a claim are held to (`from_ir`'s `holdable`, the
  program's relations by Q-M1f.522, the kernel's words it writes, `eq`, `neq`, an activator's relations), interned, in
  the order of their text. An atom is a relation wherever it stands (`z`, `eq` kept); directly, arrows 2, 3, ….
- **Drawn with what it points at, Q-M1f.527 (with .528)**: `ein_render::kb_dot::KbDotOpts::only`, the KB view's own
  `fact_set` walking the kept facts instead of every fact; a fact under an arrow is its node, held in no frame, held
  nowhere in its frame; a slice keeps held facts only; an empty slice is `// the slice keeps no fact`.
- **A goal's slice, Q-M1f.529**: `ein render kb FILE --goal GOAL` and `--query N` (exclusive). `ein_ir::read_goal`
  parses `(query :goal GOAL)` into the program's `Ast` under the name `--goal`, expands it by the program's macros
  (rebuilt from `Program::macros`), and refuses it by the walks a query's goal gets (`refuse_goal`, now one function
  `validate_queries` shares), its relations held to `relations_of` and its counts to the declarations;
  `ein_infer::slice::answering` takes the premises of every match whose guards hold (`verdict::goal_matches`, the
  matcher's `Match::premises` and `disjunct`, which `goal_bindings` now wraps). Matched against the KB drawn, root's
  fixpoint or a saved state; digraph `goal`; `--since`, `--colour-by`, `--fold` apply.
- **`ein render constraints`, Q-M1f.530**: loads, saturates root (no saved node committed), draws
  `render_constraints(kb, terms, …)` = the KB view with `only = about_relations`. In the shape digests the two views
  moved from the parse views to the KB views, drawn after root's saturation; `dot_wellformed` skips them where root
  does not saturate, as `kb-since`. Q-M1f.426 and Q-M1f.71 amended.
- **A written check, Q-M1f.531 (R-40)**: no code of its own, the view draws a knowledge base, which holds no `eq` or
  `(not (eq …))`; tests on `fact-eq-false.ein` and `not-eq-fact.ein` (each draws its two declarations and nothing of
  `eq` or `not`).
- **An `(or …)` goal's class, Q-M1f.532 (R-39)**: `read_out`'s per-state `answered` grounds, for each match, the atoms
  of the disjunct it holds by (`ein_ir::expect::goal_disjuncts`); `goal_atoms` is every disjunct's atoms, so the
  pasted claim's facts match the goal. Not one corpus program writes an `(or …)` goal, so no read-out line of the
  corpus moved.

### The CLI as it now reads

```text
ein render kb FILE [--since FILE] [--colour-by relation|origin|none] [--goal GOAL | --query N] [--fold] [--no-require-why]
ein render constraints FILE [--fold] [--no-require-why]
```

`--goal`: *draw the goal's slice: the facts of the KB drawn that answer GOAL, written as a (query :goal …) writes it,
with what they point at. Held to what a query's :goal is held to, and refused as one would be.* `--query N`: *draw
the slice of the goal of the program's N-th (query …), from 1: the facts of the KB drawn that answer it, with what
they point at; its :over and :expect are not read.* `constraints`: *the facts about relations: every fact root holds,
at its fixpoint, with a relation of the program directly under an argument arrow, drawn as `render kb` draws a fact,
with what it points at.* Built binary, *output*:

- `render kb examples/zebra2.ein --goal '(and (relation ?r) (symmetric ?r))'`: 2 octagons, `(relation next-to)` and
  `(symmetric next-to)`; `--query 1` at root: no octagon, `// the slice keeps no fact`; of `--dump-answer`'s
  `models/01.ein`, `--query 1`: 4 octagons, two of `nation-loc`, one of `drink-loc`, one of `pet-loc` (task 1's
  § 5.1 mock, now drawn).
- `render kb examples/beliefs/01_common_belief.ein --goal '(believes ?who (likes Dan ?food))'`: 5 facts of
  `believes`, `(likes Dan Soup)` in no frame, `(likes Dan Tea)` in `believes ctx` (§ 5.3's mock).
- `render kb examples/arith/add.ein --goal '(sum-of apples pears ?fruit)'`: 3 octagons, the fact and its two
  numerals, in `sum-of ctx` and `n ctx` (§ 5.4's mock).
- `--goal '(payed ?x)'`: `kb load error: --goal:1:15: query: its :goal writes (payed ?x), and the program names no
  relation payed — …`, exit 1; `--goal '(p ?x'`: the parser's `--goal:1:20: unexpected end of input`, exit 1;
  `--goal '(p ?x)) (q'`: `kb load error: --goal: '(p ?x)) (q' is not one goal — …`, exit 1; `--query 9`:
  `--query 9: examples/zebra2.ein has 1 (query …) form`, exit 1; `--query 0` and `--goal` with `--query`: exit 2.
- `render constraints`, octagons now and before: `zebra2` 169 and 20 (task 1's U (c) on root: 169 of 390);
  `beliefs/01` 7 and 4; `arith/add` 511 and 16.

### R-39, before and after (*output*)

`s1f.23.7_examples/02-goals.ein`, query 10, `(or (and (relation ?r) (includes ?r ?s)) (and (relation ?s) (includes ?r
?s)))`: before `as :expect (model (or (and (relation right-of) (includes right-of next-to)) (and (relation next-to)
(includes right-of next-to))))`; after `as :expect (model (includes right-of next-to) (relation next-to) (relation
right-of))`, and that claim pasted under the query (as `run.sh` builds `or-pasted.ein`) loads and **holds** under
`ein test` (before: `unexpected or`). `zebra2` with `(query :goal (or (symmetric ?r) (bijective ?r) (transitive
?r)))`: before seven `(or …)` forms, one a row; after `(model (bijective color-loc) (bijective drink-loc) (bijective
nation-loc) (bijective pet-loc) (bijective smoke-loc) (symmetric next-to) (transitive is-a*))`, which holds pasted
back. The rows are the same before and after.

### Goldens and checks

- **Shape golden** (`EIN_BLESS=1 EIN_CORPUS_SLOW=1 … --test id_order_invariance`): 1 584 lines changed on each side,
  every one `dot[constraints]` or `dot[constraints-fold]` (792 programs each); nothing else moved, no read-out line.
- **Other goldens**: `ein-render/tests/golden/dot/render_constraints.dot` (its fixture now loaded, with a fact held
  nowhere in its frame); `ein-cli/tests/golden/help_shape.txt` (the two options, the two `about`s); `06-config.md`'s
  option counts by hand (93 → 95, `ein render kb` 4 → 6), which `config_reference` holds.
- **`ein test examples tests stdlib --json-report`**, before (`9f5c0487`'s release build) and after: the JSON reports
  identical; the text the same but for the module-not-found path of the `std.nope` fixture, which names where each
  binary looked for the standard library (the baseline ran from a copy outside the checkout).
- **Tests run, all green**: `ein-infer` lib (the two slice tests) and `read_out`, `claim_routes`, `claim_round_trip`,
  `claim_semantics`, `expect_semantics`, `alive_invariant`; `ein-ir` and `ein-core` whole; `ein-render`
  `every_object_once`, `dot_wellformed`, `golden_dot`, `presentation_semantics`, `renaming_invariance`,
  `id_order_invariance`; `ein-cli` `render_doors`, `goal_premises`, `help_surface`, `config_reference`,
  `closed_stdout`, `cli_semantics`, `corpus_cli`, `read_out_reference`, `read_out_examples`, `import_facts`,
  `expect_cli`, `test_cli`, `model_set_report`, `embedding`, `regressions`, `program_read`, `saved_files`.
- **Static checks**: `cargo fmt --check`; `cargo clippy -p ein-core -p ein-ir -p ein-infer -p ein-render -p ein-cli
  --all-targets -- -D warnings`; `cargo doc --no-deps` on the same five with `-D warnings`; `doc_audit.py --links
  --check --tracked` (findings only under the review bundles' ignored outputs, which a worktree lacks; decision ids 0);
  `check_doc_blocks.py`; `render_diagrams.py --check` (`07-constraints-view` drawn again from the binary's output on
  its page's program); `check_generated.py`; `check_release_voice.py`; `check_tool_mirrors.py`. Not run, by the
  brief: `./run_tests.sh`, the workspace, `ci_container.sh` (a DOT writer changed, so the container run is part of
  done for the main session), the nightly.
- **The view on the corpus** (a throwaway test over `corpus_files()`, not committed): 676 programs load and saturate
  by `ein_ir::load`; the view keeps 222 888 facts on 656 of them; `z` selects 83 503 facts on 243 programs, `eq`
  26 819 on 268; of the kernel's other words only `__closed__`, one fact, `(lift complete __closed__)` of
  `examples/ein-bugs/wrong-answers/dunder-object-as-value.ein`.

### Readings taken, each with its default

- *A relation of the program*, for the slice, is the whole list a goal and a claim are held to, `holdable`: the
  kernel's words the program writes and an activator's relations included, so Q-M1f.524's *one notion with the goal's
  and the claim's* is one list, kept on the program. Alternative: the registry with `eq` and `neq`, which would drop
  the one `__closed__` fact above.
- The decisions: files for Q-M1f.523, .527, .529, .530, .531 and .532; .524 to .526 are parts of .523's definition and
  .528 of .527's drawing, each carried by that file and cited by no file of the project.
- `ein render constraints` of a saved file draws its program's root, its `(commit …)` not committed (*on root's
  saturated knowledge base*); a root that does not saturate is refused with `ein saturate`'s line, exit 1.
- `--goal` is parsed as `(query :goal GOAL)` under the source name `--goal`, so a position counts the wrapper's 13
  columns; a refusal is the loader's line after `kb load error: `, a parse error the parser's; a goal the compiler
  refuses is its line, exit 1; nothing is drawn on any refusal.
- A goal's slice is the digraph `goal`; the constraint view keeps `constraints`; an empty slice says `// the slice
  keeps no fact`.
- `--query N` counts from 1, as `query i of n`; a query the program has not is exit 1 with one line; `--query 0`,
  and `--goal` beside `--query`, are clap's usage errors, exit 2.
- The facts that answer a goal are its matches' premises: a stored negation a `(not …)` pattern matches is kept,
  and a check keeps none.
- *Its `as :expect` parses back* (R-39) needed one more thing: a claim listing a fact of a kernel word a goal reads,
  `(relation next-to)`, was refused at load (`:expect names relation, and the program names no relation relation`),
  for any goal over `(relation ?r)`, the baseline too; the claim's relation check now takes the kernel's words as the
  goal's walk does, which Q-M1f.119 already states (*the declarations … are every program's*). No corpus program met
  that refusal before.
- `import_facts.rs` now runs `render constraints` among the commands that load (it loads now); `dot_wellformed`
  skips the two constraint views where root does not saturate.
