//! M1e2 S1e2.3.8 — **every state `--dump-answer` writes reloads to its own
//! facts, by value**, on every corpus program.
//!
//! [`answer_dump.md`](../../../../docs/kernel/inference/answer_dump.md) says
//! each state file loads, and since S1e2.3.8 says what it loads to: the
//! state's facts, each a value the one printer (`ein_core::text`) wrote as text
//! the loader reads back to the same value, and each **as given**, since a
//! fact's provenance is a comment the loader skips (the inventory's S-5).
//!
//! The comparison is by value, never by text: a fact on each side is its
//! `repr`, which tells a name from an int and a nested fact from either, and
//! the state on the left is the run's own, read again through the node
//! function every route uses (`region::Nodes`), not anything the dump wrote.
//! Until S1e2.3.8 the printer wrote a name as its raw text, so `(holds "x
//! y")` reloaded as the two-argument `(holds x y)`, the int `1` and the name
//! `"1"` as one fact, and a name `#|` opened a block comment;
//! `answer_dump.rs` named the two state files it could not hold
//! (`printed_alike`), and that list is gone.
//!
//! **And every member of N is written with its core** (M1e2 S1e2.3.9,
//! P1e2.3's N6; M1f S1f.5.12): the same run's `nogoods.json` gives each
//! member of N it met a core, its search's status beside it (`within-budget`,
//! `cut-…`) and its rules, past `--region-limit` too. An empty list would read
//! as *refuted by nothing*, and until S1e2.3.9 it was what a run past the
//! limit wrote; from S1e2.3.9 until S1f.5.12 such a run wrote no core, no
//! rules and `core_status: "not-kept"`: `lattice/06_friends_tables`, 68 of
//! 68, and every program past the limit the per-commit tier dumps.
//!
//! **And each is the core the walk takes eagerly** (M1f S1f.5.12, P-15 (c)):
//! `--dump-answer` asks for N's cores, so the run takes none during its walk
//! and each member's after it, against the walk's snapshot. On every program
//! the run describes within the limit, each member a second run with
//! `--dump-lattice` visits, whose walk takes every dead node's core as it
//! goes, has the same core, status and rules in `nodes.json` as in
//! `nogoods.json` (`eager_cores`).
//!
//! **And a node that is not dead has no core** (M1f S1f.5.8, the user's
//! ruling of 2026-11-07 on D-14): every JSON surface that writes a node's
//! core writes a dead node's as a list, `[]` the empty set, and every other
//! node's as `null`, its rules beside it the same way: the summary's root and
//! minimal holes, `nogoods.json`, `ein oracle --dump`'s `region.json`,
//! `--dump-lattice`'s `nodes.json` and `--events`' `enter` lines and `region`
//! event (`no_core_unless_dead`). Until S1f.5.8 an owing hole, and every node that
//! is not dead in the last three, was written `[]`.
//!
//! **And no unfilled template reaches it** (M1f S1f.5.14's T1f.5.14.5): no
//! line of `answer.txt` or of a state file's facts carries `{?`
//! ([`no_placeholder_reaches_an_output`]).
//!
//! **Two tiers** (M1f S1f.5.13, the user's ruling of 2026-11-08 on P-16:
//! *"move to slow tier"*). Per commit the test dumps every entry that is
//! neither `slow` nor `heavy_dump`, the manifest's flag for a dump of at
//! least `ein_corpus::manifest::HEAVY_DUMP_FILES` state files, and fails on
//! one that writes that many unflagged. Under `EIN_CORPUS_SLOW=1`, which the
//! nightly's `deep-corpus` job sets, it dumps every entry, and fails as well
//! on a flagged one that writes fewer. Five entries were flagged on
//! 2026-11-09, `square-unique`'s three, `features/04_open` and
//! `features/01_not_and_absent`, each writing 64 514 to 65 214 state files;
//! until then the first four ran per commit, and most of the test's time
//! (`ein_corpus`'s `the_heavy_dump_set_is_exactly_these_five`).
//! `past-limit-lines` keeps a run past `--region-limit` in the per-commit
//! tier, three state files and 15 members of N.
//!
//! ```text
//! cargo test -p ein-cli --test answer_round_trip
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test answer_round_trip  # + the slow and heavy_dump entries
//! EIN_ROUND_TRIP_LIST=1 … -- --nocapture  # each program's state files
//! ```

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use ein_core::{Kb, ProvKind, Terms};
use ein_corpus::manifest::HEAVY_DUMP_FILES;
use ein_corpus::{Scratch, corpus, repo_root};
use ein_infer::explore::{Ask, Mode, Options, Which};
use ein_infer::read_out::{self, Listing};
use ein_infer::region::Nodes;
use ein_ir::Ast;

