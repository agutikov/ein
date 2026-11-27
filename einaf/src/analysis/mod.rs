//! **A text's analysis by trained parsers, for the formalizer**: what the
//! parsers wrote of each sentence, read from a recorded file, and written two
//! ways for step 1's prompt, as text (the graph as the parser printed it, or a
//! node a line) and as Ein facts in a vocabulary of the analysis's own.
//!
//! **The recorded analysis** ([`Analysis`], `einaf-analysis/1`) is one file a
//! text, written outside EinAf by the parsers' own scripts: the text's
//! sentences; for each AMR parser, its tool and model, its aligner and
//! whether that aligner is learned or rule-based, and a graph a sentence in
//! PENMAN with the aligner's markers; and the named entities a second model
//! found, a sentence's list each. EinAf reads it and runs no parser: its
//! tests read the recorded analyses of the six texts with no Python and no
//! GPU.
//!
//! **The forms** ([`Form`]) are EinAf's: `penman`, each graph as the parser
//! printed it, the markers removed, under its sentence; `lines`, a node a
//! line; `facts`, an Ein program of facts, each quoting its sentence as its
//! `:source`, in the vocabulary declared at its top
//! ([`VOCABULARY`], `einaf/prompts/analysis/vocabulary.v1.ein`). What of
//! AMR's spelling Ein's names do not take is [`name`]'s. A node's words are
//! shown only where the parser's aligner is a learned one (IBM's parser); a
//! rule-based aligner's (amrlib's) are kept in the file and counted, and
//! shown in no form. Nothing here reads an English word: a concept, a role
//! and a name's words are carried through as data.
//!
//! **What step 1's prompt holds** is the chosen form rendered through its
//! versioned template under `einaf/prompts/analysis/`, put in the slot
//! `analysis` of `steps/step1-analysis.v1.md`; a run with no analysis renders
//! `steps/step1.v1.md` as before. The record keeps [`AnalysisOf`]: the file
//! and its digest, the parser's tool, the form, the templates' digests and
//! the block's.

pub mod facts;
pub mod penman;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::model::sha256_hex;
use crate::prompts::Template;

/// The schema of a recorded analysis.
pub const SCHEMA: &str = "einaf-analysis/1";

macro_rules! template {
    ($path:literal) => {
        Template {
            file: $path,
            text: include_str!(concat!("../../prompts/", $path)),
        }
    };
}

/// The analysis as text, PENMAN as the parser printed it.
pub const PENMAN: Template = template!("analysis/penman.v1.md");
/// The analysis as text, a node a line.
pub const LINES: Template = template!("analysis/lines.v1.md");
/// The analysis as Ein facts.
pub const FACTS: Template = template!("analysis/facts.v1.md");
/// The note on the named entities beside the graphs.
pub const ENTITIES: Template = template!("analysis/entities.v1.md");
/// **The vocabulary of the facts form**: its declarations, each with its
/// `:why`, at the top of every facts form; versioned as a prompt is.
pub const VOCABULARY: Template = template!("analysis/vocabulary.v1.ein");

/// Every file of the analysis's prompts.
pub const TEMPLATES: [Template; 5] = [PENMAN, LINES, FACTS, ENTITIES, VOCABULARY];

/// The parser whose graphs a run is given where none is named: SPRING, the
/// most cells right in the check.
pub const DEFAULT_PARSER: &str = "spring";

// ---- the recorded analysis, as its file holds it ---------------------------

/// **A text's recorded analysis**, `einaf-analysis/1`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Analysis {
    pub schema: String,
    /// The text's name.
    pub text: String,
    /// The digest of the text's file the parsers read, where known.
    #[serde(default)]
    pub text_sha256: Option<String>,
    /// The text's sentences, in order: the parsers' input, a graph each.
    pub sentences: Vec<String>,
    /// AMR, a parser each.
    pub amr: Vec<Parse>,
    /// The named entities a second model found, where it ran.
    #[serde(default)]
    pub entities: Option<Entities>,
}

