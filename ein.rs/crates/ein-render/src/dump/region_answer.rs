//! The read-out as data: `--json-summary` as `ein-summary/2` and
//! `--dump-answer DIR` as `ein-answer/2` — M1e2
//! [T1e2.1a.5.2](../../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! fifth step, the fields
//! [`read_out.md` § 11](../../../../../docs/kernel/inference/read_out.md#11---json-summary-the-answer-blocks)
//! defines.
//!
//! **The same facts as stdout**, from the same [`ReadOut`]: what was asked,
//! R's counts and lines and shape, every state the run reached with its
//! model and its lines, each query's classes, the claim, the work. Every key
//! is present on every run, `null` where it does not apply — a consumer
//! switching on `schema` never tests for a key (M1e `SE-M2`'s rule, kept).
//! Every bound sits beside `exhausted`, so no reader has to know which
//! numbers were complete.

use std::path::Path;

use ein_core::{FactId, Kb, Terms};
use ein_infer::SolveError;
use ein_infer::described::Described;
use ein_infer::expect::Outcome;
use ein_infer::explore::{Ask, Route, Stopped, Verdict, Which};
use ein_infer::read_out::{Listed, Listing, ReadOut};
use ein_infer::region::Nodes;
use ein_ir::Ast;

use super::json::{Json, dumps_indent};
use super::serialise::facts_to_ein_text;

/// The summary's schema.
pub const SUMMARY: &str = "ein-summary/2";
/// The answer directory's schema.
pub const ANSWER: &str = "ein-answer/2";

fn strs(v: &[String]) -> Json {
    Json::Array(v.iter().map(|s| Json::str(s.as_str())).collect())
}

fn set_json(cands: &[String], set: &[u32]) -> Json {
    let mut v: Vec<String> = set.iter().map(|&b| cands[b as usize].clone()).collect();
    v.sort();
    strs(&v)
}

fn which(w: Which) -> &'static str {
    match w {
        Which::All => "all",
        Which::Minimal => "minimal",
        Which::Maximal => "maximal",
    }
}

fn lines_of(s: &Listed) -> Json {
    let mut v = Vec::new();
    if s.minimal {
        v.push(Json::str("minimal"));
    }
    if s.maximal {
        v.push(Json::str("maximal"));
    }
    Json::Array(v)
}

fn outcome(o: Outcome) -> &'static str {
    match o {
        Outcome::Held => "held",
        Outcome::Failed => "failed",
        Outcome::NotChecked => "not-checked",
    }
}

/// Why the run stopped, or `null`.
fn stopped(r: &ReadOut, d: Option<&Described>) -> Json {
    let a = &r.answer;
    match &a.stopped {
        None if !a.exhausted && !matches!(a.mode.ask, Ask::Describe | Ask::Count) => {
            Json::obj(vec![("why", Json::str("asked"))])
        }
        None => Json::Null,
        Some(Stopped::Budget { spent, projected }) => {
            let (layer, next) = d
                .and_then(|d| d.stopped_before)
                .map_or((Json::Null, Json::Null), |(l, n)| {
                    (Json::int(l as i64), Json::int(n as i64))
                });
            Json::obj(vec![
                ("why", Json::str("budget")),
                ("spent", Json::int(*spent as i64)),
                ("projected", Json::int(*projected as i64)),
                ("layer", layer),
                ("commitments", next),
            ])
        }
        Some(Stopped::Limit { n, limit }) => Json::obj(vec![
            ("why", Json::str("limit")),
            ("n", Json::int(*n as i64)),
            ("limit", Json::int(i64::from(*limit))),
        ]),
        Some(Stopped::Deadline) => Json::obj(vec![("why", Json::str("deadline"))]),
        Some(Stopped::NoSearch) => Json::obj(vec![("why", Json::str("no-search"))]),
    }
}

