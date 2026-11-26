//! **The formalization contract's categories**: each top-level form of a
//! program in exactly one row of [the contract's table](../../docs/einaf/04-contract.md#4-what-each-form-is-the-categories),
//! with the step that writes it and, where the row says so, the faithfulness
//! judge's event or note.
//!
//! A category is computed, never written: from the program's forms and what
//! its imports bring ([`crate::kernel::forms()`]), and from the problem's
//! text, whose sentences a `:source` is read against. Nothing is added to the
//! program, which stays a plain input of the kernel.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::kernel::{Form, Forms, Shape, Written};

/// **A row of the contract's table of categories**: B1's five, and the
/// program's frame, which today's forms need and which is none of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    /// A fact, a stored negation or a written `(open F)` with a `:source`.
    SourceDerived,
    /// An import; a fact that activates a rule of an imported module; a fact
    /// of a relation an imported module's rule reads, which the program does
    /// not declare.
    GenericTheory,
    /// A `relation` form of which no source-derived fact is a fact and which
    /// the answer query does not read.
    GeneratedRelation,
    /// A `rule` or a `macro` of the program's own, and a fact that activates
    /// one.
    GeneratedRule,
    /// A fact or a stored negation stated with no `:source` that no rule of
    /// the library or of the program reads as its activator.
    GeneratedAssumption,
    /// What a program needs to be well formed and to state its lattice: a
    /// `relation` form a source-derived fact or the answer query reads; a
    /// query; `(solve …)`; `(__closed__ R)`; a written `(open F)` with no
    /// `:source`; and the forms the contract does not allow, `(config …)` and
    /// `(trace …)`, which its check reports.
    Frame,
}

impl Category {
    /// Every row, in the table's order.
    pub const ALL: [Category; 6] = [
        Category::SourceDerived,
        Category::GenericTheory,
        Category::GeneratedRelation,
        Category::GeneratedRule,
        Category::GeneratedAssumption,
        Category::Frame,
    ];

    /// The row, as the table names it.
    pub fn words(self) -> &'static str {
        match self {
            Category::SourceDerived => "source-derived fact",
            Category::GenericTheory => "generic theory",
            Category::GeneratedRelation => "generated auxiliary relation",
            Category::GeneratedRule => "generated inference rule",
            Category::GeneratedAssumption => "generated assumption",
            Category::Frame => "the program's frame",
        }
    }
}

/// What the faithfulness judge reads of a form: an **event**, which counts
/// against the program, or a **note**, which does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Judged {
    /// `unfound_source`: a `:source` that is no sentence of the input.
    UnfoundSource,
    /// `generated_assumption`: a fact stated as holding with no sentence
    /// behind it.
    GeneratedAssumption,
    /// `generated_relation`, a note.
    GeneratedRelation,
    /// `generated_rule`, a note.
    GeneratedRule,
}

impl Judged {
    /// The name the judge gives it.
    pub fn name(self) -> &'static str {
        match self {
            Judged::UnfoundSource => "unfound_source",
            Judged::GeneratedAssumption => "generated_assumption",
            Judged::GeneratedRelation => "generated_relation",
            Judged::GeneratedRule => "generated_rule",
        }
    }

    /// An event counts against the program; a note does not.
    pub fn is_event(self) -> bool {
        matches!(self, Judged::UnfoundSource | Judged::GeneratedAssumption)
    }
}

/// **One top-level form in its row**, with the step that wrote it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Row {
    /// The form as `ein_ir::dump_canonical` prints it: the sidecar's key.
    pub canonical: String,
    pub category: Category,
    /// The step that wrote the form: where the program was made in steps, the
    /// first whose program holds it; otherwise the step that writes its part
    /// ([`part_step`]).
    pub step: u8,
    /// The judge's event or note, where the row gives one.
    pub judged: Option<Judged>,
}

/// **The sentences of a text**: each run of it up to a `.`, `?` or `!` that
/// ends it, followed by white space or by the end, trimmed, its white space
/// folded to one space. A `:source` is read against these.
pub fn sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        current.push(c);
        if matches!(c, '.' | '?' | '!') && chars.peek().is_none_or(|n| n.is_whitespace()) {
            push_sentence(&mut out, &current);
            current.clear();
        }
    }
    push_sentence(&mut out, &current);
    out
}

fn push_sentence(out: &mut Vec<String>, s: &str) {
    let folded = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if !folded.is_empty() {
        out.push(folded);
    }
}

/// **The answer query**: the first query that carries a `:goal-text`, its
/// index among `forms`. A program with none has no answer.
pub fn answer_query(forms: &Forms) -> Option<usize> {
    forms.forms.iter().position(|f| {
        matches!(
            f.shape,
            Shape::Query {
                goal_text: true,
                ..
            }
        )
    })
}

