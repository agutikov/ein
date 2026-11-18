//! The read-out as data: `--json-summary` as `ein-summary/2` and
//! `--dump-answer DIR` as `ein-answer/3`, the fields
//! [`08-read-out.md` § 11](../../../../../docs/ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks)
//! and [`10-answer-dump.md`](../../../../../docs/ein/user/04-reference/10-answer-dump.md)
//! define.
//!
//! **The same facts as stdout**, from the same [`ReadOut`]: what was asked,
//! R's counts and lines and shape, every state the run reached with its
//! model and its lines, each query's classes, the claim, the work. Every key
//! is present on every run, `null` where it does not apply — a consumer
//! switching on `schema` never tests for a key.
//! Every bound sits beside `exhausted`, so no reader has to know which
//! numbers were complete.

use std::path::Path;

use ein_core::Kb;
use ein_infer::described::Described;
use ein_infer::expect::Outcome;
use ein_infer::explore::{Ask, Route, Stopped, Verdict, Which};
use ein_infer::read_out::{Listed, Listing, ReadOut};

use super::json::{Json, dumps_indent};

/// The summary's schema.
pub const SUMMARY: &str = "ein-summary/2";
/// The answer directory's schema.
pub const ANSWER: &str = "ein-answer/3";

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
        // `owed` rows, on every description, `[]` where root owes none.
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
                // ∅ is in R, so the line is {∅}:
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
        // An owing frontier the route did not compute is not written as if
        // it were one: `null`, as `nogoods` is (the listing names the members
        // met, and `--dump-answer` writes them under `owing/`).
        (
            "frontier",
            if d.frontier_skipped {
                Json::Null
            } else {
                frontier
            },
        ),
        ("core", core),
        ("core_status", status),
        ("refuted_by", refuted_by),
    ])
}

/// A core's status, as every JSON surface writes it beside the core:
/// `within-budget`, `cut-` and the caps its search stopped at
/// ([`ein_infer::explain::CoreStatus::word`]), or `null` where no core was
/// taken. Every member of N carries its core (Q-M1f.26), past
/// `--region-limit` too, so no status says a core was not kept.
pub fn core_status(c: Option<ein_infer::explain::CoreStatus>) -> Json {
    c.map_or(Json::Null, |c| Json::str(c.word()))
}

/// A node's core, or the rules its contradiction's derivation takes, as every
/// JSON surface writes them (Q-M1e2.27): a core is one object, the set of facts a dead node's
/// contradiction rests on, so a dead node's is a list, and `[]` is the empty
/// set, a core that exists and holds no fact; a node that is not dead has no
/// contradiction, so its core does not exist: `null`. A dead node whose core
/// the run did not take is `null` too, beside a `null` `core_status`, and no
/// surface the binary writes holds one, each asking for the cores it prints
/// ([`ein_infer::region::Cores`]). A node that is not dead is never written
/// `[]`, which would read as *the empty set of facts suffices*: not the
/// summary's owing holes, nor any such node of `region.json`, `nodes.json`
/// or `--events`' `enter` lines.
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
        Route::TreeClasses => "tree-classes",
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

/// One binding's value: a JSON string, whatever it spells — a name, a
/// nested fact's s-expression. A run of digits is a name, and every name is a
/// string here (Q-M1f.340), so that `(p 007)` and `(p 7)` stay two, and no
/// number's digits are clamped to a JSON number's range.
fn binding(v: &str) -> Json {
    Json::str(v)
}

/// The read-out's blocks of `ein-summary/2`, in order: `asked`, `region`,
/// `verdict`, `queries`, `claim`, `work`. The caller adds `schema` and
/// `source` before them and the program's own blocks after.
pub fn summary_blocks(r: &ReadOut) -> Vec<(&'static str, Json)> {
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    // `--no-search` asks root alone, and states no R (Q-M1f.259): its
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
    /// The summary's bytes, which `answer.json` is.
    pub summary: &'a str,
    /// The read-out with no listing cut, which `answer.txt` is.
    pub answer_txt: &'a str,
    /// Each form the run printed after its read-out, uncut, by its name:
    /// `forms/<name>.txt` (Q-M1e2.34).
    pub forms: &'a [(&'a str, String)],
}

/// `NN` of `of`, zero-padded to the width of `of`.
fn nn(i: usize, of: usize) -> String {
    let width = of.max(1).to_string().len().max(2);
    format!("{:0width$}", i + 1)
}

/// The subtrees this writer owns: the state files and the forms.
const SUBTREES: [&str; 3] = ["models", "owing", "forms"];

/// Whether `dir`'s `index.json` is an answer directory's, of this schema or
/// an earlier one: what this writer clears before it writes.
fn ours(dir: &Path) -> bool {
    std::fs::read_to_string(dir.join("index.json"))
        .is_ok_and(|t| t.contains("\"schema\": \"ein-answer/"))
}