fn include_slow() -> bool {
    let on = |k: &str| std::env::var(k).as_deref() == Ok("1");
    on("EIN_CORPUS_SLOW") || on("EIN_BLESS")
}

/// A KB's facts by value: each its `repr`, a name and an int apart.
fn by_value(kb: &Kb, terms: &Terms) -> BTreeSet<String> {
    kb.facts()
        .map(|f| ein_core::repr::repr(&terms.repr_fact(f)))
        .collect()
}

/// A state file, loaded: its facts by value, and how many of them carry a
/// provenance other than a given fact's.
fn reloaded(path: &Path) -> Result<(BTreeSet<String>, usize), String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = ein_ir::load_file(&mut ast, &mut terms, path).map_err(|e| format!("{e}"))?;
    let not_given = kb
        .facts()
        .filter(|&f| {
            kb.primary(f)
                .is_some_and(|p| terms.provs.get(p).kind != ProvKind::Source)
        })
        .count();
    Ok((by_value(&kb, &terms), not_given))
}

/// **The state files that do not load today**, each the witness of an open
/// register row (M1f S1f.5.0, 2026-11-08): W30, a rule's concluded `neq`
/// stored as a fact, which a state file writes as a fact line `(neq …)`, and
/// W34, `:assert ()` stored as a fact, written `()`; the parser refuses both
/// lines. Not a printer's defect but the row's own wrong answer, so each is
/// named rather than checked, and each must still fail: a state file here
/// that loads is its row fixed, and S1f.5.7 (W30) and S1f.5.9 (W34) each take
/// their files off the list in the commit that fixes them, with
/// `answer_dump.rs`'s copy. W30's two left the list at M1f S1f.5.7's
/// T1f.5.7.4 (2026-11-09): a concluded `neq` is checked, never stored; and
/// W34's at M1f S1f.5.9's T1f.5.9.2 (2026-11-09): `:assert ()` asserts
/// nothing, so no state holds the fact. Empty since.
const DO_NOT_LOAD: [&str; 0] = [];

/// The difference, a few facts each way.
fn diff(want: &BTreeSet<String>, got: &BTreeSet<String>) -> String {
    let lost: Vec<&String> = want.difference(got).take(3).collect();
    let extra: Vec<&String> = got.difference(want).take(3).collect();
    format!(
        "{} facts lost ({lost:?}…), {} not the state's ({extra:?}…)",
        want.difference(got).count(),
        got.difference(want).count()
    )
}

