//! **`ein render h-space`**: a program's hypothesis space, its candidates
//! and the structure of the set, as a command.
//!
//! The view computes nothing of its own. [`ein_infer::structure::whole`]
//! reads every object once and `ein_render::structure` spells it: the
//! document, `h-space.json`, four graphs as DOT, and a read-out. So what is
//! held here is the door and the spelling:
//!
//! 1. **the document is the owner's reading**, on every instance: each
//!    object in it is held to the owner's own value, list for list, and on
//!    five of the six the binary's `--view json` is the library's document of
//!    an in-process reading byte for byte (the sixth, the dearest, is read
//!    once, in process). The objects: the candidates and their marks, the pair layer,
//!    the theory's share, the groups, the owed groups, the grids, the
//!    questions, the edges of both readings with their kinds, the debts with
//!    their licences, both sets of components, and the cost;
//! 2. **the instances say what the controls say**: on the declared control
//!    the five grids are five bijections, each of one relation the program
//!    states `bijective`, and every group is owed;
//! 3. **the door**: `--dump DIR` writes the document and the four graphs and
//!    moves nothing on stdout; each `--view` prints what its file holds;
//!    Graphviz draws every graph; a `.einb` gives the `.ein`'s structure; a
//!    job count moves no byte;
//! 4. **honesty**: a budget that stops a layer exits 0 and says what was not
//!    read, in the document, the read-out and on stderr; a layer that read
//!    its singles has its witnesses' holders read whatever it spent, and one
//!    that did not says so; a flagged program's
//!    document says what its statements are about; a program with no
//!    candidate, or a dead root, says so; a refused program is refused.
//!
//! And **`H` listed plain is the document's `H`**: on the six instances
//! `ein show hyps` prints the document's `candidates`, line for line, which
//! are the candidates `ein solve -H` lists, and every line is a fact a file
//! could hold.
//!
//! What the document holds under a permuted id space and under the three
//! import forms is `ein-infer/tests/structure_invariance.rs`'s, over every
//! corpus program that searches.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use ein_core::Terms;
use ein_corpus::{Scratch, repo_root};
use ein_infer::structure::{self, Mark, Question, Whole, WholeOptions};
use ein_ir::{Ast, load, parse};
use ein_render::structure as shown;
use serde_json::{Value, json};

/// The view's name under `ein render`.
const CMD: &str = shown::COMMAND;

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The six instances: the one-relation zebra, the declared control, its
/// under-determined twin, the blind enumerator's, the conflict of three and
/// the cryptarithm.
const INSTANCES: [&str; 6] = [
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/zebra2-minus-15.ein",
    "examples/branching/06_lookahead_on.ein",
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/beyond_arith/send_more_money.ein",
];

/// The owner's reading of `rel`, in process, as the view asks it, with the
/// library's document of it.
fn read(rel: &str) -> (Whole, String) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the program");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let asked = WholeOptions::default();
    let whole = structure::whole(&mut kb, &mut terms, &ast, &asked).expect("a structure");
    let inputs = shown::Inputs {
        program: rel,
        budget: asked.layer.budget,
        supports: asked.supports,
    };
    let graphs = shown::graphs(&whole, rel);
    let document = shown::document(&whole, &terms, &inputs, &graphs);
    (whole, document)
}

fn document_of(rel: &str, more: &[&str]) -> Value {
    let mut args = vec!["render", CMD, rel, "--view", "json"];
    args.extend(more);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{rel}: {}", r.err);
    serde_json::from_str(&r.out).unwrap_or_else(|e| panic!("{rel}: not JSON: {e}"))
}

