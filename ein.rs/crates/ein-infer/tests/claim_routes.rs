//! **A claim is decided alike by its least question, by the description and
//! by the oracle** — M1f
//! [S1f.6.27](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.27_ein_test_by_the_cheapest_route.md)'s
//! T1f.6.27.2, the test that exists before `ein test` asks the question.
//!
//! `ein test` reads a file's claims by their shapes and asks the least
//! question that can decide them ([`read_out::least`]): `--single` for
//! claims of at most one class over one part of `R`, `-n k+1` for claims of
//! `k`, and the description ([`read_out::describing`]) for a `(solve …
//! :expect …)` and for claims over more than one part. What that question
//! leaves undecided is asked the describing one, and [`read_out::stands`]
//! says which of the two reports stands. Held here, on every corpus file and
//! register witness whose cube the oracle enumerates, for every `(query …)`
//! the program asks, over each of the three parts of `R`:
//!
//! - **the oracle's word.** The query's classes over the part are read off
//!   the oracle's cube. Three claims are written from them: the one that
//!   lists exactly those classes, which holds; one that lists one class
//!   fewer, and one that lists one class more, `(model)`, which fail;
//! - **the least question's word**, where it decides the claim, **is the
//!   oracle's**, and so is what stands after the fall-back, which always
//!   decides;
//! - **the description's word is the oracle's**, every claim of the program
//!   read off one describing run;
//! - **the same at a budget that stops most runs**: a run that decides a
//!   claim there decides it as the oracle does, and what it leaves is *not
//!   checked*. This is where a claim is refuted by what a stopped run found
//!   ([`explore::Options::refute`]).
//!
//! So a claim's outcome under the route `ein test` takes can differ from its
//! outcome under the description only by being decided where the
//! description was not, never between held and failed. And `(solve …
//! :expect (false))`, which is asked the describing question, is held to
//! root's node on every program: it holds exactly when the oracle's root is
//! dead, at any budget that reaches root.
//!
//! ```text
//! cargo test -p ein-infer --test claim_routes
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::expect::Outcome;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out::{self, Stands};
use ein_infer::region::{self, Outcome as Cube};
use ein_ir::expect::Over;
use ein_ir::{Ast, load, parse};

fn loaded(src: &str, dir: Option<&Path>) -> Option<(Kb, Terms, Ast)> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, None).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, dir).ok()?;
    Some((kb, terms, ast))
}

fn corpus_and_register() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
}

/// The sweep runs a program a thread, so each run inside it takes one.
fn jobs() -> usize {
    1
}

fn threads() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().min(16))
}

// ── The program, less what it claims ──────────────────────────────────

/// The byte after the s-expression that starts at `i`: a parenthesised form,
/// a string, or a bare token. Comments and strings inside a form are
/// honoured.
fn skip_sexpr(b: &[u8], mut i: usize) -> usize {
    match b.get(i) {
        Some(b'(') => {
            let mut depth = 0usize;
            while i < b.len() {
                match b[i] {
                    b';' => {
                        while i < b.len() && b[i] != b'\n' {
                            i += 1;
                        }
                        continue;
                    }
                    b'"' => i = skip_sexpr(b, i) - 1,
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            return i + 1;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            i
        }
        Some(b'"') => {
            i += 1;
            while i < b.len() && b[i] != b'"' {
                i += if b[i] == b'\\' { 2 } else { 1 };
            }
            (i + 1).min(b.len())
        }
        _ => {
            while i < b.len() && !b[i].is_ascii_whitespace() && b[i] != b'(' && b[i] != b')' {
                i += 1;
            }
            i
        }
    }
}

/// `src` with every top-level `(query …)` form taken out and the `:expect`
/// of its top-level `(solve …)` with it: the program, stating no claim and
/// asking nothing.
fn unclaimed(src: &str) -> String {
    let b = src.as_bytes();
    let mut out = String::new();
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b';' => {
                let start = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                out.push_str(&src[start..i]);
            }
            b'(' => {
                let end = skip_sexpr(b, i);
                let form = &src[i..end];
                let head: String = form[1..]
                    .chars()
                    .take_while(|c| !c.is_whitespace() && *c != ')' && *c != '(')
                    .collect();
                match head.as_str() {
                    "query" => {}
                    "solve" => out.push_str(&without_expect(form)),
                    _ => out.push_str(form),
                }
                i = end;
            }
            _ => {
                let end = skip_sexpr(b, i).max(i + 1);
                out.push_str(&src[i..end]);
                i = end;
            }
        }
    }
    out
}

