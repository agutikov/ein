//! `MAX_REGS` overflow is a clean error — S1a.3.1, and the port's one
//! compile-time bound.
//!
//! ein.py numbers nothing: its bindings are a `dict`, so a rule with a
//! thousand variables costs a thousand dict entries and compiles fine. ein.rs
//! resolves every variable to a register in a fixed-size file, because that is
//! what makes the matcher's inner loop allocation-free
//! ([design/05](../../../../plans/m1a_rust/design/05_matcher.md) §3) — so the
//! count has to stop somewhere, and where it stops is observable.
//!
//! This is therefore a **divergence**, ledger entry D1, and it gets the
//! treatment the ledger asks for: a stated condition under which it becomes a
//! bug (a real program hits it), and a fixture. The fixture is built here
//! rather than checked into `examples/`, deliberately — a corpus file that
//! ein.py compiles and ein.rs refuses would fail the corpus parity test, which
//! is exactly the alarm the ledger wants kept armed for *unintended*
//! divergences.

use ein_core::Terms;
use ein_ir::{Ast, from_ir::load, parse};

/// A rule whose `:match` binds `n` distinct variables.
fn wide_rule(n: usize) -> String {
    let premises: Vec<String> = (0..n).map(|i| format!("(p ?v{i})")).collect();
    format!(
        "(relation p Thing)\n(rule wide ()\n  :match (and {})\n  :assert (q ?v0))\n",
        premises.join(" ")
    )
}

fn compile_wide(n: usize) -> Result<usize, String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let text = wide_rule(n);
    let forms = parse(&mut ast, &text, Some("<wide>")).expect("the fixture parses");
    let kb = load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    let rule = kb
        .program()
        .rules
        .values()
        .next()
        .expect("one rule")
        .clone();
    ein_infer::compile_rule(&ast, &mut terms, &rule, None)
        .map(|plan| plan.n_regs as usize)
        .map_err(|e| e.0)
}

#[test]
fn a_rule_at_the_ceiling_compiles() {
    assert_eq!(
        compile_wide(ein_infer::MAX_REGS),
        Ok(ein_infer::MAX_REGS),
        "the last legal width must still compile"
    );
}

#[test]
fn one_variable_past_the_ceiling_is_an_error_and_not_a_panic() {
    let err = compile_wide(ein_infer::MAX_REGS + 1).expect_err("past the ceiling");
    assert!(
        err.contains("more than 256 distinct variables"),
        "unhelpful message: {err}"
    );
    // The point of the bound is a message, not a limit: it has to say what to
    // do, because ein.py accepts the same file.
    assert!(err.contains("Split the rule."), "no remedy offered: {err}");
}

/// The bound is two orders of magnitude past the corpus. If a shipping rule
/// ever approaches it, D1 stops being acceptable — so the distance is measured
/// rather than asserted from memory.
#[test]
fn the_corpus_is_nowhere_near_the_ceiling() {
    let mut widest = 0usize;
    let mut widest_rule = String::new();
    for path in ein_oracle::corpus_files() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = ein_ir::load_file(&mut ast, &mut terms, &path) else {
            continue;
        };
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();
        for rule in rules {
            for activator in ein_infer::activators_for(&kb, &terms, &rule) {
                if let Ok(plan) = ein_infer::compile_rule(&ast, &mut terms, &rule, activator)
                    && plan.n_regs as usize > widest
                {
                    widest = plan.n_regs as usize;
                    widest_rule = terms.sym(plan.rule).to_string();
                }
            }
        }
    }
    assert!(widest > 0, "the corpus compiled nothing");
    assert!(
        widest * 8 <= ein_infer::MAX_REGS,
        "the widest corpus rule ({widest_rule}, {widest} registers) is within \
         8× of MAX_REGS ({}) — D1 needs re-deciding",
        ein_infer::MAX_REGS
    );
    eprintln!("widest corpus rule: {widest_rule} with {widest} registers");
}