/// **The document holds the owner's values**, list for list. Every list is
/// rebuilt here from the owner's own fields, by a candidate's text, and none
/// through `ein_render`.
fn held_to_the_owner(rel: &str, w: &Whole, d: &Value) {
    let t = &w.theory;
    let l = &t.root;
    let text = |b: u32| l.text(b);
    let one = |v: &[u32]| -> Value { json!(v.iter().map(|&b| text(b)).collect::<Vec<_>>()) };
    let two = |v: &[(u32, u32)]| -> Value {
        json!(
            v.iter()
                .map(|&(a, b)| [text(a), text(b)])
                .collect::<Vec<_>>()
        )
    };
    let sets = |v: &[Vec<u32>]| -> Value { json!(v.iter().map(|x| one(x)).collect::<Vec<_>>()) };
    let at = |what: &str| format!("{rel}: {what}");

    // The header, and the pair layer.
    assert_eq!(d["schema"], shown::SCHEMA, "{}", at("schema"));
    assert_eq!(d["program"], rel);
    assert_eq!(d["n"], l.n(), "{}", at("n"));
    assert_eq!(d["rung"], t.rung);
    assert_eq!(d["monotone"], l.monotone);
    assert_eq!(d["whole"], true, "{}: {}", at("whole"), d["unread"]);
    let p = &d["pairs"];
    assert_eq!(p["candidates"], one(&(0..l.n() as u32).collect::<Vec<_>>()));
    assert_eq!(p["at_root"], one(&l.at_root));
    assert_eq!(p["dead_alone"], one(&l.dead_alone), "{}", at("dead alone"));
    assert_eq!(p["implies"], two(&l.implies), "{}", at("implies"));
    assert_eq!(p["equivalent"], sets(&l.equivalent));
    assert_eq!(p["excluded"], two(&l.excluded), "{}", at("excluded"));
    assert_eq!(p["interacting"], two(&l.interacting));
    assert_eq!(p["pairs"]["all"], l.pairs.all);
    assert_eq!(p["pairs"]["saturated"], l.pairs.saturated);
    assert_eq!(p["pairs"]["left"], 0);

    // The theory's share.
    let small = t.small.as_ref().expect("the small KB has a layer");
    let th = &d["theory"];
    assert_eq!(
        th["theory_excluded"],
        two(&small.excluded),
        "{}",
        at("theory")
    );
    assert_eq!(th["theory_dead_alone"], one(&small.dead_alone));
    assert_eq!(th["small_kb_layer"]["excluded"], two(&small.excluded));
    assert_eq!(th["small_beyond_root"], t.small_beyond_root);
    let stated = th["stated"].as_array().expect("stated");
    assert_eq!(stated.len(), t.stated.len());
    for (s, row) in t.stated.iter().zip(stated) {
        assert_eq!(row["fact"], s.text);
        assert_eq!(row["kept"], s.kept, "{}", at(&s.text));
        assert_eq!(row["origin"], s.origin.as_str());
    }
    let of_theory = t.of_the_theory();
    let root_excluded = th["root_excluded"].as_array().expect("root_excluded");
    assert_eq!(root_excluded.len(), l.excluded.len());
    for (k, row) in root_excluded.iter().enumerate() {
        assert_eq!(
            row["of_the_theory"],
            of_theory[k],
            "{}",
            at("of the theory")
        );
        assert_eq!(row["support"], Value::Null, "no support was asked");
    }

    // The cover.
    let parts = w.parts.as_ref().expect("a cover");
    let (cover, x, c) = (&parts.cover, &parts.exactly_one, &parts.coupling);
    let cv = &d["cover"];
    let groups = cv["groups"].as_array().expect("groups");
    assert_eq!(groups.len(), cover.groups.len(), "{}", at("groups"));
    for (g, row) in groups.iter().enumerate() {
        assert_eq!(row["members"], one(cover.members(g as u32)));
        assert_eq!(
            row["open"],
            one(&cover.open(g as u32).collect::<Vec<_>>()),
            "{}",
            at("open members")
        );
    }
    assert_eq!(cv["in_no_group"], one(&cover.in_no_group()));
    assert_eq!(cv["components"], sets(&cover.components));
    assert_eq!(cv["cut"], Value::Null);
    let marks = cv["candidates"].as_array().expect("the candidates");
    let listed = d["candidates"].as_array().expect("H listed");
    assert_eq!((marks.len(), listed.len()), (l.n(), l.n()));
    for b in 0..l.n() {
        let mark = cover.marks[b].as_str();
        assert_eq!(marks[b]["at_root"], mark);
        assert_eq!(marks[b]["groups"], json!(cover.groups_of(b as u32)));
        // H listed: the same candidate, with what each object says of it.
        let row = &listed[b];
        assert_eq!(row["candidate"], text(b as u32));
        assert_eq!(row["at_root"], mark);
        assert_eq!(row["groups"], json!(cover.groups_of(b as u32)));
        assert_eq!(row["questions"], json!(c.question_of[b]));
        assert_eq!(
            row["in_an_owed_group"],
            cover.groups_of(b as u32).iter().any(|&g| x.is_owed(g))
        );
        let alive = cover.marks[b] == Mark::AliveAlone;
        assert_eq!(row["component"].is_null(), !alive, "{}", at("component"));
        if let Some(k) = c.component_of(b as u32) {
            assert_eq!(row["component"], k);
        }
    }

    // Exactly one.
    let xo = &d["exactly_one"];
    assert_eq!(xo["root_owes"], x.owed.root_owes);
    assert_eq!(xo["owed_groups"], json!(x.owed_groups()), "{}", at("owed"));
    assert_eq!(xo["groups_nothing_owes"], json!(x.unowed()));
    assert_eq!(xo["owed_sets_not_a_group"], json!(x.not_groups()));
    assert_eq!(
        xo["unlicensed"],
        json!(x.unlicensed()),
        "{}",
        at("unlicensed")
    );
    let instances = xo["instances"].as_array().expect("instances");
    let table = x.owed.table();
    assert_eq!(instances.len(), table.len());
    for ((i, owing), row) in table.iter().zip(&x.owing).zip(instances) {
        assert_eq!(row["instance"], i.text);
        assert_eq!(row["rule"], i.rule);
        assert_eq!(row["licensed"], i.alternatives.is_ok());
        assert_eq!(row["standing"], owing.as_str());
        match &i.alternatives {
            Ok(alts) => assert_eq!(row["alternatives"], one(alts)),
            Err(why) => assert_eq!(row["why"], why.as_str()),
        }
        if let Ok(ws) = &i.witnesses {
            assert_eq!(row["witnesses"].as_array().map(Vec::len), Some(ws.len()));
        }
    }
    let grids = xo["grids"].as_array().expect("grids");
    assert_eq!(grids.len(), x.grids.grids.len(), "{}", at("grids"));
    for (k, (g, row)) in x.grids.grids.iter().zip(grids).enumerate() {
        assert_eq!(row["component"], g.component);
        assert_eq!(row["bijection"], x.is_bijection(k as u32));
        for f in 0..2 {
            assert_eq!(row["families"][f]["groups"], json!(g.families[f]));
            assert_eq!(row["families"][f]["owed"], x.share(k as u32)[f].owed);
        }
    }
    assert_eq!(
        xo["components_that_are_no_grid"].as_array().map(Vec::len),
        Some(x.grids.not.len())
    );

    // The coupling: the questions by place, the edges of both readings.
    let cp = &d["coupling"];
    let questions = cp["questions"].as_array().expect("questions");
    assert_eq!(questions.len(), c.questions.len(), "{}", at("questions"));
    for (q, row) in c.questions.iter().zip(questions) {
        let members: Vec<u32> = match *q {
            Question::Group(g) => cover.members(g).to_vec(),
            Question::Alone(b) => vec![b],
        };
        let listed: Vec<&str> = row["members"]
            .as_array()
            .expect("members")
            .iter()
            .map(|m| m[0].as_str().expect("a text"))
            .collect();
        assert_eq!(listed, members.iter().map(|&b| text(b)).collect::<Vec<_>>());
        assert_eq!(
            row["is"] == "a group",
            matches!(q, Question::Group(_)),
            "{}",
            at("a question")
        );
    }
    let edges_held = |key: &str, edges: &[structure::Edge]| {
        let rows = cp[key].as_array().unwrap_or_else(|| panic!("{rel}: {key}"));
        assert_eq!(rows.len(), edges.len(), "{}", at(key));
        for (e, row) in edges.iter().zip(rows) {
            assert_eq!(row["question_ids"], json!(e.questions), "{}", at(key));
            assert_eq!(row["kind"], e.kind().as_str());
            assert_eq!(row["pairs"]["excludes"], e.pairs.excludes);
            assert_eq!(row["node"]["conflicts"], e.node.conflicts);
            assert_eq!(
                row["questions"].as_array().map(Vec::len),
                Some(e.questions.len())
            );
        }
    };
    edges_held("edges", &c.edges);
    edges_held(
        "edges_one_step_back",
        c.one_step
            .as_deref()
            .expect("the owner's reading keeps them"),
    );
    assert_eq!(cp["ties"].as_array().map(Vec::len), Some(c.ties.len()));
    assert_eq!(
        cp["components"],
        sets(&c.components),
        "{}",
        at("components")
    );
    assert_eq!(cp["conflict_components"], sets(&c.conflict_components));
    assert_eq!(
        cp["debts_across_conflict_components"],
        json!(c.debts_across())
    );
    let debts = cp["debts"].as_array().expect("debts");
    assert_eq!(debts.len(), c.debts.len(), "{}", at("debts"));
    for (debt, row) in c.debts.iter().zip(debts) {
        assert_eq!(row["instance"], debt.instance);
        assert_eq!(row["licensed"], debt.licensed, "{}", at("a debt's licence"));
        assert_eq!(row["payers"], one(&debt.payers));
        assert_eq!(row["question_ids"], json!(debt.questions));
    }

    // What it cost.
    let cost = &d["cost"];
    assert_eq!(cost["root_layer"]["saturations"], l.work.saturations);
    assert_eq!(cost["root_layer"]["facts_beyond_root"], l.work.facts);
    assert_eq!(
        cost["small_kb_layer"]["saturations"],
        small.work.saturations
    );
    assert_eq!(
        cost["small_kb_layer"]["facts_beyond_its_own_root"],
        small.work.facts
    );
    let node = c.node.expect("the whole node was read");
    assert_eq!(cost["whole_node"]["saturations"], node.saturations);
    assert_eq!(cost["whole_node"]["facts_beyond_root"], node.facts);
    assert_eq!(
        cost["saturations"],
        l.work.saturations + small.work.saturations + node.saturations
    );
}

