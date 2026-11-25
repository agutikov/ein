//! **The corpus as the test of the outcomes.** Every entry of
//! `corpus/corpus.toml` that is not `slow`, asked every question
//! `einaf::kernel` accepts, and each outcome held to what the corpus says of
//! the program: its group, the exit `ein solve` gives it, the claims it
//! states, and the engine's own static check. No value of a flagged
//! program's answer is asserted: of a flagged program only that it is
//! flagged.
//!
//! The sweep runs on the engine's worker pool (`ein_corpus::pool`,
//! `EIN_TEST_JOBS` workers), one unit a program: a program is loaded once,
//! and every question is asked of a copy of the load.
//!
//! **The cube's limit is lowered for the sweep** ([`LIMIT`]): the limit is
//! read by a program the static check flags and by no other, and a flagged
//! program's outcome is *flagged* whatever its cube is, so the lower limit
//! changes no outcome. It is what keeps the four largest flagged programs
//! from describing their cubes of 2¹⁶ nodes for nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use einaf::kernel::{
    Bounds, Checked, Claims, ClaimsChecked, Outcome, Part, Program, QueryClaim, Question, Run,
    SolveClaim, Stage, outcome,
};

/// The cube's limit the sweep asks under, the default being 16.
const LIMIT: u32 = 4;

/// Every question `einaf outcome` accepts.
const QUESTIONS: [Question; 8] = [
    Question::Describe,
    Question::Single(Part::All),
    Question::Single(Part::Minimal),
    Question::Single(Part::Maximal),
    Question::Line(Part::Minimal),
    Question::Line(Part::Maximal),
    Question::Count,
    Question::NoSearch,
];

/// What one program gave.
struct Found {
    path: String,
    group: String,
    /// The refusal's stage, where the load refused it.
    refused: Option<Stage>,
    /// The engine's static check, on a program that loaded.
    statically_flagged: Option<bool>,
    /// Every reading of every question: (question, part) → outcome.
    readings: Vec<(Question, Part, Outcome)>,
    claims: Claims,
    /// The kernel's check of the claims, on the describing run.
    checked: Option<ClaimsChecked>,
    /// An engine error, which no program should reach.
    errors: Vec<String>,
}

fn sweep() -> Vec<Found> {
    let root = ein_corpus::repo_root();
    let corpus = ein_corpus::corpus();
    let entries: Vec<_> = corpus.entry.iter().filter(|e| !e.slow).cloned().collect();
    let bounds = Bounds {
        limit: LIMIT,
        ..Bounds::default()
    };
    ein_corpus::pool::map(
        "einaf_outcomes",
        &entries,
        |e| (e.path.clone(), String::new()),
        |e| {
            let mut found = Found {
                path: e.path.clone(),
                group: e.group.clone(),
                refused: None,
                statically_flagged: None,
                readings: Vec::new(),
                claims: Claims::default(),
                checked: None,
                errors: Vec::new(),
            };
            let loaded = Program::load_file(&root.join(&e.path));
            match &loaded {
                Ok(p) => {
                    match p.statically_flagged() {
                        Ok(f) => found.statically_flagged = Some(f),
                        Err(err) => found.errors.push(format!("the static check: {err}")),
                    }
                    found.claims = p.claims();
                }
                Err(r) => found.refused = Some(r.stage),
            }
            for q in QUESTIONS {
                let run = match &loaded {
                    Ok(p) => match p.ask(q, bounds) {
                        Ok(run) => run,
                        Err(err) => {
                            found.errors.push(format!("{}: {err}", q.flags()));
                            continue;
                        }
                    },
                    Err(r) => Run::refused(q, r.clone()),
                };
                for part in q.readings() {
                    let r = outcome(&run, part).expect("a reading the question gives");
                    found.readings.push((q, part, r.outcome));
                }
                if q == Question::Describe {
                    found.checked = run.claims_checked();
                }
            }
            found
        },
    )
}

/// `ein solve FILE`'s exit for each corpus program that declares a bare
/// `solve` run, as the engine's own sweep banks it.
fn solve_exits() -> BTreeMap<String, i32> {
    let golden =
        ein_corpus::repo_root().join("ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt");
    let text = std::fs::read_to_string(&golden).expect("the corpus's exits");
    text.lines()
        .filter_map(|l| {
            let (code, rest) = l.split_once(char::is_whitespace)?;
            let path = rest.trim().strip_suffix(" :: solve")?;
            Some((path.to_string(), code.parse().ok()?))
        })
        .collect()
}

/// The outcomes a claim's shape allows over its part, where the claim holds.
fn allowed_by_solve_claim(c: SolveClaim) -> Option<BTreeSet<Outcome>> {
    use Outcome::*;
    Some(match c {
        SolveClaim::False => [Inconsistent].into(),
        SolveClaim::Region { owing: true, .. } => [Incomplete].into(),
        SolveClaim::Region { states: 0, .. } => [Inconsistent, Incomplete].into(),
        SolveClaim::Region { states: 1, .. } => [One].into(),
        SolveClaim::Region { .. } => [Several].into(),
        SolveClaim::Unread => return None,
    })
}

