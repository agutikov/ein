//! `ein solve --dump-answer DIR` — **every answer a file** (M1e2
//! [S1e2.1.2](../../../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.2_every_answer_a_file.md)).
//!
//! What a solve answered used to be three partial copies: stdout, which is
//! readable and not loadable; `--json-summary`, which says *what* and not
//! *how*; and `--dump-states`, which is root per layer — the search's frontier,
//! not its answer. The engine held the rest in the `store_lattice` proof and
//! nothing wrote it out. This does, **after** the search and from the proof, so
//! it cannot perturb the search: it is a function over a finished
//! [`Solved`], not a [`Dumper`](ein_infer::solve::Dumper).
//!
//! ```text
//! DIR/
//!   index.json          what is here: the verdict, and every file below by name
//!   program.ein         the source as loaded, imports inlined
//!   root.ein            program + root's facts after its saturation, before any hypothesis
//!   root_writes.json    what the search wrote into root after that (kill cache, writeback, forced positive)
//!   answer.json         byte-identical to --json-summary on the same run
//!   models/NN.ein       program + one recorded model's facts — loadable, and a fixpoint
//!   models/NN.json      {commitment, layer, reported, goal_bindings, owes, added, facts}
//!   open/NN.{ein,json}  the same, per recorded state that still owes
//!   refuted/NN.json     {commitment, layer, kind, unsat_core, learned_clause}
//!   alive_at_end.json   the frontier the depth cap left — [] when the search exhausted
//! ```
//!
//! The format, and what each file is *for*, is
//! [`docs/kernel/inference/answer_dump.md`](../../../../../docs/kernel/inference/answer_dump.md).
//!
//! # Two spellings of a fact, on purpose
//!
//! A `commitment` is the timeline's `{relation, args}` list
//! ([`super::state::commitment_json`]), so a row of `--dump-states`'
//! `00_timeline.jsonl` and the record here compare as equal values. Every
//! other fact list — `added`, `unsat_core`, `learned_clause`, the model facts —
//! is the summary's sorted s-expression, so it compares with
//! `verdict.solutions[].facts`. [`facts_json`], [`bindings_json`] and
//! [`owes_json`] are those spellings, and `ein-cli`'s summary calls the same
//! three: one implementation of how a model is written down.

use std::path::Path;

use ein_core::{FactId, Kb, Terms, Value};
use ein_infer::events::{sexpr, sexpr_value};
use ein_infer::obligations::Owes;
use ein_infer::solve::{LatticeProof, SolutionRecord, Solved};
use ein_infer::verdict::{Answer, Verdict, goal_bindings};
use ein_ir::Ast;

use super::json::{Json, dumps_indent};
use super::serialise::facts_to_ein_text;
use super::state::commitment_json;

/// The format's name, in `index.json`.
pub const SCHEMA: &str = "ein-answer/1";

/// A KB's facts as sorted s-expressions — the model, as a set. The spelling
/// of `--json-summary`'s `verdict.solutions[].facts`.
pub fn facts_json(terms: &Terms, kb: &Kb) -> Json {
    sorted_facts(terms, kb.facts())
}

fn sorted_facts(terms: &Terms, facts: impl IntoIterator<Item = FactId>) -> Json {
    let mut out: Vec<String> = facts.into_iter().map(|f| sexpr(terms, f)).collect();
    out.sort();
    Json::Array(out.into_iter().map(Json::Str).collect())
}

/// One goal binding, as `json.dumps` writes ein.py's.
///
/// `goal_bindings` returns the **stored** argument, and ein.py stores an IR
/// `INT` as a Python `int` — so `json.dumps` writes it as a *number*. Every
/// binding used to go through `sexpr_value`, which made it a string, and
/// `summary.json` is T0: `(query :goal (r1 ?x ?y))` over `(r1 o3 8)` was a
/// verdict-level difference on a two-line program. Found by
/// [S1a.6.6](../../../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer)'s
/// fuzzer, 2026-08-20; `examples/ein-bugs/int-goal-binding.ein` is the
/// fixture. A symbol stays a string and a nested fact its s-expression.
fn binding_value(terms: &Terms, v: Value) -> Json {
    match v.as_int() {
        Some(id) => {
            let text = terms.int_text(id);
            text.parse::<i64>()
                .map_or_else(|_| Json::BigInt(text.to_string()), Json::Int)
        }
        None => Json::Str(sexpr_value(terms, v)),
    }
}

