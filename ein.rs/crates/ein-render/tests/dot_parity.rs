//! S1a.5.1 acceptance — **T3 on the DOT surface**: every renderer, every
//! corpus entry, byte for byte.
//!
//! Seventeen views per file (`ein-render`'s `dot_shape` and
//! `utils/ir_oracle.py`'s `dot-shape` op enumerate the same names), covering
//! the per-form IR renderer in all four of its modes, the rule library in both
//! rule modes, the constraint scopes, `kb.to_dot`'s whole keyword surface, the
//! commitment lattice in both views, and the per-commitment provenance cones.
//!
//! DOT is unforgiving in the useful way: one differing attribute is a diff. So
//! a failure names its line rather than its file — and when a *slice* or
//! *lattice* view fails, check the T1/T2 status of that corpus entry first,
//! because every renderer takes its data from the engine and a DOT diff can be
//! a search-layer bug surfacing late.

use ein_core::Terms;
use ein_ir::{Ast, parse};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use ein_render::shape::{all_views, dot_shape};
use std::path::Path;

/// The repo-relative paths where ein.py is **expected** to raise and ein.rs to
/// answer — [D2](../../../../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
/// reached through every view that runs the search. Asserted, not tolerated:
/// a file listed here that stops diverging fails as loudly as one that starts.
/// The entries whose `slice` view is a **rendered derivation** that
/// [D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
/// moves — ein.rs's forks resume root's saturation and ein.py's re-derive it,
/// so the two draw different amounts of the same proof and the cone is no
/// longer byte-comparable.
///
/// Which *views* are derivations is `ein-parity`'s closed list. Which
/// **entries** actually exercise it is this, and it is asserted rather than
/// tolerated — the same discipline [`DIVERGENT`] keeps: a file listed here
/// that stops diverging fails as loudly as one that starts, so a slice that
/// begins differing for an unrelated reason cannot hide behind the cut. The
/// view is still *run* on both sides and both must answer; what replaces the
/// byte check is an ein.rs golden,
/// [S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md).
///
/// A failure prints the list it measured, in this format, to paste back.
const NARRATED_SLICES: [&str; 16] = [
    "examples/branching/03_five_hyps_one_alive.ein",
    "examples/branching/04_two_levels.ein",
    "examples/branching/05_mini_zebra.ein",
    "examples/branching/10_kill_cache_on.ein",
    "examples/branching/11_kill_cache_off.ein",
    "examples/branching/12_typed_blind_solve.ein",
    "examples/branching/13_lookahead_naf_world.ein",
    "examples/features/03_forall.ein",
    "examples/lattice/01_subset_pruned.ein",
    "examples/saturation/implies/org-chart.ein",
    "examples/saturation/implies/parent-to-ancestor.ein",
    "examples/saturation/implies/right-then-next.ein",
    "examples/saturation/transitive/colocation-chain.ein",
    "examples/saturation/transitive/mealtimes.ein",
    "examples/saturation/transitive/taxonomy.ein",
    "examples/zebra2-hints.ein",
];

// D2 reaches two files since 2026-08-20 — the str-vs-int shape Q-M1a.4 was
// written about, and the `Fact`-vs-`Fact` one the S1a.6.6 fuzzer found — and
// each of them in the three views that run the search.
const DIVERGENT: [(&str, &str); 6] = [
    ("examples/ein-bugs/mixed-type-hypothesis.ein", "lattice"),
    (
        "examples/ein-bugs/mixed-type-hypothesis.ein",
        "lattice-full",
    ),
    ("examples/ein-bugs/mixed-type-hypothesis.ein", "slice"),
    ("examples/ein-bugs/nested-fact-hypothesis.ein", "lattice"),
    (
        "examples/ein-bugs/nested-fact-hypothesis.ein",
        "lattice-full",
    ),
    ("examples/ein-bugs/nested-fact-hypothesis.ein", "slice"),
];

fn rust_view(path: &Path, view: &str) -> Option<Answer> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    match dot_shape(&mut ast, &mut terms, &forms, path.parent(), view) {
        Ok(out) => Some(Answer::Ok(out)),
        Err(msg) => Some(Answer::Err {
            kind: "DotShapeError".into(),
            msg,
        }),
    }
}

