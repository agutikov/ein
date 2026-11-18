//! **Every state `--dump-answer` writes is saved and loaded as itself**, in
//! both directions, on every corpus program
//! ([Q-M1f.392](../../../../docs/decisions/answer/Q-M1f.392-a-saved-state-is-saved-and-loaded-as-itself.md)).
//!
//! A state file is the program as it was read, every module its load read,
//! and the choices made, `(commit …)`
//! ([`10-answer-dump.md`](../../../../docs/ein/user/04-reference/10-answer-dump.md)),
//! and a load derives the state again. For each file the run wrote, its
//! models' and an empty R's owing frontier's:
//!
//! - **the first load is the state saved**: the file's facts, as `ein
//!   saturate --dump` prints them (`ein_cli::saturate::facts_section`), each
//!   with its origin and in its order, are the run's own state's, read
//!   through the node function every route uses (`region::Nodes`), fact for
//!   fact and origin for origin;
//! - **save → load → save is byte for byte**: the file loaded and its root
//!   saved again, as `ein solve F --no-search --dump-answer D` writes
//!   `D/root.ein`, is the file;
//! - **load → save → load is dump for dump**: the KB section `ein saturate
//!   --dump` prints of the file and of that second save is the same text.
//!
//! The three are read with no process between (`ein_cli::saturate::dump_section`,
//! one load giving a file's section and its root saved again, by
//! `region_answer::saved_file` from the node's commitment, as `ein solve
//! --no-search --dump-answer` saves it), and the binary is held to them on each
//! program's first and last file: `ein solve F --no-search --dump-answer D`
//! then `cmp`, and `ein saturate --dump` of both, against the section read
//! in the process.
//!
//! **And a node's container opens as the node** (Q-M1f.413): the last file
//! of each program that is a node, one with a `(commit …)`, stored by `ein kb
//! save` and solved, prints what `ein solve` of the file prints, stdout and
//! stderr byte for byte but the path, with its exit code
//! ([`by_its_container`]).
//!
//! **And every member of N is written with its core**: the same run's
//! `nogoods.json` gives each member of N it met a core, its search's status
//! beside it (`within-budget`, `cut-…`) and its rules, past `--region-limit`
//! too, never an empty list, which would read as *refuted by nothing*, nor
//! `core_status: "not-kept"` (`lattice/06_friends_tables` and every program
//! past the limit the per-commit tier dumps).
//!
//! **And each is the core the walk takes eagerly**:
//! `--dump-answer` asks for N's cores, so the run takes none during its walk
//! and each member's after it, against the walk's snapshot. On every program
//! the run describes within the limit, each member a second run with
//! `--dump-lattice` visits, whose walk takes every dead node's core as it
//! goes, has the same core, status and rules in `nodes.json` as in
//! `nogoods.json` (`eager_cores`).
//!
//! **And a node that is not dead has no core** (Q-M1e2.27): every JSON
//! surface that writes a node's
//! core writes a dead node's as a list, `[]` the empty set, and every other
//! node's as `null`, its rules beside it the same way: the summary's root and
//! minimal holes, `nogoods.json`, `ein oracle --dump`'s `region.json`,
//! `--dump-lattice`'s `nodes.json` and `--events`' `enter` lines and `region`
//! event (`no_core_unless_dead`), an owing hole among them.
//!
//! **And no unfilled template reaches it**: no
//! line of `answer.txt` carries `{?`
//! ([`no_placeholder_reaches_an_output`]).
//!
//! **Two tiers.** By default the test dumps every entry that is neither
//! `slow` nor `heavy_dump`, the manifest's flag for a dump of at least
//! `ein_corpus::manifest::HEAVY_DUMP_FILES` state files, and fails on one
//! that writes that many unflagged. Under `EIN_CORPUS_SLOW=1` it dumps every
//! entry, and fails as well on a flagged one that writes fewer. Five entries
//! are flagged, `square-unique`'s three, `features/04_open` and
//! `features/01_not_and_absent` (`ein_corpus`'s
//! `the_heavy_dump_set_is_exactly_these_five`).
//! `past-limit-lines` keeps a run past `--region-limit` in the per-commit
//! tier, three state files and 15 members of N.
//!
//! ```text
//! cargo test -p ein-cli --test answer_round_trip
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test answer_round_trip  # + the slow and heavy_dump entries
//! EIN_ROUND_TRIP_LIST=1 … -- --nocapture  # each program's state files
//! ```

