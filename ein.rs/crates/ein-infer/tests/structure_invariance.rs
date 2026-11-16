//! **The structure of the hypothesis set is the program's**: the same under
//! a permutation of the interner's ids, and under every import form.
//!
//! The structure is what [`ein_infer::structure`] reads of `H`: root's pair
//! layer, the theory's share of it (the small KB's layer over the same
//! candidates, and which statements the small KB keeps), the cover, the
//! groups with their two maps, the marks, the components and the maximal
//! cliques of root's own exclusion graph, and *exactly one*: what root owes
//! with each instance's witnesses, how each instance stands to the cover,
//! which groups are owed, and the grids; and the coupling: the questions,
//! the ties of the pair layer, the edges with what each covers, the debts
//! it is the edge of and the statements it names, the edges one application
//! back, the components over every edge and over the conflicts' alone, and
//! the debts, licensed or not; and **the coupling read from the whole node
//! alone** (`structure::node_coupling`), which is what the obligation tree
//! reads to order its branches: the questions root owes, its edges, the
//! edges one application back, the pairs and the components. Each is a
//! statement about nodes of
//! the program's lattice, so two things that are not the program must not
//! move it. **And what `ein render structure` writes of it is held with
//! it**: the document, `structure.json`, and the four graphs' DOT, spelled by
//! `ein_render::structure` from the same reading, by the one call the view
//! makes ([`ein_infer::structure::whole`]):
//!
//! 1. **the id order.** A program is read twice: once as loaded, and once
//!    into a `Terms` whose names and facts are the first run's own, interned
//!    again in a shuffled order (`id_permutation`, the determinism sweeps'
//!    own instrument). The universe is in rendered-text order in both, so
//!    every list is compared **as it is**, place for place, the work and the
//!    enumeration's steps among it, and the document and the graphs byte for
//!    byte;
//! 2. **the import form.** A program that imports is read as written, with
//!    every selective import whole, and with every one `:as` an alias
//!    (`import_forms`, the answer's own instrument). A candidate's text
//!    changes with the form, a short name or `impK.name` or the full name,
//!    and with it the universe's order, so every list is compared as a
//!    **set of texts with the module prefixes and aliases taken off**. The
//!    enumeration's steps follow the order and are not compared. Few corpus
//!    programs propose facts of an imported relation beside facts of their
//!    own, so the corpus's forms respell candidates and seldom reorder a
//!    universe; a program written here does, and is held the same way. The
//!    document names a group, a question and an instance by its place, which
//!    follows the universe's order, so it is compared where a form leaves
//!    that order as it was: every string with its prefixes taken off, and
//!    every list of strings as a set.
//!
//! Over every corpus file and register witness that **searches**, whose
//! describe saturates more than root (`corpus_ops`' `searches`, the same
//! function): where there is a structure to move. The corpus's `slow`
//! entries are read under `EIN_CORPUS_SLOW=1`, as the other corpus sweeps
//! read them, but for the instances named here, which every run reads.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test structure_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test structure_invariance
//! ```

#[allow(dead_code)]
#[path = "../../ein-render/tests/id_permutation/mod.rs"]
mod id_permutation;
#[allow(dead_code)]
#[path = "../../ein-cli/tests/import_forms/mod.rs"]
mod import_forms;
#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::structure::{
    self, Cliques, Coupling, Cover, ExactlyOne, NodeCoupling, Owing, PairLayer, Question, Theory,
    WholeOptions, WitnessIs,
};
use ein_ir::{Ast, load, parse};
use ein_render::structure as shown;

fn rel_of(p: &Path) -> String {
    p.strip_prefix(repo_root())
        .unwrap_or(p)
        .display()
        .to_string()
}

/// The instances every run reads, `slow` or not: the declared control, its
/// one-relation twin, and the negative control for pairs.
const INSTANCES: [&str; 3] = [
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/lattice/02_genuine_3set_death.ein",
];

/// `EIN_CORPUS_SLOW=1`: the corpus's `slow` entries too.
fn slow_too() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
}

/// Every corpus file and register witness, once; the `slow` entries that are
/// not [`INSTANCES`] where `slow_too` only.
fn files(slow_too: bool) -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !INSTANCES.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    files
}

/// The name every reading's document and graphs carry: a program is read
/// from its own path and from a scratch directory, and the path is no part
/// of its structure.
const PROGRAM: &str = "program.ein";

/// One reading of a program's structure, and the terms it left.
struct Read {
    theory: Theory,
    cover: Cover,
    exact: ExactlyOne,
    coupling: Coupling,
    /// The coupling read from the whole node alone, of the program loaded
    /// again into a copy of the same terms.
    node: Result<NodeCoupling, String>,
    /// `structure.json`, as `ein render structure` writes it of this
    /// reading.
    document: String,
    /// Its four graphs, as DOT.
    graphs: Vec<String>,
    terms: Terms,
}

/// The structure of the program `text`, named `name`, its modules found
/// from `base`, loaded into `terms`. `None` where it does not load;
/// `Err` where the engine refuses it, by *forbid* or otherwise.
fn read(text: &str, name: &Path, base: &Path, mut terms: Terms) -> Option<Result<Read, String>> {
    // The node alone reads a program nothing else has used: the same text
    // into a copy of the same terms, so under a permutation its ids are the
    // permuted ones too.
    let node = {
        let mut terms = terms.clone();
        let mut ast = Ast::new();
        let forms = parse(&mut ast, text, name.to_str()).ok()?;
        let mut kb = load(&mut ast, &mut terms, &forms, Some(base)).ok()?;
        structure::node_coupling(&mut kb, &mut terms, &ast, &structure::Options::default())
            .map_err(|e| e.to_string())
    };
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, name.to_str()).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, Some(base)).ok()?;
    let asked = WholeOptions::default();
    let read = structure::whole(&mut kb, &mut terms, &ast, &asked);
    Some(match read {
        Err(e) => Err(e.to_string()),
        Ok(w) => {
            let inputs = shown::Inputs {
                program: PROGRAM,
                budget: asked.layer.budget,
                supports: asked.supports,
            };
            let graphs = shown::graphs(&w, PROGRAM);
            let document = shown::document(&w, &terms, &inputs, &graphs);
            let graphs = graphs.into_iter().map(|g| g.dot).collect();
            match w.parts {
                Some(p) => Ok(Read {
                    theory: w.theory,
                    cover: p.cover,
                    exact: p.exactly_one,
                    coupling: p.coupling,
                    node,
                    document,
                    graphs,
                    terms,
                }),
                None => Err(format!("the small KB has no layer: {:?}", w.theory.small)),
            }
        }
    })
}

