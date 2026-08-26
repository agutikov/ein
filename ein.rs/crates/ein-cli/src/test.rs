//! `ein test` — the expectations a program states about itself, run.
//!
//! The fourth subcommand, and M1c
//! [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test).
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
//! gave a `(query …)` a way to state its own answer; this is the runner that
//! turns a directory of them into a status code, so that **nothing reads
//! output**. Checking that a rule works used to mean running `solve` and
//! looking, or diffing against a golden.
//!
//! ## Three things it does that `solve` does not
//!
//! **It exhausts.** An expectation is a claim about the *exhausted* answer —
//! `Solution` means one model *and no other*, `(or …)` with k disjuncts means k
//! *and no k+1-th*, `(false)` means every branch died — so a search that
//! stopped early establishes a lower bound on `k` and confirms none of them
//! ([`ein_infer::expect::Outcome::NotChecked`]). `solve` defaults to `-n 1`
//! and is right to: it is asked for an answer. `test` is asked whether a claim
//! holds, and there is no honest way to answer that from a stopped search, so
//! there is no `-n` here and `--exhaustive` is not a flag but the behaviour.
//!
//! **It runs only what the expectations need.** A query with no `:expect`
//! states nothing and is never solved — the one load that finds this out is
//! the whole of its cost. That is the acceptance criterion the stage wrote as
//! "a file with only `:derives` never enters the search": on a corpus of
//! stdlib programs the expensive thing is a query with an open hypothesis
//! space, and a file that does not ask a question must not pay for one.
//!
//! **A failure names the fact, the rule, and the models.** `solve`'s
//! `:expect FAILED` block is the same report — this command prints it under a
//! file and a query header — and T1c.1.3.3 grew it two lines for both: a
//! surplus fact carries the derivation that put it there, which is the step
//! after "there is an extra fact here" and the step at which
//! `disjunctive-prune`'s guard bug was actually found; and a `k` mismatch
//! carries every model projected through the query's own `:goal`, so "I found
//! two" says *which* two.
//!
//! ## Exit codes, and the one that differs from `solve`
//!
//! | code | what it means |
//! |---|---|
//! | 0 | every expectation held, and at least one was checked |
//! | 1 | an expectation is **false**, or could not be checked |
//! | 2 | a load error, a usage error, a budget abort — or nothing to check |
//!
//! `solve` exits **1** on a load error, because that is ein.py's code for it
//! and the port kept it. Here 1 is taken: it means *a claim is false*. A test
//! runner that cannot tell a broken file from a false claim is precisely the
//! failure T1c.1.3.5 is written against, so a load error takes 2 — the code
//! that already means "this run is not a verdict" (a usage error, a budget
//! abort). **2 dominates 1** in the summary for the same reason: if any file
//! failed to load, "every expectation was checked and some are false" would be
//! a lie about the run.
//!
//! ## `--json-report`, and why it is not `--json-summary`
//!
//! The runner's own output, machine-readable: **one row per `(query …)` of
//! the whole selection**, whether or not it claims anything. `--json-summary`
//! is a *run*'s summary — verdict, counters, config — and is refused over a
//! selection of more than one run for the reason every artefact flag is: it
//! names one path. A report is the other shape. It has no run to be more than
//! one of, so it takes any selection, and one invocation over the three
//! corpus roots is the whole census.
//!
//! What it publishes that nothing else could: the **shape** of a claim. M1c's
//! rule is that *nothing reads output*, and there was no other way to ask a
//! corpus what fraction of it states a closure claim — `:expect` is a query
//! keyword, and a grep cannot tell a keyword from a comment about one. That
//! is M1d
//! [S1d.4.1](../../../../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)'s
//! transport, and `utils/closure_census.py` is what reads it.
//!
//! Additive, in the sense the other artefact flags are: stdout, stderr, the
//! exit code and what is solved are identical with the flag and without it. A
//! query that states nothing is still not solved, and its row says so.
//!
//! ## Not a test framework
//!
//! There is no setup, teardown, fixture, tag, skip or parameterisation, and
//! there will not be. `ein test` evaluates the claims a program already
//! carries. If a rule needs a framework to be tested, the interesting finding
//! is about the rule.

