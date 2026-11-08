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
//! **It runs what the claims need, and no more** (`solve_interface.md`
//! § 2.3). Since M1e2 S1e2.1a.5 a file's claims are checked against **one**
//! region run: a `(solve … :expect …)` and a query `:over all` (the default)
//! are about all of R, so they ask for the description; a query `:over
//! maximal` or `:over minimal` alone asks for that line's mode. Every claim of
//! the file is read off that one run, the solve claim against its R and each
//! query's against its projection classes. A claim the run could not decide
//! — a budget, a flagged program past the cube's limit — is `NOT CHECKED`.
//!
//! **It runs only what the expectations need.** A file whose queries state
//! nothing, and that has no region claim, is never solved — the one load that
//! finds this out is the whole of its cost.
//!
//! **A failure names what differs.** The solve claim's report says which
//! state is missing or extra; a query's, which projection class.

//! ## Exit codes, and the one that differs from `solve`
//!
//! | code | what it means |
//! |---|---|
//! | 0 | every expectation held, and at least one was checked |
//! | 1 | an expectation is **false**, or could not be checked |
//! | 2 | a load error, a usage error, a `-T` abort — or nothing to check |
//!
//! `solve` exits **1** on a load error. Here 1 is taken: it means *a claim is
//! false*. A test
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
//! the whole selection**, whether or not it claims anything, and one per
//! region claim. Since M1e2 S1e2.1a.5 it is `ein-test-report/3`: a query
//! row's `ran` (the walk's counters) became `answer`, what its classes are
//! over and how many. Since M1e2 S1e2.3.10 it is `ein-test-report/4`,
//! additively: every row carries `outside` and `types_as_objects`, what the
//! universe leaves out beside a claim the run checked, as the summary's
//! `root.hypgen` writes them, `null` on a row no run checked. `--json-summary`
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
use ein_infer::expect::{Outcome, Report};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::Ast;
use ein_ir::expect::Over;
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

/// The artefact flags, as in [`crate::solve`]: each names **one** path, so
/// each is incompatible with a selection that is more than one run.
const ONE_PATH_FLAGS: [&str; 2] = ["events", "json-summary"];

/// `--json-report`'s version. **Not** `ein-summary/2`'s: a report is one row
/// per query of a selection, a summary is one run, and a consumer that read
/// the same version marker on both would be right to expect the same fields.
const REPORT_SCHEMA: &str = "ein-test-report/4";

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
    /// A `(solve … :expect (region …))` claim's `(merged …)` and `(owing …)`
    /// counts — M1e2 S1e2.1a.4. `None` on a query's claim, which has neither.
    region_parts: Option<(usize, usize)>,
    /// A query's `:over` — M1e2 S1e2.1a.5. `None` on the region claim.
    over: Option<Over>,
}