/// **The three listings of `H` are one, on `rel`**: what `ein show hyps`
/// prints is the document's `candidates`, line for line in the universe's
/// order, and the candidates `ein solve -H` lists, which lists them by
/// relation and whole only as `--dump-answer`'s `forms/hyp-stats.txt`, of a
/// run that searches nothing. **And every line is a fact a file could
/// hold**: the program with the lines written after it loads, and holds each
/// line as the one printer writes that fact, which is how a state file's
/// facts are written.
fn the_three_listings_of_h_are_one(rel: &str, d: &Value) {
    let shown = ein(&["show", "hyps", rel]);
    assert_eq!(shown.code, 0, "{rel}: {}", shown.err);
    let lines: Vec<&str> = shown.out.lines().collect();
    let of_document: Vec<&str> = d["candidates"]
        .as_array()
        .expect("H listed")
        .iter()
        .map(|c| c["candidate"].as_str().expect("a text"))
        .collect();
    assert!(!lines.is_empty(), "{rel} proposes nothing any more");
    assert_eq!(lines, of_document, "{rel}: show hyps and the document");
    assert_eq!(
        shown.out.len(),
        lines.iter().map(|l| l.len() + 1).sum::<usize>(),
        "{rel}: stdout holds something beside the lines"
    );

    let tmp = Scratch::new(&format!("ein-show-hyps-{}", rel.replace(['/', '.'], "-")));
    let dir = tmp.path().join("answer");
    let solved = ein(&[
        "solve",
        rel,
        "-H",
        "--no-search",
        "--dump-answer",
        &dir.to_string_lossy(),
    ]);
    assert_eq!(solved.code, 0, "{rel}: {}", solved.err);
    let form = std::fs::read_to_string(dir.join("forms/hyp-stats.txt")).expect("-H's form");
    let mut listed: Vec<&str> = form
        .lines()
        .filter_map(|l| l.strip_prefix("    "))
        .map(|l| l.split("   dead alone").next().unwrap_or(l).trim_end())
        .collect();
    let mut sorted = lines.clone();
    listed.sort_unstable();
    sorted.sort_unstable();
    assert_eq!(listed, sorted, "{rel}: show hyps and solve -H");

    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the program");
    let state = format!("{text}\n{}", shown.out);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &state, path.to_str())
        .unwrap_or_else(|e| panic!("{rel}: the lines do not parse after the program: {e}"));
    let kb = load(&mut ast, &mut terms, &forms, path.parent())
        .unwrap_or_else(|e| panic!("{rel}: the program with H written after it: {e}"));
    let held: std::collections::BTreeSet<String> = kb.facts().map(|f| terms.fact_text(f)).collect();
    for line in &lines {
        assert!(
            held.contains(*line),
            "{rel}: {line} is no fact the file holds"
        );
    }
}

/// How many of `d`'s grids are bijections, and how many it has.
fn bijections(d: &Value) -> (usize, usize) {
    let grids = d["exactly_one"]["grids"].as_array().expect("grids");
    (
        grids.iter().filter(|g| g["bijection"] == true).count(),
        grids.len(),
    )
}

fn len(v: &Value) -> usize {
    v.as_array().map_or(0, Vec::len)
}

/// The one instance whose structure is read once here and not twice: its
/// two pair layers are more than twice the facts of any other instance's.
/// Its document is the library's, of the in-process reading; that the binary
/// prints the library's document is held on the other five, and is one code
/// path whatever the program.
const READ_ONCE: &str = "examples/zebra2-minus-15.ein";