/// `(query :goal …)` binding rows, sorted; each row's keys sorted too.
pub fn bindings_json(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Json {
    let rows = goal_bindings(ast, terms, kb, None);
    let mut shown: Vec<Vec<(String, Value)>> = rows
        .iter()
        .map(|row| {
            let mut r: Vec<(String, Value)> = row
                .iter()
                .map(|(k, v)| (terms.sym(*k).to_string(), *v))
                .collect();
            r.sort_by(|a, b| a.0.cmp(&b.0));
            r
        })
        .collect();
    // Rows are ordered by their rendered form, which is what ein.py's
    // `sorted(rows, key=…)` compares.
    shown.sort_by_key(|r| {
        r.iter()
            .map(|(k, v)| (k.clone(), sexpr_value(terms, *v)))
            .collect::<Vec<_>>()
    });
    Json::Array(
        shown
            .into_iter()
            .map(|r| {
                Json::Object(
                    r.into_iter()
                        .map(|(k, v)| (k, binding_value(terms, v)))
                        .collect(),
                )
            })
            .collect(),
    )
}

/// One tally of outstanding obligations.
///
/// `total` is the instance count; `by_relation` attributes it, which only
/// `(open ?R)` can do — a bare `(open)` counts and names no slot, so the two
/// numbers agree exactly when every obligation rule in the program names its
/// relation. `instances` is the report itself, one row per debt in report
/// order, carrying the rendered `:why` a reader actually wants.
pub fn owes_json(terms: &Terms, owes: &Owes) -> Json {
    let by_relation: Vec<(String, Json)> = owes
        .by_relation()
        .into_iter()
        .map(|(r, n)| (terms.sym(r).to_string(), Json::int(n as i64)))
        .collect();
    let instances: Vec<Json> = owes
        .instances()
        .iter()
        .map(|i| {
            let mut b: Vec<(String, Json)> = i
                .bindings
                .iter()
                .map(|(k, v)| (terms.sym(*k).to_string(), Json::Str(sexpr_value(terms, *v))))
                .collect();
            b.sort_by(|a, b| a.0.cmp(&b.0));
            Json::obj(vec![
                ("rule", Json::str(terms.sym(i.rule))),
                (
                    "relation",
                    match i.relation {
                        Some(r) => Json::str(terms.sym(r)),
                        None => Json::Null,
                    },
                ),
                ("bindings", Json::Object(b)),
                ("why", Json::str(&i.why)),
            ])
        })
        .collect();
    Json::obj(vec![
        ("total", Json::int(owes.total() as i64)),
        ("by_relation", Json::Object(by_relation)),
        ("instances", Json::Array(instances)),
    ])
}

/// What the caller supplies that the proof does not hold.
pub struct AnswerInputs<'a> {
    /// The input path, echoed into `index.json` and every state file's header.
    pub source: &'a str,
    /// The program as loaded with its imports inlined — `program.ein`, and the
    /// head of every state file, which is what makes one loadable.
    pub program: &'a str,
    /// The `--json-summary` bytes for this run, which `answer.json` is.
    pub summary: &'a str,
}

/// Is `f` one of a relation's schema facts, `(relation R …)`?
///
/// The program text at the head of a state file declares every relation, and
/// the loader refuses a second declaration — which is why `--dump-states`'
/// snapshots, which write both `(relation R T…)` and `(relation R)`, do not
/// load (found at S1e2.1.1). A state file leaves them to the program.
fn is_schema(terms: &Terms, f: FactId) -> bool {
    let (rel, _) = terms.fact(f);
    terms.sym(rel) == "relation"
}

/// A state as a loadable program: a header saying what it is, the program,
/// then every fact of the state but the schema, sorted.
fn state_file(
    terms: &Terms,
    kb: &Kb,
    facts: &[FactId],
    inputs: &AnswerInputs,
    what: &str,
    commitment: &[FactId],
) -> String {
    let mut state: Vec<FactId> = facts
        .iter()
        .copied()
        .filter(|&f| !is_schema(terms, f))
        .collect();
    state.sort_by_cached_key(|&f| sexpr(terms, f));
    let commit = if commitment.is_empty() {
        "∅".to_string()
    } else {
        commitment
            .iter()
            .map(|&f| sexpr(terms, f))
            .collect::<Vec<_>>()
            .join(" ")
    };
    format!(
        ";;; {what} — written by `ein solve --dump-answer` (M1e2 S1e2.1.2).\n\
         ;;; source:     {}\n\
         ;;; commitment: {commit}\n\
         ;;;\n\
         ;;; Below: the program as loaded, imports inlined, and then every fact of\n\
         ;;; the state ({} facts; the program's own relation declarations are not\n\
         ;;; repeated). It loads, and saturating it derives nothing new when the\n\
         ;;; state is a fixpoint of the program — docs/kernel/inference/answer_dump.md.\n\n\
         {}\n\
         ;; ── the state ──────────────────────────────────────────────────────────\n\n\
         {}",
        inputs.source,
        state.len(),
        inputs.program.trim_end(),
        facts_to_ein_text(kb, terms, &state),
    )
}