/// A `(solve …)` form with its `:expect <claim>` taken out.
fn without_expect(form: &str) -> String {
    let b = form.as_bytes();
    let mut i = 1usize;
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            c if c.is_ascii_whitespace() || c == b')' => i += 1,
            _ => {
                let end = skip_sexpr(b, i);
                if &form[i..end] == ":expect" {
                    let mut v = end;
                    while v < b.len() && b[v].is_ascii_whitespace() {
                        v += 1;
                    }
                    let after = skip_sexpr(b, v);
                    return format!("{}{}", &form[..i], &form[after..]);
                }
                i = end;
            }
        }
    }
    form.to_string()
}

// ── The claims written from the oracle's classes ──────────────────────

/// `(false)`, `(model …)` or `(or (model …) …)`: the claim that lists
/// `classes`, each a class's facts.
fn claim_of(classes: &[Vec<String>]) -> String {
    let model = |f: &Vec<String>| {
        if f.is_empty() {
            "(model)".to_string()
        } else {
            format!("(model {})", f.join(" "))
        }
    };
    match classes {
        [] => "(false)".to_string(),
        [one] => model(one),
        many => format!(
            "(or {})",
            many.iter().map(model).collect::<Vec<_>>().join(" ")
        ),
    }
}

fn over_word(o: Over) -> &'static str {
    match o {
        Over::All => "all",
        Over::Minimal => "minimal",
        Over::Maximal => "maximal",
    }
}

const OVERS: [Over; 3] = [Over::All, Over::Minimal, Over::Maximal];

/// One claim written for a program: the query it is of, the claim, and what
/// the oracle says of it.
struct Written {
    goal: String,
    over: Over,
    claim: String,
    /// The classes it lists are the oracle's, as a set.
    holds: bool,
    kind: &'static str,
}

impl Written {
    fn query(&self) -> String {
        format!(
            "(query :goal {} :over {} :expect {})\n",
            self.goal,
            over_word(self.over),
            self.claim
        )
    }
}

/// What the run of one question says of the program's `at`-th query claim,
/// and of what it found.
struct Said {
    outcome: Outcome,
    exact: bool,
    mode: Mode,
}

/// Ask `mode` of `src`, deciding claims as `ein test` asks, and read what it
/// says of each query's claim, in order. `None` where the question is
/// refused for a goal the compiler rejects.
fn ask(src: &str, dir: Option<&Path>, mode: Mode, budget: u64) -> Option<Vec<Said>> {
    ask_of(loaded(src, dir)?, src, mode, budget)
}

/// [`ask`], of a program already loaded and not yet run.
fn ask_of(
    (mut kb, mut terms, ast): (Kb, Terms, Ast),
    src: &str,
    mode: Mode,
    budget: u64,
) -> Option<Vec<Said>> {
    let mut opts = explore::Options::new(mode);
    opts.jobs = jobs();
    opts.budget = budget;
    opts.refute = true;
    let out = match read_out::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(out) => out,
        Err(ein_infer::SolveError::Compile(_)) => return None,
        Err(e) => panic!("{mode:?}: {e}\n{src}"),
    };
    Some(
        out.queries
            .iter()
            .map(|q| Said {
                outcome: q.claim.as_ref().expect("every query claims").outcome,
                exact: q.exact,
                mode,
            })
            .collect(),
    )
}

/// What `ein test` reads off the two questions of a file that states one
/// query claim: the least question's word, then the describing one's where
/// the first left the claim undecided, and which stands.
fn tested(src: &str, dir: Option<&Path>, budget: u64) -> Option<(Said, Option<Said>, Outcome)> {
    let program = loaded(src, dir)?;
    let (least, describing) = (
        read_out::least(&program.0, &program.2),
        read_out::describing(&program.0, &program.2),
    );
    let first = ask_of(program, src, least, budget)?
        .pop()
        .expect("one claim");
    if least == describing || first.outcome != Outcome::NotChecked {
        let outcome = first.outcome;
        return Some((first, None, outcome));
    }
    let second = ask(src, dir, describing, budget)?.pop().expect("one claim");
    let report = |o: Outcome| ein_infer::expect::Report {
        outcome: o,
        lines: Vec::new(),
    };
    let stands = read_out::stands(&report(first.outcome), &report(second.outcome))
        .unwrap_or_else(|e| panic!("{e}\n{src}"));
    let outcome = match stands {
        Stands::First => first.outcome,
        Stands::Fallback => second.outcome,
    };
    Some((first, Some(second), outcome))
}

