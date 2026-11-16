//! The hypothesis space of a program, written out: its candidates and the
//! structure of the set, what `ein render h-space FILE` prints and writes.
//!
//! **Nothing here computes a structure.** Every object is
//! [`ein_infer::structure`]'s, read once by [`ein_infer::structure::whole`],
//! and this module only spells it: one JSON document, `h-space.json`
//! ([`json`], [`DOCUMENT`], schema [`SCHEMA`]), four graphs as DOT
//! ([`graphs`]), and a short read-out ([`summary`]). No group, grid, question or edge is defined
//! here; a name such as a question's ([`question_name`]) or a grid's
//! relations is a label read off the candidates' texts for the reader, and
//! the owner reads none.
//!
//! The five `*_lists` functions are the lists of one object each, every
//! candidate as the program writes it. `ein-infer/examples/pair_layer.rs`
//! writes the same five as a file each, through these functions, so it and
//! the view cannot spell one object two ways.
//!
//! **A file is a function of the program and of what was asked.** It holds
//! no clock and no job count, every list is in the owner's canonical order,
//! and a candidate's place is its place in the universe, which is in
//! rendered-text order. So two runs write the same bytes: in any id space,
//! which `ein-infer/tests/structure_invariance.rs` holds on every corpus
//! program that searches, and at any `--jobs`, which
//! `ein-cli/tests/h_space_view.rs` holds on one program and the owner's
//! own tests of each layer on more.
//!
//! **`H` itself, as Ein facts**, is [`hypotheses_text`], what `ein show
//! hyps FILE` prints: the candidates the document's `candidates` lists, by
//! the function the owner opens every structure over, and no node read.
//!
//! **A structure that is not whole says so.** [`unread`] lists, as
//! sentences, everything a budget or a bound stopped or cut and everything
//! left unread for it; the document carries them under `unread` with
//! `whole: false`, the read-out prints them, and each graph's title names
//! how many there are.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use ein_core::{FactId, Symbol, Terms};
use ein_infer::explore::Stopped;
use ein_infer::structure::{
    self, CLIQUE_STEPS, Coupling, Cover, Cut, Edge, EdgeKind, ExactlyOne, Hypotheses, Mark,
    NodeStop, NotAGrid, Origin, Owing, PairLayer, Question, Theory, Whole, WitnessIs,
};

use crate::dot_util::esc;
use crate::dump::Json;
use crate::read_out::{group, plural};

/// The command's name, written once: [`COMMAND`], [`SCHEMA`] and
/// [`DOCUMENT`] are spelled from it.
macro_rules! command {
    () => {
        "h-space"
    };
}

/// **The view's name under `ein render`**: `ein render h-space FILE`, the
/// hypothesis space. The parser, the dispatch, the read-out's first row, the
/// document's file and its schema all take the name from here.
pub const COMMAND: &str = command!();

/// The schema of the document.
pub const SCHEMA: &str = concat!("ein-", command!(), "/1");

/// The files `--dump DIR` writes: the document, `h-space.json`, then each
/// of [`GRAPHS`] as `<name>.dot` with `-` for `_`.
pub const DOCUMENT: &str = concat!(command!(), ".json");

/// The graphs, by the name `--view` takes: the exclusion graph, the cover,
/// the questions' hypergraph, and the hypergraph of the edges read one
/// application back.
pub const GRAPHS: [&str; 4] = ["exclusion", "cover", "coupling", "one-step"];

/// The file `--dump DIR` writes graph `name` to.
pub fn graph_file(name: &str) -> String {
    match name {
        "one-step" => "coupling_one_step.dot".to_string(),
        other => format!("{other}.dot"),
    }
}

// ── H, listed ─────────────────────────────────────────────────────────

/// **`H` as Ein facts**, what `ein show hyps FILE` prints: one candidate a
/// line, every candidate, in the universe's canonical order, and nothing
/// else. Each line is the fact as the one printer writes it
/// ([`Terms::fact_text`], which writes a state file's facts too), so it is
/// a fact a `.ein` file could hold, spelled as the program's file spells it.
/// Empty where `H` is.
pub fn hypotheses_text(h: &Hypotheses, terms: &Terms) -> String {
    let mut out = String::new();
    for c in &h.candidates {
        out.push_str(&terms.fact_text(c.fact));
        out.push('\n');
    }
    out
}

/// **What `ein show hyps` says of `H` beside it**, one sentence: the rung of
/// the ladder that built it, and how many candidates it holds, or, where it
/// holds none, why, in the words of the read-out's `universe` row for a run
/// that searches nothing.
pub fn hypotheses_note(h: &Hypotheses) -> String {
    let universe = match (h.candidates.len() as u64, h.rung) {
        (0, ein_infer::oblgen::Mode::NoSearch) => {
            "no candidate: the program states :no-search".to_string()
        }
        (0, _) => "no candidate".to_string(),
        (n, _) => plural(n, "candidate", "candidates"),
    };
    format!("H, built by the rung `{}`: {universe}", h.rung.as_str())
}

// ── The JSON's small parts ────────────────────────────────────────────

fn int(n: impl TryInto<i64>) -> Json {
    Json::Int(n.try_into().unwrap_or(i64::MAX))
}

fn text(s: &str) -> Json {
    Json::Str(s.to_string())
}

fn opt_text(s: Option<&str>) -> Json {
    s.map_or(Json::Null, text)
}

fn list<T>(items: impl IntoIterator<Item = T>, f: impl FnMut(T) -> Json) -> Json {
    Json::Array(items.into_iter().map(f).collect())
}

fn texts<'a>(items: impl IntoIterator<Item = &'a str>) -> Json {
    list(items, text)
}

fn ids(v: &[u32]) -> Json {
    list(v, |&k| int(k))
}

/// `obj` with `more` after its own keys.
fn with(obj: Json, more: Vec<(&str, Json)>) -> Json {
    match obj {
        Json::Object(mut pairs) => {
            pairs.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
            Json::Object(pairs)
        }
        other => other,
    }
}

/// Why a reading stopped, where it did.
pub fn stopped(s: &Option<Stopped>) -> Json {
    match s {
        None => Json::Null,
        Some(Stopped::Budget { spent, projected }) => Json::obj(vec![
            ("why", text("budget")),
            ("spent", int(*spent)),
            ("projected", int(*projected)),
        ]),
        Some(Stopped::Limit { n, limit }) => Json::obj(vec![
            ("why", text("limit")),
            ("n", int(*n)),
            ("limit", int(*limit)),
        ]),
        Some(Stopped::Deadline) => Json::obj(vec![("why", text("deadline"))]),
        Some(Stopped::NoSearch) => Json::obj(vec![("why", text("no-search"))]),
    }
}

fn cut_of(c: &Option<Cut>) -> Json {
    match c {
        None => Json::Null,
        Some(c) => Json::obj(vec![
            ("pairs", int(c.pairs)),
            ("candidates", int(c.candidates)),
        ]),
    }
}

/// The relation candidate `fact` is of, as text: a label for the reader.
fn relation_of(terms: &Terms, fact: FactId) -> String {
    terms.sym(structure::subject(terms, fact)).to_string()
}

// ── The lists of each object ──────────────────────────────────────────

/// **A pair layer's lists**: what one candidate and two decide, every
/// candidate as text.
pub fn pairs_lists(l: &PairLayer) -> Json {
    let one = |v: &[u32]| texts(v.iter().map(|&b| l.text(b)));
    let two = |v: &[(u32, u32)]| list(v, |&(a, b)| texts([l.text(a), l.text(b)]));
    Json::obj(vec![
        ("n", int(l.n())),
        ("monotone", Json::Bool(l.monotone)),
        ("domain", opt_text(l.domain())),
        ("root_dead", Json::Bool(l.root_dead)),
        ("complete", Json::Bool(l.complete())),
        ("stopped", stopped(&l.stopped)),
        ("singles_left", int(l.singles_left)),
        (
            "pairs",
            Json::obj(vec![
                ("all", int(l.pairs.all)),
                ("dead_member", int(l.pairs.dead_member)),
                ("implied", int(l.pairs.implied)),
                ("saturated", int(l.pairs.saturated)),
                ("left", int(l.pairs.left)),
            ]),
        ),
        (
            "work",
            Json::obj(vec![
                ("saturations", int(l.work.saturations)),
                ("facts", int(l.work.facts)),
            ]),
        ),
        (
            "candidates",
            texts(l.candidates.iter().map(|c| c.text.as_str())),
        ),
        ("at_root", one(&l.at_root)),
        ("dead_alone", one(&l.dead_alone)),
        ("implies", two(&l.implies)),
        ("equivalent", list(&l.equivalent, |c| one(c))),
        ("excluded", two(&l.excluded)),
        ("interacting", two(&l.interacting)),
        ("against_l1", two(&l.against_l1)),
        (
            "against_l3",
            list(&l.against_l3, |&(a, b, dead)| {
                Json::obj(vec![
                    ("pair", texts([l.text(a), l.text(b)])),
                    ("dead", Json::Bool(dead)),
                ])
            }),
        ),
    ])
}

