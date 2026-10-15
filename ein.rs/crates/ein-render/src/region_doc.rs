//! A `--dump-region` directory read back, and the walk joined to the cube —
//! M1e2 [S1e2.1a.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.2_the_lattice_drawn.md).
//!
//! `ein render region DIR` draws from the files a run wrote, never from a
//! second solve, so the picture is of the run whose answer it shows. The two
//! files are joined here, in Rust, because what the overlay reports —
//! *missing*, *spurious*, an *unsound* clause — is a comparison, and the review
//! bundle does not compute.
//!
//! ### The join
//!
//! The cube (`region.json`) is over the config-free root candidates; the walk
//! (`walk.json`) placed each entering on the cube node it stood on — its
//! commitment plus every forced positive promoted before it — and that
//! placement is what is read here. Two nodes are **the same state** when the
//! oracle gave them one `class` (one fixpoint), and every comparison with a
//! line is by state, not by mask: a commitment whose saturation derives a
//! candidate stands, after saturation, on a different node of the cube
//! ([`NodeDoc::closure`]), and recording it records that node's state.

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
#[derive(Clone, Debug, Default)]
pub struct RegionDoc {
    pub source: String,
    pub n: usize,
    pub limit: i64,
    pub candidates: Vec<String>,
    pub monotone: bool,
    /// Each site, rendered `rule · kind · relation`.
    pub sites: Vec<String>,
    /// `Some(note)` when the oracle refused the cube.
    pub refused: Option<String>,
    pub cube: Option<Cube>,
}

