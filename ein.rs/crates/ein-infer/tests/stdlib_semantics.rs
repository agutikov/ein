//! The stdlib as **semantics** — S1a.10.2 T1a.10.2.2.
//!
//! `stdlib/*.ein` is shared source: both engines load the same text, so a rule
//! there is not "ein.rs behaviour" that a port could quietly drop — it is the
//! language's own library, and it outlives the implementation that used to be
//! tested against it. What was only ever asserted in Python is what those
//! rules *mean*: that `(bijective R)` fans out into the operational
//! activators, that elimination forces a survivor and stays silent when there
//! are two, that a typecheck fires on the mistyped fact and on nothing else,
//! and that the kernel's `__symmetric__` fast path computes the same closure
//! as the userspace rule it replaces.
//!
//! Replaces, in whole or in part:
//!
//! | Python | subject |
//! |---|---|
//! | `ein.py/tests/inference/test_stdlib_bijection.py` | `std.bijection` — the signature-driven, is-a-free bijection family |
//! | `ein.py/tests/inference/test_stdlib_domain_elim.py` | `std.elim` — the positional variant of the same family |
//! | `ein.py/tests/inference/test_symmetric_native.py` | `__symmetric__` vs `std.algebra`'s `symmetric` |
//! | `ein.py/tests/inference/test_symmetric_hypothesis.py` | a branched-on symmetric relation counts models once |
//! | `ein.py/tests/inference/test_reflective_rule.py` | a rule that derives its own activator still terminates |
//! | `ein.py/tests/inference/test_rule_library.py` | `sibling-exclusive` with both parameters bound to one relation |
//!
//! Everything here asserts *derived facts* and *which rule derived them*.
//! Neither is an internal: a firing's rule name is in `--events`, its derived
//! facts are in the model, and both are what the human-readable trace is built
//! from. Nothing below reaches for a matcher register or a compiled plan.
//!
//! The two solve tests take the same fixtures as `search_invariants.rs` but
//! ask a different question of them: that file pins what the answer does *not*
//! depend on, this one pins what the answer **is**.

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::events::sexpr;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_infer::{Events, SaturateError, Saturator, Session, SharedMemo};
use ein_ir::{Ast, load_file, parse};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

// ── running a program to its fixpoint ──────────────────────────────

/// What one saturation of a fixture produced, in the vocabulary the trace
/// uses: rule names and s-expressions.
struct Sat {
    /// `(rule, derived)` for every **productive** firing — the ein.py
    /// `not f.redundant` filter every one of these tests was written against.
    /// A firing that re-derives a fact the KB already holds is not a
    /// conclusion, and counting it would make "fires exactly once" meaningless.
    productive: Vec<(String, String)>,
    /// Every firing's rule, redundant ones included.
    all_rules: Vec<String>,
}

impl Sat {
    /// Did any productive firing derive this fact?
    fn derived(&self, fact: &str) -> bool {
        self.productive.iter().any(|(_, d)| d == fact)
    }

    /// Productive firings of one rule — ein.py's `_fired`.
    fn fired(&self, rule: &str) -> usize {
        self.productive.iter().filter(|(r, _)| r == rule).count()
    }

    /// Every derived fact whose head is `rel`, sorted and deduplicated.
    fn derived_of(&self, rel: &str) -> Vec<String> {
        let head = format!("({rel} ");
        let mut out: Vec<String> = self
            .productive
            .iter()
            .filter(|(_, d)| d.starts_with(&head))
            .map(|(_, d)| d.clone())
            .collect();
        out.sort();
        out.dedup();
        out
    }
}