/// **The view's document is the owner's reading, on every
/// instance.** Each object in the document is the owner's, list for list,
/// and the binary prints the library's document of an in-process reading,
/// byte for byte ([`READ_ONCE`] aside, whose document is the library's
/// alone).
#[test]
fn the_document_is_the_owners_reading_on_every_instance() {
    let read: Vec<(Value, Whole)> = std::thread::scope(|scope| {
        let handles: Vec<_> = INSTANCES
            .iter()
            .map(|rel| {
                scope.spawn(move || {
                    let (whole, document) = read(rel);
                    if *rel != READ_ONCE {
                        let r = ein(&["render", CMD, rel, "--view", "json"]);
                        assert_eq!(r.code, 0, "{rel}: {}", r.err);
                        assert!(
                            r.out == document,
                            "{rel}: the binary's document is not the library's"
                        );
                    }
                    let d: Value = serde_json::from_str(&document).expect("JSON");
                    held_to_the_owner(rel, &whole, &d);
                    the_three_listings_of_h_are_one(rel, &d);
                    (d, whole)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("an instance"))
            .collect()
    });
    let [z1, z2, z2m, b06, l02, send] = &read[..] else {
        panic!("six instances");
    };

    // The declared control: 36 groups, the rows and columns of five
    // relations, every one owed; five grids, each a bijection.
    let d = &z2.0;
    assert_eq!(len(&d["cover"]["groups"]), 36);
    assert_eq!(len(&d["exactly_one"]["owed_groups"]), 36);
    assert_eq!(bijections(d), (5, 5));
    assert_eq!(len(&d["coupling"]["components"]), 1);

    // One relation: five grids, one family of each owed and the other the
    // subject of the instances L5 does not license; no bijection.
    let d = &z1.0;
    assert_eq!(len(&d["cover"]["groups"]), 36);
    assert_eq!(len(&d["exactly_one"]["owed_groups"]), 18);
    assert_eq!(len(&d["exactly_one"]["unlicensed"]), 18);
    assert_eq!(bijections(d), (0, 5));
    let unlicensed = d["coupling"]["debts"]
        .as_array()
        .expect("debts")
        .iter()
        .filter(|x| x["licensed"] == false)
        .count();
    assert!(unlicensed > 0, "the unlicensed half's debts are edges");

    // The under-determined twin: its five bijections all the same.
    assert_eq!(bijections(&z2m.0), (5, 5));
    assert_eq!(
        len(&z2m.0["exactly_one"]["owed_groups"]),
        len(&z2m.0["cover"]["groups"])
    );

    // The blind enumerator's: groups, none with an open member, none owed.
    let d = &b06.0;
    assert!(len(&d["cover"]["groups"]) > 0);
    let open: usize = d["cover"]["groups"]
        .as_array()
        .expect("groups")
        .iter()
        .map(|g| len(&g["open"]))
        .sum();
    assert_eq!(open, 0, "every member of every group is dead alone at root");
    assert_eq!(len(&d["exactly_one"]["owed_groups"]), 0);
    assert_eq!(d["rung"], "blind");

    // The conflict of three: no pair, no group, one edge of its three
    // candidates, each a question of its own.
    let d = &l02.0;
    assert_eq!(len(&d["pairs"]["excluded"]), 0);
    assert_eq!(len(&d["cover"]["groups"]), 0);
    assert_eq!(len(&d["coupling"]["questions"]), 3);
    let edges = d["coupling"]["edges"].as_array().expect("edges");
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0]["question_ids"], json!([0, 1, 2]));
    assert_eq!(edges[0]["kind"], "a conflict or a derivation");

    // The cryptarithm: the letters' groups owed and the digits' not, one
    // grid that is no bijection, and an edge of three questions or more.
    let d = &send.0;
    let (owed, groups) = (
        len(&d["exactly_one"]["owed_groups"]),
        len(&d["cover"]["groups"]),
    );
    assert!(0 < owed && owed < groups, "{owed} of {groups} owed");
    assert_eq!(bijections(d), (0, 1));
    let wide = d["coupling"]["edges"]
        .as_array()
        .expect("edges")
        .iter()
        .filter(|e| len(&e["question_ids"]) >= 3)
        .count();
    assert!(wide > 0, "a column of the sum is an edge of its letters");
    for (_, w) in &read {
        assert!(shown::whole(w));
    }
}

/// **On the declared control the five declared bijections are found, and
/// shown beside what the program states.** The view reads no relation by
/// its name: beside each relation a candidate is of it lists the facts the
/// program states that the small KB keeps and that name the relation, a
/// statement of the relation whole. On `zebra2.ein` that is `(bijective R)`
/// for each of the five, and each is one grid, a bijection; which name means
/// *bijective* is this test's to know and no line of the view's.
#[test]
fn the_declared_control_shows_its_five_bijections_found_and_stated() {
    let d = document_of("examples/zebra2.ein", &[]);
    let relations = d["declared"]["relations"].as_array().expect("relations");
    let names: Vec<&str> = relations
        .iter()
        .map(|r| r["relation"].as_str().expect("a name"))
        .collect();
    assert_eq!(
        names,
        [
            "color-loc",
            "drink-loc",
            "nation-loc",
            "pet-loc",
            "smoke-loc"
        ]
    );
    let grids = d["exactly_one"]["grids"].as_array().expect("grids");
    let mut found: Vec<u64> = Vec::new();
    for r in relations {
        let name = r["relation"].as_str().expect("a name");
        let stated: Vec<&str> = r["stated_of_it"]
            .as_array()
            .expect("stated")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(
            stated.contains(&format!("(bijective {name})").as_str()),
            "{name}: {stated:?}"
        );
        assert_eq!(r["an_obligation_can_owe_it"], true, "{name}");
        assert_eq!(r["an_active_hrule_proposes_it"], true, "{name}");
        assert_eq!(r["candidates_in_no_group"], 0, "{name}");
        // One grid, a bijection, of this relation alone.
        let its: Vec<u64> = r["bijections"]
            .as_array()
            .expect("bijections")
            .iter()
            .filter_map(Value::as_u64)
            .collect();
        assert_eq!(r["grids"], json!(its), "{name}");
        let [k] = its[..] else {
            panic!("{name}: {its:?}");
        };
        assert_eq!(grids[k as usize]["relations"], json!([name]));
        assert_eq!(grids[k as usize]["kind"], "a bijection: every group owed");
        found.push(k);
    }
    found.sort_unstable();
    assert_eq!(found, [0, 1, 2, 3, 4], "five relations, five grids");
    // And the read-out counts them.
    let r = ein(&["render", CMD, "examples/zebra2.ein"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.contains("\ngrids        5 grids · 5 bijections"),
        "{}",
        r.out
    );
    assert!(r.out.contains("\nwhole        yes"), "{}", r.out);
}

/// Graphviz draws `text`, with nothing to say of it.
fn dot_accepts(what: &str, text: &str) {
    let mut child = Command::new("dot")
        .args(["-Tsvg", "-o", "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Graphviz `dot` on PATH — a hard dependency of the gate");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(text.as_bytes())
        .expect("dot reads its input");
    let out = child.wait_with_output().expect("dot runs");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success() && err.trim().is_empty(),
        "{what}: dot refused it or warned:\n{err}"
    );
}

/// **`--dump DIR` writes the document and the four graphs, and moves
/// nothing**: each file is what its `--view` prints, stdout and stderr are
/// the same with the option and without, and Graphviz draws each graph.
#[test]
fn the_dump_is_every_view_and_moves_no_stdout() {
    let tmp = Scratch::new("ein-h-space-view-dump");
    let dir = tmp.path().join("deep").join("z");
    let file = "examples/beyond_arith/send_more_money.ein";
    let plain = ein(&["render", CMD, file]);
    let with = ein(&["render", CMD, file, "--dump", &dir.to_string_lossy()]);
    assert_eq!((plain.code, with.code), (0, 0), "{}", with.err);
    assert_eq!(plain.out, with.out, "--dump moved stdout");
    assert_eq!(plain.err, with.err, "--dump moved stderr");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "coupling.dot",
            "coupling_one_step.dot",
            "cover.dot",
            "exclusion.dot",
            "h-space.json"
        ]
    );
    let held = |name: &str| std::fs::read_to_string(dir.join(name)).expect("a file");
    let document: Value = serde_json::from_str(&held(shown::DOCUMENT)).expect("JSON");
    for (view, name) in [
        ("json", shown::DOCUMENT.to_string()),
        ("exclusion", shown::graph_file("exclusion")),
        ("cover", shown::graph_file("cover")),
        ("coupling", shown::graph_file("coupling")),
        ("one-step", shown::graph_file("one-step")),
    ] {
        let r = ein(&["render", CMD, file, "--view", view]);
        assert_eq!(r.code, 0, "--view {view}: {}", r.err);
        assert!(r.out == held(&name), "--view {view} is not {name}");
        if view == "json" {
            continue;
        }
        assert!(r.out.starts_with("graph "), "--view {view}: {}", r.out);
        dot_accepts(&format!("{file} --view {view}"), &r.out);
        // The document says what the graph holds, a line a ` -- `.
        let row = document["graphs"]
            .as_array()
            .expect("graphs")
            .iter()
            .find(|g| g["graph"] == view)
            .unwrap_or_else(|| panic!("no row for {view}"));
        assert_eq!(row["file"], name.as_str());
        assert_eq!(row["lines"], r.out.matches(" -- ").count(), "{view}");
    }
    // The hypergraph names a question by its place: two may share a name.
    let coupling = held("coupling.dot");
    assert!(coupling.contains("q0 [label=\"q0 "), "{coupling}");
    // A path that cannot be made is exit 1 before the work.
    let blocked = tmp.path().join("a-file");
    std::fs::write(&blocked, "").expect("a file");
    let under = blocked.join("d");
    let r = ein(&["render", CMD, file, "--dump", &under.to_string_lossy()]);
    assert_eq!(r.code, 1, "{}", r.err);
    assert!(r.err.starts_with("error: --dump "), "{}", r.err);
    assert!(r.out.is_empty(), "it ran: {}", r.out);
    let r = ein(&["render", CMD, file, "--dump", ""]);
    assert_eq!(r.code, 2, "an empty path is a usage error: {}", r.err);
}