/// `ein solve FILE --dump-answer DIR`, then every state file it wrote against
/// the run's own state: the number of files checked, or `None` where the
/// program does not load or the run writes no state. `eager` counts the
/// members of N held to their eager cores ([`eager_cores`]), and `held` the
/// nodes and holes held to their deadness ([`no_core_unless_dead`]).
fn check(
    file: &str,
    dir: &Path,
    bad: &mut Vec<String>,
    eager: &mut usize,
    held: &mut Held,
) -> Option<usize> {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--dump-answer"])
        .arg(dir)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    // A refuted claim exits 1 and writes its answer whole (W28's witnesses);
    // a program the binary does not answer writes no index.
    if !out.status.success() && !dir.join("index.json").exists() {
        return None;
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = ein_ir::load_file(&mut ast, &mut terms, &repo_root().join(file)).ok()?;
    let opts = Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    let r = read_out::run_with(
        &mut kb,
        &mut terms,
        &ast,
        &opts,
        &mut ein_infer::events::Events::off(),
    )
    .expect("the binary answered it");
    let mut files: Vec<(String, Vec<u32>)> = Vec::new();
    let width = |k: usize| k.max(1).to_string().len().max(2);
    let k = r.states.len();
    for (i, s) in r.states.iter().enumerate() {
        files.push((
            format!("models/{:0w$}.ein", i + 1, w = width(k)),
            s.rep.to_vec(),
        ));
    }
    if let Listing::Owing(owing) = &r.listing {
        for (i, o) in owing.iter().enumerate() {
            let w = width(owing.len());
            files.push((format!("owing/{:0w$}.ein", i + 1), o.node.set.to_vec()));
        }
    }
    no_placeholder_reaches_an_output(file, &dir.join("answer.txt"), bad);
    for (path, _) in &files {
        no_placeholder_reaches_an_output(file, &dir.join(path), bad);
    }
    members_of_n(file, dir, bad);
    *eager += eager_cores(file, dir, bad, held);
    held.add(no_core_unless_dead(file, dir, bad));
    let candidates = r.answer.candidates.clone();
    let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &candidates).expect("prepares");
    for (path, set) in &files {
        let facts: Vec<_> = set.iter().map(|&b| candidates[b as usize].fact).collect();
        let want = nodes
            .with_fixpoint(&facts, |kb, terms| by_value(kb, terms))
            .expect("saturates");
        let named = DO_NOT_LOAD.contains(&file);
        match reloaded(&dir.join(path)) {
            Err(_) if named => {}
            Err(e) => bad.push(format!("{file}: {path} does not load: {e}")),
            Ok(_) if named => bad.push(format!(
                "{file}: {path} loads, and DO_NOT_LOAD names it: its row (W30 or W34) is \
                 fixed, so drop it from the list in the same commit"
            )),
            Ok((got, not_given)) => {
                if got != want {
                    bad.push(format!("{file}: {path}: {}", diff(&want, &got)));
                }
                if not_given > 0 {
                    bad.push(format!(
                        "{file}: {path}: {not_given} facts reload with a provenance, not as given"
                    ));
                }
            }
        }
    }
    Some(files.len())
}

