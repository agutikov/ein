//! **The model's runtime**: one trait, [`Runtime`], through which EinAf asks
//! the model for a generation, and what every call returns, a [`Generation`]
//! with its [`Envelope`], or a typed [`Failure`].
//!
//! Two implementations: [`replay::Replay`], which answers from a record and
//! needs no model, and is what the offline tests use; and, behind the `llama`
//! feature, `llama::Llama`, the binding to llama.cpp, the model in EinAf's
//! process. (A third, over `llama-server`, was to be written only if no
//! binding served; one did.) The chat template the model's file carries
//! is rendered by EinAf ([`template`]), so a rendered prompt is the same
//! with the feature or without it.
//!
//! **A request is data**: its messages, whether the model thinks, the
//! grammar and where it begins, the sampler chain in order, the seed, the
//! budget of generated tokens. Its digest, [`Request::digest`], is what a
//! record is found by. **A generation keeps the thinking apart from the
//! answer**, and counts each; reaching the budget is a [`Stop`], not a
//! failure. There is no transport, so there is no retry.
//!
//! [The model's runtime](../../../docs/einaf/05-model.md) is this module's
//! page.

pub mod replay;
pub mod template;

#[cfg(feature = "llama")]
pub mod llama;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The record's schema: a line a generation, its request beside its
/// generation or its failure. The number moves when a field changes its
/// meaning.
pub const SCHEMA: &str = "einaf-generation/1";

/// One message of a chat, as the template reads it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// `system`, `user`, `assistant` or `tool`.
    pub role: String,
    pub content: String,
    /// An assistant's thinking, which the template writes back inside
    /// `<think>` where the turn follows the last question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
}

impl Message {
    pub fn new(role: &str, content: impl Into<String>) -> Self {
        Message {
            role: role.to_string(),
            content: content.into(),
            reasoning_content: None,
        }
    }
}

/// A parameter of a declared function: a string, by its name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolParam {
    pub name: String,
    pub description: String,
}

/// **A function declared to the model**: its name, what it does, and its
/// parameters, every one a string and every one required. The template
/// writes it as the chat API's `{"type": "function", "function": {…}}`, keys
/// in that order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolDecl {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParam>,
}

/// What the model is given: a chat rendered through the model's own
/// template, or a text tokenised as it stands.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Prompt {
    Chat {
        messages: Vec<Message>,
        #[serde(default)]
        tools: Vec<ToolDecl>,
    },
    Text {
        text: String,
    },
}

/// Where a grammar begins.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "at", rename_all = "kebab-case")]
pub enum GrammarStart {
    /// At the first generated token: the grammar holds the whole output.
    First,
    /// **Lazily**, where the generated text first matches one of the
    /// patterns whole; the grammar is fed from the pattern's first group.
    Patterns { patterns: Vec<String> },
}

/// A GBNF grammar: its text, its root, where it begins, and how a token is
/// held to it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grammar {
    pub text: String,
    pub root: String,
    pub start: GrammarStart,
    /// How a generated token is held to the grammar; absent, every token.
    #[serde(default, skip_serializing_if = "GrammarCheck::is_every_token")]
    pub check: GrammarCheck,
}

/// **How a generated token is held to a grammar.** Under greedy decoding
/// the two choose the same tokens: the most likely token the grammar allows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrammarCheck {
    /// The grammar is applied to every candidate, then the chain chooses:
    /// a grammar that leaves no token is seen at once.
    #[default]
    EveryToken,
    /// **The chain chooses first, and its choice alone is checked**; where
    /// the grammar refuses it, the grammar is applied to every candidate and
    /// the chain chooses again, as llama.cpp's own sampler of the server
    /// does. Every candidate is read only where the choice is refused, and
    /// while a lazy grammar has not begun.
    ChosenFirst,
}

impl GrammarCheck {
    pub fn is_every_token(&self) -> bool {
        *self == GrammarCheck::EveryToken
    }
}