/// **Two questions of one name are two questions.** On the one-relation
/// zebra the kinds of attribute at one house are several groups under one
/// pattern, and the document and the graph tell them apart by place.
#[test]
fn a_question_is_named_by_its_place_as_well() {
    let d = document_of("examples/zebra.ein", &[]);
    let questions = d["coupling"]["questions"].as_array().expect("questions");
    let mut names: Vec<&str> = questions
        .iter()
        .map(|q| q["name"].as_str().expect("a name"))
        .collect();
    let all = names.len();
    names.sort_unstable();
    names.dedup();
    assert!(names.len() < all, "no two questions share a name any more");
    // An edge's ids are distinct where its names need not be.
    let mut ambiguous = 0;
    for e in d["coupling"]["edges"].as_array().expect("edges") {
        let ids = e["question_ids"].as_array().expect("ids");
        let mut distinct: Vec<u64> = ids.iter().filter_map(Value::as_u64).collect();
        distinct.dedup();
        assert_eq!(distinct.len(), ids.len());
        let mut by_name: Vec<&str> = e["questions"]
            .as_array()
            .expect("names")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        by_name.sort_unstable();
        by_name.dedup();
        ambiguous += usize::from(by_name.len() < ids.len());
    }
    assert!(ambiguous > 0, "no edge holds two questions of one name");
    let r = ein(&["render", CMD, "examples/zebra.ein", "--view", "coupling"]);
    assert_eq!(r.code, 0, "{}", r.err);
    for q in 0..questions.len() {
        assert!(r.out.contains(&format!("  q{q} [label=\"q{q} ")), "q{q}");
    }
}

/// **A budget that stops a layer exits 0 and says what was not read**, in
/// the document, the read-out and on stderr, and each graph's title; the
/// lists hold what was read and no more. A stopped structure is never
/// printed as a whole one.
#[test]
fn a_stopped_structure_says_what_was_not_read() {
    let file = "examples/zebra2.ein";
    let budget = ["--region-budget", "2000"];
    let mut args = vec!["render", CMD, file];
    args.extend(budget);
    let r = ein(&args);
    assert_eq!(r.code, 0, "a stopped structure is an answer: {}", r.err);
    assert!(r.out.contains("\nwhole        NO: "), "{}", r.out);
    assert!(
        r.out
            .contains("\nunread       root's pair layer stopped at the budget"),
        "{}",
        r.out
    );
    assert!(
        r.err.contains(&format!(
            "note: {file}: root's pair layer stopped at the budget"
        )),
        "{}",
        r.err
    );
    let d = document_of(file, &budget);
    assert_eq!(d["whole"], false);
    assert_eq!(d["asked"]["region_budget"], 2000);
    let unread: Vec<&str> = d["unread"]
        .as_array()
        .expect("unread")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        unread
            .iter()
            .any(|s| s.starts_with("root's pair layer stopped"))
    );
    // Every sentence is a note and a row.
    for sentence in &unread {
        assert!(r.err.contains(sentence), "no note for: {sentence}");
        assert!(r.out.contains(sentence), "no row for: {sentence}");
    }
    assert_eq!(d["pairs"]["complete"], false);
    assert_eq!(d["pairs"]["stopped"]["why"], "budget");
    let whole = document_of(file, &[]);
    let (left, all) = (
        d["pairs"]["pairs"]["left"].as_u64().expect("left"),
        whole["pairs"]["pairs"]["saturated"].as_u64().expect("all"),
    );
    assert!(0 < left && left <= all, "{left} of {all} left");
    assert!(
        len(&d["pairs"]["excluded"]) < len(&whole["pairs"]["excluded"]),
        "a stopped layer lists what it read"
    );
    let mut args = vec!["render", CMD, file, "--view", "exclusion"];
    args.extend(budget);
    let graph = ein(&args);
    assert_eq!(graph.code, 0, "{}", graph.err);
    assert!(graph.out.contains("NOT WHOLE"), "{}", graph.out);
    dot_accepts("a stopped exclusion graph", &graph.out);
    // A budget of nothing is a usage error, as `ein solve`'s.
    let r = ein(&["render", CMD, file, "--region-budget", "0"]);
    assert_eq!(r.code, 2, "{}", r.err);
}