/// **The theory's share's lists**: every fact the program states with what
/// the small KB does with it, what the theory kills and excludes alone, and
/// each pair root excludes with whether it is the theory's and, where one
/// was read, its support.
pub fn theory_lists(t: &Theory) -> Json {
    let l = &t.root;
    let pair = |&(a, b): &(u32, u32)| texts([l.text(a), l.text(b)]);
    let of_theory = t.of_the_theory();
    let mut clauses: BTreeMap<(&str, bool), usize> = BTreeMap::new();
    let (mut kept, mut dropped) = (0usize, 0usize);
    for s in t.stated.iter().filter(|s| s.origin == Origin::Fact) {
        *clauses.entry((s.clause, s.kept)).or_default() += 1;
        if s.kept {
            kept += 1;
        } else {
            dropped += 1;
        }
    }
    let statement = |k: u32| {
        let s = &t.stated[k as usize];
        match &s.source {
            Some(src) => format!("{} :source {src:?}", s.text),
            None => s.text.clone(),
        }
    };
    let named = |support: &structure::Support, kept: bool| {
        list(
            support
                .stated
                .iter()
                .filter(|&&k| t.stated[k as usize].kept == kept),
            |&k| Json::Str(statement(k)),
        )
    };
    Json::obj(vec![
        (
            "small_kb",
            Json::obj(vec![
                ("kept", int(kept)),
                ("dropped", int(dropped)),
                (
                    "by_clause",
                    list(&clauses, |((c, k), n)| {
                        Json::obj(vec![
                            ("clause", text(c)),
                            ("kept", Json::Bool(*k)),
                            ("facts", int(*n)),
                        ])
                    }),
                ),
                (
                    "error",
                    opt_text(t.small.as_ref().err().map(String::as_str)),
                ),
            ]),
        ),
        (
            "stated",
            list(&t.stated, |s| {
                Json::obj(vec![
                    ("fact", text(&s.text)),
                    ("source", opt_text(s.source.as_deref())),
                    ("origin", text(s.origin.as_str())),
                    ("kept", Json::Bool(s.kept)),
                    ("clause", text(s.clause)),
                    ("read_absent", Json::Bool(s.read_absent)),
                ])
            }),
        ),
        (
            "theory_dead_alone",
            match &t.small {
                Ok(s) => texts(s.dead_alone.iter().map(|&b| l.text(b))),
                Err(_) => Json::Array(Vec::new()),
            },
        ),
        ("theory_excluded", list(t.excluded(), pair)),
        (
            "against_root",
            Json::obj(vec![
                (
                    "singles",
                    texts(t.against_root.singles.iter().map(|&b| l.text(b))),
                ),
                ("pairs", list(&t.against_root.pairs, pair)),
            ]),
        ),
        ("small_beyond_root", int(t.small_beyond_root)),
        (
            "root_excluded",
            list(l.excluded.iter().enumerate(), |(k, p)| {
                let support = t.supports.get(k).map_or(Json::Null, |s| {
                    Json::obj(vec![
                        ("read", Json::Bool(s.read)),
                        (
                            "status",
                            s.status.map_or(Json::Null, |c| Json::Str(c.word())),
                        ),
                        ("dropped", named(s, false)),
                        ("kept", named(s, true)),
                        (
                            "commitments",
                            texts(s.commitments.iter().map(|&b| l.text(b))),
                        ),
                        ("other", texts(s.other.iter().map(String::as_str))),
                        ("rules", texts(s.rules.iter().map(String::as_str))),
                        (
                            "check",
                            s.check.as_ref().map_or(Json::Null, |c| {
                                Json::obj(vec![
                                    ("dead", Json::Bool(c.dead)),
                                    ("root_dead", Json::Bool(c.root_dead)),
                                    ("beyond", int(c.beyond)),
                                    ("unread", opt_text(c.unread.as_deref())),
                                ])
                            }),
                        ),
                    ])
                });
                Json::obj(vec![
                    ("pair", pair(p)),
                    ("of_the_theory", Json::Bool(of_theory[k])),
                    ("support", support),
                ])
            }),
        ),
    ])
}

/// **The cover's lists**: every candidate with its mark and its groups, the
/// groups with their open members, the candidates in no group, the
/// components of the theory's exclusion graph, the theory's pairs alive at
/// root, and the maximal cliques of root's own graph beside.
pub fn cover_lists(c: &Cover) -> Json {
    let n = c.n();
    let one = |v: &[u32]| texts(v.iter().map(|&b| c.text(b)));
    let killed = |b: u32| c.killed_alone.binary_search(&b).is_ok();
    Json::obj(vec![
        ("n", int(n)),
        ("complete", Json::Bool(c.complete())),
        ("monotone", Json::Bool(c.monotone)),
        ("small_monotone", Json::Bool(c.small_monotone)),
        ("domain", opt_text(c.domain())),
        ("stopped", stopped(&c.stopped)),
        ("bound", int(CLIQUE_STEPS)),
        ("steps", int(c.steps)),
        ("cut", cut_of(&c.cut)),
        (
            "candidates",
            list(0..n as u32, |b| {
                Json::obj(vec![
                    ("candidate", text(c.text(b))),
                    ("at_root", text(c.marks[b as usize].as_str())),
                    ("killed_by_the_theory_alone", Json::Bool(killed(b))),
                    ("groups", ids(c.groups_of(b))),
                ])
            }),
        ),
        (
            "groups",
            list(0..c.groups.len() as u32, |g| {
                let open: Vec<u32> = c.open(g).collect();
                Json::obj(vec![("members", one(c.members(g))), ("open", one(&open))])
            }),
        ),
        ("in_no_group", one(&c.in_no_group())),
        ("components", list(&c.components, |x| one(x))),
        (
            "pairs_alive_at_root",
            c.alive_at_root.as_ref().map_or(Json::Null, |v| {
                list(v, |&(a, b)| texts([c.text(a), c.text(b)]))
            }),
        ),
        (
            "root_cliques",
            c.root_cliques.as_ref().map_or(Json::Null, |r| {
                Json::obj(vec![
                    ("steps", int(r.steps)),
                    ("cut", cut_of(&r.cut)),
                    ("cliques", list(&r.cliques, |x| one(x))),
                ])
            }),
        ),
    ])
}

/// What a grid's two families are owed, in words.
fn grid_kind(x: &ExactlyOne, grid: u32) -> &'static str {
    let share = x.share(grid);
    if x.is_bijection(grid) {
        "a bijection: every group owed"
    } else if share.iter().all(|s| s.owed == s.groups) {
        "every group owed, and the families differ in number"
    } else if share.iter().any(|s| s.owed == s.groups) {
        "one family owed whole"
    } else if share.iter().any(|s| s.owed > 0) {
        "owed in part"
    } else {
        "owed nowhere"
    }
}