/// The `region` block: what the run established about R.
fn region(r: &ReadOut, cands: &[String]) -> Json {
    let a = &r.answer;
    let d = r.described();
    let opt_int = |v: Option<u64>| v.map_or(Json::Null, |n| Json::int(n as i64));
    // `--count` reads no state, so its states are not known here, which is
    // not the same as none: `null`, as every key that does not apply is.
    let states_json = |states: Vec<Json>| {
        if a.mode.ask == Ask::Count {
            Json::Null
        } else {
            Json::Array(states)
        }
    };
    let states: Vec<Json> = r
        .states
        .iter()
        .map(|s| {
            Json::obj(vec![
                ("facts", strs(&s.model)),
                ("closure", set_json(cands, &s.closure)),
                ("from", set_json(cands, &s.rep)),
                (
                    "minimal_generators",
                    Json::Array(s.generators.iter().map(|g| set_json(cands, g)).collect()),
                ),
                ("commitments", opt_int(s.commitments)),
                ("lines", lines_of(s)),
                ("listed", Json::Bool(s.listed)),
            ])
        })
        .collect();
    let (frontier, core, status, refuted_by) = match &r.listing {
        Listing::Owing(f) => (
            Json::Array(
                f.iter()
                    .map(|o| {
                        Json::obj(vec![
                            ("facts", strs(&o.model)),
                            ("set", set_json(cands, &o.node.set)),
                            ("owes", Json::int(i64::from(o.node.owes))),
                            ("owed", strs(&o.node.owed)),
                        ])
                    })
                    .collect(),
            ),
            Json::Null,
            Json::Null,
            Json::Null,
        ),
        Listing::False => {
            let root = d.map(|d| &d.root);
            (
                Json::Null,
                root.map_or(Json::Null, |x| core_field(!x.alive, x.core_status, &x.core)),
                root.map_or(Json::Null, |x| core_status(x.core_status)),
                root.map_or(Json::Null, |x| {
                    core_field(!x.alive, x.core_status, &x.refuted_by)
                }),
            )
        }
        _ => (Json::Null, Json::Null, Json::Null, Json::Null),
    };
    let Some(d) = d else {
        // A mode: R was not described, so R's numbers are `null` — except
        // that R is empty, and of which kind, when the mode established it, as
        // stdout says it.
        let empty = match a.verdict {
            Some(Verdict::Contradiction) => Json::str("dead"),
            Some(Verdict::Open) => Json::str("owing"),
            _ if a.exhausted && a.states.is_empty() && a.mode.ask == Ask::Describe => {
                Json::str(if a.root_dead { "dead" } else { "owing" })
            }
            _ => Json::Null,
        };
        return Json::obj(vec![
            ("k_s", Json::Null),
            ("k_c", Json::Null),
            ("exhausted", Json::Bool(a.exhausted)),
            ("stopped", stopped(r, None)),
            ("complete_up_to", Json::Null),
            ("root", Json::Null),
            ("root_owes", Json::Null),
            ("root_owed", Json::Null),
            ("empty", empty),
            ("minimal", Json::Null),
            ("maximal", Json::Null),
            ("components", Json::Null),
            ("subcubes", Json::Null),
            ("merged", Json::Null),
            ("nogoods", Json::Null),
            ("discharging", Json::Null),
            ("holes", Json::Null),
            (
                "universe",
                Json::obj(vec![
                    ("candidates", Json::int(a.n() as i64)),
                    ("dead_alone", Json::Null),
                ]),
            ),
            ("route", Json::str(route(a.route))),
            ("licences", licences(r)),
            ("states", states_json(states)),
            ("frontier", frontier),
            ("core", core),
            ("core_status", status),
            ("refuted_by", refuted_by),
        ]);
    };
    let root = if d.root.in_r() {
        "in-r"
    } else if d.root.alive {
        "owing"
    } else {
        "dead"
    };
    let empty = if d.exhausted && d.states.is_empty() {
        Json::str(if d.any_alive { "owing" } else { "dead" })
    } else {
        Json::Null
    };
    let least = d.minimal_commitments.iter().map(|s| s.len()).min();
    let n = d.n();
    let top = d
        .states
        .iter()
        .any(|s| s.maximal && s.closure.len() == n && n > 0);
    let shape = d.shape.as_ref();
    let merged: Vec<&ein_infer::described::State> =
        d.states.iter().filter(|s| s.commitments > 1).collect();
    Json::obj(vec![
        ("k_s", Json::int(d.k_s() as i64)),
        ("k_c", Json::int(d.k_c as i64)),
        ("exhausted", Json::Bool(d.exhausted)),
        ("stopped", stopped(r, Some(d))),
        (
            "complete_up_to",
            d.complete_up_to.map_or(Json::Null, |l| Json::int(l as i64)),
        ),
        ("root", Json::str(root)),
        ("root_owes", Json::int(i64::from(d.root.owes))),
        // Each instance root owes, through its rule's `:why`: the read-out's
        // `owed` rows, on every description, `[]` where root owes none (M1f
        // S1f.6.26, E-19).
        ("root_owed", strs(&d.root.owed)),
        ("empty", empty),
        (
            "minimal",
            Json::obj(vec![
                (
                    "states",
                    Json::int(d.states.iter().filter(|s| s.minimal).count() as i64),
                ),
                ("commitments", Json::int(d.minimal_commitments.len() as i64)),
                ("least", least.map_or(Json::Null, |l| Json::int(l as i64))),
                // ∅ is in R, so the line is {∅} (M1f S1f.5.6, S-17 and F1):
                // the row's `1 state — ∅`, read off the same bit.
                ("trivial", Json::Bool(d.root.in_r())),
            ]),
        ),
        (
            "maximal",
            Json::obj(vec![
                (
                    "states",
                    Json::int(d.states.iter().filter(|s| s.maximal).count() as i64),
                ),
                ("trivial", Json::Bool(top && !d.dead_met)),
            ]),
        ),
        (
            "components",
            shape.map_or(Json::Null, |s| {
                Json::Array(s.components.iter().map(|&c| Json::int(c as i64)).collect())
            }),
        ),
        (
            "subcubes",
            shape
                .and_then(|s| s.subcubes.as_ref())
                .map_or(Json::Null, |v| {
                    Json::Array(v.iter().map(|&x| Json::int(x as i64)).collect())
                }),
        ),
        (
            "merged",
            if shape.is_some() && d.exhausted {
                Json::obj(vec![
                    ("states", Json::int(merged.len() as i64)),
                    (
                        "minimal",
                        Json::int(
                            merged
                                .iter()
                                .map(|s| s.generators.len() as i64)
                                .sum::<i64>(),
                        ),
                    ),
                ])
            } else {
                Json::Null
            },
        ),
        (
            "nogoods",
            // A run that took no N has no count to write; every run that
            // writes this block takes it (`ein_infer::explore::Options::nogoods`).
            if d.nogoods_skipped {
                Json::Null
            } else {
                Json::int(d.nogoods as i64)
            },
        ),
        (
            "discharging",
            shape
                .and_then(|s| s.discharging)
                .map_or(Json::Null, |e| Json::int(e as i64)),
        ),
        (
            "holes",
            shape
                .and_then(|s| s.holes.as_ref())
                .map_or(Json::Null, |(c, m)| {
                    Json::obj(vec![
                        ("count", Json::int(*c as i64)),
                        (
                            "minimal",
                            Json::Array(
                                m.iter()
                                    .map(|h| {
                                        Json::obj(vec![
                                            ("set", set_json(cands, &h.set)),
                                            ("dead", Json::Bool(h.dead)),
                                            ("core", core_field(h.dead, h.core_status, &h.core)),
                                            ("core_status", core_status(h.core_status)),
                                            ("owes", Json::int(i64::from(h.owes))),
                                        ])
                                    })
                                    .collect(),
                            ),
                        ),
                    ])
                }),
        ),
        (
            "universe",
            Json::obj(vec![
                ("candidates", Json::int(n as i64)),
                (
                    "dead_alone",
                    if crate::read_out::dead_alone_known(d) {
                        Json::int(d.dead_alone as i64)
                    } else {
                        Json::Null
                    },
                ),
            ]),
        ),
        ("route", Json::str(route(d.route))),
        ("licences", licences(r)),
        ("states", states_json(states)),
        ("frontier", frontier),
        ("core", core),
        ("core_status", status),
        ("refuted_by", refuted_by),
    ])
}