/// Whether the program at `path` searches, loaded again.
fn searches(path: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
        return false;
    };
    match load(&mut ast, &mut terms, &forms, path.parent()) {
        Ok(mut kb) => searching::searches(&ast, &mut terms, &mut kb),
        Err(_) => false,
    }
}

// ── What a reading says ───────────────────────────────────────────────

/// One layer, place for place.
fn layer_as_it_is(l: &PairLayer) -> String {
    format!(
        "{:?}",
        (
            (l.monotone, l.root_dead, l.singles_left, &l.stopped),
            (&l.at_root, &l.dead_alone, &l.implies, &l.equivalent),
            (&l.excluded, &l.interacting, &l.against_l1, &l.against_l3),
            (l.pairs, &l.licences, l.work),
        )
    )
}

fn cliques_as_they_are(c: &Option<Cliques>) -> String {
    format!("{:?}", c.as_ref().map(|c| (&c.cliques, c.steps, c.cut)))
}

/// **The whole structure, place for place**, each part under its name: the
/// candidates' texts, root's layer, the theory's share and the cover.
fn as_it_is(r: &Read) -> Vec<(&'static str, String)> {
    let (t, c) = (&r.theory, &r.cover);
    let texts: Vec<&str> = t.root.candidates.iter().map(|c| c.text.as_str()).collect();
    let stated: Vec<String> = t
        .stated
        .iter()
        .map(|s| {
            format!(
                "{:?}",
                (
                    &s.text,
                    &s.source,
                    s.origin.as_str(),
                    s.kept,
                    s.clause,
                    s.read_absent
                )
            )
        })
        .collect();
    vec![
        ("the candidates", format!("{texts:?}")),
        ("root's pair layer", layer_as_it_is(&t.root)),
        (
            "the small KB's pair layer",
            match &t.small {
                Ok(s) => layer_as_it_is(s),
                Err(e) => e.clone(),
            },
        ),
        (
            "what the program states, and what is kept",
            format!("{stated:?}"),
        ),
        (
            "the theory's share beside root",
            format!(
                "{:?}",
                (
                    t.rung,
                    &t.rules_naming_objects,
                    t.small_beyond_root,
                    &t.against_root.singles,
                    &t.against_root.pairs,
                    t.of_the_theory(),
                )
            ),
        ),
        (
            "the cover's groups and maps",
            format!(
                "{:?}",
                (
                    &c.groups,
                    &c.member_of,
                    &c.marks,
                    &c.killed_alone,
                    c.in_no_group()
                )
            ),
        ),
        (
            "the cover's graph",
            format!(
                "{:?}",
                (&c.excluded, &c.components, c.steps, c.cut, &c.alive_at_root)
            ),
        ),
        ("root's cliques", cliques_as_they_are(&c.root_cliques)),
        (
            "the cover's domain and work",
            format!(
                "{:?}",
                (
                    c.monotone,
                    c.small_monotone,
                    &c.stopped,
                    c.complete(),
                    c.root_work,
                    c.small_work
                )
            ),
        ),
        (
            "what root owes, and its witnesses",
            format!("{:?}", r.exact.owed),
        ),
        (
            "the owed groups",
            format!("{:?}", (&r.exact.owing, &r.exact.owed_by, &r.exact.about)),
        ),
        ("the grids", format!("{:?}", r.exact.grids)),
        ("the coupling", {
            let c = &r.coupling;
            format!(
                "{:?}",
                (
                    (&c.questions, &c.question_of, &c.ties, &c.reasons),
                    (&c.edges, &c.one_step, &c.debts, c.unlicensed),
                    (&c.components, &c.conflict_components, &c.committed),
                    (c.pairs_read, c.node, c.node_stopped, c.cut)
                )
            )
        }),
        (
            "the coupling from the node alone",
            match &r.node {
                Ok(c) => {
                    let texts: Vec<&str> = c.candidates.iter().map(|c| c.text.as_str()).collect();
                    format!(
                        "{:?}",
                        (
                            (texts, &c.questions, c.owed, &c.question_of),
                            (&c.edges, &c.one_step, &c.pairs, &c.components),
                            (c.dead, c.node, c.node_stopped, c.cut)
                        )
                    )
                }
                Err(e) => e.clone(),
            },
        ),
        ("the view's structure.json", r.document.clone()),
        ("the view's graphs", r.graphs.join("\n")),
    ]
}

/// **The document with what an import form may respell taken out**: every
/// string through `strip`, and every list of strings sorted, since a list
/// of names is in the order of their texts.
fn document_as_sets(document: &str, strip: &dyn Fn(&str) -> String) -> serde_json::Value {
    use serde_json::Value;
    fn walk(v: Value, strip: &dyn Fn(&str) -> String) -> Value {
        match v {
            // A name after `=` or inside backticks starts a name too.
            Value::String(s) => Value::String(strip(&s.replace('=', " = ").replace('`', " ` "))),
            Value::Array(items) => {
                let mut items: Vec<Value> = items.into_iter().map(|x| walk(x, strip)).collect();
                if items.iter().all(Value::is_string) {
                    items.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
                }
                Value::Array(items)
            }
            Value::Object(m) => {
                Value::Object(m.into_iter().map(|(k, x)| (k, walk(x, strip))).collect())
            }
            other => other,
        }
    }
    walk(
        serde_json::from_str(document).expect("structure.json is JSON"),
        strip,
    )
}