/// What the sweep found on one program.
#[derive(Default)]
struct Swept {
    failures: Vec<String>,
    counted: BTreeMap<&'static str, usize>,
}

/// One program of the sweep: the module's four claims about it.
fn sweep(path: &Path) -> Swept {
    let mut swept = Swept::default();
    let Swept { failures, counted } = &mut swept;
    let mut count = |k: &'static str| *counted.entry(k).or_default() += 1;
    let oracle = region::Options {
        jobs: jobs(),
        ..region::Options::default()
    };
    // A labelled block, so that a program the sweep leaves is a `break`.
    'program: {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        let dir = path.parent();
        let Ok(src) = std::fs::read_to_string(path) else {
            break 'program;
        };
        let Some((kb, _, ast)) = loaded(&src, dir) else {
            break 'program;
        };
        // The goals the program asks, as written.
        let mut goals: Vec<String> = kb
            .program()
            .queries
            .iter()
            .filter_map(|q| ein_infer::goal_written(&ast, q))
            .map(|g| ein_ir::dump::dump_compact(&ast, g))
            .collect();
        goals.sort();
        goals.dedup();
        drop(kb);
        let base = unclaimed(&src);
        // The probe: every goal over every part, claiming nothing.
        let probes: Vec<(String, Over)> = goals
            .iter()
            .flat_map(|g| OVERS.iter().map(move |&o| (g.clone(), o)))
            .collect();
        let mut probe = base.clone();
        for (g, o) in &probes {
            probe.push_str(&format!("\n(query :goal {g} :over {})", over_word(*o)));
        }
        let Some((mut kb, mut terms, ast)) = loaded(&probe, dir) else {
            count("the program less its claims does not load");
            break 'program;
        };
        let Ok(Cube::Region(r)) = region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            break 'program;
        };
        count("programs the oracle reaches");
        let root_dead = r.root_dead;
        // With no candidate R is root's node or nothing, the three parts are
        // one and no budget stops a run before root: the claims are written
        // over all of R, and asked once.
        let rooted = r.n() == 0;
        let describe = Mode {
            which: Which::All,
            ask: Ask::Describe,
        };
        let cube = explore::of_cube(describe, *r);
        let read = match read_out::of_answer(
            &mut kb,
            &mut terms,
            &ast,
            cube,
            &explore::Options::new(describe),
        ) {
            Ok(read) => read,
            Err(ein_infer::SolveError::Compile(_)) => {
                count("a goal the compiler refuses");
                break 'program;
            }
            Err(e) => panic!("{rel}: the oracle's read-out: {e}"),
        };
        drop(kb);

        // `(solve … :expect (false))`: root's node, at any budget.
        let falsed = match solve_close(&base) {
            Some(at) => {
                let mut b = base.clone();
                b.insert_str(at, " :expect (false)");
                b
            }
            None => format!("{base}\n(solve :expect (false))\n"),
        };
        for budget in [u64::MAX, 8] {
            let Some((mut kb, mut terms, ast)) = loaded(&falsed, dir) else {
                count("(false): does not load");
                break;
            };
            let mut opts = explore::Options::new(read_out::least(&kb, &ast));
            if opts.mode != describe {
                failures.push(format!("{rel}: a solve claim is asked {:?}", opts.mode));
            }
            opts.jobs = jobs();
            opts.budget = budget;
            opts.refute = true;
            let got = read_out::run(&mut kb, &mut terms, &ast, &opts)
                .unwrap_or_else(|e| panic!("{rel}: (false): {e}"));
            let outcome = got.claim.expect("the solve claim").outcome;
            let want = if root_dead {
                Outcome::Held
            } else {
                Outcome::Failed
            };
            let exhausted = got.answer.exhausted;
            match outcome {
                Outcome::NotChecked if budget == u64::MAX => {
                    failures.push(format!("{rel}: (false) is not checked, unbudgeted"))
                }
                Outcome::NotChecked => count("(false): not checked at budget 8"),
                o if o != want => failures.push(format!(
                    "{rel}: (false) is {o:?} at budget {budget}, and the oracle's root is {}",
                    if root_dead { "dead" } else { "alive" }
                )),
                Outcome::Failed if !exhausted => count("(false): refuted at root by a stopped run"),
                _ => count("(false): decided"),
            }
        }
        if probes.is_empty() {
            break 'program;
        }
        count("programs with a query");

        // The claims: the oracle's classes, one fewer, one more.
        let mut written: Vec<Written> = Vec::new();
        for ((goal, over), q) in probes.iter().zip(&read.queries) {
            assert!(
                q.exact && q.as_asked,
                "{rel}: the oracle's classes are exact"
            );
            if rooted && *over != Over::All {
                continue;
            }
            let classes: Vec<Vec<String>> = q.classes.iter().map(|c| c.facts.clone()).collect();
            let w = |claim: &[Vec<String>], holds: bool, kind: &'static str| Written {
                goal: goal.clone(),
                over: *over,
                claim: claim_of(claim),
                holds,
                kind,
            };
            written.push(w(&classes, true, "the oracle's classes"));
            if !classes.is_empty() {
                written.push(w(&classes[..classes.len() - 1], false, "one class fewer"));
            }
            if !classes.iter().any(Vec::is_empty) {
                let mut more = classes.clone();
                more.push(Vec::new());
                written.push(w(&more, false, "one class more"));
            }
        }
        // Each claim alone, as `ein test` reads a file that states it. A
        // claim the loader refuses, `(model)` beside a goal that must bind,
        // is left out of the sweep.
        let mut alone: Vec<(Said, Option<Said>, Outcome)> = Vec::new();
        written.retain(
            |w| match tested(&format!("{base}\n{}", w.query()), dir, u64::MAX) {
                Some(t) => {
                    alone.push(t);
                    true
                }
                None => {
                    count("a written claim the loader refuses");
                    false
                }
            },
        );
        // The description's word, every claim off one run.
        let mut all = base.clone();
        for w in &written {
            all.push('\n');
            all.push_str(&w.query());
        }
        let Some(described) = ask(&all, dir, describe, u64::MAX) else {
            count("a goal the compiler refuses");
            break 'program;
        };
        for ((w, d), unbudgeted) in written.iter().zip(&described).zip(alone) {
            let want = if w.holds {
                Outcome::Held
            } else {
                Outcome::Failed
            };
            let tag = format!(
                "{rel}: {} over {}, {} — {}",
                w.goal,
                over_word(w.over),
                w.kind,
                w.claim
            );
            if d.outcome != want {
                failures.push(format!(
                    "{tag}: the description says {:?}, the oracle {want:?}",
                    d.outcome
                ));
            }
            let one = format!("{base}\n{}", w.query());
            let stopped = if rooted { None } else { tested(&one, dir, 8) };
            for (budget, (first, second, stands)) in [(u64::MAX, Some(unbudgeted)), (8, stopped)]
                .into_iter()
                .filter_map(|(b, t)| t.map(|t| (b, t)))
            {
                let budgeted = budget != u64::MAX;
                if first.outcome != Outcome::NotChecked && first.outcome != want {
                    failures.push(format!(
                        "{tag}: {} says {:?} at budget {budget}, the oracle {want:?}",
                        first.mode.flags(),
                        first.outcome
                    ));
                }
                if let Some(s) = &second
                    && s.outcome != Outcome::NotChecked
                    && s.outcome != want
                {
                    failures.push(format!(
                        "{tag}: the fall-back {} says {:?} at budget {budget}, the oracle {want:?}",
                        s.mode.flags(),
                        s.outcome
                    ));
                }
                if !budgeted && stands != want {
                    failures.push(format!(
                        "{tag}: what stands is {stands:?}, the oracle {want:?}"
                    ));
                }
                if budgeted {
                    if stands == Outcome::NotChecked {
                        count("at budget 8: not checked");
                    } else {
                        count("at budget 8: decided");
                    }
                    if first.outcome == Outcome::Failed && !first.exact {
                        count("at budget 8: refuted by what a stopped run found");
                    }
                    continue;
                }
                count("claims");
                count(match first.mode.ask {
                    Ask::Single => "asked --single",
                    Ask::Find(_) => "asked -n k+1",
                    _ => "asked the description",
                });
                match (&second, first.outcome, first.exact) {
                    (Some(_), _, _) => count("fell back to the description"),
                    (None, Outcome::Failed, false) => {
                        count("refuted by a state found, nothing exhausted")
                    }
                    (None, Outcome::Held, _) => count("held by the least question"),
                    (None, _, _) => count("failed by the least question, exhausted"),
                }
                if w.over != Over::All {
                    count("claims over a line");
                }
            }
        }
    }
    swept
}

