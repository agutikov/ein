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
//! **And no member of N is written with an empty core** (M1e2 S1e2.3.9,
//! P1e2.3's N6): the same run's `nogoods.json` gives each member of N it met
//! a core, its search's status beside it (`within-budget`, `cut-…`), or, on a
//! run stopped past `--region-limit`, which keeps each node it met as its two
//! bits, no core, no rules and `core_status: "not-kept"`. An empty list would
//! read as *refuted by nothing*, and until S1e2.3.9 it was what such a run
//! wrote: `lattice/06_friends_tables`, 68 of 68, and every program past the
//! limit the per-commit tier dumps.
//!
//! ```text
//! cargo test -p ein-cli --test answer_round_trip
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test answer_round_trip  # + the slow entries
//! ```

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use ein_core::{Kb, ProvKind, Terms};
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
/// `answer_dump.rs`'s copy.
const DO_NOT_LOAD: [&str; 3] = [
    "examples/ein-bugs/wrong-answers/assert-empty.ein",
    "examples/ein-bugs/wrong-answers/assert-neq-w-x.ein",
    "examples/ein-bugs/wrong-answers/assert-neq-x-x.ein",
];

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
/// program does not load or the run writes no state.
fn check(file: &str, dir: &Path, bad: &mut Vec<String>) -> Option<usize> {
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
    members_of_n(file, dir, bad);
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

/// `nogoods.json`'s members, each with a core and its search's status, or,
/// where the run stopped at `--region-limit`, with no core, no rules and
/// `not-kept`: never an empty core (M1e2 S1e2.3.9). Each failure goes to
/// `bad`.
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
        let ok = match (core.as_array(), status) {
            (Some(c), _) => !c.is_empty() && searched && rules.is_array(),
            (None, Some("not-kept")) => core.is_null() && rules.is_null() && stopped_at_limit,
            (None, _) => false,
        };
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

/// The shards [`shard`] splits the corpus into, one test each, so the four
/// programs whose dump is past `--region-limit` (65 214 state files on
/// `square-unique/corner-house`) run beside the rest and not after them.
const SHARDS: usize = 8;

/// Every corpus program `ein solve` answers, whether or not its entry declares
/// the run, every state file of its describe run (the listed states, the
/// members met and not listed, and an empty R's owing frontier), each against
/// the run's own state, by value: the entries whose manifest index is `k`
/// modulo [`SHARDS`].
fn shard(k: usize) {
    let root = Scratch::new(&format!("ein-answer-round-trip-{k}"));
    let slow = include_slow();
    let list = std::env::var("EIN_ROUND_TRIP_LIST").is_ok();
    let mut bad = Vec::new();
    let (mut programs, mut states) = (0, 0);
    let entries = corpus();
    for (i, e) in entries.select(&[], None, slow).iter().enumerate() {
        if i % SHARDS != k {
            continue;
        }
        let dir = root.join(format!("e{i}"));
        match check(&e.path, &dir, &mut bad) {
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
        programs > 20 && states > programs,
        "shard {k}: {programs} programs, {states} states"
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
        let wrote = check(file, &root.join(format!("w{i}")), &mut bad);
        states += wrote.unwrap_or_else(|| panic!("{file}: no state written"));
    }
    assert!(states >= 40, "{states}");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