/// Where two documents first differ, as a path with the two values.
fn document_difference(a: &serde_json::Value, b: &serde_json::Value, at: &str) -> Option<String> {
    use serde_json::Value;
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let keys: BTreeSet<&String> = x.keys().chain(y.keys()).collect();
            keys.into_iter().find_map(|k| match (x.get(k), y.get(k)) {
                (Some(p), Some(q)) => document_difference(p, q, &format!("{at}.{k}")),
                _ => Some(format!("{at}.{k}: in one document only")),
            })
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .find_map(|(k, (p, q))| document_difference(p, q, &format!("{at}[{k}]"))),
        (p, q) if p == q => None,
        (p, q) => {
            let cut = |v: &Value| v.to_string().chars().take(240).collect::<String>();
            Some(format!(
                "{at}:\n    one:   {}\n    other: {}",
                cut(p),
                cut(q)
            ))
        }
    }
}

/// One layer as sets of texts, each through `name`.
fn layer_as_sets(l: &PairLayer, name: &dyn Fn(u32) -> String) -> String {
    let one = |v: &[u32]| -> BTreeSet<String> { v.iter().map(|&b| name(b)).collect() };
    let ordered = |v: &[(u32, u32)]| -> BTreeSet<(String, String)> {
        v.iter().map(|&(a, b)| (name(a), name(b))).collect()
    };
    let pairs = |v: &[(u32, u32)]| -> BTreeSet<BTreeSet<String>> {
        v.iter()
            .map(|&(a, b)| BTreeSet::from([name(a), name(b)]))
            .collect()
    };
    let classes: BTreeSet<BTreeSet<String>> = l.equivalent.iter().map(|c| one(c)).collect();
    let against_l3: BTreeSet<(BTreeSet<String>, bool)> = l
        .against_l3
        .iter()
        .map(|&(a, b, dead)| (BTreeSet::from([name(a), name(b)]), dead))
        .collect();
    format!(
        "{:?}",
        (
            (l.monotone, l.root_dead, l.singles_left, &l.stopped),
            (
                one(&l.at_root),
                one(&l.dead_alone),
                ordered(&l.implies),
                classes
            ),
            (pairs(&l.excluded), pairs(&l.interacting)),
            (pairs(&l.against_l1), against_l3),
            (l.pairs, &l.licences, l.work),
        )
    )
}

/// **The whole structure as sets of texts**, every name through `strip`,
/// which takes a module's prefix or an alias off it.
fn as_sets(r: &Read, strip: &dyn Fn(&str) -> String) -> Vec<(&'static str, String)> {
    let (t, c) = (&r.theory, &r.cover);
    let name = |b: u32| strip(&t.root.candidates[b as usize].text);
    let one = |v: &[u32]| -> BTreeSet<String> { v.iter().map(|&b| name(b)).collect() };
    let sets =
        |v: &[Vec<u32>]| -> BTreeSet<BTreeSet<String>> { v.iter().map(|g| one(g)).collect() };
    let pairs = |v: &[(u32, u32)]| -> BTreeSet<BTreeSet<String>> {
        v.iter()
            .map(|&(a, b)| BTreeSet::from([name(a), name(b)]))
            .collect()
    };
    let candidates: BTreeSet<String> = (0..t.root.n() as u32).map(name).collect();
    // What is stated, as a bag: two forms may write one fact's text alike.
    let mut stated: BTreeMap<(String, &str, bool, &str, bool), usize> = BTreeMap::new();
    for s in &t.stated {
        *stated
            .entry((
                strip(&s.text),
                s.origin.as_str(),
                s.kept,
                s.clause,
                s.read_absent,
            ))
            .or_default() += 1;
    }
    let of_theory: BTreeSet<(BTreeSet<String>, bool)> = t
        .root
        .excluded
        .iter()
        .zip(t.of_the_theory())
        .map(|(&(a, b), theory)| (BTreeSet::from([name(a), name(b)]), theory))
        .collect();
    // What root owes, as a bag: each instance by its text, with its
    // alternatives or the reason, its witnesses, and how it stands. A group
    // is its members.
    let x = &r.exact;
    let group = |g: u32| one(c.members(g));
    // A name after `=` or inside backticks starts a name too.
    let strip = |s: &str| strip(&s.replace('=', " = ").replace('`', " ` "));
    let mut owed: BTreeMap<String, usize> = BTreeMap::new();
    for (i, owing) in x.owed.table().iter().zip(&x.owing) {
        let witnesses = i.witnesses.as_ref().map_err(|e| strip(e)).map(|ws| {
            ws.iter()
                .map(|w| {
                    let is = match w.is {
                        WitnessIs::Candidate(b) => format!("the candidate {}", name(b)),
                        other => other.as_str().to_string(),
                    };
                    (
                        strip(&w.text),
                        is,
                        one(&w.held_at_root),
                        one(&w.held_in_theory),
                    )
                })
                .collect::<BTreeSet<_>>()
        });
        let stands = match owing {
            Owing::Group(g) => format!("the group {:?}", group(*g)),
            Owing::Inside(groups) => format!(
                "inside {:?}",
                groups.iter().map(|&g| group(g)).collect::<BTreeSet<_>>()
            ),
            Owing::NotExcluding { alive, holds } => format!(
                "not excluding, {alive} alive, holding {:?}",
                holds.iter().map(|&g| group(g)).collect::<BTreeSet<_>>()
            ),
            Owing::One(b) => format!("one alternative, {}", name(*b)),
            Owing::None => "no alternative".to_string(),
            Owing::Unlicensed { payers, about } => format!(
                "unlicensed, paid by {:?}, about {:?}",
                payers.as_ref().map(|p| one(p)),
                about.map(group)
            ),
        };
        let said = format!(
            "{:?}",
            (
                strip(&i.rule),
                strip(&i.text),
                i.alternatives
                    .as_ref()
                    .map(|a| one(a))
                    .map_err(|e| strip(e)),
                i.denied,
                witnesses,
                stands
            )
        );
        *owed.entry(said).or_default() += 1;
    }
    // Each group with how many instances owe it and are about it.
    let owed_groups: BTreeMap<BTreeSet<String>, (usize, usize)> = (0..c.groups.len())
        .map(|g| (group(g as u32), (x.owed_by[g].len(), x.about[g].len())))
        .collect();
    // A grid is its two families, in either order; a component that is none
    // is its members and the reason.
    let grids: BTreeSet<BTreeSet<BTreeSet<BTreeSet<String>>>> = x
        .grids
        .grids
        .iter()
        .map(|g| {
            g.families
                .iter()
                .map(|f| f.iter().map(|&g| group(g)).collect())
                .collect()
        })
        .collect();
    let not_grids: BTreeSet<(BTreeSet<String>, String)> = x
        .grids
        .not
        .iter()
        .map(|&(k, why)| (one(&c.components[k as usize]), format!("{why:?}")))
        .collect();
    // Each candidate with its mark and the groups that hold it.
    let members: BTreeMap<String, (&str, BTreeSet<BTreeSet<String>>)> = (0..c.n() as u32)
        .map(|b| {
            let groups = c.groups_of(b).iter().map(|&g| one(c.members(g))).collect();
            (name(b), (c.marks[b as usize].as_str(), groups))
        })
        .collect();
    vec![
        (
            "the candidates",
            // No two candidates are one text with the prefixes off.
            format!("{:?}", (t.root.n(), candidates)),
        ),
        ("root's pair layer", layer_as_sets(&t.root, &name)),
        (
            "the small KB's pair layer",
            match &t.small {
                Ok(s) => layer_as_sets(s, &name),
                Err(e) => strip(e),
            },
        ),
        (
            "what the program states, and what is kept",
            format!("{stated:?}"),
        ),
        (
            "the theory's share beside root",
            format!(
                "{:?}",
                (
                    t.rung,
                    t.rules_naming_objects
                        .iter()
                        .map(|r| strip(r))
                        .collect::<BTreeSet<_>>(),
                    t.small_beyond_root,
                    one(&t.against_root.singles),
                    pairs(&t.against_root.pairs),
                    of_theory,
                )
            ),
        ),
        (
            "the cover's groups and maps",
            format!(
                "{:?}",
                (
                    sets(&c.groups),
                    members,
                    one(&c.killed_alone),
                    one(&c.in_no_group())
                )
            ),
        ),
        (
            "the cover's graph",
            format!(
                "{:?}",
                (
                    pairs(&c.excluded),
                    sets(&c.components),
                    c.cut,
                    c.alive_at_root.as_ref().map(|v| pairs(v))
                )
            ),
        ),
        (
            "root's cliques",
            format!(
                "{:?}",
                c.root_cliques.as_ref().map(|r| (sets(&r.cliques), r.cut))
            ),
        ),
        (
            "the cover's domain and work",
            format!(
                "{:?}",
                (
                    c.monotone,
                    c.small_monotone,
                    &c.stopped,
                    c.complete(),
                    c.root_work,
                    c.small_work
                )
            ),
        ),
        (
            "what root owes, and its witnesses",
            format!(
                "{:?}",
                (
                    x.owed.root_owes,
                    x.owed.instances.as_ref().err(),
                    x.owed.singles_read,
                    x.owed.theory_read,
                    owed
                )
            ),
        ),
        ("the owed groups", format!("{owed_groups:?}")),
        ("the grids", format!("{:?}", (grids, not_grids))),
        ("the coupling", coupling_as_sets(r, &strip)),
        ("the coupling from the node alone", node_as_sets(r, &strip)),
    ]
}