/// **Each form of a program in its row**, the step being the one that writes
/// its part ([`part_step`]): for a program not made in steps.
pub fn categories(forms: &Forms, text: &str) -> Vec<Row> {
    let sentences = sentences(text);
    let frame = frame_relations(forms);
    forms
        .forms
        .iter()
        .map(|f| {
            let (category, judged) = category(forms, &frame, &sentences, f);
            Row {
                canonical: f.canonical.clone(),
                category,
                step: part_step(category, &f.shape),
                judged,
            }
        })
        .collect()
}

/// **Each form of the last of a program's steps in its row**, the step that
/// wrote it the first of `steps` whose program holds its canonical text.
pub fn categories_by_steps(steps: &[&Forms], text: &str) -> Vec<Row> {
    let Some(last) = steps.last() else {
        return Vec::new();
    };
    let held: Vec<BTreeSet<&str>> = steps
        .iter()
        .map(|s| s.forms.iter().map(|f| f.canonical.as_str()).collect())
        .collect();
    categories(last, text)
        .into_iter()
        .map(|mut row| {
            row.step = held
                .iter()
                .position(|h| h.contains(row.canonical.as_str()))
                .map_or(steps.len() as u8, |i| i as u8 + 1);
            row
        })
        .collect()
}

/// **The step that writes a form's part**, by the contract's table of parts:
/// step 1 the statements, the queries and the relations they use; step 2 the
/// rest of the ontology and the theory selected; step 3 the rules and the
/// hypothesis mechanism. A step may write a form of an earlier step's part,
/// never one of a later step's.
pub fn part_step(category: Category, shape: &Shape) -> u8 {
    match category {
        Category::SourceDerived => 1,
        Category::GeneratedRelation | Category::GeneratedAssumption | Category::GenericTheory => 2,
        Category::GeneratedRule => 3,
        Category::Frame => match shape {
            Shape::Query { .. } | Shape::Relation { .. } => 1,
            _ => 3,
        },
    }
}

/// The relations the frame declares: those a source-derived fact is a fact
/// of, and those the answer query reads.
fn frame_relations(forms: &Forms) -> BTreeSet<&str> {
    let mut out: BTreeSet<&str> = forms
        .forms
        .iter()
        .filter_map(|f| match &f.shape {
            Shape::Fact {
                relation,
                source: Some(_),
                ..
            } => Some(relation.as_str()),
            _ => None,
        })
        .collect();
    if let Some(i) = answer_query(forms)
        && let Shape::Query { reads, .. } = &forms.forms[i].shape
    {
        out.extend(reads.iter().map(String::as_str));
    }
    out
}

/// One form's row, and the judge's event or note.
fn category(
    forms: &Forms,
    frame: &BTreeSet<&str>,
    sentences: &[String],
    f: &Form,
) -> (Category, Option<Judged>) {
    match &f.shape {
        Shape::Relation { name } => {
            if frame.contains(name.as_str()) {
                (Category::Frame, None)
            } else {
                (Category::GeneratedRelation, Some(Judged::GeneratedRelation))
            }
        }
        Shape::Rule { .. } | Shape::Macro { .. } => {
            (Category::GeneratedRule, Some(Judged::GeneratedRule))
        }
        Shape::Import { .. } => (Category::GenericTheory, None),
        Shape::Query { .. } | Shape::Solve | Shape::Config | Shape::Trace => {
            (Category::Frame, None)
        }
        Shape::Fact {
            relation,
            written,
            source,
        } => {
            if let Some(source) = source {
                let found = sentences.iter().any(|s| s == source.trim());
                return (
                    Category::SourceDerived,
                    (!found).then_some(Judged::UnfoundSource),
                );
            }
            let relation = relation.as_str();
            if *written == Written::Open || relation == "__closed__" {
                (Category::Frame, None)
            } else if forms.library.rules.contains(relation) {
                (Category::GenericTheory, None)
            } else if forms.rules().contains(relation) {
                (Category::GeneratedRule, Some(Judged::GeneratedRule))
            } else if forms.library.reads.contains(relation) && !forms.declared().contains(relation)
            {
                (Category::GenericTheory, None)
            } else {
                (
                    Category::GeneratedAssumption,
                    Some(Judged::GeneratedAssumption),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sentence_ends_at_its_mark_and_white_space() {
        assert_eq!(
            sentences("Ann is older than Bob. Bob is older than Cy.\nWho is the youngest?\n"),
            [
                "Ann is older than Bob.",
                "Bob is older than Cy.",
                "Who is the youngest?"
            ]
        );
        assert_eq!(
            sentences("It costs 2.5 now. Ok"),
            ["It costs 2.5 now.", "Ok"]
        );
    }
}