/// A core's status, as every JSON surface writes it beside the core:
/// `within-budget`, `cut-` and the caps its search stopped at
/// ([`ein_infer::explain::CoreStatus::word`]), or `null` where no core was
/// taken (M1e2 S1e2.2.23, W23). M1e2 S1e2.3.9's `not-kept`, beside a member
/// of N a run past `--region-limit` had kept as its two bits, retired with M1f
/// S1f.5.12, which gives every member its core.
pub fn core_status(c: Option<ein_infer::explain::CoreStatus>) -> Json {
    c.map_or(Json::Null, |c| Json::str(c.word()))
}

/// A node's core, or the rules its contradiction's derivation takes, as every
/// JSON surface writes them (M1f S1f.5.8, the user's ruling of 2026-11-07 on
/// D-14, Q-M1e2.27): a core is one object, the set of facts a dead node's
/// contradiction rests on, so a dead node's is a list, and `[]` is the empty
/// set, a core that exists and holds no fact; a node that is not dead has no
/// contradiction, so its core does not exist: `null`. A dead node whose core
/// the run did not take is `null` too, beside a `null` `core_status`, and no
/// surface the binary writes holds one, each asking for the cores it prints
/// ([`ein_infer::region::Cores`]). Until S1f.5.8 a node that is not dead was
/// written `[]`, which reads as *the empty set of facts suffices*: the
/// summary's owing holes, and every such node of `region.json`, `nodes.json`
/// and `--events`' `enter` lines.
pub fn core_field(
    dead: bool,
    status: Option<ein_infer::explain::CoreStatus>,
    v: &[String],
) -> Json {
    if dead && status.is_some() {
        strs(v)
    } else {
        Json::Null
    }
}