/// The coupling read from the node alone as sets of texts: a question is
/// its members, with whether an instance owes it; an edge its questions; a
/// pair and a component the candidates each holds. The node's cost follows
/// the program and not the spelling.
fn node_as_sets(r: &Read, strip: &dyn Fn(&str) -> String) -> String {
    let c = match &r.node {
        Ok(c) => c,
        Err(e) => return strip(e),
    };
    let name = |b: u32| strip(c.text(b));
    let one = |v: &[u32]| -> BTreeSet<String> { v.iter().map(|&b| name(b)).collect() };
    type Sets = BTreeSet<BTreeSet<String>>;
    let questions: BTreeSet<(bool, BTreeSet<String>)> = c
        .questions
        .iter()
        .enumerate()
        .map(|(q, members)| (q < c.owed, one(members)))
        .collect();
    let edges_of = |edges: &[Vec<u32>]| -> BTreeSet<Sets> {
        edges
            .iter()
            .map(|e| e.iter().map(|&q| one(&c.questions[q as usize])).collect())
            .collect()
    };
    let pairs: Sets = c.pairs.iter().map(|&(a, b)| one(&[a, b])).collect();
    let components: Sets = c.components.iter().map(|k| one(k)).collect();
    format!(
        "{:?}",
        (
            (questions, edges_of(&c.edges), edges_of(&c.one_step)),
            (pairs, components),
            (c.dead, c.node, c.node_stopped, c.cut)
        )
    )
}

