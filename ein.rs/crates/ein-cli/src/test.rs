//! `ein test` — the expectations a program states about itself, run.
//!
//! [`05-cli.md` § 2.3](../../../../docs/ein/user/04-reference/05-cli.md#23-ein-test).
//! A `(query …)` and a `(solve …)` may state their own answer
//! ([claims](../../../../docs/ein/user/03-syntax-and-semantics/12-query.md#claims));
//! this is the runner that turns a directory of them into a status code, so
//! that **nothing reads output**: checking that a rule works is not running
//! `solve` and looking, nor diffing against a golden.
//!
//! ## Three things it does that `solve` does not
//!
//! **It asks the question the claims are, and no more** (Q-M1f.221).
//! A file's claims are read by their shapes and asked **the least question
//! that can decide them** ([`ein_infer::read_out::least`]): a `(solve …
//! :expect …)` asks for the description of R, whose route on a dead root is
//! root's saturation; queries' claims that are all over one part of R, the
//! most classes one lists being k, ask that part for k + 1 states, `-n k+1`,
//! and `--single` where k is at most one. A run that finds fewer has listed
//! the whole part and decides every claim; one that finds k + 1 refutes a
//! claim where a state found gives a class the claim does not list. **What
//! that question leaves undecided is asked the describing one**
//! ([`ein_infer::read_out::describing`]): all of R, or the one line every
//! claim is over.
//! So a file is one run where one run decides it and two where it does not,
//! and no claim a description decides is left unchecked. Each claim is read
//! off the run that decided it ([`ein_infer::read_out::stands`]), the solve
//! claim against its R and each query's against its projection classes, and
//! `-v` and `--json-report` say which question that was. A claim neither
//! question decided, a budget or a flagged program past the cube's limit, is
//! `NOT CHECKED`, and says what was asked first and why it did not decide.
//! No option chooses the question; `--no-search` asks none
//! about R: each file is read at root alone, as `ein solve --no-search` reads
//! it, with no fallback, and a claim root does not decide is `NOT CHECKED`
//! and moves no exit code (Q-M1f.257).
//!
//! **It runs only what the expectations need.** A file whose queries state
//! nothing, and that has no region claim, is never solved — the one load that
//! finds this out is the whole of its cost. Nor is a saved file's node: its
//! claims are its program's, about the program's R, which no question about
//! the node decides, so each is `NOT CHECKED` with no run, moving no exit code
//! (Q-M1f.403).
//!
//! **A failure names what differs.** The solve claim's report says which
//! state is missing or extra; a query's, which projection class. Under
//! `--diff` every claim checked is followed by its diff
//! ([`ein_render::claim_diff`]): the models it lists against the states,
//! owing nodes or classes the run found, over what its check compares, as
//! counts and, at `full`, model by model, the near-miss *one fact off* among
//! them. It explains and decides nothing: every verdict, every line but the
//! diff's and the exit code are the same with it and without.

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
//! runner that cannot tell a broken file from a false claim fails at its one
//! job, so a load error takes 2 — the code
//! that already means "this run is not a verdict" (a usage error, a budget
//! abort). **2 dominates 1** in the summary for the same reason: if any file
//! failed to load, "every expectation was checked and some are false" would be
//! a lie about the run.
//!
//! ## `--json-report`, and why it is not `--json-summary`
//!
//! The runner's own output, machine-readable: **one row per `(query …)` of
//! the whole selection**, whether or not it claims anything, and one per
//! region claim, `ein-test-report/6`. A query row's `answer` says what its
//! classes are over and how many. Every row carries `outside`, what the
//! universe leaves out beside a claim the run checked, as the summary's
//! `root.hypgen` writes it; `route`, the question
//! its claim's outcome was read off, as `ein solve`'s flags spell it and
//! `describe` for the bare run; and `fell_back_from`, the question asked
//! first where it left the claim undecided; each `null` on a row no run
//! checked; and `diff`, the claim's diff under `--diff`, `null` without it
//! and on a row no run checked. `--json-summary`
//! is a *run*'s summary — verdict, counters, config — and is refused over a
//! selection of more than one run for the reason every artefact flag is: it
//! names one path. A report is the other shape. It has no run to be more than
//! one of, so it takes any selection, and one invocation over the three
//! corpus roots is the whole census.
//!
//! What it publishes that nothing else could: the **shape** of a claim. The
//! rule is that *nothing reads output*, and there is no other way to ask a
//! corpus what fraction of it states a closure claim — `:expect` is a query
//! keyword, and a grep cannot tell a keyword from a comment about one. That
//! is the transport of
//! [the claims about a set of states](../../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#3-claims-about-the-set-of-states).
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
use ein_infer::explore::{self, Mode};
use ein_infer::read_out::{self, ReadOut, Stands};
use ein_ir::Ast;
use ein_ir::expect::Over;
use ein_render::claim_diff::{self, Level};
use ein_render::dump::Json;
use ein_render::dump::json::dumps_indent;