#[test]
fn every_dot_view_of_every_corpus_file_is_byte_identical() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("every_dot_view_of_every_corpus_file_is_byte_identical");
    };
    let views = all_views();
    let (mut bad, mut compared, mut bytes) = (Vec::new(), 0usize, 0usize);
    let mut seen_divergent: Vec<(String, String)> = Vec::new();
    let mut seen_narrated: Vec<String> = Vec::new();
    let mut files = 0usize;
    for path in &corpus_files() {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        let name = rel.display();
        let before = compared;
        for view in &views {
            let Some(got) = rust_view(path, view) else {
                continue;
            };
            let want = py.ask(serde_json::json!({
                "op": "dot-shape",
                "path": path.to_string_lossy(),
                "view": view,
            }));
            let expected = DIVERGENT
                .iter()
                .any(|(f, v)| *f == rel.to_str().unwrap_or_default() && v == view);
            match (&got, &want) {
                (Answer::Ok(_), Answer::Err { .. }) if expected => {
                    seen_divergent.push((name.to_string(), (*view).to_string()));
                }
                _ if expected => bad.push(format!(
                    "{name} [{view}] is a ledger entry and no longer diverges\n  \
                     rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
                (Answer::Ok(a), Answer::Ok(b)) => {
                    compared += 1;
                    bytes += a.len();
                    if a == b {
                    } else if ein_parity::is_narration(view) && !ein_parity::strict() {
                        seen_narrated.push(rel.to_str().unwrap_or_default().to_string());
                    } else {
                        bad.push(format!("{name} [{view}]\n{}", first_difference(a, b)));
                    }
                }
                // Both refuse: a file the loader rejects has no KB to render,
                // and the message parity that covers it is S1a.2.3's.
                (Answer::Err { .. }, Answer::Err { .. }) => {}
                _ => bad.push(format!(
                    "{name} [{view}]\n  rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
            }
        }
        if compared > before {
            files += 1;
        }
    }
    seen_divergent.sort();
    let mut want_divergent: Vec<(String, String)> = DIVERGENT
        .iter()
        .map(|(f, v)| ((*f).to_string(), (*v).to_string()))
        .collect();
    want_divergent.sort();
    assert_eq!(
        seen_divergent, want_divergent,
        "the ledger's divergent views are not the ones that diverged"
    );
    seen_narrated.sort();
    seen_narrated.dedup();
    if !ein_parity::strict() {
        assert_eq!(
            seen_narrated,
            NARRATED_SLICES,
            "the narrated slices are not the ones that were narrated; measured:\n{}",
            seen_narrated
                .iter()
                .map(|f| format!("    \"{f}\","))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} views differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("T3 (dot): {files} files, {compared} views, {bytes} bytes, 0 differences");
    assert!(
        compared >= 600,
        "only {compared} views compared — the sweep lost its corpus"
    );
}

fn brief(a: &Answer) -> String {
    match a {
        Answer::Ok(s) => format!("{} lines", s.lines().count()),
        Answer::Err { kind, msg } => format!("{kind}: {msg}"),
    }
}

/// The first differing line, with three lines of leading context — a whole
/// digraph diff is unreadable and only the first difference is a cause.
fn first_difference(a: &str, b: &str) -> String {
    let (a, b): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for i in 0..a.len().max(b.len()) {
        let (x, y) = (a.get(i), b.get(i));
        if x != y {
            let mut out: Vec<String> = ((i.saturating_sub(3))..i)
                .map(|j| format!("     {}", a[j]))
                .collect();
            out.push(format!("  rs {}", x.unwrap_or(&"<end>")));
            out.push(format!("  py {}", y.unwrap_or(&"<end>")));
            return format!("  line {}:\n{}", i + 1, out.join("\n"));
        }
    }
    "  (no line differs — trailing newline?)".to_string()
}

/// The node kinds and form heads the corpus does not reach.
///
/// The corpus is a set of *puzzles*, so it exercises what puzzles contain:
/// no `(trace …)` form, no `(query …)` without a `:goal`, no nullary fact, no
/// range or string in an argument position, none of the deprecated `(facts …)`
/// wrappers. Each is a distinct branch in `_emit_fact` / `value_label` /
/// `to_dot`'s dispatch, and a branch nothing renders is a branch nothing
/// compares — so they are fixtures here, sent as `text` rather than a path,
/// and swept through every view exactly as a corpus file is.
const FIXTURES: [(&str, &str); 18] = [
    (
        "equality",
        "(relation r T T)\n(= a b)\n(= (color House-1) Red)",
    ),
    (
        "nullary",
        "(relation z)\n(z)\n(z :source \"nothing at all\")",
    ),
    (
        "arg-kinds",
        "(relation r T T)\n(r 7 \"a string\")\n(r 1..5 1..*)\n(r _ ?x)\n(r (nested a b) c)",
    ),
    (
        "unary-and-nary",
        "(relation u T)\n(u a)\n(u a :source \"(1)\")\n(quad a b c d)\n(quad a b c d :rule mk :using (x))",
    ),
    (
        "negation",
        "(not (r a b))\n(not (u a))\n(not plain)\n(not (r a b) :source \"(9)\")",
    ),
    (
        "is-a-shapes",
        "(is-a a T)\n(is-a 7 T)\n(is-a ?x T)\n(is-a _ T)",
    ),
    (
        "relation-decls",
        "(ontology (relation R) (relation R2 T) (relation R3 T U V) (relation R4 7 T))",
    ),
    (
        "query-full",
        "(query :goal (drinks Water ?h) :hrules (h1 h2) :goal-text \"who drinks water\")",
    ),
    (
        "trace-steps",
        "(trace (step s1 :rule from-condition :using (c10) :derives (lives-in N H1))\n  \
         (step s2 :rule adjacent :using (and s1 c15) :derives (color-loc Blue H2))\n  \
         (step s3 :using (s2))\n  (step s4 :derives (final X)))",
    ),
    ("trace-empty", "(trace)"),
    ("config", "(config :max-set-size 3)"),
    (
        "wrappers",
        "(ontology (relation r T T) (is-a a T))\n(facts (r a b :source \"(1)\"))\n\
         (reasoning (r b c :rule sym :using (x)))",
    ),
    (
        "rule-guards",
        "(relation r T T)\n(relation s T T)\n(rule g ()\n  \
         :match  (and (r ?a ?b) (absent (and (s ?a ?c) (r ?c ?b))) (neq ?a ?b))\n  \
         :assert (s ?a ?b)\n  :why \"guarded\")",
    ),
    (
        "rule-forall-and-not",
        "(relation r T T)\n(rule f ()\n  \
         :match  (and (r ?a ?b) (forall (r ?b ?c)) (not (r ?b ?a)) (eq ?a ?a))\n  \
         :assert (and (r ?b ?a) (not (r ?a ?a)))\n  :why \"forall\")",
    ),
    (
        "rule-nary-and-or",
        "(rule n (?rel)\n  :match  (or (quad ?a ?b ?c ?d) (?rel ?a ?b))\n  \
         :assert (quad ?d ?c ?b ?a))",
    ),
    (
        "rule-half",
        "(rule only-match () :match (r ?a ?b))\n(hrule h () :assert (r a b))",
    ),
    (
        "rule-shared-guard-var",
        "(relation r T T)\n(rule sh ()\n  \
         :match  (and (r ?a ?b) (absent (r ?a ?z)) (absent (r ?b ?z)))\n  \
         :assert (r ?b ?a))",
    ),
    (
        "constraint-scopes",
        "(relation next-to House House)\n(relation right-of House House)\n\
         (bijective next-to)\n(symmetric next-to)\n(includes right-of next-to)\n\
         (slot-partition next-to instance type Attribute House)\n(square-unique right-of House)",
    ),
];

/// The same sweep over hand-written fixtures instead of the corpus.
///
/// Only the parse views run: a fixture is a *syntax* probe, and half of them
/// would not load into a KB at all (a bare `(quad a b c d)` names no declared
/// relation). What the loader does with them is S1a.2.3's question, already
/// answered on the real corpus.
#[test]
fn every_ir_node_kind_renders_identically() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("every_ir_node_kind_renders_identically");
    };
    let views: Vec<&str> = ein_render::shape::PARSE_VIEWS.to_vec();
    let (mut bad, mut compared) = (Vec::new(), 0usize);
    for (name, text) in FIXTURES {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, text, None).unwrap_or_else(|e| panic!("{name}: {e}"));
        for view in &views {
            let got = ein_render::shape::dot_shape(&mut ast, &mut Terms::new(), &forms, None, view)
                .unwrap_or_else(|e| panic!("{name} [{view}]: {e}"));
            let want = py.ask(serde_json::json!({
                "op": "dot-shape", "text": text, "view": view,
            }));
            match want {
                Answer::Ok(want) => {
                    compared += 1;
                    if got != want {
                        bad.push(format!(
                            "{name} [{view}]\n{}",
                            first_difference(&got, &want)
                        ));
                    }
                }
                Answer::Err { kind, msg } => {
                    bad.push(format!("{name} [{view}]\n  py refused: {kind}: {msg}"))
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} fixture views differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("node kinds: {compared} fixture views, 0 differences");
}
