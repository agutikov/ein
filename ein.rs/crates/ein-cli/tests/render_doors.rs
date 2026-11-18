//! **Every renderer has a door** (Q-M1e.20): a renderer built and tested is
//! reachable from a command, for the reviewer of a wrong answer if for no
//! one else. `ein_render::kb_dot`, `ein_render::ir_dot` and the visited
//! nodes' lattice:
//!
//! ```text
//! ein render program FILE [--fold] [--rule-mode …]          the program as written
//! ein render kb FILE [--since FILE] [--colour-by …] [--fold] a KB, one digraph
//!   [--goal GOAL | --query N]                                or a goal's slice of it
//! ein solve FILE --dump-lattice DIR …                        every node the run visited
//! ein render lattice FILE [--view full]                      those nodes drawn
//! ```
//!
//! The third door is a region run's visited nodes (`ein-lattice/2`,
//! [`06-lattice-dump.md`](../../../../docs/ein/ein.rs/06-lattice-dump.md)), and
//! `ein render lattice` draws them.
//!
//! The renderers' own standard is `ein-render/tests/dot_wellformed.rs`: what
//! they emit, `dot` accepts. This extends it from the renderer to the door —
//! the bytes the *command* prints go through `dot -Tsvg` — and holds the
//! one claim each door adds of its own.

use std::io::Write;
use std::process::{Command, Stdio};

use ein_corpus::{Scratch, repo_root};

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

fn scratch(tag: &str) -> Scratch {
    Scratch::new(&format!("ein-render-doors-{tag}"))
}

/// `dot -Tsvg` over the text, as `dot_wellformed.rs` does: Graphviz is a hard
/// dependency of the gate, so its absence fails rather than skips.
fn dot_accepts(what: &str, text: &str) {
    let mut child = Command::new("dot")
        .arg("-Tsvg")
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
    assert!(
        out.status.success(),
        "{what}: dot refused it:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn every_door_prints_dot_that_dot_draws() {
    let dir = scratch("dot");
    let lattice = dir.join("lattice");
    for (what, args) in [
        (
            "render program",
            vec!["render", "program", "examples/zebra2.ein"],
        ),
        (
            "render program --fold",
            vec!["render", "program", "examples/zebra2.ein", "--fold"],
        ),
        ("render kb", vec!["render", "kb", "examples/zebra2.ein"]),
        (
            "render kb --fold",
            vec!["render", "kb", "examples/zebra2.ein", "--fold"],
        ),
        (
            "render kb --colour-by origin",
            vec![
                "render",
                "kb",
                "examples/zebra2.ein",
                "--colour-by",
                "origin",
            ],
        ),
    ] {
        let r = ein(&args);
        assert_eq!(r.code, 0, "{what}: {}", r.err);
        assert!(r.out.contains("digraph"), "{what} printed no digraph");
        dot_accepts(what, &r.out);
    }
    // The third door writes files, and the fourth draws the same nodes.
    for view in ["solution", "full"] {
        let r = ein(&[
            "render",
            "lattice",
            "examples/ein-bugs/wrong-answers/owed-seats.ein",
            "--view",
            view,
        ]);
        assert_eq!(r.code, 0, "render lattice --view {view}: {}", r.err);
        dot_accepts(&format!("render lattice --view {view}"), &r.out);
    }
    let r = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-n",
        "1",
        "--dump-lattice",
        &lattice.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "--dump-lattice: {}", r.err);
    assert!(lattice.join("nodes.json").is_file());
    assert!(lattice.join("nodes.md").is_file());
    // The hypothesis space's door prints one undirected graph a view; what each
    // holds is `h_space_view.rs`'s.
    for view in ["exclusion", "cover", "coupling", "one-step"] {
        let r = ein(&[
            "render",
            "h-space",
            "examples/lattice/02_genuine_3set_death.ein",
            "--view",
            view,
        ]);
        assert_eq!(r.code, 0, "render h-space --view {view}: {}", r.err);
        assert!(
            r.out.starts_with("graph "),
            "render h-space --view {view} printed no graph"
        );
        dot_accepts(&format!("render h-space --view {view}"), &r.out);
    }
}

