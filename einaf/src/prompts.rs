//! **The prompts of the formalization's steps**: versioned templates under
//! `einaf/prompts/`, rendered by filling named slots, and the examples they
//! show, each a text with the program each step wrote for it.
//!
//! A template is a text with slots `{{name}}`; a line that begins with `%%`
//! is the template's own note (which section of the contract it implements)
//! and is not rendered. Rendering fails on a slot the caller does not fill
//! and on a value given for a slot the template does not have, so that a
//! prompt holds exactly what the template says. **A template's digest and
//! the rendered prompt's are the record's**: a prompt edit changes a digest,
//! and a changed digest is a new experiment.
//!
//! What a prompt holds is what the contract's § 5 lists
//! (`docs/einaf/04-contract.md`): the excerpt of Ein's reference
//! ([`EXCERPT`]), the catalogue (steps 2 and 3 only,
//! [`crate::catalogue`]), the examples for its step ([`EXAMPLES`]), the text
//! with its answer line, and the program of the steps before it.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::sha256_hex;

/// **A versioned template**: its file under `einaf/prompts/` and its text.
#[derive(Clone, Copy, Debug)]
pub struct Template {
    pub file: &'static str,
    pub text: &'static str,
}

impl Template {
    /// The template's digest, of its text as the file holds it.
    pub fn sha256(&self) -> String {
        sha256_hex(self.text.as_bytes())
    }

    /// **Render the template**, every slot filled from `slots`: a slot with
    /// no value, or a value for no slot, is an error naming it.
    pub fn render(&self, slots: &BTreeMap<&str, String>) -> Result<String, String> {
        let body: String = self
            .text
            .lines()
            .filter(|l| !l.starts_with("%%"))
            .map(|l| format!("{l}\n"))
            .collect();
        let mut out = String::new();
        let mut used: Vec<&str> = Vec::new();
        let mut rest = body.as_str();
        while let Some(i) = rest.find("{{") {
            out.push_str(&rest[..i]);
            let after = &rest[i + 2..];
            let Some(j) = after.find("}}") else {
                return Err(format!("{}: a slot is not closed", self.file));
            };
            let name = &after[..j];
            let value = slots
                .get(name)
                .ok_or_else(|| format!("{}: the slot {{{{{name}}}}} is not filled", self.file))?;
            out.push_str(value);
            used.push(name);
            rest = &after[j + 2..];
        }
        out.push_str(rest);
        if let Some(extra) = slots.keys().find(|k| !used.contains(k)) {
            return Err(format!(
                "{}: the template has no slot {{{{{extra}}}}}",
                self.file
            ));
        }
        Ok(out.trim_end().to_string() + "\n")
    }
}

macro_rules! template {
    ($path:literal) => {
        Template {
            file: $path,
            text: include_str!(concat!("../prompts/", $path)),
        }
    };
}

/// The excerpt of Ein's reference every request of the steps holds, its
/// second version: the narrower name set, `config` named as a form the model
/// does not write, and its examples of no family the phase asks.
pub const EXCERPT: Template = template!("ein-excerpt.v2.md");
/// The system message: one sentence of instructions and the excerpt.
pub const SYSTEM: Template = template!("steps/system.v1.md");
/// Step 1: the facts, the answer query and a partial ontology.
pub const STEP1: Template = template!("steps/step1.v1.md");
/// Step 2: the complete ontology and the theory selected.
pub const STEP2: Template = template!("steps/step2.v1.md");
/// Step 2's second call, on a marker: the rules of the missing property.
pub const STEP2B: Template = template!("steps/step2b.v1.md");
/// Step 3: the complete theory and the hypothesis mechanism.
pub const STEP3: Template = template!("steps/step3.v1.md");
/// The whole program in one call.
pub const WHOLE: Template = template!("steps/whole.v1.md");

/// Every template of the steps, the excerpt among them.
pub const TEMPLATES: [Template; 7] = [EXCERPT, SYSTEM, STEP1, STEP2, STEP2B, STEP3, WHOLE];