fn allowed_by_query_claim(c: QueryClaim) -> BTreeSet<Outcome> {
    use Outcome::*;
    match c.classes {
        // A query's `(false)`: no class, which is an empty R of either kind.
        0 => [Inconsistent, Incomplete].into(),
        // One class: a state at least, and states can share a class.
        1 => [One, Several].into(),
        // Two classes need two states.
        _ => [Several].into(),
    }
}

#[test]
fn every_corpus_program_has_one_outcome_a_question_and_it_agrees_with_the_corpus() {
    let found = sweep();
    let exits = solve_exits();
    let mut bad: Vec<String> = Vec::new();
    let mut tally: BTreeMap<(String, Part), BTreeMap<Outcome, usize>> = BTreeMap::new();
    let mut claims_compared = 0usize;
    let mut claims_not_held = 0usize;

    for f in &found {
        bad.extend(
            f.errors
                .iter()
                .map(|e| format!("{}: engine error under {e}", f.path)),
        );
        let of = |q: Question, p: Part| {
            f.readings
                .iter()
                .find(|(q2, p2, _)| *q2 == q && *p2 == p)
                .map(|r| r.2)
        };
        // One outcome a reading, and every reading the question gives.
        for q in QUESTIONS {
            for p in q.readings() {
                let n = f
                    .readings
                    .iter()
                    .filter(|(q2, p2, _)| *q2 == q && *p2 == p)
                    .count();
                if n != 1 && f.errors.is_empty() {
                    bad.push(format!(
                        "{}: {} {} has {n} outcomes",
                        f.path,
                        q.flags(),
                        p.words()
                    ));
                }
            }
        }
        for (q, p, o) in &f.readings {
            *tally
                .entry((q.flags(), *p))
                .or_default()
                .entry(*o)
                .or_default() += 1;
        }
        let outcomes: BTreeSet<Outcome> = f.readings.iter().map(|r| r.2).collect();

        // The group: a negative fixture is invalid under every question, a
        // parse-negative one at the parse and a load-negative one at the
        // parse or the load (the manifest keeps one whose refusal is the
        // grammar's among the load-negatives, the module three others
        // import); and only a refused load is invalid.
        let stages: &[Stage] = match f.group.as_str() {
            "parse-negative" => &[Stage::Parse],
            "load-negative" => &[Stage::Parse, Stage::Load],
            _ => &[],
        };
        if !stages.is_empty()
            && (outcomes != BTreeSet::from([Outcome::Invalid])
                || !f.refused.is_some_and(|s| stages.contains(&s)))
        {
            bad.push(format!(
                "{}: a {} entry, refused {:?}, reads {outcomes:?}",
                f.path, f.group, f.refused
            ));
        }
        if f.refused.is_some() != outcomes.contains(&Outcome::Invalid)
            || (f.refused.is_some() && outcomes.len() != 1)
        {
            bad.push(format!(
                "{}: refused {:?} and reads {outcomes:?}",
                f.path, f.refused
            ));
        }

        // The engine's static check: a program it flags is flagged under
        // every question, and nothing else; one that reads flagged and is
        // not statically flagged was found so in flight.
        if f.statically_flagged == Some(true) && outcomes != BTreeSet::from([Outcome::Flagged]) {
            bad.push(format!(
                "{}: the static check flags it, and it reads {outcomes:?}",
                f.path
            ));
        }
        if f.statically_flagged == Some(false) && outcomes.contains(&Outcome::Flagged) {
            // In flight: a hypothesis held above root and not at root. Such
            // a program reads flagged under the questions that build that
            // state; the corpus has none today, and one would be named here.
            bad.push(format!(
                "{}: not flagged by the static check, and reads flagged (found in flight?)",
                f.path
            ));
        }

        // The exit `ein solve` gives it: invalid exits non-zero, and a
        // non-zero exit is invalid or a claim the run refuted.
        if let Some(&exit) = exits.get(&f.path) {
            let invalid = f.refused.is_some();
            let refuted = f.checked.as_ref().is_some_and(|c| {
                c.solve == Some(Checked::Failed) || c.queries.contains(&Some(Checked::Failed))
            });
            if invalid && exit == 0 {
                bad.push(format!("{}: invalid, and `ein solve` exits 0", f.path));
            }
            if !invalid && exit != 0 && !refuted {
                bad.push(format!(
                    "{}: `ein solve` exits {exit}, and it is neither invalid nor refutes a claim",
                    f.path
                ));
            }
        }

        // Questions that read one part agree wherever both decided it: the
        // outcome is the program's, not the route's.
        let groups: [(Part, &[Question]); 3] = [
            (
                Part::All,
                &[
                    Question::Describe,
                    Question::Single(Part::All),
                    Question::Count,
                ],
            ),
            (
                Part::Minimal,
                &[
                    Question::Describe,
                    Question::Single(Part::Minimal),
                    Question::Line(Part::Minimal),
                ],
            ),
            (
                Part::Maximal,
                &[
                    Question::Describe,
                    Question::Single(Part::Maximal),
                    Question::Line(Part::Maximal),
                ],
            ),
        ];
        for (part, qs) in groups {
            let decided: BTreeSet<Outcome> = qs
                .iter()
                .filter_map(|&q| of(q, part))
                .filter(|o| *o != Outcome::NotDecided)
                .collect();
            if decided.len() > 1 {
                bad.push(format!(
                    "{}: {} the questions read {decided:?}",
                    f.path,
                    part.words()
                ));
            }
        }
        // R empty is R empty over every part, and `--no-search` says so of a
        // dead root alone.
        let empty: BTreeSet<Outcome> = f
            .readings
            .iter()
            .filter(|(q, _, o)| *q != Question::NoSearch && *o != Outcome::NotDecided)
            .map(|r| r.2)
            .filter(|o| matches!(o, Outcome::Inconsistent | Outcome::Incomplete))
            .collect();
        let answers: BTreeSet<Outcome> = f
            .readings
            .iter()
            .filter(|(q, _, o)| *q != Question::NoSearch && o.is_answer())
            .map(|r| r.2)
            .collect();
        if !empty.is_empty() && answers != empty {
            bad.push(format!(
                "{}: R is empty under one question and not another: {answers:?}",
                f.path
            ));
        }
        if let Some(ns) = of(Question::NoSearch, Part::All)
            && ns == Outcome::Inconsistent
            && of(Question::Describe, Part::All) != Some(Outcome::Inconsistent)
        {
            bad.push(format!(
                "{}: --no-search reads a dead root, describe does not",
                f.path
            ));
        }

        // The claims the program states, where the kernel holds them on
        // the describing run: the outcome over the claim's part is one the
        // claim's shape allows. A flagged program is left out: nothing is
        // claimed of its answer.
        if outcomes.contains(&Outcome::Flagged) || f.refused.is_some() {
            continue;
        }
        let checked = f.checked.clone().unwrap_or_default();
        if let (Some(c), Some(k)) = (f.claims.solve, checked.solve) {
            if k == Checked::Held {
                claims_compared += 1;
                let o = of(Question::Describe, Part::All).expect("describe's reading");
                if let Some(allowed) = allowed_by_solve_claim(c)
                    && o != Outcome::NotDecided
                    && !allowed.contains(&o)
                {
                    bad.push(format!(
                        "{}: (solve :expect …) {c:?} holds, and it reads {o:?}",
                        f.path
                    ));
                }
            } else {
                claims_not_held += 1;
            }
        }
        for (c, k) in f.claims.queries.iter().zip(&checked.queries) {
            let (Some(c), Some(k)) = (c, k) else {
                continue;
            };
            if *k != Checked::Held {
                claims_not_held += 1;
                continue;
            }
            claims_compared += 1;
            let o = of(Question::Describe, c.over).expect("describe's reading");
            if o != Outcome::NotDecided && !allowed_by_query_claim(*c).contains(&o) {
                bad.push(format!(
                    "{}: (query :expect …) {c:?} holds, and it reads {o:?}",
                    f.path
                ));
            }
        }
    }

    // The count, for the record: every outcome a question reads, by reading.
    let mut stages: BTreeMap<String, usize> = BTreeMap::new();
    for f in &found {
        if let Some(s) = f.refused {
            *stages.entry(format!("{s:?}")).or_default() += 1;
        }
    }
    eprintln!(
        "{} programs; {} refused, by stage {stages:?}; {} flagged by the static check; \
         {} with a bare `solve` exit compared; {claims_compared} claims compared, \
         {claims_not_held} not held on the describing run",
        found.len(),
        found.iter().filter(|f| f.refused.is_some()).count(),
        found
            .iter()
            .filter(|f| f.statically_flagged == Some(true))
            .count(),
        found.iter().filter(|f| exits.contains_key(&f.path)).count(),
    );
    for ((q, p), by) in &tally {
        let line: Vec<String> = Outcome::ALL
            .iter()
            .map(|o| format!("{} {}", o.word(), by.get(o).copied().unwrap_or(0)))
            .collect();
        eprintln!("  {q:<20} {:<22} {}", p.words(), line.join(" · "));
    }

    // Not vacuous: the sweep met every outcome, and compared claims.
    let met: BTreeSet<Outcome> = tally.values().flat_map(|by| by.keys().copied()).collect();
    assert_eq!(
        met,
        Outcome::ALL.into_iter().collect::<BTreeSet<_>>(),
        "the corpus sweep did not meet every outcome"
    );
    assert!(claims_compared > 0, "no claim was compared");
    assert!(found.len() > 800, "the sweep read {} programs", found.len());
    assert!(
        bad.is_empty(),
        "{} findings:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// The sweep reads `corpus/corpus.toml`'s entries through the engine's own
/// manifest reader: a path the manifest names is a file.
#[test]
fn the_corpus_entries_are_files() {
    let root = ein_corpus::repo_root();
    for e in ein_corpus::corpus().entry {
        assert!(Path::new(&root.join(&e.path)).is_file(), "{}", e.path);
    }
}