/// **A layer that read its singles has its holders read, whatever it
/// spent.** Which candidates' own nodes hold each witness of an owed
/// instance is read after a layer, off its singles, which the layer left in
/// the cache: nothing is computed, so no budget is asked. A layer's last
/// step may carry it past its budget, so one fact under what root's layer
/// spends the layer reads every pair, having spent more than it was given;
/// the holders are read all the same, every debt of an instance L5 does not
/// license is an edge, and the structure is whole, the document the one of
/// the default budget but for what was asked. The same of the small KB's
/// layer one fact under its own spend: every unlicensed instance is about
/// the group it is about at the default budget.
///
/// Each budget is read off the program's own whole document, so the test
/// follows the layers' spend wherever the engine moves it.
#[test]
fn a_layer_read_whole_has_its_holders_read() {
    let file = "examples/zebra.ein";
    let whole = document_of(file, &[]);
    assert_eq!(whole["whole"], true);
    let debts = |d: &Value| d["coupling"]["debts"].as_array().expect("debts").clone();
    let unlicensed = debts(&whole)
        .iter()
        .filter(|x| x["licensed"] == false)
        .count();
    assert!(unlicensed > 0, "{file} has no unlicensed debt any more");

    // One fact under what root's layer spends.
    let spent = whole["pairs"]["work"]["facts"].as_u64().expect("facts");
    let budget = (spent - 1).to_string();
    let r = ein(&["render", CMD, file, "--region-budget", &budget]);
    assert_eq!(r.code, 0, "{}", r.err);
    let d = document_of(file, &["--region-budget", &budget]);
    assert_eq!(d["pairs"]["complete"], true, "root's layer read every pair");
    assert_eq!(d["pairs"]["stopped"], Value::Null);
    assert_eq!(d["pairs"]["pairs"]["left"], 0);
    assert_eq!(
        d["pairs"]["work"]["facts"], spent,
        "and spent more than it was given"
    );
    assert_eq!(d["exactly_one"]["singles_read"], true);
    assert_eq!(d["exactly_one"]["theory_read"], true);
    assert_eq!(d["coupling"]["payers_read"], true);
    assert_eq!(d["coupling"]["complete"], true);
    assert_eq!(d["whole"], true, "{}", d["unread"]);
    assert_eq!(debts(&d), debts(&whole), "every debt is an edge");
    for key in [
        "candidates",
        "pairs",
        "theory",
        "cover",
        "exactly_one",
        "coupling",
    ] {
        assert!(d[key] == whole[key], "{key} moved one fact under the spend");
    }
    assert!(r.out.contains("\nwhole        yes"), "{}", r.out);
    assert!(!r.err.contains("note:"), "{}", r.err);

    // One fact under what the small KB's layer spends. Root's layer, which
    // spends more, stops there; the small KB's reads every pair, and what
    // the theory alone brings of each witness is read.
    let small_spent = whole["theory"]["small_kb_layer"]["work"]["facts"]
        .as_u64()
        .expect("facts");
    assert!(small_spent < spent, "the small KB's layer costs root's now");
    let d = document_of(file, &["--region-budget", &(small_spent - 1).to_string()]);
    let small = &d["theory"]["small_kb_layer"];
    assert_eq!(
        small["complete"], true,
        "the small KB's layer read every pair"
    );
    assert_eq!(small["pairs"]["left"], 0);
    assert_eq!(small["work"]["facts"], small_spent);
    assert_eq!(d["exactly_one"]["theory_read"], true);
    assert_eq!(d["exactly_one"]["singles_read"], true);
    let about = |d: &Value| -> Vec<(Value, Value)> {
        d["exactly_one"]["instances"]
            .as_array()
            .expect("instances")
            .iter()
            .filter(|i| i["licensed"] == false)
            .map(|i| {
                (
                    i["about_group"].clone(),
                    i["pay_by_the_theory_alone"].clone(),
                )
            })
            .collect()
    };
    assert_eq!(about(&d).len(), unlicensed);
    assert!(about(&d).iter().all(|(g, p)| !g.is_null() && !p.is_null()));
    assert_eq!(about(&d), about(&whole));
    assert_eq!(d["pairs"]["complete"], false, "root's layer stopped");
    assert_eq!(d["whole"], false);
    let unread = d["unread"].as_array().expect("unread");
    assert!(
        unread
            .iter()
            .filter_map(Value::as_str)
            .all(|s| !s.starts_with("which candidates' own nodes")),
        "{unread:?}"
    );
}

