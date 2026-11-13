# S1f.6.30 — Name clashes between imported theories: investigation, proposals, and a stop

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. **Stopped for the user at T5, 2026-11-13:**
T1 to T5 done ([the record](#record-2026-11-13)); the proposals and the questions are
[`import_clashes.md`](import_clashes.md), Q-M1f.231 to .236. **Ruled 2026-11-13 and closed:** the user chose a
model of their own, (h), full names and short names, in conversation that day ([`import_clashes.md` § 7](import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names)); Q-M1f.231 to
.236 ruled, .237 to .240 defaulted; T6, the fix, is the stage of its own
[S1f.6.35](s1f.6.35_imports_by_full_name.md), after S1f.6.34 and before S1f.6.31. **An investigation that stops
for the user** before any change to the grammar or to what an import means; its first task, the tagged calls
as an interface, changes neither and is built.
**Depends on:** nothing in the phase. It is the last of the stages independent of the stdlib layout, and its
proposals are read before [S1f.6.31](s1f.6.31_the_stdlib_layout.md) moves the modules, since a proposal may
bear on how the moved modules name their internals.

## The ruling

[Question 8](questions.md#q8) asked whether to rename the theory's internal relations with a common prefix:
(a) no rename, a clash refused with the name in the message and `:as` for a program that wants one of the
names; or (b) a prefix on all of them, sixty relations renamed. The user took neither, 2026-11-12:

> usage of imported theories should be convenient, and allow mixing and connecting theories, if they have
> common things (like digits.ein) they are already connected, potential names clash is a gap of import
> mechanism, plan a stage for investigation, proposals and fix

So the clash is **the import mechanism's gap,** not the theory's to dodge by spelling. This stage is the
investigation and the proposals; the fix follows the user's choice among them.

**The tagged calls.** [Question 41](questions.md#q41), *are `std.arith`'s tagged calls a public interface*,
got *"I don't understand the question"*. [Round 2, item 41](questions_round2.md#q41) explains it and takes
**(a), yes, they are an interface,** meanwhile: *documented as what a theory built on the calculator calls,
and held by a test so they cannot change shape unnoticed*, and *the stage on import name clashes (your 8)
takes their names into account.* **The user's reply there, committed as `dc88520b`: *"(a)"*.**

## What is known

From [`malformed_numbers.md` § 6](malformed_numbers.md#6-the-names-the-theory-occupies-what-a-clash-does-today)
(S1f.6.12's T2, 2026-11-11), on the shipped theory:

- a second *declaration* of any name of the theory is refused at load, with the name in the message;
- a written *fact* of one of the theory's relations is not, and `(done (sum 2 2) 5)` is an answer nobody
  computed ([reply 7](questions.md#q7); the false-facts question is P1g.10's);
- `z` and the digits are names and are shared harmlessly;
- a qualified import prefixes the relations and not `z`.

From the phase's [`findings.md`](findings.md), the import mechanism's other rough places, each a neighbour:

- **E-8:** a qualified import brings the module's `(query …)`, `(solve …)` and `(config …)`
  ([S1f.6.25](s1f.6.25_the_section_j_fixes.md) fixes it);
- **E-20:** a flat import of a theory built on a theory leaves the lower theory's rules behind in silence
  (S1f.6.25 adds a line);
- **E-32:** a file-relative import cannot reach a sibling directory, so `examples/binary_logic/bits.ein` is
  a copy;
- and from S1f.6.11: a qualified import renames a fact's relation too, so a library of facts imported `:as`
  lands in `o.is-a` and does not join the program's.

The circuit's own relations have plain names, `done`, `b`, `sum`, `order`, `short` among them
(`questions.md` 8), and the tagged calls are `add-walk`, `mul-call`, `div-call`, `cmp-walk`, answered by
`done`, `div-done`, `order-is` (`questions.md` 41).

## Tasks

1. **T1f.6.30.1 — the tagged calls, an interface (built; no stop).** `std.arith`'s header and
   [`stdlib/README.md`](../../../stdlib/README.md) name the seven relations as what a theory built on the
   calculator calls, each with its shape and what answers it, apart from the circuit's own relations. A test
   in `arith_theory.rs` holds their declared shapes, and holds that every module of the stdlib and every
   example that calls one writes it in that shape, so a change of shape fails per commit. `std.int`'s and
   `std.rat`'s tagged calls (`int-norm`, `int-add-call`, `int-mul-call`, `int-cmp-call`, and `std.rat`'s)
   are listed the same way. Q-M1f.189 is marked answered, with the user's reply.
2. **T1f.6.30.2 — what mixing and connecting two theories does today, by probes on the release binary.**
   Each a real program with its output, in one table:
   - two theories imported flat that both import `std.digits`: the tables arrive once (S1f.6.11's diamond),
     the program's facts connect them; this is the user's *"already connected"*, shown;
   - two theories imported flat that each declare a relation of one name (`done`): the refusal, as worded;
   - the same with one of them `:as`: what is prefixed (its relations, its internal ones too), what is not
     (`z`, the digits, the names inside facts), **whether the shared tables arrive twice,** once as `fa`
     and once as `a.fa`, and whether the two theories still connect through them;
   - a program's own relation named as an internal one of a theory it imports flat, and as a public one;
   - a theory imported twice, flat and `:as`;
   - the calculator with `std.int` and `std.rat` and the binary examples in one program, flat: every clash
     named;
   - a program that calls a qualified theory's tagged call from a rule of its own.
3. **T1f.6.30.3 — the census:** over every stdlib module and every example theory, the relations it
   declares, which a program is meant to write (requests, `let`, the tagged calls) and which are the
   circuit's own; and every pair of modules that cannot be imported flat together today, with the name that
   stops them.
4. **T1f.6.30.4 — the proposals, each on real computed examples.** The user's rule for a syntax or design
   change is real programs first, so each proposal is shown as the probes of T2 rewritten under it, with
   what each would print, computed on a prototype where the proposal can be prototyped outside the engine
   (by rewriting module text) and marked *not computed* where it cannot. The stage does not pre-judge the
   list; what it must cover:
   - **what a module exports:** whether a module can say which of its relations a program sees, so that
     `done` of one theory and `done` of another never meet. What the unseen ones are called in a state, a
     trace and a core is part of the proposal, since every surface prints them;
   - **what `:as` shares:** whether a qualified import of a theory should share the modules that theory
     imports (the tables) with the importer's other imports;
   - **renaming on import** of a chosen name;
   - **a convention only** (a prefix by module, option (b) of question 8), priced: sixty relations renamed,
     every state, trace and golden of the calculator moved;
   - **nothing,** with the page saying how to mix theories today.
   Each with: what it changes in `ein.y` or `ein_ir::imports`, which corpus answers and goldens move, and
   what it does to E-8, E-20 and E-32.
5. **T1f.6.30.5 — the page and the stop.** `import_clashes.md` in this directory: the probes, the census,
   the proposals with their examples, a recommendation, and the questions, filed under the next free ids.
   **The stage stops here.** Nothing in the grammar or in what an import brings changes before the user
   rules.
6. **T1f.6.30.6 — the fix, as ruled:** planned after the stop, as tasks of this file or a stage of its own,
   with register rows first where today's behaviour is ruled a wrong answer.

## Acceptance

- The tagged calls are named on the module's page and held by a test.
- Every probe of T2 has its program and its output on the page.
- Each proposal is shown on the same programs, with what would move.
- The user has ruled, and T6 is planned to the ruling; or the page says the user chose to leave it.

## What it must not do

- **Not change `ein.l`, `ein.y` or `ein_ir::imports` before the stop.**
- **Not rename a relation of a shipped module** before the stop; and the layout move renames modules, not
  relations.
- **Not reserve names in the loader** ([reply 7](questions.md#q7), (a): not in this phase).
- **Not fix E-32** (a sibling directory) as a side effect; it is named in the proposals and ruled with them.

## The standing rules

A wrong answer of the engine met by a probe is a row in `corpus/wrong_answers.toml`, filed `open` before
its fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Syntax and design changes show real
computed examples first. Config never changes an answer; exit non-zero only on error. The gate is
`./run_tests.sh` for T1's commit, then `./run_tests.sh --nightly`; `utils/ci_container.sh` is not needed.

## Record, 2026-11-13

The release binary every probe runs is `ein.rs/target/release/ein`, built at `2ef68530`, the last commit that
moved engine source (`cargo build --release -p ein-cli` on `53645cb0` reports it current); the scratch is under
`build/s1f.6.30/`, which is not tracked, so every program and output the page relies on is quoted on it.

### T1f.6.30.1 — the tagged calls, an interface, done

**What is promised.** `std.arith`'s header gains a section, *The tagged calls: what a theory built on this one
calls*, and `stdlib/README.md`'s `std.arith` section a row and a block, each writing the five calls and their
four answers in one shape:

```text
(add-walk TAG X Y C z)    asked: X + Y + C     answered by (done TAG S); C a carry in, 0 or 1
(sub-walk TAG X Y 0 z)    asked: X − Y         answered by (done TAG D) where X is not the smaller,
                                               and by (short TAG) where it is
(cmp-walk TAG X Y eq)     asked: X against Y   answered by (order-is TAG O), O one of lt, eq, gt
(mul-call TAG X Y)        asked: X × Y         answered by (done TAG P)
(div-call TAG X Y)        asked: X ÷ Y         answered by (div-done TAG Q R); flagged where Y is zero
(operand TAG X)           wanted: what X is    answered by (operand-is TAG N); X a number or a let's name
```

(the last row added by T1's second commit, below) with what a caller writes in them (numbers as `n` and `z` build them, never a `let`'s name; a tag of its own,
never one of the theory's, E-39; one call a tag). The header's list of the module's names now has three parts
and an interface: the nine leave *the circuit's own*. `std.int`'s and `std.rat`'s headers and README sections
lay their calls out the same way, `(int-norm TAG P N)`, `(int-add-call TAG P1 N1 P2 N2)`, `(int-mul-call TAG
P1 N1 P2 N2)` answered by `(int-done TAG INTEGER)`, `(int-cmp-call TAG P1 N1 P2 N2)` answered by `std.arith`'s
`(order-is TAG O)`; `(rat-norm TAG P N D)` and `(rat-mul-call TAG P1 N1 D1 P2 N2 D2)` answered by `(rat-done TAG
RATIONAL)`, `(rat-cross TAG P1 N1 D1 P2 N2 D2)` answered by `(rat-cross-is TAG POSITIVE NEGATIVE)`. No rule
moved: the three files differ in comments alone, and the manifest was written again.

**What holds it.** `ein-infer/tests/arith_theory.rs`'s `the_tagged_calls_keep_their_shapes`, from a table of
nineteen rows, `TAGGED`, each a module, the call or answer as written, and the declaration's types:

- each module declares each of its rows with those types;
- the module's header and `stdlib/README.md` write each row verbatim;
- every `.ein` under `stdlib/`, `examples/` and `tests/` that writes one of the names at a form's head, parses,
  resolves its imports and holds the module's declaration of it (bare, or after a prefix, the declaration's
  types and `:why` equal to the module's), writes every form of it with the row's count of arguments; and where
  it *starts* a walk, a written fact or a rule's conclusion whose `:match` reads no fact of that walk, writes
  the row's names in their places, `z`, `0` or `eq`;
- and it is not vacuous: `stdlib/arith.ein`, `int.ein`, `rat.ein`, `casting_out_nines.ein` and `conv.ein` are
  read, and each of the three walks is started somewhere.

The loader refuses a form of a declared relation with another count of arguments in a program it loads
(S1f.6.23); the test holds the count against the interface by name, in every file, and the start values, which
no loader sees. On this tree it reads 363 forms in 18 files (`cargo test -p ein-infer --test arith_theory
the_tagged_calls -- --nocapture`, 2026-11-13): the eight banked `examples/beyond_arith/` programs, `conv.ein`,
the three modules and six programs of `tests/stdlib/arith/`; the walks are started 40 times (`add-walk`), 16
(`cmp-walk`) and 14 (`sub-walk`). The two other files that name a call, `examples/arith/div_by_zero.ein` and
`tests/stdlib/arith/20_div_by_zero.ein`, name it in a comment.

**What the test found on its first run.** Two things round 2's list did not hold, each a default
([Q-M1f.228](../open_questions.md#q-m1f228--what-the-interface-of-the-tagged-calls-holds-beyond-the-seven-names-round-2-showed),
the user's to revise):

- `std.int` calls `sub-walk` (rules `int-norm-walks` and `int-div-rest`), and `tests/stdlib/arith/08_sub.ein`
  reads `short`: the seven names were the eight examples' and none subtracts by the walk. **Taken:** `sub-walk`
  and `short` are in the interface, nine relations (eleven since the look-up below).
- `examples/binary_arithmetic/conv.ein`'s `to-dec-double` starts an addition with the bit as its carry in,
  `(add-walk (doubling ?b ?rest) ?acc ?acc ?bit z)`, and the test as first written, to round 2's
  `(add-walk TAG X Y 0 z)`, failed on it: *`(add-walk (doubling ?b ?rest) ?acc ?acc ?bit z)` starts a walk with
  `?bit` where `(add-walk TAG X Y 0 z)` writes 0*. **Taken:** the carry is the caller's, `(add-walk TAG X Y C z)`,
  C `0` or `1`; the test holds `z`.

**What T3's census found after.** `std.int` and `std.rat` read two names of `std.arith`'s that the nine did
not hold, `operand` and `operand-is`: they call the operand look-up for every operand of every request, so a
name goes through `std.arith`'s `let`, and extend it, `int-operand` and `rat-operand` answering it for an
integer and a rational. The prototypes of T4 that hide what a module does not export failed sixteen claims
of `tests/stdlib/int/` and `rat/` until the two were exported (below). **Taken, by the same default:** the
look-up is in the interface, `(operand TAG X)` answered by `(operand-is TAG N)`, eleven relations; the
header, the README and `TAGGED` (21 rows) say so, and the test reads 423 forms in the same 18 files
(`cargo test -p ein-infer --test arith_theory the_tagged_calls -- --nocapture`, 2026-11-13).

Q-M1f.189 is marked answered and built. No finding: no engine behaviour is in question.

**Run** (2026-11-13): `python3 utils/stdlib_manifest.py --write`, then the check, *12 modules match*;
`gen_binary.py --check`, `gen_gates.py --check` and `utils/gen_arith_examples.py --check`, each exit 0 (the
binary theory is generated from `stdlib/arith.ein`'s rules, which did not move); `cargo test -p ein-infer --test
arith_theory` 21 passed, `--test int_theory` 11, `--test rat_theory` 11, `--test stdlib_coverage` 2, `--test
stdlib_semantics` 16; `cargo test -p ein-ir`, every binary green, `the_embedded_copy_matches_the_manifest` among
them; `cargo test -p ein-cli --test arith_search_free` 2 passed; `cargo fmt --all --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`,
clean. No corpus answer, golden or printed line moves: the modules moved in comments alone.

### T1f.6.30.2 — what mixing and connecting two theories does today, done

[`import_clashes.md` § 2](import_clashes.md#2-the-probes-t2): twenty-five rows, each a program or two with its output, in
one table; every case of the task's list and four more, P4c to P4f (a written fact of the wrong count, one of
the right count, and the two names the census of T3 shows real programs meet, a rule `add-column` and a
relation `b`), P8 (a core and a trace under a prefix) and P10 (a stack of theories, each `:as`). Programs
under `build/s1f.6.30/p/`, run by `build/s1f.6.30/run.sh` on the release binary at `2ef68530`, each in a
`systemd-run --user --scope` of 2 GB and no swap and killed after 60 s, 2026-11-13; the facts by relation
from `ein saturate`'s state before saturation. The first run of the runner had no cap: P7a declared `price`,
`total` and `fee-of` and left them open, so describing R searched them, and was stopped at 4 GB; every probe
since closes the relations it declares, and every run is capped.

**What it found.** Flat imports connect by name (P1a, P1b, P6): the tables arrive once, and every shipped
module and example theory loads flat beside every other (P6c), each but `std.arith` having names of its own.
The same sharing is the clash (P2, P4): a declaration of a name a flat theory declares is refused with the
first name and no place, and a written fact of the right count joins the theory's relation in silence (P4d).
`:as` is the way out and shares nothing (P3, P5, P10): the tables arrive once per prefix, 3 656 facts where
one calculator is 1 839, and two theories under two prefixes connect only through undeclared names, the
digits. A mistake under `:as` is silence (P3c, P7b, P7c): **E-49**, filed. No wrong answer of the engine.

### T1f.6.30.3 — the census, done

[`import_clashes.md` § 3](import_clashes.md#3-the-census-t3), taken by `build/s1f.6.30/census.py`
(2026-11-13): a reader of Ein's text and a copy of `ein_ir::imports`' flat selection and its check of two
bodies, over the twelve stdlib modules and the fourteen example theories, each imported by every name it
declares itself; **held to the loader on every pair** (`--engine`, a program of the two flat imports under
`ein saturate`: 325 of 325 agree, refused exactly where the census finds a clash and on one of its names).
The scope and the method are a default, [Q-M1f.229](../open_questions.md#q-m1f229--what-the-census-of-imported-names-reads-and-how),
the user's to revise.

- **By role.** `std.arith` declares 74 relations: six a program states, five results and three flags it
  reads, `n`, the eleven of the interface, and forty-eight of the circuit's own; and 116 rules. Question 8's
  *"sixty relations"* is forty-seven, or fifty-eight with the interface. `stdlib/README.md`'s *some sixty
  names* of the circuit's own is corrected to *some forty-five* besides `b` and `arith-rule`.
- **The pairs.** Two of 325 cannot be imported flat together, each by rules of one name and two bodies:
  `std.bijection` and `std.elim` (`domain-elimination`, `typecheck-arg-0`, `typecheck-arg-1`), and
  `std.algebra` and `binary_logic/props_rules.ein` (`complement`, `identity`); and one name is a rule of one
  module and a relation of another, `std.algebra`'s `difference` against `std.arith`'s tag, so a program that
  activates the rule beside the calculator is refused by the count (P11, run). **E-50**, filed. The calculator
  clashes with nothing else shipped.
- **The corpus's names.** 24 programs that do not bring `std.arith` use one of its names, `b` and `n` eight
  each, the rule `add-column` in `cryptarithm.ein` and `send_more_money.ein`; `done` none. 222 use
  `binary_logic.inputs`' `is-a`, and 37 `word_problems.words`' names, `holds` 28 of them.

### T1f.6.30.4 — the proposals, each on real computed examples, done

[`import_clashes.md` § 4](import_clashes.md#4-the-proposals-t4): the five the task lists, (a) exports, (b)
what `:as` shares, (c) a rename on import, (d) a convention, (e) nothing with how to mix theories today; (f),
(a) to (c) as one mechanism; and (g), four ways for E-32. Each with what it changes in `ein.y` and
`ein_ir::imports`, what moves, and what it does to E-8, E-20 and E-32. The prototypes are module text,
written by `build/s1f.6.30/proto.py` and run on the probes by `build/s1f.6.30/proposals.sh` (the release binary
at `2ef68530`, 2026-11-13); what moves is two sweeps of the 133 corpus programs that bring `std.arith`
(`build/s1f.6.30/sweep.py`), and a bound of `corpus_shapes.md5`'s digests, since that test refuses
`$EIN_STDLIB`. How they are written and priced is a default,
[Q-M1f.230](../open_questions.md#q-m1f230--how-the-proposals-are-prototyped-and-priced), the user's to revise.

**What the prototypes computed.**

- **(a)** `std.arith` exporting 26 of its 74 relations and 2 of its 116 rules: P4e and P4f load, P2a and P4a
  are still refused, on `done`, an exported answer; `ein solve` moves on 17 of 133 (14 `refuted by` lines, 3
  claims naming a tag), R on none, `--dump` on 111. The first export list hid `operand` and `operand-is` and
  failed sixteen claims of `tests/stdlib/int/` and `rat/`: the look-up joined the interface (T1's second
  commit).
- **(b)** the tables once: P5 1 978 facts where 3 656; P10 connected, `(r.rat-simple-of (r.rat (i.int (i.n z
  3) z) (i.n z 1)) …)`, 2 021 facts where 3 943. Whole only with (a), since a module only a qualified import
  brings needs a name of its own.
- **(c)** P2a and P4a load; **P4a beside `std.int` is refused**, *conflicting definitions of 'strip-none'*:
  a rename rewrites one copy, and a theory built on the renamed one brings the other.
- **(d1)** P4f loads, P2a, P4a and P4e do not; `ein solve` moves on 3, `--dump` on 111. **(d2)** P2a and P4a
  load; 18 files that write the interface are rewritten.
- **(e)** six lines, each a probe of § 2.

### T1f.6.30.5 — the page and the stop, done

[`import_clashes.md`](import_clashes.md) is whole: § 0 in short, § 1 how an import works today, § 2 the probes,
§ 3 the census, § 4 the proposals, **§ 5 the recommendation**, (f) in two steps, (a) and (b) first, then an
exported name refused only where the program writes it and `:rename` a second name of one copy, and (g1) for
E-32 after; and **§ 6 the questions**, filed in [`open_questions.md`](../open_questions.md) as
[Q-M1f.231](../open_questions.md#q-m1f231--which-way-the-import-mechanism-closes-the-gap) (which way),
[.232](../open_questions.md#q-m1f232--how-a-module-says-what-it-exports-and-what-stdarith-exports) (how a module
says what it exports, and `std.arith`'s list), [.233](../open_questions.md#q-m1f233--how-a-name-a-module-does-not-export-is-printed)
(how a hidden name prints), [.234](../open_questions.md#q-m1f234--two-flat-imports-that-export-one-name) (two
exporters of one name), [.235](../open_questions.md#q-m1f235--a-module-in-a-sibling-directory-e-32) (E-32) and
[.236](../open_questions.md#q-m1f236--e-49-and-e-50) (E-49 and E-50), **open, the user's**, none defaulted. P11
was run under (a), (d1) and (d2) for the page: it loads under each, the algebra test's claim held.

**The stage stops here.** Nothing in `ein.l`, `ein.y` or `ein_ir::imports` moved; no relation of a shipped
module was renamed; no name is reserved in the loader; E-32 is not fixed. What moved in the tree: T1's header
comments and README sections, its test and the manifest; `stdlib/README.md`'s count of the circuit's own names
(T3); and records. T6 waits for the user's ruling; where today's behaviour is ruled a wrong answer, its register
rows come first.

**The stage's commits:** `ffb47f3c` (T1), `63955507` (T1 completed, the operand look-up), `da81ee14` (T2),
`bec8b32e` (T3), `b90a3278` (T4), and this one (T5). **Defaults taken,** each the user's to revise:
Q-M1f.228 (what the interface holds beyond round 2's seven names), Q-M1f.229 (the census's scope and method),
Q-M1f.230 (how the proposals are prototyped and priced). **Findings:** E-49 and E-50, neither a wrong answer.
**No register row:** no probe met a wrong answer of the engine. **Not run here:** the whole gate and the
nightly, which the coordinator runs on the last commit.



**The gate, run by the coordinator** (2026-11-14), once over the batch of code commits since `c2302ad5`
(`2ef68530`, Q-M1f.226 (b); S1f.6.29's `d78513af` to `748d7be5`; S1f.6.30's `ffb47f3c` and `63955507`), in a
detached worktree on `748d7be5`: `./run_tests.sh`, exit 0, 54 min 8 s, 1 155 tests in 136 binaries, its link
check over 557 pages with no finding (`build/gate-batch1/gate.log`). **No nightly:** on the user's word of
2026-11-13 (*"yes do this instead"*), one gate per batch of code commits, and the nightly after a large engine
change, the next being S1f.6.35. `utils/ci_container.sh` was not run: no DOT writer, review bundle script or CI
install changed.
