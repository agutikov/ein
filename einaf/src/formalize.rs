//! **The formalization in steps**: a text becomes an Ein program in three
//! calls of the model, the facts first, then the ontology, then the theory
//! ([the contract, § 3](../../docs/einaf/04-contract.md#3-the-steps)), or in
//! one call, the whole program, a mode kept for comparison.
//!
//! **Each step** is one request ([`crate::prompts`]): the excerpt of Ein,
//! the catalogue (steps 2 and 3), the examples cut to the step, the text and
//! the program of the steps before it, rendered from versioned templates;
//! under the step's grammar ([`crate::grammar::steps`]) inside the fenced
//! block the prompt asks for, held to by the chain's choice first, or under
//! none; thinking off, greedy. Step 2's reply that carries the marker of a
//! missing property, `;; missing: …`, is followed by step 2's second call,
//! which writes the rules under the marker.
//!
//! **The program is assembled in synthesis order** ([`assemble_parts`]):
//! step 1's statements and answer query, then step 2's output, whose
//! ontology stands in place of step 1's, then step 2's second call and step
//! 3, each part under a comment that names its step. **Each step's program
//! is judged at root** through the bounded call (`--no-search`), with the
//! writer of H the kernel reports ([`crate::kernel::root`]) and the
//! contract's check's findings that apply to it ([`crate::check`]); the
//! complete program is asked `--no-search` and `--single`, and its forms
//! are put in the contract's categories, each with the step that wrote it.
//!
//! A run is recorded whole ([`Run`]): every request and generation
//! ([`crate::model::Recorded`], the call record replay reads), each step's
//! program, judgement and findings, the categories.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::catalogue::{self, Catalogue, Generated};
use crate::check::{self, Checked, Step1};
use crate::contract::{self, Row};
use crate::grammar::steps::{self as gsteps, Params, Step};
use crate::grammar::{self, Frame};
use crate::kernel::{self, Bounds, Feedback, Parsed, Question, Root, Source};
use crate::model::{
    self, Failure, Generation, Message, Prompt, Recorded, Request, Runtime, Sampler, sha256_hex,
};
use crate::prompts::{self, AnswerLine, Example, Shown, Template};

/// The schema of a run's record.
pub const SCHEMA: &str = "einaf-steps/1";

/// The marker of a missing property, a comment line of its own.
pub const MARKER: &str = gsteps::MARKER;

/// **How the program is written**: in the three steps, or whole in one call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    #[default]
    Steps,
    Whole,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Mode> {
        match s {
            "steps" => Some(Mode::Steps),
            "whole" => Some(Mode::Whole),
            _ => None,
        }
    }
}

/// **How a request is made**, beside its prompt and grammar. One mode today,
/// thinking off; a later stage adds the thinking it guides.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequestMode {
    /// Thinking off, the grammar from the first token inside the fence.
    #[default]
    ThinkingOff,
}

impl RequestMode {
    pub fn parse(s: &str) -> Option<RequestMode> {
        match s {
            "thinking-off" => Some(RequestMode::ThinkingOff),
            _ => None,
        }
    }

    fn thinking(self) -> bool {
        match self {
            RequestMode::ThinkingOff => false,
        }
    }

    fn frame(self) -> Frame {
        match self {
            RequestMode::ThinkingOff => Frame {
                after_thinking: false,
                fence: true,
            },
        }
    }
}

/// **What a step's output the budget of generated tokens cut becomes.**
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Cut {
    /// Read as it stands: a text cut in a form does not parse, and the step
    /// ends the run.
    #[default]
    End,
    /// **The forms written before the cut**: every top-level form the output
    /// completed, the one the cut stopped in dropped, and every form that
    /// repeats one written before it dropped too, which is the loop a greedy
    /// decoding runs into under a grammar; the comments between them kept. A text
    /// with no complete form is empty, and ends the run.
    KeepForms,
}

impl Cut {
    pub fn parse(s: &str) -> Option<Cut> {
        match s {
            "end" => Some(Cut::End),
            "keep-forms" => Some(Cut::KeepForms),
            _ => None,
        }
    }
}

fn one() -> u32 {
    1
}

