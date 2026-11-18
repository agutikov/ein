//! The constraint view: a knowledge base's facts about relations, drawn.
//!
//! **What it keeps is the slice of the facts about relations** (Q-M1f.523,
//! Q-M1f.530), [`ein_infer::slice::about_relations`]: every fact the knowledge
//! base holds with a relation of the program directly under one of its
//! argument arrows, `(symmetric co-located)`, `(bijective color-loc)`, a
//! declaration `(relation co-located Attribute Attribute)` and the fact
//! `(relation co-located)` the loader adds beside it. It is read off the
//! graph alone: no name is read for what it means, and no fact's provenance
//! or file, so the view tracks whatever a program states and derives. A
//! written check, `(eq a w)`, is no fact a knowledge base holds, and is not
//! drawn (Q-M1f.531).
//!
//! **Each is drawn as a fact by the KB view's code** ([`crate::kb_dot`],
//! Q-M1f.419), `(symmetric co-located)` an octagon whose first arrow is at
//! `symmetric` and whose second is at `co-located`, never a property written
//! into the relation's node, or folded under `--fold`. A fact under an arrow
//! of one is its own node, in the frame of its context where the knowledge
//! base does not hold it (Q-M1f.395, Q-M1f.398, Q-M1f.527).

use ein_core::{Kb, Terms};

use crate::kb_dot::{KbDotOpts, to_dot};

/// Draw the facts about relations of `kb`, a program's root at its fixpoint
/// as `ein render constraints` hands it, each in the canonical form, or folded
/// where `fold`.
pub fn render_constraints(kb: &Kb, terms: &Terms, name: &str, fold: bool) -> String {
    let only = ein_infer::slice::about_relations(kb, terms);
    to_dot(
        kb,
        terms,
        &KbDotOpts {
            only: Some(&only),
            name,
            fold,
            ..KbDotOpts::default()
        },
    )
}