fn route(r: Route) -> &'static str {
    match r {
        Route::DownSet => "1",
        Route::Cube => "2",
        Route::States => "states",
        Route::Tree => "tree",
        Route::Root => "root",
    }
}

fn licences(r: &ReadOut) -> Json {
    Json::Array(
        r.answer
            .licences
            .iter()
            .map(|l| Json::str(l.label()))
            .collect(),
    )
}

/// The read-out's blocks of `ein-summary/2`, in order: `asked`, `region`,
/// `verdict`, `queries`, `claim`, `work`. The caller adds `schema` and
/// `source` before them and the program's own blocks after.
/// One binding's value: a JSON string, whatever it spells — a name, a
/// nested fact's s-expression. Since S1f.5.5's swap a run of digits is a name
/// (S1f.5.4's ruling), and S1f.5.4's Q4 (a), decided 2026-11-08, keeps every
/// name a string here, so that `(p 007)` and `(p 7)` stay two. Until then an
/// integer was a JSON number, its digits unclamped (`Json::BigInt` past
/// `i64`), what M1e's fuzzer found the walk's summary quoting. No reader of
/// `ein-summary/2` reads a binding's JSON type, so the schema id stays.
fn binding(v: &str) -> Json {
    Json::str(v)
}

pub fn summary_blocks(r: &ReadOut) -> Vec<(&'static str, Json)> {
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    // `--no-search` (M1f S1f.6.36) asks root alone, and states no R: its
    // `region` is `null`, as `--monotone`'s is, and root is the `root` block's.
    let root_alone = a.route == Route::Root;
    let (ask, n) = match a.mode.ask {
        _ if root_alone => ("no-search", Json::Null),
        Ask::Describe => ("describe", Json::Null),
        Ask::Find(n) => ("find", Json::int(n as i64)),
        Ask::Single => ("single", Json::Null),
        Ask::Count => ("count", Json::Null),
    };
    let verdict = match (a.mode.ask, a.verdict) {
        (Ask::Single, Some(v)) => Json::obj(vec![
            ("word", Json::str(v.word())),
            ("which", Json::str(which(a.mode.which))),
        ]),
        (Ask::Single, None) => Json::obj(vec![
            ("word", Json::Null),
            ("which", Json::str(which(a.mode.which))),
        ]),
        _ => Json::Null,
    };
    let queries: Vec<Json> = r
        .queries
        .iter()
        .map(|q| {
            // What the classes are over: the set the query's `:over` names,
            // or the states the run listed when it asked another.
            let over = match (q.as_asked, q.over) {
                (false, _) => "listed",
                (true, ein_ir::expect::Over::All) => "all",
                (true, ein_ir::expect::Over::Minimal) => "minimal",
                (true, ein_ir::expect::Over::Maximal) => "maximal",
            };
            Json::obj(vec![
                ("goal", Json::str(q.goal.as_str())),
                ("over", Json::str(over)),
                ("exact", Json::Bool(q.exact)),
                (
                    "classes",
                    Json::Array(
                        q.classes
                            .iter()
                            .map(|c| {
                                Json::obj(vec![
                                    ("facts", strs(&c.facts)),
                                    (
                                        "rows",
                                        Json::Array(
                                            c.rows
                                                .iter()
                                                .map(|row| {
                                                    Json::Object(
                                                        row.iter()
                                                            .map(|(k, v)| (k.clone(), binding(v)))
                                                            .collect(),
                                                    )
                                                })
                                                .collect(),
                                        ),
                                    ),
                                    (
                                        "states",
                                        Json::Array(
                                            c.states
                                                .iter()
                                                .map(|&i| Json::int(i as i64 + 1))
                                                .collect(),
                                        ),
                                    ),
                                ])
                            })
                            .collect(),
                    ),
                ),
                // The query's own `:expect`, checked against its classes.
                (
                    "expect",
                    q.claim.as_ref().map_or(Json::Null, |c| {
                        Json::obj(vec![
                            ("outcome", Json::str(outcome(c.outcome))),
                            ("lines", strs(&c.lines)),
                        ])
                    }),
                ),
            ])
        })
        .collect();
    let claim = r.claim.as_ref().map_or(Json::Null, |c| {
        Json::obj(vec![
            ("outcome", Json::str(outcome(c.outcome))),
            ("lines", strs(&c.lines)),
        ])
    });
    let (saturations, facts, layers) = match r.described() {
        Some(d) => (d.work.saturations, d.work.facts, d.work.layers),
        None => (a.saturations, a.facts, None),
    };
    vec![
        (
            "asked",
            Json::obj(vec![
                ("which", Json::str(which(a.mode.which))),
                ("ask", Json::str(ask)),
                ("n", n),
            ]),
        ),
        (
            "region",
            if root_alone {
                Json::Null
            } else {
                region(r, &cands)
            },
        ),
        ("verdict", verdict),
        ("queries", Json::Array(queries)),
        ("claim", claim),
        (
            "work",
            Json::obj(vec![
                ("saturations", Json::int(saturations as i64)),
                ("facts", Json::int(facts as i64)),
                ("layers", layers.map_or(Json::Null, |l| Json::int(l as i64))),
            ]),
        ),
    ]
}