/// ***Exactly one*'s lists**: every instance root owes with its witnesses
/// and how it stands to the cover, every group with who owes it, the owed
/// groups, the groups nothing owes, the owed sets that are no group, the
/// instances L5 does not license, the grids and the components that are no
/// grid. A grid's `relations` are read off its members' texts, for the
/// reader.
pub fn exactly_one_lists(c: &Cover, x: &ExactlyOne, terms: &Terms) -> Json {
    let one = |v: &[u32]| texts(v.iter().map(|&b| c.text(b)));
    let table = x.owed.table();
    let relations_of = |members: &[u32]| -> Json {
        let set: BTreeSet<String> = members
            .iter()
            .map(|&b| relation_of(terms, c.candidates[b as usize].fact))
            .collect();
        texts(set.iter().map(String::as_str))
    };

    let instances = list(table.iter().zip(&x.owing).enumerate(), |(k, (i, owing))| {
        let witnesses = match &i.witnesses {
            Err(why) => Json::obj(vec![("unknown", text(why))]),
            Ok(ws) => list(ws, |w| {
                Json::obj(vec![
                    ("fact", text(&w.text)),
                    ("is", text(w.is.as_str())),
                    ("held_at_root_by", one(&w.held_at_root)),
                    ("held_in_the_theory_by", one(&w.held_in_theory)),
                ])
            }),
        };
        let mut row = vec![
            ("instance", text(&i.text)),
            ("rule", text(&i.rule)),
            ("standing", text(owing.as_str())),
            ("licensed", Json::Bool(i.alternatives.is_ok())),
            (
                "witnesses_denied_at_root",
                i.witnesses.as_ref().ok().map_or(Json::Null, |ws| {
                    int(ws.iter().filter(|w| w.is == WitnessIs::Denied).count())
                }),
            ),
            ("witnesses", witnesses),
        ];
        match (&i.alternatives, owing) {
            (Ok(alts), _) => {
                row.push(("alternatives", one(alts)));
                match owing {
                    Owing::Group(g) => row.push(("group", int(*g))),
                    Owing::Inside(groups) => row.push(("inside", ids(groups))),
                    Owing::NotExcluding { alive, holds } => {
                        row.push(("pairs_the_theory_does_not_exclude", int(*alive)));
                        row.push(("holds_groups", ids(holds)));
                    }
                    _ => {}
                }
            }
            (Err(why), Owing::Unlicensed { payers, about }) => {
                row.push(("why", text(why)));
                row.push((
                    "pay_by_the_theory_alone",
                    payers.as_ref().map_or(Json::Null, |p| one(p)),
                ));
                row.push(("about_group", about.map_or(Json::Null, int)));
                if let Some((all, by)) = x.brought(k as u32) {
                    row.push(("witnesses_no_candidate_and_not_denied", int(all)));
                    row.push(("of_them_brought_by_a_candidate_at_root", int(by)));
                }
            }
            // An unlicensed instance stands as one, by the owner's reading.
            (Err(why), _) => row.push(("why", text(why))),
        }
        Json::obj(row)
    });

    let instance_texts = |v: &[u32]| texts(v.iter().map(|&k| table[k as usize].text.as_str()));
    let groups = list(0..c.groups.len() as u32, |g| {
        let open: Vec<u32> = c.open(g).collect();
        Json::obj(vec![
            ("group", int(g)),
            ("members", one(c.members(g))),
            ("open", one(&open)),
            ("owed_by", instance_texts(&x.owed_by[g as usize])),
            (
                "an_unlicensed_instance_is_about_it",
                instance_texts(&x.about[g as usize]),
            ),
            (
                "grid",
                x.grids.of_group[g as usize].map_or(Json::Null, |(grid, family)| {
                    Json::obj(vec![("grid", int(grid)), ("family", int(family))])
                }),
            ),
        ])
    });

    let members_of = |component: u32| &c.components[component as usize];
    let grids = list(x.grids.grids.iter().enumerate(), |(k, g)| {
        let share = x.share(k as u32);
        let members = members_of(g.component);
        let family = |f: usize| {
            Json::obj(vec![
                ("groups", ids(&g.families[f])),
                ("owed", int(share[f].owed)),
                ("an_unlicensed_instance_is_about", int(share[f].about)),
            ])
        };
        Json::obj(vec![
            ("grid", int(k)),
            ("component", int(g.component)),
            ("members", int(members.len())),
            (
                "open",
                int(members
                    .iter()
                    .filter(|&&b| c.marks[b as usize] == Mark::AliveAlone)
                    .count()),
            ),
            ("cells", int(g.families[0].len() * g.families[1].len())),
            ("relations", relations_of(members)),
            ("families", Json::Array(vec![family(0), family(1)])),
            ("kind", text(grid_kind(x, k as u32))),
            ("bijection", Json::Bool(x.is_bijection(k as u32))),
        ])
    });
    let not_grids = list(&x.grids.not, |&(component, why)| {
        let members = members_of(component);
        let mut groups: Vec<u32> = members
            .iter()
            .flat_map(|&b| c.groups_of(b).iter().copied())
            .collect();
        groups.sort_unstable();
        groups.dedup();
        Json::obj(vec![
            ("component", int(component)),
            ("members", int(members.len())),
            ("groups", ids(&groups)),
            (
                "owed",
                int(groups.iter().filter(|&&g| x.is_owed(g)).count()),
            ),
            ("relations", relations_of(members)),
            ("why", text(why.as_str())),
            (
                "detail",
                match why {
                    NotAGrid::MoreThanTwo { most } => Json::obj(vec![("most", int(most))]),
                    NotAGrid::InOne { members } => {
                        Json::obj(vec![("members_in_one_group", int(members))])
                    }
                    _ => Json::Null,
                },
            ),
        ])
    });

    Json::obj(vec![
        ("n", int(c.n())),
        ("domain", opt_text(c.domain())),
        ("root_owes", int(x.owed.root_owes)),
        (
            "no_table",
            opt_text(x.owed.instances.as_ref().err().map(|e| e.as_str())),
        ),
        ("singles_read", Json::Bool(x.owed.singles_read)),
        ("theory_read", Json::Bool(x.owed.theory_read)),
        ("instances", instances),
        ("groups", groups),
        ("owed_groups", ids(&x.owed_groups())),
        ("groups_nothing_owes", ids(&x.unowed())),
        ("owed_sets_not_a_group", ids(&x.not_groups())),
        ("unlicensed", ids(&x.unlicensed())),
        ("grids", grids),
        ("components_that_are_no_grid", not_grids),
    ])
}

/// The members of a set of candidates as one pattern, where they differ in
/// one word: `(val D *)`. Otherwise its first member and how many there
/// are.
pub fn pattern(texts: &[&str]) -> String {
    let words: Vec<Vec<&str>> = texts
        .iter()
        .map(|t| t.split_whitespace().collect())
        .collect();
    let Some(first) = words.first() else {
        return "(none)".to_string();
    };
    if texts.len() == 1 {
        return texts[0].to_string();
    }
    if words.iter().all(|w| w.len() == first.len()) {
        let differ: Vec<usize> = (0..first.len())
            .filter(|&k| words.iter().any(|w| w[k] != first[k]))
            .collect();
        if let [k] = differ[..] {
            let mut out: Vec<String> = first.iter().map(|w| w.to_string()).collect();
            let (open, close) = (first[k].starts_with('('), first[k].ends_with(')'));
            out[k] = format!(
                "{}*{}",
                if open { "(" } else { "" },
                if close { ")" } else { "" }
            );
            return out.join(" ");
        }
    }
    format!("{} … ×{}", texts[0], texts.len())
}

/// The candidates question `q` holds: a group's members, or the one
/// candidate in no group.
pub fn question_members(c: &Coupling, cover: &Cover, q: u32) -> Vec<u32> {
    match c.questions[q as usize] {
        Question::Group(g) => cover.members(g).to_vec(),
        Question::Alone(b) => vec![b],
    }
}

/// **A question's name**: the [`pattern`] of its members' texts. A label,
/// and **no identity**: two questions of one program can share one, so
/// every list names a question by its place too.
pub fn question_name(c: &Coupling, cover: &Cover, q: u32) -> String {
    let m = question_members(c, cover, q);
    pattern(&m.iter().map(|&b| c.text(b)).collect::<Vec<_>>())
}

/// A group's name: the [`pattern`] of its members' texts, as a question's.
fn group_name(cover: &Cover, g: u32) -> String {
    let m: Vec<&str> = cover.members(g).iter().map(|&b| cover.text(b)).collect();
    pattern(&m)
}

/// How many of question `q`'s members are alive alone at root.
pub fn question_open(c: &Coupling, cover: &Cover, q: u32) -> usize {
    question_members(c, cover, q)
        .iter()
        .filter(|&&b| cover.marks[b as usize] == Mark::AliveAlone)
        .count()
}

