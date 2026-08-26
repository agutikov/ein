//! **Every stdlib rule is activated by a program written to activate it** —
//! M1c [S1c.1.5](../../../../docs/history/m1c_external_validation/README.md#s1c15--in-the-gate).
//!
//! [S1c.1.1](../../../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)
//! measured the gap this closes: **38 of the stdlib's 73 rules never fired**
//! in any of 400 corpus runs, and 20 more were activated by `examples/zebra.ein`
//! and by nothing else. [S1c.1.4](../../../../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus)
//! wrote the 45 programs under [`tests/stdlib/`](../../../../tests/README.md)
//! that took the zero set to 0. This is what stops it growing back.
//!
//! ## The claim is about the *suite*, not about the corpus
//!
//! [`utils/stdlib_census.py --check`](../../../../utils/stdlib_census.py)
//! sweeps all 180 corpus entries and exits 1 while any rule is at zero. That
//! is the weaker claim, and it is weak in the way that matters here: a rule
//! added tomorrow that happens to fire somewhere inside `examples/zebra.ein`
//! would pass it **with no test written**. So the sweep below is scoped to
//! `tests/stdlib/` — the directory whose whole job is to activate rules — and
//! the claim is that the suite stands on its own. It does, as of S1c.1.5:
//! 73 of 73, with no `examples/` entry contributing.
//!
//! Scoping it also found the one rule the suite did not run. `transitive`'s
//! fixture was a two-cycle, where the `(neq ?a ?c)` guard refuses every match
//! the rule finds — deliberate, and the right test of the *guard*, but it left
//! the rule's assertion resting on six puzzles. `21_transitive.ein` grew a
//! three-chain.
//!
//! ## Why this is a test and not the script
//!
//! The script shells out to a release binary 557 times and takes 37 s; a check
//! shaped like that runs when somebody remembers it. This one runs in
//! `cargo test`, needs no binary, and costs **0.04 s** — 45 programs of three
//! declarations and two facts exhaust in microseconds. **It fails the
//! moment a rule is added without a program**, which is the only moment anyone
//! will read it.
//!
//! What it does *not* re-implement is the measurement: the census stays the
//! instrument — per-rule firing counts, productive vs redundant, the sole-
//! activator table — and this is the one bit of it that has to be true on
//! every commit. The attribution rule below is the census's `resolve()`, and
//! the two must agree: a local declaration shadows a stdlib name outright, a
//! module the file never imported cannot have fired, and the arity of the
//! activator splits `std.elim`'s four-parameter `domain-elimination` from
//! `std.bijection`'s two-parameter one.
//!
//! ## What it deliberately does not check
//!
//! The dual — "every program activates a stdlib rule" — is **not** gated, and
//! four fixtures would fail it: `algebra/08_checks_satisfied.ein` and
//! `18_totality_open_world.ein` exist to show a rule is loaded, activated and
//! *silent*, and the two `macro/` programs test expansion, which `std.macro`
//! does with no rules at all. A rule not firing is the case S1c.1.4's notes
//! call the one that finds bugs; a gate that forbade it would forbid the
//! better half of the suite.
//!
//! Nor is *sensitivity*. A program can fire a rule and still pass with the
//! rule broken — that claim was taken by hand over 51 mutants (50 caught,
//! [`tests/README.md`](../../../../tests/README.md)) and it is not the kind of
//! number a gate can hold.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use ein_core::Terms;
use ein_corpus::{ein_files_under, repo_root};
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, Node, NodeId, load_query, parse};

/// The suite under test — the directory S1c.1.4 put the programs in, and the
/// third corpus root beside `examples/` and `stdlib/`.
const SUITE: &str = "tests";

/// The subject.
const STDLIB: &str = "stdlib";

/// The one module that declares no rules, and so is absent from the inventory
/// by construction. Named rather than inferred: `std.macro` ships `forall` and
/// `open`, which are *macros*, and a day when `algebra.ein` parses to zero
/// rules must not read the same as this.
const RULE_FREE_MODULE: &str = "std.macro";

// ── the declaration inventory ──────────────────────────────────────