/// What the answer directory is written from, beside the read-out.
#[derive(Clone, Copy, Debug)]
pub struct Inputs<'a> {
    /// The source path, as given.
    pub source: &'a str,
    /// The program, imports inlined — the head of every state file.
    pub program: &'a str,
    /// The summary's bytes, which `answer.json` is.
    pub summary: &'a str,
    /// The read-out with no listing cut, which `answer.txt` is.
    pub answer_txt: &'a str,
    /// Each form the run printed after its read-out, uncut, by its name:
    /// `forms/<name>.txt` (M1e2 S1e2.3.3, the user's ruling (a) of
    /// 2026-11-03).
    pub forms: &'a [(&'a str, String)],
}

/// `NN` of `of`, zero-padded to the width of `of`.
fn nn(i: usize, of: usize) -> String {
    let width = of.max(1).to_string().len().max(2);
    format!("{:0width$}", i + 1)
}

/// The subtrees this writer owns: the state files, and since M1e2 S1e2.3.3
/// the forms.
const SUBTREES: [&str; 3] = ["models", "owing", "forms"];

/// Remove what a previous `ein-answer/2` write left, and nothing else: a
/// directory whose `index.json` names another schema, or none, is written
/// beside and not cleared.
pub fn clear_previous(dir: &Path) -> std::io::Result<()> {
    let ours = std::fs::read_to_string(dir.join("index.json"))
        .is_ok_and(|t| t.contains(&format!("\"schema\": \"{ANSWER}\"")));
    if ours {
        for sub in SUBTREES {
            match std::fs::remove_dir_all(dir.join(sub)) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
                _ => {}
            }
        }
    }
    Ok(())
}