/// **The coupling's lists**: the questions, the ties of the pair layer, the
/// edges of the whole node and those read one application back, both sets
/// of components, and the debts that are edges. An edge and a debt name
/// their questions twice: by name (`questions`) and by place in the
/// `questions` list (`question_ids`).
pub fn coupling_lists(c: &Coupling, cover: &Cover) -> Json {
    let name = |q: u32| question_name(c, cover, q);
    let one = |v: &[u32]| texts(v.iter().map(|&b| c.text(b)));
    let edges_raw = |edges: &[Edge]| {
        list(edges, |e| {
            Json::obj(vec![
                ("questions", list(&e.questions, |&q| Json::Str(name(q)))),
                ("question_ids", ids(&e.questions)),
                ("kind", text(e.kind().as_str())),
                (
                    "pairs",
                    Json::obj(vec![
                        ("excludes", int(e.pairs.excludes)),
                        ("implies", int(e.pairs.implies)),
                        ("interacts", int(e.pairs.interacts)),
                    ]),
                ),
                (
                    "node",
                    Json::obj(vec![
                        ("conflicts", int(e.node.conflicts)),
                        ("derivations", int(e.node.derivations)),
                        ("implications", int(e.node.implications)),
                    ]),
                ),
                (
                    "debts",
                    texts(
                        e.debts
                            .iter()
                            .map(|&d| c.debts[d as usize].instance.as_str()),
                    ),
                ),
                (
                    "stated",
                    list(&e.stated, |&(k, n)| {
                        Json::Array(vec![text(&c.reasons[k as usize]), int(n)])
                    }),
                ),
            ])
        })
    };
    Json::obj(vec![
        ("reading", text(c.reading.as_str())),
        ("complete", Json::Bool(c.complete())),
        ("domain", opt_text(c.domain())),
        (
            "questions",
            list(0..c.questions.len() as u32, |q| {
                Json::obj(vec![
                    (
                        "is",
                        text(match c.questions[q as usize] {
                            Question::Group(_) => "a group",
                            Question::Alone(_) => "a candidate in no group",
                        }),
                    ),
                    ("name", Json::Str(name(q))),
                    ("open", int(question_open(c, cover, q))),
                    (
                        "members",
                        list(question_members(c, cover, q), |b| {
                            Json::Array(vec![
                                text(c.text(b)),
                                text(cover.marks[b as usize].as_str()),
                            ])
                        }),
                    ),
                ])
            }),
        ),
        (
            "ties",
            list(&c.ties, |t| {
                Json::obj(vec![
                    ("pair", texts([c.text(t.pair.0), c.text(t.pair.1)])),
                    ("kind", text(t.kind.as_str())),
                    (
                        "stated",
                        t.stated.as_ref().map_or(Json::Null, |s| {
                            texts(s.iter().map(|&k| c.reasons[k as usize].as_str()))
                        }),
                    ),
                ])
            }),
        ),
        ("edges", edges_raw(&c.edges)),
        (
            "edges_one_step_back",
            c.one_step.as_deref().map_or(Json::Null, edges_raw),
        ),
        ("components", list(&c.components, |k| one(k))),
        (
            "conflict_components",
            list(&c.conflict_components, |k| one(k)),
        ),
        (
            "debts",
            list(&c.debts, |d| {
                Json::obj(vec![
                    ("rule", text(&d.rule)),
                    ("instance", text(&d.instance)),
                    ("licensed", Json::Bool(d.licensed)),
                    ("payers", one(&d.payers)),
                    ("questions", list(&d.questions, |&q| Json::Str(name(q)))),
                    ("question_ids", ids(&d.questions)),
                ])
            }),
        ),
        ("debts_across_conflict_components", ids(&c.debts_across())),
        ("unlicensed_instances", int(c.unlicensed)),
        ("committed", one(&c.committed)),
    ])
}

// ── The graphs ────────────────────────────────────────────────────────

/// One graph as DOT, with what it holds.
#[derive(Clone, Debug)]
pub struct Graph {
    /// One of [`GRAPHS`].
    pub name: &'static str,
    pub dot: String,
    pub nodes: usize,
    /// The lines it draws: a ` -- ` each.
    pub lines: usize,
}

/// **The questions' hypergraph over `edges`**, as DOT: a question a box
/// under its place, `q3`, with its name, an edge of two questions a line,
/// and an edge of more a point joined to each of its questions. An edge of
/// a conflict or a derivation is labelled with how many ties and ways it
/// covers; a debt's alone is dashed and named for its debts, *unlicensed*
/// where L5 licenses none of one. A question in none of `edges` is grey.
///
/// `places` writes each question's place into its label, which is what
/// tells two questions of one name apart; `ein-infer/examples/pair_layer.rs`
/// writes its `coupling.dot` without.
pub fn coupling_dot(
    title: &str,
    c: &Coupling,
    cover: &Cover,
    edges: &[Edge],
    places: bool,
) -> String {
    let mut dot = format!(
        "graph coupling {{\n  label=\"{}\";\n  node [shape=box, fontsize=10];\n",
        esc(title)
    );
    let in_an_edge: BTreeSet<u32> = edges
        .iter()
        .flat_map(|e| e.questions.iter().copied())
        .collect();
    for q in 0..c.questions.len() as u32 {
        let _ = writeln!(
            dot,
            "  q{q} [label=\"{}{}\\n{} open of {}\"{}];",
            if places {
                format!("q{q} ")
            } else {
                String::new()
            },
            esc(&question_name(c, cover, q)),
            question_open(c, cover, q),
            question_members(c, cover, q).len(),
            if in_an_edge.contains(&q) {
                ""
            } else {
                ", color=gray, fontcolor=gray"
            }
        );
    }
    for (k, e) in edges.iter().enumerate() {
        let weight = e.pairs.all() + e.node.all();
        // A debt's edge alone is dashed, and named for its debts.
        let (style, label) = match e.kind() {
            EdgeKind::Debt => {
                let unlicensed = e.debts.iter().any(|&d| !c.debts[d as usize].licensed);
                let label = format!(
                    "owed ×{}{}",
                    e.debts.len(),
                    if unlicensed { ", unlicensed" } else { "" }
                );
                (", style=dashed", label)
            }
            EdgeKind::Both => ("", format!("{weight}, owed ×{}", e.debts.len())),
            EdgeKind::Tie => ("", weight.to_string()),
        };
        if let [a, b] = e.questions[..] {
            let _ = writeln!(
                dot,
                "  q{a} -- q{b} [label=\"{label}\", fontsize=8{style}];"
            );
        } else {
            let _ = writeln!(dot, "  e{k} [shape=point, xlabel=\"{label}\"];");
            let dashed = if style.is_empty() {
                ""
            } else {
                " [style=dashed]"
            };
            for q in &e.questions {
                let _ = writeln!(dot, "  e{k} -- q{q}{dashed};");
            }
        }
    }
    dot += "}\n";
    dot
}

/// The fill of a candidate's box, by the component of the theory's
/// exclusion graph it is in.
const FILLS: [&str; 8] = [
    "#ddf4ff", "#fff8c5", "#dafbe1", "#ffebe9", "#fbefff", "#fff1e5", "#e6f6f8", "#f0f0f0",
];

/// A candidate's node: dashed and grey where it is dead alone at root.
fn candidate_node(id: &str, label: &str, dead: bool, fill: Option<&str>) -> String {
    let mut style = match (dead, fill) {
        (true, Some(_)) => ", style=\"filled,dashed\"".to_string(),
        (true, None) => ", style=dashed".to_string(),
        (false, Some(_)) => ", style=filled".to_string(),
        (false, None) => String::new(),
    };
    if dead {
        style += ", color=\"#8c959f\", fontcolor=\"#6e7781\"";
    }
    if let Some(fill) = fill {
        let _ = write!(style, ", fillcolor=\"{fill}\"");
    }
    format!("  {id} [label=\"{}\"{style}];\n", esc(label))
}

/// **The exclusion graph**, as DOT, in two strokes. A pair the theory
/// excludes alone is a solid line, dotted where a member is dead alone at
/// root, so that root's own layer does not list it. A pair root excludes
/// that the theory does not, the puzzle's, is a dashed line, labelled with
/// the statements its support names where supports were asked: `s7` is
/// place 7 of the document's `theory.stated`. A candidate in no pair of
/// either stroke is not drawn; one dead alone at root is dashed and grey,
/// and a box is filled by its component of the theory's graph.
fn exclusion_dot(title: &str, w: &Whole) -> Graph {
    let t = &w.theory;
    let l = &t.root;
    let dead_alone: BTreeSet<u32> = l.dead_alone.iter().copied().collect();
    let theory: BTreeSet<(u32, u32)> = t.excluded().iter().copied().collect();
    let of_theory = t.of_the_theory();
    // The puzzle's pairs, each with its place in root's list.
    let puzzle: Vec<(usize, (u32, u32))> = l
        .excluded
        .iter()
        .copied()
        .enumerate()
        .filter(|(k, p)| !of_theory[*k] && !theory.contains(p))
        .collect();
    let mut used: BTreeSet<u32> = BTreeSet::new();
    for &(a, b) in theory.iter().chain(puzzle.iter().map(|(_, p)| p)) {
        used.insert(a);
        used.insert(b);
    }
    let mut part: BTreeMap<u32, usize> = BTreeMap::new();
    if let Some(p) = &w.parts {
        for (k, members) in p.cover.components.iter().enumerate() {
            for &b in members {
                part.insert(b, k);
            }
        }
    }
    let mut dot = format!(
        "graph exclusion {{\n  graph [label=\"{}\", labelloc=t, fontsize=11, overlap=scale];\n  \
         node [shape=box, fontsize=9, margin=\"0.06,0.03\", height=0.25];\n",
        esc(title)
    );
    for &b in &used {
        let fill = part.get(&b).map(|k| FILLS[k % FILLS.len()]);
        dot += &candidate_node(&format!("n{b}"), l.text(b), dead_alone.contains(&b), fill);
    }
    for &(a, b) in &theory {
        let stroke = if dead_alone.contains(&a) || dead_alone.contains(&b) {
            "color=\"#0969da\", style=dotted"
        } else {
            "color=\"#0969da\", penwidth=1.3"
        };
        let _ = writeln!(dot, "  n{a} -- n{b} [{stroke}];");
    }
    for &(k, (a, b)) in &puzzle {
        // The statements the small KB drops that the support names, six a
        // line.
        let named: Vec<String> = match t.supports.get(k) {
            Some(s) if s.read => s
                .stated
                .iter()
                .filter(|&&k| !t.stated[k as usize].kept)
                .map(|k| format!("s{k}"))
                .collect(),
            _ => Vec::new(),
        };
        let label = if named.is_empty() {
            String::new()
        } else {
            let lines: Vec<String> = named.chunks(6).map(|c| c.join(" ")).collect();
            format!(", label=\"{}\"", lines.join("\\n"))
        };
        let _ = writeln!(
            dot,
            "  n{a} -- n{b} [color=\"#cf222e\", style=dashed, fontcolor=\"#cf222e\", \
             fontsize=8{label}];"
        );
    }
    dot += "}\n";
    Graph {
        name: "exclusion",
        dot,
        nodes: used.len(),
        lines: theory.len() + puzzle.len(),
    }
}