/// One commitment of the walk.
#[derive(Clone, Debug, Default)]
pub struct WSet {
    pub facts: Vec<String>,
    /// Its mask, when every fact is a coordinate.
    pub mask: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct WEntering {
    pub set: WSet,
    pub layer: u32,
    pub outcome: String,
    /// The cube node it stood on.
    pub at: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct WRecorded {
    pub set: WSet,
    /// `model` or `open`.
    pub as_: String,
    pub owes: i64,
    pub at: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct WDead {
    pub set: WSet,
    pub layer: u32,
    pub outcome: String,
    pub clause: Vec<String>,
    pub core: Vec<String>,
}

/// `walk.json`.
#[derive(Clone, Debug, Default)]
pub struct WalkDoc {
    pub verdict: String,
    pub k: i64,
    pub exhausted: bool,
    pub max_set_size: u32,
    pub solutions: Option<u64>,
    pub proposed: Vec<String>,
    pub alive0: Vec<String>,
    pub promotions: Vec<(String, usize)>,
    pub enterings: Vec<WEntering>,
    pub recorded: Vec<WRecorded>,
    pub dead: Vec<WDead>,
    pub nogoods: Vec<WSet>,
    pub root_writes: Vec<String>,
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
                        format!("{} · {} · {}", f("rule"), f("kind"), f("relation"))
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

fn wset(v: Option<&Json>) -> WSet {
    WSet {
        facts: strings(v.and_then(|v| v.get("facts"))),
        mask: mask_of(v.and_then(|v| v.get("mask"))),
    }
}

impl WalkDoc {
    pub fn from_json(j: &Json) -> Result<WalkDoc, String> {
        if j.get("schema").and_then(Json::as_str) != Some("ein-walk/1") {
            return Err("walk.json: not an ein-walk/1 document".into());
        }
        let v = j.get("verdict");
        let b = j.get("bounds");
        let u = j.get("universe");
        let list = |k: &str| j.get(k).map(Json::items).unwrap_or(&[]);
        Ok(WalkDoc {
            verdict: v
                .and_then(|v| v.get("type"))
                .and_then(Json::as_str)
                .unwrap_or("")
                .to_string(),
            k: int(v.and_then(|v| v.get("k"))),
            exhausted: v
                .and_then(|v| v.get("exhausted"))
                .and_then(Json::as_bool)
                .unwrap_or(false),
            max_set_size: int(b.and_then(|b| b.get("max_set_size"))) as u32,
            solutions: b
                .and_then(|b| b.get("solutions"))
                .and_then(Json::as_i64)
                .map(|n| n as u64),
            proposed: strings(u.and_then(|u| u.get("proposed"))),
            alive0: strings(u.and_then(|u| u.get("alive0"))),
            promotions: list("promotions")
                .iter()
                .map(|p| {
                    (
                        p.get("fact")
                            .and_then(Json::as_str)
                            .unwrap_or("")
                            .to_string(),
                        int(p.get("after_enterings")) as usize,
                    )
                })
                .collect(),
            enterings: list("enterings")
                .iter()
                .map(|e| WEntering {
                    set: wset(e.get("commitment")),
                    layer: int(e.get("layer")) as u32,
                    outcome: e
                        .get("outcome")
                        .and_then(Json::as_str)
                        .unwrap_or("")
                        .to_string(),
                    at: mask_of(e.get("state_mask")),
                })
                .collect(),
            recorded: list("recorded")
                .iter()
                .map(|e| WRecorded {
                    set: wset(e.get("commitment")),
                    as_: e.get("as").and_then(Json::as_str).unwrap_or("").to_string(),
                    owes: int(e.get("owes")),
                    at: mask_of(e.get("state_mask")),
                })
                .collect(),
            dead: list("dead")
                .iter()
                .map(|e| WDead {
                    set: wset(e.get("commitment")),
                    layer: int(e.get("layer")) as u32,
                    outcome: e
                        .get("outcome")
                        .and_then(Json::as_str)
                        .unwrap_or("")
                        .to_string(),
                    clause: strings(e.get("learned_clause")),
                    core: strings(e.get("core")),
                })
                .collect(),
            nogoods: list("learned_nogoods")
                .iter()
                .map(|g| wset(Some(g)))
                .collect(),
            root_writes: strings(j.get("root_writes")),
        })
    }
}

/// Read `DIR/region.json` and `DIR/walk.json`.
pub fn read_dir(dir: &Path) -> Result<(RegionDoc, WalkDoc), String> {
    let load = |name: &str| -> Result<Json, String> {
        let p = dir.join(name);
        let text = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        parse(&text).map_err(|e| format!("{}: {e}", p.display()))
    };
    Ok((
        RegionDoc::from_json(&load("region.json")?)?,
        WalkDoc::from_json(&load("walk.json")?)?,
    ))
}

// ── the overlay ─────────────────────────────────────────────────────

/// Why the search never stood on a node.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Skip {
    /// A coordinate this run's generator never proposed.
    Outside,
    /// A coordinate dropped before the search: the lookahead, or a root
    /// negative it wrote.
    PreKilled,
    /// It lacks a fact the forced-positive cascade promoted into root, so the
    /// search's root was already past it.
    BelowRoot,
    /// A coordinate whose singleton the search refuted, and dropped from
    /// `alive`.
    ElementRefuted,
    /// A learned clause is a subset of it.
    Clause,
    /// Its commitment is larger than `--max-set-size`.
    PastCap,
    /// None of those: the search stopped first, or its join did not propose it.
    NotGenerated,
}

impl Skip {
    pub fn as_str(self) -> &'static str {
        match self {
            Skip::Outside => "outside the run's universe",
            Skip::PreKilled => "pre-killed",
            Skip::BelowRoot => "below the search's root",
            Skip::ElementRefuted => "element refuted",
            Skip::Clause => "pruned by a clause",
            Skip::PastCap => "past -m",
            Skip::NotGenerated => "never generated",
        }
    }
}

/// What the run did with one node.
#[derive(Clone, Debug, Default)]
pub struct Mark {
    /// Each entering that stood here: its outcome.
    pub entered: Vec<String>,
    /// `model` or `open`, when a recorded state is this node.
    pub recorded: Option<String>,
    /// Why the search never stood here, when it did not.
    pub skipped: Option<(Skip, String)>,
    /// The search's root (∅), or the root the cascade moved it to.
    pub root: Option<&'static str>,
    /// A node the search stood on whose saturation lands here: the search
    /// held this node's state without entering it.
    pub reached_from: Option<u64>,
    /// Where the search's own reading of the node and the cube's disagree.
    pub disagrees: Option<String>,
}

impl Mark {
    /// The short tag a drawing prints.
    pub fn tag(&self) -> String {
        if let Some(r) = &self.recorded {
            return format!("recorded {r}");
        }
        if let Some(r) = self.root {
            return r.to_string();
        }
        if !self.entered.is_empty() {
            return format!("entered {}", self.entered.join(", "));
        }
        if self.reached_from.is_some() {
            return "reached by saturation".to_string();
        }
        match &self.skipped {
            Some((s, _)) => s.as_str().to_string(),
            None => String::new(),
        }
    }

    /// The search stood on it, or on a node whose saturation lands here.
    pub fn visited(&self) -> bool {
        self.stood() || self.reached_from.is_some()
    }

    /// The search stood on it: root, an entering, or a recorded state.
    pub fn stood(&self) -> bool {
        self.recorded.is_some() || self.root.is_some() || !self.entered.is_empty()
    }
}

/// Something the search excluded with every superset — a learned clause, a
/// refuted singleton, a lookahead kill — checked against the cube.
#[derive(Clone, Debug)]
pub struct Exclusion {
    pub mask: u64,
    /// `clause`, `refuted`, `lookahead`, joined when one set is several.
    pub kinds: Vec<&'static str>,
    /// The smallest superset the cube says is **alive**: the exclusion is
    /// sound exactly when there is none.
    pub alive_superset: Option<u64>,
}

/// The walk joined to the cube.
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    pub marks: Vec<Mark>,
    /// The root the search ended on: every promotion's bit.
    pub search_root: u64,
    /// Recorded models' state classes.
    pub model_classes: FxHashSet<u64>,
    /// `[minimal, maximal]`: line nodes no recorded model is the state of.
    pub missing: [Vec<u64>; 2],
    /// `[minimal, maximal]`: recorded models whose state is on no node of the
    /// line.
    pub spurious: [Vec<u64>; 2],
    pub exclusions: Vec<Exclusion>,
    /// Recorded states with a fact that is not a coordinate.
    pub outside_recorded: Vec<String>,
}

impl Overlay {
    pub fn unsound(&self) -> impl Iterator<Item = &Exclusion> {
        self.exclusions
            .iter()
            .filter(|e| e.alive_superset.is_some())
    }

