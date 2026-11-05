//! An `ein oracle --dump` directory read back, and a region run joined to the
//! cube — M1e2 [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md)
//! drew the cube, and [S1e2.1a.7](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md)
//! put a region run on it.
//!
//! `ein render region DIR` draws from the files the commands wrote, never from
//! a second solve, so the picture is of the run whose answer it shows. The
//! cube is `ein oracle --dump`'s `region.json`; the run is `ein solve`'s
//! `--dump-lattice` (`nodes.json`, every node it visited) and `--dump-answer`
//! (`index.json`, every state it reported). They are joined here, in Rust,
//! because what the overlay reports — *missing*, *spurious*, the run's reading
//! of a node against the cube's — is a comparison, and the review bundle does
//! not compute.
//!
//! ### The join
//!
//! Every commitment in the three files is rendered fact text over the same
//! config-free candidate list, so a commitment's node is the mask of its
//! facts. Two nodes are **the same state** when the oracle gave them one
//! `class` (one fixpoint), and every comparison with R is by state, not by
//! mask: a state the run reports sits on its closure, the node its fixpoint's
//! candidates make ([`NodeDoc::closure`]), and every commitment of that class
//! is answered by it.
//!
//! ### What is compared, under ruling (a)
//!
//! The answer to a program is R (S1e2.1a.3's ruling (a), 2026-10-16), and a
//! run answers the part of R its question asked: all of it, its minimal line
//! or its maximal line (`asked.which`). A node of that part whose state no
//! reported state is, is **missing** when the run says it is complete
//! (`exhausted`) and **not reached** when it stopped, which is a bound and not
//! a defect (`solve_interface.md`'s C3). A reported state whose state is no
//! node of that part is **spurious**. Until S1e2.1a.7 the overlay was the
//! walk's, and compared it with each line separately, because ruling (a) was
//! not yet taken.

use std::path::Path;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::dump::json::{Json, parse};

/// The status bits, as `region.json` spells them.
pub const DEAD: u8 = 1;
pub const OWING: u8 = 2;

/// One node of the cube.
#[derive(Clone, Debug, Default)]
pub struct NodeDoc {
    pub status: u8,
    pub minimal: bool,
    pub maximal: bool,
    pub hole: bool,
    pub component: Option<u32>,
    pub class: u64,
    pub closure: u64,
    pub excluded: u64,
    pub beyond: u32,
    pub beyond_hypothesised: u32,
    pub owes: u32,
    pub owed: Vec<String>,
    pub core: Vec<String>,
    /// `region.json`'s `core_status`, read back; `None` where no core was
    /// taken (M1e2 S1e2.2.23).
    pub core_status: Option<ein_infer::explain::CoreStatus>,
    pub refuted_by: Vec<String>,
    pub absent_premises: Vec<String>,
}

/// `region.json`'s `region` block.
#[derive(Clone, Debug, Default)]
pub struct Cube {
    pub nodes: Vec<NodeDoc>,
    pub root_dead: bool,
    pub size_r: u64,
    pub minimal: Vec<u64>,
    pub maximal: Vec<u64>,
    pub holes: Vec<u64>,
    pub minimal_holes: Vec<u64>,
    /// `(hole, bit, node of R)`.
    pub repairs: Vec<(u64, usize, u64)>,
    pub components: Vec<u64>,
    pub subcubes: Option<Vec<(u64, u64)>>,
    pub nogoods: Vec<u64>,
    pub discharging: Vec<u64>,
    pub local_max: Vec<u64>,
    pub local_min: Vec<u64>,
    pub dead_not_upward_closed: u64,
    pub discharged_not_upward_closed: u64,
}

/// `region.json`.
/// What a site puts at stake, in the read-out's words, from `region.json`'s
/// key; an unknown key as it is.
fn at_stake_words(key: &str) -> String {
    ein_infer::detector::AtStake::from_key(key)
        .map_or_else(|| key.to_string(), |a| a.words().to_string())
}