#[test]
fn every_claim_is_decided_as_the_oracle_decides_it() {
    let files = corpus_and_register();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let found: std::sync::Mutex<Vec<(usize, Swept, std::time::Duration)>> =
        std::sync::Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..threads() {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(path) = files.get(i) else {
                        break;
                    };
                    let t0 = std::time::Instant::now();
                    let swept = sweep(path);
                    found.lock().expect("no sweep panics holding it").push((
                        i,
                        swept,
                        t0.elapsed(),
                    ));
                }
            });
        }
    });
    let mut found = found.into_inner().expect("every sweep is done");
    found.sort_by_key(|(i, _, _)| *i);
    let mut failures: Vec<String> = Vec::new();
    let mut counted: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, swept, took) in found {
        failures.extend(swept.failures);
        for (k, v) in swept.counted {
            *counted.entry(k).or_default() += v;
        }
        if took.as_secs() >= 5 {
            eprintln!("{:>6.1} s  {}", took.as_secs_f64(), files[i].display());
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    eprintln!("{counted:#?}");
    let n = |k: &str| counted.get(k).copied().unwrap_or(0);
    // Not vacuous: every row of the table occurred, and each way a claim is
    // decided. The counts of 2026-11-13 are in the stage's record.
    assert!(n("programs the oracle reaches") >= 300, "{counted:?}");
    assert!(
        n("claims") >= 1000 && n("claims over a line") >= 400,
        "{counted:?}"
    );
    assert!(
        n("asked --single") >= 500 && n("asked -n k+1") >= 100,
        "{counted:?}"
    );
    assert!(
        n("held by the least question") >= 300
            && n("failed by the least question, exhausted") >= 100
            && n("refuted by a state found, nothing exhausted") >= 10
            && n("fell back to the description") >= 10,
        "{counted:?}"
    );
    assert!(
        n("at budget 8: decided") >= 100
            && n("at budget 8: not checked") >= 10
            && n("at budget 8: refuted by what a stopped run found") >= 1,
        "{counted:?}"
    );
    assert!(
        n("(false): decided") >= 300 && n("(false): refuted at root by a stopped run") >= 5,
        "{counted:?}"
    );
}

/// The byte of the `)` that closes the first top-level `(solve …)` form.
fn solve_close(src: &str) -> Option<usize> {
    let b = src.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'(' => {
                let end = skip_sexpr(b, i);
                let head: String = src[i + 1..end]
                    .chars()
                    .take_while(|c| !c.is_whitespace() && *c != ')' && *c != '(')
                    .collect();
                if head == "solve" {
                    return Some(end - 1);
                }
                i = end;
            }
            _ => i = skip_sexpr(b, i).max(i + 1),
        }
    }
    None
}