/// The coupling as sets of texts: a question is its members, an edge its
/// questions with what it covers, its debts and the statements it names,
/// and a tie, a component and a debt the candidates each holds. What the node cost
/// follows the program and not the spelling, and is compared with them.
fn coupling_as_sets(r: &Read, strip: &dyn Fn(&str) -> String) -> String {
    let c = &r.coupling;
    let name = |b: u32| strip(c.text(b));
    let one = |v: &[u32]| -> BTreeSet<String> { v.iter().map(|&b| name(b)).collect() };
    let question = |q: u32| -> BTreeSet<String> {
        match c.questions[q as usize] {
            // A reading a test cut may hold fewer groups than it names.
            Question::Group(g) => one(r.cover.groups.get(g as usize).map_or(&[], |m| m)),
            Question::Alone(b) => one(&[b]),
        }
    };
    let questions: BTreeSet<(bool, BTreeSet<String>)> = (0..c.questions.len() as u32)
        .map(|q| {
            let alone = matches!(c.questions[q as usize], Question::Alone(_));
            (alone, question(q))
        })
        .collect();
    let reason = |k: u32| strip(&c.reasons[k as usize]);
    let ties: BTreeSet<_> = c
        .ties
        .iter()
        .map(|t| {
            let stated: Option<BTreeSet<String>> = t
                .stated
                .as_ref()
                .map(|s| s.iter().map(|&k| reason(k)).collect());
            (one(&[t.pair.0, t.pair.1]), t.kind.as_str(), stated)
        })
        .collect();
    type Sets = BTreeSet<BTreeSet<String>>;
    let questions_of = |v: &[u32]| -> Sets { v.iter().map(|&q| question(q)).collect() };
    let edges_of = |edges: &[structure::Edge]| -> BTreeSet<_> {
        edges
            .iter()
            .map(|e| {
                let stated: BTreeSet<(String, u32)> =
                    e.stated.iter().map(|&(k, n)| (reason(k), n)).collect();
                // A debt it is the edge of, by its instance.
                let debts: BTreeSet<String> = e
                    .debts
                    .iter()
                    .map(|&d| strip(&c.debts[d as usize].instance))
                    .collect();
                (
                    questions_of(&e.questions),
                    format!("{:?}", (e.pairs, e.node)),
                    debts,
                    stated,
                )
            })
            .collect()
    };
    let edges = (edges_of(&c.edges), c.one_step.as_deref().map(edges_of));
    let parts = |v: &[Vec<u32>]| -> Sets { v.iter().map(|k| one(k)).collect() };
    let components = (parts(&c.components), parts(&c.conflict_components));
    // The debts as a bag: two instances may read alike with the prefixes
    // off.
    let mut debts: BTreeMap<_, usize> = BTreeMap::new();
    for d in &c.debts {
        let key = (
            strip(&d.rule),
            strip(&d.instance),
            d.licensed,
            one(&d.payers),
            questions_of(&d.questions),
        );
        *debts.entry(key).or_default() += 1;
    }
    let debts = (debts, c.unlicensed);
    format!(
        "{:?}",
        (
            (questions, ties, edges, components, debts),
            (
                one(&c.committed),
                c.pairs_read,
                c.node,
                c.node_stopped,
                c.cut
            )
        )
    )
}

/// The first part two readings differ in, with where.
fn difference(a: &[(&'static str, String)], b: &[(&'static str, String)]) -> Option<String> {
    a.iter().zip(b).find(|(x, y)| x != y).map(|(x, y)| {
        let at =
            x.1.char_indices()
                .zip(y.1.chars())
                .find(|((_, p), q)| p != q)
                .map_or(x.1.len().min(y.1.len()), |((i, _), _)| i);
        let from = (0..=at.saturating_sub(120))
            .rev()
            .find(|&i| x.1.is_char_boundary(i) && y.1.is_char_boundary(i))
            .unwrap_or(0);
        let cut = |s: &str| s[from..].chars().take(320).collect::<String>();
        format!(
            "{}:\n    one:   …{}\n    other: …{}",
            x.0,
            cut(&x.1),
            cut(&y.1)
        )
    })
}

/// What one program's readings came to.
#[derive(Default)]
struct Held {
    failures: Vec<String>,
    /// Two readings were compared.
    compared: bool,
    candidates: usize,
    pairs: usize,
    theory_pairs: usize,
    groups: usize,
    root_cliques: usize,
    flagged: bool,
    /// Names and facts the permutation moved, of those it could.
    moved: (usize, usize),
    /// Import forms compared with the program as written.
    forms: usize,
    /// Of them, those in which a candidate is written otherwise, and those
    /// in which the universe's order is another.
    respelled: usize,
    reordered: usize,
    /// Forms whose document was compared with the program's as written:
    /// those that leave the universe's order as it was.
    documents: usize,
}

fn counts(out: &mut Held, r: &Read) {
    out.candidates = r.theory.root.n();
    out.pairs = r.theory.root.excluded.len();
    out.theory_pairs = r.cover.excluded.len();
    out.groups = r.cover.groups.len();
    out.root_cliques = r.cover.root_cliques.as_ref().map_or(0, |c| c.cliques.len());
    out.flagged = !r.theory.root.monotone;
}

// ── The sweep ─────────────────────────────────────────────────────────

const SEED: i64 = 1;

/// The program in a permuted id space, held to `plain`, its reading as
/// loaded, which left `terms`.
fn held_under_ids(
    rel: &str,
    text: &str,
    path: &Path,
    plain: &Result<Read, String>,
    terms: &Terms,
) -> Held {
    let mut out = Held::default();
    // A refusal is held too: refused alike in both id spaces. The permuted
    // table is then of a run that interned nothing past the load.
    let shuffled = id_permutation::permuted(terms, SEED);
    out.moved = id_permutation::moved(terms, &shuffled);
    let base = path.parent().expect("a directory");
    let Some(again) = read(text, path, base, shuffled) else {
        out.failures
            .push(format!("{rel}: does not load in a permuted id space"));
        return out;
    };
    match (plain, &again) {
        (Ok(a), Ok(b)) => {
            out.compared = true;
            counts(&mut out, a);
            if let Some(d) = difference(&as_it_is(a), &as_it_is(b)) {
                out.failures
                    .push(format!("{rel}: moved under a permuted id space, in {d}"));
            }
        }
        (Err(_), Err(_)) => {}
        (a, b) => out.failures.push(format!(
            "{rel}: read in one id space and refused in the other: {:?} / {:?}",
            a.as_ref().err(),
            b.as_ref().err()
        )),
    }
    out
}

/// Whether `text` writes an import.
fn imports(text: &str, path: &Path) -> bool {
    let mut ast = Ast::new();
    parse(&mut ast, text, path.to_str())
        .map(|forms| forms.iter().any(|&f| ast.head_name(f) == Some("import")))
        .unwrap_or(false)
}