/// **The options of a run.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Options {
    pub mode: Mode,
    /// Each request under its step's grammar, or the whole grammar's root in
    /// one call; `false`: no grammar.
    pub grammar: bool,
    pub request: RequestMode,
    pub catalogue: catalogue::Size,
    /// The budget of generated tokens of one call.
    pub max_tokens: u32,
    /// The clock of each kernel call, in seconds.
    pub clock_s: u64,
    /// The seed of every request, recorded; greedy decoding draws nothing.
    #[serde(default = "one")]
    pub seed: u32,
    /// What an output the budget cut becomes.
    #[serde(default)]
    pub cut: Cut,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            mode: Mode::Steps,
            grammar: true,
            request: RequestMode::ThinkingOff,
            catalogue: catalogue::Size::Whole,
            max_tokens: 12_288,
            clock_s: 60,
            seed: 1,
            cut: Cut::End,
        }
    }
}

/// **What a run is given**: the text, its name, and its answer line.
#[derive(Clone, Debug)]
pub struct Input {
    pub id: String,
    pub text: String,
    pub answer_line: AnswerLine,
}

/// What the kernel said of one step's program at root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Judged {
    /// The bounded call under each question asked, `--no-search` first.
    pub feedback: Vec<Feedback>,
    /// Root and H: the writer of H, its size, root's narration.
    pub root: RootSummary,
    /// The contract's check, the findings that apply to the step.
    pub check: Checked,
}

/// **Root, as the record keeps it**: the writer of H and what root fired,
/// counted.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootSummary {
    pub writer: Option<String>,
    pub candidates: usize,
    pub root_dead: bool,
    pub monotone: bool,
    /// Each rule root fired, and how often.
    pub fired: BTreeMap<String, usize>,
    /// Each obligation root owes, and how many instances.
    pub owed: BTreeMap<String, usize>,
    pub stopped: Option<String>,
    pub refused: Vec<kernel::Finding>,
    /// Root's narration outgrew what is kept of it.
    #[serde(default)]
    pub narration_cut: bool,
}

impl RootSummary {
    fn of(root: &Root) -> RootSummary {
        let count = |list: &[kernel::Fired]| {
            let mut m = BTreeMap::new();
            for f in list {
                *m.entry(f.rule.clone()).or_insert(0) += 1;
            }
            m
        };
        RootSummary {
            writer: root.writer.clone(),
            candidates: root.candidates,
            root_dead: root.root_dead,
            monotone: root.monotone,
            fired: count(&root.fired),
            owed: count(&root.owed),
            stopped: root.stopped.clone(),
            refused: root.refused.clone(),
            narration_cut: root.narration_cut,
        }
    }
}

/// **One call of a run**, as the record keeps it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepRun {
    /// `1`, `2`, `2b`, `3`, or `whole`.
    pub step: String,
    /// The template's file and its digest.
    pub template: String,
    pub template_sha256: String,
    /// The rendered prompt's digest: the system message, a line, the user
    /// message.
    pub prompt_sha256: String,
    /// The grammar's digest; `null` with no grammar.
    pub grammar_sha256: Option<String>,
    /// The request's digest, the key of its call in the call record.
    pub digest: String,
    /// The generation's tokens, its prompt's, and how it stopped.
    pub generated_tokens: Option<usize>,
    pub prompt_tokens: Option<usize>,
    pub stop: Option<String>,
    pub generation_ms: Option<f64>,
    /// The call's failure, where it failed.
    pub failure: Option<String>,
    /// What the step wrote: its fenced block, or its whole answer where it
    /// wrote none.
    pub output: Option<String>,
    /// Whether the output was read from a fenced block.
    pub fenced: bool,
    /// Where the budget cut the output and [`Cut::KeepForms`] took the forms
    /// written before the cut: what was dropped. `output` is what was kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cut: Option<CutOff>,
    /// The parse of the output alone.
    pub parsed: Option<Parsed>,
    /// The markers of a missing property it wrote.
    pub markers: Vec<String>,
    /// The program assembled up to this step.
    pub program: Option<String>,
    /// The judgement of that program at root, and the check's findings.
    pub judged: Option<Judged>,
}