/// Load `src` — stdlib imports resolve through the usual `$EIN_STDLIB` →
/// checkout → embedded chain — and saturate it under a step budget.
///
/// The budget is the runaway guard, not the subject: every fixture here
/// reaches its fixpoint in tens of steps, so a program that needs 20 000 has
/// stopped converging, and the error names the candidate it was looping on.
fn try_saturate(src: &str, max_steps: usize) -> Result<Sat, SaturateError> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<stdlib-fixture>")).expect("the fixture parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    let mut events = Events::off();
    // Rendered after the session ends: `saturate` holds `terms` mutably for
    // the duration, so the callback can only keep ids.
    let mut raw: Vec<(Symbol, Vec<FactId>, bool)> = Vec::new();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s)?;
        sat.saturate(&mut s, Some(max_steps), &mut |f| {
            raw.push((f.rule, f.derived.to_vec(), f.redundant))
        })?;
    }
    let mut out = Sat {
        productive: Vec::new(),
        all_rules: Vec::new(),
    };
    for (rule, derived, redundant) in raw {
        let rule = terms.sym(rule).to_string();
        if !redundant {
            for id in derived {
                out.productive.push((rule.clone(), sexpr(&terms, id)));
            }
        }
        out.all_rules.push(rule);
    }
    Ok(out)
}

fn saturate(src: &str) -> Sat {
    try_saturate(src, 20_000).unwrap_or_else(|e| panic!("the fixture must reach a fixpoint: {e}"))
}

/// The `knows` extension after saturating `src`, as sorted s-expressions.
///
/// Rendered rather than compared by `FactId`, because the two runs of the
/// parity test intern into two different arenas: an id comparison there would
/// report a difference where there is only a different interning order.
fn knows_extension(src: &str) -> Vec<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<symmetric>")).expect("parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, Some(20_000), &mut |_| {})
            .expect("saturates");
    }
    let rel = terms.syms.get("knows").expect("declared");
    let mut out: Vec<String> = kb.facts_of(rel).map(|f| sexpr(&terms, f)).collect();
    out.sort();
    out.dedup();
    out
}

// ── the preambles the Python fixtures shared ───────────────────────

/// Only the entry rules are listed; `:symbols` auto-closure pulls the rest of
/// the family in behind them.
const BIJECTION_SETUP: &str = "(import std.macro :symbols (forall))\n\
     (import std.algebra :symbols (bijective-properties))\n\
     (import std.bijection :symbols (bijective-setup typecheck-setup))\n";

const BIJECTION_ELIM: &str = "(import std.macro :symbols (forall))\n\
     (import std.bijection :symbols (domain-elimination range-elimination))\n";

const BIJECTION_NEG: &str =
    "(import std.bijection :symbols (functional-negative injective-negative))\n";

const BIJECTION_TC: &str = "(import std.bijection :symbols (typecheck-arg-0 typecheck-arg-1))\n";

/// `std.elim` is the *positional* formulation of the same family — the arg
/// types are spelled out in the activator instead of read off the
/// `(relation R A B)` signature — and it ships its own copy of every rule.
/// That is why the elim tests below are not merged into the bijection ones:
/// two modules hold two definitions, and either can rot without the other
/// noticing.
const ELIM: &str = "(import std.macro :symbols (forall))\n\
     (import std.elim :symbols (typecheck-arg-0 typecheck-arg-1\n\
                                domain-elimination no-room-left))\n";

// ── std.bijection ──────────────────────────────────────────────────