/// `ein solve --count --dump-answer DIR` (M1e2 T1e2.1c.4.6): the read-out
/// uncut as `answer.txt` and the summary as `answer.json`, and nothing else,
/// since `--count` reads no state. What a previous `ein-answer/2` write left,
/// its `index.json` and `nogoods.json` among it, is removed, so the directory
/// names no state file that is not there.
pub fn write_counted(dir: &Path, answer_txt: &str, summary: &str) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let ours = std::fs::read_to_string(dir.join("index.json"))
        .is_ok_and(|t| t.contains(&format!("\"schema\": \"{ANSWER}\"")));
    clear_previous(dir).map_err(io)?;
    if ours {
        for file in ["index.json", "nogoods.json"] {
            match std::fs::remove_file(dir.join(file)) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(io(e)),
                _ => {}
            }
        }
    }
    std::fs::write(dir.join("answer.txt"), answer_txt).map_err(io)?;
    std::fs::write(dir.join("answer.json"), summary).map_err(io)
}

/// `ein solve --no-search --dump-answer DIR` (M1f S1f.6.36): the read-out
/// uncut as `answer.txt`, the summary as `answer.json`, root's state as
/// `root.ein`, loadable, as a state file is, whatever root is, and each form
/// asked for under `forms/`. No state of R is written, and no `index.json` or
/// `nogoods.json`, which a previous `ein-answer/2` write's are removed with,
/// as `--count`'s are: R was not searched.
pub fn write_root(
    dir: &Path,
    r: &ReadOut,
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    inputs: &Inputs,
) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    write_counted(dir, inputs.answer_txt, inputs.summary)?;
    let a = &r.answer;
    let standing = match r.described().map(|d| &d.root) {
        Some(x) if x.in_r() => "in R — alive, and owes nothing".to_string(),
        Some(x) if x.alive => format!("not in R — owes {}", x.owes),
        Some(_) => "dead — its own fixpoint derives (false)".to_string(),
        None => "not read".to_string(),
    };
    let mut head = vec![
        format!("root — written by `ein solve --no-search --dump-answer` ({ANSWER})."),
        format!("source:     {}", inputs.source),
        format!("root:       {standing}"),
        "R:          not searched".to_string(),
    ];
    if let Some(d) = r.described() {
        head.extend(d.root.owed.iter().map(|w| format!("  {w}")));
    }
    let mut nodes = ein_infer::region::Nodes::new(root, terms, ast, &a.candidates)
        .map_err(|e| e.to_string())?;
    let text = nodes
        .with_fixpoint(&[], |kb, terms| state_file(terms, kb, inputs, &head))
        .map_err(|e| e.to_string())?;
    std::fs::write(dir.join("root.ein"), text).map_err(io)?;
    if !inputs.forms.is_empty() {
        std::fs::create_dir_all(dir.join("forms")).map_err(io)?;
    }
    for (form, text) in inputs.forms {
        std::fs::write(dir.join(format!("forms/{form}.txt")), text).map_err(io)?;
    }
    Ok(())
}