/// The surgery reads a program's forms as the loader does: a `(query …)` in
/// a comment or a string stays, a claim's nested forms go with it, and the
/// `(solve …)` keeps its lattice.
#[test]
fn a_program_less_its_claims_keeps_everything_else() {
    let src = "; (query :goal (p ?x)) in a comment\n\
               (relation p T :why \"a (query in a string\")\n\
               (p a)\n\
               (solve :no-hypothesis (p) ; a comment (\n\
                 :expect (region (model (p a))))\n\
               (query :goal (p ?x) :expect (or (model (p a)) (model)))\n\
               (p b)\n";
    let got = unclaimed(src);
    assert_eq!(
        got,
        "; (query :goal (p ?x)) in a comment\n\
         (relation p T :why \"a (query in a string\")\n\
         (p a)\n\
         (solve :no-hypothesis (p) ; a comment (\n\
                 )\n\
         \n\
         (p b)\n"
    );
    assert_eq!(solve_close(&got).map(|i| &got[i..i + 1]), Some(")"));
    assert_eq!(claim_of(&[]), "(false)");
    assert_eq!(claim_of(&[vec![]]), "(model)");
    assert_eq!(
        claim_of(&[vec!["(p a)".to_string()], vec![]]),
        "(or (model (p a)) (model))"
    );
}