/// The program with its imports whole and with each under an alias, held
/// to `written`, its reading as the file writes it.
fn held_under_imports(rel: &str, text: &str, path: &Path, written: &Result<Read, String>) -> Held {
    let mut out = Held::default();
    if let Ok(r) = written {
        counts(&mut out, r);
    }
    let mut prefixes = import_forms::module_prefixes(path.parent().expect("a directory"));
    for mode in ['W', 'A'] {
        let Some((form, extra)) = import_forms::form(path, text, mode) else {
            out.failures
                .push(format!("{rel}: form {mode} could not be written"));
            continue;
        };
        for p in extra {
            if !prefixes.contains(&p) {
                prefixes.push(p);
            }
        }
        prefixes.sort_by_key(|p| std::cmp::Reverse(p.len()));
        let chars: Vec<Vec<char>> = prefixes.iter().map(|p| p.chars().collect()).collect();
        let strip = |s: &str| import_forms::strip(s, &chars);
        let scratch = import_forms::beside_its_modules(path, &form);
        let name = scratch.path().join(path.file_name().expect("a name"));
        let Some(other) = read(&form, &name, scratch.path(), Terms::new()) else {
            out.failures
                .push(format!("{rel}: form {mode} does not load\n{form}"));
            continue;
        };
        match (written, &other) {
            (Ok(a), Ok(b)) => {
                out.compared = true;
                out.forms += 1;
                let order = |r: &Read, name: &dyn Fn(&str) -> String| -> Vec<String> {
                    let candidates = r.theory.root.candidates.iter();
                    candidates.map(|c| name(&c.text)).collect()
                };
                let as_written = |s: &str| s.to_string();
                out.respelled += usize::from(order(a, &as_written) != order(b, &as_written));
                let reordered = order(a, &strip) != order(b, &strip);
                out.reordered += usize::from(reordered);
                // The document's places follow the universe's order.
                if !reordered {
                    out.documents += 1;
                    let one = document_as_sets(&a.document, &strip);
                    let other = document_as_sets(&b.document, &strip);
                    if let Some(d) = document_difference(&one, &other, "structure.json") {
                        out.failures.push(format!(
                            "{rel}: the document of the program as written and of its form \
                             {mode} differ, at {d}"
                        ));
                    }
                }
                let (a, b) = (as_sets(a, &strip), as_sets(b, &strip));
                if let Some(d) = difference(&a, &b) {
                    out.failures.push(format!(
                        "{rel}: the program as written and its form {mode} differ, in {d}"
                    ));
                }
            }
            (Err(_), Err(_)) => out.forms += 1,
            (a, b) => out.failures.push(format!(
                "{rel}: read as written and refused in form {mode}, or the reverse: {:?} / {:?}",
                a.as_ref().err(),
                b.as_ref().err()
            )),
        }
    }
    out
}

/// One program that searches: its reading as loaded, held to a permuted id
/// space and, where it imports, to its two other import forms. `None` where
/// it does not load or does not search.
fn held(path: &Path) -> Option<(Held, Option<Held>)> {
    if !searches(path) {
        return None;
    }
    let rel = rel_of(path);
    let text = std::fs::read_to_string(path).ok()?;
    let plain = read(&text, path, path.parent()?, Terms::new())?;
    let terms = match &plain {
        Ok(r) => r.terms.clone(),
        Err(_) => Terms::new(),
    };
    let ids = held_under_ids(&rel, &text, path, &plain, &terms);
    let forms = imports(&text, path).then(|| held_under_imports(&rel, &text, path, &plain));
    Some((ids, forms))
}

/// **The sweep, once a run**: every program read as loaded once, and both
/// invariances held to that one reading. The two tests below each read
/// their half of it.
fn sweep() -> &'static [(Held, Option<Held>)] {
    static SWEEP: std::sync::OnceLock<Vec<(Held, Option<Held>)>> = std::sync::OnceLock::new();
    SWEEP.get_or_init(|| {
        let files: Vec<PathBuf> = files(slow_too());
        ein_corpus::pool::map(
            "structure_invariance",
            &files,
            |p| (rel_of(p), String::new()),
            |p| held(p),
        )
        .into_iter()
        .flatten()
        .collect()
    })
}

