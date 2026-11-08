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
//! read here and nowhere else, as `model` is under a query's `:expect`. Each
//! is a word a position gives a meaning to since M1f S1f.5.5's swap, a name
//! wherever a name goes but not a top-level fact's relation (`ein.y`'s
//! `FactHead`), so a `(region …)` pasted at the top of a file is refused at the
//! parse, where until then it would have loaded as a fact of a new relation,
//! checked nothing and said nothing, but for the loader's guard (M1e2
//! S1e2.1a.4, `CLAIM_HEADS` until T1f.5.5.5).

use crate::ast::{Ast, Node, NodeId};
use crate::expect::Model;

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

/// Read a `(solve … :expect …)` value, or say what is wrong with it.
///
/// **Its shape is the grammar's** since M1f S1f.5.5's swap (`ein.y`'s
/// `RegionClaim`, `RegionItem` and `MergedFrom`): `(false)`, or a `(region
/// …)` of `(model …)`s, `(merged (model …) <from>)`s whose `<from>` is a
/// `(model …)` or an `(or …)` of one or more, and `(owing (model …) …)`s of
/// one or more. Until T1f.5.5.5 this module refused every other shape, eleven
/// lines; since the swap the parse refuses each first. What stays is what a
/// grammar cannot see: one `(owing …)` at most, not beside a state, and a
/// `(merged …)` whose state is one the region lists.
pub fn parse(ast: &Ast, node: NodeId) -> Result<Claim, String> {
    match form(ast, node).expect("a region claim is a form (ein.y's RegionClaim)") {
        ("false", _) => Ok(Claim::False),
        ("region", items) => region(ast, items).map(Claim::Region),
        (other, _) => unreachable!("a region claim is (false) or (region …), not ({other} …)"),
    }
}

fn form(ast: &Ast, node: NodeId) -> Option<(&str, &[NodeId])> {
    let Node::SForm { head, args } = ast.node(node) else {
        return None;
    };
    Some((ast.atom_name(head)?, ast.args(args)))
}

/// A `(model …)`'s facts, which the grammar makes the one shape at each place
/// this reads one.
fn model(ast: &Ast, node: NodeId) -> Model {
    match form(ast, node) {
        Some(("model", facts)) => Model {
            facts: facts.to_vec(),
        },
        _ => unreachable!("a (model …) (ein.y's Model)"),
    }
}

fn region(ast: &Ast, items: &[NodeId]) -> Result<Region, String> {
    let mut out = Region::default();
    let mut owing_seen = false;
    for &item in items {
        match form(ast, item) {
            Some(("model", _)) => out.models.push(model(ast, item)),
            Some(("merged", [to, from])) => {
                let from = match form(ast, *from) {
                    Some(("or", alts)) => alts.iter().map(|&a| model(ast, a)).collect(),
                    _ => vec![model(ast, *from)],
                };
                out.merged.push(Merged {
                    to: model(ast, *to),
                    from,
                });
            }
            Some(("owing", models)) => {
                if owing_seen {
                    return Err("(region …) — one `(owing …)`, listing every owing state".into());
                }
                owing_seen = true;
                out.owing.extend(models.iter().map(|&m| model(ast, m)));
            }
            _ => unreachable!(
                "a region lists (model …), (merged …) and (owing …) (ein.y's RegionItem)"
            ),
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