fn state_file(terms: &Terms, kb: &Kb, inputs: &Inputs, head: &[String]) -> String {
    // The program's declarations and the loader's `(written T)` are stated
    // again when the file is loaded (M1f S1f.6.38), and a file that wrote a
    // `written` fact would be refused.
    let (relation, written) = (terms.kernel.relation, terms.kernel.written);
    let mut state: Vec<FactId> = kb
        .facts()
        .filter(|&f| ![relation, written].contains(&terms.facts.get(f).0))
        .collect();
    state.sort_by_cached_key(|&f| ein_infer::events::sexpr(terms, f));
    let mut text = String::new();
    for h in head {
        text.push_str(";;; ");
        text.push_str(h);
        text.push('\n');
    }
    // Said only where there are any, so a state file of a program that writes
    // no nested fact reads as it did before S1f.6.38.
    let stated = if kb.facts().any(|f| terms.facts.get(f).0 == written) {
        "\n;;; The loader states (written T) for each fact nested in a fact the file\n\
         ;;; holds, so those are not repeated either."
    } else {
        ""
    };
    format!(
        "{text};;;\n\
         ;;; Below: the program as loaded, imports inlined, and then every fact of\n\
         ;;; the state ({} facts; the program's own relation declarations are not\n\
         ;;; repeated). It loads, and saturating it derives nothing new: the state is\n\
         ;;; a fixpoint of the program — read_out.md § 8.2.{stated}\n\n\
         {}\n\
         ;; ── the state ──────────────────────────────────────────────────────────\n\n\
         {}",
        state.len(),
        inputs.program.trim_end(),
        facts_to_ein_text(kb, terms, &state),
    )
}

/// `nogoods.json`: each member of `N` a describing run met, with its unsat
/// core, the core's status and the rules that refuted it, `[]` when the run
/// described nothing. The run takes every member's core, after its walk where
/// the walk took none, through the node function against the walk's snapshot,
/// past `--region-limit` too (M1f S1f.5.12, P-15 (c):
/// `ein_infer::region::Cores::nogoods`). A member the run carries no core for
/// was described by a run not asked for `N`'s cores, which the binary never
/// writes: refused, never printed as a core. Until S1f.5.12 a member the state
/// route did not saturate was saturated here, through a second preparation of
/// root, and a member a run past the limit met was written `not-kept`.
pub fn nogoods(r: &ReadOut) -> Result<Json, String> {
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let mut dead: Vec<Json> = Vec::new();
    if let Some(d) = r.described() {
        for x in &d.dead {
            if x.core_status.is_none() {
                return Err(format!(
                    "N's member {} carries no core: the run was not asked for N's cores \
                     (region::Cores::nogoods)",
                    a.name(&x.set)
                ));
            }
            dead.push(Json::obj(vec![
                ("set", set_json(&cands, &x.set)),
                ("core", core_field(true, x.core_status, &x.core)),
                ("core_status", core_status(x.core_status)),
                ("refuted_by", core_field(true, x.core_status, &x.refuted_by)),
            ]));
        }
    }
    Ok(Json::Array(dead))
}