/// A `(rule …)` head in `stdlib/*.ein`.
#[derive(Clone, Debug)]
struct Decl {
    module: String,
    name: String,
    /// The activator's arity, which is what a `fire` or `owe` event's
    /// `activator` list can be compared against — the census's last tiebreak, and the one that
    /// splits `std.elim`'s `(?R ?isa ?OT ?VT)` `domain-elimination` from
    /// `std.bijection`'s `(?R ?isa)`. On today's suite it never has to: the
    /// two modules do not import each other, so the closure has already
    /// separated them by the time arity is asked. It is here because the
    /// census has it, and the two attributions must be one rule.
    params: usize,
}

impl Decl {
    fn key(&self) -> String {
        format!("{}/{}", self.module, self.name)
    }
}

/// Every `(rule …)` under `stdlib/`, and every module's own `(import …)` list.
///
/// Parsed with the engine's own parser rather than scanned: a rule head this
/// could not read would be a rule the gate silently stopped requiring a test
/// for, which is the failure a coverage check must not have.
fn inventory() -> (Vec<Decl>, BTreeMap<String, Vec<String>>) {
    let dir = repo_root().join(STDLIB);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no modules in {}", dir.display());

    let (mut decls, mut imports) = (Vec::new(), BTreeMap::new());
    for path in &paths {
        let module = format!("std.{}", path.file_stem().expect("stem").to_string_lossy());
        let mut ast = Ast::new();
        let text = std::fs::read_to_string(path).expect("module text");
        let forms = parse(&mut ast, &text, path.to_str()).expect("the stdlib parses");
        let (rules, mods) = heads(&ast, &forms);
        for (name, params) in rules {
            decls.push(Decl {
                module: module.clone(),
                name,
                params,
            });
        }
        imports.insert(module, mods);
    }
    (decls, imports)
}

/// The `(rule …)` / `(hrule …)` names a form list declares — with their
/// parameter counts — and the modules it imports.
///
/// Read off a file's **own** forms, before [`load_query`] resolves imports by
/// splicing the modules' text in place. After resolution there is nothing left
/// to tell a file's rules from its library's.
fn heads(ast: &Ast, forms: &[NodeId]) -> (Vec<(String, usize)>, Vec<String>) {
    let (mut rules, mut imports) = (Vec::new(), Vec::new());
    for &form in forms {
        let args = ast.form_args(form).to_vec();
        match ast.head_name(form) {
            Some("rule" | "hrule") if args.len() >= 2 => {
                if let Some(name) = ast.atom_name(args[0]) {
                    rules.push((name.to_string(), params_of(ast, args[1])));
                }
            }
            Some("import") if !args.is_empty() => {
                if let Some(m) = ast.atom_name(args[0]) {
                    imports.push(m.to_string());
                }
            }
            _ => {}
        }
    }
    (rules, imports)
}

/// How many parameters a rule's `(?R ?isa …)` form declares.
///
/// Two shapes need care and neither is exotic: the head of a parameter list is
/// a `Var`, so `head_name` is `None` there and the count is `1 + args`; and
/// `()` — fifteen rules over five of the seven modules — parses to the
/// synthetic `@empty` head, which is **zero** parameters rather than one.
fn params_of(ast: &Ast, form: NodeId) -> usize {
    let Node::SForm { head, args } = ast.node(form) else {
        return 0;
    };
    if ast.atom_name(head) == Some("@empty") {
        0
    } else {
        1 + ast.args(args).len()
    }
}

// ── running one program ────────────────────────────────────────────

/// What one fixture did, and what it could have done it with.
struct Ran {
    /// `(rule, activator arity)` per `fire` event, redundant firings included:
    /// the census counts activation, and a rule that matched, passed its
    /// guards and re-derived a fact was activated.
    fired: Vec<(String, usize)>,
    /// The `std.*` modules the file's imports reach, transitively.
    closure: BTreeSet<String>,
    /// The rule names the file declares itself. Twelve fixtures declare
    /// thirteen rules between them, ten of those the same `probe-undecided`;
    /// none shadows a stdlib name today, and the day one does it must not be
    /// credited to the stdlib.
    local: BTreeSet<String>,
}