/// **One AMR parser's reading of the text.**
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Parse {
    /// Its short name: `spring`, `xfm`, `ibm`.
    pub parser: String,
    /// The tool, its version, its model and the model's digest, as the
    /// parser's script recorded them.
    pub tool: serde_json::Value,
    /// Whose markers the graphs carry.
    pub aligner: Aligner,
    /// A graph a sentence, in the sentences' order.
    pub graphs: Vec<GraphOf>,
}

/// **The aligner** whose markers a parse's graphs carry: a node's concept
/// to the tokens it was read from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aligner {
    pub name: String,
    /// `learned` (the parser's own, by what it learned) or `rule-based`.
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markers: Option<String>,
}

impl Aligner {
    /// Whether a form shows the words its markers give: a learned aligner's
    /// only.
    pub fn shown(&self) -> bool {
        self.kind == "learned"
    }
}

/// **One sentence's graph**, as the parser's script wrote it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphOf {
    /// The sentence's number, from 1.
    pub n: usize,
    /// The tokens the aligner's markers index, from 0.
    #[serde(default)]
    pub tokens: Vec<String>,
    /// PENMAN, the aligner's markers in it; `null` where the parser wrote no
    /// graph.
    pub penman: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// **The named entities** a second model found, a sentence's list each.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub tool: serde_json::Value,
    pub sentences: Vec<Vec<Entity>>,
}

/// A named entity: its words and its type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub text: String,
    pub label: String,
}

// ---- the choices -----------------------------------------------------------

/// **How the analysis is written for the prompt.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    /// Each graph as the parser printed it, the aligner's markers removed,
    /// under `# ::snt` and its sentence.
    Penman,
    /// A node a line under the sentence.
    Lines,
    /// Ein facts, each quoting its sentence as its `:source`.
    Facts,
}

impl Form {
    pub fn parse(s: &str) -> Option<Form> {
        match s {
            "penman" => Some(Form::Penman),
            "lines" => Some(Form::Lines),
            "facts" => Some(Form::Facts),
            _ => None,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Form::Penman => "penman",
            Form::Lines => "lines",
            Form::Facts => "facts",
        }
    }

    /// The form's template.
    pub fn template(self) -> Template {
        match self {
            Form::Penman => PENMAN,
            Form::Lines => LINES,
            Form::Facts => FACTS,
        }
    }
}

/// **The text form a run is given where none is named**: PENMAN, the
/// parser's own notation, the shorter of the two text forms on the Zebra
/// puzzle; both fit the prompt's budget.
pub const DEFAULT_FORM: Form = Form::Penman;

/// **What of the analysis a run is given**: one parser's graphs, in one
/// form, and the named entities beside them or not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub parser: String,
    pub form: Form,
    pub entities: bool,
}

impl Default for Choice {
    fn default() -> Self {
        Choice {
            parser: DEFAULT_PARSER.to_string(),
            form: DEFAULT_FORM,
            entities: false,
        }
    }
}

// ---- the spelling ----------------------------------------------------------

/// **The words of Ein's grammar**, each its own token: a declarator, a
/// connective, a position word, a head of a saved file. Each is written with
/// `_` after it where AMR writes it as a concept, a role or a value, so that
/// none is read as Ein's (`and`, AMR's conjunction, is `and_`).
pub const EIN_WORDS: [&str; 25] = [
    "and", "or", "neq", "rule", "query", "solve", "config", "trace", "macro", "import", "not",
    "relation", "step", "eq", "absent", "false", "true", "open", "model", "region", "merged",
    "owing", "program", "module", "commit",
];

/// **An AMR symbol, a string or a name's words written as an Ein name**:
/// each character a step's grammar lets a model write in a name (`A-Z a-z
/// 0-9 _ -`) as it is, any other character `_`, words joined by `_`; a word of Ein's
/// grammar ([`EIN_WORDS`]) with `_` after it, and a name of `_` alone, or of
/// nothing, `__` (a bare `_` is refused).
/// A role is written without its colon (`:ARG0` is `ARG0`): a colon begins
/// a keyword.
pub fn name(text: &str) -> String {
    let joined: Vec<String> = text
        .split_whitespace()
        .map(|w| {
            w.chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect()
        })
        .collect();
    let mut s = joined.join("_");
    if EIN_WORDS.contains(&s.as_str()) || s.chars().all(|c| c == '_') {
        s.push('_');
    }
    if s == "_" {
        s.push('_');
    }
    s
}

