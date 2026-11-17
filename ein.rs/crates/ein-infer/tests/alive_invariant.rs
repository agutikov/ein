//! **The alive-set invariant, evaluated over the corpus**
//! ([saturation.md](../../../../docs/ein/reasoning/kb-evolution/saturation.md#the-alive-set-invariant)).
//!
//! [`ein_infer::invariant`] is the check; this is what it says, banked. The
//! invariant licenses the per-KB `alive` recompute, the `state_key` dedup
//! that produces `k`, and the tree's exhaustiveness-by-discharge argument, so
//! it is evaluated rather than assumed.
//!
//! # The seed
//!
//! Clause 1, *rules assert no new objects*, read against the names the
//! loaded **facts** mention alone, would break on a constant only a rule
//! names, and on `examples/ein-bugs/alive-set-fresh-name.ein` breaking it
//! would cost a model (`W4`): a blind enumerator that built its objects from
//! the KB's names would put such a constant in no candidate. Such a constant
//! is a name of the program — *rules are part of root, so rules can't assert
//! new names* (Q-M1e.21) — and the universe is seeded with every name of the
//! program (Q-M1e2.9): a constant in a rule's `:match` too, and one a fact
//! names only inside a nested fact (`W15`). So the three readings read:
//!
//! | | claim | corpus |
//! |---|---|---|
//! | **R0** | every name of the program is among the blind enumerator's objects, the ones the names index does not hold included | the names the index does not hold, [`SEEDED`], each a candidate object |
//! | **R1** | nothing a run derives names what the loaded program, its rules' constants included, did not | **0** |
//! | **R2** | nothing the **search** reaches names what **root's fixpoint** and the rules' constants did not | **0** |
//!
//! With the facts' names alone as the baseline, R1 would find `Ann` and `G`
//! and R2 `Ann`. A name R1 or R2 found would be one no rule's text predicts —
//! a relation head written as a parameter and bound to a value the program
//! does not hold, which the loader and the compiler refuse — and the test
//! that sweeps for it is [`no_run_reaches_a_name_the_program_does_not_have`].

