//! The `solve` answer and its table — `ein.py`'s `trace/answer.py`.
//!
//! **No hardcoded vocabulary.** Every word of English here comes from
//! *puzzle-authored* templates; there is no relation→verb table. Two template
//! sources drive the text, both through the rule `:why` engine
//! ([`crate::why::render_why`]):
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
use ein_infer::verdict::{Answer, Verdict};
use ein_infer::{canon::state_key, goal_bindings, query_value};
use ein_ir::{Ast, Node, NodeId};

use crate::why::render_why;

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

/// The number of *distinct* models among the branches — ein.py's
/// `len({state_key(b.kb) for b in branches}) or len(branches)`, whose `or`
/// makes an empty branch list report its own length.
fn distinct_models(branches: &[ein_infer::Solution]) -> usize {
    let mut keys: Vec<Box<[FactId]>> = branches.iter().map(|b| state_key(&b.kb)).collect();
    // determinism-ok: the sort is `dedup`'s precondition and nothing else —
    // what leaves this function is a *count*, which no order can move.
    keys.sort();
    keys.dedup();
    if keys.is_empty() {
        branches.len()
    } else {
        keys.len()
    }
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
        Answer::Verdict(Verdict::Ambiguity(branches)) => {
            let k = distinct_models(branches);
            format!("Ambiguous — {k} distinct complete models; the puzzle is under-determined.")
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            let srcs = core_sources(root, terms, unsat_core);
            let core = if srcs.is_empty() {
                format!("{} facts", unsat_core.len())
            } else {
                srcs.join(", ")
            };
            format!("No solution — the constraints are contradictory (unsat core: {core}).")
        }
        // ein.py's fall-through prints the *class* name, and `Aborted` is the
        // only shape that reaches it.
        Answer::Aborted { .. } => "Unexpected verdict: Aborted".to_string(),
    }
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
pub fn render_solution_table(
    ast: &Ast,
    terms: &mut Terms,
    root: &Kb,
    answer: &Answer,
    solution_nodes: Option<u64>,
    exhausted: bool,
    source: Option<&str>,
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

    match answer {
        Answer::Verdict(Verdict::Solution(s)) => {
            let cert = if exhausted {
                ""
            } else {
                "   (not certified — pass --exhaustive)"
            };
            lines.push(format!(
                "  solutions (k)   {}{cert}",
                solution_nodes.map_or("1".to_string(), |k| k.to_string())
            ));
            lines.push("  verdict         Solution".to_string());
            lines.push(String::new());
            let block = solution_block(ast, terms, &s.kb, "");
            lines.extend(block);
        }
        Answer::Verdict(Verdict::Ambiguity(branches)) => {
            let kk = distinct_models(branches);
            lines.push(format!("  solutions (k)   {kk}"));
            lines.push(
                "  verdict         Ambiguous — distinct complete models; \
                 the puzzle is under-determined"
                    .to_string(),
            );
            for i in 0..branches.len() {
                lines.push(String::new());
                let header = format!("model {}/{}", i + 1, branches.len());
                let block = solution_block(ast, terms, &branches[i].kb, &header);
                lines.extend(block);
            }
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            lines.push("  solutions (k)   0".to_string());
            lines.push(
                "  verdict         No solution — the constraints are contradictory".to_string(),
            );
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
            lines.push(format!("  unsat core ({} facts)", core.len()));
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
        other => {
            let text = render_answer(ast, terms, root, other, exhausted);
            lines.push(format!("  {text}"));
        }
    }

    Ok(lines.join("\n"))
}