/// **Where a layer stopped before its last single the holders are not read,
/// and nothing calls the structure whole.** At a budget of one fact root's
/// layer and the small KB's each stop among their singles. No witness has a
/// holder listed, an instance L5 does not license has no payers and is no
/// edge, and the owner's coupling says so itself: `payers_read` is false and
/// `complete` with it. The view says it in a sentence a layer, each a note
/// and a row, beside the sentence that says the layer stopped.
#[test]
fn holders_a_stopped_layer_left_unread_are_said() {
    let file = "examples/zebra.ein";
    let budget = ["--region-budget", "1"];
    let d = document_of(file, &budget);
    assert!(d["pairs"]["singles_left"].as_u64().is_some_and(|n| n > 0));
    assert_eq!(d["exactly_one"]["singles_read"], false);
    assert_eq!(d["exactly_one"]["theory_read"], false);
    assert_eq!(d["coupling"]["payers_read"], false);
    assert_eq!(d["coupling"]["complete"], false);
    assert_eq!(d["whole"], false);
    assert!(
        d["coupling"]["unlicensed_instances"]
            .as_u64()
            .is_some_and(|n| n > 0)
    );
    let debts = d["coupling"]["debts"].as_array().expect("debts");
    assert!(debts.iter().all(|x| x["licensed"] == true), "{debts:?}");
    let unread: Vec<&str> = d["unread"]
        .as_array()
        .expect("unread")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let mut args = vec!["render", CMD, file];
    args.extend(budget);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{}", r.err);
    for (place, layer) in [
        ("at root", "root's layer"),
        ("in the small KB", "its layer"),
    ] {
        let said = format!("was not read {place}: {layer} stopped before its last single");
        let [sentence] = unread
            .iter()
            .filter(|s| s.contains(&said))
            .collect::<Vec<_>>()[..]
        else {
            panic!("no one sentence says: {said}: {unread:?}");
        };
        assert!(r.err.contains(*sentence), "no note: {}", r.err);
        assert!(r.out.contains(*sentence), "no row: {}", r.out);
    }
    assert!(
        unread
            .iter()
            .any(|s| s.starts_with("root's pair layer stopped")),
        "{unread:?}"
    );
    assert!(r.out.contains("\nwhole        NO: "), "{}", r.out);
}

/// **A flagged program gets what the owner gives a flagged program, and the
/// document says what that is about**: every pair saturated, no licence, no
/// table of what root owes, and the three sentences that say nothing above
/// the nodes read follows.
#[test]
fn a_flagged_programs_document_says_what_it_is_about() {
    let file = "examples/ein-bugs/wrong-answers/core-activator-committed.ein";
    let d = document_of(file, &[]);
    assert_eq!(d["monotone"], false);
    assert_eq!(d["domain"]["pair_layer"], structure::FLAGGED);
    assert_eq!(d["domain"]["cover"], structure::COVER_FLAGGED);
    assert_eq!(d["domain"]["coupling"], structure::COUPLING_FLAGGED);
    assert_eq!(d["pairs"]["licences"], json!([]));
    assert_eq!(d["pairs"]["pairs"]["saturated"], d["pairs"]["pairs"]["all"]);
    assert_eq!(
        d["exactly_one"]["no_table"],
        "the static check flags the program"
    );
    assert_eq!(d["whole"], true, "flagged is no cut: {}", d["unread"]);
    let r = ein(&["render", CMD, file]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out
            .contains(&format!("\ndomain       {}", structure::FLAGGED)),
        "{}",
        r.out
    );
}

/// **A program with no candidate, and one with a dead root, say so**: each
/// exits 0 with a document, and the graphs hold their titles alone.
#[test]
fn a_program_that_does_not_search_says_so() {
    let file = "examples/features/06_symmetric_native.ein";
    let d = document_of(file, &[]);
    assert_eq!(d["n"], 0);
    assert_eq!(d["whole"], true);
    assert!(
        d["says"][0]
            .as_str()
            .is_some_and(|s| s.starts_with("H is empty")),
        "{}",
        d["says"]
    );
    assert_eq!(len(&d["candidates"]), 0);
    assert_eq!(len(&d["coupling"]["questions"]), 0);
    let r = ein(&["render", CMD, file]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("\nsays         H is empty"), "{}", r.out);
    for view in shown::GRAPHS {
        let g = ein(&["render", CMD, file, "--view", view]);
        assert_eq!(g.code, 0, "{}", g.err);
        assert_eq!(g.out.matches(" -- ").count(), 0);
        dot_accepts(&format!("{file} --view {view}"), &g.out);
    }

    let dead = "examples/ein-bugs/wrong-answers/or-one-key.ein";
    let d = document_of(dead, &[]);
    assert_eq!(d["root_dead"], true);
    assert!(d["n"].as_u64().is_some_and(|n| n > 0));
    assert_eq!(
        len(&d["pairs"]["dead_alone"]) as u64,
        d["n"].as_u64().unwrap()
    );
    assert!(
        d["says"][0]
            .as_str()
            .is_some_and(|s| s.starts_with("root's own fixpoint derives (false)")),
        "{}",
        d["says"]
    );
    assert_eq!(d["whole"], true, "a dead root is no cut: {}", d["unread"]);
    assert_eq!(d["cost"]["whole_node"], Value::Null);
}

/// **`--supports` names what each pair of the puzzle rests on**: every pair
/// root excludes gets its support, and a pair of the puzzle is labelled in
/// the exclusion graph with the places of the statements the small KB drops
/// that its support names. Without the option no pair has one, and the
/// groups, the grids and the components are the same either way.
#[test]
fn supports_name_the_statements_a_puzzles_pair_rests_on() {
    let file = "examples/binary_logic/knights.ein";
    let (bare, with) = (document_of(file, &[]), document_of(file, &["--supports"]));
    assert_eq!(with["asked"]["supports"], true);
    assert_eq!(with["whole"], true, "{}", with["unread"]);
    let stated = with["theory"]["stated"].as_array().expect("stated");
    let pairs = with["theory"]["root_excluded"].as_array().expect("pairs");
    assert!(!pairs.is_empty());
    let mut puzzles = 0;
    for p in pairs {
        let support = &p["support"];
        assert_eq!(support["read"], true, "{p}");
        if p["of_the_theory"] == false {
            puzzles += 1;
            assert!(
                len(&support["dropped"]) > 0,
                "a puzzle's pair names a clue: {p}"
            );
        }
    }
    assert!(puzzles > 0, "{file} has no pair of the puzzle any more");
    assert_eq!(
        with["cost"]["supports"]["saturations"],
        pairs.len(),
        "a support is its node saturated again"
    );
    for (a, b) in bare["theory"]["root_excluded"]
        .as_array()
        .expect("pairs")
        .iter()
        .zip(pairs)
    {
        assert_eq!(a["support"], Value::Null);
        assert_eq!(a["pair"], b["pair"]);
        assert_eq!(a["of_the_theory"], b["of_the_theory"]);
    }
    for key in ["cover", "exactly_one", "pairs"] {
        assert_eq!(bare[key], with[key], "--supports moved {key}");
    }
    assert_eq!(
        bare["coupling"]["components"],
        with["coupling"]["components"]
    );
    // The graph's labels are places in `theory.stated`, each a dropped fact.
    let g = ein(&["render", CMD, file, "--view", "exclusion", "--supports"]);
    assert_eq!(g.code, 0, "{}", g.err);
    dot_accepts("a labelled exclusion graph", &g.out);
    let mut labelled = 0;
    for line in g.out.lines().filter(|l| l.contains("label=\"s")) {
        labelled += 1;
        let label = line
            .split("label=\"")
            .nth(1)
            .and_then(|l| l.split('"').next());
        for id in label.expect("a label").split([' ', '\\', 'n']) {
            let Some(k) = id.strip_prefix('s').and_then(|k| k.parse::<usize>().ok()) else {
                continue;
            };
            assert_eq!(
                stated[k]["kept"], false,
                "s{k} is a fact the small KB keeps"
            );
        }
    }
    assert_eq!(labelled, puzzles, "every pair of the puzzle is labelled");
}