/// What `ein_infer::claim` found for a `(solve … :expect …)` claim: R, by the
/// route that described it.
struct RegionRan {
    route: Option<&'static str>,
    monotone: bool,
    k_s: u64,
    k_c: u64,
    ms: f64,
    /// On a flagged program, the detector's three findings (M1e2 S1e2.1b.5):
    /// the status's word, how many nodes hold a stale fact, how many parent
    /// paths reach another state.
    warning: Option<(&'static str, u64, u64)>,
}

/// What H leaves out, beside every claim a run checked, as the text surface
/// prints it there (`outside the universe: …`, `a type as an object: …`) and
/// `--json-summary` writes it in `root.hypgen` (M1e2 S1e2.3.10, P1e2.3's N8):
/// `outside` (M1e2 S1e2.2.8) and `types_as_objects` (S1e2.2.10), under the
/// summary's field names and in its shapes, `[]` and empty lists when H
/// leaves nothing out.
#[derive(Clone)]
struct Scope {
    outside: Json,
    types_as_objects: Json,
}

/// What a query's claim was checked against — `ein-test-report/4`'s
/// `answer`: the set its classes are over, whether that set is complete, and
/// how many classes and states it held.
struct Answered {
    over: &'static str,
    exact: bool,
    classes: usize,
    states: usize,
}

/// One row of `--json-report` — one `(query …)`, or one file that never
/// became a program.
struct Row {
    path: String,
    /// 1-based, and **0 for a file that did not load**: a load error is a fact
    /// about the file, and inventing a query number for it would be inventing
    /// a query. 0 too on the row of a `(solve … :expect …)` claim, which is
    /// not a query's and says so in [`Self::place`].
    query: usize,
    queries: usize,
    goal: Option<String>,
    /// The relations the `:goal` asks about, from [`ein_ir::pattern_relations`]
    /// — what an expectation *must* close, and the first factor of the write
    /// cost of the claim this query does not yet carry.
    goal_relations: Vec<String>,
    claim: Option<Claim>,
    outcome: &'static str,
    answer: Option<Answered>,
    /// `solve` for the row of a `(solve … :expect …)` claim, `query` for every
    /// other — M1e2 S1e2.1a.4, where a file gained a claim that is not a
    /// query's.
    place: &'static str,
    /// The solve claim's run: R as it described it. A query's is
    /// [`Self::answer`].
    region: Option<RegionRan>,
    /// What H leaves out, where a run checked the row's claim (M1e2
    /// S1e2.3.10): the lines `ein test` prints beside that claim, as data.
    scope: Option<Scope>,
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
            ("place", Json::str(self.place)),
        ];
        pairs.push((
            "expect",
            match &self.claim {
                None => Json::Null,
                Some(c) => {
                    let mut kv = vec![
                        ("shape", Json::str(c.shape)),
                        ("models", Json::int(c.models as i64)),
                        ("facts", Json::int(c.facts as i64)),
                        ("negated", Json::int(c.negated as i64)),
                        (
                            "relations",
                            Json::Array(c.relations.iter().map(Json::str).collect()),
                        ),
                    ];
                    if let Some((merged, owing)) = c.region_parts {
                        kv.push(("merged", Json::int(merged as i64)));
                        kv.push(("owing", Json::int(owing as i64)));
                    }
                    if let Some(over) = c.over {
                        kv.push(("over", Json::str(over_key(over))));
                    }
                    Json::obj(kv)
                }
            },
        ));
        match &self.answer {
            None => pairs.push(("answer", Json::Null)),
            Some(a) => pairs.push((
                "answer",
                Json::obj(vec![
                    ("over", Json::str(a.over)),
                    ("exact", Json::Bool(a.exact)),
                    ("classes", Json::int(a.classes as i64)),
                    ("states", Json::int(a.states as i64)),
                ]),
            )),
        }
        if let Some(r) = &self.region {
            pairs.push((
                "region",
                Json::obj(vec![
                    ("route", r.route.map_or(Json::Null, Json::str)),
                    ("monotone", Json::Bool(r.monotone)),
                    ("k_s", Json::int(r.k_s as i64)),
                    ("k_c", Json::int(r.k_c as i64)),
                    ("ms", Json::Float(r.ms)),
                ]),
            ));
            if let (Some((status, stale, differ)), Some((_, Json::Object(fields)))) =
                (r.warning, pairs.last_mut())
            {
                fields.push(("status".to_string(), Json::str(status)));
                fields.push(("stale_nodes".to_string(), Json::int(stale as i64)));
                fields.push(("paths_differ".to_string(), Json::int(differ as i64)));
            }
        }
        // On every row, `null` where no run checked the row's claim.
        let (outside, types) = self.scope.as_ref().map_or((Json::Null, Json::Null), |s| {
            (s.outside.clone(), s.types_as_objects.clone())
        });
        pairs.push(("outside", outside));
        pairs.push(("types_as_objects", types));
        Json::obj(pairs)
    }
}

/// `all`, `minimal`, `maximal` — the keyword's own spelling.
fn over_key(over: Over) -> &'static str {
    match over {
        Over::All => "all",
        Over::Minimal => "minimal",
        Over::Maximal => "maximal",
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
        region_parts: None,
        over: Some(
            ein_infer::query_value(ast, query, "over")
                .and_then(|n| Over::parse(ast, n).ok())
                .unwrap_or_default(),
        ),
    })
}