    pub fn of(r: &RegionDoc, w: &WalkDoc) -> Overlay {
        let Some(cube) = r.cube.as_ref() else {
            return Overlay::default();
        };
        let n = r.n;
        let size = 1usize << n;
        let bit: FxHashMap<&str, usize> = r
            .candidates
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str(), i))
            .collect();
        // determinism-ok: membership only.
        let proposed: FxHashSet<&str> = w.proposed.iter().map(String::as_str).collect();
        // determinism-ok: membership only.
        let alive0: FxHashSet<&str> = w.alive0.iter().map(String::as_str).collect();
        let promoted: FxHashSet<&str> = w.promotions.iter().map(|p| p.0.as_str()).collect();
        let search_root = w
            .promotions
            .iter()
            .filter_map(|p| bit.get(p.0.as_str()))
            .fold(0u64, |a, &b| a | 1 << b);
        let bits_where = |pred: &dyn Fn(&str) -> bool| -> u64 {
            r.candidates
                .iter()
                .enumerate()
                .filter(|(_, c)| pred(c))
                .fold(0, |a, (i, _)| a | 1 << i)
        };
        let outside = bits_where(&|c| !proposed.contains(c));
        let prekilled =
            bits_where(&|c| proposed.contains(c) && !alive0.contains(c) && !promoted.contains(c));
        let refuted_singletons: u64 = w
            .dead
            .iter()
            .filter_map(|d| d.set.mask)
            .filter(|m| m.count_ones() == 1)
            .fold(0, |a, m| a | m);
        let clauses: Vec<u64> = w
            .nogoods
            .iter()
            .filter_map(|g| g.mask)
            .filter(|&g| g != 0)
            .collect();

