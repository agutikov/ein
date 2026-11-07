# The parser probe: one grammar, six generators

Evidence for [S1f.5.5](../s1f.5.5_the_grammar.md). Taken 2026-11-07 at `916d97ac`, on the release
profile. **A throwaway, kept so the numbers can be re-taken.** It changes no engine code, and no crate
in `ein.rs/` depends on it. `harness` links `ein-ir`, read-only, for today's parser.

The grammar is a by-position draft under the user's name rule of 2026-11-07
([Q-M1e2.24](../../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)), written once per notation. It
is a working draft for choosing a generator. T1f.5.5.2 writes the real one.

| grammar | file |
|---|---|
| LALRPOP, its own lexer | [`c_lalrpop/src/ein.lalrpop`](c_lalrpop/src/ein.lalrpop) |
| LALRPOP, `logos` lexer | [`c_logos/src/ein.lalrpop`](c_logos/src/ein.lalrpop), [`c_logos/src/lib.rs`](c_logos/src/lib.rs) |
| pest | [`c_pest/src/ein.pest`](c_pest/src/ein.pest) |
| peg | [`c_peg/src/lib.rs`](c_peg/src/lib.rs) |
| grmtools | [`c_grm/src/ein.l`](c_grm/src/ein.l), [`c_grm/src/ein.y`](c_grm/src/ein.y) |
| parol | [`c_parol/ein.par`](c_parol/ein.par) |
| grmtools, the grammar read at run time | [`c_grm_rt/src/main.rs`](c_grm_rt/src/main.rs), reading `c_grm`'s two files |

## Re-taking it

Run from this directory. Keep the target directory outside the source tree (`build/` is gitignored).

```sh
export CARGO_TARGET_DIR=$PWD/../../../../build/pg-probe/target
cargo build --release -p harness
H=$CARGO_TARGET_DIR/release/harness
$H corpus                                # accept, reject and tree, each candidate against the others
$H mutate 40000                          # the same over random byte edits of the corpus
$H errors '(r _ ?x)' '(p "1")' '(p a?x)' # where each one refuses, and what it expected
(cd ../../../..; taskset -c 2 $H speed examples/zebra2.ein)
$H deep lalrpop 1000000                  # hand, pest, peg, lalrpop, logos+lalrpop, grmtools, parol
python3 read_lalrpop.py c_lalrpop/src/ein.lalrpop $H ein_from_lalrpop.ebnf
python3 census.py; python3 census2.py    # what the corpus writes, per token and per form
PG_TARGET=$CARGO_TARGET_DIR/.. ./buildcost.sh $PWD   # a clean build of each, -j 4
cargo build --release -p c_grm_rt; G=$CARGO_TARGET_DIR/release/c_grm_rt
$G corpus                                # grmtools with ein.l and ein.y read at run time, against its compiled build
(cd ../../../..; taskset -c 2 $G time examples/zebra2.ein)   # building the grammar, lexing, parsing
$G recover four_mistakes.ein             # CPCT+: every error of one run, each with its repairs
for i in $(seq 20); do $G recover four_mistakes.ein | grep -v 'recovery run' | md5sum; done | sort | uniq -c   # F11
# F11's patch: lrpar 0.15.0 copied out of the registry, patched, and used through [patch.crates-io]
L=$CARGO_TARGET_DIR/../lrpar-det; rm -rf $L; cp -r ~/.cargo/registry/src/*/lrpar-0.15.0 $L
(cd $L && patch -p0 < $OLDPWD/lrpar_deterministic.patch)
CARGO_TARGET_DIR=$CARGO_TARGET_DIR-det cargo build --release -p c_grm_rt --config "patch.crates-io.lrpar.path=\"$L\""
P=$CARGO_TARGET_DIR-det/release/c_grm_rt
for i in $(seq 200); do $P recover four_mistakes.ein | grep -v 'recovery run' | md5sum; done | sort | uniq -c   # one line
EIN_PROBE_STEPS=1 $P recover four_mistakes.ein   # the steps one parse's recovery took
```

The 5 000 mutants were written by a seeded script (character edits of the corpus, `random.Random(1)`, one to
four edits each) that is not kept; any seeded set re-takes the comparison.

## What it found

**One language.** All six accept the same 352 of the corpus's 390 files and refuse the same 38, and the
five that build the common tree build the same one (parol is compared on accept and reject). They also
agree on all 40 000 mutants, after the mutation run found one bug in my LR lexers: a string glued to the
next token, `:why " ":priority 0`, lexed as two tokens where a space in the string kept the catch-all
from matching. A second bad-token rule closes it.