/// The claim a `(solve … :expect …)` states, read like [`claim_of`].
fn solve_claim_of(ast: &Ast, claim: &ein_ir::claim::Claim) -> Claim {
    let (mut facts, mut negated) = (0usize, 0usize);
    let mut relations: Vec<String> = Vec::new();
    let (shape, models, parts, listed): (_, _, _, Vec<&ein_ir::expect::Model>) = match claim {
        ein_ir::claim::Claim::False => ("false", 0, None, Vec::new()),
        ein_ir::claim::Claim::Region(r) => (
            "region",
            r.models.len(),
            Some((r.merged.len(), r.owing.len())),
            r.models.iter().chain(r.owing.iter()).collect(),
        ),
    };
    for m in listed {
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
    Claim {
        shape,
        models,
        facts,
        negated,
        relations,
        region_parts: parts,
        over: None,
    }
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

/// The walk's options `test` took, retired with `solve`'s at the switch.
const RETIRED: [(&str, &str); 2] = [
    (
        "max-set-size",
        "-m/--max-set-size is retired: a depth cap changed answers, and `ein test` \
         runs the region mode each file's claims need, within --region-budget",
    ),
    (
        "max-enterings",
        "-E/--max-enterings is retired: --region-budget FACTS bounds each file's run",
    ),
];

pub fn run(m: &ArgMatches) -> i32 {
    if crate::solve::refuse_retired(m, &RETIRED) {
        return 2;
    }
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
        crate::solve::artefact_failed("json-report", out, e);
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
fn write_report(path: &str, tally: &Tally, rows: &[Row]) -> std::io::Result<()> {
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
}

/// One file: load it once, find out what it claims, and check every claim
/// against **one** region run, in the mode the claims need (the module
/// header).
///
/// Everything it finds goes into `tally`, which is where the exit code comes
/// from: a per-file code would have to be combined by a second rule, and the
/// counters already say more than the maximum of three integers does.
fn check_file(path: &Path, m: &ArgMatches, volume: Volume, tally: &mut Tally, rows: &mut Vec<Row>) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let row = |query: usize, queries: usize, place: &'static str, outcome: &'static str| Row {
        path: path.display().to_string(),
        query,
        queries,
        goal: None,
        goal_relations: Vec::new(),
        claim: None,
        outcome,
        answer: None,
        place,
        region: None,
        scope: None,
    };
    let load_opts = crate::common::load_options(m);
    let Some(mut kb) = crate::common::load_any_or_exit(&mut ast, &mut terms, path, &load_opts)
    else {
        println!("{}", status("ERROR", path));
        tally.errors += 1;
        // A file that did not load states nothing, because a claim is a
        // property of a *program*. The `examples/broken/load/` fixtures that
        // contain the token `:expect` are here — they exist to be refused,
        // and counting them as claims would put the loader's own negatives in
        // the numerator of "what fraction of the corpus claims a model set".
        rows.push(row(0, 0, "query", Came::Error.key()));
        return;
    };
    let n_queries = kb.program().queries.len();
    // The `(solve … :expect …)` claim first: it is about the lattice every
    // query asks over, so its header prints first and its row comes first.
    let solve_claim = kb
        .program()
        .solve()
        .and_then(|s| ein_infer::solve_value(&ast, s, "expect"))
        .and_then(|node| ein_ir::claim::parse(&ast, node).ok());
    let solve_row = solve_claim.as_ref().map(|c| {
        let mut r = row(0, n_queries, "solve", Came::Error.key());
        r.claim = Some(solve_claim_of(&ast, c));
        rows.push(r);
        rows.len() - 1
    });
    // Every query gets a row, claim or no claim: the census's question is a
    // *fraction*, and a report that listed only the numerator could not answer
    // it. A file with no `(query …)` at all — every `stdlib/*.ein` is one —
    // still gets a row, so that the rows account for the selection file for
    // file.
    let first = rows.len();
    if n_queries == 0 {
        rows.push(row(0, 0, "query", "no-query"));
    }
    let mut overs: Vec<(usize, Over)> = Vec::new();
    for (i, query) in kb.program().queries.iter().enumerate() {
        let goal = ein_infer::query_value(&ast, query, "goal");
        let mut r = row(i + 1, n_queries, "query", "no-expect");
        r.goal = goal.map(|g| ein_ir::dump_compact(&ast, g));
        r.goal_relations = goal
            .map(|g| ein_ir::pattern_relations(&ast, g))
            .unwrap_or_default();
        r.claim = claim_of(&ast, query);
        if let Some(c) = &r.claim {
            overs.push((i, c.over.unwrap_or_default()));
        }
        rows.push(r);
    }
    if overs.is_empty() && solve_claim.is_none() {
        tally.silent += 1;
        if volume != Volume::Quiet {
            println!("{}", status("(no expect)", path));
        }
        return;
    }

    // The one run: the description when any claim is about all of R, or the
    // claims ask both lines; otherwise the one line they ask.
    let asks = |o: Over| overs.iter().any(|(_, x)| *x == o);
    let which =
        if solve_claim.is_some() || asks(Over::All) || (asks(Over::Minimal) && asks(Over::Maximal))
        {
            Which::All
        } else if asks(Over::Maximal) {
            Which::Maximal
        } else {
            Which::Minimal
        };
    let mut opts = explore::Options::new(Mode {
        which,
        ask: Ask::Describe,
    });
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    opts.deadline = m
        .get_one::<f64>("max-time")
        .map(|s| Instant::now() + std::time::Duration::from_secs_f64(s.max(0.0)));
    // A claim states no core: the run takes one only where `--events` prints
    // it (M1e2 S1e2.2.21, the user's ruling of 2026-11-05).
    opts.cores = m.get_one::<String>("events").is_some();
    let config = kb.program().config.clone().unwrap_or_default();
    let file = path.display().to_string();
    let mut events = crate::solve::events_start(m, &file, &config);
    crate::solve::events_load(&mut events, &terms, &kb);
    let t0 = Instant::now();
    let failed = |tally: &mut Tally, why: String| {
        tally.errors += 1;
        println!("{}", status(Came::Error.label(), path));
        println!("    {why}");
    };
    let r = match ein_infer::read_out::run_with(&mut kb, &mut terms, &ast, &opts, &mut events) {
        Ok(r) => r,
        Err(ein_infer::SolveError::Deadline) => {
            // Never a pass and never a failure: the claims were neither
            // confirmed nor refuted.
            return failed(tally, "aborted before an answer: --max-time".to_string());
        }
        Err(e) => {
            eprintln!("{e}");
            return failed(tally, e.to_string());
        }
    };
    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    crate::modes::emit_region(&mut events, &r);
    if r.answer.stopped == Some(explore::Stopped::Deadline) {
        return failed(tally, "aborted before an answer: --max-time".to_string());
    }
    if let Some(out) = m.get_one::<String>("json-summary") {
        let written = crate::summary::build_region(&ast, &mut terms, &mut kb, &r, &file, &config)
            .and_then(|s| crate::summary::write(out, &s).map_err(|e| e.to_string()));
        if let Err(e) = written {
            crate::solve::artefact_failed("json-summary", out, e);
        }
    }

    let came_of = |report: &Report, tally: &mut Tally| match report.outcome {
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
    let mut worst = Came::Held;
    let mut lines: Vec<String> = Vec::new();
    // The warning, beside each claim printed on a flagged program (the
    // proposal's § 4.3, approved 2026-11-03): under `-v`, and under a claim
    // that FAILED or was NOT CHECKED.
    let warning = (!r.answer.monotonicity.is_monotone()).then(|| {
        format!(
            "    {}",
            ein_render::read_out::warning_line(
                &r.answer.monotonicity,
                r.answer.warning.as_deref(),
                r.answer.root_dead
            )
        )
    });
    // What the universe leaves out (M1e2 S1e2.2.8, Q-M1e2.3 (a)), beside
    // each claim printed as the warning is: a claim is checked over R, and R
    // is over H.
    let outside = ein_render::read_out::outside_text(&r.answer.root.outside)
        .map(|t| format!("    outside the universe: {t}"));
    // What the untyped rung bound as an object (M1e2 S1e2.2.10, Q-M1e.12
    // (a)), beside each claim printed as the scope is.
    let types = ein_render::read_out::types_text(&r.answer.root.types_as_objects)
        .map(|t| format!("    a type as an object: {t}"));
    // The same two, as data, on the row of every claim the run checked.
    let scope = Scope {
        outside: ein_render::dump::region::outside_json(&r.answer.root.outside),
        types_as_objects: ein_render::dump::region::types_json(&r.answer.root.types_as_objects),
    };
    if let (Some(i), Some(claim)) = (solve_row, solve_claim.as_ref()) {
        let report = r.claim.clone().unwrap_or_else(|| Report {
            outcome: Outcome::NotChecked,
            lines: vec!["the claim was not read".to_string()],
        });
        let came = came_of(&report, tally);
        worst = worst.max(came);
        let counts = r.answer.counts.as_ref();
        let (k_s, k_c) = counts.map_or((0, 0), |c| (c.k_s, c.k_c));
        let route = match r.answer.route {
            explore::Route::DownSet => "route 1",
            explore::Route::Cube => "route 2",
            explore::Route::States => "state route",
            explore::Route::Tree => "obligation tree",
        };
        rows[i].outcome = came.key();
        rows[i].region = Some(RegionRan {
            route: Some(route),
            monotone: r.answer.monotonicity.is_monotone(),
            k_s,
            k_c,
            ms,
            warning: r.answer.warning.as_deref().map(|w| {
                (
                    w.status.word(),
                    w.state.stale.len() as u64,
                    w.paths.as_ref().map_or(0, |p| p.differ.len() as u64),
                )
            }),
        });
        rows[i].scope = Some(scope.clone());
        let shape = match claim {
            ein_ir::claim::Claim::False => "(false)",
            ein_ir::claim::Claim::Region(x) if x.models.is_empty() && x.owing.is_empty() => {
                "(region)"
            }
            ein_ir::claim::Claim::Region(_) => "(region …)",
        };
        if volume == Volume::Verbose || came != Came::Held {
            lines.push(match volume {
                Volume::Verbose => format!(
                    "  solve · :expect {shape} — {} ({route}, k_s = {k_s}, k_c = {k_c})",
                    came.verb()
                ),
                _ => format!("  solve · :expect {shape} — {}", came.verb()),
            });
            lines.extend(report.lines.iter().map(|l| format!("    {l}")));
            lines.extend(warning.clone());
            lines.extend(outside.clone());
            lines.extend(types.clone());
        }
    }
    // The read-out answers the queries that have a :goal, in order.
    let mut answered: Vec<Option<usize>> = Vec::with_capacity(n_queries);
    let mut next = 0usize;
    for q in &kb.program().queries {
        if ein_infer::query_value(&ast, q, "goal").is_some() {
            answered.push(Some(next));
            next += 1;
        } else {
            answered.push(None);
        }
    }
    for &(qi, _) in &overs {
        let at = answered[qi].and_then(|k| r.queries.get(k));
        let report = at.and_then(|q| q.claim.clone()).unwrap_or_else(|| Report {
            outcome: Outcome::NotChecked,
            lines: vec!["the query has no :goal to answer".to_string()],
        });
        let came = came_of(&report, tally);
        worst = worst.max(came);
        let row = &mut rows[first + qi];
        row.outcome = came.key();
        row.answer = at.map(|q| Answered {
            over: if q.as_asked {
                over_key(q.over)
            } else {
                "listed"
            },
            exact: q.exact,
            classes: q.classes.len(),
            states: q.classes.iter().map(|c| c.states.len()).sum(),
        });
        row.scope = Some(scope.clone());
        if volume == Volume::Verbose || came != Came::Held {
            let goal = row.goal.clone().unwrap_or_else(|| "?".to_string());
            let where_ = if n_queries == 1 {
                format!(":goal {goal}")
            } else {
                format!("query {} of {n_queries} · :goal {goal}", qi + 1)
            };
            lines.push(match (volume, at) {
                (Volume::Verbose, Some(q)) => format!(
                    "  {where_} — {} ({}, {} over {} states)",
                    came.verb(),
                    if q.as_asked {
                        q.over.words()
                    } else {
                        "the states listed"
                    },
                    if q.classes.len() == 1 {
                        "1 class".to_string()
                    } else {
                        format!("{} classes", q.classes.len())
                    },
                    q.classes.iter().map(|c| c.states.len()).sum::<usize>()
                ),
                _ => format!("  {where_} — {}", came.verb()),
            });
            lines.extend(report.lines.iter().map(|l| format!("    {l}")));
            lines.extend(warning.clone());
            lines.extend(outside.clone());
            lines.extend(types.clone());
        }
    }

    if volume != Volume::Quiet || worst != Came::Held {
        println!("{}", status(worst.label(), path));
        for l in lines {
            println!("{l}");
        }
    }
}