/// **What [`Cut::KeepForms`] dropped of an output the budget cut.**
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutOff {
    /// The complete forms kept.
    pub kept: usize,
    /// The complete forms dropped, each a repeat of one written before it.
    pub repeats: usize,
    /// The bytes after the last form kept: the form the cut stopped in,
    /// and the repeats' text.
    pub dropped_bytes: usize,
}

/// **The forms an output the budget cut wrote before the cut**
/// ([`Cut::KeepForms`]): the text up to the end of its last complete
/// top-level form, less each form that repeats one before it, each kept form
/// with the comment lines above it.
pub fn keep_forms(output: &str) -> (String, CutOff) {
    let spans = top_level_spans(output);
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut kept = String::new();
    let mut off = CutOff::default();
    let mut prev_end = 0;
    for (s, e) in spans {
        let form = &output[s..e];
        let between = &output[prev_end..s];
        prev_end = e;
        if !seen.insert(form.trim()) {
            off.repeats += 1;
            continue;
        }
        kept.push_str(between);
        kept.push_str(form);
        off.kept += 1;
    }
    if !kept.is_empty() {
        kept.push('\n');
    }
    off.dropped_bytes = output.len().saturating_sub(kept.len());
    (kept, off)
}

/// **What step 2 changed of step 1's ontology**, by the forms' canonical
/// texts: expected, recorded, and no finding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologyChange {
    pub kept: usize,
    /// Step 1's ontology forms step 2's ontology does not hold.
    pub dropped: Vec<String>,
    /// Step 2's ontology forms step 1's did not hold.
    pub added: Vec<String>,
    /// Relations step 1 declared that step 2 declares otherwise.
    pub redeclared: Vec<String>,
}

/// **A run, whole**: what was asked, each call, the program, its judgement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub schema: String,
    pub id: String,
    pub options: Options,
    pub text_sha256: String,
    pub answer_line: String,
    pub answer_line_sha256: String,
    /// The excerpt's and the catalogue's digests, and the catalogue's size.
    pub excerpt_sha256: String,
    pub catalogue_sha256: String,
    /// Every program a prompt quoted, from the package's root.
    pub quoted: Vec<String>,
    /// The directory of modules given beside the library, as given; `null`
    /// in the main arms.
    #[serde(default)]
    pub module_dir: Option<String>,
    pub steps: Vec<StepRun>,
    /// Where the chain ended before its last step, and why.
    pub stopped: Option<String>,
    /// The complete program, assembled.
    pub program: Option<String>,
    /// The complete program's judgement: `--no-search` and `--single`.
    pub complete: Option<Judged>,
    /// Each form of the complete program in its row, with the step that
    /// wrote it.
    pub categories: Vec<Row>,
    pub ontology_change: Option<OntologyChange>,
}

/// **Split step 1's output** into its statements, everything before its
/// first `relation` form and the comment lines just above it, and its
/// ontology, the rest.
pub fn split_step1(output: &str) -> (String, String) {
    let spans = top_level_spans(output);
    let Some(&(start, _)) = spans
        .iter()
        .find(|(s, e)| output[*s..*e].starts_with("(relation"))
    else {
        return (output.to_string(), String::new());
    };
    // Back over the comment and blank lines directly above it.
    let mut cut = output[..start].rfind('\n').map_or(0, |i| i + 1);
    loop {
        if cut == 0 {
            break;
        }
        let prev_end = cut - 1;
        let prev_start = output[..prev_end].rfind('\n').map_or(0, |i| i + 1);
        let line = output[prev_start..prev_end].trim();
        if line.starts_with(';') || line.is_empty() {
            cut = prev_start;
        } else {
            break;
        }
    }
    (
        output[..cut].trim_end().to_string() + "\n",
        output[cut..].to_string(),
    )
}

