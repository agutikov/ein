//! The `solve` answer and its table — `ein.py`'s `trace/answer.py`.
//!
//! **No hardcoded vocabulary.** Every word of English here comes from
//! *puzzle-authored* templates; there is no relation→verb table. Two template
//! sources drive the text, both through the rule `:why` engine
//! ([`ein_core::render_why`]):
//!
//! - **per-relation** — `(relation R T1 T2 … :why "<tmpl>")` renders one fact
//!   of `R`, with `{?1}` / `{?2}` bound to its arguments *positionally*. That
//!   drives the table's *rendered query facts* column. A relation without a
//!   `:why` renders as its raw IR s-expression `(R a b)` — never invented prose.
//! - **per-query** — `(query … :goal-text "<tmpl>")` renders the headline NL
//!   result, with the goal's own variables bound from the solution. Absent,
//!   the result line says so.
//!
//! [`render_solution_table`] assembles the five fields the CLI prints —
//! solutions (k) · verdict · query bindings · rendered query facts · NL result
//! — for each verdict shape. [`render_answer`] is the one-line headline.

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_infer::obligations::Owes;
use ein_infer::verdict::{Answer, Verdict};
use ein_infer::{goal_bindings, query_value};
use ein_ir::{Ast, Node, NodeId};

use ein_core::render_why;

/// One binding row: variable name → its value, rendered.
type Row = Vec<(String, String)>;

fn row_of(terms: &Terms, raw: &[(Symbol, Value)]) -> Row {
    raw.iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
        .collect()
}

/// A fact as a flat IR s-expression — the no-template fallback.
fn sexpr(relation_name: &str, args: &[String]) -> String {
    if args.is_empty() {
        format!("({relation_name})")
    } else {
        format!("({relation_name} {})", args.join(" "))
    }
}

/// The conjuncts of a goal — a top-level `(and …)` unwrapped.
fn conjuncts(ast: &Ast, goal: Option<NodeId>) -> Vec<NodeId> {
    let Some(goal) = goal else { return Vec::new() };
    if let Node::SForm { head, args } = ast.node(goal)
        && matches!(ast.node(head), Node::Atom(s) if ast.sym(s) == "and")
    {
        return ast.args(args).to_vec();
    }
    vec![goal]
}

/// `(rel, ground_args)` for one goal conjunct under `b`.
///
/// A variable resolves to its bound value (an unbound one keeps its `?name`);
/// atoms and integers stay literal. `None` for a non-fact conjunct.
fn ground(ast: &Ast, conj: NodeId, b: &Row) -> Option<(String, Vec<String>)> {
    let Node::SForm { head, args } = ast.node(conj) else {
        return None;
    };
    let Node::Atom(rel) = ast.node(head) else {
        return None;
    };
    let out: Vec<String> = ast
        .args(args)
        .iter()
        .map(|a| match ast.node(*a) {
            Node::Var(s) => {
                let name = ast.sym(s);
                b.iter()
                    .find(|(k, _)| k == name)
                    .map(|(_, v)| v.clone())
                    .unwrap_or_else(|| format!("?{name}"))
            }
            Node::Atom(s) => ast.sym(s).to_string(),
            Node::Int(s) => ast.sym(s).to_string(),
            _ => ein_ir::node_repr(ast, *a),
        })
        .collect();
    Some((ast.sym(rel).to_string(), out))
}

/// One ground fact as text, through its relation's `:why` template, or its IR
/// s-expression when the relation has none.
fn render_fact(kb: Option<&Kb>, terms: &Terms, relation_name: &str, args: &[String]) -> String {
    let why = kb
        .zip(terms.syms.get(relation_name))
        .and_then(|(kb, s)| kb.program().relations.get(s))
        .and_then(|r| r.why)
        .map(|w| terms.sym(w).to_string())
        .filter(|w| !w.is_empty());
    match why {
        Some(tmpl) => {
            let slots: Vec<(String, String)> = args
                .iter()
                .enumerate()
                .map(|(i, a)| ((i + 1).to_string(), a.clone()))
                .collect();
            render_why(&tmpl, &slots)
        }
        None => sexpr(relation_name, args),
    }
}