/// **`bijective-setup-fans-out-operational-activators`.**
///
/// `(bijective R)` is a *declaration*, not an instruction: nothing in it names
/// the type-membership relation the elimination rules have to scan. The two
/// hierarchy knobs supply that, and the fan-out is what turns the declaration
/// into rules that can run — so the interesting part is not that activators
/// appear but that they appear at **two different arities**. The 1-arg markers
/// (`(functional color-of)`, std.algebra) are properties the elimination rules
/// guard on; the 2-arg ones (`(total color-of is-a)`, `(domain-elimination
/// color-of is-a)`, std.bijection) are activators carrying the hierarchy
/// relation as a parameter, which is the whole is-a-free trick: no rule body
/// in the library names `is-a`. `typecheck-setup` goes further and reads the
/// *signature* — `(relation color-of House Color)` — to put the declared
/// domain type into `(typecheck-arg-0 color-of is-a House)`.
#[test]
fn a_bijective_declaration_fans_out_into_the_operational_activators() {
    let sat = saturate(&format!(
        "{BIJECTION_SETUP}
        (relation color-of House Color)
        (bijection-hierarchy is-a)
        (typecheck-hierarchy is-a)
        (bijective color-of)
        (is-a H1 House) (is-a Red Color)
        "
    ));
    for want in [
        // std.algebra's `bijective-properties`: the 1-arg property markers.
        "(functional color-of)",
        "(injective color-of)",
        // std.bijection's `bijective-setup`: 2-arg, hierarchy-carrying.
        "(total color-of is-a)",
        "(surjective color-of is-a)",
        "(domain-elimination color-of is-a)",
        "(range-elimination color-of is-a)",
        // std.bijection's `typecheck-setup`: signature-driven, so the type
        // comes off the `(relation …)` declaration.
        "(typecheck-arg-0 color-of is-a House)",
        "(typecheck-arg-1 color-of is-a Color)",
    ] {
        assert!(
            sat.derived(want),
            "the fan-out did not derive {want}; it derived {:?}",
            sat.productive
        );
    }
}

