//! `MAX_REGS` overflow is a clean refusal — S1a.3.1, and the engine's one
//! compile-time bound.
//!
//! ein.rs resolves every variable to a register in a fixed-size file, because
//! that is what makes the matcher's inner loop allocation-free
//! ([design/05](../../../../docs/history/m1a_rust/design/05_matcher.md) §3) — so the
//! count has to stop somewhere, and where it stops is observable: a rule may
//! not bind more than 256 variables.
//!
//! The bound gets a stated condition under which it becomes a bug (a real
//! program hits it), and a fixture. The fixture is built here rather than
//! checked into `examples/`.
//!
//! **Refused at load since M1f S1f.5.21** (W63, Q-M1f.73): the loader counts
//! every distinct variable a `:match` writes, the rule's parameters among
//! them, against [`ein_ir::MAX_MATCH_VARIABLES`], which `MAX_REGS` is, so every
//! command refuses a rule past it alike and the compiler's check is an
//! invariant. Until then the compiler refused it where it compiled the plan,
//! so a rule no state activated loaded and answered.

use ein_core::Terms;
use ein_ir::{Ast, from_ir::load, parse};

/// A rule whose `:match` binds `n` distinct variables.
fn wide_rule(n: usize) -> String {
    let premises: Vec<String> = (0..n).map(|i| format!("(p ?v{i})")).collect();
    format!(
        "(relation p Thing :why \"p holds of {{?1}}\")\n(rule wide ()\n  :match (and {})\n  :assert (q ?v0) :why \"p holds of {{?v0}} and of each other value the rule binds, so q holds of {{?v0}}\")\n",
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

/// One variable past the ceiling is refused **at load**, with a line that
/// says what to do — and never reaches the compiler, whose check is an
/// invariant since M1f S1f.5.21.
#[test]
fn one_variable_past_the_ceiling_is_refused_at_load() {
    assert_eq!(ein_infer::MAX_REGS, ein_ir::MAX_MATCH_VARIABLES);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let text = wide_rule(ein_infer::MAX_REGS + 1);
    let forms = parse(&mut ast, &text, Some("<wide>")).expect("the fixture parses");
    let err = load(&mut ast, &mut terms, &forms, None)
        .expect_err("past the ceiling")
        .0;
    // The point of the bound is a message, not a limit: it has to say what to
    // do.
    assert_eq!(
        err,
        "rule 'wide': its :match writes 257 distinct variables, and a rule binds at most 256 \
         — split the rule"
    );
}

/// The one module whose rules are wide **by design**: `std.num9` (M1f
/// S1f.6.13), school arithmetic on nine digits in one fact. A rule of it has
/// a premise a column, so it binds some multiple of nine variables: 35 for a
/// sum, and [`WIDEST_BY_DESIGN`] for the one that computes a divisor's
/// multiples by 2 to 9, eight rows of nine columns in one firing. The module
/// is generated (`utils/gen_num9.py`), its width is fixed, and no rule of it
/// grows unless the generator is changed, which moves the number below.
const WIDE_BY_DESIGN: &str = "stdlib/num9.ein";

/// `div9-start`: nine digits of the dividend, nine of the divisor, and for
/// each multiple by 2 to 9 its nine digits and the nine carries between its
/// columns: 18 + 8 × 18.
const WIDEST_BY_DESIGN: (&str, usize) = ("div9-start", 162);

/// The names of the rules [`WIDE_BY_DESIGN`] declares.
fn wide_by_design() -> std::collections::BTreeSet<String> {
    let path = ein_corpus::repo_root().join(WIDE_BY_DESIGN);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("std.num9 parses");
    forms
        .iter()
        .filter(|&&f| ast.head_name(f) == Some("rule"))
        .filter_map(|&f| ast.form_args(f).first().and_then(|&a| ast.atom_name(a)))
        // Each by its short name, as `stdlib/num9.ein` loaded alone prints it,
        // and by its full name, as a program that imports it does since M1f
        // S1f.6.35.
        .flat_map(|n| [n.to_string(), format!("std.num9.{n}")])
        .collect()
}

/// The bound is two orders of magnitude past the rules people write. If one
/// ever approaches it, the bound stops being acceptable — so the distance is
/// measured rather than asserted from memory.
///
/// **Since M1f S1f.6.13 one module is measured apart**
/// ([Q-M1f.173](../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f173--max_regs-and-the-wide-rules-of-stdnum9),
/// defaulted, the user's to overrule). `std.num9`'s generated rules are
/// within 8× of the ceiling by what they are, and the widest takes 162 of
/// the 256 registers. That is held as a number, [`WIDEST_BY_DESIGN`], and
/// under the ceiling; every other rule of the corpus is held to the distance
/// it always was. The bound itself is unmoved: nothing hits it.
#[test]
fn the_corpus_is_nowhere_near_the_ceiling() {
    let by_design = wide_by_design();
    assert!(
        by_design.len() >= 40,
        "std.num9 declared {} rules",
        by_design.len()
    );
    let (mut widest, mut widest_rule) = (0usize, String::new());
    let (mut designed, mut designed_rule) = (0usize, String::new());
    for path in ein_corpus::corpus_files() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = ein_ir::load_file(&mut ast, &mut terms, &path) else {
            continue;
        };
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();
        for rule in rules {
            for activator in ein_infer::activators_for(&kb, &terms, &rule) {
                let Ok(plan) = ein_infer::compile_rule(&ast, &mut terms, &rule, activator) else {
                    continue;
                };
                let (regs, name) = (plan.n_regs as usize, terms.sym(plan.rule).to_string());
                let (top, top_rule) = if by_design.contains(&name) {
                    (&mut designed, &mut designed_rule)
                } else {
                    (&mut widest, &mut widest_rule)
                };
                if regs > *top {
                    *top = regs;
                    *top_rule = name;
                }
            }
        }
    }
    assert!(widest > 0, "the corpus compiled nothing");
    assert!(
        widest * 8 <= ein_infer::MAX_REGS,
        "the widest corpus rule ({widest_rule}, {widest} registers) is within \
         8× of MAX_REGS ({}) — the bound needs re-deciding",
        ein_infer::MAX_REGS
    );
    assert_eq!(
        (designed_rule.trim_start_matches("std.num9."), designed),
        WIDEST_BY_DESIGN,
        "the widest rule of {WIDE_BY_DESIGN} moved: its rules are wide by design, \
         and how wide is a decision about MAX_REGS ({}), which Q-M1f.173 took at \
         this number",
        ein_infer::MAX_REGS
    );
    assert!(
        designed < ein_infer::MAX_REGS,
        "a rule of {WIDE_BY_DESIGN} is at the ceiling"
    );
    eprintln!(
        "widest corpus rule: {widest_rule} with {widest} registers; of {WIDE_BY_DESIGN}, \
         {designed_rule} with {designed}"
    );
}