/// **One sampler of the chain**, with its parameters, applied in the
/// chain's order after the grammar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sampler", rename_all = "kebab-case")]
pub enum Sampler {
    /// The most likely token.
    Greedy,
    /// Repetition penalties over the last `last_n` tokens.
    Penalties {
        last_n: i32,
        repeat: f32,
        frequency: f32,
        presence: f32,
    },
    TopK {
        k: i32,
    },
    TopP {
        p: f32,
        min_keep: usize,
    },
    MinP {
        p: f32,
        min_keep: usize,
    },
    Temp {
        t: f32,
    },
    /// A draw from what is left, seeded by the request's seed.
    Dist,
}

impl Sampler {
    /// **Greedy decoding**, this phase's own runs: one run is the whole
    /// distribution.
    pub fn greedy() -> Vec<Sampler> {
        vec![Sampler::Greedy]
    }

    /// **The model's own defaults**, as its card gives them: presence
    /// penalty 1.5, top-k 20, top-p 0.95, temperature 1, then a draw.
    pub fn model_default() -> Vec<Sampler> {
        vec![
            Sampler::Penalties {
                last_n: 64,
                repeat: 1.0,
                frequency: 0.0,
                presence: 1.5,
            },
            Sampler::TopK { k: 20 },
            Sampler::TopP {
                p: 0.95,
                min_keep: 1,
            },
            Sampler::Temp { t: 1.0 },
            Sampler::Dist,
        ]
    }
}

/// **A request for one generation.**
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub prompt: Prompt,
    /// Whether the model thinks before it answers; a chat's template opens
    /// or closes the thinking block in the prompt.
    pub thinking: bool,
    pub grammar: Option<Grammar>,
    /// The chain, in order; the grammar, if any, comes first.
    pub sampler: Vec<Sampler>,
    pub seed: u32,
    /// The budget of generated tokens, the thinking included.
    pub max_tokens: u32,
    /// How many alternatives a token's log-probability is returned with;
    /// `None` for none.
    pub logprobs: Option<u32>,
    /// Continue the kept context of this name: the tokens it holds are not
    /// evaluated again where the prompt begins with them.
    pub keep: Option<u32>,
}

impl Request {
    /// A chat of one user message, greedy, with no grammar.
    pub fn chat(messages: Vec<Message>, thinking: bool, max_tokens: u32) -> Self {
        Request {
            prompt: Prompt::Chat {
                messages,
                tools: Vec::new(),
            },
            thinking,
            grammar: None,
            sampler: Sampler::greedy(),
            seed: 1,
            max_tokens,
            logprobs: None,
            keep: None,
        }
    }

    /// **What a record is found by**: `sha256:` of the request as JSON.
    pub fn digest(&self) -> String {
        let json = serde_json::to_string(self).expect("a request is JSON");
        sha256_hex(json.as_bytes())
    }
}

/// Why a generation stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stop {
    /// The model ended its turn.
    EndOfTurn,
    /// The budget of generated tokens was spent.
    MaxTokens,
    /// The context was full.
    ContextFull,
}

/// A generated token's log-probability under the model, before the grammar
/// and the sampler chain, and its most likely alternatives.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenLogprob {
    pub token: i32,
    pub logprob: f32,
    pub top: Vec<(i32, f32)>,
}

/// **What one call generated**, the thinking apart from the answer, and the
/// envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Generation {
    /// What the model wrote before it closed its thinking; empty with
    /// thinking off.
    pub thinking: String,
    /// What it wrote after, its end of turn not included.
    pub answer: String,
    pub logprobs: Option<Vec<TokenLogprob>>,
    pub envelope: Envelope,
}

/// **The envelope**: what every call returns and every record keeps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    /// [`SCHEMA`].
    pub schema: String,
    pub model: ModelPin,
    pub runtime: RuntimeId,
    pub context: ContextId,
    /// `null` for a text prompt, which no template renders.
    pub template: Option<TemplateId>,
    pub prompt: PromptId,
    pub sampler: SamplerId,
    pub grammar: Option<GrammarId>,
    pub generation: GenerationId,
}

/// **The model, pinned**: its file's SHA-256, and what its header says.
/// The file's path is no part of it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPin {
    /// `sha256:` and the hex digest of the file, as hashed when it was
    /// loaded.
    pub sha256: String,
    /// `general.architecture`.
    pub architecture: String,
    /// `general.size_label`.
    pub size_label: String,
    /// The count of parameters, as llama.cpp counts them.
    pub parameters: u64,
    /// The quantization, from `general.file_type`.
    pub quantization: String,
    /// The context length the model was trained with.
    pub trained_context: u32,
    pub layers: u32,
    pub vocabulary: u32,
}