/// The query's `:goal-text` template rendered under `b`.
fn goal_text(ast: &Ast, kb: Option<&Kb>, b: &Row) -> Option<String> {
    let query = kb?.program().query()?;
    let node = query_value(ast, query, "goal-text")?;
    let Node::Str(s) = ast.node(node) else {
        return None;
    };
    Some(render_why(ast.sym(s), b))
}

fn query_goal(ast: &Ast, kb: Option<&Kb>) -> Option<NodeId> {
    let query = kb?.program().query()?;
    query_value(ast, query, "goal")
}

/// The `:source` sentences of an unsat core, sorted and deduplicated.
fn core_sources(kb: &Kb, terms: &Terms, core: &[FactId]) -> Vec<String> {
    let mut out: Vec<String> = core
        .iter()
        .filter_map(|f| {
            let prov = terms.provs.get(kb.primary(*f)?);
            if prov.kind != ein_core::ProvKind::Source {
                return None;
            }
            prov.source
                .map(|s| terms.sym(s))
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

// ── The headline ───────────────────────────────────────────────────

/// A one-line headline for a verdict.
///
/// `Solution` renders the `:goal-text` against the solution bindings, or a
/// neutral "Solved." when there is none — no invented prose. `Ambiguity` and
/// `Contradiction` keep their stable wording.
pub fn render_answer(
    ast: &Ast,
    terms: &mut Terms,
    root: &Kb,
    answer: &Answer,
    exhausted: bool,
) -> String {
    match answer {
        Answer::Verdict(Verdict::Solution(s)) => {
            let goal = query_goal(ast, Some(&s.kb));
            let rows = goal_bindings(ast, terms, &s.kb, goal);
            let text = rows
                .first()
                .and_then(|r| goal_text(ast, Some(&s.kb), &row_of(terms, r)));
            match text {
                None => "Solved.".to_string(),
                Some(mut text) => {
                    if !exhausted {
                        text.push_str("  (a solution — pass --exhaustive to certify uniqueness)");
                    }
                    text
                }
            }
        }
        Answer::Verdict(v @ Verdict::Ambiguity(_)) => {
            let k = v.read_out(exhausted).k;
            // **The count is a claim, and `exhausted` is what licenses it.**
            // With the lattice exhausted these *are* the models; without it
            // they are the models found, and a deeper layer may hold more —
            // `saturation/type-exclusivity/colors.ein -e` says 5 at the
            // default cap and has 9 at `-m 6`. M1d
            // [S1d.3.3](../../../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
            // T1d.3.3.2: a `Solution` has qualified itself since ein.py and
            // the verdict that reports a model *set* did not.
            if exhausted {
                format!("Ambiguous — {k} distinct complete models; the puzzle is under-determined.")
            } else {
                format!(
                    "Ambiguous — at least {k} distinct complete models; \
                     the search did not exhaust the lattice."
                )
            }
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            let srcs = core_sources(root, terms, unsat_core);
            let core = if srcs.is_empty() {
                format!("{} facts", unsat_core.len())
            } else {
                srcs.join(", ")
            };
            // **`k = 0` is a claim too, and `exhausted` is what licenses it**
            // — M1d T1d.10.5.2b, finishing the table
            // [S1d.3.3](../../../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
            // gave `Solution` and `Ambiguity` and deliberately did not give
            // this arm. A refutation needs the lattice exhausted; without it
            // the zero says *no model within the cap*, which is what
            // `saturation/type-exclusivity/pets.ein` demonstrates by saying
            // "contradictory" at `-m 5`…`-m 8` and holding **35 models** at
            // `-m 10`. The core stays in the sentence and changes its name:
            // those commitments really are refuted, they just do not refute
            // the program.
            if exhausted {
                format!("No solution — the constraints are contradictory (unsat core: {core}).")
            } else {
                format!(
                    "No model found — the search did not exhaust the lattice                      (refuted so far: {core})."
                )
            }
        }
        Answer::Verdict(Verdict::Open { owes, .. }) => {
            // The word the other three cannot say: consistent, quiescent, and
            // owed. `Contradiction` would claim a refutation nothing derived
            // and `Solution` would claim a model nothing witnessed — M1d
            // S1d.2.6, and `ideas.md`'s middle outcome.
            // …and it qualifies itself for the same reason `Contradiction`
            // does. An open state under a cap has an obligation unwitnessed
            // *and* a frontier unvisited, and a deeper layer could discharge
            // it. No corpus cell reaches this arm truncated today — every
            // `Open` in the corpus is exhausted — so the branch is here to be
            // right rather than because something moved.
            let held = if exhausted {
                "the requirement is unmet, not refuted"
            } else {
                "the requirement is unmet and the search did not exhaust"
            };
            format!("Open — {}; {held}.", owed_phrase(terms, owes))
        }
        // ein.py's fall-through prints the *class* name, and `Aborted` is the
        // only shape that reaches it.
        Answer::Aborted { .. } => "Unexpected verdict: Aborted".to_string(),
    }
}

/// `owes 4 (color-loc: 2, pet-loc: 2)` — the count, and what it is owed on.
///
/// The per-relation split is what `(open ?R)` buys over a bare `(open)`: an
/// unattributed debt contributes to the total and to no parenthesis, so the
/// two numbers agree exactly when every obligation rule names its relation.
fn owed_phrase(terms: &Terms, owes: &[Owes]) -> String {
    let total: usize = owes.iter().map(|o| o.total()).sum();
    // Merged across the open states, in first-owed order — the same order
    // `Owes::by_relation` reports within one.
    let mut split: Vec<(String, usize)> = Vec::new();
    for o in owes {
        for (r, n) in o.by_relation() {
            let r = terms.sym(r).to_string();
            match split.iter_mut().find(|(s, _)| *s == r) {
                Some((_, m)) => *m += n,
                None => split.push((r, n)),
            }
        }
    }
    if split.is_empty() {
        return format!("owes {total}");
    }
    let split: Vec<String> = split
        .into_iter()
        .map(|(r, n)| format!("{r}: {n}"))
        .collect();
    format!("owes {total} ({})", split.join(", "))
}

// ── The five-field table ───────────────────────────────────────────

fn rule(width: usize) -> String {
    "─".repeat(width)
}

/// A left-aligned two-column block. Column one is as wide as its widest entry
/// — and as the header label, so an optional header row stays aligned.
fn two_col(rows: &[(String, String)], indent: &str, header: Option<(&str, &str)>) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut w = rows
        .iter()
        .map(|(a, _)| a.chars().count())
        .max()
        .unwrap_or(0);
    let mut out: Vec<String> = Vec::new();
    if let Some((h0, h1)) = header {
        w = w.max(h0.chars().count());
        out.push(pad_row(indent, h0, h1, w));
    }
    out.extend(rows.iter().map(|(a, b)| pad_row(indent, a, b, w)));
    out
}

/// `f"{indent}{a:<{w}}  {b}".rstrip()` — the width is in *characters*, as
/// Python's format spec counts them.
fn pad_row(indent: &str, a: &str, b: &str, w: usize) -> String {
    let pad = w.saturating_sub(a.chars().count());
    let line = format!("{indent}{a}{}  {b}", " ".repeat(pad));
    line.trim_end().to_string()
}

/// The bindings, rendered facts and result sections for one model.
fn solution_block(ast: &Ast, terms: &mut Terms, kb: &Kb, header: &str) -> Vec<String> {
    let goal = query_goal(ast, Some(kb));
    let raw = goal_bindings(ast, terms, kb, goal);
    let mut out: Vec<String> = Vec::new();
    if !header.is_empty() {
        out.push(format!("  {header}"));
    }
    let Some(first) = raw.first() else {
        out.push("    (no query goal to project)".to_string());
        return out;
    };
    let b = row_of(terms, first);

    // Query bindings, sorted by variable name for deterministic output.
    out.push("  query bindings".to_string());
    let mut binding_rows: Vec<(String, String)> = b
        .iter()
        .map(|(k, v)| (format!("?{k}"), format!("= {v}")))
        .collect();
    binding_rows.sort();
    out.extend(two_col(&binding_rows, "    ", None));
    out.push(String::new());

    // Rendered query facts: ground each conjunct, render via the relation's `:why`.
    let fact_rows: Vec<(String, String)> = conjuncts(ast, goal)
        .into_iter()
        .filter_map(|c| ground(ast, c, &b))
        .map(|(rel, args)| {
            (
                sexpr(&rel, &args),
                render_fact(Some(kb), terms, &rel, &args),
            )
        })
        .collect();
    out.extend(two_col(
        &fact_rows,
        "    ",
        Some(("query facts", "rendered")),
    ));
    out.push(String::new());

    // The NL result, from `:goal-text`.
    out.push("  result".to_string());
    match goal_text(ast, Some(kb), &b) {
        None => out.push("    (query has no :goal-text template)".to_string()),
        Some(text) => out.push(format!("    {text}")),
    }
    out
}

/// The full `solve` table.
///
/// All text is rendered from puzzle data; this function contributes the field
/// labels and the layout, never domain vocabulary.
///
/// `exhausted` qualifies **every count it prints**: a `Solution`'s `k` since
/// ein.py, and — since M1d
/// [S1d.3.3](../../../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
/// — an `Ambiguity`'s, which is the one that needed it most. `models`
/// chooses the model *set*'s projection ([`crate::models::ModelsForm`]) and
/// is read by the `Ambiguity` arm alone.
#[allow(clippy::too_many_arguments)]
pub fn render_solution_table(
    ast: &Ast,
    terms: &mut Terms,
    root: &Kb,
    answer: &Answer,
    exhausted: bool,
    source: Option<&str>,
    models: crate::models::ModelsForm,
) -> Result<String, String> {
    // ein.py compiles the `:goal` pattern inside `_solution_block`, so a goal
    // the compiler rejects — `(query :goal (?R Rex Animal))`, an unbound
    // relation head — raises out of the renderer. Note *where*: only a
    // verdict with a model renders a block, so a `Contradiction` prints its
    // unsat core and exits 0 with the same broken goal. Reproducing that is
    // the whole subtlety, and getting it wrong turned one divergence into
    // another for the length of one commit (S1a.6.6, 2026-08-20).
    let has_model = matches!(
        answer,
        Answer::Verdict(Verdict::Solution(_)) | Answer::Verdict(Verdict::Ambiguity(_))
    );
    if has_model && let Some(e) = ein_infer::verdict::goal_plan_error(ast, terms, root, None) {
        return Err(format!("ein.inference.compile.CompileError: {e}"));
    }
    let mut lines: Vec<String> = vec![
        match source {
            Some(s) => format!("solve · {s}"),
            None => "solve".to_string(),
        },
        rule(62),
    ];

    // **The count and its qualifier are read off the verdict, once.** Each arm
    // below used to choose both for itself: `Solution` printed the *search's*
    // node count, `Ambiguity` re-derived the distinct count, and the two `k =
    // 0` arms wrote the same literal twice. That is the seam M1e `AR-M2` is
    // about — `finalise` constructs a verdict once and three crates render it
    // — and the structural half of the fix is that there is now nothing in an
    // arm to choose *with*: [`ReadOut`] arrives made, and `solution_nodes` is
    // no longer a parameter of this function.
    if let Answer::Verdict(v) = answer {
        let ro = v.read_out(exhausted);
        lines.push(format!("  solutions (k)   {}{}", ro.k, ro.suffix()));
    }

    match answer {
        Answer::Verdict(Verdict::Solution(s)) => {
            lines.push("  verdict         Solution".to_string());
            lines.push(String::new());
            let block = solution_block(ast, terms, &s.kb, "");
            lines.extend(block);
        }
        Answer::Verdict(Verdict::Ambiguity(branches)) => {
            // One word, and it is the whole of the rule's second row: *these
            // are the models* against *these are models found*. The rest of
            // the sentence is unchanged because it stays true either way —
            // two models found is under-determined however deep the search
            // went.
            lines.push(format!(
                "  verdict         Ambiguous — distinct complete models{}; \
                 the puzzle is under-determined",
                if exhausted { "" } else { " found" }
            ));
            // `--models key` — the model *set* as its determining key, M1d
            // P1d.3's (b). A rendering and never a replacement: the models it
            // stands in for are still in `verdict.solutions`, in
            // `--json-summary`, in `--events` and under `-p`, and an
            // unaffordable key falls back to the enumeration below.
            let mut listed = true;
            if models == crate::models::ModelsForm::Key {
                let kbs: Vec<&Kb> = branches.iter().map(|b| &b.kb).collect();
                lines.push(String::new());
                match crate::models::key_table(terms, &kbs, exhausted, "  ") {
                    crate::models::KeyOutcome::Table(rows) => {
                        lines.extend(rows);
                        listed = false;
                    }
                    crate::models::KeyOutcome::Unaffordable(why) => {
                        lines.push("  determining key — none within budget".to_string());
                        lines.extend(crate::models::wrap(
                            &format!("{why}, so the models are printed instead."),
                            "    ",
                        ));
                    }
                }
            }
            if listed {
                for i in 0..branches.len() {
                    lines.push(String::new());
                    let header = format!("model {}/{}", i + 1, branches.len());
                    let block = solution_block(ast, terms, &branches[i].kb, &header);
                    lines.extend(block);
                }
            }
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            lines.push(format!(
                "  verdict         {}",
                if exhausted {
                    "No solution — the constraints are contradictory"
                } else {
                    "No model found — the search did not exhaust the lattice"
                }
            ));
            let mut core: Vec<FactId> = unsat_core.clone();
            core.sort_by_key(|f| {
                let (rel, args) = terms.fact(*f);
                (
                    terms.sym(rel).to_string(),
                    args.iter().map(|a| terms.display(*a)).collect::<Vec<_>>(),
                )
            });
            // The source frontier — the given conditions that jointly force
            // the conflict. The human-meaningful "which inputs clash"; the raw
            // fact list below is the full core.
            let srcs = core_sources(root, terms, &core);
            if !srcs.is_empty() {
                lines.push(format!("  conflicting sources: {}", srcs.join(", ")));
            }
            lines.push(String::new());
            // An *unsat core* is an explanation of why the program has no
            // model, and a truncated run has not shown that. What it holds is
            // the commitments it refuted on the way — which is why
            // `zebra2-minus-15 -m 1` printing `unsat core (0 facts)` read as
            // "the empty set is contradictory" instead of "nothing died".
            lines.push(format!(
                "  {} ({} facts)",
                if exhausted {
                    "unsat core"
                } else {
                    "refuted so far"
                },
                core.len()
            ));
            let rows: Vec<(String, String)> = core
                .iter()
                .map(|f| {
                    let (rel, args) = terms.fact(*f);
                    let rel = terms.sym(rel).to_string();
                    let args: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
                    (
                        sexpr(&rel, &args),
                        render_fact(Some(root), terms, &rel, &args),
                    )
                })
                .collect();
            lines.extend(two_col(&rows, "    ", None));
        }
        Answer::Verdict(Verdict::Open { states, owes }) => {
            // `solutions (k)` is 0 above because an open state is **not** a
            // model: the read-out's `complete` means discharged. It is
            // deliberately not the same number as `stats.solution_nodes`,
            // which counts what the *search* recorded and which S1d.2.6 left
            // alone — the two disagree on exactly the twelve corpus entries
            // that reach this word, and this row is where the difference is
            // printed rather than hidden.
            lines.push(format!("  open states     {}", states.len()));
            lines.push(format!(
                "  verdict         Open — {}{}",
                owed_phrase(terms, owes),
                if exhausted {
                    ""
                } else {
                    "; the search did not exhaust"
                }
            ));
            let total: usize = owes.iter().map(|o| o.total()).sum();
            lines.push(String::new());
            lines.push(format!("  outstanding obligations ({total})"));
            for o in owes.iter() {
                for why in crate::trace::linearize::owe_lines(terms, o) {
                    lines.push(format!("    {why}"));
                }
            }
            for i in 0..states.len() {
                lines.push(String::new());
                let header = if states.len() == 1 {
                    "open state".to_string()
                } else {
                    format!("open state {}/{}", i + 1, states.len())
                };
                let block = solution_block(ast, terms, &states[i].kb, &header);
                lines.extend(block);
            }
        }
        other => {
            let text = render_answer(ast, terms, root, other, exhausted);
            lines.push(format!("  {text}"));
        }
    }

    Ok(lines.join("\n"))
}