use std::path::Path;
use std::process::Command;

use ein_core::Terms;
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

/// **The witnesses whose `answer.txt` prints an unfilled template
/// reference**, each a known wrong answer, named rather than checked. Empty:
/// the texts `answer.txt` fills are a query's `:goal-text` and an obligation
/// rule's `:why`, each by the variables its form binds, and a relation's
/// `:why` reads its declaration alone, never a fact of it (Q-M1f.58), so no
/// fact with fewer arguments than its declaration's labels leaves one
/// unfilled.
const UNFILLED_TEMPLATE: [&str; 0] = [];

/// What the invariances held, summed over files.
#[derive(Default)]
struct Counts {
    /// State files whose first load is the state saved.
    states: usize,
    /// State files saved again byte for byte.
    bytes: usize,
    /// State files whose two loads print the same dump.
    dumps: usize,
    /// State files the binary was run on.
    cli: usize,
    /// Nodes whose container answered as the file.
    containers: usize,
}

impl Counts {
    fn add(&mut self, o: &Counts) {
        self.states += o.states;
        self.bytes += o.bytes;
        self.dumps += o.dumps;
        self.cli += o.cli;
        self.containers += o.containers;
    }
}

/// One load of `path`, in the process: `ein saturate --dump`'s KB section,
/// its facts, and its root saved again as `ein solve FILE --no-search
/// --dump-answer DIR` writes `DIR/root.ein`.
fn section(path: &Path) -> Result<ein_cli::saturate::Dumped, String> {
    ein_cli::saturate::dump_section(path, &ein_ir::LoadOptions::default())
}

/// The KB section of `ein saturate --dump`'s output, from its banner.
fn printed_section(stdout: &str) -> String {
    let at = stdout.find("\n=====").map_or(stdout.len(), |i| i);
    stdout[at..].to_string()
}