/// Load and solve one program to exhaustion — the run `ein test` makes — and
/// record what it activated.
///
/// Exhaustive rather than a bare `saturate`, though on today's suite the two
/// reach exactly the same 73 rules (measured, 2026-08-24). The difference is
/// what happens to a rule whose only firing is *inside* a hypothesis: under
/// saturation the gate would report it untested and the fixture's author would
/// have to find out why. It costs nothing to be right about that: 45 programs
/// of this size exhaust in 0.04 s, all told.
fn run(path: &Path, imports: &BTreeMap<String, Vec<String>>) -> Ran {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let load = |index: usize| {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str())
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let kb = load_query(&mut ast, &mut terms, &forms, path.parent(), index)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        (ast, terms, kb, forms)
    };

    // `ein test`'s own shape: load once to find out what the file claims, and
    // reuse that load for the first query it does. Which queries run is its
    // rule too, rather than "all of them" — a query with no `:expect` states
    // nothing, and the one fixture in the repo that would cost most to enter
    // (`features/04_open.ein`, an unbounded enumeration) is exactly one of
    // those. Solving it would be paying for coverage nobody claimed.
    let (ast, terms, kb, forms) = load(0);
    let (declared, seeds) = heads(&ast, &forms);
    let claims: Vec<usize> = (0..kb.program().queries.len())
        .filter(|&i| ein_infer::query_value(&ast, &kb.program().queries[i], "expect").is_some())
        .collect();

    let mut fired = Vec::new();
    let mut preloaded = Some((ast, terms, kb));
    for &index in &claims {
        let (ast, mut terms, mut kb) = match preloaded.take() {
            Some(first) if index == 0 => first,
            _ => {
                let (ast, terms, kb, _) = load(index);
                (ast, terms, kb)
            }
        };
        let buffer = Buffer::new();
        let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
        let opts = SolveOptions {
            config: Some(kb.program().config.clone().unwrap_or_default()),
            ..SolveOptions::default()
        };
        solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
            .unwrap_or_else(|e| panic!("{}: {e:?}", path.display()));
        fired.extend(read_activations(path, &buffer.to_string_lossy()));
    }

    Ran {
        fired,
        closure: closure(&seeds, imports),
        local: declared.into_iter().map(|(n, _)| n).collect(),
    }
}

/// The events that say a program reached a rule: `(rule, activator arity)`.
///
/// **Two kinds, and the second is not an optimisation of the first.** A
/// saturation rule reaches the agenda and emits `fire`. An **obligation** rule
/// (M1d S1d.2.4) never can — it derives nothing and is deliberately kept out
/// of the agenda, so its only evidence is the `owe` it emits from the
/// post-fixpoint pass. Both lines carry `rule` and `activator` and both mean
/// the same thing here: this program activated that rule and observed what it
/// concluded. Reading only `fire` would have put every obligation rule
/// permanently in the zero set — which is why
/// [S1d.2.3](../../../../docs/history/m1d_satisfiability/README.md#s1d23--the-form)
/// deferred shipping the duals until this pass existed.
fn read_activations(path: &Path, log: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    for line in log.lines() {
        let ev: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("{}: unreadable event line: {e}", path.display()));
        if ev["e"] != "fire" && ev["e"] != "owe" {
            continue;
        }
        out.push((
            ev["rule"]
                .as_str()
                .expect("a fire or an owe names its rule")
                .to_string(),
            ev["activator"].as_array().map_or(0, Vec::len),
        ));
    }
    out
}

/// The `std.*` modules an import list reaches, transitively.
///
/// A module the map does not know is a file-relative import and contributes
/// nothing: the question is only which *stdlib* declarations were in scope.
fn closure(seeds: &[String], imports: &BTreeMap<String, Vec<String>>) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut todo: Vec<String> = seeds.to_vec();
    while let Some(module) = todo.pop() {
        let Some(next) = imports.get(&module) else {
            continue;
        };
        if seen.insert(module) {
            todo.extend(next.iter().cloned());
        }
    }
    seen
}

/// Which stdlib declarations an activated rule name refers to — `stdlib_census.py`'s
/// `resolve()`, and it must stay the same rule.
///
/// A local declaration wins outright: a file that declares `symmetric` fired
/// *its* `symmetric`. Then the import closure, which is what actually splits
/// the two `domain-elimination`s and the two `typecheck-arg-*` pairs today.
/// Then arity, the census's last tiebreak, which no program in the suite
/// reaches. Returns indices into the inventory, and an empty result means
/// "not the stdlib's".
fn resolve(
    name: &str,
    arity: usize,
    ran: &Ran,
    decls: &[Decl],
    by_name: &BTreeMap<String, Vec<usize>>,
) -> Vec<usize> {
    if ran.local.contains(name) {
        return Vec::new();
    }
    let Some(all) = by_name.get(name) else {
        return Vec::new();
    };
    let mut cands: Vec<usize> = all
        .iter()
        .copied()
        .filter(|&i| ran.closure.contains(&decls[i].module))
        .collect();
    if cands.len() > 1 {
        let narrowed: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|&i| decls[i].params == arity)
            .collect();
        if !narrowed.is_empty() {
            cands = narrowed;
        }
    }
    cands
}