/// **An answer line**: what the text is asked to answer with, the same
/// words for every arm of a family; this stage's own, by name.
#[derive(Clone, Copy, Debug)]
pub struct AnswerLine {
    pub name: &'static str,
    pub template: Template,
}

/// The answer lines this stage's runs need: who drinks water and who owns
/// the zebra, how many for the word problems, and a sentence for any other
/// text.
pub const ANSWER_LINES: [AnswerLine; 3] = [
    AnswerLine {
        name: "zebra",
        template: template!("answer-lines/zebra.v1.md"),
    },
    AnswerLine {
        name: "word",
        template: template!("answer-lines/word.v1.md"),
    },
    AnswerLine {
        name: "generic",
        template: template!("answer-lines/generic.v1.md"),
    },
];

/// An answer line by its name.
pub fn answer_line(name: &str) -> Option<AnswerLine> {
    ANSWER_LINES.iter().find(|a| a.name == name).copied()
}

/// **An example**: a text, its family, and what each step wrote for it.
/// Its family is none the phase's texts have, so that no item is shown an
/// example of its own family.
#[derive(Clone, Copy, Debug)]
pub struct Example {
    pub id: &'static str,
    pub family: &'static str,
    pub text: &'static str,
    /// What step 1, step 2 and step 3 each wrote: a step's own output, not
    /// the program so far.
    pub steps: [&'static str; 3],
}

macro_rules! example {
    ($id:literal, $family:literal) => {
        Example {
            id: $id,
            family: $family,
            text: include_str!(concat!("../prompts/examples/", $id, "/text.txt")),
            steps: [
                include_str!(concat!("../prompts/examples/", $id, "/step1.ein")),
                include_str!(concat!("../prompts/examples/", $id, "/step2.ein")),
                include_str!(concat!("../prompts/examples/", $id, "/step3.ein")),
            ],
        }
    };
}

/// **The examples every request shows**: a smaller attribute puzzle over the
/// Zebra puzzle's theory, three stalls in a row; and two word problems over
/// the calculator alone, a purchase and an age. None is of a family the
/// phase's texts are of.
pub const EXAMPLES: [Example; 3] = [
    example!("shops", "stalls"),
    example!("buying", "buying"),
    example!("ages", "ages"),
];

impl Example {
    /// The files every program of the example is, from the package's root.
    pub fn files(&self) -> Vec<String> {
        (1..=3)
            .map(|n| format!("prompts/examples/{}/step{n}.ein", self.id))
            .collect()
    }

    /// The example's whole program, its steps assembled in the order they
    /// were made ([`crate::formalize::assemble_parts`]).
    pub fn program(&self) -> String {
        crate::formalize::assemble_parts(
            &crate::formalize::split_step1(self.steps[0]).0,
            &[self.steps[1], self.steps[2]],
        )
    }
}

/// Which steps an example shows: up to step `n` (1 to 3), or the whole
/// program assembled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shown {
    UpTo(u8),
    Whole,
}

/// **The examples' block** of a prompt: each a text and the steps it shows.
pub fn examples_block(examples: &[Example], shown: Shown) -> String {
    let mut s = String::new();
    for (i, e) in examples.iter().enumerate() {
        s.push_str(&format!(
            "## Example {}\n\nText:\n{}\n",
            i + 1,
            e.text.trim_end()
        ));
        match shown {
            Shown::UpTo(n) => {
                for (k, step) in e.steps.iter().enumerate().take(n as usize) {
                    s.push_str(&format!(
                        "\nStep {}:\n```ein\n{}\n```\n",
                        k + 1,
                        step.trim_end()
                    ));
                }
            }
            Shown::Whole => {
                s.push_str(&format!(
                    "\nProgram:\n```ein\n{}\n```\n",
                    e.program().trim_end()
                ));
            }
        }
        s.push('\n');
    }
    s
}