/// **Which runtime answered.**
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeId {
    /// The binding's crate, `llama-cpp-2`, or `replay`.
    pub binding: String,
    pub binding_version: String,
    /// The commit of llama.cpp the binding built.
    pub llama_cpp: String,
    /// The backend the model's layers run on: `CUDA0`, `CPU`.
    pub backend: String,
    /// The device's description.
    pub device: String,
    /// The layers on the device, of the model's layers.
    pub gpu_layers: u32,
}

/// The context the generation ran in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextId {
    /// Its length in tokens.
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_ubatch: u32,
    pub flash_attention: String,
    /// The device's free memory before the model was loaded, in MiB.
    pub device_free_mib_at_load: Option<u64>,
    /// The device's free memory once the context was made, in MiB.
    pub device_free_mib_loaded: Option<u64>,
    pub device_total_mib: Option<u64>,
}

/// The chat template, which is a file of the record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateId {
    /// `sha256:` of the template's text.
    pub sha256: String,
    /// What rendered it.
    pub renderer: String,
    pub thinking: bool,
    /// How many functions were declared.
    pub tools: usize,
}

/// The rendered prompt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptId {
    /// `sha256:` of the rendered text.
    pub text_sha256: String,
    /// `sha256:` of the token ids, written one a line in decimal.
    pub tokens_sha256: String,
    /// Their count.
    pub tokens: usize,
    /// How many of them a kept context already held, and were not
    /// evaluated again.
    pub reused: usize,
}

/// The sampler chain, and the seed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplerId {
    pub chain: Vec<Sampler>,
    pub seed: u32,
}

/// The grammar, where it was asked to begin, and where it began.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrammarId {
    /// `sha256:` of its text.
    pub sha256: String,
    pub root: String,
    pub start: GrammarStart,
    /// The generated token at which it began constraining, by its index;
    /// `null` where a lazy grammar never began.
    pub began_at: Option<usize>,
    /// The wall time of the grammar's sampler made, its text parsed, in
    /// milliseconds; absent from a record made before it was kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_ms: Option<f64>,
    /// How a token was held to it; absent, every token.
    #[serde(default, skip_serializing_if = "GrammarCheck::is_every_token")]
    pub check: GrammarCheck,
    /// Under [`GrammarCheck::ChosenFirst`], the tokens whose first choice
    /// the grammar refused, each chosen again; absent under every token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chosen_again: Option<usize>,
}

/// What was generated, and the time it took.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerationId {
    /// Every generated token's id, the thinking's included, the end of turn
    /// not.
    pub tokens: Vec<i32>,
    /// The thinking's tokens, its closing tag included.
    pub thinking: usize,
    /// The answer's tokens.
    pub answer: usize,
    pub stop: Stop,
    /// The wall time of the prompt's evaluation, in milliseconds.
    pub prompt_ms: f64,
    /// The wall time of the generation, in milliseconds.
    pub generation_ms: f64,
}

/// **A call that failed**, each a value. Reaching the budget is not one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "failure", rename_all = "kebab-case")]
pub enum Failure {
    /// The model's file does not load.
    ModelLoad { path: String, message: String },
    /// The template does not render the messages.
    Template { message: String },
    /// The prompt does not fit the context.
    PromptTooLong { prompt: usize, context: u32 },
    /// The grammar's text does not parse.
    GrammarParse { sha256: String, root: String },
    /// **The grammar leaves no token**: a bug of the grammar, never retried.
    GrammarDeadEnd { sha256: String, at: usize },
    /// No record answers the request.
    NotRecorded { digest: String },
    /// llama.cpp refused a step of the call.
    Engine { message: String },
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::ModelLoad { path, message } => {
                write!(f, "the model's file {path} does not load: {message}")
            }
            Failure::Template { message } => write!(f, "the template does not render: {message}"),
            Failure::PromptTooLong { prompt, context } => write!(
                f,
                "the prompt is {prompt} tokens, and the context {context}"
            ),
            Failure::GrammarParse { sha256, root } => {
                write!(
                    f,
                    "the grammar {sha256} does not parse from its root {root}"
                )
            }
            Failure::GrammarDeadEnd { sha256, at } => write!(
                f,
                "the grammar {sha256} leaves no token at the generated token {at}"
            ),
            Failure::NotRecorded { digest } => write!(f, "no record answers the request {digest}"),
            Failure::Engine { message } => write!(f, "llama.cpp: {message}"),
        }
    }
}