/// Remove what a previous answer directory's write left, and nothing else: a
/// directory whose `index.json` names another schema, or none, is written
/// beside and not cleared.
pub fn clear_previous(dir: &Path) -> std::io::Result<()> {
    let ours = ours(dir);
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

/// `ein solve --count --dump-answer DIR`: the read-out
/// uncut as `answer.txt` and the summary as `answer.json`, and nothing else,
/// since `--count` reads no state. What a previous answer directory's write left,
/// its `index.json` and `nogoods.json` among it, is removed, so the directory
/// names no state file that is not there.
pub fn write_counted(dir: &Path, answer_txt: &str, summary: &str) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let ours = ours(dir);
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

/// `ein solve --no-search --dump-answer DIR`: the read-out
/// uncut as `answer.txt`, the summary as `answer.json`, root's node saved as
/// `root.ein` ([`saved_file`]), whatever root is: a program's root, or the
/// node a saved file names, which is its file again byte for byte. And each
/// form asked for under `forms/`. No state of R is written, and no
/// `index.json` or `nogoods.json`, which a previous write's are removed
/// with, as `--count`'s are: R was not searched.
pub fn write_root(dir: &Path, r: &ReadOut, root: &Kb, inputs: &Inputs) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    write_counted(dir, inputs.answer_txt, inputs.summary)?;
    std::fs::write(dir.join("root.ein"), root_file(r, root)?).map_err(io)?;
    if !inputs.forms.is_empty() {
        std::fs::create_dir_all(dir.join("forms")).map_err(io)?;
    }
    for (form, text) in inputs.forms {
        std::fs::write(dir.join(format!("forms/{form}.txt")), text).map_err(io)?;
    }
    Ok(())
}

/// **Root's node saved**, as `--no-search --dump-answer` writes it in
/// `root.ein`: the program with no `(commit …)`, or a saved file's own node,
/// which is that file again.
pub fn root_file(r: &ReadOut, root: &Kb) -> Result<String, String> {
    let a = &r.answer;
    saved_file(root, &a.root.commitment(&a.candidates, &[]))
}

/// **A node saved** (Q-M1f.400 to Q-M1f.402): the program as it was read,
/// every module its load read, each as it was read, and `commit`, the
/// candidates of H the node commits, in H's order ([`ein_ir::saved::write`]).
/// Nothing of the state is in it: a load derives it again, by the rules that
/// derived it, from the same choices. `root` is the run's: its
/// [`ein_core::Program::sources`] is what its load read, a saved file's
/// carried texts among them.
pub fn saved_file(root: &Kb, commit: &[String]) -> Result<String, String> {
    let Some(sources) = root.program().sources.as_deref() else {
        return Err(
            "the program was loaded from forms and not from a text, so no file of its \
                    states can carry it"
                .to_string(),
        );
    };
    let commit: Vec<&str> = commit.iter().map(String::as_str).collect();
    Ok(ein_ir::saved::write(sources, &commit))
}

/// `nogoods.json`: each member of `N` a describing run met, with its unsat
/// core, the core's status and the rules that refuted it, `[]` when the run
/// described nothing. The run takes every member's core, after the route
/// where the route took none, through the node function against the route's
/// snapshot, past `--region-limit` too (Q-M1f.26,
/// `ein_infer::region::Cores::nogoods`). A member the run carries no core for
/// was described by a run not asked for `N`'s cores, which the binary never
/// writes: refused, never printed as a core. No member is saturated here.
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

/// Write the answer directory into `dir`, which exists: `answer.txt`,
/// `answer.json`, one saved file per state the run reached under `models/`
/// (listed or not: a partial run's other members are where its cut says they
/// are), the owing frontier of an empty R under `owing/`, `nogoods.json`,
/// each form the run printed after its read-out, uncut, under `forms/`, and
/// `index.json` naming what each file is. A state's file is the program
/// with the commitment the state was read from ([`saved_file`]); no state is
/// read to write it.
pub fn write(dir: &Path, r: &ReadOut, root: &Kb, inputs: &Inputs) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    clear_previous(dir).map_err(io)?;
    std::fs::write(dir.join("answer.txt"), inputs.answer_txt).map_err(io)?;
    std::fs::write(dir.join("answer.json"), inputs.summary).map_err(io)?;
    let a = &r.answer;
    let owing: &[ein_infer::read_out::OwingState] = match &r.listing {
        Listing::Owing(f) => f,
        _ => &[],
    };
    let mut index_states: Vec<Json> = Vec::new();
    let mut index_owing: Vec<Json> = Vec::new();
    if !r.states.is_empty() {
        std::fs::create_dir_all(dir.join("models")).map_err(io)?;
    }
    let k = r.states.len();
    for (i, s) in r.states.iter().enumerate() {
        let path = format!("models/{}.ein", nn(i, k));
        let commit = a.root.commitment(&a.candidates, &s.rep);
        std::fs::write(dir.join(&path), saved_file(root, &commit)?).map_err(io)?;
        index_states.push(Json::obj(vec![
            ("path", Json::str(path)),
            ("state", Json::int(i as i64 + 1)),
            ("lines", lines_of(s)),
            ("listed", Json::Bool(s.listed)),
            ("commit", strs(&commit)),
        ]));
    }
    if !owing.is_empty() {
        std::fs::create_dir_all(dir.join("owing")).map_err(io)?;
    }
    for (i, o) in owing.iter().enumerate() {
        let path = format!("owing/{}.ein", nn(i, owing.len()));
        let commit = a.root.commitment(&a.candidates, &o.node.set);
        std::fs::write(dir.join(&path), saved_file(root, &commit)?).map_err(io)?;
        index_owing.push(Json::obj(vec![
            ("path", Json::str(path)),
            ("commit", strs(&commit)),
        ]));
    }
    // Each form uncut, the stdout form's rest.
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