**What the draft refuses that today's parser accepts: 34 files**, all fixtures or witnesses, under
`examples/broken/load/`, `examples/ein-bugs/`, `examples/syntax/` and `corpus/fuzz_findings/`. None is a
puzzle, a stdlib module or a `tests/` program. Today's parser also refuses 4 of the 38.

**The measurements**, `examples/zebra2.ein` (27 732 bytes), the median of three runs of best-of-seven:

| | hand-written today | LALRPOP | LALRPOP + logos | pest | parol | grmtools | peg |
|---|---|---|---|---|---|---|---|
| grammar format | Rust code: `lex.rs` and `parse.rs` | one `.lalrpop` file: BNF productions with a Rust type and an action per alternative, macros, and a `match` block of literal and regex tokens | the same productions with an `extern` token block. The tokens are `#[token]` and `#[regex]` attributes on a Rust enum | one `.pest` file of PEG rules, no actions. Read by a derive macro, or by `pest_meta` and `pest_vm` at run time | one `.par` file: EBNF, scanner states, no actions. It generates the AST types and a trait to implement | Lex `.l` and Yacc `.y`, Rust actions in the `.y`. Read at build time or at run time | PEG rules inside a Rust macro, `peg::parser!{ }`, with the actions inline |
| parsing strategy | a longest-match lexer, then recursive descent. An overlap goes to the alternative tried first | table-driven LR(1) by the lane-table method (LALR(1) on request). The lexer takes the longest match, ties broken by tier, and is built from the regexes at run time | LR(1) as LALRPOP. `logos` compiles every token into one automaton at build time | PEG: recursive descent that backtracks and takes the first alternative that matches. No separate lexer, no memoising | LL(k): a predictive pushdown automaton, a lookahead automaton per non-terminal, no backtracking. Its LALR(1) mode was not used | table-driven LR by Pager's method, LR(1)'s power in merged tables. A lex-style lexer: longest match, then the earlier rule | PEG: recursive descent that backtracks, no separate lexer. Memoising per rule on request (`#[cache]`) |
| error recovery | none: one `ParseError`, at the first error | none unless the grammar writes `!`; then it drops tokens, records an `ErrorRecovery` and goes on. The draft writes none | as LALRPOP | none: the furthest position, the rules expected there and those not wanted | automatic in LL mode, on by default: token insertion and deletion chosen by edit distance. None in LALR mode. Left on here | automatic, CPCT+, on by default: every error in a run, each with ranked repairs. Turned off here (`RecoveryKind::None`) | none: the furthest position and the literals expected there |
| target use cases, as each describes itself | Ein's language alone | *"usability as its primary goal"*: compact grammars for a language fixed at build time | `logos`: *"Create ridiculously fast Lexers"*, faster than one written by hand | *"general purpose … accessibility, correctness, and performance"*: a grammar kept apart from the code, or loaded at run time | language implementations that want their AST types generated, a top-down parse, Rust and C# from one file | Lex and Yacc grammars brought to Rust, grammars loaded at run time, every syntax error reported with a repair | *"simple yet flexible"*: a parser inside Rust source, over `&str`, `&[u8]` or a slice of tokens |
| parse, tree, intern, line lookup (µs) | 165 | 240, plus 150 once per run | 139 | 1 168 | 855 (its own typed tree only) | 2 756 | 321 |
| an overlapping alternative added | — | **build fails** | not run | **builds in silence** | **build fails** | **build fails** | not run |
| `(p (p (p …` nested 100 000 deep | **aborts** | parses | parses | refuses, with an error | parses | parses | **aborts** |
| nested 1 000 000 deep | **aborts** | parses | parses | refuses | **aborts** | parses | **aborts** |
| crates at run time, added | 0 | 5 | 13 | 15 | 41 | 41 | 6 |
| crates, build time included | 0 | 45 | 52 | 20 | 101 | 51 | 6 |
| clean build, `-j 4`, alone (s) | 0 | 9.9 | 12.4 | 7.3 | 18.5 | 23.7 | 2.2 |
| grammar lines | 1 547 of Rust | 80 | 107 + 59 | 65 | 60 | 44 + 276 | 55, in a Rust macro |
| generated Rust lines | 0 | 13 153 | 11 800 | in a macro | 8 023 | 8 768 | in a macro |
| builds under `#![forbid(unsafe_code)]` | yes | yes | with `logos`' `forbid_unsafe` | yes | yes | yes | yes |

The first four rows are read from each crate's source and README at the version `Cargo.lock` holds, not
measured. Recovery was tried for grmtools alone (below). In the timings every candidate but parol ran with
it off or absent, and parol's, left on, refused the same files as the others.