/// **The cover**, as DOT: a group a box and a candidate an oval joined to
/// each group it is in, the components of the theory's exclusion graph each
/// a cluster under what it is, a grid, a bijection, or why it is no grid.
/// In a grid the groups of one family are drawn before their members and
/// those of the other after, so a member stands between its row and its
/// column. An owed group has a double border and names the rule that owes
/// it; a group an unlicensed instance is about is dashed. A candidate in
/// no group is not drawn.
fn cover_dot(title: &str, cover: &Cover, x: &ExactlyOne) -> Graph {
    let table = x.owed.table();
    let mut dot = format!(
        "graph cover {{\n  graph [label=\"{}\", labelloc=t, fontsize=11, rankdir=LR];\n  \
         node [fontsize=9];\n",
        esc(title)
    );
    // Each component's standing: its grid, or why it is none.
    let mut standing: BTreeMap<u32, String> = BTreeMap::new();
    for (k, g) in x.grids.grids.iter().enumerate() {
        standing.insert(g.component, format!("grid {k}: {}", grid_kind(x, k as u32)));
    }
    for &(component, why) in &x.grids.not {
        standing.insert(component, format!("no grid: {}", why.as_str()));
    }
    let mut component_of_group: BTreeMap<u32, u32> = BTreeMap::new();
    for (k, members) in cover.components.iter().enumerate() {
        for &b in members {
            for &g in cover.groups_of(b) {
                component_of_group.insert(g, k as u32);
            }
        }
    }
    let (mut nodes, mut edges) = (0usize, 0usize);
    for (k, members) in cover.components.iter().enumerate() {
        let k = k as u32;
        let groups: Vec<u32> = (0..cover.groups.len() as u32)
            .filter(|g| component_of_group.get(g) == Some(&k))
            .collect();
        if groups.is_empty() {
            continue;
        }
        let _ = writeln!(
            dot,
            "  subgraph cluster_c{k} {{\n    label=\"component {k}: {}\";\n    fontsize=10;",
            esc(standing.get(&k).map_or("", String::as_str))
        );
        for &g in &groups {
            let open = cover.open(g).count();
            let mut label = format!(
                "g{g} {}\\n{open} open of {}",
                esc(&group_name(cover, g)),
                cover.members(g).len()
            );
            let mut rules: BTreeSet<&str> = BTreeSet::new();
            for &i in &x.owed_by[g as usize] {
                rules.insert(table[i as usize].rule.as_str());
            }
            let style = if !rules.is_empty() {
                let rules: Vec<&str> = rules.into_iter().collect();
                let _ = write!(label, "\\nowed by {}", esc(&rules.join(", ")));
                ", peripheries=2"
            } else if !x.about[g as usize].is_empty() {
                label += "\\nan unlicensed instance is about it";
                ", style=dashed"
            } else {
                ""
            };
            let _ = writeln!(dot, "    g{g} [shape=box, label=\"{label}\"{style}];");
            nodes += 1;
        }
        for &b in members {
            if cover.groups_of(b).is_empty() {
                continue;
            }
            let dead = cover.marks[b as usize] == Mark::DeadAlone;
            dot += "  ";
            dot += &candidate_node(&format!("n{b}"), cover.text(b), dead, None);
            nodes += 1;
            for &g in cover.groups_of(b) {
                // A grid's first family before the member, its second after.
                let after = matches!(x.grids.of_group[g as usize], Some((_, 1)));
                if after {
                    let _ = writeln!(dot, "    n{b} -- g{g};");
                } else {
                    let _ = writeln!(dot, "    g{g} -- n{b};");
                }
                edges += 1;
            }
        }
        dot += "  }\n";
    }
    dot += "}\n";
    Graph {
        name: "cover",
        dot,
        nodes,
        lines: edges,
    }
}

/// A graph with nothing to draw: its title alone.
fn empty_graph(name: &'static str, title: &str) -> Graph {
    Graph {
        name,
        dot: format!(
            "graph {} {{\n  label=\"{}\";\n}}\n",
            name.replace('-', "_"),
            esc(title)
        ),
        nodes: 0,
        lines: 0,
    }
}

/// How many nodes and lines a hypergraph's DOT draws over `edges`.
fn hypergraph_counts(c: &Coupling, edges: &[Edge]) -> (usize, usize) {
    let points = edges.iter().filter(|e| e.arity() != 2).count();
    let lines: usize = edges
        .iter()
        .map(|e| if e.arity() == 2 { 1 } else { e.arity() })
        .sum();
    (c.questions.len() + points, lines)
}

/// **The four graphs of one structure**, in [`GRAPHS`]' order, each titled
/// with the program, what it draws and how many of [`unread`]'s sentences
/// stand over it. Where the small KB has no layer there is no cover and no
/// coupling, and those graphs hold their title alone.
pub fn graphs(w: &Whole, program: &str) -> Vec<Graph> {
    let t = &w.theory;
    let partial = match unread(w).len() {
        0 => String::new(),
        1 => "; NOT WHOLE: 1 thing was stopped, cut or left unread, which the document lists"
            .to_string(),
        k => format!(
            "; NOT WHOLE: {k} things were stopped, cut or left unread, which the document lists"
        ),
    };
    let of_theory = t.of_the_theory().iter().filter(|&&x| x).count();
    let exclusion = exclusion_dot(
        &format!(
            "{program}: the exclusion graph. {} pairs the theory excludes alone (solid; dotted \
             where a member is dead alone at root), {} the puzzle's (dashed){partial}",
            t.excluded().len(),
            t.root.excluded.len() - of_theory
        ),
        w,
    );
    let Some(p) = &w.parts else {
        let none = format!("{program}: the small KB has no layer, so nothing is drawn{partial}");
        return vec![
            exclusion,
            empty_graph("cover", &none),
            empty_graph("coupling", &none),
            empty_graph("one-step", &none),
        ];
    };
    let (cover, x, c) = (&p.cover, &p.exactly_one, &p.coupling);
    let cover_graph = cover_dot(
        &format!(
            "{program}: the cover. {} groups ({} owed, double border), {} grids, {} candidates \
             in no group not drawn{partial}",
            cover.groups.len(),
            x.owed_groups().len(),
            x.grids.grids.len(),
            cover.in_no_group().len()
        ),
        cover,
        x,
    );
    let hyper = |name: &'static str, what: &str, edges: &[Edge]| {
        let (nodes, lines) = hypergraph_counts(c, edges);
        Graph {
            name,
            dot: coupling_dot(
                &format!(
                    "{program}: the questions' hypergraph, {what}. {} questions, {} edges ({} a \
                     debt's alone, dashed){partial}",
                    c.questions.len(),
                    edges.len(),
                    edges.iter().filter(|e| e.kind() == EdgeKind::Debt).count()
                ),
                c,
                cover,
                edges,
                true,
            ),
            nodes,
            lines,
        }
    };
    vec![
        exclusion,
        cover_graph,
        hyper("coupling", "the whole node's edges", &c.edges),
        hyper(
            "one-step",
            "the edges read one application back",
            c.one_step.as_deref().unwrap_or(&[]),
        ),
    ]
}

// ── What is not whole, and what the statements are about ─────────────

fn stopped_words(s: &Stopped) -> String {
    match s {
        Stopped::Budget { spent, projected } => format!(
            "at the budget, {} facts spent and the next step projecting {}",
            group(*spent),
            group(*projected)
        ),
        Stopped::Limit { n, limit } => {
            format!("at the cube's limit, {n} candidates past {limit}")
        }
        Stopped::Deadline => "at the deadline".to_string(),
        Stopped::NoSearch => "before any node, by asking".to_string(),
    }
}