/// **`domain-elimination-forces-the-last-survivor`.**
///
/// The closed-world step a human solver calls "by elimination": H1 must have a
/// colour (`total`) and at most one (`functional`), and every colour but Green
/// is excluded, so Green is *forced*. What makes it worth a test is that the
/// conclusion is positive and nothing in the input asserts it — the rule
/// manufactures a fact out of a `forall` over stored negatives, which is only
/// sound because both algebraic properties are declared.
#[test]
fn domain_elimination_forces_the_last_surviving_value() {
    let sat = saturate(&format!(
        "{BIJECTION_ELIM}
        (relation color-of House Color)
        (functional color-of) (total color-of)
        (domain-elimination color-of is-a)
        (is-a H1 House)
        (is-a Red Color) (is-a Blue Color) (is-a Green Color)
        (not (color-of H1 Red) :source \"a\") (not (color-of H1 Blue) :source \"b\")
        "
    ));
    assert_eq!(
        sat.derived_of("color-of"),
        ["(color-of H1 Green)"],
        "the survivor, and only the survivor, must be forced"
    );
    assert_eq!(sat.fired("domain-elimination"), 1);
}

/// **`range-elimination-forces-the-last-survivor`.**
///
/// The same argument reflected through the relation's other position: Red must
/// be somewhere (`surjective`) and in at most one house (`injective`), so with
/// H1 and H2 excluded it is H3's. The mirror is a separate rule with its own
/// property guards, not a converse of the first, so it can break on its own —
/// and what distinguishes the two is only which position the quantified
/// variable sits in.
#[test]
fn range_elimination_forces_the_last_surviving_object() {
    let sat = saturate(&format!(
        "{BIJECTION_ELIM}
        (relation color-of House Color)
        (injective color-of) (surjective color-of)
        (range-elimination color-of is-a)
        (is-a Red Color)
        (is-a H1 House) (is-a H2 House) (is-a H3 House)
        (not (color-of H1 Red) :source \"a\") (not (color-of H2 Red) :source \"b\")
        "
    ));
    assert_eq!(
        sat.derived_of("color-of"),
        ["(color-of H3 Red)"],
        "the surviving *house* must be forced, from the range side"
    );
    assert_eq!(sat.fired("range-elimination"), 1);
}

/// **`domain-elimination-is-silent-with-two-survivors`.**
///
/// The guard is the subject, not the conclusion. With Blue and Green both
/// still open the rule must derive nothing at all — and the failure mode it
/// rules out is specific and severe: a `forall` that quantified over the
/// *excluded* colours instead of the surviving ones would happily "force" Blue
/// and Green both, and a test that only ever checked the one-survivor case
/// would stay green, because each forced fact is individually plausible.
#[test]
fn domain_elimination_says_nothing_while_two_values_survive() {
    let sat = saturate(&format!(
        "{BIJECTION_ELIM}
        (relation color-of House Color)
        (functional color-of) (total color-of)
        (domain-elimination color-of is-a)
        (is-a H1 House)
        (is-a Red Color) (is-a Blue Color) (is-a Green Color)
        (not (color-of H1 Red) :source \"a\") (not (color-of H1 Blue) :source \"b\")
        "
    ));
    assert!(
        sat.derived_of("color-of").is_empty(),
        "two survivors licence no conclusion, but it derived {:?}",
        sat.derived_of("color-of")
    );
    assert_eq!(sat.fired("domain-elimination"), 0);
}

/// **`functional-negative-completes-the-row`.**
///
/// Elimination consumes stored negatives; this is where they come from. One
/// positive on a functional relation excludes every *other* value of the range
/// type — the row of the grid, filled in. The `neq` guard is the load-bearing
/// part, and the reason the assertion is an equality over the whole derived
/// set rather than two `contains`: without the guard the rule would also
/// derive `(not (color-of H1 Red))`, contradicting the fact it fired on.
#[test]
fn a_functional_positive_excludes_every_other_value() {
    let sat = saturate(&format!(
        "{BIJECTION_NEG}
        (relation color-of House Color)
        (functional-negative color-of is-a)
        (is-a Red Color) (is-a Blue Color) (is-a Green Color)
        (color-of H1 Red :source \"given\")
        "
    ));
    assert_eq!(
        sat.derived_of("not"),
        ["(not (color-of H1 Blue))", "(not (color-of H1 Green))"],
        "the row must be completed, and the fired-on value left alone"
    );
}

/// **`injective-negative-completes-the-column`.**
///
/// The column of the same grid: on an injective relation one positive excludes
/// every other *object*. Same shape, other position — and it is a second test
/// rather than a parameter of the first because the two rules differ by
/// exactly which argument the fresh variable sits in, which is the mistake a
/// copy-paste of one into the other makes.
#[test]
fn an_injective_positive_excludes_every_other_object() {
    let sat = saturate(&format!(
        "{BIJECTION_NEG}
        (relation color-of House Color)
        (injective-negative color-of is-a)
        (is-a H1 House) (is-a H2 House) (is-a H3 House)
        (color-of H1 Red :source \"given\")
        "
    ));
    assert_eq!(
        sat.derived_of("not"),
        ["(not (color-of H2 Red))", "(not (color-of H3 Red))"],
        "the column must be completed, and the fired-on object left alone"
    );
}

/// **`typecheck-arg-fires-only-on-a-mistyped-argument`** — merges
/// `test_typecheck_arg0_fires_on_mistyped` and
/// `test_typecheck_silent_when_well_typed`, which are one claim seen from its
/// two sides.
///
/// The rule is negation-as-failure over a hierarchy relation supplied as a
/// parameter, so both halves have to be pinned together: a typecheck that
/// never fires passes the second test on its own, and one that always fires
/// passes the first. The mistyped fixture also declares a well-formed
/// `(is-a Bob Person)`, so what is detected is "not a House" rather than
/// "unknown symbol" — which a rule keyed on the wrong argument would also
/// report.
#[test]
fn a_typecheck_fires_on_the_mistyped_argument_and_on_nothing_else() {
    let mistyped = saturate(&format!(
        "{BIJECTION_TC}
        (relation color-of House Color)
        (typecheck-arg-0 color-of is-a House)
        (typecheck-arg-1 color-of is-a Color)
        (is-a H1 House) (is-a Red Color) (is-a Bob Person) (is-a Person T)
        (color-of Bob Red :source \"bad\")
        "
    ));
    assert_eq!(
        mistyped.fired("typecheck-arg-0"),
        1,
        "arg 0 is a Person where a House is declared — exactly one report"
    );
    assert_eq!(
        mistyped.fired("typecheck-arg-1"),
        0,
        "arg 1 is a well-typed Color; only the offending position may fire"
    );

    let well_typed = saturate(&format!(
        "{BIJECTION_TC}
        (relation color-of House Color)
        (typecheck-arg-0 color-of is-a House)
        (typecheck-arg-1 color-of is-a Color)
        (is-a H1 House) (is-a Red Color)
        (color-of H1 Red :source \"ok\")
        "
    ));
    assert_eq!(
        well_typed.fired("typecheck-arg-0") + well_typed.fired("typecheck-arg-1"),
        0,
        "a well-typed fact must produce no report at all"
    );
}

/// **`total-detects-an-object-with-no-room`.**
///
/// `std.algebra`'s totality *check*, as distinct from `std.bijection`'s
/// elimination: same premises, opposite conclusion. Both read a `forall` over
/// the range type; elimination fires when exactly one value survives and this
/// one when none does. The open-world caveat is what the fixture exercises —
/// the check may fire only when every partner is **explicitly** excluded by a
/// stored `(not …)`, because a "must hold" reading would fire on every
/// not-yet-decided object in the puzzle, i.e. on the whole of a fresh zebra.
#[test]
fn totality_reports_an_object_whose_every_partner_is_excluded() {
    let src = "(import std.macro :symbols (forall))\n\
        (import std.algebra :symbols (total))\n\
        (relation color-of House Color)
        (total color-of is-a)
        (is-a H1 House)
        (is-a Red Color) (is-a Blue Color)
        ";
    let boxed_in = saturate(&format!(
        "{src}
        (not (color-of H1 Red) :source \"a\") (not (color-of H1 Blue) :source \"b\")
        "
    ));
    assert_eq!(
        boxed_in.fired("total"),
        1,
        "every Color excluded for H1 is ⊥, reported once"
    );
    assert!(boxed_in.derived("(false)"), "and the report is a `(false)`");

    // One colour still open — the undecided state a puzzle spends its whole
    // search in.
    let open = saturate(&format!("{src}(not (color-of H1 Red) :source \"a\")\n"));
    assert_eq!(
        open.fired("total"),
        0,
        "an undecided object is not a violation"
    );
}

// ── std.elim — the positional twin ─────────────────────────────────

/// **`no-room-left-fires-when-every-value-is-excluded`.**
///
/// `std.elim`'s half of the same pair: `domain-elimination` forces the last
/// survivor, `no-room-left` reports the case where there is none. They share
/// their `functional`+`total` guard and differ only in what the `forall`
/// quantifies over, which is precisely why one can be right while the other is
/// wrong — so the boundary is asserted from both sides: two colours excluded
/// fires it once, one colour excluded fires it not at all.
#[test]
fn no_room_left_reports_the_object_with_nowhere_to_go() {
    let src = "(relation color-of House Color) (relation is-a T T)
        (functional color-of 0 1) (total color-of 0)
        (no-room-left color-of is-a House Color)
        (is-a House T) (is-a Color T)
        (is-a H1 House)
        (is-a Red Color) (is-a Blue Color)
        ";
    let boxed_in = saturate(&format!(
        "{ELIM}{src}
        (not (color-of H1 Red) :source \"(a)\")
        (not (color-of H1 Blue) :source \"(b)\")
        "
    ));
    assert_eq!(boxed_in.fired("no-room-left"), 1);

    let survivor = saturate(&format!(
        "{ELIM}{src}(not (color-of H1 Red) :source \"(a)\")\n"
    ));
    assert_eq!(
        survivor.fired("no-room-left"),
        0,
        "Blue is still available, so there is room"
    );
}

/// **`elim-typecheck-fires-only-on-a-mistyped-argument`.**
///
/// The claim of `a_typecheck_fires_on_the_mistyped_argument_and_on_nothing_else`
/// against `std.elim`'s own copy of the rule, and deliberately not merged into
/// it: the two modules ship two definitions of `typecheck-arg-0`, and
/// `MANIFEST.sha256` notices that one of them *changed*, never that it stopped
/// working. The fixture is the zebra-flavoured one the Python test used — a
/// Person handed to a relation declared over Houses, which is the mistake a
/// hand-written puzzle actually makes.
#[test]
fn the_elim_typecheck_also_fires_only_on_the_mistyped_argument() {
    let mistyped = saturate(&format!(
        "{ELIM}
        (relation color-of House Color) (relation is-a T T)
        (typecheck-arg-0 color-of is-a House)
        (is-a House T) (is-a Color T) (is-a Person T)
        (is-a Englishman Person)
        (is-a H1 House) (is-a Red Color)
        (color-of Englishman Red :source \"(bad)\")
        "
    ));
    assert_eq!(mistyped.fired("typecheck-arg-0"), 1);

    let well_typed = saturate(&format!(
        "{ELIM}
        (relation color-of House Color) (relation is-a T T)
        (typecheck-arg-0 color-of is-a House)
        (typecheck-arg-1 color-of is-a Color)
        (is-a House T) (is-a Color T)
        (is-a H1 House) (is-a Red Color)
        (color-of H1 Red :source \"(ok)\")
        "
    ));
    assert_eq!(
        well_typed.fired("typecheck-arg-0") + well_typed.fired("typecheck-arg-1"),
        0
    );
}

/// **`domain-elimination-needs-its-property-facts`.**
///
/// Declaring the activator opts a relation *into* the rule; it does not
/// licence the conclusion. Forcing the survivor is sound only because the
/// relation was declared `functional` (H1 gets at most one colour) and `total`
/// (H1 gets at least one) — drop those two facts and the same excluded-colour
/// evidence supports nothing, because H1 may simply have no colour at all. An
/// activator that fired on its own would be a soundness bug rather than a
/// missing guard, which is why the silence gets a test and a control: the same
/// program *with* the properties does force Blue, so what is asserted above is
/// the guard's doing and not a broken fixture.
#[test]
fn domain_elimination_needs_the_algebraic_properties_not_just_its_activator() {
    let src = "(relation color-of House Color) (relation is-a T T)
        (domain-elimination color-of is-a House Color)
        (is-a House T) (is-a Color T)
        (is-a H1 House)
        (is-a Red Color) (is-a Blue Color)
        (not (color-of H1 Red) :source \"(a)\")
        ";
    let bare = saturate(&format!("{ELIM}{src}"));
    assert_eq!(
        bare.fired("domain-elimination"),
        0,
        "no (functional …) / (total …) ⇒ no forcing"
    );

    let declared = saturate(&format!(
        "{ELIM}{src}(functional color-of 0 1) (total color-of 0)\n"
    ));
    assert_eq!(declared.fired("domain-elimination"), 1);
    assert!(declared.derived("(color-of H1 Blue)"));
}

// ── the native symmetric mirror ────────────────────────────────────

/// **`native-mirror-equals-the-userspace-rule`.**
///
/// `(__symmetric__ R)` closes R under arg-swap inside the saturator — no
/// compiled plan, no matcher run — and it exists only because that is faster
/// than the `(symmetric R)` rule in `std.algebra`. An optimisation is allowed
/// to be faster and nothing else, so the two must agree on the extension, and
/// this is the assertion that lets the kernel path be substituted for the
/// library one.
///
/// Equality alone would be satisfied by two paths that both do nothing, so the
/// closure is spelled out as well. The self-loop is in the fixture on purpose:
/// mirroring `(knows E E)` produces the fact it started from, and an
/// implementation that treated its own output as new work would either
/// duplicate it or loop.
#[test]
fn the_native_mirror_computes_the_same_closure_as_the_stdlib_rule() {
    let edges = "(relation knows T T)\n(knows A B)\n(knows C D)\n(knows E E)\n";
    let native = knows_extension(&format!("(__symmetric__ knows)\n{edges}"));
    let userspace = knows_extension(&format!(
        "(import std.algebra :symbols (symmetric))\n(symmetric knows)\n{edges}"
    ));
    assert_eq!(
        native,
        [
            "(knows A B)",
            "(knows B A)",
            "(knows C D)",
            "(knows D C)",
            "(knows E E)"
        ],
        "three edges mirror to five facts — the self-loop mirrors onto itself"
    );
    assert_eq!(
        native, userspace,
        "the kernel fast path must be the userspace rule's closure, not another one"
    );
}

// ── symmetric relations under search ───────────────────────────────

/// Solve a corpus file exhaustively — `stop_after: None`, because a truncated
/// search reports `k` as a lower bound and every claim below is about the
/// exact count.
fn solve_exhaustively(rel: &str) -> (Answer, u64, bool, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut events = Events::off();
    let opts = SolveOptions {
        stop_after: None,
        ..SolveOptions::default()
    };
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e:?}"));
    (
        solved.answer,
        solved.stats.solution_nodes,
        solved.stats.exhausted,
        terms,
    )
}

fn holds(kb: &Kb, terms: &Terms, rel: &str, a: &str, b: &str) -> bool {
    let (Some(rel), Some(a), Some(b)) = (terms.syms.get(rel), terms.syms.get(a), terms.syms.get(b))
    else {
        return false;
    };
    terms
        .probe_fact(rel, &[Value::sym(a), Value::sym(b)])
        .is_some_and(|f| kb.contains(f))
}

/// **`a-symmetric-pair-counts-as-one-model`.**
///
/// `05_mini_zebra` branches on `co-located`, a relation the puzzle declares
/// symmetric, so every commitment has a twin: guessing `(co-located Bob
/// Coffee)` and guessing `(co-located Coffee Bob)` are the same guess. The
/// kernel used to know that — it canonicalised symmetric pairs and mirrored on
/// death — and S1.7.24 took all of it out. Nothing replaced it *specifically*:
/// the two orientations saturate to the same KB through the puzzle's own
/// `(rule symmetric)`, so they collide at the generic `state_key` dedup, and
/// `k` comes back 1 for a reason that has nothing to do with symmetry. This
/// test is what keeps that accident honest — and it asserts the model too,
/// because `k = 1` says nothing about *which* model survived.
#[test]
fn an_undecided_symmetric_pair_is_one_model_not_two() {
    let (answer, k, exhausted, terms) = solve_exhaustively("examples/branching/05_mini_zebra.ein");
    assert!(exhausted, "uniqueness needs an exhausted search");
    assert_eq!(k, 1, "the two orientations of one guess are one model");
    let Answer::Verdict(Verdict::Solution(s)) = &answer else {
        panic!("expected a Solution, got {}", answer.as_str());
    };
    assert!(
        holds(&s.kb, &terms, "co-located", "Bob", "Coffee"),
        "Bob drinks the Coffee"
    );
    assert!(
        holds(&s.kb, &terms, "co-located", "Bob", "Dog"),
        "Bob owns the Dog"
    );
}

/// **`a-two-model-symmetric-puzzle-is-not-double-counted`.**
///
/// The other half of the same contract, and the half a dedup bug cannot fake:
/// `04_two_levels` really has two answers — Blue and Green are free to swap
/// between H2 and H3 — over the same symmetric `co-located`. So `k` must be
/// exactly 2. A regression that counted orientations separately would inflate
/// it; one that over-merged states would collapse it to 1 and turn a genuine
/// ambiguity into a false claim of uniqueness. Only holding both fixtures at
/// once pins the dedup between those two failures.
#[test]
fn a_genuinely_ambiguous_symmetric_puzzle_reports_exactly_its_two_models() {
    let (answer, k, exhausted, _) = solve_exhaustively("examples/branching/04_two_levels.ein");
    assert!(exhausted);
    assert_eq!(k, 2, "two placements, not four orientations");
    let Answer::Verdict(Verdict::Ambiguity(models)) = &answer else {
        panic!("expected an Ambiguity, got {}", answer.as_str());
    };
    assert_eq!(models.len(), 2, "and both are recorded as branches");
}

// ── reflective activation ──────────────────────────────────────────

/// **`a-reflective-loop-reaches-a-fixpoint`.**
///
/// A derived fact whose head is a rule name becomes an activator on the next
/// pass — that is what lets `std.algebra`'s lemmas work, and what lets
/// `bijective-setup` above fan a declaration out into rules. It also means a
/// rule set can feed itself: here `a` derives `(mark k)`, `mark` derives
/// `(a k)`, and `(a k)` is `a`'s own activator. Nothing in the rules says stop.
///
/// What stops it is the firing dedup: second time round every candidate has
/// been seen with the same bindings, so it is never re-enqueued. The test
/// therefore shows both halves — that the loop actually closed (the second
/// rule ran, so the derived activator really was consumed) and that it closed
/// *quickly*, in tens of firings rather than by exhausting a budget. Without
/// the first assertion the second would be satisfied by a program that never
/// got going.
#[test]
fn a_rule_that_derives_its_own_activator_still_reaches_a_fixpoint() {
    let src = "(rule a (?rel) :match (?rel ?x ?y) :assert (mark ?rel) :why \"w\" :priority 100)
        (rule mark () :match (mark ?r) :assert (a ?r) :why \"w\" :priority 100)
        (relation k T T) (relation mark T) (relation a T)
        (a k)
        (k A B :source \"(1)\")
        ";
    let sat = saturate(src);
    assert!(
        sat.derived("(mark k)"),
        "the activator-shaped conclusion must be derived at all"
    );
    assert!(
        sat.all_rules.iter().any(|r| r == "mark"),
        "the derived `(mark k)` must have activated the second rule — without \
         that the loop never closes and termination is vacuous"
    );
    assert!(
        sat.all_rules.len() < 100,
        "bounded, not runaway: {} firings",
        sat.all_rules.len()
    );
}

/// **`sibling-exclusive-under-a-self-parametrised-activator`.**
///
/// `sibling-exclusive` takes the relation that defines the sibling group and
/// the relation those siblings may not stand in — and `(sibling-exclusive
/// is-a is-a)` binds both to one relation, so the rule reads its own premise
/// relation and negates it: two leaves of one type are not each other's type.
/// Why that is not obviously fine: the derived `(not (is-a Red Blue))` is a
/// negation *of the relation the rule scans*, so an engine that let stored
/// negations into that relation's extent would feed them straight back in. It
/// converges instead, and the positives are untouched — which is what the last
/// assertion checks, because a rule that had also negated `(is-a Red Color)`
/// would still satisfy the first two.
#[test]
fn sibling_exclusive_can_negate_the_relation_it_reads() {
    let sat = saturate(
        "(rule sibling-exclusive (?siblings-via ?exclusive-under)
           :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
           :assert (not (?exclusive-under ?a ?b))
           :why \"sib\" :priority 300)
         (relation is-a       T T)
         (relation co-located T T)
         (sibling-exclusive is-a co-located)
         (sibling-exclusive is-a is-a)
         (is-a Color T)
         (is-a Red Color) (is-a Blue Color)
        ",
    );
    let negatives = sat.derived_of("not");
    for want in ["(not (is-a Red Blue))", "(not (is-a Blue Red))"] {
        assert!(
            negatives.contains(&want.to_string()),
            "the self-parametrised activator must exclude the leaf pair both \
             ways; derived {negatives:?}"
        );
    }
    assert!(
        !negatives.iter().any(|n| n.contains("(is-a Red Color)")),
        "membership must survive the exclusion of its siblings: {negatives:?}"
    );
}