use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::ArgMatches;
use ein_core::Terms;
use ein_infer::expect::Outcome;
use ein_infer::solve::{NoDumper, SolveError, SolveOptions, solve};
use ein_ir::Ast;
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

/// The artefact flags, as in [`crate::solve`]: each names **one** path, so
/// each is incompatible with a selection that is more than one run.
const ONE_PATH_FLAGS: [&str; 2] = ["events", "json-summary"];

/// `--json-report`'s version. **Not** `ein-summary/1`'s: a report is one row
/// per query of a selection, a summary is one run, and a consumer that read
/// the same version marker on both would be right to expect the same fields.
const REPORT_SCHEMA: &str = "ein-test-report/1";

/// What one query came to — [`Outcome`] plus the two ways a run can fail to
/// produce one at all.
///
/// Kept as a type rather than as an exit code, because the roll-up to a file's
/// label has to distinguish `FAILED` from `NOT CHECKED` and both are 1.
///
/// **The declaration order is the dominance order**, which `max` over a file's
/// queries reads: an `Error` beats everything, because a run that did not
/// happen is not a verdict about the file; a `Failed` beats a `NotChecked`,
/// because a claim shown to be false is a stronger thing to put in the status
/// column than one nobody could check.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Came {
    Held,
    NotChecked,
    Failed,
    /// A load error, a budget abort, a compile or saturation failure: the run
    /// did not happen, so the file is neither green nor refuted.
    Error,
}

impl Came {
    fn label(self) -> &'static str {
        match self {
            Came::Held => "ok",
            Came::Failed => "FAILED",
            Came::NotChecked => "NOT CHECKED",
            Came::Error => "ERROR",
        }
    }

    /// The word used *inside* a file's report, where "ok" would be an odd
    /// thing to say about one query of three.
    fn verb(self) -> &'static str {
        match self {
            Came::Held => "holds",
            other => other.label(),
        }
    }

    /// The machine spelling — `--json-report`'s `outcome`. Lower-case and
    /// hyphenated because it is a key, not a status column, and a consumer
    /// that has to lower-case a label is a consumer reading output.
    fn key(self) -> &'static str {
        match self {
            Came::Held => "held",
            Came::Failed => "failed",
            Came::NotChecked => "not-checked",
            Came::Error => "error",
        }
    }
}

/// How much of a passing run to print.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Volume {
    /// `-q` — the summary line, and whatever was not `ok`.
    Quiet,
    /// One line per file, plus every failure's detail.
    Normal,
    /// `-v` — one line per *query*, held ones included.
    Verbose,
}

/// The run's counters. Every line of the summary is one of these.
#[derive(Default)]
struct Tally {
    files: usize,
    /// Files carrying no `:expect` at all — reported, never counted as a pass.
    silent: usize,
    held: usize,
    failed: usize,
    not_checked: usize,
    /// Load errors, budget aborts: the run did not happen.
    errors: usize,
}

impl Tally {
    fn checked(&self) -> usize {
        self.held + self.failed + self.not_checked
    }

    /// 2 dominates 1 — see the module header.
    fn code(&self) -> i32 {
        if self.errors > 0 || self.checked() == 0 {
            return 2;
        }
        i32::from(self.failed + self.not_checked > 0)
    }

    fn line(&self, elapsed: f64) -> String {
        let plural = |n: usize, one: &str| {
            if n == 1 {
                one.to_string()
            } else {
                format!("{one}s")
            }
        };
        let mut s = format!(
            "{} {}, {} {}: {} held, {} FAILED, {} not checked, {} {}  ({:.2} s)",
            self.files,
            plural(self.files, "file"),
            self.checked(),
            plural(self.checked(), "expectation"),
            self.held,
            self.failed,
            self.not_checked,
            self.errors,
            plural(self.errors, "error"),
            elapsed,
        );
        if self.silent > 0 {
            s.push_str(&format!(
                "; {} {} no expectations",
                self.silent,
                if self.silent == 1 {
                    "file states"
                } else {
                    "files state"
                },
            ));
        }
        s
    }
}