/// What a stopped layer left, as one sentence.
fn layer_left(what: &str, l: &PairLayer) -> Option<String> {
    let s = l.stopped.as_ref()?;
    Some(format!(
        "{what} stopped {}: {} singles and {} pairs were not read, and its lists hold what was",
        stopped_words(s),
        group(l.singles_left),
        group(l.pairs.left)
    ))
}

fn cut_words(what: &str, c: &Cut) -> String {
    format!(
        "the enumeration of {what} stopped at its bound of {} calls: {} excluded pairs lie inside \
         none listed, and {} candidates of such a pair are in none listed",
        group(CLIQUE_STEPS),
        group(c.pairs),
        group(c.candidates)
    )
}

/// **Everything a budget or a bound stopped or cut, and everything left
/// unread for it**, a sentence each, in the order the objects are read.
/// Empty where the structure is whole. A support that was not asked is not
/// here: the document's `asked` says what was.
pub fn unread(w: &Whole) -> Vec<String> {
    let t = &w.theory;
    let mut out = Vec::new();
    out.extend(layer_left("root's pair layer", &t.root));
    match &t.small {
        Err(e) => out.push(format!(
            "the small KB has no layer, so no theory's share, cover, owed group, grid or \
             coupling was read: {e}"
        )),
        Ok(small) => out.extend(layer_left("the small KB's pair layer", small)),
    }
    if !t.supports.is_empty() {
        let not_read = t.supports.iter().filter(|s| !s.read).count();
        if not_read > 0 {
            out.push(format!(
                "{} of the {} supports were not read: the search's closure passed the budget's \
                 count of facts",
                group(not_read as u64),
                group(t.supports.len() as u64)
            ));
        }
        let cut = t
            .supports
            .iter()
            .filter(|s| s.read && s.status.is_some_and(|c| c.qualifier().is_some()))
            .count();
        if cut > 0 {
            // The owner's words for a cut core: a real frontier, and a
            // smaller one may exist.
            out.push(format!(
                "{} of the {} supports were cut at the search's budget: each is a real \
                 frontier of its death and a smaller one may exist, so it may name a statement \
                 the death does not need",
                group(cut as u64),
                group(t.supports.len() as u64)
            ));
        }
    }
    let Some(p) = &w.parts else {
        return out;
    };
    if let Some(c) = &p.cover.cut {
        out.push(cut_words("the groups", c));
    }
    if let Some(c) = p.cover.root_cliques.as_ref().and_then(|r| r.cut.as_ref()) {
        out.push(cut_words("the cliques of root's own exclusion graph", c));
    }
    // Which candidates' own nodes hold each witness is read off a layer's
    // singles, which the layer's own reading left in the cache: it is unread
    // where the layer stopped before its last single, and there alone. The
    // layer's own sentence, above, says it stopped; these say what follows.
    let owed = &p.exactly_one.owed;
    if owed.instances.is_ok() {
        if !owed.singles_read {
            out.push(
                "which candidates' own nodes hold each witness of an instance root owes was \
                 not read at root: root's layer stopped before its last single, and the \
                 holders are read off the singles. No witness is listed as held at root, and \
                 an instance L5 does not license has no payers read, so the debts listed are \
                 the licensed ones alone and the components may be finer than R's"
                    .to_string(),
            );
        }
        if !owed.theory_read {
            out.push(
                "which candidates' own nodes hold each witness of an instance root owes was \
                 not read in the small KB: its layer stopped before its last single, and the \
                 holders are read off the singles. No witness is listed as held by the \
                 theory, and no instance L5 does not license is said to be about a group"
                    .to_string(),
            );
        }
    }
    let c = &p.coupling;
    if !c.pairs_read {
        out.push(
            "root's pairs were not all read: the ties of the pair layer are those of the pairs \
             read"
                .to_string(),
        );
    }
    match c.node_stopped {
        // No node to read, which is no cut: [`says`] has them.
        None | Some(NodeStop::RootDead | NodeStop::Empty) => {}
        Some(stop) => out.push(format!(
            "the whole node was not read, so no edge of a conflict or a derivation past the \
             pairs' is listed: {}",
            stop.as_str()
        )),
    }
    if c.cut.ways > 0 {
        out.push(format!(
            "{} applications had more than {} ways and were each read as one way that needs \
             every question any of them names",
            group(c.cut.ways),
            structure::WAYS
        ));
    }
    if c.cut.descriptions > 0 {
        out.push(format!(
            "{} ways had more than {} descriptions, of which the first were kept",
            group(c.cut.descriptions),
            structure::DESCRIPTIONS
        ));
    }
    if c.cut.unsettled {
        out.push(format!(
            "the labels were still moving after {} rounds and are taken as they stood",
            structure::ROUNDS
        ));
    }
    out
}

/// **What is true of the program that makes its structure small or
/// empty**, a sentence each: no candidate, a dead root, no candidate alive
/// alone, no table of alternatives. None is a cut.
pub fn says(w: &Whole) -> Vec<String> {
    let t = &w.theory;
    let mut out = Vec::new();
    if t.root.n() == 0 {
        out.push(
            "H is empty: the program proposes no candidate, so it does not search and there is \
             no structure to read"
                .to_string(),
        );
        return out;
    }
    if t.root.root_dead {
        out.push(
            "root's own fixpoint derives (false): every candidate is dead alone, no pair is \
             read, and R is empty"
                .to_string(),
        );
    }
    let Some(p) = &w.parts else {
        return out;
    };
    if p.coupling.node_stopped == Some(NodeStop::Empty) && !t.root.root_dead {
        out.push(
            "no candidate is alive alone at root, so there is no whole node to read and no \
             component"
                .to_string(),
        );
    }
    if let Err(why) = &p.exactly_one.owed.instances {
        out.push(format!(
            "no table of what root owes is read, so no group is owed: {}",
            why.as_str()
        ));
    }
    out
}

/// The structure is whole: nothing in [`unread`].
pub fn whole(w: &Whole) -> bool {
    unread(w).is_empty()
}

// ── The document ──────────────────────────────────────────────────────

/// What was asked, for the document's header.
#[derive(Clone, Copy, Debug)]
pub struct Inputs<'a> {
    /// The program, as the command was given it.
    pub program: &'a str,
    /// `--region-budget`, each layer's.
    pub budget: u64,
    /// `--supports`.
    pub supports: bool,
}

fn work(saturations: u64, facts: u64, facts_key: &str) -> Json {
    Json::obj(vec![
        ("saturations", int(saturations)),
        (facts_key, int(facts)),
    ])
}

/// What each object cost, in the read-out's units.
fn cost(w: &Whole) -> Json {
    let t = &w.theory;
    let small = t.small.as_ref().ok();
    let node = w.parts.as_ref().and_then(|p| p.coupling.node);
    let total = t.root.work.saturations
        + small.map_or(0, |s| s.work.saturations)
        + t.support_work.supports
        + node.map_or(0, |n| n.saturations);
    Json::obj(vec![
        (
            "root_layer",
            work(
                t.root.work.saturations,
                t.root.work.facts,
                "facts_beyond_root",
            ),
        ),
        (
            "small_kb_layer",
            small.map_or(Json::Null, |s| {
                work(
                    s.work.saturations,
                    s.work.facts,
                    "facts_beyond_its_own_root",
                )
            }),
        ),
        (
            "supports",
            Json::obj(vec![("saturations", int(t.support_work.supports))]),
        ),
        (
            "cover",
            w.parts.as_ref().map_or(Json::Null, |p| {
                Json::obj(vec![
                    ("saturations", int(0)),
                    ("clique_calls", int(p.cover.steps)),
                    (
                        "root_clique_calls",
                        p.cover
                            .root_cliques
                            .as_ref()
                            .map_or(Json::Null, |r| int(r.steps)),
                    ),
                    ("bound", int(CLIQUE_STEPS)),
                ])
            }),
        ),
        (
            "exactly_one",
            w.parts
                .as_ref()
                .map_or(Json::Null, |_| Json::obj(vec![("saturations", int(0))])),
        ),
        (
            "whole_node",
            node.map_or(Json::Null, |n| {
                Json::obj(vec![
                    ("saturations", int(n.saturations)),
                    ("facts_beyond_root", int(n.facts)),
                    ("applications", int(n.applications)),
                    ("steps", int(n.steps)),
                    ("contradictions", int(n.contradictions)),
                    ("rounds", int(n.rounds)),
                    ("merged", int(n.merged)),
                ])
            }),
        ),
        ("saturations", int(total)),
    ])
}