/// Write `ein-answer/2` into `dir`, which exists: `answer.txt`, `answer.json`,
/// one loadable program per state the run reached under `models/` (listed or
/// not: a partial run's other members are where its cut says they are), the
/// owing frontier of an empty R under `owing/`, `nogoods.json`, each form the
/// run printed after its read-out, uncut, under `forms/` (M1e2 S1e2.3.3), and
/// `index.json` naming what each file is. Each state is read again through the
/// node function every route uses.
pub fn write(
    dir: &Path,
    r: &ReadOut,
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    inputs: &Inputs,
) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let solve = |e: SolveError| e.to_string();
    clear_previous(dir).map_err(io)?;
    std::fs::write(dir.join("answer.txt"), inputs.answer_txt).map_err(io)?;
    std::fs::write(dir.join("answer.json"), inputs.summary).map_err(io)?;
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let name = |set: &[u32]| -> String {
        if set.is_empty() {
            "∅".to_string()
        } else {
            let v: Vec<&str> = set.iter().map(|&b| cands[b as usize].as_str()).collect();
            format!("{{{}}}", v.join(", "))
        }
    };
    let facts = |set: &[u32]| -> Vec<FactId> {
        set.iter().map(|&b| a.candidates[b as usize].fact).collect()
    };
    let owing: &[ein_infer::read_out::OwingState] = match &r.listing {
        Listing::Owing(f) => f,
        _ => &[],
    };
    let mut index_states: Vec<Json> = Vec::new();
    let mut index_owing: Vec<Json> = Vec::new();
    if !r.states.is_empty() || !owing.is_empty() {
        let mut nodes = Nodes::new(root, terms, ast, &a.candidates).map_err(solve)?;
        if !r.states.is_empty() {
            std::fs::create_dir_all(dir.join("models")).map_err(io)?;
        }
        let k = r.states.len();
        for (i, s) in r.states.iter().enumerate() {
            let path = format!("models/{}.ein", nn(i, k));
            let lines: Vec<&str> = [(s.minimal, "minimal"), (s.maximal, "maximal")]
                .into_iter()
                .filter_map(|(on, l)| on.then_some(l))
                .collect();
            let mut head = vec![
                format!(
                    "state {} of {k} — written by `ein solve --dump-answer` ({ANSWER}).",
                    i + 1
                ),
                format!("source:     {}", inputs.source),
                format!("from:       {}", name(&s.rep)),
                format!(
                    "lines:      {}",
                    if lines.is_empty() {
                        "—".to_string()
                    } else {
                        lines.join(", ")
                    }
                ),
            ];
            if let Some(c) = s.commitments {
                let more = if c >= ein_infer::count::CAP {
                    " or more"
                } else {
                    ""
                };
                head.push(format!("reached by: {c}{more} commitments"));
            }
            if !s.generators.is_empty() {
                head.push(format!(
                    "minimal:    {}",
                    s.generators
                        .iter()
                        .map(|g| name(g))
                        .collect::<Vec<_>>()
                        .join(" | ")
                ));
            }
            let text = nodes
                .with_fixpoint(&facts(&s.rep), |kb, terms| {
                    state_file(terms, kb, inputs, &head)
                })
                .map_err(solve)?;
            std::fs::write(dir.join(&path), text).map_err(io)?;
            index_states.push(Json::obj(vec![
                ("path", Json::str(path)),
                ("state", Json::int(i as i64 + 1)),
                ("lines", lines_of(s)),
                ("listed", Json::Bool(s.listed)),
            ]));
        }
        if !owing.is_empty() {
            std::fs::create_dir_all(dir.join("owing")).map_err(io)?;
        }
        for (i, o) in owing.iter().enumerate() {
            let path = format!("owing/{}.ein", nn(i, owing.len()));
            let mut head = vec![
                format!(
                    "an owing node of an empty R — written by `ein solve --dump-answer` \
                     ({ANSWER})."
                ),
                format!("source:     {}", inputs.source),
                format!("commitment: {}", name(&o.node.set)),
                format!("owes:       {}", o.node.owes),
            ];
            head.extend(o.node.owed.iter().map(|w| format!("  {w}")));
            let text = nodes
                .with_fixpoint(&facts(&o.node.set), |kb, terms| {
                    state_file(terms, kb, inputs, &head)
                })
                .map_err(solve)?;
            std::fs::write(dir.join(&path), text).map_err(io)?;
            index_owing.push(Json::obj(vec![("path", Json::str(path))]));
        }
    }
    // Each form uncut, the stdout form's rest (M1e2 S1e2.3.3).
    let mut index_forms: Vec<Json> = Vec::new();
    if !inputs.forms.is_empty() {
        std::fs::create_dir_all(dir.join("forms")).map_err(io)?;
    }
    for (form, text) in inputs.forms {
        let path = format!("forms/{form}.txt");
        std::fs::write(dir.join(&path), text).map_err(io)?;
        index_forms.push(Json::obj(vec![
            ("form", Json::str(*form)),
            ("path", Json::str(path)),
        ]));
    }
    std::fs::write(dir.join("nogoods.json"), dumps_indent(&nogoods(r)?) + "\n").map_err(io)?;
    let blocks = summary_blocks(r);
    let get = |k: &str| {
        blocks
            .iter()
            .find(|(key, _)| *key == k)
            .map(|(_, v)| v.clone())
            .unwrap_or(Json::Null)
    };
    let index = Json::obj(vec![
        ("schema", Json::str(ANSWER)),
        ("source", Json::str(inputs.source)),
        ("asked", get("asked")),
        ("region", get("region")),
        ("models", Json::Array(index_states)),
        ("owing", Json::Array(index_owing)),
        ("forms", Json::Array(index_forms)),
    ]);
    std::fs::write(dir.join("index.json"), dumps_indent(&index) + "\n").map_err(io)?;
    Ok(())
}