fn write(dir: &Path, name: &str, text: &str) -> std::io::Result<()> {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, text).map_err(|e| std::io::Error::new(e.kind(), format!("{name}: {e}")))
}

fn write_json(dir: &Path, name: &str, value: &Json) -> std::io::Result<()> {
    write(dir, name, &(dumps_indent(value) + "\n"))
}

/// A file name's number: `01`, `02`, … — two digits, or as many as the count
/// needs, so the files sort the way they are numbered.
fn nn(i: usize, of: usize) -> String {
    let width = of.to_string().len().max(2);
    format!("{:0width$}", i + 1)
}

/// Which recorded nodes the answer reports as models and which as open states
/// — the partition `Run::finalise` makes, made again from the proof.
///
/// A program that states an obligation is judged by discharge: a node that
/// still owes is an open state. Where the search found both kinds, the answer
/// is the models and the owing nodes are *not reported* — they are still
/// written, under `open/`, with `reported: false`, because a reviewer asking
/// *what did the search find* wants them and the read-out does not show them.
fn partition(proof: &LatticeProof, scoped: bool) -> (Vec<&SolutionRecord>, Vec<&SolutionRecord>) {
    proof
        .solutions
        .iter()
        .partition(|r| !(scoped && !r.owes.is_empty()))
}

