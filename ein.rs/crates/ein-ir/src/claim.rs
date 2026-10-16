//! `(solve … :expect …)` — the expected region, M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
//!
//! The shape, which the loader validates; the comparison against R is
//! `ein_infer::claim`. The grammar is the one the user ruled on 2026-10-16
//! ([`region_examples.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)):
//!
//! ```lisp
//! (solve :expect (false))                          ; every node dead
//! (solve :expect (region))                         ; R empty, either kind
//! (solve
//!   :hrules (guess (a-prop Item) guess (b-prop Item))
//!   :expect (region
//!             (model)                              ; one state of R each
//!             (model (a-prop X))
//!             (merged (model (h A) (h B))          ; a state with several
//!                     (or (model (h A))            ; generating commitments,
//!                         (model (h B))))          ; and its minimal ones
//!             (owing (model …) …)))                ; R empty, these owe
//! ```
//!
//! **The words are this keyword's alone.** `region`, `merged` and `owing` are
//! read here and nowhere else, as `model` is under a query's `:expect`. None of
//! them is reserved, apart from the top-level guard in the loader: a `(region
//! …)` pasted at the top of a file would otherwise load as a fact of a new
//! relation, check nothing and say nothing.

use crate::ast::{Ast, Node, NodeId};
use crate::expect::{Model, fact};

/// A parsed `(solve … :expect …)` value.
#[derive(Clone, Debug)]
pub enum Claim {
    /// `(false)`: every node of the lattice is dead.
    False,
    /// `(region …)`.
    Region(Region),
}

/// `(region …)`'s items, in source order within each kind.
#[derive(Clone, Debug, Default)]
pub struct Region {
    /// The states of R, one `(model …)` each.
    pub models: Vec<Model>,
    /// The states reached by more than one commitment, with their minimal
    /// ones. All or nothing: none claims states only, one claims them all.
    pub merged: Vec<Merged>,
    /// `(owing …)`'s models: R is empty, and these alive nodes owe.
    pub owing: Vec<Model>,
}

/// `(merged <to> <from>)`.
#[derive(Clone, Debug)]
pub struct Merged {
    /// One of the states the region lists.
    pub to: Model,
    /// Its minimal generating commitments, as the candidates committed.
    pub from: Vec<Model>,
}

/// The heads the loader refuses at the top level.
pub const CLAIM_HEADS: [&str; 3] = ["region", "merged", "owing"];

const SHAPE: &str = "(solve … :expect …) — expected `(false)` or `(region …)`";

/// Read a `(solve … :expect …)` value, or say what is wrong with it.
pub fn parse(ast: &Ast, node: NodeId) -> Result<Claim, String> {
    let Some((name, items)) = form(ast, node) else {
        return Err(SHAPE.into());
    };
    match name {
        "false" if items.is_empty() => Ok(Claim::False),
        "false" => Err("(solve … :expect (false …)) — ⊥ takes no arguments".into()),
        "region" => region(ast, items).map(Claim::Region),
        "model" | "or" => Err(format!(
            "(solve … :expect ({name} …)) — a region is stated as `(region (model …) …)`; \
             `({name} …)` is a query's answer"
        )),
        other => Err(format!("(solve … :expect ({other} …)) — {SHAPE}")),
    }
}

fn form(ast: &Ast, node: NodeId) -> Option<(&str, &[NodeId])> {
    let Node::SForm { head, args } = ast.node(node) else {
        return None;
    };
    Some((ast.atom_name(head)?, ast.args(args)))
}

fn region(ast: &Ast, items: &[NodeId]) -> Result<Region, String> {
    let mut out = Region::default();
    let mut owing_seen = false;
    for &item in items {
        match form(ast, item) {
            Some(("model", facts)) => out.models.push(model(ast, facts)?),
            Some(("merged", parts)) => {
                let [to, from] = parts else {
                    return Err("(merged …) — expected `(merged (model …) <from>)`, \
                                a state and its minimal commitments"
                        .into());
                };
                let to = match form(ast, *to) {
                    Some(("model", facts)) => model(ast, facts)?,
                    _ => {
                        return Err("(merged …) — its first part is a `(model …)`, \
                                     one of the states the region lists"
                            .into());
                    }
                };
                let from = match form(ast, *from) {
                    Some(("model", facts)) => vec![model(ast, facts)?],
                    Some(("or", alts)) if !alts.is_empty() => {
                        let mut v = Vec::with_capacity(alts.len());
                        for &a in alts {
                            match form(ast, a) {
                                Some(("model", facts)) => v.push(model(ast, facts)?),
                                _ => {
                                    return Err("(merged …) — every commitment in its \
                                                 `(or …)` is a `(model …)`"
                                        .into());
                                }
                            }
                        }
                        v
                    }
                    _ => {
                        return Err("(merged …) — its second part is a `(model …)`, or an \
                                     `(or (model …) …)` of them"
                            .into());
                    }
                };
                out.merged.push(Merged { to, from });
            }
            Some(("owing", models)) => {
                if owing_seen {
                    return Err("(region …) — one `(owing …)`, listing every owing state".into());
                }
                owing_seen = true;
                if models.is_empty() {
                    return Err("(owing) — it lists the alive nodes that owe; an empty R \
                                with nothing listed is `(region)`"
                        .into());
                }
                for &m in models {
                    match form(ast, m) {
                        Some(("model", facts)) => out.owing.push(model(ast, facts)?),
                        _ => return Err("(owing …) — every item is a `(model …)`".into()),
                    }
                }
            }
            _ => {
                return Err(format!(
                    "(region …) — `{}` is not `(model …)`, `(merged …)` or `(owing …)`",
                    crate::expect::render(ast, item)
                ));
            }
        }
    }
    if !out.owing.is_empty() && !out.models.is_empty() {
        return Err("(region …) — `(owing …)` says R is empty, and the region lists states".into());
    }
    if !out.merged.is_empty() && out.models.is_empty() {
        return Err("(region …) — a `(merged …)` names a state, and the region lists none".into());
    }
    // Each `<to>` is one of the listed states, compared as fact sets.
    let key = |m: &Model| {
        let mut v: Vec<String> = m
            .facts
            .iter()
            .map(|&f| crate::expect::render(ast, f))
            .collect();
        v.sort();
        v
    };
    let listed: Vec<Vec<String>> = out.models.iter().map(key).collect();
    for m in &out.merged {
        if !listed.contains(&key(&m.to)) {
            return Err("(merged …) — its state is not one the region lists".into());
        }
    }
    Ok(out)
}

fn model(ast: &Ast, items: &[NodeId]) -> Result<Model, String> {
    for &item in items {
        fact(ast, item).map_err(|e| e.replacen(":expect", "(solve … :expect …)", 1))?;
    }
    Ok(Model {
        facts: items.to_vec(),
    })
}