/// **The structure does not depend on the order ids were assigned in.**
#[test]
fn the_structure_is_the_same_under_a_permutation_of_the_ids() {
    let mut failures: Vec<&str> = Vec::new();
    let (mut searching, mut compared, mut with_pairs, mut with_groups) = (0usize, 0, 0, 0);
    let (mut flagged, mut groups, mut cliques, mut unmoved) = (0usize, 0usize, 0usize, 0usize);
    for (h, _) in sweep() {
        failures.extend(h.failures.iter().map(String::as_str));
        searching += 1;
        compared += usize::from(h.compared);
        with_pairs += usize::from(h.pairs > 0);
        with_groups += usize::from(h.groups > 0);
        flagged += usize::from(h.flagged);
        groups += h.groups;
        cliques += h.root_cliques;
        // A permutation that moves nothing compares a run with itself.
        unmoved += usize::from(h.compared && h.moved.0 * 2 < h.moved.1);
    }
    assert!(
        failures.is_empty(),
        "{} program(s) whose structure depends on the id order:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "structure_invariance, ids: {searching} programs that search, {compared} compared, \
         {with_pairs} with an excluded pair, {with_groups} with a group ({groups} groups, \
         {cliques} cliques of root's graph), {flagged} flagged"
    );
    assert_eq!(unmoved, 0, "permutations that moved under half the names");
    // Not vacuous: programs of each kind.
    assert!(compared >= 150, "only {compared} programs compared");
    assert!(with_pairs >= 30, "only {with_pairs} with an excluded pair");
    assert!(with_groups >= 35, "only {with_groups} with a group");
    assert!(flagged >= 15, "only {flagged} flagged programs compared");
}

/// **The structure does not depend on how the program imports.**
#[test]
fn the_structure_is_the_same_under_every_import_form() {
    let mut failures: Vec<&str> = Vec::new();
    let (mut importing, mut forms, mut with_pairs, mut with_groups) = (0usize, 0, 0, 0);
    let (mut groups, mut cliques) = (0usize, 0usize);
    let (mut respelled, mut reordered, mut reordered_with_groups) = (0usize, 0usize, 0usize);
    let mut documents = 0usize;
    for h in sweep().iter().filter_map(|(_, h)| h.as_ref()) {
        failures.extend(h.failures.iter().map(String::as_str));
        importing += 1;
        forms += h.forms;
        documents += h.documents;
        respelled += h.respelled;
        reordered += h.reordered;
        reordered_with_groups += usize::from(h.reordered > 0 && h.groups > 1);
        with_pairs += usize::from(h.pairs > 0);
        with_groups += usize::from(h.groups > 0);
        groups += h.groups;
        cliques += h.root_cliques;
    }
    assert!(
        failures.is_empty(),
        "{} place(s) where the structure depends on the import form:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!(
        "structure_invariance, imports: {importing} programs that search and import, {forms} \
         forms compared, {with_pairs} with an excluded pair, {with_groups} with a group \
         ({groups} groups, {cliques} cliques of root's graph); a candidate is written \
         otherwise in {respelled} forms and the universe's order is another in {reordered}, \
         {reordered_with_groups} programs with two groups or more among them; the view's \
         document compared in {documents} forms"
    );
    // The document is compared in every form that leaves the universe's
    // order, which is nearly all of them.
    assert!(
        documents + reordered >= forms && documents >= 80,
        "only {documents} documents compared, of {forms} forms"
    );
    // Every program in both forms, and not vacuous.
    assert_eq!(forms, 2 * importing);
    assert!(importing >= 40, "only {importing} programs that import");
    assert!(with_pairs >= 20, "only {with_pairs} with an excluded pair");
    assert!(with_groups >= 20, "only {with_groups} with a group");
    // And the prefixes are taken off something: forms that write a
    // candidate otherwise than the program does.
    assert!(
        respelled >= 10,
        "only {respelled} forms respell a candidate"
    );
}

/// A module that declares the relation a program proposes facts of, and
/// the program: two relations' candidates, one written by an imported name.
const PLACES: &str = r#"(relation at Thing Place :why "{?1} is at {?2}")
"#;
const REORDERED: &str = r#"(import std.algebra :symbols (functional))
(import places :symbols (at))
(relation is-a  T T         :why "{?1} is a {?2}")
(relation likes Thing Thing :why "{?1} likes {?2}")
(is-a a Thing) (is-a b Thing) (is-a p Place) (is-a q Place)
(functional at)
(functional likes)
(relation shy Thing :why "{?1} is shy")
(shy a)
(rule shy-at-p ()
  :match  (and (shy ?x) (at ?x p) (likes ?x ?x))
  :assert (false)
  :why    "{?x} is shy, and is not at p while liking itself")
(rule liked ()
  :match  (and (is-a ?y Thing) (absent (and (is-a ?x Thing) (likes ?x ?y))))
  :assert (open likes)
  :why    "{?y} owes someone who likes it")
(hrule put ()
  :match  (and (is-a ?x Thing) (is-a ?y Place))
  :assert (at ?x ?y)
  :why    "guess that {?x} is at {?y}")
(hrule like ()
  :match  (and (is-a ?x Thing) (is-a ?y Thing))
  :assert (likes ?x ?y)
  :why    "guess that {?x} likes {?y}")
(query :goal (at ?x ?y))
"#;

/// **A form that orders the universe otherwise moves no set.** Few corpus
/// programs propose facts of an imported relation beside facts of their
/// own, so the corpus's forms seldom reorder a universe, which is what the
/// comparison as sets is for. Here one does: `(at a p)` is before `(likes a
/// a)` as written and as `(imp0.at a p)`, and after it as `(places.at a
/// p)`. The four groups, a thing's places and a thing's likings, are the
/// same sets in all three forms, and so is the coupling, one edge between
/// two of them by one stated fact and one by two debts; and a cover that
/// kept the first two groups of its own order would not be, which is what
/// the comparison would say.
#[test]
fn a_form_that_reorders_the_universe_moves_no_set() {
    let dir = ein_corpus::Scratch::new("ein-structure-forms");
    std::fs::write(dir.path().join("places.ein"), PLACES).expect("the module");
    let path = dir.path().join("reordered.ein");
    std::fs::write(&path, REORDERED).expect("the program");
    let written = read(REORDERED, &path, dir.path(), Terms::new())
        .expect("loads")
        .expect("a structure");
    assert_eq!((written.cover.n(), written.cover.groups.len()), (8, 4));
    // One statement ties a thing's places to its likings: one tie, and one
    // edge of the two questions, which every form must name by the same
    // two sets.
    let c = &written.coupling;
    // And each thing owes a liker, one of two candidates in two questions:
    // two debts, and one edge of the two things' likings that is theirs.
    assert_eq!(c.ties.len(), 1);
    assert_eq!((c.debts.len(), c.unlicensed), (2, 0), "{:?}", c.unlicensed);
    let kinds: Vec<_> = c.edges.iter().map(|e| (e.arity(), e.kind())).collect();
    assert_eq!(
        kinds,
        [
            (2, structure::EdgeKind::Tie),
            (2, structure::EdgeKind::Debt)
        ],
        "{:?}",
        c.edges
    );
    let order = |r: &Read, strip: &dyn Fn(&str) -> String| -> Vec<String> {
        let candidates = r.theory.root.candidates.iter();
        candidates.map(|c| strip(&c.text)).collect()
    };
    let mut reordered = 0;
    for mode in ['W', 'A'] {
        let (form, extra) = import_forms::form(&path, REORDERED, mode).expect("the form");
        let mut prefixes = import_forms::module_prefixes(dir.path());
        prefixes.extend(extra);
        prefixes.sort_by_key(|p| std::cmp::Reverse(p.len()));
        let chars: Vec<Vec<char>> = prefixes.iter().map(|p| p.chars().collect()).collect();
        let strip = |s: &str| import_forms::strip(s, &chars);
        let scratch = import_forms::beside_its_modules(&path, &form);
        let other = read(
            &form,
            &scratch.path().join("reordered.ein"),
            scratch.path(),
            Terms::new(),
        )
        .expect("the form loads")
        .expect("a structure");
        assert_eq!(
            difference(&as_sets(&written, &strip), &as_sets(&other, &strip)),
            None,
            "form {mode}"
        );
        if order(&written, &strip) == order(&other, &strip) {
            continue;
        }
        reordered += 1;
        // A cover that followed the order: its first two groups.
        let first_two = |r: &Read| {
            let mut cut = Read {
                theory: r.theory.clone(),
                cover: r.cover.clone(),
                exact: r.exact.clone(),
                coupling: r.coupling.clone(),
                node: r.node.clone(),
                document: r.document.clone(),
                graphs: r.graphs.clone(),
                terms: Terms::new(),
            };
            cut.cover.groups.truncate(2);
            for groups in &mut cut.cover.member_of {
                groups.retain(|&g| g < 2);
            }
            cut
        };
        let d = difference(
            &as_sets(&first_two(&written), &strip),
            &as_sets(&first_two(&other), &strip),
        )
        .expect("a difference");
        assert!(d.starts_with("the cover's groups and maps"), "{d}");
    }
    assert_eq!(
        reordered, 1,
        "the whole import orders the universe otherwise"
    );
}

/// **A structure that follows the id order, or a name's spelling, is
/// caught**: the comparisons are of what they say they are. A reading with
/// two candidates' groups exchanged differs from itself place for place,
/// and a reading whose names keep their prefix differs as sets.
#[test]
fn a_structure_that_moves_is_caught() {
    let path = repo_root().join("examples/zebra2.ein");
    let text = std::fs::read_to_string(&path).expect("the program");
    let base = path.parent().expect("a directory");
    let r = read(&text, &path, base, Terms::new())
        .expect("loads")
        .expect("a structure");
    let said = as_it_is(&r);
    assert!(difference(&said, &said).is_none());

    let mut moved = Read {
        theory: r.theory.clone(),
        cover: r.cover.clone(),
        exact: r.exact.clone(),
        coupling: r.coupling.clone(),
        node: r.node.clone(),
        document: r.document.clone(),
        graphs: r.graphs.clone(),
        terms: Terms::new(),
    };
    moved.cover.member_of.swap(0, 1);
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the cover's groups and maps"), "{d}");

    moved.cover = r.cover.clone();
    moved.theory.root.excluded.pop();
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("root's pair layer"), "{d}");

    // What root owes in another order, a group owed by another instance, a
    // grid's families exchanged: each is caught place for place.
    moved.theory = r.theory.clone();
    let instances = moved.exact.owed.instances.as_mut().expect("a table");
    instances.swap(0, 1);
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("what root owes"), "{d}");
    moved.exact = r.exact.clone();
    moved.exact.owed_by.swap(0, 1);
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the owed groups"), "{d}");
    moved.exact = r.exact.clone();
    moved.exact.grids.grids[0].families.swap(0, 1);
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the grids"), "{d}");
    // As sets a grid is its two families in either order, and a group that
    // loses the instance that owes it is caught.
    let plain = |s: &str| s.to_string();
    assert_eq!(
        difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)),
        None
    );
    moved.exact.owed_by[0].clear();
    let d = difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)).expect("a difference");
    assert!(d.starts_with("the owed groups"), "{d}");

    // An edge lost, and a component's candidates in another order.
    moved.exact = r.exact.clone();
    moved.coupling.edges.pop();
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the coupling"), "{d}");
    let d = difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)).expect("a difference");
    assert!(d.starts_with("the coupling"), "{d}");
    moved.coupling = r.coupling.clone();
    moved.coupling.one_step.as_mut().expect("kept").pop();
    let d = difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)).expect("a difference");
    assert!(d.starts_with("the coupling"), "{d}");
    moved.coupling = r.coupling.clone();
    moved.coupling.conflict_components.pop();
    let d = difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)).expect("a difference");
    assert!(d.starts_with("the coupling"), "{d}");
    moved.coupling = r.coupling.clone();
    moved.coupling.components[0].reverse();
    assert!(difference(&said, &as_it_is(&moved)).is_some());
    assert_eq!(
        difference(&as_sets(&r, &plain), &as_sets(&moved, &plain)),
        None
    );

    let renamed = |s: &str| s.replace("color-loc", "m.color-loc");
    let d = difference(&as_sets(&r, &plain), &as_sets(&r, &renamed)).expect("a difference");
    assert!(d.starts_with("the candidates"), "{d}");

    // The view's document, held the same two ways. Place for place it is
    // its bytes, so one candidate's mark moved is a difference.
    let marked = r.document.replacen("\"alive alone\"", "\"dead alone\"", 1);
    assert_ne!(
        marked, r.document,
        "the document marks a candidate alive alone"
    );
    moved.coupling = r.coupling.clone();
    moved.document = marked.clone();
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the view's structure.json"), "{d}");
    // As sets it is the same with itself, and says where it is not.
    let whole = document_as_sets(&r.document, &plain);
    assert_eq!(document_difference(&whole, &whole, "structure.json"), None);
    let d = document_difference(&whole, &document_as_sets(&marked, &plain), "structure.json")
        .expect("a difference");
    assert!(d.starts_with("structure.json.candidates[0].at_root"), "{d}");
    // A name that keeps its prefix is a difference, and none once the
    // prefix is taken off: the comparison passes for the strip and not for
    // want of looking.
    let respelled = r.document.replace("color-loc", "m.color-loc");
    assert!(
        document_difference(
            &whole,
            &document_as_sets(&respelled, &plain),
            "structure.json"
        )
        .is_some()
    );
    let prefix = [vec!['m']];
    let strip = |s: &str| import_forms::strip(s, &prefix);
    assert_eq!(
        document_difference(
            &document_as_sets(&r.document, &strip),
            &document_as_sets(&respelled, &strip),
            "structure.json"
        ),
        None
    );
    // And a graph's DOT moved is a difference too.
    moved.document = r.document.clone();
    moved.graphs[0].push_str("// moved\n");
    let d = difference(&said, &as_it_is(&moved)).expect("a difference");
    assert!(d.starts_with("the view's graphs"), "{d}");
}