/// The byte ranges of a text's top-level forms: a paren at depth 0 to its
/// match, strings and comments skipped.
pub fn top_level_spans(text: &str) -> Vec<(usize, usize)> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    if b[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
            }
            b'(' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b')' if depth > 0 => {
                depth -= 1;
                if depth == 0 {
                    out.push((start, i + 1));
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The heading of a part of the assembled program.
fn heading(step: &str) -> &'static str {
    match step {
        "1" => ";; ---- step 1: the statements and the answer query",
        "2" => ";; ---- step 2: the ontology whole, the imports and the selection",
        "2b" => ";; ---- step 2, its second call: the rules of a missing property",
        "3" => ";; ---- step 3: the rest of the theory and the hypothesis mechanism",
        _ => ";; ---- the whole program, in one call",
    }
}

/// **The program assembled in synthesis order**: step 1's statements, then
/// each later part, each under the comment that names its step.
pub fn assemble_parts(statements: &str, later: &[&str]) -> String {
    let names = ["2", "3"];
    let mut s = format!("{}\n\n{}", heading("1"), statements.trim_end());
    for (i, part) in later.iter().enumerate() {
        s.push_str(&format!(
            "\n\n{}\n\n{}",
            heading(names.get(i).copied().unwrap_or("3")),
            part.trim_end()
        ));
    }
    s.push('\n');
    s
}

/// The program from named parts, each under its heading.
fn assemble(parts: &[(&str, &str)]) -> String {
    let mut s = String::new();
    for (step, text) in parts {
        if !s.is_empty() {
            s.push_str("\n\n");
        }
        s.push_str(heading(step));
        s.push_str("\n\n");
        s.push_str(text.trim_end());
    }
    s.push('\n');
    s
}

/// The markers a text writes, each its property's words.
pub fn markers(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.trim_start().strip_prefix(MARKER))
        .map(|m| m.trim().to_string())
        .collect()
}

/// What the requests of a run share.
struct Shared<'a> {
    input: &'a Input,
    options: Options,
    catalogue: Catalogue,
    generated: Generated,
    catalogue_text: String,
    system: String,
}

impl Shared<'_> {
    /// Where a program's local imports are found: the given directory.
    fn base_dir(&self) -> Option<&std::path::Path> {
        self.catalogue.base_dir()
    }

    fn bounds(&self) -> Bounds {
        Bounds {
            clock: Some(Duration::from_secs(self.options.clock_s)),
            ..Bounds::default()
        }
    }

    /// A request: the system message, the user message, the grammar.
    fn request(&self, user: String, grammar: Option<model::Grammar>) -> Request {
        Request {
            prompt: Prompt::Chat {
                messages: vec![
                    Message::new("system", self.system.clone()),
                    Message::new("user", user),
                ],
                tools: Vec::new(),
            },
            thinking: self.options.request.thinking(),
            grammar,
            sampler: Sampler::greedy(),
            seed: self.options.seed,
            max_tokens: self.options.max_tokens,
            logprobs: None,
            keep: None,
        }
    }

    /// Judge a program at root: the bounded call, root and H, the check.
    fn judge(
        &self,
        program: &str,
        name: &str,
        step: u8,
        step1: Option<&Step1>,
        questions: &[Question],
    ) -> Judged {
        let source = Source::Text {
            text: program,
            name,
            base_dir: self.base_dir(),
        };
        let feedback = kernel::call(source, questions, self.bounds());
        let root = kernel::root(source, self.bounds());
        let check = check::check(&check::Input {
            program,
            name,
            step,
            text: &self.input.text,
            catalogue: &self.catalogue,
            generated: &self.generated,
            step1,
            root: &root,
        });
        Judged {
            feedback,
            root: RootSummary::of(&root),
            check,
        }
    }
}