/// The artefact flags, as in [`crate::solve`]: each names **one** path, so
/// each is incompatible with a selection that is more than one run.
const ONE_PATH_FLAGS: [&str; 2] = ["events", "json-summary"];

/// `--json-report`'s version. **Not** `ein-summary/2`'s: a report is one row
/// per query of a selection, a summary is one run, and a consumer that read
/// the same version marker on both would be right to expect the same fields.
const REPORT_SCHEMA: &str = "ein-test-report/6";

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
    /// `--no-search` (Q-M1f.257): a claim root does not decide is not
    /// checked because the run was asked not to search, which is no failure
    /// to check, so it moves no exit code.
    no_search: bool,
    /// The claims a saved file's node holds, each NOT CHECKED (Q-M1f.403):
    /// a claim is its program's, about the program's R, so none is asked
    /// of the node ([`NotRun`]), which is no failure to check and moves no
    /// exit code.
    saved: usize,
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
        let unchecked = if self.no_search {
            0
        } else {
            self.not_checked - self.saved
        };
        i32::from(self.failed + unchecked > 0)
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
/// is the one thing about `:expect` no other surface publishes, and what a
/// census of the corpus's claims is built on. `models` is the number of
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
    /// counts. `None` on a query's claim, which has neither.
    region_parts: Option<(usize, usize)>,
    /// A query's `:over`. `None` on the region claim.
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
    /// On a flagged program, the detector's three findings:
    /// the status's word, how many nodes hold a stale fact, how many parent
    /// paths reach another state.
    warning: Option<(&'static str, u64, u64)>,
}

/// What H leaves out, beside every claim a run checked, as the text surface
/// prints it there (`outside the universe: …`) and `--json-summary` writes it
/// in `root.hypgen`: `outside` (Q-M1e2.3), under the summary's field name and
/// in its shape, `[]` when H leaves nothing out.
#[derive(Clone)]
struct Scope {
    outside: Json,
}

/// What a query's claim was checked against — `--json-report`'s
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
    /// other: a file may state a claim that is not a query's.
    place: &'static str,
    /// The solve claim's run: R as it described it. A query's is
    /// [`Self::answer`].
    region: Option<RegionRan>,
    /// What H leaves out, where a run checked the row's claim: the lines
    /// `ein test` prints beside that claim, as data.
    scope: Option<Scope>,
    /// The question the row's claim was read off ([`Mode::flags`]), where a
    /// run checked it (Q-M1f.221).
    route: Option<String>,
    /// The question asked first, where it left the claim undecided and the
    /// outcome is the describing question's.
    fell_back_from: Option<String>,
    /// The claim's diff under `--diff` ([`claim_diff::json`]), where a run
    /// decided the claim.
    diff: Option<Json>,
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
        let outside = self
            .scope
            .as_ref()
            .map_or(Json::Null, |s| s.outside.clone());
        pairs.push(("outside", outside));
        let word = |w: &Option<String>| w.as_deref().map_or(Json::Null, Json::str);
        pairs.push(("route", word(&self.route)));
        pairs.push(("fell_back_from", word(&self.fell_back_from)));
        pairs.push(("diff", self.diff.clone().unwrap_or(Json::Null)));
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