/// The binary on one state file: `ein solve F --no-search --dump-answer D`
/// then `D/root.ein` against F byte for byte, and `ein saturate --dump` of
/// both, each section against `whole`, the one read in the process.
fn by_the_binary(file: &str, path: &Path, again: &Path, whole: &str, bad: &mut Vec<String>) {
    let ein = |args: &[&std::ffi::OsStr]| {
        Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(args)
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs")
    };
    let out = ein(&[
        "solve".as_ref(),
        path.as_os_str(),
        "--no-search".as_ref(),
        "--dump-answer".as_ref(),
        again.as_os_str(),
    ]);
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let root = again.join("root.ein");
    match (std::fs::read(path), std::fs::read(&root)) {
        (Ok(a), Ok(b)) if a == b => {}
        (Ok(_), Ok(_)) => bad.push(format!(
            "{file}: {name}: `ein solve F --no-search --dump-answer D` writes D/root.ein, which \
             is not F byte for byte"
        )),
        _ => bad.push(format!(
            "{file}: {name}: `ein solve F --no-search --dump-answer D` wrote no root.ein (exit \
             {:?}): {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        )),
    }
    for p in [path, root.as_path()] {
        let out = ein(&["saturate".as_ref(), "--dump".as_ref(), p.as_os_str()]);
        let printed = printed_section(&String::from_utf8_lossy(&out.stdout));
        if !out.status.success() || printed != whole {
            bad.push(format!(
                "{file}: {}: `ein saturate --dump` prints another KB section than the load in \
                 the process (exit {:?})",
                p.display(),
                out.status.code()
            ));
        }
    }
}

/// **A node's container opens as the node** (Q-M1f.413): `ein kb save F
/// X.einb` of the saved file `path`, a node, then `ein solve` of each: the
/// container's stdout and stderr are the file's, byte for byte, but for the
/// path each echoes, and its exit code is the file's. A load's `note:` names
/// a position in the text the load read, which in a container is the
/// program it stores, resolved and import-flattened, so a note's position
/// is compared as `<position>` (Q-M1f.15). Whether the container answered as
/// the file; a failure goes to `bad`.
#[cfg(feature = "einb")]
fn by_its_container(file: &str, path: &Path, scratch: &Path, bad: &mut Vec<String>) -> bool {
    let ein = |args: &[&std::ffi::OsStr]| {
        Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(args)
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs")
    };
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let einb = scratch.join("node.einb");
    let saved = ein(&[
        "kb".as_ref(),
        "save".as_ref(),
        path.as_os_str(),
        einb.as_os_str(),
    ]);
    let notes_only = String::from_utf8_lossy(&saved.stderr)
        .lines()
        .all(|l| l.starts_with("note: "));
    if !saved.status.success() || !notes_only {
        bad.push(format!(
            "{file}: {name}: `ein kb save` of a node (exit {:?}): {}",
            saved.status.code(),
            String::from_utf8_lossy(&saved.stderr)
        ));
        return false;
    }
    let (from_file, from_container) = (
        ein(&["solve".as_ref(), path.as_os_str()]),
        ein(&["solve".as_ref(), einb.as_os_str()]),
    );
    let (container, saved) = (einb.display().to_string(), path.display().to_string());
    let text = |b: &[u8]| positions(&String::from_utf8_lossy(b));
    let read = |b: &[u8]| text(b).replace(&container, &saved);
    let mut same = true;
    for (what, want, got) in [
        (
            "stdout",
            text(&from_file.stdout),
            read(&from_container.stdout),
        ),
        (
            "stderr",
            text(&from_file.stderr),
            read(&from_container.stderr),
        ),
    ] {
        if want != got {
            same = false;
            bad.push(format!(
                "{file}: {name}: `ein solve` of its container prints another {what} than of the \
                 file:\n{}",
                first_difference(&want, &got)
            ));
        }
    }
    if from_file.status.code() != from_container.status.code() {
        same = false;
        bad.push(format!(
            "{file}: {name}: `ein solve` of its container exits {:?}, of the file {:?}",
            from_container.status.code(),
            from_file.status.code()
        ));
    }
    let _ = std::fs::remove_file(&einb);
    same
}

/// `text` with every position in a program a load read, `<program saved in
/// F>:L:C` or `<program stored in X>:L:C`, written `<position>`.
#[cfg(feature = "einb")]
fn positions(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("<program ") {
        out.push_str(&rest[..at]);
        let after = &rest[at..];
        let Some(close) = after.find('>') else {
            out.push_str(after);
            return out;
        };
        let tail = &after[close + 1..];
        // `:L:C`, two runs of digits.
        let digits = |t: &str| t.find(|c: char| !c.is_ascii_digit()).unwrap_or(t.len());
        let n = tail
            .strip_prefix(':')
            .map(|t| (digits(t), t))
            .filter(|(d, _)| *d > 0)
            .and_then(|(d, t)| {
                let t2 = t[d..].strip_prefix(':')?;
                let d2 = digits(t2);
                (d2 > 0).then_some(1 + d + 1 + d2)
            });
        match n {
            Some(n) => {
                out.push_str("<position>");
                rest = &tail[n..];
            }
            None => {
                out.push_str(&after[..=close]);
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

/// `ein solve FILE --dump-answer DIR`, then every state file it wrote held to
/// the run's own state and to the two invariances (the module header): the
/// number of files checked, or `None` where the program does not load or the
/// run writes no state. `eager` counts the members of N held to their eager
/// cores ([`eager_cores`]), `held` the nodes and holes held to their deadness
/// ([`no_core_unless_dead`]), and `counts` the files each invariance held.
fn check(
    file: &str,
    dir: &Path,
    bad: &mut Vec<String>,
    eager: &mut usize,
    held: &mut Held,
    counts: &mut Counts,
) -> Option<usize> {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--dump-answer"])
        .arg(dir)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    // A refuted claim exits 1 and writes its answer whole;
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
    let mut unfilled: Vec<String> = Vec::new();
    no_placeholder_reaches_an_output(file, &dir.join("answer.txt"), &mut unfilled);
    match (UNFILLED_TEMPLATE.contains(&file), unfilled.is_empty()) {
        (false, _) => bad.append(&mut unfilled),
        (true, false) => {}
        (true, true) => bad.push(format!(
            "{file}: no output prints an unfilled template reference, and UNFILLED_TEMPLATE \
             names it: it is fixed, so drop it from the list in the same commit"
        )),
    }
    members_of_n(file, dir, bad);
    *eager += eager_cores(file, dir, bad, held);
    held.add(no_core_unless_dead(file, dir, bad));
    let candidates = r.answer.candidates.clone();
    let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &candidates).expect("prepares");
    let again = dir.join("again");
    std::fs::create_dir_all(&again).expect("a scratch directory");
    let last = files.len().saturating_sub(1);
    // The last file that is a node, for its container.
    let mut node: Option<&str> = None;
    for (i, (path, set)) in files.iter().enumerate() {
        let saved = dir.join(path);
        // The run's own state, each fact with its origin.
        let facts: Vec<_> = set.iter().map(|&b| candidates[b as usize].fact).collect();
        let want = nodes
            .with_fixpoint(&facts, |kb, terms| {
                ein_cli::saturate::facts_section(terms, kb)
            })
            .expect("saturates");
        // The first load.
        let first = match section(&saved) {
            Ok(s) => s,
            Err(e) => {
                bad.push(format!("{file}: {path} does not load: {e}"));
                continue;
            }
        };
        let (whole, second) = (first.whole, first.saved);
        if first.facts == want {
            counts.states += 1;
        } else {
            bad.push(format!(
                "{file}: {path}: the first load is not the state the run saved:\n{}",
                first_difference(&want, &first.facts)
            ));
        }
        // Save → load → save.
        let text = std::fs::read_to_string(&saved).expect("the run wrote it");
        if text.contains("\n(commit\n") {
            node = Some(path.as_str());
        }
        if second == text {
            counts.bytes += 1;
        } else {
            bad.push(format!(
                "{file}: {path}: saved again, it is not the same bytes:\n{}",
                first_difference(&text, &second)
            ));
        }
        // Load → save → load.
        let path_again = again.join(format!("{i}.ein"));
        std::fs::write(&path_again, &second).expect("a scratch file");
        match section(&path_again) {
            Ok(d) if d.whole == whole => counts.dumps += 1,
            Ok(d) => bad.push(format!(
                "{file}: {path}: its second save loads to another dump:\n{}",
                first_difference(&whole, &d.whole)
            )),
            Err(e) => bad.push(format!(
                "{file}: {path}: its second save does not load: {e}"
            )),
        }
        // The binary, on the first and the last.
        if i == 0 || i == last {
            by_the_binary(file, &saved, &again.join(format!("b{i}")), &whole, bad);
            counts.cli += 1;
        }
    }
    #[cfg(feature = "einb")]
    if let Some(path) = node
        && by_its_container(file, &dir.join(path), &again, bad)
    {
        counts.containers += 1;
    }
    #[cfg(not(feature = "einb"))]
    let _ = node;
    let _ = std::fs::remove_dir_all(&again);
    Some(files.len())
}

/// Where two texts first differ: the line on each side.
fn first_difference(want: &str, got: &str) -> String {
    let (w, g): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
    let at = w
        .iter()
        .zip(&g)
        .position(|(a, b)| a != b)
        .unwrap_or(w.len().min(g.len()));
    format!(
        "  line {}: {:?}\n  where it should be {:?} ({} lines against {})",
        at + 1,
        g.get(at).copied().unwrap_or("<end>"),
        w.get(at).copied().unwrap_or("<end>"),
        g.len(),
        w.len()
    )
}

/// **No unfilled template reaches an output**: no line of `answer.txt` (the
/// read-out, a class's `:goal-text` filled) carries `{?`, which
/// [`ein_core::render_why`] leaves where a reference names nothing it binds.
/// A state file carries the program as written, `:why` templates and all,
/// and fills none. `ein-ir/tests/template_wellformedness.rs` holds the
/// cause, the template, and this holds the effect wherever a template is
/// filled.
fn no_placeholder_reaches_an_output(file: &str, path: &Path, bad: &mut Vec<String>) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    for line in text.lines().filter(|l| l.contains("{?")) {
        bad.push(format!(
            "{file}: {} prints an unfilled template reference: {line}",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
}

/// `nogoods.json`'s members, each with a core, its search's status and its
/// rules, past `--region-limit` too: never `not-kept`, and
/// never an empty core but on ∅, a dead root's, whose
/// refutation rests on no fact. A member that is not ∅ has a core naming its
/// commitment's facts, so `[]`, the empty set, is root's alone
/// (`empty-match-false.ein`'s, whose `:match ()` fires, Q-M1e2.36). Each
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

/// N's cores against the eager ones: where the run in `dir`
/// described R within `--region-limit`, `ein solve FILE --dump-lattice`, whose
/// walk takes every dead node's core as it visits it, and for each member of
/// `nogoods.json` that run visited, its core, status and rules in `nodes.json`
/// equal `nogoods.json`'s. A member the route reads off others and does not
/// visit (the state route's `N`, by duality) is not compared. How many were
/// compared. Every node of that `nodes.json` is held to its deadness too, a
/// dead one's core a list and every other's `null` ([`no_core_unless_dead`]).
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

/// **A node that is not dead has no core** (Q-M1e2.27): a core is one
/// object, the set of facts
/// a dead node's contradiction rests on, so a dead node's `core` and
/// `refuted_by` are lists, `[]` the empty set, and a node that is not dead,
/// whose core does not exist, has `null`. Held on every JSON surface that
/// writes a node's core: the summary's root and minimal holes
/// (`answer.json`, `index.json`, and `ein oracle --json-summary`),
/// `nogoods.json`, and `ein oracle --dump`'s `region.json`, `--dump-lattice`'s
/// `nodes.json` and `--events`' `enter` lines and `region` event, the last
/// four where the oracle answers at `--region-limit 10`, which bounds what
/// they cost, the `enter` lines node for node against `nodes.json`; [`eager_cores`] holds the
/// `nodes.json` of every other program it dumps. A node that is not dead
/// written `[]` would read as *the empty set of facts suffices*.
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

/// The shards [`shard`] splits the corpus into, one test each. By default no
/// program dominates them (`type-exclusivity/pets` writes the most); under
/// `EIN_CORPUS_SLOW=1` they put the five
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
    let mut counts = Counts::default();
    let entries = corpus();
    let selected: Vec<_> = entries
        .select(&[], None, slow)
        .into_iter()
        .filter(|e| slow || !e.heavy_dump)
        .collect();
    // The nightly's shard of them (`EIN_TEST_SHARD`), before the eight
    // tests split it.
    let selected = ein_corpus::shard::keep(selected, |e| e.path.clone());
    // The shard's entries on the corpus sweeps' pool, which
    // the eight shards share, each entry with its own tallies, folded back in
    // the entries' order.
    let mine: Vec<(usize, &ein_corpus::Entry)> = selected
        .iter()
        .enumerate()
        .filter(|(i, _)| i % SHARDS == k)
        .map(|(i, e)| (i, *e))
        .collect();
    let done = ein_corpus::pool::map(
        "answer_round_trip",
        &mine,
        |(_, e)| (e.path.clone(), "--dump-answer".to_string()),
        |&(i, e)| {
            let dir = root.join(format!("e{i}"));
            let (mut bad, mut eager, mut held) = (Vec::new(), 0, Held::default());
            let mut counts = Counts::default();
            let wrote = check(&e.path, &dir, &mut bad, &mut eager, &mut held, &mut counts);
            // A dump past the limit is tens of thousands of files.
            let _ = std::fs::remove_dir_all(&dir);
            (wrote, bad, eager, held, counts)
        },
    );
    for ((_, e), (wrote, b, n, h, c)) in mine.iter().zip(done) {
        bad.extend(b);
        eager += n;
        held.add(h);
        counts.add(&c);
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
    }
    // A shard of the nightly's holds its share of the programs and some
    // state; more states than programs, and the kinds of state, are each held
    // by the whole run, per commit. A nightly shard's eighth is a dozen or two
    // programs as the corpus deals them, and a dealing of many whose R is
    // empty or one state holds fewer states than programs with nothing wrong.
    let whole = ein_corpus::shard::current().is_none();
    assert!(
        programs > ein_corpus::shard::share(20)
            && states > 0
            && (!whole || states > programs)
            && (!whole || (eager > 0 && held.dead > 0 && held.not_dead > 0)),
        "shard {k}: {programs} programs, {states} states, {eager} members of N held to their \
         eager cores, {} dead nodes and {} others held to their deadness",
        held.dead,
        held.not_dead
    );
    eprintln!(
        "shard {k}: {programs} programs, {states} state files: {} loaded as the state saved, {} \
         saved again byte for byte, {} loaded again to the same dump, {} through the binary, {} \
         nodes' containers answering as the node; {eager} members of N held to their eager \
         cores, {} dead nodes and holes each with a core and {} others each with none",
        counts.states,
        counts.bytes,
        counts.dumps,
        counts.cli,
        counts.containers,
        held.dead,
        held.not_dead
    );
    assert!(
        !whole || !cfg!(feature = "einb") || counts.containers > 0,
        "shard {k}: no node's container was held to its file"
    );
    assert!(
        counts.states == states && counts.bytes == states && counts.dumps == states,
        "shard {k}: of {states} state files, {} loaded as the state saved, {} saved again byte \
         for byte, {} loaded again to the same dump:\n{}",
        counts.states,
        counts.bytes,
        counts.dumps,
        bad.join("\n")
    );
    assert!(
        bad.is_empty(),
        "shard {k}: {} of {states} state files over {programs} programs:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_0() {
    shard(0);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_1() {
    shard(1);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_2() {
    shard(2);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_3() {
    shard(3);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_4() {
    shard(4);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_5() {
    shard(5);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_6() {
    shard(6);
}

#[test]
fn every_state_file_is_saved_and_loaded_as_itself_7() {
    shard(7);
}

/// The witnesses: programs in the final grammar whose names exercise the
/// printer, written here, and `syntax/arg-kinds`. A state file writes each
/// choice as the printer spells the candidate, and a load reads it back to
/// the same candidate of H.
///
/// A string is no value (Q-M1e2.23) and there is no int: a run of digits is
/// a name (Q-M1e2.24). What the printer must get right is the name rule's
/// characters, so the
/// witnesses write `7` beside `07`, `-7` and `1..5`, every decorator, and the
/// brackets, as fact arguments, candidates, nested facts and derived facts.
#[test]
fn the_witnesses_are_saved_and_loaded_as_their_states() {
    let root = Scratch::new("ein-answer-round-trip-w");
    let digits = root.join("digits.ein");
    std::fs::write(
        &digits,
        ";; A run of digits is a name: `7`, `07`, `-7` and `1..5` are four.\n\
         (relation seen Thing :why \"{?3} is seen\")\n\
         (relation pick Thing :why \"{?3} is picked\")\n\
         (relation pair Thing Thing :why \"{?3} pairs with {?4}\")\n\
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
         (relation has Thing Thing :why \"{?3} has {?4}\")\n\
         (relation tag Thing :why \"{?3} is tagged\")\n\
         (relation kept Thing :why \"{?3} is kept\")\n\
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
            &mut Counts::default(),
        );
        states += wrote.unwrap_or_else(|| panic!("{file}: no state written"));
    }
    assert!(states >= 40, "{states}");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