The hand-written figure is its whole `parse`, with interning and locations. Each generated candidate's
run adds an interner and a line lookup to its tree, so the columns are close but not the same work.
`ein-ir` depends on two crates besides `ein-core` today; every column above is what it would add. The
nesting rows skip dropping the tree, as a real port keeps today's arena, and parol's abort at 1 000 000 may
be its own tree's drop rather than its parse.

**Token-level ambiguity too.** LALRPOP and `logos` each refused to build when two of my token rules could
match the same text, until I ranked them: *ambiguity detected between the terminal …*. LALRPOP's generated
code passes `cargo clippy -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc`, and holds no `HashMap`
and no `unsafe`.

**The grammar comes back out of the file.** LALRPOP has no public reader, since `mod parser` is private.
[`read_lalrpop.py`](read_lalrpop.py), 167 lines, strips the actions, writes
[`ein_from_lalrpop.ebnf`](ein_from_lalrpop.ebnf), and interprets it: 40 productions and 44 token rules,
agreeing with the generated parser on 390 of 390 files. `pest_meta` is a public reader for pest, and
`cfgrammar` for grmtools; neither was run here.

**grmtools' two features, tried on Ein's draft** (2026-11-07, added when the user asked which generator
supports the most). *Grammar at run time:* `c_grm_rt` reads `ein.l` and `ein.y` when it starts, with no
build step, and accepts 352 of 390 corpus files with 0 disagreements against the compiled build. Building
the grammar and its tables takes 1.3 ms per run. *Recovery:* [`four_mistakes.ein`](four_mistakes.ein) is a
valid program with four mistakes written into it: a missing `)`, `a?x`, an unknown `:bogus` and a `_` in a
fact. CPCT+ reports three errors in one run, 0.3 ms, each with ranked repairs (`Delete _`; `Insert :priority,
Delete :bogus`). The missing `)` surfaces at the next error, merged with it, since the next line reads as a
nested fact. Today's `ein solve` reports one error, at `4:2`. *Speed:* of grmtools' 2.1 ms on `zebra2.ein`
without actions, 1.9 ms is `lrlex`'s lexing, so its parser proper is about as fast as LALRPOP's. Its
generated parser passes `cargo clippy -D warnings`.

**F11: the recovery is not deterministic.** Run 20 times on `four_mistakes.ein`, CPCT+ printed two outputs,
14 and 6, which swap its two repairs for `:bogus` (2026-11-07). `lrpar` 0.15's `cpctplus.rs` de-duplicates
repairs through a `HashSet` and sorts them by length alone with `sort_unstable_by`, so equal-length repairs
keep the set's random order, and `parser.rs` stops recovery at a 500 ms wall-clock budget. The user chose
grmtools the same day, and recovery on, so [S1f.5.5](../s1f.5.5_the_grammar.md#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)
patches `lrpar` (below).

**F11's patch.** [`lrpar_deterministic.patch`](lrpar_deterministic.patch), 73 changed lines in `cpctplus.rs`
and `parser.rs` of `lrpar` 0.15.0: duplicates are dropped in the order found and the sort is stable, a count
of search steps over the parse replaces the 500 ms clock, and `EIN_PROBE_STEPS=1` prints the steps a parse's
recovery took, which is the probe's alone. Tried 2026-11-07:

| | unpatched | patched |
|---|---|---|
| `four_mistakes.ein`, 200 runs | two reports, 97 and 103 | one report, 200 times |
| 5 000 seeded mutants, 2 113 refused, three runs | reports differ between runs on 1 032 | on none |

Steps per refused mutant, at a budget of a million: half under 70, 99 % under 18 795, and four spend the
whole budget, the slowest in 9.4 s. At 30 000 steps, 19 are cut, and the slowest of the 22 hardest files plus
two made by hand (`zebra2.ein` with every `)` deleted, `zebra.ein` with every `(` doubled) takes 0.33 s; at
50 000, 0.78 s; at 100 000, 2.0 s. The `)`-less `zebra2.ein` reports 71 errors at 30 000 steps and at a million.
Two builds of `c_grm` generate the same `ein.y.rs`, apart from a build-time stamp in a comment.

**A finding that is not about generators: F10.** `ein solve` on a fact nested 100 000 deep exits 134 with
*has overflowed its stack*, and 10 000 deep exits 0. The corpus's deepest program nests 8.

**Not measured.** `lelwel` and `rustemo` were not tried. `nom`, `winnow` and `chumsky` are parser
combinators, so the grammar would be Rust code and no generator reads it. tree-sitter needs a C runtime.
None of this was run on a loaded program past the parse: macros, the loader and the solver were not wired
to any candidate.