/// **No unfilled template reaches an output** — M1f S1f.5.14's T1f.5.14.5,
/// the second of the stage's two checks: no line of `answer.txt` (the
/// read-out, a class's `:goal-text` filled) or of a state file (each fact's
/// English, its rule's `:why` and its relation's filled) carries `{?`, which
/// [`ein_core::render_why`] leaves where a reference names nothing it binds.
/// A state file re-writes the program above its facts, `:why` templates and
/// all, so only the lines after its `── the state` marker are read. It came
/// back clean on the corpus as it stood when it was written, which is a
/// result too: `ein-ir/tests/template_wellformedness.rs` holds the cause, the
/// template, and this holds the effect wherever a template is filled.
fn no_placeholder_reaches_an_output(file: &str, path: &Path, bad: &mut Vec<String>) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    let rendered = match text.split_once("── the state") {
        Some((_, facts)) => facts,
        None if path.extension().is_some_and(|x| x == "ein") => return,
        None => &text,
    };
    for line in rendered.lines().filter(|l| l.contains("{?")) {
        bad.push(format!(
            "{file}: {} prints an unfilled template reference: {line}",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
}

/// `nogoods.json`'s members, each with a core, its search's status and its
/// rules, past `--region-limit` too: never `not-kept` (M1f S1f.5.12), and
/// never an empty core (M1e2 S1e2.3.9) but on ∅, a dead root's, whose
/// refutation rests on no fact. A member that is not ∅ has a core naming its
/// commitment's facts, so `[]`, the empty set, is root's alone
/// (`empty-match-false.ein`'s, since M1f S1f.5.9 made `:match ()` fire). Each
/// failure goes to `bad`.
fn members_of_n(file: &str, dir: &Path, bad: &mut Vec<String>) {
    let read = |name: &str| -> Option<serde_json::Value> {
        serde_json::from_str(&std::fs::read_to_string(dir.join(name)).ok()?).ok()
    };
    let (Some(nogoods), Some(summary)) = (read("nogoods.json"), read("answer.json")) else {
        bad.push(format!("{file}: no nogoods.json or answer.json"));
        return;
    };
    let region = &summary["region"];
    let stopped_at_limit = region["exhausted"] == false && region["stopped"]["why"] == "limit";
    for m in nogoods.as_array().into_iter().flatten() {
        let (core, status, rules) = (&m["core"], m["core_status"].as_str(), &m["refuted_by"]);
        let searched = status.is_some_and(|w| w == "within-budget" || w.starts_with("cut-"));
        let root = m["set"].as_array().is_some_and(|s| s.is_empty());
        let ok =
            core.as_array().is_some_and(|c| !c.is_empty() || root) && searched && rules.is_array();
        if !ok {
            bad.push(format!(
                "{file}: nogoods.json's {} has the core {core} as {status:?} and the rules {rules}{}",
                m["set"],
                if stopped_at_limit {
                    ", on a run stopped at --region-limit"
                } else {
                    ""
                }
            ));
        }
    }
}

/// N's cores against the eager ones (M1f S1f.5.12): where the run in `dir`
/// described R within `--region-limit`, `ein solve FILE --dump-lattice`, whose
/// walk takes every dead node's core as it visits it, and for each member of
/// `nogoods.json` that run visited, its core, status and rules in `nodes.json`
/// equal `nogoods.json`'s. A member the route reads off others and does not
/// visit (the state route's `N`, by duality) is not compared. How many were
/// compared. Every node of that `nodes.json` is held to its deadness too, a
/// dead one's core a list and every other's `null` ([`no_core_unless_dead`],
/// M1f S1f.5.8).
fn eager_cores(file: &str, dir: &Path, bad: &mut Vec<String>, held: &mut Held) -> usize {
    let read = |path: &Path| -> Option<serde_json::Value> {
        serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
    };
    let (Some(nogoods), Some(summary)) = (
        read(&dir.join("nogoods.json")),
        read(&dir.join("answer.json")),
    ) else {
        return 0;
    };
    let region = &summary["region"];
    let stopped_at_limit = region["exhausted"] == false && region["stopped"]["why"] == "limit";
    let members = nogoods.as_array().cloned().unwrap_or_default();
    if stopped_at_limit || members.is_empty() {
        return 0;
    }
    let lattice = dir.join("lattice");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--dump-lattice"])
        .arg(&lattice)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let Some(nodes) = read(&lattice.join("nodes.json")) else {
        bad.push(format!(
            "{file}: --dump-lattice wrote no nodes.json (exit {:?})",
            out.status.code()
        ));
        return 0;
    };
    let by_set: std::collections::BTreeMap<String, &serde_json::Value> =
        members.iter().map(|m| (m["set"].to_string(), m)).collect();
    let mut compared = 0;
    for node in nodes["nodes"].as_array().into_iter().flatten() {
        let at = format!("{file}: nodes.json's {}", node["commitment"]);
        let dead = node["kind"] == "dead";
        held_to_its_deadness(held, bad, &at, dead, node, &["core", "refuted_by"]);
        let Some(m) = by_set.get(&node["commitment"].to_string()) else {
            continue;
        };
        compared += 1;
        for f in ["core", "core_status", "refuted_by"] {
            if m[f] != node[f] {
                bad.push(format!(
                    "{file}: N's {}: nogoods.json's {f} {} is not the eager {}",
                    m["set"], m[f], node[f]
                ));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&lattice);
    compared
}

/// The nodes and holes [`no_core_unless_dead`] held, by whether each is dead.
#[derive(Default)]
struct Held {
    dead: usize,
    not_dead: usize,
}

impl Held {
    fn add(&mut self, other: Held) {
        self.dead += other.dead;
        self.not_dead += other.not_dead;
    }
}

/// One node or hole of a JSON surface: where it is dead, each of `fields` is a
/// list; where it is not, each is `null`, and present.
fn held_to_its_deadness(
    held: &mut Held,
    bad: &mut Vec<String>,
    at: &str,
    dead: bool,
    x: &serde_json::Value,
    fields: &[&str],
) {
    for f in fields {
        let ok = match x.get(f) {
            Some(v) if dead => v.is_array(),
            Some(v) => v.is_null(),
            None => false,
        };
        if !ok {
            bad.push(format!(
                "{at}: a {} node's {f} is {}, where it should be {}",
                if dead { "dead" } else { "not-dead" },
                x.get(f).map_or("absent".to_string(), |v| v.to_string()),
                if dead { "a list" } else { "null" }
            ));
        }
    }
    if dead {
        held.dead += 1;
    } else {
        held.not_dead += 1;
    }
}

/// A summary's `region` block: root's core and rules where root's kind is
/// stated, and each minimal hole's core.
fn summary_cores(held: &mut Held, bad: &mut Vec<String>, at: &str, summary: &serde_json::Value) {
    let region = &summary["region"];
    if let Some(root) = region["root"].as_str() {
        held_to_its_deadness(
            held,
            bad,
            &format!("{at}'s root"),
            root == "dead",
            region,
            &["core", "refuted_by"],
        );
    }
    for h in region["holes"]["minimal"].as_array().into_iter().flatten() {
        held_to_its_deadness(
            held,
            bad,
            &format!("{at}'s hole {}", h["set"]),
            h["dead"] == true,
            h,
            &["core"],
        );
    }
}

/// **A node that is not dead has no core** (M1f S1f.5.8, the user's ruling
/// of 2026-11-07 on D-14, Q-M1e2.27): a core is one object, the set of facts
/// a dead node's contradiction rests on, so a dead node's `core` and
/// `refuted_by` are lists, `[]` the empty set, and a node that is not dead,
/// whose core does not exist, has `null`. Held on every JSON surface that
/// writes a node's core: the summary's root and minimal holes
/// (`answer.json`, `index.json`, and `ein oracle --json-summary`),
/// `nogoods.json`, and `ein oracle --dump`'s `region.json`, `--dump-lattice`'s
/// `nodes.json` and `--events`' `enter` lines and `region` event, the last
/// four where the oracle answers at `--region-limit 10`, which bounds what
/// they cost, the `enter` lines node for node against `nodes.json`; [`eager_cores`] holds the
/// `nodes.json` of every other program it dumps. Until S1f.5.8 a node that is
/// not dead was written `[]`, which reads as *the empty set of facts
/// suffices*.
fn no_core_unless_dead(file: &str, dir: &Path, bad: &mut Vec<String>) -> Held {
    let mut held = Held::default();
    let read = |path: &Path| -> Option<serde_json::Value> {
        serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
    };
    for name in ["answer.json", "index.json"] {
        if let Some(j) = read(&dir.join(name)) {
            summary_cores(&mut held, bad, &format!("{file}: {name}"), &j);
        }
    }
    if let Some(n) = read(&dir.join("nogoods.json")) {
        for m in n.as_array().into_iter().flatten() {
            let at = format!("{file}: nogoods.json's {}", m["set"]);
            held_to_its_deadness(&mut held, bad, &at, true, m, &["core", "refuted_by"]);
        }
    }
    let cube = dir.join("cube");
    let oracle_summary = dir.join("oracle.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["oracle", file, "--region-limit", "10", "--dump"])
        .arg(&cube)
        .arg("--json-summary")
        .arg(&oracle_summary)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    // Past the limit, or a program the oracle refuses: nothing to read.
    if !out.status.success() {
        return held;
    }
    if let Some(j) = read(&oracle_summary) {
        summary_cores(&mut held, bad, &format!("{file}: ein oracle's summary"), &j);
    }
    match read(&cube.join("region.json")) {
        Some(r) => {
            for n in r["region"]["nodes"].as_array().into_iter().flatten() {
                let dead = n["status"].as_str().is_some_and(|s| s.starts_with("dead"));
                let at = format!("{file}: region.json's node {}", n["mask"]);
                held_to_its_deadness(&mut held, bad, &at, dead, n, &["core", "refuted_by"]);
            }
        }
        None => bad.push(format!("{file}: ein oracle --dump wrote no region.json")),
    }
    let lattice = dir.join("visited");
    let events = dir.join("events.jsonl");
    Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--dump-lattice"])
        .arg(&lattice)
        .arg("--events")
        .arg(&events)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let Some(nodes) = read(&lattice.join("nodes.json")) else {
        bad.push(format!("{file}: --dump-lattice wrote no nodes.json"));
        return held;
    };
    let nodes = nodes["nodes"].as_array().cloned().unwrap_or_default();
    for n in &nodes {
        let at = format!("{file}: nodes.json's {}", n["commitment"]);
        let dead = n["kind"] == "dead";
        held_to_its_deadness(&mut held, bad, &at, dead, n, &["core", "refuted_by"]);
    }
    let lines: Vec<serde_json::Value> = std::fs::read_to_string(&events)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .collect();
    // The `region` event carries the summary's blocks, its holes among them.
    for e in lines.iter().filter(|e| e["e"] == "region") {
        summary_cores(&mut held, bad, &format!("{file}: the region event"), e);
    }
    let enter: Vec<&serde_json::Value> = lines.iter().filter(|e| e["e"] == "enter").collect();
    if enter.len() != nodes.len() {
        bad.push(format!(
            "{file}: {} enter lines, where nodes.json has {} nodes",
            enter.len(),
            nodes.len()
        ));
    }
    for (&e, n) in enter.iter().zip(&nodes) {
        let at = format!("{file}: the enter line of {}", e["commitment"]);
        held_to_its_deadness(&mut held, bad, &at, e["kind"] == "dead", e, &["core"]);
        if (&e["commitment"], &e["kind"], &e["core"]) != (&n["commitment"], &n["kind"], &n["core"])
        {
            bad.push(format!(
                "{at}: {} {} {}, where nodes.json has {} {} {}",
                e["commitment"], e["kind"], e["core"], n["commitment"], n["kind"], n["core"]
            ));
        }
    }
    let _ = std::fs::remove_dir_all(&cube);
    let _ = std::fs::remove_dir_all(&lattice);
    let _ = std::fs::remove_file(&events);
    held
}

/// The shards [`shard`] splits the corpus into, one test each. Per commit no
/// program dominates them since S1f.5.13 (`type-exclusivity/pets` writes the
/// most, 11 339 state files); under `EIN_CORPUS_SLOW=1` they put the five
/// `heavy_dump` entries and the zebra family's beside the rest, not after
/// them.
const SHARDS: usize = 8;

/// The `heavy_dump` flag held to the count a dump wrote, `None` where the
/// program wrote none: an unflagged entry that wrote
/// [`HEAVY_DUMP_FILES`] or more, or a flagged one that wrote fewer.
fn heavy_dump_flag(e: &ein_corpus::Entry, wrote: Option<usize>) -> Option<String> {
    let n = wrote.unwrap_or(0);
    match (e.heavy_dump, n >= HEAVY_DUMP_FILES) {
        (false, true) => Some(format!(
            "{}: its dump writes {n} state files, {HEAVY_DUMP_FILES} or more, and its \
             corpus entry is not `heavy_dump = true`: flag it, which moves its round trip \
             to the slow tier, and bank it in `the_heavy_dump_set_is_exactly_these_five`",
            e.path
        )),
        (true, false) => Some(format!(
            "{}: `heavy_dump = true`, and its dump writes {n} state files, under \
             {HEAVY_DUMP_FILES}: take the flag off, which brings its round trip back to \
             every commit",
            e.path
        )),
        _ => None,
    }
}

/// Every corpus program `ein solve` answers, whether or not its entry declares
/// the run, every state file of its describe run (the listed states, the
/// members met and not listed, and an empty R's owing frontier), each against
/// the run's own state, by value: the entries whose index in the selection is
/// `k` modulo [`SHARDS`]. The selection is every entry under
/// `EIN_CORPUS_SLOW=1`, and per commit every entry that is neither `slow` nor
/// `heavy_dump`; each entry it dumps is held to its `heavy_dump` flag
/// ([`heavy_dump_flag`]).
fn shard(k: usize) {
    let root = Scratch::new(&format!("ein-answer-round-trip-{k}"));
    let slow = include_slow();
    let list = std::env::var("EIN_ROUND_TRIP_LIST").is_ok();
    let mut bad = Vec::new();
    let (mut programs, mut states, mut eager) = (0, 0, 0);
    let mut held = Held::default();
    let entries = corpus();
    let selected: Vec<_> = entries
        .select(&[], None, slow)
        .into_iter()
        .filter(|e| slow || !e.heavy_dump)
        .collect();
    for (i, e) in selected.iter().enumerate() {
        if i % SHARDS != k {
            continue;
        }
        let dir = root.join(format!("e{i}"));
        let wrote = check(&e.path, &dir, &mut bad, &mut eager, &mut held);
        bad.extend(heavy_dump_flag(e, wrote));
        match wrote {
            Some(n) => {
                programs += 1;
                states += n;
                if list {
                    eprintln!("{n:>6} {}", e.path);
                }
            }
            None if list => eprintln!("  none {}", e.path),
            None => {}
        }
        // A dump past the limit is tens of thousands of files.
        let _ = std::fs::remove_dir_all(&dir);
    }
    assert!(
        programs > 20 && states > programs && eager > 0 && held.dead > 0 && held.not_dead > 0,
        "shard {k}: {programs} programs, {states} states, {eager} members of N held to their \
         eager cores, {} dead nodes and {} others held to their deadness",
        held.dead,
        held.not_dead
    );
    eprintln!(
        "shard {k}: {programs} programs, {states} state files, {eager} members of N held to \
         their eager cores, {} dead nodes and holes each with a core and {} others each with \
         none",
        held.dead, held.not_dead
    );
    assert!(
        bad.is_empty(),
        "shard {k}: {} of {states} state files over {programs} programs:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_0() {
    shard(0);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_1() {
    shard(1);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_2() {
    shard(2);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_3() {
    shard(3);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_4() {
    shard(4);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_5() {
    shard(5);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_6() {
    shard(6);
}

#[test]
fn every_state_file_reloads_to_its_own_facts_by_value_7() {
    shard(7);
}

/// The witnesses: programs in the final grammar whose names exercise the
/// printer, written here, and `syntax/arg-kinds`.
///
/// Restated at S1f.5.5's swap (T1f.5.5.4). Until then they were the probe the
/// inventory of 2026-11-05 re-took S-5 on and W16's pair, whose states held the
/// int `1` and the name `"1"` in one position and which `answer_dump.rs` named
/// as the files the printer could not write, and `arg-kinds`. Each writes a
/// string as a value, which S1f.5.2's ruling refuses at the parse, and since
/// S1f.5.4's name rule there is no int: a run of digits is a name. What the
/// printer must now get right is the name rule's characters, so the
/// witnesses write `7` beside `07`, `-7` and `1..5`, every decorator, and the
/// brackets, as fact arguments, candidates, nested facts and derived facts.
#[test]
fn the_witnesses_reload_to_their_states() {
    let root = Scratch::new("ein-answer-round-trip-w");
    let digits = root.join("digits.ein");
    std::fs::write(
        &digits,
        ";; A run of digits is a name: `7`, `07`, `-7` and `1..5` are four.\n\
         (relation seen Thing :why \"{?1} is seen\")\n\
         (relation pick Thing :why \"{?1} is picked\")\n\
         (relation pair Thing Thing :why \"{?1} pairs with {?2}\")\n\
         (seen 7) (seen 07) (seen -7) (seen 1..5)\n\
         (rule pairs () :match (and (pick ?x) (seen ?y) (neq ?x ?y)) :assert (pair ?x ?y)\n\
           :why \"{?x}, picked, pairs with {?y}\")\n\
         (solve :hypothesis-relations (pick))\n",
    )
    .expect("the witness is written");
    let decorated = root.join("decorated.ein");
    std::fs::write(
        &decorated,
        ";; The name rule's decorators and brackets, bare and in a nested fact.\n\
         (relation has Thing Thing :why \"{?1} has {?2}\")\n\
         (relation tag Thing :why \"{?1} is tagged\")\n\
         (relation kept Thing :why \"{?1} is kept\")\n\
         (has $x [a]) (has a/b {b}) (has it's f') (has <= (box !x @x ~))\n\
         (has a^b%=+& a.b-c_d*)\n\
         (hrule tags () :match (has ?x ?y) :assert (tag ?x) :why \"{?x} may be tagged\")\n\
         (rule keep () :match (and (tag ?x) (has ?x ?y)) :assert (kept ?y) :why \"{?y} kept\")\n\
         (solve :hrules (tags))\n",
    )
    .expect("the witness is written");
    let mut bad = Vec::new();
    let mut states = 0;
    for (i, file) in [
        digits.to_str().expect("utf-8"),
        decorated.to_str().expect("utf-8"),
        "examples/syntax/arg-kinds.ein",
    ]
    .iter()
    .enumerate()
    {
        let wrote = check(
            file,
            &root.join(format!("w{i}")),
            &mut bad,
            &mut 0,
            &mut Held::default(),
        );
        states += wrote.unwrap_or_else(|| panic!("{file}: no state written"));
    }
    assert!(states >= 40, "{states}");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