/// What one `(query …)` **claims**, read off the loaded program.
///
/// Not the comparison — that is [`Outcome`] — but the claim's *shape*, which
/// is the one thing about `:expect` no surface published before and the
/// column M1d S1d.4.1's census is built on. `models` is the number of
/// `(model …)` disjuncts: 0 for `(false)`, 1 for a bare `(model …)`, and for
/// `(or …)` the k it asserts.
struct Claim {
    shape: &'static str,
    models: usize,
    /// Facts listed, summed over the disjuncts — half the **write cost** of a
    /// closure claim, and the half that is a fact about this file rather than
    /// a counterfactual.
    facts: usize,
    /// …of which `(not …)`. Kept apart because a listed negative is checked
    /// for *presence* and closes nothing, so it costs a line and buys a
    /// different thing.
    negated: usize,
    /// The relations the expectation names, sorted — *naming a relation
    /// closes it*, so this is the set whose extent the claim pins.
    relations: Vec<String>,
}

/// What the run made of a claim — the fields only a solve can fill.
struct Ran {
    verdict: String,
    /// **Models**, `Verdict::k` — what a claim is a claim about.
    k: usize,
    /// What the *search* recorded, which since M1d S1d.2.6 is a different
    /// number on an `Open` state and the same one everywhere else.
    solution_nodes: u64,
    exhausted: bool,
    layers: u64,
    enterings: u64,
    /// Wall clock for this query's solve. **The one field that does not
    /// reproduce**, which is why nothing digests a report: it is a measurement
    /// surface, not a golden.
    ms: f64,
}

/// One row of `--json-report` — one `(query …)`, or one file that never
/// became a program.
struct Row {
    path: String,
    /// 1-based, and **0 for a file that did not load**: a load error is a fact
    /// about the file, and inventing a query number for it would be inventing
    /// a query.
    query: usize,
    queries: usize,
    goal: Option<String>,
    /// The relations the `:goal` asks about, from [`ein_ir::pattern_relations`]
    /// — what an expectation *must* close, and the first factor of the write
    /// cost of the claim this query does not yet carry.
    goal_relations: Vec<String>,
    claim: Option<Claim>,
    outcome: &'static str,
    ran: Option<Ran>,
}

impl Row {
    fn json(&self) -> Json {
        let mut pairs = vec![
            ("path", Json::str(&self.path)),
            ("query", Json::int(self.query as i64)),
            ("queries", Json::int(self.queries as i64)),
            ("goal", self.goal.as_deref().map_or(Json::Null, Json::str)),
            (
                "goal_relations",
                Json::Array(self.goal_relations.iter().map(Json::str).collect()),
            ),
            ("outcome", Json::str(self.outcome)),
        ];
        pairs.push((
            "expect",
            match &self.claim {
                None => Json::Null,
                Some(c) => Json::obj(vec![
                    ("shape", Json::str(c.shape)),
                    ("models", Json::int(c.models as i64)),
                    ("facts", Json::int(c.facts as i64)),
                    ("negated", Json::int(c.negated as i64)),
                    (
                        "relations",
                        Json::Array(c.relations.iter().map(Json::str).collect()),
                    ),
                ]),
            },
        ));
        match &self.ran {
            None => pairs.push(("ran", Json::Null)),
            Some(r) => pairs.push((
                "ran",
                Json::obj(vec![
                    ("verdict", Json::str(&r.verdict)),
                    ("k", Json::int(r.k as i64)),
                    ("solution_nodes", Json::int(r.solution_nodes as i64)),
                    ("exhausted", Json::Bool(r.exhausted)),
                    ("layers", Json::int(r.layers as i64)),
                    ("enterings", Json::int(r.enterings as i64)),
                    ("ms", Json::Float(r.ms)),
                ]),
            )),
        }
        Json::obj(pairs)
    }
}

/// The claim a query states, or `None` — read from the **loaded** program.
///
/// The loader has already refused every shape `expect::parse` can, so a parse
/// failure here is an engine bug and the row simply carries no claim rather
/// than inventing one; [`check_query`] is where it is reported.
fn claim_of(ast: &Ast, query: &ein_core::Query) -> Option<Claim> {
    let node = ein_infer::query_value(ast, query, "expect")?;
    let e = ein_ir::expect::parse(ast, node).ok()?;
    let (mut facts, mut negated) = (0usize, 0usize);
    let mut relations: Vec<String> = Vec::new();
    for m in e.models() {
        for &f in &m.facts {
            let Ok(f) = ein_ir::expect::fact(ast, f) else {
                continue;
            };
            facts += 1;
            negated += usize::from(f.negated);
            if !relations.iter().any(|r| r == f.relation) {
                relations.push(f.relation.to_string());
            }
        }
    }
    relations.sort();
    Some(Claim {
        shape: match e {
            ein_ir::expect::Expectation::Contradiction => "false",
            ein_ir::expect::Expectation::One(_) => "model",
            ein_ir::expect::Expectation::Any(_) => "or",
        },
        models: e.models().len(),
        facts,
        negated,
        relations,
    })
}