/// **`H` listed**: every candidate with what each object says of it.
fn candidates(w: &Whole) -> Json {
    let t = &w.theory;
    let l = &t.root;
    let n = l.n();
    let mut implies: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut implied_by: Vec<Vec<u32>> = vec![Vec::new(); n];
    for &(a, b) in &l.implies {
        implies[a as usize].push(b);
        implied_by[b as usize].push(a);
    }
    let mut class: Vec<Option<&Vec<u32>>> = vec![None; n];
    for c in &l.equivalent {
        for &b in c {
            class[b as usize] = Some(c);
        }
    }
    // The pairs each candidate is in, by stroke.
    let (mut theory, mut puzzle) = (vec![0u32; n], vec![0u32; n]);
    for &(a, b) in t.excluded() {
        theory[a as usize] += 1;
        theory[b as usize] += 1;
    }
    for (&(a, b), of_theory) in l.excluded.iter().zip(t.of_the_theory()) {
        if !of_theory {
            puzzle[a as usize] += 1;
            puzzle[b as usize] += 1;
        }
    }
    let one = |v: &[u32]| texts(v.iter().map(|&b| l.text(b)));
    let place = |parts: &[Vec<u32>], b: u32| {
        (0..parts.len())
            .find(|&k| parts[k].binary_search(&b).is_ok())
            .map_or(Json::Null, int)
    };
    list(0..n as u32, |b| {
        let i = b as usize;
        let mut row = vec![("candidate", text(l.text(b)))];
        if let Some(p) = &w.parts {
            let (cover, x, c) = (&p.cover, &p.exactly_one, &p.coupling);
            row.push(("at_root", text(cover.marks[i].as_str())));
            row.push((
                "killed_by_the_theory_alone",
                Json::Bool(cover.killed_alone.binary_search(&b).is_ok()),
            ));
            row.push(("groups", ids(cover.groups_of(b))));
            row.push((
                "in_an_owed_group",
                Json::Bool(cover.groups_of(b).iter().any(|&g| x.is_owed(g))),
            ));
            row.push(("questions", ids(&c.question_of[i])));
            row.push(("component", place(&c.components, b)));
            row.push(("conflict_component", place(&c.conflict_components, b)));
        }
        row.push(("implies", one(&implies[i])));
        row.push(("implied_by", one(&implied_by[i])));
        row.push((
            "equivalent_to",
            class[i].map_or(Json::Array(Vec::new()), |c| {
                texts(c.iter().filter(|&&o| o != b).map(|&o| l.text(o)))
            }),
        ));
        row.push(("pairs_the_theory_excludes", int(theory[i])));
        row.push(("pairs_the_puzzle_excludes", int(puzzle[i])));
        Json::obj(row)
    })
}

/// **What the program declares, beside**: the mechanisms as the engine
/// reads them at root's fixpoint, and for each relation a candidate is of,
/// in the universe's order, the facts the program states that the small KB
/// keeps and that name it, a statement of the relation whole, with the
/// grids its candidates are in. Read off names for the reader; nothing of
/// the structure is read from it.
fn declared(w: &Whole, terms: &Terms) -> Json {
    let t = &w.theory;
    let names = |set: &rustc_hash::FxHashSet<Symbol>| -> Vec<String> {
        // determinism-ok: collected into a sorted list of texts.
        let sorted: BTreeSet<String> = set.iter().map(|&r| terms.sym(r).to_string()).collect();
        sorted.into_iter().collect()
    };
    let strs = |v: &[String]| texts(v.iter().map(String::as_str));
    let sets = w.declared.as_ref().ok().map(|m| {
        (
            names(&m.proposed()),
            names(&m.owable()),
            names(&m.closed),
            m.listed.as_ref().map(&names),
            names(&m.excluded),
        )
    });
    let mechanisms = match (&w.declared, &sets) {
        (Ok(m), Some((proposed, owable, closed, listed, excluded))) => Json::obj(vec![
            ("hrules", int(m.hrules.len())),
            ("active_hrule_plans", int(m.proposals.len())),
            ("obligation_rules", int(m.obligation_rules.len())),
            ("obligation_plans", int(m.obligations.len())),
            ("proposed", strs(proposed)),
            ("owable", strs(owable)),
            ("closed", strs(closed)),
            ("listed", listed.as_ref().map_or(Json::Null, |l| strs(l))),
            ("excluded", strs(excluded)),
            ("no_search", Json::Bool(m.no_search)),
        ]),
        (Err(e), _) => Json::obj(vec![("error", text(e))]),
        (Ok(_), None) => Json::Null,
    };

    // The candidates' relations, in the order of their first candidates.
    let mut relations: Vec<(Symbol, Vec<u32>)> = Vec::new();
    for (b, c) in t.root.candidates.iter().enumerate() {
        let rel = structure::subject(terms, c.fact);
        match relations.iter_mut().find(|(r, _)| *r == rel) {
            Some((_, of)) => of.push(b as u32),
            None => relations.push((rel, vec![b as u32])),
        }
    }
    let rows = list(&relations, |(rel, of)| {
        let name = terms.sym(*rel).to_string();
        let stated: Vec<&str> = t
            .stated
            .iter()
            .filter(|s| s.kept && s.origin == Origin::Fact)
            .filter(|s| {
                let mut named = Vec::new();
                structure::names_of(terms, s.fact, &mut named);
                named.contains(rel)
            })
            .map(|s| s.text.as_str())
            .collect();
        let mut row = vec![
            ("relation", text(&name)),
            ("candidates", int(of.len())),
            ("stated_of_it", texts(stated)),
        ];
        if let Some((proposed, owable, closed, listed, excluded)) = &sets {
            let has = |v: &[String]| Json::Bool(v.contains(&name));
            row.push(("an_active_hrule_proposes_it", has(proposed)));
            row.push(("an_obligation_can_owe_it", has(owable)));
            row.push(("closed", has(closed)));
            row.push(("listed", listed.as_ref().map_or(Json::Null, |l| has(l))));
            row.push(("excluded", has(excluded)));
        }
        if let Some(p) = &w.parts {
            let (cover, x) = (&p.cover, &p.exactly_one);
            let grids: BTreeSet<u32> = of
                .iter()
                .flat_map(|&b| cover.groups_of(b).iter().copied())
                .filter_map(|g| x.grids.of_group[g as usize].map(|(grid, _)| grid))
                .collect();
            let grids: Vec<u32> = grids.into_iter().collect();
            let bijections: Vec<u32> = grids
                .iter()
                .copied()
                .filter(|&k| x.is_bijection(k))
                .collect();
            row.push((
                "candidates_in_no_group",
                int(of
                    .iter()
                    .filter(|&&b| cover.groups_of(b).is_empty())
                    .count()),
            ));
            row.push(("grids", ids(&grids)));
            row.push(("bijections", ids(&bijections)));
        }
        Json::obj(row)
    });
    Json::obj(vec![("mechanisms", mechanisms), ("relations", rows)])
}

/// **The document, `h-space.json`** ([`DOCUMENT`]): the whole structure of
/// one program.
///
/// `graphs` are [`graphs`]' of the same reading: the document says what
/// each holds, so that a reader knows a graph's size before laying it out.
pub fn json(w: &Whole, terms: &Terms, inputs: &Inputs<'_>, graphs: &[Graph]) -> Json {
    let t = &w.theory;
    let unread = unread(w);
    let says = says(w);
    let parts = w.parts.as_ref();
    let coupling = parts.map_or(Json::Null, |p| {
        let c = &p.coupling;
        with(
            coupling_lists(c, &p.cover),
            vec![
                ("pairs_read", Json::Bool(c.pairs_read)),
                ("payers_read", Json::Bool(c.payers_read)),
                ("node_stopped", opt_text(c.node_stopped.map(|s| s.as_str()))),
                (
                    "cut",
                    Json::obj(vec![
                        ("ways", int(c.cut.ways)),
                        ("descriptions", int(c.cut.descriptions)),
                        ("unsettled", Json::Bool(c.cut.unsettled)),
                    ]),
                ),
            ],
        )
    });
    Json::obj(vec![
        ("schema", text(SCHEMA)),
        ("program", text(inputs.program)),
        (
            "asked",
            Json::obj(vec![
                ("region_budget", int(inputs.budget)),
                ("supports", Json::Bool(inputs.supports)),
            ]),
        ),
        ("n", int(t.root.n())),
        ("rung", text(t.rung)),
        ("monotone", Json::Bool(t.root.monotone)),
        ("root_dead", Json::Bool(t.root.root_dead)),
        ("whole", Json::Bool(unread.is_empty())),
        ("unread", texts(unread.iter().map(String::as_str))),
        ("says", texts(says.iter().map(String::as_str))),
        (
            "domain",
            Json::obj(vec![
                ("pair_layer", opt_text(t.root.domain())),
                ("cover", opt_text(parts.and_then(|p| p.cover.domain()))),
                (
                    "coupling",
                    opt_text(parts.and_then(|p| p.coupling.domain())),
                ),
            ]),
        ),
        ("cost", cost(w)),
        (
            "graphs",
            list(graphs, |g| {
                Json::obj(vec![
                    ("graph", text(g.name)),
                    ("file", Json::Str(graph_file(g.name))),
                    ("nodes", int(g.nodes)),
                    ("lines", int(g.lines)),
                ])
            }),
        ),
        ("candidates", candidates(w)),
        (
            "pairs",
            with(
                pairs_lists(&t.root),
                vec![("licences", texts(t.root.licences.iter().map(|x| x.label())))],
            ),
        ),
        (
            "theory",
            with(
                theory_lists(t),
                vec![
                    (
                        "rules_naming_objects",
                        texts(t.rules_naming_objects.iter().map(String::as_str)),
                    ),
                    (
                        "small_kb_layer",
                        t.small.as_ref().map_or(Json::Null, |s| {
                            with(
                                pairs_lists(s),
                                vec![("licences", texts(s.licences.iter().map(|x| x.label())))],
                            )
                        }),
                    ),
                ],
            ),
        ),
        ("cover", parts.map_or(Json::Null, |p| cover_lists(&p.cover))),
        (
            "exactly_one",
            parts.map_or(Json::Null, |p| {
                exactly_one_lists(&p.cover, &p.exactly_one, terms)
            }),
        ),
        ("coupling", coupling),
        ("declared", declared(w, terms)),
    ])
}