        let mut marks = vec![Mark::default(); size];
        marks[0].root = Some("root");
        if search_root != 0 {
            marks[search_root as usize].root = Some("root after the cascade");
        }
        for e in &w.enterings {
            if let Some(m) = e.at {
                marks[m as usize].entered.push(e.outcome.clone());
            }
        }
        let mut outside_recorded = Vec::new();
        let mut model_classes = FxHashSet::default();
        for rec in &w.recorded {
            match rec.at {
                Some(m) => {
                    marks[m as usize].recorded = Some(rec.as_.clone());
                    if rec.as_ == "model" {
                        model_classes.insert(cube.nodes[m as usize].class);
                    }
                }
                None => outside_recorded.push(format!("{{{}}}", rec.set.facts.join(", "))),
            }
        }
        let tick = |m: u64| -> String {
            r.facts(m)
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        // Where each node the search stood on saturates to.
        for m in 0..size {
            if !marks[m].stood() {
                continue;
            }
            let cl = cube.nodes[m].closure as usize;
            if cl != m && !marks[cl].stood() && marks[cl].reached_from.is_none() {
                marks[cl].reached_from = Some(m as u64);
            }
        }
        let stopped = w.solutions.is_some_and(|s| w.recorded.len() as u64 >= s);
        for (m, mark) in marks.iter_mut().enumerate() {
            let mu = m as u64;
            let node = &cube.nodes[m];
            if mark.reached_from.is_some() && !mark.stood() {
                continue;
            }
            if mark.visited() {
                let dead = node.status & DEAD != 0;
                if mark.entered.iter().any(|o| o == "alive") && dead {
                    mark.disagrees = Some("entered alive; the cube says dead".into());
                } else if !mark.entered.is_empty()
                    && mark.entered.iter().all(|o| o != "alive")
                    && !dead
                {
                    mark.disagrees = Some("refuted by the search; the cube says alive".into());
                } else if mark.recorded.as_deref() == Some("model") && node.status != 0 {
                    mark.disagrees = Some(format!(
                        "recorded as a model; the cube says {}",
                        status_word(node.status)
                    ));
                }
                continue;
            }
            let commitment = mu & !search_root;
            mark.skipped = Some(if mu & outside != 0 {
                (Skip::Outside, tick(mu & outside))
            } else if mu & prekilled != 0 {
                (Skip::PreKilled, tick(mu & prekilled))
            } else if search_root & !mu != 0 {
                (
                    Skip::BelowRoot,
                    format!("lacks {}", tick(search_root & !mu)),
                )
            } else if mu & refuted_singletons != 0 {
                (Skip::ElementRefuted, tick(mu & refuted_singletons))
            } else if let Some(&g) = clauses.iter().find(|&&g| mu & g == g) {
                (Skip::Clause, r.name(g))
            } else if commitment.count_ones() > w.max_set_size {
                (
                    Skip::PastCap,
                    format!("{} facts, -m {}", commitment.count_ones(), w.max_set_size),
                )
            } else if stopped {
                (
                    Skip::NotGenerated,
                    format!(
                        "the search stopped at -n {}",
                        w.solutions.unwrap_or_default()
                    ),
                )
            } else {
                (Skip::NotGenerated, "no join proposed it".into())
            });
        }

        let classes_of = |line: &[u64]| -> FxHashSet<u64> {
            line.iter().map(|&m| cube.nodes[m as usize].class).collect()
        };
        let models: Vec<u64> = w
            .recorded
            .iter()
            .filter(|x| x.as_ == "model")
            .filter_map(|x| x.at)
            .collect();
        let mut missing: [Vec<u64>; 2] = Default::default();
        let mut spurious: [Vec<u64>; 2] = Default::default();
        for (i, line) in [&cube.minimal, &cube.maximal].into_iter().enumerate() {
            let on = classes_of(line);
            missing[i] = line
                .iter()
                .copied()
                .filter(|&m| !model_classes.contains(&cube.nodes[m as usize].class))
                .collect();
            let mut sp: Vec<u64> = models
                .iter()
                .copied()
                .filter(|&m| !on.contains(&cube.nodes[m as usize].class))
                .collect();
            sp.sort_by_key(|m| (m.count_ones(), *m));
            sp.dedup();
            spurious[i] = sp;
        }

        // Every exclusion, checked: sound exactly when every superset is dead.
        let mut by_mask: Vec<(u64, Vec<&'static str>)> = Vec::new();
        let mut add = |m: u64, kind: &'static str| match by_mask.iter_mut().find(|e| e.0 == m) {
            Some(e) => {
                if !e.1.contains(&kind) {
                    e.1.push(kind)
                }
            }
            None => by_mask.push((m, vec![kind])),
        };
        for &g in &clauses {
            add(g, "clause");
        }
        for b in 0..n {
            if refuted_singletons >> b & 1 == 1 {
                add(1 << b, "refuted");
            }
            if prekilled >> b & 1 == 1 {
                add(1 << b, "lookahead");
            }
        }
        let mut order: Vec<u64> = (0..size as u64).collect();
        order.sort_by_key(|m| (m.count_ones(), *m));
        let exclusions = by_mask
            .into_iter()
            .map(|(mask, kinds)| Exclusion {
                mask,
                kinds,
                alive_superset: order
                    .iter()
                    .copied()
                    .find(|&m| m & mask == mask && cube.nodes[m as usize].status & DEAD == 0),
            })
            .collect();

        Overlay {
            marks,
            search_root,
            model_classes,
            missing,
            spurious,
            exclusions,
            outside_recorded,
        }
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