/// Write the answer directory for one finished solve.
///
/// `root` is the KB the solve ran on — root at the end of the search, whose
/// provenance every recorded state inherits. `solved.proof` must be present:
/// the CLI sets `store_lattice` whenever `--dump-answer` is given.
pub fn write_answer_dir(
    dir: &Path,
    ast: &Ast,
    terms: &mut Terms,
    root: &Kb,
    solved: &Solved,
    inputs: &AnswerInputs,
) -> std::io::Result<()> {
    let proof = solved.proof.as_ref().ok_or_else(|| {
        std::io::Error::other("the solve kept no proof — --dump-answer needs store_lattice")
    })?;
    std::fs::create_dir_all(dir)?;
    write(
        dir,
        "program.ein",
        &(inputs.program.trim_end().to_string() + "\n"),
    )?;
    write(dir, "answer.json", inputs.summary)?;
    write(
        dir,
        "root.ein",
        &state_file(
            terms,
            root,
            &proof.root_initial,
            inputs,
            "root, after its saturation and before any hypothesis",
            &[],
        ),
    )?;

    // What the search wrote into root after it started — every fact of the
    // final root that the saturated one did not have, with the rule (or the
    // engine's reserved name) its provenance carries.
    let initial: rustc_hash::FxHashSet<FactId> = proof.root_initial.iter().copied().collect();
    let mut root_writes: Vec<(String, Json)> = root
        .facts()
        .filter(|f| !initial.contains(f))
        .map(|f| {
            let prov = root.primary(f).map(|p| terms.provs.get(p));
            let by = prov
                .and_then(|p| p.rule)
                .map_or(Json::Null, |r| Json::str(terms.sym(r)));
            let s = sexpr(terms, f);
            (
                s.clone(),
                Json::obj(vec![("fact", Json::Str(s)), ("by", by)]),
            )
        })
        .collect();
    root_writes.sort_by(|a, b| a.0.cmp(&b.0));
    write_json(
        dir,
        "root_writes.json",
        &Json::Array(root_writes.into_iter().map(|(_, j)| j).collect()),
    )?;

    let scoped = !root.program().obligations.is_empty();
    let (models, open) = partition(proof, scoped);
    let reported_open = matches!(&solved.answer, Answer::Verdict(Verdict::Open { .. }));
    let mut index_models = Vec::new();
    let mut index_open = Vec::new();
    for (kind, records, reported, index) in [
        ("model", &models, true, &mut index_models),
        ("open state", &open, reported_open, &mut index_open),
    ] {
        let sub = if kind == "model" { "models" } else { "open" };
        for (i, r) in records.iter().enumerate() {
            let stem = format!("{sub}/{}", nn(i, records.len()));
            let facts: Vec<FactId> = r.kb.facts().collect();
            let what = format!("{kind} {} of {}", i + 1, records.len());
            write(
                dir,
                &format!("{stem}.ein"),
                &state_file(terms, &r.kb, &facts, inputs, &what, &r.commitment),
            )?;
            let added = sorted_facts(
                terms,
                facts.iter().copied().filter(|f| !initial.contains(f)),
            );
            let bindings = bindings_json(ast, terms, &r.kb);
            write_json(
                dir,
                &format!("{stem}.json"),
                &Json::obj(vec![
                    ("commitment", commitment_json(terms, &r.commitment)),
                    ("layer", Json::int(r.layer)),
                    ("reported", Json::Bool(reported)),
                    ("goal_bindings", bindings),
                    ("owes", owes_json(terms, &r.owes)),
                    ("added", added),
                    ("facts", sorted_facts(terms, facts.iter().copied())),
                ]),
            )?;
            index.push(Json::str(stem));
        }
    }

    for (i, d) in proof.dead_commitments.iter().enumerate() {
        write_json(
            dir,
            &format!("refuted/{}.json", nn(i, proof.dead_commitments.len())),
            &Json::obj(vec![
                ("commitment", commitment_json(terms, &d.commitment)),
                ("layer", Json::int(d.layer)),
                ("kind", Json::str(d.kind.as_str())),
                (
                    "unsat_core",
                    sorted_facts(terms, d.unsat_core.iter().copied()),
                ),
                (
                    "learned_clause",
                    sorted_facts(terms, d.learned_clause.iter().copied()),
                ),
            ]),
        )?;
    }
    write_json(
        dir,
        "alive_at_end.json",
        &Json::Array(
            proof
                .alive_at_end
                .iter()
                .map(|c| commitment_json(terms, c))
                .collect(),
        ),
    )?;

    let verdict = solved.answer.as_str().to_string();
    let k = solved.answer.k(solved.stats.solution_nodes);
    write_json(
        dir,
        "index.json",
        &Json::obj(vec![
            ("schema", Json::str(SCHEMA)),
            ("source", Json::str(inputs.source)),
            ("verdict", Json::Str(verdict)),
            ("k", Json::int(k as i64)),
            ("exhausted", Json::Bool(solved.stats.exhausted)),
            (
                "root_initial_facts",
                Json::int(proof.root_initial.len() as i64),
            ),
            ("models", Json::Array(index_models)),
            ("open", Json::Array(index_open)),
            ("refuted", Json::int(proof.dead_commitments.len() as i64)),
            ("alive_at_end", Json::int(proof.alive_at_end.len() as i64)),
        ]),
    )?;
    Ok(())
}

/// The answer directory for a run a budget cut short: no proof, so no states
/// — `answer.json` and an `index.json` that says why, rather than a directory
/// a reader has to guess is empty on purpose.
pub fn write_aborted_answer_dir(
    dir: &Path,
    inputs: &AnswerInputs,
    reason: &str,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    write(
        dir,
        "program.ein",
        &(inputs.program.trim_end().to_string() + "\n"),
    )?;
    write(dir, "answer.json", inputs.summary)?;
    write_json(
        dir,
        "index.json",
        &Json::obj(vec![
            ("schema", Json::str(SCHEMA)),
            ("source", Json::str(inputs.source)),
            ("verdict", Json::str("Aborted")),
            ("reason", Json::str(reason)),
            ("models", Json::Array(Vec::new())),
            ("open", Json::Array(Vec::new())),
        ]),
    )
}

/// Clear what a previous `--dump-answer` into the same directory left, so a
/// run with three models does not sit beside a fourth from the last one.
///
/// Only a directory whose `index.json` names this format is touched, and only
/// the three subtrees this writer owns: pointing the flag at a directory that
/// holds something else writes beside it and deletes nothing.
pub fn clear_previous(dir: &Path) -> std::io::Result<()> {
    let ours = std::fs::read_to_string(dir.join("index.json"))
        .is_ok_and(|t| t.contains(&format!("\"schema\": \"{SCHEMA}\"")));
    if ours {
        for sub in ["models", "open", "refuted"] {
            match std::fs::remove_dir_all(dir.join(sub)) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
                _ => {}
            }
        }
    }
    Ok(())
}