/// The claim a query states, or `None` where it states none — read from the
/// **loaded** program, whose claim's shape is the grammar's.
fn claim_of(ast: &Ast, query: &ein_core::Query) -> Option<Claim> {
    let node = ein_infer::query_value(ast, query, "expect")?;
    let e = ein_ir::expect::parse(ast, node);
    let (mut facts, mut negated) = (0usize, 0usize);
    let mut relations: Vec<String> = Vec::new();
    for m in e.models() {
        for &f in &m.facts {
            let f = ein_ir::expect::fact(ast, f);
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
            let f = ein_ir::expect::fact(ast, f);
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
    let mut tally = Tally {
        no_search: m.get_flag("no-search"),
        ..Tally::default()
    };
    // Accumulated whether or not anyone asked for it — 197 rows over the whole
    // corpus, and a flag that changed what the run *did* would not be additive.
    let mut rows: Vec<Row> = Vec::new();
    for path in &files {
        tally.files += 1;
        // Every saturation of the file, its load's judgement of the
        // mechanisms among them, under the cap; each question adds the
        // file's deadline (`explore`'s § A saturation is bounded).
        let cap = ein_infer::saturator::Bound {
            deadline: None,
            max_steps: Some(crate::modes::max_steps(m)),
        };
        ein_infer::saturator::bounded(cap, || check_file(path, m, volume, &mut tally, &mut rows));
        // What its load noticed, after the file's rows.
        crate::common::say_notes();
    }
    if let Some(out) = m.get_one::<String>("json-report")
        && let Err(e) = write_report(out, &tally, &rows)
    {
        crate::solve::artefact_failed("json-report", out, e);
    }
    println!("{}", tally.line(t0.elapsed().as_secs_f64()));
    // **Reported, never skipped past**, in the shape this command can fail
    // in. A selection that checked nothing has said nothing
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
        route: None,
        fell_back_from: None,
        diff: None,
    };
    let load_opts = crate::common::load_options(m);
    let Some(kb) = crate::common::load_program(&mut ast, &mut terms, path, &load_opts) else {
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
    let first_row = rows.len();
    if n_queries == 0 {
        rows.push(row(0, 0, "query", "no-query"));
    }
    let mut overs: Vec<(usize, Over)> = Vec::new();
    for (i, query) in kb.program().queries.iter().enumerate() {
        let goal = ein_infer::query_value(&ast, query, "goal");
        let mut r = row(i + 1, n_queries, "query", "no-expect");
        // The goal as written; its relations, and what is matched, are the
        // expansion's.
        r.goal = ein_infer::goal_written(&ast, query).map(|g| ein_ir::dump_compact(&ast, g));
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
    // A saved file's node (Q-M1f.403): its claims are its program's, about
    // the program's R, and no question about the node decides one, so none
    // is asked.
    if kb.program().commit.is_some() {
        let claims = NotRun {
            solve: solve_row.zip(solve_claim.as_ref()),
            queries: &overs,
            first_row,
            n_queries,
        };
        return claims.say(path, tally, rows);
    }

    // The question the claims are, and the describing one it falls back to
    // (the module header). One deadline for the file, whatever is asked.
    // Under `--no-search` one question, root alone, and no
    // fallback: what root does not decide stays NOT CHECKED.
    let no_search = m.get_flag("no-search");
    let (least, describing) = if no_search {
        let root = Mode {
            which: explore::Which::All,
            ask: explore::Ask::Describe,
        };
        (root, root)
    } else {
        (read_out::least(&kb, &ast), read_out::describing(&kb, &ast))
    };
    let deadline = m
        .get_one::<f64>("max-time")
        .map(|s| Instant::now() + std::time::Duration::from_secs_f64(s.max(0.0)));
    let failed = |tally: &mut Tally, why: String| {
        tally.errors += 1;
        println!("{}", status(Came::Error.label(), path));
        println!("    {why}");
    };
    let tree = explore::DescribeRoute::Tree;
    let mut first = match ask(path, m, least, deadline, (ast, terms, kb), tree) {
        Ok(run) => run,
        Err(why) => return failed(tally, why),
    };
    // A flagged program: the loud warning, once a file (Q-M1f.564). Its
    // claims are checked as any program's are.
    if !first.r.answer.monotonicity.is_monotone() {
        eprintln!(
            "{}",
            ein_infer::detector::warning_line(&path.display().to_string())
        );
    }
    // The read-out answers the queries that have a :goal, in order.
    let mut answered: Vec<Option<usize>> = Vec::with_capacity(n_queries);
    let mut next = 0usize;
    for q in &first.kb.program().queries {
        if ein_infer::query_value(&first.ast, q, "goal").is_some() {
            answered.push(Some(next));
            next += 1;
        } else {
            answered.push(None);
        }
    }
    // A claim the least question left undecided is asked the describing one,
    // of the program loaded again: a run saturates the root it is given.
    let undecided = least != describing
        && overs.iter().any(|&(qi, _)| {
            answered[qi]
                .and_then(|k| first.r.queries.get(k))
                .and_then(|q| q.claim.as_ref())
                .is_some_and(|c| c.outcome == Outcome::NotChecked)
        });
    let mut second = None;
    if undecided {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Some(kb) = crate::common::load_program(&mut ast, &mut terms, path, &load_opts) else {
            return failed(tally, "the program did not load a second time".to_string());
        };
        second = match ask(path, m, describing, deadline, (ast, terms, kb), tree) {
            Ok(run) => Some(run),
            Err(why) => return failed(tally, why),
        };
    }
    // Each query claim, read off the run that decided it.
    let no_goal = || Report {
        outcome: Outcome::NotChecked,
        lines: vec!["the query has no :goal to answer".to_string()],
    };
    let mut settled: Vec<(usize, Report, bool)> = Vec::with_capacity(overs.len());
    for &(qi, _) in &overs {
        let of = |run: &Asked| {
            answered[qi]
                .and_then(|k| run.r.queries.get(k))
                .and_then(|q| q.claim.clone())
                .unwrap_or_else(no_goal)
        };
        let one = of(&first);
        let Some(run) = second.as_ref() else {
            settled.push((qi, one, false));
            continue;
        };
        let mut two = of(run);
        match read_out::stands(&one, &two) {
            Ok(Stands::First) => settled.push((qi, one, false)),
            Ok(Stands::Fallback) => {
                if two.outcome == Outcome::NotChecked && answered[qi].is_some() {
                    two.lines.push(format!(
                        "asked first: {}, which {}",
                        least.flags(),
                        read_out::undecided_words(&first.r.answer)
                    ));
                }
                settled.push((qi, two, true));
            }
            Err(e) => {
                eprintln!("{e}");
                return failed(tally, e.to_string());
            }
        }
    }
    // The artefact of one run is the last one asked: the run every claim
    // the first left undecided was read off.
    if let Some(out) = m.get_one::<String>("json-summary") {
        let run = second.as_mut().unwrap_or(&mut first);
        let written = crate::summary::build_region(
            &run.ast,
            &mut run.terms,
            &mut run.kb,
            &run.r,
            &run.file,
            &run.config,
        )
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
    // A solve claim over an empty R of the owing kind needs its owing
    // frontier, which the tree and the classes do not compute: where the
    // describing question left the claim unchecked for that, it is asked
    // again by the state route, of the program loaded again.
    let frontier_skipped = first.r.described().is_some_and(|d| d.frontier_skipped);
    let mut by_states = None;
    if solve_claim.is_some()
        && frontier_skipped
        && first
            .r
            .claim
            .as_ref()
            .is_some_and(|c| c.outcome == Outcome::NotChecked)
    {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Some(kb) = crate::common::load_program(&mut ast, &mut terms, path, &load_opts) else {
            return failed(tally, "the program did not load a second time".to_string());
        };
        by_states = match ask(
            path,
            m,
            describing,
            deadline,
            (ast, terms, kb),
            explore::DescribeRoute::States,
        ) {
            Ok(run) => Some(run),
            Err(why) => return failed(tally, why),
        };
    }
    // `--diff`: the solve claim's models against R, of the run its outcome
    // is read off.
    let level = m.get_one::<String>("diff").and_then(|w| Level::parse(w));
    let solve_diff = match (level, solve_claim.as_ref()) {
        (Some(level), Some(claim)) => {
            solve_diff(by_states.as_mut().unwrap_or(&mut first), claim, m, level)
        }
        _ => None,
    };
    let solve_run = by_states.as_ref().unwrap_or(&first);
    if let (Some(i), Some(claim)) = (solve_row, solve_claim.as_ref()) {
        // A solve claim is asked the describing question alone.
        let (r, beside) = (&solve_run.r, Beside::of(&solve_run.r));
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
            explore::Route::Root => "root alone",
            explore::Route::TreeClasses => "tree and classes",
        };
        rows[i].outcome = came.key();
        rows[i].region = Some(RegionRan {
            route: Some(route),
            monotone: r.answer.monotonicity.is_monotone(),
            k_s,
            k_c,
            ms: solve_run.ms,
            warning: r.answer.warning.as_deref().map(|w| {
                (
                    w.status.word(),
                    w.state.stale.len() as u64,
                    w.paths.as_ref().map_or(0, |p| p.differ.len() as u64),
                )
            }),
        });
        rows[i].scope = Some(beside.scope.clone());
        rows[i].route = Some(solve_run.flags());
        let shape = solve_shape(claim);
        let diffed = solve_diff.is_some() && volume != Volume::Quiet;
        if volume == Volume::Verbose || came != Came::Held || diffed {
            lines.push(match volume {
                // `--no-search` counts no state of R.
                Volume::Verbose if r.answer.route == explore::Route::Root => format!(
                    "  solve · :expect {shape} — {} (--no-search: root alone)",
                    came.verb()
                ),
                Volume::Verbose => format!(
                    "  solve · :expect {shape} — {} ({route}, k_s = {k_s}, k_c = {})",
                    came.verb(),
                    if k_c >= ein_infer::count::CAP {
                        format!("{k_c} or more")
                    } else {
                        k_c.to_string()
                    }
                ),
                _ => format!("  solve · :expect {shape} — {}", came.verb()),
            });
            lines.extend(report.lines.iter().map(|l| format!("    {l}")));
            lines.extend(solve_diff.iter().flat_map(|d| d.lines.iter().cloned()));
            lines.extend(beside.lines());
        }
        rows[i].diff = solve_diff.map(|d| d.json);
    }
    for (qi, report, fell_back) in settled {
        let run = match (&second, fell_back) {
            (Some(run), true) => run,
            _ => &first,
        };
        let beside = Beside::of(&run.r);
        let at = answered[qi].and_then(|k| run.r.queries.get(k));
        let came = came_of(&report, tally);
        worst = worst.max(came);
        let row = &mut rows[first_row + qi];
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
        row.scope = Some(beside.scope.clone());
        // A claim on a query with no :goal is answered by no run.
        if at.is_some() {
            row.route = Some(run.flags());
            row.fell_back_from = fell_back.then(|| least.flags());
        }
        // `--diff`: the claim's models against the classes it was read off,
        // where they decided it.
        let diff = match (level, at) {
            (Some(level), Some(q)) if report.outcome != Outcome::NotChecked => {
                ein_infer::query_value(&run.ast, &run.kb.program().queries[qi], "expect")
                    .and_then(|node| ein_infer::read_out::query_compared(&run.ast, node, q))
                    .map(|c| Diffed::of(&c, level))
            }
            _ => None,
        };
        let diffed = diff.is_some() && volume != Volume::Quiet;
        if volume == Volume::Verbose || came != Came::Held || diffed {
            let where_ = where_of(row.goal.as_deref(), qi, n_queries);
            lines.push(match (volume, at) {
                (Volume::Verbose, Some(q)) => format!(
                    "  {where_} — {} ({}{}: {}, {} over {} states)",
                    came.verb(),
                    run.flags(),
                    if fell_back {
                        format!(", after {}", least.flags())
                    } else {
                        String::new()
                    },
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
            lines.extend(diff.iter().flat_map(|d| d.lines.iter().cloned()));
            lines.extend(beside.lines());
        }
        row.diff = diff.map(|d| d.json);
    }

    if volume != Volume::Quiet || worst != Came::Held {
        println!("{}", status(worst.label(), path));
        for l in lines {
            println!("{l}");
        }
    }
}

/// A `(solve … :expect …)` claim's shape, as its header prints it.
fn solve_shape(claim: &ein_ir::claim::Claim) -> &'static str {
    match claim {
        ein_ir::claim::Claim::False => "(false)",
        ein_ir::claim::Claim::Region(x) if x.models.is_empty() && x.owing.is_empty() => "(region)",
        ein_ir::claim::Claim::Region(_) => "(region …)",
    }
}

/// Where a query's claim is, as its header prints it: its `:goal`, and its
/// place among the file's queries where there are several.
fn where_of(goal: Option<&str>, qi: usize, n_queries: usize) -> String {
    let goal = goal.unwrap_or("?");
    if n_queries == 1 {
        format!(":goal {goal}")
    } else {
        format!("query {} of {n_queries} · :goal {goal}", qi + 1)
    }
}

/// **A saved file's node, asked nothing** (Q-M1f.403): its
/// claims are its program's, about the program's R, and no question about
/// the node decides one, so no run is made, not root's either. Each claim is
/// NOT CHECKED with the reason, moving no exit code; its header names no
/// question, since none was asked; and its row carries no run's data:
/// `answer`, `outside`, `route`, `fell_back_from` and `diff` are `null`,
/// and a solve row has no `region`, as on a row no run checked. Nothing
/// saturates, so `--json-summary` and `--events` write nothing for the file,
/// as for a file that states no claim.
struct NotRun<'a> {
    /// The solve claim's row and the claim, where the file states one.
    solve: Option<(usize, &'a ein_ir::claim::Claim)>,
    /// The queries that state a claim, each by its index.
    queries: &'a [(usize, Over)],
    /// The file's first query row.
    first_row: usize,
    n_queries: usize,
}

impl NotRun<'_> {
    fn say(&self, path: &Path, tally: &mut Tally, rows: &mut [Row]) {
        let why = format!("    {}", read_out::SAVED_CHECKS_NO_CLAIM);
        let mut lines: Vec<String> = Vec::new();
        let mut not_checked = |row: &mut Row, header: String| {
            tally.not_checked += 1;
            tally.saved += 1;
            row.outcome = Came::NotChecked.key();
            lines.push(header);
            lines.push(why.clone());
        };
        let verb = Came::NotChecked.verb();
        if let Some((i, claim)) = self.solve {
            let header = format!("  solve · :expect {} — {verb}", solve_shape(claim));
            not_checked(&mut rows[i], header);
        }
        for &(qi, _) in self.queries {
            let row = &mut rows[self.first_row + qi];
            let header = format!(
                "  {} — {verb}",
                where_of(row.goal.as_deref(), qi, self.n_queries)
            );
            not_checked(row, header);
        }
        // Printed at every volume, as every file whose worst outcome is not
        // `ok` is.
        println!("{}", status(Came::NotChecked.label(), path));
        for l in lines {
            println!("{l}");
        }
    }
}

/// A claim's diff, as `ein test` prints it under the claim and as the
/// report writes it on the claim's row.
struct Diffed {
    lines: Vec<String>,
    json: Json,
}

impl Diffed {
    fn of(c: &ein_infer::claim::Compared, level: Level) -> Diffed {
        let d = claim_diff::diff(c);
        Diffed {
            lines: claim_diff::lines(c, &d, level, "    "),
            json: claim_diff::json(c, &d, level),
        }
    }
}

/// The `(solve … :expect …)` claim's diff, of the run `run` its outcome was
/// read off, where that run decided it: R's states, or its owing frontier,
/// each saturated again as the check reads it. A claim that lists no model,
/// `(false)`, has none. Taking it moves nothing the run answered; a run that
/// cannot take it says why where the diff would be, and writes `null`.
fn solve_diff(
    run: &mut Asked,
    claim: &ein_ir::claim::Claim,
    m: &ArgMatches,
    level: Level,
) -> Option<Diffed> {
    if run
        .r
        .claim
        .as_ref()
        .is_none_or(|c| c.outcome == Outcome::NotChecked)
    {
        return None;
    }
    let mut opts = ein_infer::claim::Options::default();
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    match ein_infer::claim::compared(&mut run.kb, &mut run.terms, &run.ast, claim, &run.r, &opts) {
        Ok(c) => c.map(|c| Diffed::of(&c, level)),
        Err(e) => Some(Diffed {
            lines: vec![format!("    diff: not taken — {e}")],
            json: Json::Null,
        }),
    }
}

/// One question asked of a file: the program it was asked of, which a run
/// saturates, what it answered, and how long it took.
struct Asked {
    ast: Ast,
    terms: Terms,
    kb: ein_core::Kb,
    r: ReadOut,
    ms: f64,
    mode: Mode,
    /// Describe's route, where it was asked for the state route.
    route: explore::DescribeRoute,
    config: ein_core::SolverConfig,
    file: String,
}

impl Asked {
    /// The question as `ein solve`'s flags spell it: `--no-search` where the
    /// run read root alone, else the mode's.
    fn flags(&self) -> String {
        if self.r.answer.route == explore::Route::Root {
            "--no-search".to_string()
        } else if self.route == explore::DescribeRoute::States {
            format!("{} --describe-route states", self.mode.flags())
        } else {
            self.mode.flags()
        }
    }
}

/// Ask `mode` of a **freshly loaded** program, deciding its claims
/// ([`explore::Options::refute`]), within `--region-budget`, `--max-steps` and
/// the file's deadline, narrating into `--events`. `Err` is why the run is no verdict.
fn ask(
    path: &Path,
    m: &ArgMatches,
    mode: Mode,
    deadline: Option<Instant>,
    (ast, mut terms, mut kb): (Ast, Terms, ein_core::Kb),
    route: explore::DescribeRoute,
) -> Result<Asked, String> {
    let mut opts = explore::Options::new(mode);
    opts.describe_route = route;
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    opts.deadline = deadline;
    opts.max_steps = Some(crate::modes::max_steps(m));
    opts.refute = true;
    opts.no_search = m.get_flag("no-search");
    // A claim states no core: the run takes one only where `--events` prints
    // it, at each node visited. `ein test` prints no `N`.
    opts.cores = ein_infer::region::Cores {
        visited: m.get_one::<String>("events").is_some(),
        nogoods: false,
    };
    // Nor does a claim read `N`, and no line here prints it or its count:
    // the description is asked without it, but where an artefact that
    // carries the count is written.
    opts.nogoods =
        m.get_one::<String>("events").is_some() || m.get_one::<String>("json-summary").is_some();
    let config = kb.program().config.clone().unwrap_or_default();
    let file = path.display().to_string();
    let mut events = crate::solve::events_start(m, &file, &config);
    crate::solve::events_load(&mut events, &terms, &kb);
    let t0 = Instant::now();
    // Never a pass and never a failure: the claims were neither confirmed
    // nor refuted.
    let aborted = || "aborted before an answer: --max-time".to_string();
    let r = match read_out::run_with(&mut kb, &mut terms, &ast, &opts, &mut events) {
        Ok(r) => r,
        Err(ein_infer::SolveError::Deadline) => return Err(aborted()),
        Err(e) => {
            eprintln!("{e}");
            return Err(e.to_string());
        }
    };
    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    crate::modes::emit_region(&mut events, &r);
    if r.answer.stopped == Some(explore::Stopped::Deadline) {
        return Err(aborted());
    }
    Ok(Asked {
        ast,
        terms,
        kb,
        r,
        ms,
        mode,
        route,
        config,
        file,
    })
}

/// What is printed beside each claim read off one run, and written on its
/// row.
struct Beside {
    /// The warning, beside each claim printed on a flagged program: under
    /// `-v`, and under a claim
    /// that FAILED or was NOT CHECKED.
    warning: Option<String>,
    /// What the universe leaves out (Q-M1e2.3), beside
    /// each claim printed as the warning is: a claim is checked over R, and R
    /// is over H.
    outside: Option<String>,
    /// The same, as data, on the row of every claim the run checked.
    scope: Scope,
}

impl Beside {
    fn of(r: &ReadOut) -> Beside {
        Beside {
            warning: (!r.answer.monotonicity.is_monotone()).then(|| {
                format!(
                    "    {}",
                    ein_render::read_out::warning_line(
                        &r.answer.monotonicity,
                        r.answer.warning.as_deref(),
                        r.answer.root_dead
                    )
                )
            }),
            outside: ein_render::read_out::outside_text(&r.answer.root.outside)
                .map(|t| format!("    outside the universe: {t}")),
            scope: Scope {
                outside: ein_render::dump::region::outside_json(&r.answer.root.outside),
            },
        }
    }

    fn lines(&self) -> impl Iterator<Item = String> {
        [self.warning.clone(), self.outside.clone()]
            .into_iter()
            .flatten()
    }
}