/// **The question is a function of the claims' shapes**, row by row of the
/// stage's table, and of nothing else: the same program asks another
/// question when its claims change, and no option is read.
#[test]
fn the_least_question_is_read_off_the_claims_shapes() {
    const BODY: &str = "(relation p T :why \"{?1} is p\")\n(relation q T :why \"{?1} is q\")\n\
                        (hrule guess (?R) :match (thing ?x) :assert (?R ?x) :why \"suppose {?x}\")\n\
                        (relation thing T :why \"{?1} is a thing\")\n(thing a)\n(thing b)\n";
    let solve = "(solve :hrules (guess (p)))\n";
    let mode = |which, ask| Mode { which, ask };
    let describe = mode(Which::All, Ask::Describe);
    let questions = |claims: &str| -> (Mode, Mode) {
        let src = format!("{BODY}{claims}");
        let (kb, _, ast) = loaded(&src, None).unwrap_or_else(|| panic!("does not load:\n{src}"));
        (read_out::least(&kb, &ast), read_out::describing(&kb, &ast))
    };
    let q =
        |over: &str, claim: &str| format!("(query :goal (p ?x) :over {over} :expect {claim})\n");
    let one = "(model (p a))";
    let two = "(or (model (p a)) (model (p b)))";
    let three = "(or (model (p a)) (model (p b)) (model (p a) (p b)))";
    for (claims, least, describing) in [
        // One class, and none, over all of R: is there exactly this.
        (
            format!("{solve}{}", q("all", one)),
            mode(Which::All, Ask::Single),
            describe,
        ),
        (
            format!("{solve}{}", q("all", "(false)")),
            mode(Which::All, Ask::Single),
            describe,
        ),
        // k classes: these k and no (k+1)-th state.
        (
            format!("{solve}{}", q("all", two)),
            mode(Which::All, Ask::Find(3)),
            describe,
        ),
        (
            format!("{solve}{}{}", q("all", one), q("all", three)),
            mode(Which::All, Ask::Find(4)),
            describe,
        ),
        // Over one line: that line, the same way.
        (
            format!("{solve}{}", q("minimal", one)),
            mode(Which::Minimal, Ask::Single),
            mode(Which::Minimal, Ask::Describe),
        ),
        (
            format!("{solve}{}{}", q("maximal", two), q("maximal", one)),
            mode(Which::Maximal, Ask::Find(3)),
            mode(Which::Maximal, Ask::Describe),
        ),
        // Over more than one part: the description of all of R.
        (
            format!("{solve}{}{}", q("all", one), q("maximal", one)),
            describe,
            describe,
        ),
        (
            format!("{solve}{}{}", q("minimal", one), q("maximal", one)),
            describe,
            describe,
        ),
        // A solve claim, alone or beside a query's: the description.
        (
            "(solve :hrules (guess (p)) :expect (false))\n".to_string(),
            describe,
            describe,
        ),
        (
            format!(
                "(solve :hrules (guess (p)) :expect (region)){}",
                q("all", one)
            ),
            describe,
            describe,
        ),
        // A claim on a query with no goal is answered by no run: as before.
        (
            format!("{solve}(query :expect (model))\n"),
            describe,
            describe,
        ),
        (
            format!("{solve}(query :expect (model))\n{}", q("all", two)),
            mode(Which::All, Ask::Find(3)),
            describe,
        ),
    ] {
        assert_eq!(questions(&claims), (least, describing), "{claims}");
    }
    assert_eq!(describe.flags(), "describe");
    assert_eq!(mode(Which::All, Ask::Single).flags(), "--single");
    assert_eq!(mode(Which::All, Ask::Find(4)).flags(), "-n 4");
    assert_eq!(mode(Which::Maximal, Ask::Describe).flags(), "--maximal");
    assert_eq!(mode(Which::Minimal, Ask::Find(2)).flags(), "--minimal -n 2");

    // What stands of two reports of one claim.
    let report = |o: Outcome| ein_infer::expect::Report {
        outcome: o,
        lines: Vec::new(),
    };
    use Outcome::{Failed, Held, NotChecked};
    for (first, fallback, want) in [
        (NotChecked, Held, Some(Stands::Fallback)),
        (NotChecked, Failed, Some(Stands::Fallback)),
        (NotChecked, NotChecked, Some(Stands::Fallback)),
        (Held, Held, Some(Stands::Fallback)),
        (Failed, Failed, Some(Stands::Fallback)),
        (Held, NotChecked, Some(Stands::First)),
        (Failed, NotChecked, Some(Stands::First)),
        // Two questions deciding one claim differently is no outcome.
        (Held, Failed, None),
        (Failed, Held, None),
    ] {
        assert_eq!(
            read_out::stands(&report(first), &report(fallback)).ok(),
            want,
            "{first:?} then {fallback:?}"
        );
    }
}