#[derive(Clone, Debug, Default)]
pub struct RegionDoc {
    pub source: String,
    pub n: usize,
    pub limit: i64,
    pub candidates: Vec<String>,
    pub monotone: bool,
    /// Each site, rendered `rule · kind · relation`, with what it puts at
    /// stake where `region.json` says (M1e2 S1e2.1b.5).
    pub sites: Vec<String>,
    /// The detector's word for the cube's half: `exercised`, `conservative`
    /// or `not known`; `None` on a monotone program or an older dump.
    pub status: Option<String>,
    /// How many nodes of the cube hold a stale fact, where `region.json`
    /// says.
    pub stale_nodes: Option<u64>,
    /// `Some(note)` when the oracle refused the cube.
    pub refused: Option<String>,
    pub cube: Option<Cube>,
}

fn strings(v: Option<&Json>) -> Vec<String> {
    v.map(|j| {
        j.items()
            .iter()
            .filter_map(|x| x.as_str().map(str::to_string))
            .collect()
    })
    .unwrap_or_default()
}

fn int(v: Option<&Json>) -> i64 {
    v.and_then(Json::as_i64).unwrap_or(0)
}

fn mask_of(v: Option<&Json>) -> Option<u64> {
    v.and_then(Json::as_i64).map(|m| m as u64)
}