/// The user message of a step, rendered.
fn render(template: &Template, slots: &[(&'static str, String)]) -> Result<String, String> {
    let map: BTreeMap<&str, String> = slots.iter().cloned().collect();
    template.render(&map)
}

/// **Run the formalization of `input`**, each request answered by
/// `runtime`; every call is appended to `calls`.
pub fn run(
    runtime: &mut dyn Runtime,
    input: &Input,
    options: Options,
    calls: &mut Vec<Recorded>,
) -> Run {
    run_with(
        runtime,
        input,
        options,
        Catalogue::new(options.catalogue),
        calls,
    )
}

/// **Run the formalization of `input` over `catalogue`**, a directory of
/// modules given beside the library among it ([`Catalogue::with_modules`]).
pub fn run_with(
    runtime: &mut dyn Runtime,
    input: &Input,
    options: Options,
    catalogue: Catalogue,
    calls: &mut Vec<Recorded>,
) -> Run {
    let catalogue_text = catalogue.text();
    let module_dir = catalogue.base_dir().map(|d| d.display().to_string());
    let excerpt = prompts::EXCERPT.text.to_string();
    let system =
        render(&prompts::SYSTEM, &[("excerpt", excerpt)]).expect("the system message renders");
    let shared = Shared {
        input,
        options,
        generated: catalogue::generated(),
        catalogue_text,
        catalogue,
        system,
    };
    let mut quoted: Vec<String> = prompts::EXAMPLES.iter().flat_map(Example::files).collect();
    quoted.extend(shared.catalogue.entries.iter().map(|e| e.file()));
    let answer_line = input
        .answer_line
        .template
        .render(&BTreeMap::new())
        .unwrap_or_default();
    let mut run = Run {
        schema: SCHEMA.to_string(),
        id: input.id.clone(),
        options,
        text_sha256: sha256_hex(input.text.as_bytes()),
        answer_line: input.answer_line.template.file.to_string(),
        answer_line_sha256: input.answer_line.template.sha256(),
        excerpt_sha256: prompts::EXCERPT.sha256(),
        catalogue_sha256: sha256_hex(shared.catalogue_text.as_bytes()),
        quoted,
        module_dir,
        steps: Vec::new(),
        stopped: None,
        program: None,
        complete: None,
        categories: Vec::new(),
        ontology_change: None,
    };
    let text = input.text.trim_end().to_string();
    match options.mode {
        Mode::Whole => run_whole(
            runtime,
            &shared,
            &text,
            answer_line.trim_end(),
            &mut run,
            calls,
        ),
        Mode::Steps => run_steps(
            runtime,
            &shared,
            &text,
            answer_line.trim_end(),
            &mut run,
            calls,
        ),
    }
    run
}

/// One call: render, ask, read the fenced block, parse.
#[allow(clippy::too_many_arguments)]
fn call(
    runtime: &mut dyn Runtime,
    shared: &Shared<'_>,
    step: &str,
    template: &Template,
    user: String,
    grammar: Option<gsteps::Rendered>,
    whole_root: bool,
    calls: &mut Vec<Recorded>,
) -> StepRun {
    let frame = shared.options.request.frame();
    let (g, grammar_sha) = if whole_root {
        let text = grammar::root_text();
        let sha = sha256_hex(text.as_bytes());
        (
            Some(grammar::request_grammar(&text, "root", frame)),
            Some(sha),
        )
    } else {
        match &grammar {
            Some(r) => (Some(r.request(frame)), Some(r.sha256.clone())),
            None => (None, None),
        }
    };
    let request = shared.request(user.clone(), g);
    let result: Result<Generation, Failure> = runtime.generate(&request);
    calls.push(Recorded::new(&request, &result));
    let mut out = StepRun {
        step: step.to_string(),
        template: template.file.to_string(),
        template_sha256: template.sha256(),
        prompt_sha256: sha256_hex(format!("{}\n{}", shared.system, user).as_bytes()),
        grammar_sha256: grammar_sha,
        digest: request.digest(),
        generated_tokens: None,
        prompt_tokens: None,
        stop: None,
        generation_ms: None,
        failure: None,
        output: None,
        fenced: false,
        cut: None,
        parsed: None,
        markers: Vec::new(),
        program: None,
        judged: None,
    };
    match result {
        Ok(g) => {
            out.generated_tokens = Some(g.envelope.generation.count());
            out.prompt_tokens = Some(g.envelope.prompt.tokens);
            out.stop = serde_json::to_value(g.envelope.generation.stop)
                .ok()
                .and_then(|v| {
                    v.as_str()
                        .map(str::to_string)
                        .or_else(|| Some(v.to_string()))
                });
            out.generation_ms = Some(g.envelope.generation.generation_ms);
            let (mut text, fenced) = match model::first_fenced_block(&g.answer) {
                Some(b) => (b, true),
                None => (g.answer.clone(), false),
            };
            out.fenced = fenced;
            if g.envelope.generation.stop == model::Stop::MaxTokens
                && shared.options.cut == Cut::KeepForms
            {
                let (kept, off) = keep_forms(&text);
                text = kept;
                out.cut = Some(off);
            }
            out.parsed = Some(kernel::parse(&text, &format!("step {step}")));
            out.markers = markers(&text);
            out.output = Some(text);
        }
        Err(f) => out.failure = Some(f.to_string()),
    }
    out
}

fn parses(s: &StepRun) -> bool {
    s.parsed.as_ref().is_some_and(|p| p.forms.is_some())
        && s.output.as_deref().is_some_and(|o| !o.trim().is_empty())
}

fn run_steps(
    runtime: &mut dyn Runtime,
    shared: &Shared<'_>,
    text: &str,
    answer_line: &str,
    run: &mut Run,
    calls: &mut Vec<Recorded>,
) {
    let opts = shared.options;
    let sentences = contract::sentences(text);
    let grammar_catalogue = shared.catalogue.grammar();
    let rendered = |step: Step, program: Option<&str>| -> Option<gsteps::Rendered> {
        if !opts.grammar {
            return None;
        }
        let mut p = Params {
            sentences: Some(sentences.clone()),
            ..Params::default()
        };
        if step != Step::One {
            p.catalogue = grammar_catalogue.clone();
        }
        if let Some(program) = program {
            p.program = kernel::vocabulary(program, "program").ok()?;
        }
        Some(gsteps::render(step, &p))
    };
    let id = &shared.input.id;

    // Step 1.
    let user = render(
        &prompts::STEP1,
        &[
            (
                "examples",
                prompts::examples_block(&prompts::EXAMPLES, Shown::UpTo(1)),
            ),
            ("text", text.to_string()),
            ("answer_line", answer_line.to_string()),
        ],
    )
    .expect("step 1's prompt renders");
    let mut s1 = call(
        runtime,
        shared,
        "1",
        &prompts::STEP1,
        user,
        rendered(Step::One, None),
        false,
        calls,
    );
    if !parses(&s1) {
        run.stopped = Some("step 1's output does not parse, or is empty".to_string());
        run.steps.push(s1);
        return;
    }
    let out1 = s1.output.clone().unwrap_or_default();
    let (statements, ontology1) = split_step1(&out1);
    let program1 = assemble(&[("1", &out1)]);
    s1.judged = Some(shared.judge(
        &program1,
        &format!("{id}.step1.ein"),
        1,
        None,
        &[Question::NoSearch],
    ));
    s1.program = Some(program1.clone());
    run.steps.push(s1);
    let step1 = check::step1_of(&out1);

    // Step 2.
    let user = render(
        &prompts::STEP2,
        &[
            ("catalogue", shared.catalogue_text.clone()),
            (
                "examples",
                prompts::examples_block(&prompts::EXAMPLES, Shown::UpTo(2)),
            ),
            ("text", text.to_string()),
            ("program", out1.trim_end().to_string() + "\n"),
        ],
    )
    .expect("step 2's prompt renders");
    let mut s2 = call(
        runtime,
        shared,
        "2",
        &prompts::STEP2,
        user,
        rendered(Step::Two, Some(&out1)),
        false,
        calls,
    );
    if !parses(&s2) {
        run.stopped = Some("step 2's output does not parse, or is empty".to_string());
        run.steps.push(s2);
        return;
    }
    let out2 = s2.output.clone().unwrap_or_default();
    run.ontology_change = Some(ontology_change(&ontology1, &out2));
    let program2 = assemble(&[("1", &statements), ("2", &out2)]);
    s2.judged = Some(shared.judge(
        &program2,
        &format!("{id}.step2.ein"),
        2,
        Some(&step1),
        &[Question::NoSearch],
    ));
    s2.program = Some(program2.clone());
    let wrote_markers = s2.markers.clone();
    run.steps.push(s2);

    // Step 2's second call, on a marker.
    let mut parts: Vec<(&str, String)> = vec![("1", statements.clone()), ("2", out2.clone())];
    if !wrote_markers.is_empty() {
        let user = render(
            &prompts::STEP2B,
            &[
                (
                    "markers",
                    wrote_markers
                        .iter()
                        .map(|m| format!("{MARKER}{m}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                ),
                ("catalogue", shared.catalogue_text.clone()),
                (
                    "examples",
                    prompts::examples_block(&prompts::EXAMPLES, Shown::UpTo(3)),
                ),
                ("text", text.to_string()),
                ("program", program2.clone()),
            ],
        )
        .expect("step 2b's prompt renders");
        let mut s2b = call(
            runtime,
            shared,
            "2b",
            &prompts::STEP2B,
            user,
            rendered(Step::TwoRules, Some(&program2)),
            false,
            calls,
        );
        if parses(&s2b) {
            let out = s2b.output.clone().unwrap_or_default();
            parts.push(("2b", out));
            let refs: Vec<(&str, &str)> = parts.iter().map(|(a, b)| (*a, b.as_str())).collect();
            let p = assemble(&refs);
            s2b.judged = Some(shared.judge(
                &p,
                &format!("{id}.step2b.ein"),
                2,
                Some(&step1),
                &[Question::NoSearch],
            ));
            s2b.program = Some(p);
        }
        run.steps.push(s2b);
    }
    let refs: Vec<(&str, &str)> = parts.iter().map(|(a, b)| (*a, b.as_str())).collect();
    let before3 = assemble(&refs);

    // Step 3.
    let user = render(
        &prompts::STEP3,
        &[
            ("catalogue", shared.catalogue_text.clone()),
            (
                "examples",
                prompts::examples_block(&prompts::EXAMPLES, Shown::UpTo(3)),
            ),
            ("text", text.to_string()),
            ("program", before3.clone()),
        ],
    )
    .expect("step 3's prompt renders");
    let mut s3 = call(
        runtime,
        shared,
        "3",
        &prompts::STEP3,
        user,
        rendered(Step::Three, Some(&before3)),
        false,
        calls,
    );
    if !parses(&s3) {
        run.stopped = Some("step 3's output does not parse, or is empty".to_string());
        run.steps.push(s3);
        // The program of steps 1 and 2 is what there is.
        complete(shared, run, &before3, &program1, &before3, Some(&step1));
        return;
    }
    let out3 = s3.output.clone().unwrap_or_default();
    parts.push(("3", out3));
    let refs: Vec<(&str, &str)> = parts.iter().map(|(a, b)| (*a, b.as_str())).collect();
    let program3 = assemble(&refs);
    s3.program = Some(program3.clone());
    run.steps.push(s3);
    complete(shared, run, &program3, &program1, &before3, Some(&step1));
    // Step 3's own judgement is the complete program's.
    if let Some(last) = run.steps.last_mut() {
        last.judged = run.complete.clone();
    }
}

/// The complete program's judgement and categories.
fn complete(
    shared: &Shared<'_>,
    run: &mut Run,
    program: &str,
    program1: &str,
    program2: &str,
    step1: Option<&Step1>,
) {
    let id = &shared.input.id;
    run.program = Some(program.to_string());
    run.complete = Some(shared.judge(
        program,
        &format!("{id}.ein"),
        3,
        step1,
        &[Question::NoSearch, Question::Single(kernel::Part::All)],
    ));
    let forms = |p: &str, n: &str| {
        kernel::forms(Source::Text {
            text: p,
            name: n,
            base_dir: shared.base_dir(),
        })
        .ok()
    };
    let text = &shared.input.text;
    run.categories = match (
        forms(program1, "1"),
        forms(program2, "2"),
        forms(program, "3"),
    ) {
        (Some(a), Some(b), Some(c)) => contract::categories_by_steps(&[&a, &b, &c], text),
        (_, _, Some(c)) => contract::categories(&c, text),
        _ => Vec::new(),
    };
}

fn run_whole(
    runtime: &mut dyn Runtime,
    shared: &Shared<'_>,
    text: &str,
    answer_line: &str,
    run: &mut Run,
    calls: &mut Vec<Recorded>,
) {
    let user = render(
        &prompts::WHOLE,
        &[
            ("catalogue", shared.catalogue_text.clone()),
            (
                "examples",
                prompts::examples_block(&prompts::EXAMPLES, Shown::Whole),
            ),
            ("text", text.to_string()),
            ("answer_line", answer_line.to_string()),
        ],
    )
    .expect("the whole prompt renders");
    let mut s = call(
        runtime,
        shared,
        "whole",
        &prompts::WHOLE,
        user,
        None,
        shared.options.grammar,
        calls,
    );
    if !parses(&s) {
        run.stopped = Some("the output does not parse, or is empty".to_string());
        run.steps.push(s);
        return;
    }
    let out = s.output.clone().unwrap_or_default();
    let program = assemble(&[("whole", &out)]);
    s.program = Some(program.clone());
    run.steps.push(s);
    let id = &shared.input.id;
    run.program = Some(program.clone());
    run.complete = Some(shared.judge(
        &program,
        &format!("{id}.ein"),
        3,
        None,
        &[Question::NoSearch, Question::Single(kernel::Part::All)],
    ));
    if let Ok(f) = kernel::forms(Source::Text {
        text: &program,
        name: id,
        base_dir: shared.base_dir(),
    }) {
        run.categories = contract::categories(&f, &shared.input.text);
    }
    if let Some(last) = run.steps.last_mut() {
        last.judged = run.complete.clone();
    }
}

/// What step 2's ontology changed of step 1's: by canonical text, the
/// imports and the selecting facts aside.
fn ontology_change(ontology1: &str, out2: &str) -> OntologyChange {
    let forms = |t: &str| kernel::shapes(t, "ontology").unwrap_or_default();
    let one = forms(ontology1);
    let two: Vec<kernel::Form> = forms(out2)
        .into_iter()
        .take_while(|f| !matches!(f.shape, kernel::Shape::Import { .. }))
        .collect();
    let set = |v: &[kernel::Form]| -> BTreeSet<String> {
        v.iter().map(|f| f.canonical.clone()).collect()
    };
    let (a, b) = (set(&one), set(&two));
    let declared = |v: &[kernel::Form]| -> BTreeMap<String, String> {
        v.iter()
            .filter_map(|f| match &f.shape {
                kernel::Shape::Relation { name } => Some((name.clone(), f.canonical.clone())),
                _ => None,
            })
            .collect()
    };
    let (d1, d2) = (declared(&one), declared(&two));
    OntologyChange {
        kept: a.intersection(&b).count(),
        dropped: a.difference(&b).cloned().collect(),
        added: b.difference(&a).cloned().collect(),
        redeclared: d1
            .iter()
            .filter(|(n, c)| d2.get(*n).is_some_and(|c2| c2 != *c))
            .map(|(n, _)| n.clone())
            .collect(),
    }
}

/// **The hand's steps as one generation a step**: a directory of
/// `einaf/contract/`'s `step1.ein` to `step3.ein`, each the program so far,
/// cut into what each step wrote: step 1 whole; step 2 its program less
/// step 1's statements and answer query; step 3 its program less step 2's
/// forms. A rule keeps the comment lines directly above it.
pub fn hand_parts(step1: &str, step2: &str, step3: &str) -> [String; 3] {
    let canon = |t: &str| -> BTreeSet<String> {
        kernel::shapes(t, "hand")
            .map(|v| v.into_iter().map(|f| f.canonical).collect())
            .unwrap_or_default()
    };
    let (statements, _) = split_step1(step1);
    let drop1 = canon(&statements);
    let drop2 = canon(step2);
    [
        step1.to_string(),
        forms_not_in(step2, &drop1),
        forms_not_in(step3, &drop2),
    ]
}

/// The forms of `text` whose canonical text `drop` does not hold, each
/// with the comment lines directly above it, in their order.
fn forms_not_in(text: &str, drop: &BTreeSet<String>) -> String {
    let mut out = Vec::new();
    let spans = top_level_spans(text);
    let mut prev_end = 0;
    for (s, e) in spans {
        let form = &text[s..e];
        let canonical = kernel::shapes(form, "form")
            .ok()
            .and_then(|v| v.into_iter().next())
            .map(|f| f.canonical)
            .unwrap_or_default();
        let between = &text[prev_end..s];
        prev_end = e;
        if drop.contains(&canonical) {
            continue;
        }
        // The comment lines directly above the form, after the last blank
        // line between it and the form before.
        let comments: Vec<&str> = between
            .split("\n\n")
            .last()
            .unwrap_or("")
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with(';'))
            .collect();
        let mut block = comments.join("\n");
        if !block.is_empty() {
            block.push('\n');
        }
        block.push_str(form);
        out.push(block);
    }
    out.join("\n\n") + "\n"
}