/// The status column. Vocabulary shared with `solve`'s `:expect` line on
/// purpose: `NOT CHECKED` means there what it means here.
fn status(label: &str, path: &Path) -> String {
    format!("{label:<11}  {}", path.display())
}

/// The `.ein` files a path argument names — itself, or every `.ein` under it.
///
/// Recursive, and **sorted at every level**: a summary line whose composition
/// moves with the filesystem's readdir order is a summary line nobody can
/// diff. A file named *explicitly* is taken whatever its extension, so
/// `ein test x.einb` works the way every other subcommand takes a container;
/// a directory contributes `.ein` only, because `.einb` is a cache and a
/// directory holding both would run each program twice.
///
/// **A walk does not follow a symlinked directory**, which is `walkdir`'s
/// default and the reason it is: a link back up the tree is an infinite walk,
/// and a gate command that hangs is worse than one that misses a file nobody
/// asked it for. An explicitly named path is still followed — naming it is
/// asking for it.
fn collect(arg: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let meta = std::fs::metadata(arg).map_err(|e| format!("{}: {e}", arg.display()))?;
    if meta.is_file() {
        out.push(arg.to_path_buf());
        return Ok(());
    }
    if !meta.is_dir() {
        return Err(format!("{}: neither a file nor a directory", arg.display()));
    }
    let mut entries: Vec<PathBuf> = std::fs::read_dir(arg)
        .map_err(|e| format!("{}: {e}", arg.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for e in entries {
        let Ok(meta) = std::fs::symlink_metadata(&e) else {
            continue;
        };
        if meta.is_symlink() {
            continue;
        }
        if meta.is_dir() {
            collect(&e, out)?;
        } else if e.extension().is_some_and(|x| x == "ein") {
            out.push(e);
        }
    }
    Ok(())
}

pub fn run(m: &ArgMatches) -> i32 {
    let volume = if m.get_flag("quiet") {
        Volume::Quiet
    } else if m.get_flag("verbose") {
        Volume::Verbose
    } else {
        Volume::Normal
    };

    let mut files: Vec<PathBuf> = Vec::new();
    let args: Vec<&String> = m.get_many::<String>("path").expect("required").collect();
    for arg in &args {
        if let Err(e) = collect(Path::new(arg), &mut files) {
            eprintln!("error: {e}");
            return 2;
        }
    }
    // `ein test dir dir/x.ein` names one program twice, and a summary line that
    // counted it twice would be reporting the argv rather than the corpus.
    // determinism-ok: first occurrence wins, so the order is still the walk's.
    let mut seen: Vec<PathBuf> = Vec::with_capacity(files.len());
    files.retain(|p| {
        let key = std::fs::canonicalize(p).unwrap_or_else(|_| p.clone());
        let fresh = !seen.contains(&key);
        if fresh {
            seen.push(key);
        }
        fresh
    });
    if files.is_empty() {
        eprintln!(
            "error: no .ein files under {}",
            args.iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return 2;
    }
    // An artefact flag over a selection of more than one file is refused
    // before anything is written, exactly as `solve` refuses one over a file
    // of more than one query. The per-file check below is the other half:
    // with this one passed, a run carrying an artefact flag has exactly one
    // file, and that file must ask exactly one question.
    if files.len() > 1
        && let Some(flag) = ONE_PATH_FLAGS
            .iter()
            .find(|f| m.get_one::<String>(f).is_some())
    {
        eprintln!(
            "error: --{flag} names one path and this selection is {} files — \
             name one file, or drop the flag",
            files.len()
        );
        return 2;
    }

    let t0 = Instant::now();
    let mut tally = Tally::default();
    // Accumulated whether or not anyone asked for it — 197 rows over the whole
    // corpus, and a flag that changed what the run *did* would not be additive.
    let mut rows: Vec<Row> = Vec::new();
    for path in &files {
        tally.files += 1;
        check_file(path, m, volume, &mut tally, &mut rows);
    }
    if let Some(out) = m.get_one::<String>("json-report")
        && let Err(e) = write_report(out, &tally, &rows)
    {
        eprintln!("{e}");
    }
    println!("{}", tally.line(t0.elapsed().as_secs_f64()));
    // **Reported, never skipped past** — M1c's acceptance, in the shape this
    // command can fail in. A selection that checked nothing has said nothing
    // about anything, and a green exit there is the one result a test runner
    // must never produce. Only when nothing *errored*, though: a file that
    // failed to load has already said why, and diagnosing it a second time as
    // "carries no :expect" would be a wrong diagnosis of a real error.
    if tally.checked() == 0 && tally.errors == 0 {
        eprintln!("error: nothing to check — no (query …) in the selection carries an :expect");
    }
    tally.code()
}

/// `--json-report` — the rows, plus the tally the summary line prints.
///
/// Two properties borrowed from `--json-summary`, for its reasons: **additive**
/// (a file, never a stream and never the exit code) and **self-describing**
/// (field order fixed by construction, `schema` versioned). The third —
/// order-free — it does not need: rows are in walk order, and the walk is
/// sorted at every level, so a report diffs against yesterday's.
fn write_report(path: &str, tally: &Tally, rows: &[Row]) -> Result<(), String> {
    let doc = Json::obj(vec![
        ("schema", Json::str(REPORT_SCHEMA)),
        (
            "tally",
            Json::obj(vec![
                ("files", Json::int(tally.files as i64)),
                ("silent", Json::int(tally.silent as i64)),
                ("held", Json::int(tally.held as i64)),
                ("failed", Json::int(tally.failed as i64)),
                ("not_checked", Json::int(tally.not_checked as i64)),
                ("errors", Json::int(tally.errors as i64)),
            ]),
        ),
        ("rows", Json::Array(rows.iter().map(Row::json).collect())),
    ]);
    std::fs::write(path, dumps_indent(&doc) + "\n")
        .map_err(|e| format!("could not write {path}: {e}"))
}

/// One file: load it once to find out what it claims, then check each claim.
///
/// The planning load is what makes "only the work the expectations need runs"
/// true — it reads `Program::queries` and stops there for a file that states
/// nothing. It is not wasted on the common case either: a single-query file's
/// plan load *is* its run load, reused below.
///
/// Everything it finds goes into `tally`, which is where the exit code comes
/// from: a per-file code would have to be combined by a second rule, and the
/// counters already say more than the maximum of three integers does.
fn check_file(path: &Path, m: &ArgMatches, volume: Volume, tally: &mut Tally, rows: &mut Vec<Row>) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let first = rows.len();
    let Some(kb) = crate::common::load_any_query_or_exit(&mut ast, &mut terms, path, 0) else {
        println!("{}", status("ERROR", path));
        tally.errors += 1;
        // A file that did not load states nothing, because a claim is a
        // property of a *program*. Three `examples/broken/load/` fixtures
        // contain the token `:expect` and are here — they exist to be refused,
        // and counting them as claims would put the loader's own negatives in
        // the numerator of "what fraction of the corpus claims a model set".
        rows.push(Row {
            path: path.display().to_string(),
            query: 0,
            queries: 0,
            goal: None,
            goal_relations: Vec::new(),
            claim: None,
            outcome: Came::Error.key(),
            ran: None,
        });
        return;
    };
    let n_queries = kb.program().queries.len();
    let claims: Vec<usize> = (0..n_queries)
        .filter(|&i| ein_infer::query_value(&ast, &kb.program().queries[i], "expect").is_some())
        .collect();
    // Every query gets a row, claim or no claim: the census's question is a
    // *fraction*, and a report that listed only the numerator could not answer
    // it. Read off the planning load, which has already parsed all of them.
    // A file with no `(query …)` at all — every `stdlib/*.ein` is one — still
    // gets a row, so that the rows account for the selection file for file and
    // "how much of the corpus claims a model set" has a denominator it can
    // check rather than one it has to be told.
    if n_queries == 0 {
        rows.push(Row {
            path: path.display().to_string(),
            query: 0,
            queries: 0,
            goal: None,
            goal_relations: Vec::new(),
            claim: None,
            outcome: "no-query",
            ran: None,
        });
    }
    for (i, query) in kb.program().queries.iter().enumerate() {
        let goal = ein_infer::query_value(&ast, query, "goal");
        rows.push(Row {
            path: path.display().to_string(),
            query: i + 1,
            queries: n_queries,
            goal: goal.map(|g| ein_ir::dump_compact(&ast, g)),
            goal_relations: goal
                .map(|g| ein_ir::pattern_relations(&ast, g))
                .unwrap_or_default(),
            claim: claim_of(&ast, query),
            // Overwritten below for a query that is actually checked; a query
            // stating nothing keeps this and is never solved, which is the
            // command's second promise and the flag must not move it.
            outcome: "no-expect",
            ran: None,
        });
    }

    if claims.is_empty() {
        tally.silent += 1;
        if volume != Volume::Quiet {
            println!("{}", status("(no expect)", path));
        }
        return;
    }
    if claims.len() > 1
        && let Some(flag) = ONE_PATH_FLAGS
            .iter()
            .find(|f| m.get_one::<String>(f).is_some())
    {
        eprintln!(
            "error: --{flag} names one path and {} states {} expectations — \
             split the queries, or drop the flag",
            path.display(),
            claims.len()
        );
        // A usage refusal, not a finding: nothing about the file was checked,
        // so it must not read as a pass.
        tally.errors += 1;
        for &index in &claims {
            rows[first + index].outcome = Came::Error.key();
        }
        return;
    }

    let mut worst = Came::Held;
    let mut lines: Vec<String> = Vec::new();
    let mut preloaded = Some((ast, terms, kb));
    for &index in &claims {
        let loaded = match preloaded.take() {
            Some(t) if index == 0 => Some(t),
            _ => {
                let mut ast = Ast::new();
                let mut terms = Terms::new();
                crate::common::load_any_query_or_exit(&mut ast, &mut terms, path, index)
                    .map(|kb| (ast, terms, kb))
            }
        };
        let Some((ast, mut terms, mut kb)) = loaded else {
            tally.errors += 1;
            worst = worst.max(Came::Error);
            rows[first + index].outcome = Came::Error.key();
            continue;
        };
        let (came, label, detail, ran) = check_query(
            &ast, &mut terms, &mut kb, path, index, n_queries, m, volume, tally,
        );
        worst = worst.max(came);
        rows[first + index].outcome = came.key();
        rows[first + index].ran = ran;
        if volume == Volume::Verbose || came != Came::Held {
            lines.push(format!("  {label}"));
            lines.extend(detail.into_iter().map(|l| format!("    {l}")));
        }
    }

    if volume != Volume::Quiet || worst != Came::Held {
        println!("{}", status(worst.label(), path));
        for l in lines {
            println!("{l}");
        }
    }
}

/// One query's claim: solve to exhaustion, compare, report.
///
/// Returns what it came to, the one-line query header, the disagreement lines,
/// and — for `--json-report` — what the run itself found. The last is `None`
/// exactly when there was no run to describe.
#[allow(clippy::too_many_arguments)]
fn check_query(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut ein_core::Kb,
    path: &Path,
    index: usize,
    n_queries: usize,
    m: &ArgMatches,
    volume: Volume,
    tally: &mut Tally,
) -> (Came, String, Vec<String>, Option<Ran>) {
    let query = kb.program().query().expect("planned above");
    let goal = ein_infer::query_value(ast, query, "goal")
        .map(|g| ein_ir::dump_compact(ast, g))
        .unwrap_or_else(|| "?".to_string());
    let where_ = if n_queries == 1 {
        format!(":goal {goal}")
    } else {
        format!("query {} of {n_queries} · :goal {goal}", index + 1)
    };
    let node = ein_infer::query_value(ast, query, "expect").expect("planned above");
    let expectation = match ein_ir::expect::parse(ast, node) {
        Ok(e) => e,
        Err(e) => {
            // The loader rejected every shape `expect::parse` can refuse, so
            // this is unreachable from a program and says so rather than
            // inventing a failed expectation out of an engine bug.
            eprintln!("internal error: :expect passed the loader and did not parse: {e}");
            tally.errors += 1;
            return (Came::Error, where_, vec![e], None);
        }
    };

    let config = kb.program().config.clone().unwrap_or_default();
    let file = path.display().to_string();
    let mut events = crate::solve::events_start(m, &file, &config);
    crate::solve::events_load(&mut events, terms, kb);

    let opts = SolveOptions {
        // The whole point: `None` exhausts. See the module header.
        stop_after: None,
        max_set_size: (*m.get_one::<i64>("max-set-size").unwrap_or(&5)).max(0) as u32,
        config: Some(config.clone()),
        max_time: m.get_one::<f64>("max-time").copied(),
        max_enterings: m
            .get_one::<i64>("max-enterings")
            .map(|n| (*n).max(0) as u64),
        store_lattice: false,
        jobs: 1,
        ..SolveOptions::default()
    };

    let mut dumper = NoDumper;
    let t0 = Instant::now();
    let mut solved = match solve(kb, terms, ast, &mut events, &mut dumper, &opts) {
        Ok(s) => s,
        Err(SolveError::Budget { reason, .. }) => {
            // Never a pass and never a failure: the claim was neither
            // confirmed nor refuted, and reporting a budget abort as either
            // is the outcome this command exists to prevent.
            tally.errors += 1;
            return (
                Came::Error,
                where_,
                vec![format!("aborted before an answer: {reason}")],
                None,
            );
        }
        Err(SolveError::Compile(e)) => {
            eprintln!("{}", crate::common::compile_error_line(e));
            tally.errors += 1;
            return (
                Came::Error,
                where_,
                vec!["the rules did not compile".into()],
                None,
            );
        }
        Err(SolveError::Saturate(e)) => {
            eprintln!("{}", crate::common::saturate_error_line(&e));
            tally.errors += 1;
            return (Came::Error, where_, vec!["saturation failed".into()], None);
        }
        Err(e) => {
            eprintln!("{e}");
            tally.errors += 1;
            return (Came::Error, where_, vec![e.to_string()], None);
        }
    };

    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    let report = ein_infer::expect::check(
        ast,
        terms,
        &expectation,
        &solved.answer,
        solved.stats.exhausted,
    );
    // **`k` here is the verdict's, not `solution_nodes`** — the two parted on
    // twelve corpus entries at M1d S1d.2.6, and a claim is a claim about
    // models. Both go in the row; the verbose header below still prints the
    // search's count, which is what it has printed since S1c.1.3.
    let ran = Ran {
        verdict: solved.answer.as_str().to_string(),
        k: match &solved.answer {
            ein_infer::verdict::Answer::Verdict(v) => v.k(),
            ein_infer::verdict::Answer::Aborted { .. } => solved.stats.solution_nodes as usize,
        },
        solution_nodes: solved.stats.solution_nodes,
        exhausted: solved.stats.exhausted,
        layers: solved.stats.base.layers_explored,
        enterings: solved.stats.base.enterings_total,
        ms,
    };
    crate::solve::events_verdict(&mut events, terms, &solved.answer, &solved.stats);
    if let Some(out) = m.get_one::<String>("json-summary") {
        match crate::summary::build(
            ast,
            terms,
            kb,
            &mut solved.answer,
            &solved.stats,
            &config,
            &file,
            &mut events,
            &solved.owes,
        ) {
            Ok(s) => {
                if let Err(e) = crate::summary::write(out, &s) {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    let came = match report.outcome {
        Outcome::Held => {
            tally.held += 1;
            Came::Held
        }
        Outcome::Failed => {
            tally.failed += 1;
            Came::Failed
        }
        Outcome::NotChecked => {
            tally.not_checked += 1;
            Came::NotChecked
        }
    };
    let header = match volume {
        Volume::Verbose => format!(
            "{where_} — {} ({}, k = {})",
            came.verb(),
            solved.answer.as_str(),
            solved.stats.solution_nodes
        ),
        _ => format!("{where_} — {}", came.verb()),
    };
    (came, header, report.lines, Some(ran))
}