/// **The runtime**: a request in, a generation or a failure out.
pub trait Runtime {
    fn generate(&mut self, request: &Request) -> Result<Generation, Failure>;
}

/// **A call as a record keeps it**: the request, its digest, and what came
/// back. One JSON line a call.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recorded {
    /// [`SCHEMA`].
    pub schema: String,
    pub digest: String,
    pub request: Request,
    pub generation: Option<Generation>,
    pub failure: Option<Failure>,
}

impl Recorded {
    pub fn new(request: &Request, result: &Result<Generation, Failure>) -> Self {
        Recorded {
            schema: SCHEMA.to_string(),
            digest: request.digest(),
            request: request.clone(),
            generation: result.as_ref().ok().cloned(),
            failure: result.as_ref().err().cloned(),
        }
    }

    /// What the call returned.
    pub fn result(&self) -> Result<Generation, Failure> {
        match (&self.generation, &self.failure) {
            (Some(g), _) => Ok(g.clone()),
            (None, Some(f)) => Err(f.clone()),
            (None, None) => Err(Failure::NotRecorded {
                digest: self.digest.clone(),
            }),
        }
    }
}

/// SHA-256 of `bytes`, as `sha256:` and the hex digest.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut s = String::from("sha256:");
    for b in Sha256::digest(bytes) {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// The digest of a list of token ids, as the envelope takes it: `sha256:` of
/// the ids, one a line in decimal.
pub fn tokens_digest(tokens: &[i32]) -> String {
    let mut s = String::with_capacity(tokens.len() * 7);
    for t in tokens {
        s.push_str(&t.to_string());
        s.push('\n');
    }
    sha256_hex(s.as_bytes())
}

/// **The thinking and the answer of a generated text**: with thinking on, the
/// text up to the first `</think>` is the thinking and the rest, white space
/// at its start removed, is the answer; a text that never closes its
/// thinking is all thinking. With thinking off, all is the answer.
pub fn split_thinking(text: &str, thinking: bool) -> (String, String) {
    if !thinking {
        return (String::new(), text.to_string());
    }
    match text.find("</think>") {
        Some(i) => (
            text[..i].trim().to_string(),
            text[i + "</think>".len()..].trim_start().to_string(),
        ),
        None => (text.trim().to_string(), String::new()),
    }
}

/// **The first fenced block of an answer**: the lines between the first line
/// that opens a fence (three backticks, any info string after them) and the
/// next line that closes one; `None` where the answer has none. A block
/// never closed runs to the end.
pub fn first_fenced_block(answer: &str) -> Option<String> {
    let mut lines = answer.lines();
    lines.by_ref().find(|l| l.trim_start().starts_with("```"))?;
    let mut block = Vec::new();
    for l in lines {
        if l.trim_start().starts_with("```") {
            break;
        }
        block.push(l);
    }
    let mut s = block.join("\n");
    s.push('\n');
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_thinking_is_cut_at_its_first_closing_tag() {
        assert_eq!(
            split_thinking("a\nb\n</think>\n\n(p a)", true),
            ("a\nb".to_string(), "(p a)".to_string())
        );
        assert_eq!(
            split_thinking("never closed", true),
            ("never closed".to_string(), String::new())
        );
        assert_eq!(
            split_thinking("(p a)", false),
            (String::new(), "(p a)".to_string())
        );
    }

    #[test]
    fn the_first_fenced_block_is_read() {
        let a = "Here:\n```ein\n(p a)\n(q b)\n```\nand\n```\n(r)\n```\n";
        assert_eq!(first_fenced_block(a).as_deref(), Some("(p a)\n(q b)\n"));
        assert_eq!(first_fenced_block("(p a)"), None);
        assert_eq!(
            first_fenced_block("```lisp\n(p a)").as_deref(),
            Some("(p a)\n")
        );
    }
}