/// **A program the loader refuses is refused by every view**, with the line
/// and the exit every other command gives it (Q-M1f.71).
///
/// A view that drew a program before loading it would give a refused program
/// a picture, a view answering a program every other command refuses. The
/// three views that draw a program as written read it through the loader
/// first, and draw a program it accepts as written; the constraint view draws
/// what it loads.
#[test]
fn every_view_refuses_what_the_loader_refuses() {
    let file = "examples/broken/load/derivation_cycle.ein";
    let refusal = ein(&["saturate", file]);
    assert_eq!(refusal.code, 1, "{file} loads now");
    let line = refusal.err.lines().next().expect("a line").to_string();
    for view in [
        &["render", "program", file][..],
        &["render", "rules", file],
        &["render", "rule", "--name", "r", file],
        &["render", "constraints", file],
        &["render", "h-space", file],
    ] {
        let r = ein(view);
        assert_eq!(r.code, 1, "{view:?}: {}", r.err);
        assert_eq!(r.err.lines().next(), Some(line.as_str()), "{view:?}");
        assert!(
            r.out.is_empty(),
            "{view:?} drew a refused program:\n{}",
            r.out
        );
    }
    // A program the loader accepts draws as before.
    let ok = "examples/features/01_not_and_absent.ein";
    let r = ein(&["render", "program", ok]);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts(ok, &r.out);
}