/// **A job count moves no byte, and a `.einb` gives its `.ein`'s
/// structure**: the document is the same at `--jobs 4`, and the same of a
/// container but for the path it echoes.
#[test]
fn jobs_and_the_container_move_no_byte() {
    let file = "examples/beyond_arith/send_more_money.ein";
    let one = ein(&["render", CMD, file, "--view", "json"]);
    let four = ein(&["render", CMD, file, "--view", "json", "--jobs", "4"]);
    assert_eq!((one.code, four.code), (0, 0), "{}", four.err);
    assert!(one.out == four.out, "--jobs 4 moved the document");

    if !cfg!(feature = "einb") {
        return;
    }
    let tmp = Scratch::new("ein-h-space-view-einb");
    let einb = tmp.path().join("send.einb");
    let einb = einb.to_string_lossy().into_owned();
    let saved = ein(&["kb", "save", file, &einb]);
    assert_eq!(saved.code, 0, "{}", saved.err);
    let held = ein(&["render", CMD, &einb, "--view", "json"]);
    assert_eq!(held.code, 0, "{}", held.err);
    assert!(
        held.out.replace(&einb, file) == one.out,
        "the container's document is not the program's"
    );
}

fn page() -> String {
    std::fs::read_to_string(repo_root().join("docs/ein/reasoning/kb-evolution/hypothesis-space.md"))
        .expect("docs/ein/reasoning/kb-evolution/hypothesis-space.md")
}

/// **The page's read-out is the binary's**: the block
/// `docs/ein/reasoning/kb-evolution/hypothesis-space.md` shows of the declared control
/// is what `ein render h-space examples/zebra2.ein` prints, so every number
/// in it is taken again by this test. A read-out that moves fails here with
/// the block to paste.
#[test]
fn the_pages_read_out_is_the_binarys() {
    const BEGIN: &str = "<!-- generated: ein render h-space examples/zebra2.ein -->\n```text\n";
    const END: &str = "\n```\n<!-- /generated -->";
    let text = page();
    let start = text.find(BEGIN).expect("the page's generated block") + BEGIN.len();
    let end = start + text[start..].find(END).expect("the block's end");
    let r = ein(&["render", CMD, "examples/zebra2.ein"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        text[start..end] == *r.out.trim_end_matches('\n'),
        "docs/ein/reasoning/kb-evolution/hypothesis-space.md's read-out is not the binary's. Paste:\n\n{}",
        r.out
    );
}

/// Every object key of `v`, at any depth.
fn keys(v: &Value, out: &mut std::collections::BTreeSet<String>) {
    match v {
        Value::Object(m) => {
            for (k, x) in m {
                out.insert(k.clone());
                keys(x, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|x| keys(x, out)),
        _ => {}
    }
}

/// **The page names every key a document holds.** Six documents of six
/// kinds, the declared control, one relation with its unlicensed half, a
/// program read with its supports, the blind enumerator's, a flagged one and
/// a stopped one: every key any of them holds, at any depth, is in the
/// page's account of `h-space.json`, in backticks.
///
/// One direction only. A key the writer has for a case none of these
/// reaches, a group an instance is inside, a component with a member in more
/// than two groups, is in the page by reading the writer and is held by
/// nothing here.
#[test]
fn the_page_names_every_key_a_document_holds() {
    let text = page();
    let mut held = std::collections::BTreeSet::new();
    for (file, more) in [
        ("examples/zebra2.ein", &[][..]),
        ("examples/zebra.ein", &[]),
        ("examples/binary_logic/knights.ein", &["--supports"]),
        ("examples/branching/06_lookahead_on.ein", &[]),
        (
            "examples/ein-bugs/wrong-answers/core-activator-committed.ein",
            &[],
        ),
        ("examples/zebra2.ein", &["--region-budget", "2000"]),
    ] {
        keys(&document_of(file, more), &mut held);
    }
    assert!(held.len() >= 150, "only {} keys read", held.len());
    let missing: Vec<&String> = held
        .iter()
        .filter(|k| !text.contains(&format!("`{k}`")))
        .collect();
    assert!(
        missing.is_empty(),
        "docs/ein/reasoning/kb-evolution/hypothesis-space.md does not name these keys of \
         h-space.json: {missing:?}"
    );
}

/// **A program the loader refuses is refused**, with the line and the exit
/// every other command gives it, and nothing on stdout; and a file that is
/// not there likewise.
#[test]
fn a_refused_program_has_no_h_space() {
    let file = "examples/broken/load/derivation_cycle.ein";
    let refusal = ein(&["saturate", file]);
    assert_eq!(refusal.code, 1, "{file} loads now");
    let line = refusal.err.lines().next().expect("a line").to_string();
    for view in ["summary", "json", "coupling"] {
        let r = ein(&["render", CMD, file, "--view", view]);
        assert_eq!(r.code, 1, "--view {view}: {}", r.err);
        assert_eq!(r.err.lines().next(), Some(line.as_str()));
        assert!(r.out.is_empty(), "{}", r.out);
    }
    let r = ein(&["render", CMD, "examples/no-such-file.ein"]);
    assert_eq!(r.code, 1, "{}", r.err);
    assert!(r.out.is_empty());
    let r = ein(&["render", CMD, "examples/zebra2.ein", "--view", "table"]);
    assert_eq!(r.code, 2, "an unknown view is a usage error: {}", r.err);
    assert!(Path::new(&repo_root().join(file)).is_file());
}