// ---- a recorded analysis given to a run ------------------------------------

/// **An analysis given to a run**: the file as given, its digest, what it
/// holds, and what of it the run is given.
#[derive(Clone, Debug)]
pub struct Given {
    pub file: String,
    pub sha256: String,
    pub analysis: Analysis,
    pub choice: Choice,
}

/// One sentence's graph, read.
pub struct Read<'a> {
    pub n: usize,
    pub sentence: &'a str,
    pub of: &'a GraphOf,
    /// The graph, or why it is none: no graph written, or one that does not
    /// read.
    pub graph: Result<penman::Graph, String>,
}

impl<'a> Read<'a> {
    /// The words the aligner gave a node, where they are shown.
    pub fn words(&self, shown: bool, var: &str) -> Vec<&'a str> {
        let Ok(g) = &self.graph else {
            return Vec::new();
        };
        if !shown {
            return Vec::new();
        }
        g.node(var)
            .map(|n| {
                n.align
                    .iter()
                    .filter_map(|&i| self.of.tokens.get(i).map(String::as_str))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Given {
    /// **Read a recorded analysis** and check that it holds what `choice`
    /// asks of it.
    pub fn read(path: &Path, choice: Choice) -> Result<Given, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let text =
            String::from_utf8(bytes).map_err(|_| format!("{}: not UTF-8", path.display()))?;
        Given::of(&text, &path.display().to_string(), choice)
    }

    /// **A recorded analysis from its text**, `file` the name its record
    /// keeps.
    pub fn of(text: &str, file: &str, choice: Choice) -> Result<Given, String> {
        let analysis: Analysis =
            serde_json::from_str(text).map_err(|e| format!("{file}: not an analysis: {e}"))?;
        if analysis.schema != SCHEMA {
            return Err(format!(
                "{file}: the schema is {}, not {SCHEMA}",
                analysis.schema
            ));
        }
        let given = Given {
            file: file.to_string(),
            sha256: sha256_hex(text.as_bytes()),
            analysis,
            choice,
        };
        let parse = given.parse().ok_or_else(|| {
            format!(
                "{file}: no parser {}: it holds {}",
                given.choice.parser,
                given
                    .analysis
                    .amr
                    .iter()
                    .map(|p| p.parser.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
        if parse.graphs.len() != given.analysis.sentences.len()
            || parse.graphs.iter().enumerate().any(|(i, g)| g.n != i + 1)
        {
            return Err(format!(
                "{file}: {}'s graphs are not one a sentence, in order",
                parse.parser
            ));
        }
        if given.choice.entities {
            match &given.analysis.entities {
                Some(e) if e.sentences.len() == given.analysis.sentences.len() => {}
                Some(_) => return Err(format!("{file}: the entities are not a list a sentence")),
                None => return Err(format!("{file}: it holds no named entities")),
            }
        }
        Ok(given)
    }

    /// The parse the run is given.
    pub fn parse(&self) -> Option<&Parse> {
        self.analysis
            .amr
            .iter()
            .find(|p| p.parser == self.choice.parser)
    }

    fn the_parse(&self) -> &Parse {
        self.parse().expect("the parse was checked when read")
    }

    /// **Whether the analysis is of `text`**: its sentences are the ones
    /// EinAf splits the text into.
    pub fn is_of(&self, text: &str) -> Result<(), String> {
        let ours = crate::contract::sentences(text);
        if ours != self.analysis.sentences {
            let at = ours
                .iter()
                .zip(&self.analysis.sentences)
                .position(|(a, b)| a != b)
                .unwrap_or(ours.len().min(self.analysis.sentences.len()));
            return Err(format!(
                "{}: the analysis is of other sentences than the text's: the text has {}, the analysis {}; they part at sentence {}",
                self.file,
                ours.len(),
                self.analysis.sentences.len(),
                at + 1
            ));
        }
        Ok(())
    }

    /// Each sentence's graph, read.
    pub fn graphs(&self) -> Vec<Read<'_>> {
        let parse = self.the_parse();
        parse
            .graphs
            .iter()
            .map(|g| Read {
                n: g.n,
                sentence: &self.analysis.sentences[g.n - 1],
                of: g,
                graph: match &g.penman {
                    Some(p) => penman::read(p),
                    None => Err("the parser wrote no graph".to_string()),
                },
            })
            .collect()
    }

    /// Whether the form shows a node's words: the aligner is learned.
    pub fn words_shown(&self) -> bool {
        self.the_parse().aligner.shown()
    }

    /// The entities of sentence `n`, where they are beside the graphs.
    fn entities(&self, n: usize) -> &[Entity] {
        match (&self.analysis.entities, self.choice.entities) {
            (Some(e), true) => e.sentences.get(n - 1).map_or(&[], Vec::as_slice),
            _ => &[],
        }
    }

    fn entities_line(&self, n: usize, prefix: &str) -> Option<String> {
        let e = self.entities(n);
        if e.is_empty() {
            return None;
        }
        Some(format!(
            "{prefix}named entities: {}",
            e.iter()
                .map(|e| format!("{} ({})", e.text, e.label))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }

    /// **The form alone**: the text a form's template puts in its slot; for
    /// the facts, the program, which the kernel loads.
    pub fn form(&self) -> String {
        match self.choice.form {
            Form::Penman => self.penman_form(),
            Form::Lines => self.lines_form(),
            Form::Facts => facts::program(self).text,
        }
    }

    fn penman_form(&self) -> String {
        let mut blocks = Vec::new();
        for r in self.graphs() {
            let mut b = format!("# ::snt {}", r.sentence);
            if let Some(l) = self.entities_line(r.n, "# ") {
                b.push('\n');
                b.push_str(&l);
            }
            b.push('\n');
            match (&r.graph, &r.of.penman) {
                (Ok(_), Some(p)) => {
                    b.push_str(penman::without_alignments(p).as_deref().unwrap_or(p));
                }
                (Err(why), _) => b.push_str(&format!("# no graph: {why}")),
                (Ok(_), None) => unreachable!("a graph read is a graph written"),
            }
            blocks.push(b);
        }
        blocks.join("\n\n") + "\n"
    }

    fn lines_form(&self) -> String {
        let shown = self.words_shown();
        let mut blocks = Vec::new();
        for r in self.graphs() {
            let mut b = format!("Sentence {}: {}", r.n, r.sentence);
            if let Some(l) = self.entities_line(r.n, "  ") {
                b.push('\n');
                b.push_str(&l);
            }
            match &r.graph {
                Err(why) => b.push_str(&format!("\n  no graph: {why}")),
                Ok(g) => {
                    for node in &g.nodes {
                        b.push_str(&format!(
                            "\n  {} {}",
                            node.var,
                            node.concept.as_deref().unwrap_or("-")
                        ));
                        let words = r.words(shown, &node.var);
                        if !words.is_empty() {
                            b.push_str(&format!(" [{}]", words.join(" ")));
                        }
                        for e in g.out_of(&node.var) {
                            let target = match &e.target {
                                penman::Target::Node(v) | penman::Target::Constant(v) => v.clone(),
                                penman::Target::Str(s) => format!("{s:?}"),
                            };
                            b.push_str(&format!(" :{} {target}", e.role));
                        }
                    }
                }
            }
            blocks.push(b);
        }
        blocks.join("\n\n") + "\n"
    }

    /// **What step 1's prompt holds**: the form, rendered through its
    /// template, the entities' note after its first paragraph where they are
    /// beside the graphs.
    pub fn block(&self) -> String {
        let note = if self.choice.entities {
            format!(
                " {}",
                ENTITIES
                    .render(&Default::default())
                    .expect("the note has no slot")
                    .trim_end()
            )
        } else {
            String::new()
        };
        let slots = [
            ("analysis", self.form().trim_end().to_string()),
            ("entities", note),
        ]
        .into_iter()
        .collect();
        self.choice
            .form
            .template()
            .render(&slots)
            .expect("the analysis's template renders")
    }

    /// **The counts**: the parse's graphs, nodes and nodes its aligner gave
    /// words to.
    pub fn counts(&self) -> Counts {
        counts_of(&self.graphs())
    }

    /// **The record's part**: what of which file the run was given, and the
    /// digests of what its prompt held.
    pub fn record(&self) -> AnalysisOf {
        let parse = self.the_parse();
        let template = self.choice.form.template();
        AnalysisOf {
            schema: SCHEMA.to_string(),
            file: self.file.clone(),
            sha256: self.sha256.clone(),
            text: self.analysis.text.clone(),
            parser: parse.parser.clone(),
            tool: parse.tool.clone(),
            aligner: parse.aligner.clone(),
            words: self.words_shown() && self.choice.form != Form::Penman,
            form: self.choice.form,
            entities: if self.choice.entities {
                self.analysis.entities.as_ref().map(|e| e.tool.clone())
            } else {
                None
            },
            template: template.file.to_string(),
            template_sha256: template.sha256(),
            entities_template_sha256: self.choice.entities.then(|| ENTITIES.sha256()),
            vocabulary_sha256: (self.choice.form == Form::Facts).then(|| VOCABULARY.sha256()),
            block_sha256: sha256_hex(self.block().as_bytes()),
            counts: self.counts(),
        }
    }
}

/// **The counts of a parse**: its sentences, the graphs that read, the
/// concept nodes, and those the aligner gave a token.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub sentences: usize,
    pub graphs: usize,
    pub nodes: usize,
    pub aligned: usize,
}

fn counts_of(reads: &[Read<'_>]) -> Counts {
    let mut c = Counts {
        sentences: reads.len(),
        ..Counts::default()
    };
    for r in reads {
        if let Ok(g) = &r.graph {
            c.graphs += 1;
            c.nodes += g.nodes.len();
            c.aligned += g.nodes.iter().filter(|n| !n.align.is_empty()).count();
        }
    }
    c
}

/// **What a run's record keeps of the analysis it was given.**
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisOf {
    pub schema: String,
    /// The file, as given, and its digest.
    pub file: String,
    pub sha256: String,
    /// The text the analysis is of, by name.
    pub text: String,
    /// The parser whose graphs the prompt held, and its tool.
    pub parser: String,
    pub tool: serde_json::Value,
    /// Its aligner; `words`: whether the form showed a node's words.
    pub aligner: Aligner,
    pub words: bool,
    pub form: Form,
    /// The named entities' tool, where they were beside the graphs.
    pub entities: Option<serde_json::Value>,
    /// The form's template and its digest; the entities' note's and the
    /// vocabulary's digests, where the prompt held them.
    pub template: String,
    pub template_sha256: String,
    pub entities_template_sha256: Option<String>,
    pub vocabulary_sha256: Option<String>,
    /// The digest of the block the prompt held.
    pub block_sha256: String,
    pub counts: Counts,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_written_in_the_characters_a_model_may_write() {
        assert_eq!(name("drink-01"), "drink-01");
        assert_eq!(name("Old Gold"), "Old_Gold");
        assert_eq!(name("and"), "and_");
        assert_eq!(name("O'Brien"), "O_Brien");
        assert_eq!(name("Zürich"), "Z_rich");
        assert_eq!(name("?"), "__");
        assert_eq!(name(""), "__");
        assert_eq!(name("5"), "5");
        assert_eq!(name("Kool-Aid"), "Kool-Aid");
        assert_eq!(name("St. Louis"), "St__Louis");
        assert_eq!(name("1/2"), "1_2");
    }
}