/// **`--since` draws what a commitment added**, from the files `--dump-answer`
/// wrote: a state against ∅'s, one `Terms` for both, so a fact ∅ also held is
/// drawn plain and one it lacked is drawn at `penwidth=3`. ∅'s file is
/// `models/01.ein` where ∅ is in R.
#[test]
fn render_kb_since_root_emboldens_what_the_commitment_added() {
    let tmp = scratch("since");
    let dir = tmp.join("answer");
    let file = "examples/features/11_expect_ambiguity.ein";
    let r = ein(&["solve", file, "--dump-answer", &dir.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    // State 1 is ∅; state 6 seats both guests.
    let root = dir.join("models/01.ein");
    let model = dir.join("models/06.ein");
    let plain = ein(&["render", "kb", &model.to_string_lossy()]);
    assert_eq!(plain.code, 0, "{}", plain.err);
    assert!(
        !plain.out.contains("penwidth=3"),
        "nothing to embolden without --since"
    );
    let r = ein(&[
        "render",
        "kb",
        &model.to_string_lossy(),
        "--since",
        &root.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts("render kb --since", &r.out);
    let bold: Vec<&str> = r.out.lines().filter(|l| l.contains("penwidth=3")).collect();
    // State 6 seats two guests: two facts of `seat` ∅ did not have, each
    // drawn bold with its first arrow at the relation, written back from it.
    assert_eq!(
        bold.iter()
            .filter(|l| l.contains("\"seat\" -> ") && l.contains("[xlabel=\"1\", dir=back"))
            .count(),
        2,
        "the bold facts are not the commitment's:\n{}",
        bold.join("\n")
    );
}

/// The octagons a drawing declares, each a fact's node.
fn octagons(dot: &str) -> usize {
    dot.lines().filter(|l| l.contains("[shape=octagon")).count()
}

/// The facts of `relation` a drawing draws as octagons: the arrows `1` at its
/// box.
fn facts_of(dot: &str, relation: &str) -> usize {
    let head = format!("  \"{relation}\" -> f_");
    dot.lines()
        .filter(|l| l.starts_with(&head) && l.contains("[xlabel=\"1\", dir=back"))
        .count()
}

/// **A goal's slice is drawn by `render kb`** (Q-M1f.529): under `--goal` or
/// `--query N`, the facts of the knowledge base drawn that answer the goal,
/// with what they point at, a fact it does not hold in its frame; of root, or
/// of a saved state, against which the goal is matched and never against R.
#[test]
fn render_kb_draws_a_goals_slice() {
    // At root: the membership fact and the fact of `symmetric` the goal's one
    // row names, of the 390 root holds.
    let r = ein(&[
        "render",
        "kb",
        "examples/zebra2.ein",
        "--goal",
        "(and (relation ?r) (symmetric ?r))",
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.starts_with("digraph goal {"), "{}", r.out);
    assert_eq!(octagons(&r.out), 2, "{}", r.out);
    assert_eq!(facts_of(&r.out, "symmetric"), 1, "{}", r.out);
    assert_eq!(facts_of(&r.out, "relation"), 1, "{}", r.out);
    dot_accepts("render kb --goal", &r.out);
    // A fact under an arrow is drawn with the facts that point at it: held,
    // in no frame; held nowhere, in the frame of what points at it.
    let beliefs = [
        "render",
        "kb",
        "examples/beliefs/01_common_belief.ein",
        "--goal",
        "(believes ?who (likes Dan ?food))",
    ];
    let r = ein(&beliefs);
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(facts_of(&r.out, "believes"), 5, "{}", r.out);
    assert_eq!(facts_of(&r.out, "likes"), 2, "{}", r.out);
    assert_eq!(
        r.out.matches("label=\"believes ctx\"").count(),
        1,
        "{}",
        r.out
    );
    let mut folded = beliefs.to_vec();
    folded.push("--fold");
    let r = ein(&folded);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts("render kb --goal --fold", &r.out);
    // The program's own query, of the state it answers from: zebra2's at root
    // has nothing to answer, and of the saved solution its four facts.
    let r = ein(&["render", "kb", "examples/zebra2.ein", "--query", "1"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(octagons(&r.out), 0, "{}", r.out);
    assert!(r.out.contains("// the slice keeps no fact"), "{}", r.out);
    let tmp = scratch("goal");
    let dir = tmp.join("answer");
    let solved = ein(&[
        "solve",
        "examples/zebra2.ein",
        "--dump-answer",
        &dir.to_string_lossy(),
    ]);
    assert_eq!(solved.code, 0, "{}", solved.err);
    let state = dir.join("models/01.ein");
    let r = ein(&["render", "kb", &state.to_string_lossy(), "--query", "1"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(octagons(&r.out), 4, "{}", r.out);
    assert_eq!(facts_of(&r.out, "nation-loc"), 2, "{}", r.out);
    assert_eq!(facts_of(&r.out, "drink-loc"), 1, "{}", r.out);
    assert_eq!(facts_of(&r.out, "pet-loc"), 1, "{}", r.out);
    dot_accepts("render kb STATE --query 1", &r.out);
}

/// **A goal given beside the program is refused as a query's would be**
/// (Q-M1f.529), with the query's line and the views' exit, 1, and nothing
/// drawn; a query the program has not is refused the same way, and the two
/// options together are a usage error, exit 2.
#[test]
fn render_kb_refuses_a_goal_a_query_could_not_ask() {
    let file = "examples/zebra2.ein";
    for (goal, first) in [
        ("(payed ?x)", "kb load error: --goal:1:"),
        (
            "(?r Red ?h)",
            "kb load error: query: its :goal writes (?r Red ?h)",
        ),
        ("(p ?x", "--goal:1:"),
        ("(p ?x)) (q", "kb load error: --goal: "),
        (
            "(color-loc ?c ?h) :expect (model)",
            "kb load error: --goal: ",
        ),
    ] {
        let r = ein(&["render", "kb", file, "--goal", goal]);
        assert_eq!(r.code, 1, "{goal}: {}", r.err);
        assert!(r.err.starts_with(first), "{goal}: {}", r.err);
        assert!(r.out.is_empty(), "{goal} drew:\n{}", r.out);
    }
    let r = ein(&["render", "kb", file, "--query", "2"]);
    assert_eq!(r.code, 1, "{}", r.err);
    assert_eq!(
        r.err.lines().next(),
        Some("--query 2: examples/zebra2.ein has 1 (query …) form")
    );
    let r = ein(&[
        "render",
        "kb",
        file,
        "--query",
        "1",
        "--goal",
        "(color-loc ?c ?h)",
    ]);
    assert_eq!(r.code, 2, "{}", r.err);
}

/// **`render constraints` draws the facts about relations of root**
/// (Q-M1f.530): of zebra2's root at its fixpoint, the declarations and the
/// facts the loader adds beside them, and every fact of a property, given or
/// derived, each an octagon with its relation under an argument arrow; and
/// no written check, which no knowledge base holds (Q-M1f.531).
#[test]
fn render_constraints_draws_roots_facts_about_relations() {
    let r = ein(&["render", "constraints", "examples/zebra2.ein"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.starts_with("digraph constraints {"), "{}", r.out);
    assert_eq!(facts_of(&r.out, "bijective"), 5, "{}", r.out);
    assert_eq!(facts_of(&r.out, "symmetric"), 1, "{}", r.out);
    assert!(facts_of(&r.out, "relation") >= 18, "{}", r.out);
    // A fact of a relation the program states, about no relation, is not kept.
    assert_eq!(facts_of(&r.out, "color-loc"), 0, "{}", r.out);
    for file in [
        "examples/ein-bugs/wrong-answers/fact-eq-false.ein",
        "examples/ein-bugs/wrong-answers/not-eq-fact.ein",
    ] {
        let r = ein(&["render", "constraints", file]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        assert_eq!(facts_of(&r.out, "eq"), 0, "{file}: {}", r.out);
        assert_eq!(facts_of(&r.out, "not"), 0, "{file}: {}", r.out);
        assert_eq!(facts_of(&r.out, "relation"), 2, "{file}: {}", r.out);
    }
}

/// **One program in both forms, through every door that draws facts**
/// (Q-M1f.419): by default a fact of three arrows is an octagon with no name
/// in it, its arrow `1` at the relation's atom, and under `--fold` it is one
/// arrow from its first argument to its second, labelled with the relation
/// (Q-M1f.425). The trace's drawings take the option too, and in a step's
/// drawing no fact its derivation's arrows touch is folded (Q-M1f.429).
#[test]
fn every_door_draws_a_fact_in_both_forms() {
    let tmp = scratch("fold");
    let file = "examples/zebra2.ein";
    // A folded arrow: an edge with its relation's name on it, its `label` in a
    // rule's panel and its `xlabel` in a knowledge base, where a number is an
    // `xlabel` too; a guard's link, undirected, carries its glyph.
    let folded = |out: &str| {
        out.lines().any(|l| {
            l.contains(" -> ")
                && !l.contains("dir=none")
                && (l.contains("[label=\"")
                    || l.split("[xlabel=\"")
                        .nth(1)
                        .is_some_and(|rest| !rest.starts_with(|c: char| c.is_ascii_digit())))
        })
    };
    for door in [
        &["render", "program", file][..],
        &["render", "kb", file],
        &["render", "rules", file],
        &["render", "rule", "--name", "co-located", file],
    ] {
        let canonical = ein(door);
        assert_eq!(canonical.code, 0, "{door:?}: {}", canonical.err);
        assert!(
            canonical.out.contains("[xlabel=\"1\""),
            "{door:?}: no first arrow at a relation:\n{}",
            canonical.out
        );
        assert!(!folded(&canonical.out), "{door:?}: folded by default");
        assert!(!canonical.out.contains("label=\"#"), "{door:?}");
        let mut with = door.to_vec();
        with.push("--fold");
        let fold = ein(&with);
        assert_eq!(fold.code, 0, "{with:?}: {}", fold.err);
        assert!(folded(&fold.out), "{with:?}: nothing folded:\n{}", fold.out);
        dot_accepts(&format!("{with:?}"), &fold.out);
    }
    let r = ein(&["render", "constraints", "--fold", file]);
    assert_eq!(r.code, 0, "{}", r.err);
    dot_accepts("render constraints --fold", &r.out);
    // The trace: each step's drawing and each state's knowledge base.
    let trace = |extra: &[&str]| -> String {
        let out = tmp.join(format!("trace{}.md", extra.len()));
        let mut argv = vec!["solve", "examples/features/11_expect_ambiguity.ein", "-F"];
        let out_s = out.to_string_lossy().into_owned();
        argv.extend(["--trace", out_s.as_str()]);
        argv.extend(extra);
        let r = ein(&argv);
        assert_eq!(r.code, 0, "{argv:?}: {}", r.err);
        std::fs::read_to_string(&out).expect("the trace is written")
    };
    let (canonical, fold) = (trace(&[]), trace(&["--fold"]));
    let seat = |md: &str| {
        md.lines()
            .any(|l| l.contains(" -> ") && (l.contains("label=\"seat")))
    };
    assert!(!seat(&canonical), "the trace folds by default");
    assert!(seat(&fold), "--fold folds nothing in the trace");
    // A step's own drawing: a fact a firing reads or derives is the end of
    // the derivation's arrows, and an arrow goes only to or from a node, so
    // it is its octagon under `--fold` too, and the step is drawn as it is
    // by default.
    let packing = |extra: &[&str]| -> String {
        let out = tmp.join(format!("packing{}.md", extra.len()));
        let out_s = out.to_string_lossy().into_owned();
        let mut argv = vec![
            "solve",
            "examples/beliefs/02_packing.ein",
            "--trace",
            out_s.as_str(),
        ];
        argv.extend(extra);
        let r = ein(&argv);
        assert_eq!(r.code, 0, "{argv:?}: {}", r.err);
        std::fs::read_to_string(&out).expect("the trace is written")
    };
    let blocks = |md: &str| -> Vec<String> {
        md.split("```dot\n")
            .skip(1)
            .filter_map(|b| b.split_once("```").map(|(d, _)| d.to_string()))
            .collect()
    };
    let (canonical, fold) = (blocks(&packing(&[])), blocks(&packing(&["--fold"])));
    assert!(!canonical.is_empty(), "no step drawn");
    assert_eq!(
        canonical, fold,
        "a step's drawing folds a fact its derivation touches"
    );
    assert!(
        fold.iter().all(|b| !b.contains("shape=point")),
        "a point where an arrow meets an arrow"
    );
}

/// **The dump is the run's nodes.** `nodes.json` lists, in visit order,
/// exactly the nodes the run's `--events` stream enters, with the kind each
/// was classified as — so the drawing and the stream are one account of the
/// run. The walk's door was held to the library's `LatticeDumper` tree file
/// for file; that tree has no CLI caller since the switch.
#[test]
fn dump_lattice_lists_the_nodes_the_run_entered() {
    let dir = scratch("lattice");
    let d = dir.join("d");
    let ev = dir.join("e.jsonl");
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let r = ein(&[
        "solve",
        file,
        "--dump-lattice",
        &d.to_string_lossy(),
        "--events",
        &ev.to_string_lossy(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let nodes: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(d.join("nodes.json")).unwrap()).unwrap();
    assert_eq!(nodes["schema"], "ein-lattice/2");
    let entered = std::fs::read_to_string(&ev)
        .unwrap()
        .lines()
        .filter(|l| l.contains("\"e\": \"enter\""))
        .count();
    assert_eq!(
        nodes["visited"], entered as u64,
        "the dump and the stream disagree"
    );
    assert_eq!(nodes["nodes"].as_array().unwrap().len(), entered);
    let in_r = nodes["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| n["kind"] == "in-r")
        .count();
    assert_eq!(in_r, 7, "owed-seats' R is seven states, each visited once");
}

/// **The dump moves no answer, beside any other flag.** The walk had one
/// dumper per solve and refused `--dump-lattice` with `-v`, `--layer-progress`,
/// `-t` and `--dump-states`; a region run's dump is a reader of the run, so
/// it combines with each and stdout is the same with it and without.
/// `--dump-states` is retired, and says what replaced it.
#[test]
fn the_dump_combines_with_the_other_flags() {
    let dir = scratch("combine");
    let d = dir.join("d").to_string_lossy().into_owned();
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    for other in [vec!["-v"], vec!["--layer-progress"], vec!["-s"]] {
        let mut with = vec!["solve", file, "--dump-lattice", &d];
        with.extend(other.iter().copied());
        let mut without = vec!["solve", file];
        without.extend(other.iter().copied());
        let (a, b) = (ein(&with), ein(&without));
        assert_eq!((a.code, b.code), (0, 0), "{other:?}: {}", a.err);
        let steady = |o: &str| {
            o.lines()
                .filter(|l| !l.trim_start().starts_with("wall"))
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(steady(&a.out), steady(&b.out), "{other:?} moved stdout");
        assert_eq!(a.err, b.err, "{other:?} moved stderr");
    }
    let r = ein(&["solve", file, "-t", "--dump-lattice", &d]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("\ntiming\n"), "the timing table is gone");
    let r = ein(&["solve", file, "--dump-states", &d]);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(
        r.err.contains("-D/--dump-states is retired") && r.err.contains("--dump-lattice DIR"),
        "{}",
        r.err
    );
    assert!(r.out.is_empty(), "a retired flag ran anyway");
}