/// Every program in the suite, sorted.
fn programs() -> Vec<PathBuf> {
    let files = ein_files_under(&repo_root().join(SUITE));
    assert!(!files.is_empty(), "no programs under {SUITE}/");
    files
}

// ── the gate ───────────────────────────────────────────────────────

/// **The coverage claim, as an assertion.**
///
/// The failure message names the rules and their modules, because the fix is
/// a program under `tests/stdlib/<module>/` and the module says which
/// directory. `stdlib_census.md` §6 is where the smallest activating program
/// for a rule was worked out the first time.
#[test]
fn every_stdlib_rule_is_activated_by_a_program() {
    let (decls, imports) = inventory();
    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, d) in decls.iter().enumerate() {
        by_name.entry(d.name.clone()).or_default().push(i);
    }

    // Vacuity: a parser change that stopped reading rule heads would leave the
    // inventory empty and every claim below true. Every module but one
    // declares rules, and that one is named.
    assert!(!decls.is_empty(), "no rules parsed out of {STDLIB}/");
    let with_rules: BTreeSet<&str> = decls.iter().map(|d| d.module.as_str()).collect();
    let silent: Vec<&String> = imports
        .keys()
        .filter(|m| m.as_str() != RULE_FREE_MODULE && !with_rules.contains(m.as_str()))
        .collect();
    assert!(
        silent.is_empty(),
        "modules that parsed to no rules at all: {silent:?} \
         (only {RULE_FREE_MODULE} declares none)"
    );

    let t0 = Instant::now();
    let mut activated: BTreeSet<usize> = BTreeSet::new();
    let (mut firings, mut solved) = (0usize, 0usize);
    for path in programs() {
        let ran = run(&path, &imports);
        firings += ran.fired.len();
        solved += 1;
        for (rule, arity) in &ran.fired {
            activated.extend(resolve(rule, *arity, &ran, &decls, &by_name));
        }
    }
    let took = t0.elapsed();

    // Which *program* activated a rule is the census's to report; here the
    // only interesting set is the one nothing reached.
    let zero: Vec<String> = decls
        .iter()
        .enumerate()
        .filter(|(i, _)| !activated.contains(i))
        .map(|(_, d)| d.key())
        .collect();
    assert!(
        zero.is_empty(),
        "{} of {} stdlib rules are activated by no program under {SUITE}/ \
         — a rule with no test is not tested, it is merely not contradicted:\n  {}\n\
         (write one under {SUITE}/stdlib/<module>/; \
          `python3 utils/stdlib_census.py -k {SUITE}/stdlib` is the same census with the numbers)",
        zero.len(),
        decls.len(),
        zero.join("\n  "),
    );
    assert!(
        firings > 0,
        "{solved} programs solved and nothing fired — the event sink is not recording"
    );
    eprintln!(
        "{} rules, all activated; {solved} programs, {firings} firings, {:.2} s",
        decls.len(),
        took.as_secs_f64()
    );
}

/// **A program that states nothing is not a test.**
///
/// `ein test` reports it — `(no expect)`, and a selection of only such files
/// exits 2 — but reporting is what a person reads and this directory is
/// swept. A fixture whose `:expect` was deleted in a refactor would otherwise
/// load, run, and pass forever.
#[test]
fn every_program_states_an_expectation() {
    let silent: Vec<String> = programs()
        .into_iter()
        .filter(|path| {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let text = std::fs::read_to_string(path).expect("program text");
            let forms = parse(&mut ast, &text, path.to_str())
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let kb = load_query(&mut ast, &mut terms, &forms, path.parent(), 0)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            !kb.program()
                .queries
                .iter()
                .any(|q| ein_infer::query_value(&ast, q, "expect").is_some())
        })
        .map(|p| {
            p.strip_prefix(repo_root())
                .unwrap_or(&p)
                .display()
                .to_string()
        })
        .collect();
    assert!(
        silent.is_empty(),
        "programs under {SUITE}/ that state no expectation:\n  {}",
        silent.join("\n  "),
    );
}