impl RegionDoc {
    pub fn from_json(j: &Json) -> Result<RegionDoc, String> {
        if j.get("schema").and_then(Json::as_str) != Some("ein-region/1") {
            return Err("region.json: not an ein-region/1 document".into());
        }
        let candidates = strings(j.get("candidates"));
        let bit: FxHashMap<&str, usize> = candidates
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str(), i))
            .collect();
        let set = |v: &Json| -> Result<u64, String> {
            v.items().iter().try_fold(0u64, |acc, f| {
                let t = f.as_str().unwrap_or("");
                bit.get(t)
                    .map(|&b| acc | 1 << b)
                    .ok_or_else(|| format!("region.json: `{t}` is not a candidate"))
            })
        };
        let sets = |v: Option<&Json>| -> Result<Vec<u64>, String> {
            v.map(|v| v.items().iter().map(set).collect())
                .unwrap_or(Ok(Vec::new()))
        };
        let m = j.get("monotonicity");
        let sites = m
            .and_then(|m| m.get("sites"))
            .map(|s| {
                s.items()
                    .iter()
                    .map(|x| {
                        let f = |k| x.get(k).and_then(Json::as_str).unwrap_or("?");
                        let stake = match x.get("at_stake").and_then(Json::as_str) {
                            Some(k) => format!(" ({})", at_stake_words(k)),
                            None => String::new(),
                        };
                        format!("{} · {} · {}{stake}", f("rule"), f("kind"), f("relation"))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut doc = RegionDoc {
            source: j
                .get("source")
                .and_then(Json::as_str)
                .unwrap_or("")
                .to_string(),
            n: int(j.get("n")) as usize,
            limit: int(j.get("limit")),
            candidates: candidates.clone(),
            monotone: m
                .and_then(|m| m.get("monotone"))
                .and_then(Json::as_bool)
                .unwrap_or(true),
            sites,
            status: m
                .and_then(|m| m.get("status"))
                .and_then(|s| s.get("bit"))
                .and_then(Json::as_str)
                .map(str::to_string),
            stale_nodes: m
                .and_then(|m| m.get("state"))
                .and_then(|s| s.get("stale_nodes"))
                .and_then(Json::as_i64)
                .map(|n| n as u64),
            refused: j
                .get("refused")
                .and_then(|r| r.get("note"))
                .and_then(Json::as_str)
                .map(str::to_string),
            cube: None,
        };
        let Some(r) = j.get("region").filter(|r| !r.is_null()) else {
            return Ok(doc);
        };
        let size = 1usize << doc.n;
        let mut nodes = vec![NodeDoc::default(); size];
        for x in r.get("nodes").map(Json::items).unwrap_or(&[]) {
            let mk = int(x.get("mask")) as usize;
            if mk >= size {
                return Err(format!(
                    "region.json: node mask {mk} outside the {}-cube",
                    doc.n
                ));
            }
            let status = match x.get("status").and_then(Json::as_str) {
                Some("R") => 0,
                Some("owing") => OWING,
                Some("dead") => DEAD,
                Some("dead+owing") => DEAD | OWING,
                other => return Err(format!("region.json: status {other:?}")),
            };
            let flag = |k| x.get(k).and_then(Json::as_bool).unwrap_or(false);
            nodes[mk] = NodeDoc {
                status,
                minimal: flag("minimal"),
                maximal: flag("maximal"),
                hole: flag("hole"),
                component: x.get("component").and_then(Json::as_i64).map(|c| c as u32),
                class: int(x.get("class")) as u64,
                closure: mask_of(x.get("closure")).unwrap_or(mk as u64),
                excluded: mask_of(x.get("excluded")).unwrap_or(0),
                beyond: int(x.get("beyond")) as u32,
                beyond_hypothesised: int(x.get("beyond_hypothesised")) as u32,
                owes: int(x.get("owes")) as u32,
                owed: strings(x.get("owed")),
                core: strings(x.get("core")),
                core_status: x
                    .get("core_status")
                    .and_then(Json::as_str)
                    .and_then(ein_infer::explain::CoreStatus::from_word),
                refuted_by: strings(x.get("refuted_by")),
                absent_premises: strings(x.get("absent_premises")),
            };
        }
        let repairs = r
            .get("repairs")
            .map(Json::items)
            .unwrap_or(&[])
            .iter()
            .map(|x| {
                let h = set(x.get("hole").unwrap_or(&Json::Null))?;
                let y = set(x.get("into").unwrap_or(&Json::Null))?;
                let adds = x.get("adds").and_then(Json::as_str).unwrap_or("");
                let b = *bit
                    .get(adds)
                    .ok_or_else(|| format!("region.json: repair adds `{adds}`"))?;
                Ok((h, b, y))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let subcubes = match r.get("subcubes") {
            Some(v) if !v.is_null() => Some(
                v.items()
                    .iter()
                    .map(|x| {
                        Ok((
                            set(x.get("from").unwrap_or(&Json::Null))?,
                            set(x.get("to").unwrap_or(&Json::Null))?,
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
            _ => None,
        };
        let violation = |k: &str| {
            int(r
                .get("violations")
                .and_then(|v| v.get(k))
                .and_then(|v| v.get("count"))) as u64
        };
        doc.cube = Some(Cube {
            nodes,
            root_dead: r.get("root_dead").and_then(Json::as_bool).unwrap_or(false),
            size_r: int(r.get("size")) as u64,
            minimal: sets(r.get("minimal"))?,
            maximal: sets(r.get("maximal"))?,
            holes: sets(r.get("holes"))?,
            minimal_holes: sets(r.get("minimal_holes"))?,
            repairs,
            components: r
                .get("components")
                .map(Json::items)
                .unwrap_or(&[])
                .iter()
                .map(|c| int(c.get("size")) as u64)
                .collect(),
            subcubes,
            nogoods: sets(r.get("nogoods"))?,
            discharging: sets(r.get("discharging"))?,
            local_max: sets(r.get("local_max"))?,
            local_min: sets(r.get("local_min"))?,
            dead_not_upward_closed: violation("dead_not_upward_closed"),
            discharged_not_upward_closed: violation("discharged_not_upward_closed"),
        });
        Ok(doc)
    }

    /// `{a, b}` or `∅`, in candidate order.
    pub fn name(&self, mask: u64) -> String {
        if mask == 0 {
            return "∅".to_string();
        }
        format!("{{{}}}", self.facts(mask).join(", "))
    }

    pub fn facts(&self, mask: u64) -> Vec<&str> {
        (0..self.n)
            .filter(|i| mask >> i & 1 == 1)
            .map(|i| self.candidates[i].as_str())
            .collect()
    }
}

impl RegionDoc {
    /// The mask of `facts`, when every one is a candidate.
    pub fn mask(&self, facts: &[String]) -> Option<u64> {
        facts.iter().try_fold(0u64, |acc, f| {
            self.candidates
                .iter()
                .position(|c| c == f)
                .map(|b| acc | 1 << b)
        })
    }
}

/// Read `DIR/region.json`, an `ein oracle --dump` directory.
pub fn read_dir(dir: &Path) -> Result<RegionDoc, String> {
    RegionDoc::from_json(&load(dir, "region.json")?)
}

fn load(dir: &Path, name: &str) -> Result<Json, String> {
    let p = dir.join(name);
    let text = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
    parse(&text).map_err(|e| format!("{}: {e}", p.display()))
}

// ── the run ─────────────────────────────────────────────────────────

/// One state a region run reported — `index.json`'s `region.states[i]` — or
/// one node of an empty R's owing frontier (`region.frontier[i]`).
#[derive(Clone, Debug, Default)]
pub struct RunState {
    /// stdout's numbering, 1-based: `models/NN.ein`, or `owing/NN.ein` for
    /// the frontier.
    pub number: usize,
    /// The candidates its fixpoint holds; for a frontier node, its
    /// commitment.
    pub closure: Vec<String>,
    /// The commitment it was read from.
    pub from: Vec<String>,
    /// `minimal`, `maximal`: the lines the run printed it on.
    pub lines: Vec<String>,
    /// stdout listed it. A state the run reached past stdout's cut is in the
    /// directory only.
    pub listed: bool,
    /// A frontier node's debt.
    pub owes: u32,
    /// Its node: `closure`'s mask, when every fact is a coordinate.
    pub at: Option<u64>,
    /// `from`'s mask.
    pub from_at: Option<u64>,
}

/// One node a region run visited — `nodes.json`'s `nodes[i]`.
#[derive(Clone, Debug, Default)]
pub struct Visit {
    pub commitment: Vec<String>,
    /// `dead`, `owing` or `in-r`: the run's reading of the node.
    pub kind: String,
    pub closure: Vec<String>,
    pub at: Option<u64>,
    pub closure_at: Option<u64>,
}

/// A region run, read back from its two dumps — M1e2 S1e2.1a.7. Either may be
/// absent: `--answer` alone draws what it reported, `--visited` alone what it
/// visited, neither the cube alone.
#[derive(Clone, Debug, Default)]
pub struct RunDoc {
    /// `--answer DIR`'s `index.json` was read (`ein-answer/2`).
    pub answered: bool,
    /// `--visited DIR`'s `nodes.json` was read (`ein-lattice/2`).
    pub visited: bool,
    /// `all`, `minimal` or `maximal`: the part of R the run asked.
    pub which: String,
    /// `describe`, `find` or `single`.
    pub ask: String,
    /// `-n N`.
    pub n: Option<i64>,
    pub exhausted: bool,
    /// Why the run stopped: `asked`, `budget`, `limit`, `deadline`.
    pub stopped: Option<String>,
    /// `dead` or `owing`, when the run established R empty.
    pub empty: Option<String>,
    pub route: String,
    pub states: Vec<RunState>,
    pub frontier: Vec<RunState>,
    pub visits: Vec<Visit>,
}

impl RunDoc {
    /// No run: an `ein oracle` dump drawn alone.
    pub fn none() -> RunDoc {
        RunDoc::default()
    }

    /// Read a run's `--dump-answer` and `--dump-lattice` directories against
    /// the cube they are drawn on. A run over another universe is an error:
    /// its commitments would not be the cube's nodes.
    pub fn read(
        answer: Option<&Path>,
        visited: Option<&Path>,
        doc: &RegionDoc,
    ) -> Result<RunDoc, String> {
        let mut run = RunDoc::default();
        let at = |facts: &[String]| doc.mask(facts);
        if let Some(dir) = answer {
            let j = load(dir, "index.json")?;
            let p = dir.join("index.json");
            if j.get("schema").and_then(Json::as_str) != Some("ein-answer/2") {
                return Err(format!(
                    "{}: not an ein-answer/2 document (`ein solve --dump-answer`)",
                    p.display()
                ));
            }
            let asked = j.get("asked");
            let s = |v: Option<&Json>, k: &str| {
                v.and_then(|v| v.get(k))
                    .and_then(Json::as_str)
                    .unwrap_or("")
                    .to_string()
            };
            let r = j.get("region");
            let cands = r
                .and_then(|r| r.get("universe"))
                .and_then(|u| u.get("candidates"))
                .and_then(Json::as_i64);
            if cands.is_some_and(|c| c as usize != doc.n) {
                return Err(format!(
                    "{}: the run's universe has {} candidates and the cube's {} — \
                     not a run of the program the cube is of",
                    p.display(),
                    cands.unwrap_or_default(),
                    doc.n
                ));
            }
            run.answered = true;
            run.which = s(asked, "which");
            run.ask = s(asked, "ask");
            run.n = asked.and_then(|a| a.get("n")).and_then(Json::as_i64);
            run.exhausted = r
                .and_then(|r| r.get("exhausted"))
                .and_then(Json::as_bool)
                .unwrap_or(false);
            run.stopped = r
                .and_then(|r| r.get("stopped"))
                .and_then(|x| x.get("why"))
                .and_then(Json::as_str)
                .map(str::to_string);
            run.empty = r
                .and_then(|r| r.get("empty"))
                .and_then(Json::as_str)
                .map(str::to_string);
            run.route = s(r, "route");
            let list = |k: &str| r.and_then(|r| r.get(k)).map(Json::items).unwrap_or(&[]);
            run.states = list("states")
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let closure = strings(x.get("closure"));
                    let from = strings(x.get("from"));
                    RunState {
                        number: i + 1,
                        at: at(&closure),
                        from_at: at(&from),
                        closure,
                        from,
                        lines: strings(x.get("lines")),
                        listed: x.get("listed").and_then(Json::as_bool).unwrap_or(true),
                        owes: 0,
                    }
                })
                .collect();
            run.frontier = list("frontier")
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let set = strings(x.get("set"));
                    RunState {
                        number: i + 1,
                        at: at(&set),
                        from_at: at(&set),
                        closure: set.clone(),
                        from: set,
                        lines: Vec::new(),
                        listed: true,
                        owes: int(x.get("owes")) as u32,
                    }
                })
                .collect();
        }
        if let Some(dir) = visited {
            let j = load(dir, "nodes.json")?;
            let p = dir.join("nodes.json");
            if j.get("schema").and_then(Json::as_str) != Some("ein-lattice/2") {
                return Err(format!(
                    "{}: not an ein-lattice/2 document (`ein solve --dump-lattice`)",
                    p.display()
                ));
            }
            if strings(j.get("candidates")) != doc.candidates {
                return Err(format!(
                    "{}: the run's candidates are not the cube's — not a run of the \
                     program the cube is of",
                    p.display()
                ));
            }
            run.visited = true;
            if run.route.is_empty() {
                run.route = j
                    .get("route")
                    .and_then(Json::as_str)
                    .unwrap_or("")
                    .to_string();
            }
            run.visits = j
                .get("nodes")
                .map(Json::items)
                .unwrap_or(&[])
                .iter()
                .map(|x| {
                    let commitment = strings(x.get("commitment"));
                    let closure = strings(x.get("closure"));
                    Visit {
                        at: at(&commitment),
                        closure_at: at(&closure),
                        commitment,
                        kind: x
                            .get("kind")
                            .and_then(Json::as_str)
                            .unwrap_or("")
                            .to_string(),
                        closure,
                    }
                })
                .collect();
        }
        Ok(run)
    }

    /// A run was given at all.
    pub fn present(&self) -> bool {
        self.answered || self.visited
    }

    /// The part of R the run asked, as the read-out names it.
    pub fn part(&self) -> &'static str {
        match self.which.as_str() {
            "minimal" => "the minimal line",
            "maximal" => "the maximal line",
            _ => "R",
        }
    }

    /// What the run asked, in one phrase: `all of R`, `the maximal line, -n 2`.
    pub fn asked(&self) -> String {
        let which = match self.which.as_str() {
            "minimal" => "the minimal line",
            "maximal" => "the maximal line",
            _ => "all of R",
        };
        match (self.ask.as_str(), self.n) {
            ("find", Some(n)) => format!("{which}, -n {n}"),
            ("single", _) => format!("{which}, --single"),
            _ => which.to_string(),
        }
    }

    /// The drawing's one line about the run.
    pub fn line(&self) -> String {
        if !self.present() {
            return "no run overlaid (an `ein oracle` dump alone)".to_string();
        }
        let mut parts = Vec::new();
        if self.answered {
            let states = if self.states.is_empty() && !self.frontier.is_empty() {
                format!(
                    "R empty, {} owing frontier node{}",
                    self.frontier.len(),
                    if self.frontier.len() == 1 { "" } else { "s" }
                )
            } else if self.states.is_empty() && self.empty.is_some() {
                format!("R empty ({})", self.empty.as_deref().unwrap_or(""))
            } else {
                format!(
                    "{} state{} reported",
                    self.states.len(),
                    if self.states.len() == 1 { "" } else { "s" }
                )
            };
            parts.push(format!(
                "run: {} — {states}, {}",
                self.asked(),
                if self.exhausted {
                    "exhausted".to_string()
                } else {
                    format!("stopped ({})", self.stopped.as_deref().unwrap_or("?"))
                }
            ));
        } else {
            parts.push("run".to_string());
        }
        if self.visited {
            parts.push(format!("{} nodes visited", self.visits.len()));
        }
        parts.join(" · ")
    }
}

// ── the overlay ─────────────────────────────────────────────────────

/// Why the run did not visit a node, when its visits are known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unvisited {
    /// A node the run visited is dead and below it: licence L1, *dead is
    /// upward-closed*, says this one is dead too. The node is that dead one.
    AboveDead(u64),
    /// The run stopped before it: `-n`, `--single`, a budget, a deadline.
    Stopped,
    /// Neither: the question asked did not need it.
    NotNeeded,
}

impl Unvisited {
    pub fn as_str(self) -> &'static str {
        match self {
            Unvisited::AboveDead(_) => "above a dead node",
            Unvisited::Stopped => "the run stopped first",
            Unvisited::NotNeeded => "not needed",
        }
    }
}

/// What the run did with one node.
#[derive(Clone, Debug, Default)]
pub struct Mark {
    /// The state the run reported is this node's: its number, and whether
    /// stdout listed it.
    pub state: Option<(usize, bool)>,
    /// An empty R's owing frontier: its number, and what it owes.
    pub frontier: Option<(usize, u32)>,
    /// States read from this commitment whose saturation lands elsewhere.
    pub read_from: Vec<usize>,
    /// The run visited this node, and read it as this kind.
    pub visited: Option<String>,
    /// Why the run never visited it, when its visits are known.
    pub unvisited: Option<Unvisited>,
    /// Where the run's reading of the node and the cube's disagree.
    pub disagrees: Option<String>,
}

impl Mark {
    /// The short tag a drawing prints under the facts.
    pub fn tag(&self) -> String {
        if let Some((n, listed)) = self.state {
            return format!("state {n}{}", if listed { "" } else { " (dump only)" });
        }
        if let Some((n, owes)) = self.frontier {
            return format!("frontier {n}, owes {owes}");
        }
        if !self.read_from.is_empty() {
            let ns: Vec<String> = self.read_from.iter().map(|n| n.to_string()).collect();
            return format!("from: state {}", ns.join(", "));
        }
        if self.visited.is_some() {
            return "visited".to_string();
        }
        match self.unvisited {
            Some(u) => u.as_str().to_string(),
            None => String::new(),
        }
    }

    /// A state or a frontier node the run reported.
    pub fn reported(&self) -> bool {
        self.state.is_some() || self.frontier.is_some()
    }
}

/// A region run joined to the cube.
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    pub marks: Vec<Mark>,
    /// A run is overlaid.
    pub run: bool,
    /// Its answer is: what it reported is compared with the part asked.
    pub answered: bool,
    /// The part of R the run asked: `R`, `the minimal line`, `the maximal line`.
    pub part: &'static str,
    /// The part of R the run asked, as nodes; empty without an answer.
    pub asked: Vec<u64>,
    /// Nodes of the part asked whose state no reported state is, on a run
    /// that says it is complete.
    pub missing: Vec<u64>,
    /// The same on a run that stopped: a bound, not a defect (C3).
    pub unreached: Vec<u64>,
    /// Reported states whose state is no node of the part asked, with why.
    pub spurious: Vec<(u64, String)>,
    /// `(dead, alive)`: a node the run did not visit that lies above a dead
    /// node it did, and the cube says is alive — an exclusion L1 would make,
    /// refuted by the cube. Each dead node with its smallest such superset.
    pub unsound: Vec<(u64, u64)>,
    /// Unvisited nodes above a dead node the run visited.
    pub above_dead: usize,
    /// Nodes where the run's reading and the cube's disagree.
    pub disagree: Vec<u64>,
    /// Reported states with a fact that is not a coordinate.
    pub outside: Vec<String>,
}

impl Overlay {
    pub fn of(r: &RegionDoc, run: &RunDoc) -> Overlay {
        let Some(cube) = r.cube.as_ref() else {
            return Overlay {
                run: run.present(),
                ..Overlay::default()
            };
        };
        let size = 1usize << r.n;
        let mut marks = vec![Mark::default(); size];
        if !run.present() {
            return Overlay {
                marks,
                ..Overlay::default()
            };
        }
        let class = |m: u64| cube.nodes[m as usize].class;
        let mut outside = Vec::new();

        // What the run reported, placed on its state's node.
        let mut held: FxHashSet<u64> = FxHashSet::default();
        for s in &run.states {
            let Some(c) = s.at else {
                outside.push(format!("state {}: {{{}}}", s.number, s.closure.join(", ")));
                continue;
            };
            marks[c as usize].state = Some((s.number, s.listed));
            held.insert(class(c));
            if let Some(f) = s.from_at
                && f != c
            {
                marks[f as usize].read_from.push(s.number);
            }
        }
        for s in &run.frontier {
            match s.at {
                Some(m) => marks[m as usize].frontier = Some((s.number, s.owes)),
                None => outside.push(format!(
                    "frontier {}: {{{}}}",
                    s.number,
                    s.closure.join(", ")
                )),
            }
        }

        // What the run visited, each node's reading against the cube's.
        let mut dead_visited = vec![false; size];
        for v in &run.visits {
            let Some(m) = v.at else {
                continue;
            };
            let node = &cube.nodes[m as usize];
            let want = class_word(node.status);
            let mark = &mut marks[m as usize];
            mark.visited = Some(v.kind.clone());
            if v.kind == "dead" {
                dead_visited[m as usize] = true;
            }
            if v.kind != want {
                mark.disagrees = Some(format!(
                    "the run read it {}; the cube says {}",
                    v.kind,
                    status_word(node.status)
                ));
            } else if node.status & DEAD == 0
                && let Some(cl) = v.closure_at
                && cl != node.closure
            {
                mark.disagrees = Some(format!(
                    "the run's closure is {}; the cube's {}",
                    r.name(cl),
                    r.name(node.closure)
                ));
            }
        }

        // The part asked, and the comparison with it, by state.
        let mut asked: Vec<u64> = match run.which.as_str() {
            "minimal" => cube.minimal.clone(),
            "maximal" => cube.maximal.clone(),
            _ => (0..size as u64)
                .filter(|&m| cube.nodes[m as usize].status == 0)
                .collect(),
        };
        if !run.answered {
            asked.clear();
        }
        let part: FxHashSet<u64> = asked.iter().map(|&m| class(m)).collect();
        let (mut missing, mut unreached) = (Vec::new(), Vec::new());
        for &m in &asked {
            if !held.contains(&class(m)) {
                if run.exhausted {
                    missing.push(m);
                } else {
                    unreached.push(m);
                }
            }
        }
        let on_line = |line: &[u64], c: u64| line.iter().any(|&x| class(x) == c);
        let mut spurious = Vec::new();
        for s in &run.states {
            let Some(c) = s.at else {
                continue;
            };
            let node = &cube.nodes[c as usize];
            if !part.contains(&class(c)) {
                let why = if node.status != 0 {
                    format!("the cube says {}", status_word(node.status))
                } else {
                    format!("in R, not on {}", run.part())
                };
                spurious.push((c, why));
            } else {
                // C1: printed as minimal or maximal only when it is on that
                // line of the oracle's R; and C2: a line the run certified —
                // exhausted, and asked for or described with R — names every
                // state on it. A line mode prints only the line it asked.
                let lines = |l: &str| s.lines.iter().any(|x| x == l);
                let (min, max) = (
                    on_line(&cube.minimal, class(c)),
                    on_line(&cube.maximal, class(c)),
                );
                let knows = |l: &str| run.exhausted && (run.which == "all" || run.which == l);
                let wrong = |l: &str, on: bool| lines(l) && !on || knows(l) && on && !lines(l);
                if wrong("minimal", min) || wrong("maximal", max) {
                    let said: Vec<&str> = s.lines.iter().map(String::as_str).collect();
                    let cube_says: Vec<&str> = [("minimal", min), ("maximal", max)]
                        .iter()
                        .filter(|x| x.1)
                        .map(|x| x.0)
                        .collect();
                    marks[c as usize].disagrees = Some(format!(
                        "printed on {}; the cube's lines: {}",
                        if said.is_empty() {
                            "no line".to_string()
                        } else {
                            said.join(", ")
                        },
                        if cube_says.is_empty() {
                            "neither".to_string()
                        } else {
                            cube_says.join(", ")
                        }
                    ));
                }
            }
            if let Some(f) = s.from_at
                && class(f) != class(c)
                && marks[f as usize].disagrees.is_none()
            {
                marks[f as usize].disagrees = Some(format!(
                    "state {} was read from it; the cube says it is the state of {}",
                    s.number,
                    r.name(cube.nodes[f as usize].closure)
                ));
            }
        }
        for s in &run.frontier {
            let Some(m) = s.at else {
                continue;
            };
            let st = cube.nodes[m as usize].status;
            if st != OWING && marks[m as usize].disagrees.is_none() {
                marks[m as usize].disagrees = Some(format!(
                    "on the run's owing frontier; the cube says {}",
                    status_word(st)
                ));
            }
        }
        spurious.sort_by_key(|(m, _)| (m.count_ones(), *m));
        spurious.dedup_by_key(|(m, _)| *m);

        // Why each node the run did not visit was not, and every exclusion
        // licence L1 would make, checked against the cube: the smallest dead
        // node the run visited below each unvisited one.
        let mut unsound = Vec::new();
        let mut above_dead = 0;
        if run.visited {
            let mut below: Vec<Option<u64>> = vec![None; size];
            for m in 0..size {
                if dead_visited[m] {
                    below[m] = Some(m as u64);
                    continue;
                }
                let mut best: Option<u64> = None;
                for b in 0..r.n {
                    if m >> b & 1 == 1
                        && let Some(d) = below[m ^ 1 << b]
                        && best.is_none_or(|x| (d.count_ones(), d) < (x.count_ones(), x))
                    {
                        best = Some(d);
                    }
                }
                below[m] = best;
            }
            let mut order: Vec<u64> = (0..size as u64).collect();
            order.sort_by_key(|m| (m.count_ones(), *m));
            let mut seen: FxHashSet<u64> = FxHashSet::default();
            for m in order {
                let mark = &mut marks[m as usize];
                if mark.visited.is_some() {
                    continue;
                }
                mark.unvisited = Some(match below[m as usize] {
                    Some(d) => {
                        above_dead += 1;
                        if cube.nodes[m as usize].status & DEAD == 0 && seen.insert(d) {
                            unsound.push((d, m));
                        }
                        Unvisited::AboveDead(d)
                    }
                    None if !run.exhausted => Unvisited::Stopped,
                    None => Unvisited::NotNeeded,
                });
            }
        }
        let disagree = (0..size as u64)
            .filter(|&m| marks[m as usize].disagrees.is_some())
            .collect();

        Overlay {
            marks,
            run: true,
            answered: run.answered,
            part: run.part(),
            asked,
            missing,
            unreached,
            spurious,
            unsound,
            above_dead,
            disagree,
            outside,
        }
    }

    pub fn is_spurious(&self, m: u64) -> Option<&str> {
        self.spurious
            .iter()
            .find(|x| x.0 == m)
            .map(|x| x.1.as_str())
    }

    pub fn is_unsound(&self, m: u64) -> Option<u64> {
        self.unsound.iter().find(|x| x.0 == m).map(|x| x.1)
    }
}

/// A status as the run's `kind` spells it: `dead`, `owing`, `in-r`.
fn class_word(s: u8) -> &'static str {
    if s & DEAD != 0 {
        "dead"
    } else if s & OWING != 0 {
        "owing"
    } else {
        "in-r"
    }
}

/// `R`, `owing`, `dead`, `dead+owing`.
pub fn status_word(s: u8) -> &'static str {
    match (s & DEAD != 0, s & OWING != 0) {
        (false, false) => "R",
        (false, true) => "owing",
        (true, false) => "dead",
        (true, true) => "dead+owing",
    }
}