use ein_core::{FactId, Kb, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::invariant::{self, Universe};
use ein_infer::region::Nodes;
use ein_ir::{Ast, load_file};

/// `ein solve FILE`, bounded for a sweep: a describe that stops at a small
/// budget, and a flagged program's cube only where it is small, lists the
/// states it met — what this file means by *what a run reaches*.
fn describe_options() -> explore::Options {
    let mut o = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    o.budget = 20_000;
    o.limit = 10;
    o
}

/// Every state the answer met and every owing frontier node, as the facts
/// that reach it.
fn reached(a: &explore::Answer) -> Vec<Vec<FactId>> {
    let Some(d) = a.described.as_deref() else {
        return Vec::new();
    };
    let facts = |set: &[u32]| -> Vec<FactId> {
        set.iter().map(|&b| a.candidates[b as usize].fact).collect()
    };
    d.states
        .iter()
        .map(|s| facts(&s.rep))
        .chain(d.frontier.iter().map(|o| facts(&o.set)))
        .collect()
}

/// **Every name of the program the names index does not hold, corpus-wide**,
/// at load, as `(path, [rendered…])` — what [`invariant::seeded`] reports.
///
/// Each is a name of the program (Q-M1e.21, Q-M1e2.9) and a candidate
/// object: a rule's `:assert` constant (`Ann`, `G`, `Z`), a constant in a
/// rule's `:match` (`match-only`'s `Z`), and one a fact names only inside a
/// nested fact (the rest).
///
/// - `mixed-type-hypothesis.ein` — `Ann`, in an `hrule`'s `:assert`. That
///   query drives the hrule rung, whose candidates already name `Ann`; the
///   seed reaches its blind probe (`EIN_LEFTOVER`) alone.
/// - `07_schroder.ein` — `G`, in the probe rule `probe-undecided`, which fires
///   in **root** saturation, so `G` is a name at root without the seed too;
///   and `C`, `D`, inside its stored negatives.
/// - `wrong-answers/fresh-name-default.ein` — `Z`, `W4` in isolation.
/// - `wrong-answers/match-only.ein` and `negative-only.ein` — `Z`, `W15`:
///   without the seed no candidate names it, and R would be {∅} where the
///   `-declared` controls' four states are.
/// - `wrong-answers/oblig-fresh-name.ein`, `-two.ein` and
///   `oblig-stdlib-total-owed.ein` — `Z`, in `special`'s `:assert`, `W14`:
///   an object of the blind enumerator, which the obligations rung those
///   programs take does not consult.
/// - the three `*-disproved`, `syntax/equality` and `syntax/arg-kinds` — the
///   corpus programs whose H the seed moves (`W15`'s rows; `arg-kinds`' `a`
///   and `b` inside `(r (nested a b) c)`, its solve scoped, R {∅}).
/// - a run of digits is a name: `stdlib/elim.ein`'s `0` and `1`, `:match`
///   constants of `domain-elimination`; `syntax/relation-decls`' `7`, inside
///   its `ontology` fact; and `wrong-answers/range-in-match.ein`'s `1..3`, a
///   `:match` constant.
/// - `ein-bugs/binding-key-{both,swapped,nested-only}.ein` — `Y`, inside an
///   activator's nested `(src Y)`, `W7`'s family: H is `(holds Y)`.
/// - `wrong-answers/or-one-key-twin.ein` — `w`, in `m1`'s and `s`'s
///   `:assert`, `W39`: the witness beside it names a rule `w`, which makes `w`
///   a name of the program there, and the twin, whose rules are `w1` and
///   `w2`, names it in no fact.
/// - `ein-bugs/activator-all-nested.ein` — `a`, inside `(item (box a))`,
///   `W16`: its one candidate is the hrule's `(checked (box a))`, and the
///   name is no candidate's.
/// - `W30`'s `assert-neq-w-x.ein` and `assert-eq-x-w.ein`, `w` in a
///   concluded `neq` or `eq`, a constant of the rule's `:assert` and so a
///   name of the program though the conclusion is checked and never stored,
///   which makes `(p w)` a candidate; and `W31`'s `empty-match-fact.ein` and
///   `empty-match-hrule.ein`, `b` in a conclusion no firing reaches.
/// - `W52`'s `relvar-eq-assert.ein`, `w` in a conclusion whose relation a
///   parameter binds to `eq`, as `W30`'s `assert-eq-x-w.ein`'s; and `W54`'s
///   `goal-nested-var.ein`, `a` and `b` inside its stored negations.
/// - `syntax/query-full.ein`'s `House`, `Water` and `Tea`, constants of the
///   two hrules its `:hrules` names, which the program declares, since the
///   loader refuses a list naming an hrule the program does not declare.
/// - `W94`'s `and-in-and.ein` and `and-in-and-macro.ein`, `a`, a name only a
///   rule's conclusion writes: a constant of the conclusion, read at any
///   depth, however the conclusion's `and` nests.
/// - the programs whose `z` only a rule's conclusion wrote meet it first in
///   the fact the loader states from that conclusion, `(written (n z 1))`;
///   and `z` is `std.math.arith`'s, a declared relation of no argument, a
///   name of every program that loads the calculator, so a `z` is listed
///   only where a fact names it inside a nested fact.
/// - every program of `tests/stdlib/math/rat/` and
///   `tests/stdlib/math/gcd/12_what_stays_silent.ein`, which imports
///   `std.math.int` beside `std.math.gcd`: `pos` and `neg` in
///   `std.math.int`'s division's rules, met before `std.math.rat`'s own `pos`
///   and `neg` in `rat-simplify-pos` and `-neg`.
/// - three programs of `tests/stdlib/math/binary/`, on the pattern of
///   decimal's (a letter or a decimal digit in a numeral, a name only a rule
///   writes); and `stdlib/math/positional.ein` alone, the rules both bases
///   share with every relation a parameter, so no table and no fact of its
///   own: `0` and `1`, a carry, are names only its rules' premises write.
///   `eq`, which its `cmp-start` writes, is a kernel word and never an
///   object, and is not listed.
/// - a rule a program does not list is named by its full name,
///   `std.math.int.int-div-pn`; `elim/03` and `04` hold `std.elim` whole
///   and meet `0` and `1` in its `domain-elimination`.
/// - `beliefs/01` and `02` write a belief's or a claim's content only as
///   another fact's argument, so its names are no top-level argument.
/// - the rest — nested names on programs with no relation the blind rung
///   proposes over, so H stays empty; `22_symmetric_negative`'s `C` is a
///   name at root's fixpoint too, `(undecided B C)`.
///
/// A program the loader refuses loads here no more: the shapes the grammar
/// refuses (a string, a keyword pair inside a term, a bare `_`, `eq` at
/// another arity) and the loader's refusals (a premise that is no form, a
/// relation variable no parameter binds, a relation parameter bound to a
/// fact, an hrule concluding more than one fact).
const SEEDED: [(&str, &[&str]); 128] = [
    (
        "examples/arith/int.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "examples/arith/rat.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "examples/beliefs/01_common_belief.ein",
        &[
            "`Dan`, in (believes Ann (likes Dan Soup))",
            "`Eve`, in (believes Ann (likes Eve Fish))",
            "`Fish`, in (believes Ann (likes Eve Fish))",
            "`Soup`, in (believes Ann (likes Dan Soup))",
            "`Tea`, in (believes Ann (likes Dan Tea))",
        ],
    ),
    (
        "examples/beliefs/02_packing.ein",
        &["`Tea`, in (claims Bob (likes Dan Tea))"],
    ),
    (
        "examples/beyond_arith/equation_refuted.ein",
        &[
            "`e`, asserted by rule `um-start`",
            "`z`, in (unmul 7 (n (n (n z 8) 6) 2))",
        ],
    ),
    (
        "examples/beyond_arith/inverse_addition.ein",
        &[
            "`e`, asserted by rule `un-start`",
            "`z`, in (unadd (n (n (n (n z 4) 5) 6) 7) (n (n (n (n (n (n z 1) 2) 7) 6) 4) 5))",
        ],
    ),
    (
        "examples/beyond_arith/tables_backwards.ein",
        &[
            "`e`, asserted by rule `um-start`",
            "`z`, in (unmul 7 (n (n (n z 8) 6) 1))",
        ],
    ),
    (
        "examples/binary_logic/post.ein",
        &[
            "`affine-class`, asserted by rule `inside-affine`",
            "`monotone-class`, asserted by rule `inside-monotone`",
            "`one-preserving`, asserted by rule `inside-one`",
            "`self-dual-class`, asserted by rule `inside-self-dual`",
            "`zero-preserving`, asserted by rule `inside-zero`",
        ],
    ),
    (
        "examples/ein-bugs/activator-all-nested.ein",
        &["`a`, in (item (box a))"],
    ),
    (
        "examples/ein-bugs/binding-key-both.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/binding-key-nested-only.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/binding-key-swapped.ein",
        &["`Y`, in (note edge (src Y))"],
    ),
    (
        "examples/ein-bugs/mixed-type-hypothesis.ein",
        &["`Ann`, asserted by hrule `guess`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/and-in-and-macro.ein",
        &["`a`, asserted by rule `nested`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/and-in-and.ein",
        &["`a`, asserted by rule `nested`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/assert-eq-x-w.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/assert-neq-w-x.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/dump-selection-library.ein",
        &[
            "`Green`, asserted by rule `palette`",
            "`Red`, asserted by rule `palette`",
        ],
    ),
    (
        "examples/ein-bugs/wrong-answers/dump-selection.ein",
        &[
            "`Green`, asserted by rule `palette`",
            "`Red`, asserted by rule `palette`",
        ],
    ),
    (
        "examples/ein-bugs/wrong-answers/empty-match-fact.ein",
        &["`b`, asserted by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/empty-match-hrule.ein",
        &["`b`, asserted by hrule `h`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/fresh-name-default.ein",
        &["`Z`, asserted by rule `spawn`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/goal-nested-var.ein",
        &["`a`, in (not (p a))", "`b`, in (not (p b))"],
    ),
    (
        "examples/ein-bugs/wrong-answers/match-only.ein",
        &["`Z`, matched by rule `reach-z`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/negative-only.ein",
        &["`Z`, in (not (q Z A))"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-fresh-name.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein",
        &["`Z`, asserted by rule `special`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/or-one-key-twin.ein",
        &["`w`, asserted by rule `m1`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/range-in-match.ein",
        &["`1..3`, matched by rule `r`"],
    ),
    (
        "examples/ein-bugs/wrong-answers/relvar-eq-assert.ein",
        &["`w`, asserted by rule `r`"],
    ),
    (
        "examples/saturation/hypothesis-contradiction/coloc-disproved.ein",
        &[
            "`House-2`, in (hypothesis (co-located Norwegian House-2))",
            "`Norwegian`, in (hypothesis (co-located Norwegian House-2))",
        ],
    ),
    (
        "examples/saturation/hypothesis-contradiction/next-to-disproved.ein",
        &[
            "`X`, in (hypothesis (next-to X Y))",
            "`Y`, in (hypothesis (next-to X Y))",
        ],
    ),
    (
        "examples/saturation/hypothesis-contradiction/right-of-disproved.ein",
        &[
            "`A`, in (hypothesis (right-of A B))",
            "`B`, in (hypothesis (right-of A B))",
        ],
    ),
    (
        "examples/syntax/arg-kinds.ein",
        &["`a`, in (r (nested a b) c)", "`b`, in (r (nested a b) c)"],
    ),
    (
        "examples/syntax/equality.ein",
        &["`House-1`, in (= (color House-1) Red)"],
    ),
    (
        "examples/syntax/negation.ein",
        &["`a`, in (not (r a b))", "`b`, in (not (r a b))"],
    ),
    (
        "examples/syntax/query-full.ein",
        &[
            "`House`, matched by hrule `h1`",
            "`Tea`, asserted by hrule `h2`",
            "`Water`, asserted by hrule `h1`",
        ],
    ),
    (
        "examples/syntax/relation-decls.ein",
        &[
            "`7`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R2`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R3`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R4`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`R`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`T`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`U`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
            "`V`, in (ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
        ],
    ),
    (
        "examples/syntax/wrappers.ein",
        &[
            "`T`, in (ontology (relation r T T) (is-a a T))",
            "`a`, in (ontology (relation r T T) (is-a a T))",
            "`b`, in (facts (r a b))",
            "`c`, in (reasoning (r b c))",
            "`r`, in (ontology (relation r T T) (is-a a T))",
        ],
    ),
    (
        "examples/word_problems/boxes12.ein",
        &["`read-need`, asserted by rule `readings`"],
    ),
    (
        "examples/word_problems/boxes13.ein",
        &["`read-need`, asserted by rule `readings`"],
    ),
    (
        "examples/word_problems/readings.ein",
        &["`read-need`, asserted by rule `readings`"],
    ),
    (
        "stdlib/elim.ein",
        &[
            "`0`, matched by rule `domain-elimination`",
            "`1`, matched by rule `domain-elimination`",
        ],
    ),
    (
        "stdlib/math/int.ein",
        &[
            "`neg`, asserted by rule `int-div-pn`",
            "`pos`, asserted by rule `int-div-pp`",
        ],
    ),
    (
        "stdlib/math/positional.ein",
        &[
            "`0`, matched by rule `add-column-first`",
            "`1`, matched by rule `add-carry-out`",
        ],
    ),
    (
        "stdlib/math/rat.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/algebra/07_schroder.ein",
        &[
            "`C`, in (not (t A C))",
            "`D`, in (not (t D F))",
            "`G`, matched by rule `probe-undecided`",
        ],
    ),
    (
        "tests/stdlib/algebra/22_symmetric_negative.ein",
        &["`C`, in (not (sits-with A C))"],
    ),
    (
        "tests/stdlib/elim/03_typecheck_arg0_violated.ein",
        &[
            "`0`, matched by rule `std.elim.domain-elimination`",
            "`1`, matched by rule `std.elim.domain-elimination`",
        ],
    ),
    (
        "tests/stdlib/elim/04_typecheck_arg1_violated.ein",
        &[
            "`0`, matched by rule `std.elim.domain-elimination`",
            "`1`, matched by rule `std.elim.domain-elimination`",
        ],
    ),
    (
        "tests/stdlib/math/binary/01_well_formed_everywhere.ein",
        &["`high`, asserted by rule `ask-mask`"],
    ),
    (
        "tests/stdlib/math/binary/02_a_name_in_a_bits_place.ein",
        &["`2`, in (bin-add (b2 (b2 z 1) 2) (b2 z 1))"],
    ),
    (
        "tests/stdlib/math/binary/06_what_stays_silent.ein",
        &[
            "`2`, asserted by rule `assemble`",
            "`7`, in (bin-let seven (b2 7 1))",
            "`end`, in (bin-let far (b2 end 1))",
            "`high`, asserted by rule `assemble`",
        ],
    ),
    (
        "tests/stdlib/math/decimal-arith/22_malformed_digit.ein",
        &["`x`, in (add (n (n (n z 1) x) 7) (n z 5))"],
    ),
    (
        "tests/stdlib/math/decimal-arith/31_malformed_through_a_name.ein",
        &[
            "`x`, in (let broken (n (n z x) 1))",
            "`y`, in (let unused (n z y))",
        ],
    ),
    (
        "tests/stdlib/math/decimal-arith/38_what_stays_silent.ein",
        &["`end`, in (add (n end 1) (n z 5))"],
    ),
    (
        "tests/stdlib/math/decimal/01_well_formed_everywhere.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pear`, asserted by rule `ask-price`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein",
        &["`x`, in (let price (n (n z 1) x))"],
    ),
    (
        "tests/stdlib/math/decimal/05_what_stays_silent.ein",
        &[
            "`end`, in (let far (n end 1))",
            "`pear`, asserted by rule `assemble`",
            "`x`, asserted by rule `assemble`",
        ],
    ),
    (
        "tests/stdlib/math/gcd/12_what_stays_silent.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/01_simplify.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/02_neg.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/03_add.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/04_sub.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/05_mul.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/06_cmp.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/07_unsimplified.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/08_named_and_chained.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/09_div_euclid.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/10_div_trunc.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/11_div_floor.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/12_div_euclid_by_zero.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/13_div_trunc_by_zero.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/14_div_floor_by_zero.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/15_letter_in_a_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (int-add (int (n z x) z) (int (n z 1) z))",
        ],
    ),
    (
        "tests/stdlib/math/int/16_letter_in_a_negative_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (int-add (int z (n (n z x) 1)) (int (n z 1) z))",
        ],
    ),
    (
        "tests/stdlib/math/int/17_letter_in_the_second_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (int-mul (int (n z 1) z) (int (n z x) z))",
        ],
    ),
    (
        "tests/stdlib/math/int/18_part_ends_in_a_digit.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/19_part_ends_in_a_named_constant.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/20_undeclared_first_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/21_undeclared_second_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/22_digit_as_an_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/23_digit_as_a_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/24_letter_through_a_name.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (let a (int (n z x) z))",
        ],
    ),
    (
        "tests/stdlib/math/int/25_let_twice.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/26_let_zero.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/27_let_digit.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/28_natural_where_an_integer_goes.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/29_named_constant_as_a_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/30_letter_in_a_division.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (int-div-euclid (int (n z x) z) (int (n z 1) z))",
        ],
    ),
    (
        "tests/stdlib/math/int/31_digit_as_the_negative_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/32_digit_zero_as_a_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/33_digit_zero_as_the_negative_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/34_named_constant_as_the_negative_part.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/35_undeclared_divisor.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/int/36_undeclared_only_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/01_simplify.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/02_neg.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/03_add.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/04_sub.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/05_mul.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/06_div.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/07_cmp.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/08_unsimplified.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/09_named_and_chained.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/10_div_by_zero_rational.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/11_zero_denominator_first.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/12_zero_denominator_second.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/13_zero_denominator_one_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/14_letter_in_the_numerator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (rat-add (rat (int (n z x) z) (n z 2)) (rat (int (n z 1) z) (n z 2)))",
        ],
    ),
    (
        "tests/stdlib/math/rat/15_letter_in_the_denominator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (rat-add (rat (int (n z 1) z) (n z x)) (rat (int (n z 1) z) (n z 2)))",
        ],
    ),
    (
        "tests/stdlib/math/rat/16_denominator_ends_in_a_digit.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/17_undeclared_first_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/18_undeclared_second_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/19_digit_as_an_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/20_letter_through_a_name.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
            "`x`, in (let a (rat (int (n z x) z) (n z 2)))",
        ],
    ),
    (
        "tests/stdlib/math/rat/21_let_twice.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/22_integer_where_a_rational_goes.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/23_natural_where_a_rational_goes.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/24_digit_as_the_numerator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/25_digit_zero_as_the_numerator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/26_named_constant_as_the_numerator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/27_digit_as_the_denominator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/28_digit_zero_as_the_denominator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/29_named_constant_as_the_denominator.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/30_undeclared_only_operand.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
    (
        "tests/stdlib/math/rat/31_equal_values_two_facts.ein",
        &[
            "`neg`, asserted by rule `std.math.int.int-div-pn`",
            "`pos`, asserted by rule `std.math.int.int-div-pp`",
        ],
    ),
];

/// The pair in `SEEDED`'s shape and not in `SEEDED`:
/// it is checked by [`the_pair_answers_one_region`] instead, where the
/// answers are the claim.
const PAIR: [&str; 2] = [
    "examples/ein-bugs/alive-set-fresh-name.ein",
    "examples/ein-bugs/alive-set-fresh-name-declared.ein",
];

/// **R0 — every name of the program is a candidate object, over every corpus
/// program.**
///
/// Free: it reads the loaded facts at every depth and the registered rules'
/// `:match` and `:assert` constants once, at load, and never runs the search.
/// That is what makes it a statement about the *program*: a rule that would
/// name a constant only on an input nobody has written is seeded all the
/// same. What it lists is what the list adds over the names index; every name
/// it lists is an object here, so every one is held to the candidates.
#[test]
fn every_name_of_the_program_is_a_candidate_object() {
    let mut found: Vec<(String, String)> = Vec::new();
    let mut programs = 0usize;
    for path in &corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        programs += 1;
        if PAIR.contains(&rel.as_str()) {
            continue;
        }
        let objects = ein_infer::candidate_objects(&kb, &terms, &ast);
        // The kernel's own words are never an object, whatever a program
        // writes them as: `candidate_objects` leaves them out by name.
        // `std.math.positional` writes `eq`, the comparison's start, in a
        // rule's conclusion and in no fact. The
        // kernel's two dunders are left out the same way: a name of the
        // program where one is written as a value, and no object.
        let k = &terms.kernel;
        let kernel = [
            k.not,
            k.r#false,
            k.and,
            k.or,
            k.absent,
            k.eq,
            k.neq,
            k.closed,
            k.symmetric,
        ];
        for c in invariant::seeded(&kb, &terms, &ast) {
            if kernel.contains(&c.name) {
                continue;
            }
            assert!(
                objects.contains(&c.name),
                "{rel}: {} is a name of the program and not among the blind \
                 enumerator's objects {:?} — the seed is gone",
                c.render(&terms),
                objects.iter().map(|&o| terms.sym(o)).collect::<Vec<_>>()
            );
            found.push((rel.clone(), c.render(&terms)));
        }
    }
    assert!(
        programs >= 160,
        "only {programs} corpus files loaded — the sweep stopped looking"
    );
    found.sort();
    let mut want: Vec<(String, String)> = SEEDED
        .iter()
        .flat_map(|(p, names)| names.iter().map(|n| (p.to_string(), n.to_string())))
        .collect();
    want.sort();
    assert_eq!(
        found, want,
        "the corpus programs with a name the names index does not hold moved. \
         A new one is a program the seed reaches; a missing one means a fact now \
         names it at top level, and this list has to say so."
    );
}

/// **The breaches open register rows hold**, each named rather than failed
/// and held both ways, as the row's own wrong answer. Empty: `W34`'s, a
/// synthetic name an `:assert ()` would store, is fixed — `:assert ()`
/// asserts nothing, and no run reaches a name the program does not have.
const KNOWN_R1: [(&str, &str); 0] = [];

/// **R1 and R2 — no run reaches a name the program does not have.**
///
/// The dynamic half exists to confirm what the static half assumes: a
/// *variable* leaf can only carry a value the KB already holds, so nothing but
/// a rule's constant can introduce a name — and a rule's constant is a name of
/// the program. Measured over every state a bounded describe meets on the
/// corpus, the scan finds **nothing**, against either baseline, but for the
/// breaches open register rows hold ([`KNOWN_R1`], none). Baselines that
/// left the rules' constants out would find `Ann` and `G` under R1, `Ann`
/// under R2.
#[test]
fn no_run_reaches_a_name_the_program_does_not_have() {
    let mut r1: Vec<(String, String)> = Vec::new();
    let mut r2: Vec<(String, String)> = Vec::new();
    let mut solved = 0usize;
    // On the corpus sweeps' pool: a program a unit, its rows
    // named by the program, so no unit's rows meet another's.
    let files = corpus_files();
    let found = ein_corpus::pool::map(
        "alive_invariant::no_run_reaches",
        &files,
        |p| {
            (
                p.strip_prefix(repo_root())
                    .unwrap_or(p)
                    .display()
                    .to_string(),
                String::new(),
            )
        },
        |path| {
            let mut r1: Vec<(String, String)> = Vec::new();
            let mut r2: Vec<(String, String)> = Vec::new();
            let mut solved = 0usize;
            'unit: {
                let rel = path
                    .strip_prefix(repo_root())
                    .unwrap_or(path)
                    .display()
                    .to_string();
                let mut ast = Ast::new();
                let mut terms = Terms::new();
                let Ok(mut kb) = load_file(&mut ast, &mut terms, path) else {
                    break 'unit;
                };
                let loaded = Universe::of(&kb, &terms, &ast);
                let at_load = kb.n_facts();
                // The route saturates root in place, once, and writes nothing else
                // into it: what root holds after the run is root's fixpoint.
                let Ok(answer) = explore::run(&mut kb, &mut terms, &ast, &describe_options())
                else {
                    break 'unit;
                };
                let at_root = kb.n_facts();
                let root_universe = Universe::of(&kb, &terms, &ast);

                let collect = |u: &Universe,
                               k: &ein_core::Kb,
                               terms: &Terms,
                               from: usize,
                               into: &mut Vec<(String, String)>| {
                    let mut b = Vec::new();
                    u.breaches(k, terms, from, &mut b);
                    for x in b {
                        let row = (rel.clone(), x.render(terms));
                        if !into.contains(&row) {
                            into.push(row);
                        }
                    }
                };
                collect(&loaded, &kb, &terms, at_load, &mut r1);

                solved += 1;
                let sets = reached(&answer);
                let Ok(mut nodes) = Nodes::new(&mut kb, &mut terms, &ast, &answer.candidates)
                else {
                    break 'unit;
                };
                nodes
                    .with_fixpoints(&sets, |kbs: &mut [Kb], terms: &mut Terms| {
                        for k in kbs.iter() {
                            collect(&loaded, k, terms, at_load, &mut r1);
                            collect(&root_universe, k, terms, at_root, &mut r2);
                        }
                    })
                    .unwrap_or_else(|e| panic!("{rel}: {e}"));
            }
            (r1, r2, solved)
        },
    );
    for (a, b, n) in found {
        r1.extend(a);
        r2.extend(b);
        solved += n;
    }
    assert!(
        solved >= 155,
        "only {solved} corpus files reached a solve — the sweep stopped looking"
    );
    r1.sort();
    let mut known: Vec<(String, String)> = KNOWN_R1
        .iter()
        .map(|(p, b)| (p.to_string(), b.to_string()))
        .collect();
    known.sort();
    assert_eq!(
        r1, known,
        "R1 — a run derived a name that is neither a loaded fact's nor a rule's \
         constant: a value no rule's text predicts, which the seed cannot hold \
         (KNOWN_R1 names the breaches open register rows hold)"
    );
    assert_eq!(
        r2,
        Vec::<(String, String)>::new(),
        "R2 — the search reached a name root's fixpoint and the rules' constants \
         do not hold, so the candidates taken at root cannot be about it"
    );
}

/// **The pair answers one region.**
///
/// The two files differ by the single fact `(seen Z)`, over a relation nothing
/// else mentions. It changes no rule, no constraint and no goal: all it does
/// is put `Z` in a fact.
///
/// Without the seed that one fact would be the difference between two
/// answers: the twin's R `{∅}`, a rule's constant in no candidate (`W4`), and
/// the control's maximal state `{(q A Z), (q B Z)}`. `Z` is a name of both
/// programs, the fact names it in one of them, and the two describe the same
/// R: four states, that one the maximal.
#[test]
fn the_pair_answers_one_region() {
    let run = |rel: &str, which: Which| {
        let path = repo_root().join(rel);
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb =
            load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        let seeded: Vec<String> = invariant::seeded(&kb, &terms, &ast)
            .iter()
            .map(|c| c.render(&terms))
            .collect();
        let a = explore::run(
            &mut kb,
            &mut terms,
            &ast,
            &explore::Options::new(Mode {
                which,
                ask: Ask::Describe,
            }),
        )
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(a.exhausted, "{rel}: the answer is not known whole");
        let mut states: Vec<Vec<String>> = a
            .states
            .iter()
            .map(|s| {
                let mut v: Vec<String> = s
                    .closure
                    .iter()
                    .map(|&b| a.candidates[b as usize].text.clone())
                    .filter(|t| t.starts_with("(q "))
                    .collect();
                v.sort();
                v
            })
            .collect();
        states.sort();
        (seeded, states)
    };

    let (twin_seeded, twin_line) = run(PAIR[0], Which::Maximal);
    let (control_seeded, control_line) = run(PAIR[1], Which::Maximal);

    // The same rule, the same constant, and only the twin's needs the seed.
    assert_eq!(
        twin_seeded,
        vec!["`Z`, asserted by rule `spawn`".to_string()],
        "the fixture stopped naming Z in its rule alone"
    );
    assert!(
        control_seeded.is_empty(),
        "the control names Z in `(seen Z)`, so its rule's constant is a fact's \
         name too and nothing is seeded. It reported {control_seeded:?}"
    );

    let maximal = vec![vec!["(q A Z)".to_string(), "(q B Z)".to_string()]];
    assert_eq!(
        twin_line, maximal,
        "the twin's maximal line is the state naming Z: the seed is what puts \
         its two candidates in H"
    );
    assert_eq!(control_line, maximal, "the control's maximal state moved");

    // All of R, not only the line: one fact names Z, and nothing else differs.
    let (_, twin_all) = run(PAIR[0], Which::All);
    let (_, control_all) = run(PAIR[1], Which::All);
    assert_eq!(
        twin_all.len(),
        4,
        "the twin's R: ∅, the two singletons, both"
    );
    assert_eq!(twin_all, control_all, "the pair describes two regions");
}