/// [`json`] as the text of [`DOCUMENT`]: indented by two, in the
/// document's own key order, a newline at its end.
pub fn document(w: &Whole, terms: &Terms, inputs: &Inputs<'_>, graphs: &[Graph]) -> String {
    crate::dump::json::dumps_indent(&json(w, terms, inputs, graphs)) + "\n"
}

// ── The read-out ──────────────────────────────────────────────────────

/// **The read-out**: what `ein render h-space FILE` prints where no
/// `--view` asks for a graph or the document. One row an object, each a
/// count the document lists in full, then what the structure cost, what
/// each graph holds, and every sentence of [`says`] and of [`unread`].
pub fn summary(w: &Whole, inputs: &Inputs<'_>, graphs: &[Graph]) -> String {
    let t = &w.theory;
    let l = &t.root;
    let g = |n: usize| group(n as u64);
    let p = |n: usize, one: &str, many: &str| plural(n as u64, one, many);
    let mut out = String::new();
    let mut row = |label: &str, what: String| {
        let _ = writeln!(out, "{label:<13}{what}");
    };
    row(COMMAND, inputs.program.to_string());
    row(
        "H",
        format!(
            "{} · {} dead alone · {} at root · built by the rung `{}`",
            p(l.n(), "candidate", "candidates"),
            g(l.dead_alone.len()),
            g(l.at_root.len()),
            t.rung
        ),
    );
    row(
        "pairs",
        format!(
            "{} · {} saturated · {} excluded at root · {} interacting · {} · {}",
            group(l.pairs.all),
            group(l.pairs.saturated),
            g(l.excluded.len()),
            g(l.interacting.len()),
            p(l.implies.len(), "implication", "implications"),
            p(
                l.equivalent.len(),
                "equivalence class",
                "equivalence classes"
            )
        ),
    );
    let of_theory = t.of_the_theory().iter().filter(|&&x| x).count();
    let facts = t.stated.iter().filter(|s| s.origin == Origin::Fact);
    let (stated, dropped) = facts.fold((0usize, 0usize), |(n, d), s| {
        (n + 1, d + usize::from(!s.kept))
    });
    match &t.small {
        Ok(small) => row(
            "theory",
            format!(
                "{} excluded by the theory alone · of root's {}, {} the theory's and {} the \
                 puzzle's · the small KB drops {} of {} and kills {} alone",
                p(small.excluded.len(), "pair", "pairs"),
                g(l.excluded.len()),
                g(of_theory),
                g(l.excluded.len() - of_theory),
                g(dropped),
                p(stated, "stated fact", "stated facts"),
                p(small.dead_alone.len(), "candidate", "candidates")
            ),
        ),
        Err(e) => row("theory", format!("the small KB has no layer: {e}")),
    }
    if inputs.supports {
        row(
            "supports",
            format!(
                "{} read, of the {} root excludes",
                g(t.supports.iter().filter(|s| s.read).count()),
                p(l.excluded.len(), "pair", "pairs")
            ),
        );
    } else {
        row(
            "supports",
            "not asked: --supports names the statements each pair of the puzzle rests on"
                .to_string(),
        );
    }
    if let Some(parts) = &w.parts {
        let (cover, x, c) = (&parts.cover, &parts.exactly_one, &parts.coupling);
        let unowed = x.unowed();
        row(
            "cover",
            format!(
                "{} · {} in no group · {} of the theory's graph",
                p(cover.groups.len(), "group", "groups"),
                p(cover.in_no_group().len(), "candidate", "candidates"),
                p(cover.components.len(), "component", "components")
            ),
        );
        row(
            "exactly one",
            format!(
                "{} · {} nothing owes ({} an unlicensed instance is about) · {} that are no \
                 group · {} L5 does not license, of the {} root owes",
                p(x.owed_groups().len(), "owed group", "owed groups"),
                p(unowed.len(), "group", "groups"),
                g(unowed
                    .iter()
                    .filter(|&&k| !x.about[k as usize].is_empty())
                    .count()),
                p(x.not_groups().len(), "owed set", "owed sets"),
                p(x.unlicensed().len(), "instance", "instances"),
                group(u64::from(x.owed.root_owes))
            ),
        );
        row(
            "grids",
            format!(
                "{} · {} · {} that are no grid",
                p(x.grids.grids.len(), "grid", "grids"),
                p(
                    (0..x.grids.grids.len() as u32)
                        .filter(|&k| x.is_bijection(k))
                        .count(),
                    "bijection",
                    "bijections"
                ),
                p(x.grids.not.len(), "component", "components")
            ),
        );
        row(
            "coupling",
            format!(
                "{} · {} · {} ({} of three questions or more, {} a debt's alone) · {} one \
                 application back · {} that are edges ({} unlicensed)",
                p(c.questions.len(), "question", "questions"),
                p(c.ties.len(), "tie", "ties"),
                p(c.edges.len(), "edge", "edges"),
                g(c.edges.iter().filter(|e| e.arity() >= 3).count()),
                g(c.edges
                    .iter()
                    .filter(|e| e.kind() == EdgeKind::Debt)
                    .count()),
                c.one_step
                    .as_ref()
                    .map_or("no edges read".to_string(), |e| p(e.len(), "edge", "edges")),
                p(c.debts.len(), "debt", "debts"),
                g(c.debts.iter().filter(|d| !d.licensed).count())
            ),
        );
        row(
            "components",
            format!(
                "{} over every edge · {} over the conflicts' and the derivations' alone · {} \
                 across those",
                g(c.components.len()),
                g(c.conflict_components.len()),
                p(c.debts_across().len(), "debt", "debts")
            ),
        );
    }
    let small = t.small.as_ref().ok();
    let node = w.parts.as_ref().and_then(|p| p.coupling.node);
    let sat = |n: u64| plural(n, "saturation", "saturations");
    row(
        "work",
        format!(
            "root's layer {} · {} facts beyond root; the small KB's {}; the whole node {}; \
             supports {}",
            sat(l.work.saturations),
            group(l.work.facts),
            small.map_or("not read".to_string(), |s| format!(
                "{} · {} facts beyond its own root",
                sat(s.work.saturations),
                group(s.work.facts)
            )),
            node.map_or("not read".to_string(), |n| format!(
                "{} · {} facts beyond root",
                sat(n.saturations),
                group(n.facts)
            )),
            sat(t.support_work.supports)
        ),
    );
    for graph in graphs {
        row(
            "graph",
            format!(
                "{} · {} · {}",
                graph_file(graph.name),
                p(graph.nodes, "node", "nodes"),
                p(graph.lines, "line", "lines")
            ),
        );
    }
    for d in [
        l.domain(),
        w.parts.as_ref().and_then(|p| p.cover.domain()),
        w.parts.as_ref().and_then(|p| p.coupling.domain()),
    ]
    .into_iter()
    .flatten()
    {
        row("domain", d.to_string());
    }
    for s in says(w) {
        row("says", s);
    }
    let unread = unread(w);
    row(
        "whole",
        if unread.is_empty() {
            format!(
                "yes: nothing was stopped or cut at --region-budget {}",
                inputs.budget
            )
        } else {
            format!(
                "NO: {} stopped, cut or left unread at --region-budget {}",
                p(unread.len(), "thing was", "things were"),
                inputs.budget
            )
        },
    );
    for s in unread {
        row("unread", s);
    }
    out
}
