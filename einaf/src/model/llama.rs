//! **The binding**: the model in EinAf's process, through `llama-cpp-2`, a
//! Rust binding to llama.cpp that builds a llama.cpp of its own. Behind the
//! `llama` feature (and `cuda` for the GPU); nothing else in EinAf names the
//! binding.
//!
//! One model and one context a process. A call renders its prompt through
//! the model's own template ([`super::template`]), tokenises it with the
//! special tokens parsed, evaluates what the context does not already hold,
//! and samples token by token: the model's logits, their log-probabilities
//! where asked, the grammar if any, then the request's chain. The grammar is
//! applied by itself before the chain, so a grammar that leaves no token is
//! seen as such ([`Failure::GrammarDeadEnd`]) and where a lazy grammar began
//! is read off the first token it constrained.

use std::fs::File;
use std::io::Read;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::time::Instant;

use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use sha2::{Digest, Sha256};

use super::{
    ContextId, Failure, Generation, GenerationId, GrammarId, GrammarStart, ModelPin, Prompt,
    PromptId, Request, Runtime, RuntimeId, Sampler, SamplerId, Stop, TemplateId, TokenLogprob,
    template,
};
use super::{Envelope, SCHEMA, sha256_hex, split_thinking, tokens_digest};

/// The binding's crate and version, as the manifest pins it.
pub const BINDING: &str = "llama-cpp-2";
pub const BINDING_VERSION: &str = "0.1.161";

/// **The llama.cpp the binding builds**: `llama-cpp-sys-2` 0.1.161 vendors
/// llama.cpp as a submodule, at this commit (the crate's own commit,
/// `e7af607e`, names it). The crate carries no git metadata, so its build
/// cannot say it; the pin of the manifest does.
pub const LLAMA_CPP_COMMIT: &str = "d81235049384534c167caea52b85a694f6103d14";

/// The GBNF grammar llama.cpp's own conversion writes for a JSON schema.
pub fn json_schema_to_grammar(schema: &str) -> Result<String, String> {
    llama_cpp_2::json_schema_to_grammar(schema).map_err(|e| e.to_string())
}

/// How the binding is set up.
#[derive(Clone, Debug)]
pub struct Options {
    /// The model's file; never its identity, which is its SHA-256.
    pub model: PathBuf,
    /// The context's length in tokens.
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_ubatch: u32,
    /// Layers put on the device; more than the model has puts all of them.
    pub gpu_layers: u32,
    /// llama.cpp's own log on stderr.
    pub verbose: bool,
}

impl Options {
    pub fn new(model: impl Into<PathBuf>, n_ctx: u32) -> Self {
        Options {
            model: model.into(),
            n_ctx,
            n_batch: 2048,
            n_ubatch: 512,
            gpu_layers: 999,
            verbose: false,
        }
    }
}

/// **The runtime through the binding**: a model loaded, and one context.
pub struct Llama {
    model: &'static LlamaModel,
    ctx: LlamaContext<'static>,
    template: String,
    template_sha: String,
    pin: ModelPin,
    runtime: RuntimeId,
    context: ContextId,
    /// The kept context's name and the tokens it holds.
    kept: Option<(u32, Vec<i32>)>,
}

/// The device's free and total memory, in MiB, of the first device that is
/// not the CPU.
fn device_memory() -> Option<(u64, u64, String, String)> {
    llama_cpp_2::list_llama_ggml_backend_devices()
        .into_iter()
        .find(|d| d.backend != "CPU")
        .map(|d| {
            (
                (d.memory_free / (1024 * 1024)) as u64,
                (d.memory_total / (1024 * 1024)) as u64,
                d.name,
                d.description,
            )
        })
}

/// SHA-256 of a file, streamed.
fn file_sha256(path: &Path) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    let mut s = String::from("sha256:");
    for b in h.finalize() {
        s.push_str(&format!("{b:02x}"));
    }
    Ok(s)
}

/// The quantization a GGUF's `general.file_type` names.
fn quantization(file_type: &str) -> String {
    let name = match file_type {
        "0" => "F32",
        "1" => "F16",
        "2" => "Q4_0",
        "3" => "Q4_1",
        "7" => "Q8_0",
        "8" => "Q5_0",
        "9" => "Q5_1",
        "10" => "Q2_K",
        "11" => "Q3_K_S",
        "12" => "Q3_K_M",
        "13" => "Q3_K_L",
        "14" => "Q4_K_S",
        "15" => "Q4_K_M",
        "16" => "Q5_K_S",
        "17" => "Q5_K_M",
        "18" => "Q6_K",
        "32" => "BF16",
        other => return format!("file_type {other}"),
    };
    name.to_string()
}

impl Llama {
    /// **Load the model and make the context.** The file is hashed first, and
    /// the device's free memory read before the load and after the context.
    pub fn load(options: &Options) -> Result<Self, Failure> {
        let load_failure = |message: String| Failure::ModelLoad {
            path: options.model.display().to_string(),
            message,
        };
        let sha256 = file_sha256(&options.model).map_err(|e| load_failure(e.to_string()))?;
        let mut backend = LlamaBackend::init().map_err(|e| load_failure(e.to_string()))?;
        if !options.verbose {
            backend.void_logs();
        }
        // The backend lives as long as the process: a model and its context
        // are made once, and leaked, so that the context may borrow the
        // model for the life of the runtime.
        let backend: &'static LlamaBackend = Box::leak(Box::new(backend));
        let before = device_memory();
        let params = LlamaModelParams::default().with_n_gpu_layers(options.gpu_layers);
        let model = LlamaModel::load_from_file(backend, &options.model, &params)
            .map_err(|e| load_failure(e.to_string()))?;
        let model: &'static LlamaModel = Box::leak(Box::new(model));
        let template = model
            .chat_template(None)
            .map_err(|e| load_failure(format!("no chat template: {e}")))?
            .to_string()
            .map_err(|e| load_failure(format!("the chat template is no UTF-8: {e}")))?;
        let cparams = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(options.n_ctx))
            .with_n_batch(options.n_batch)
            .with_n_ubatch(options.n_ubatch)
            .with_n_seq_max(1);
        let ctx = model
            .new_context(backend, cparams)
            .map_err(|e| Failure::Engine {
                message: format!("the context of {} tokens: {e}", options.n_ctx),
            })?;
        let after = device_memory();
        let meta = |k: &str| model.meta_val_str(k).unwrap_or_default();
        let layers = model.n_layer();
        let pin = ModelPin {
            sha256,
            architecture: meta("general.architecture"),
            size_label: meta("general.size_label"),
            parameters: model.n_params(),
            quantization: quantization(&meta("general.file_type")),
            trained_context: model.n_ctx_train(),
            layers,
            vocabulary: model.n_vocab() as u32,
        };
        let (backend_name, device) = match &after {
            Some((_, _, name, desc)) if options.gpu_layers > 0 => (name.clone(), desc.clone()),
            _ => ("CPU".to_string(), "CPU".to_string()),
        };
        let runtime = RuntimeId {
            binding: BINDING.to_string(),
            binding_version: BINDING_VERSION.to_string(),
            llama_cpp: LLAMA_CPP_COMMIT.to_string(),
            backend: backend_name.clone(),
            device,
            gpu_layers: if backend_name == "CPU" {
                0
            } else {
                options.gpu_layers.min(layers)
            },
        };
        let context = ContextId {
            n_ctx: ctx.n_ctx(),
            n_batch: ctx.n_batch(),
            n_ubatch: ctx.n_ubatch(),
            flash_attention: "auto".to_string(),
            device_free_mib_at_load: before.as_ref().map(|d| d.0),
            device_free_mib_loaded: after.as_ref().map(|d| d.0),
            device_total_mib: after.as_ref().map(|d| d.1),
        };
        let template_sha = sha256_hex(template.as_bytes());
        Ok(Llama {
            model,
            ctx,
            template,
            template_sha,
            pin,
            runtime,
            context,
            kept: None,
        })
    }

    /// The model's pin.
    pub fn pin(&self) -> &ModelPin {
        &self.pin
    }

    pub fn runtime(&self) -> &RuntimeId {
        &self.runtime
    }

    pub fn context(&self) -> &ContextId {
        &self.context
    }

    /// The chat template's text, as the file carries it.
    pub fn template(&self) -> &str {
        &self.template
    }

    /// The prompt's text and its tokens, as a call renders and tokenises them.
    pub fn render(&self, request: &Request) -> Result<(String, Vec<i32>), Failure> {
        let text = match &request.prompt {
            Prompt::Chat { messages, tools } => {
                template::render(&self.template, messages, tools, request.thinking)?
            }
            Prompt::Text { text } => text.clone(),
        };
        let tokens = self
            .model
            .vocab()
            .tokenize(text.as_bytes(), false, true)
            .into_iter()
            .map(|t| t.0)
            .collect();
        Ok((text, tokens))
    }

    fn sampler(&self, s: &Sampler, seed: u32) -> LlamaSampler {
        match *s {
            Sampler::Greedy => LlamaSampler::greedy(),
            Sampler::Penalties {
                last_n,
                repeat,
                frequency,
                presence,
            } => LlamaSampler::penalties(self.model.n_vocab(), last_n, repeat, frequency, presence),
            Sampler::TopK { k } => LlamaSampler::top_k(k),
            Sampler::TopP { p, min_keep } => LlamaSampler::top_p(p, min_keep),
            Sampler::MinP { p, min_keep } => LlamaSampler::min_p(p, min_keep),
            Sampler::Temp { t } => LlamaSampler::temp(t),
            Sampler::Dist => LlamaSampler::dist(seed),
        }
    }

    fn decode(&mut self, tokens: &[i32], from: usize, last_logits: bool) -> Result<(), Failure> {
        let engine = |m: String| Failure::Engine { message: m };
        let n_batch = self.context.n_batch as usize;
        let mut batch = LlamaBatch::new(n_batch, 1);
        let total = tokens.len();
        let mut pos = from;
        while pos < total {
            batch.clear();
            let end = (pos + n_batch).min(total);
            for (p, &t) in tokens.iter().enumerate().take(end).skip(pos) {
                let logits = last_logits && p == total - 1;
                batch
                    .add(LlamaToken(t), p as i32, &[0], logits)
                    .map_err(|e| engine(e.to_string()))?;
            }
            self.ctx
                .decode(&mut batch)
                .map_err(|e| engine(format!("decode at {pos}: {e}")))?;
            pos = end;
        }
        Ok(())
    }

    /// Evaluate one generated token at `pos`, its logits kept.
    fn decode_one(&mut self, token: i32, pos: usize) -> Result<(), Failure> {
        let mut batch = LlamaBatch::new(1, 1);
        batch
            .add(LlamaToken(token), pos as i32, &[0], true)
            .map_err(|e| Failure::Engine {
                message: e.to_string(),
            })?;
        self.ctx.decode(&mut batch).map_err(|e| Failure::Engine {
            message: format!("decode at {pos}: {e}"),
        })
    }
}

/// The log-probability of each candidate, from the raw logits, and the top
/// `n` of them.
fn logprobs_of(logits: &[f32], chosen: i32, n: u32) -> TokenLogprob {
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let sum: f64 = logits.iter().map(|&l| ((l - max) as f64).exp()).sum();
    let lse = max as f64 + sum.ln();
    let lp = |i: usize| (logits[i] as f64 - lse) as f32;
    let mut idx: Vec<usize> = (0..logits.len()).collect();
    let n = (n as usize).min(idx.len());
    if n > 0 {
        idx.select_nth_unstable_by(n - 1, |&a, &b| logits[b].total_cmp(&logits[a]));
        idx.truncate(n);
        idx.sort_by(|&a, &b| logits[b].total_cmp(&logits[a]));
    } else {
        idx.clear();
    }
    TokenLogprob {
        token: chosen,
        logprob: lp(chosen as usize),
        top: idx.into_iter().map(|i| (i as i32, lp(i))).collect(),
    }
}

impl Runtime for Llama {
    fn generate(&mut self, req: &Request) -> Result<Generation, Failure> {
        let (text, tokens) = self.render(req)?;
        let n_ctx = self.context.n_ctx;
        if tokens.len() >= n_ctx as usize {
            return Err(Failure::PromptTooLong {
                prompt: tokens.len(),
                context: n_ctx,
            });
        }
        // **The kept context**: where the request continues the context kept
        // under its name and the prompt begins with every token it holds,
        // only the rest is evaluated. The model's recurrent layers cannot be
        // cut back, so any other prompt is evaluated whole.
        let reused = match (req.keep, &self.kept) {
            (Some(k), Some((name, held)))
                if k == *name && held.len() < tokens.len() && tokens.starts_with(held) =>
            {
                held.len()
            }
            _ => 0,
        };
        if reused == 0 {
            self.ctx.clear_kv_cache();
        }
        self.kept = None;
        let t0 = Instant::now();
        self.decode(&tokens, reused, true)?;
        let prompt_ms = t0.elapsed().as_secs_f64() * 1e3;

        let grammar_sha = req.grammar.as_ref().map(|g| sha256_hex(g.text.as_bytes()));
        let mut grammar = match &req.grammar {
            None => None,
            Some(g) => {
                let made = match &g.start {
                    GrammarStart::First => LlamaSampler::grammar(self.model, &g.text, &g.root),
                    GrammarStart::Patterns { patterns } => LlamaSampler::grammar_lazy_patterns(
                        self.model,
                        &g.text,
                        &g.root,
                        patterns,
                        &[],
                    ),
                };
                Some(made.map_err(|_| Failure::GrammarParse {
                    sha256: grammar_sha.clone().unwrap_or_default(),
                    root: g.root.clone(),
                })?)
            }
        };
        let mut chain =
            LlamaSampler::chain_simple(req.sampler.iter().map(|s| self.sampler(s, req.seed)));

        let t1 = Instant::now();
        let mut out: Vec<i32> = Vec::new();
        let mut logprobs = Vec::new();
        let mut began_at = None;
        let mut pos = tokens.len();
        let mut logits_at: i32 =
            ((tokens.len() - reused - 1) % self.context.n_batch as usize) as i32;
        let stop = loop {
            if out.len() >= req.max_tokens as usize {
                break Stop::MaxTokens;
            }
            if pos >= n_ctx as usize {
                break Stop::ContextFull;
            }
            let mut cands = self.ctx.token_data_array_ith(logits_at);
            let raw: Option<Vec<f32>> = req
                .logprobs
                .map(|_| cands.data.iter().map(|d| d.logit()).collect());
            if let Some(g) = grammar.as_mut() {
                let open_before = cands.data.iter().filter(|d| d.logit().is_finite()).count();
                cands.apply_sampler(g);
                let open = cands.data.iter().filter(|d| d.logit().is_finite()).count();
                if open == 0 {
                    return Err(Failure::GrammarDeadEnd {
                        sha256: grammar_sha.clone().unwrap_or_default(),
                        at: out.len(),
                    });
                }
                if began_at.is_none() && open < open_before {
                    began_at = Some(out.len());
                }
            }
            cands.apply_sampler(&mut chain);
            let token = cands.selected_token().ok_or_else(|| Failure::Engine {
                message: format!("the chain selected no token at {}", out.len()),
            })?;
            if let Some(g) = grammar.as_mut() {
                g.try_accept(token).map_err(|e| Failure::Engine {
                    message: format!("the grammar refused its own token {}: {e}", token.0),
                })?;
            }
            chain.accept(token);
            if let (Some(n), Some(raw)) = (req.logprobs, raw) {
                logprobs.push(logprobs_of(&raw, token.0, n));
            }
            if self.model.vocab().is_eog(token) {
                break Stop::EndOfTurn;
            }
            out.push(token.0);
            self.decode_one(token.0, pos)?;
            pos += 1;
            logits_at = 0;
        };
        let generation_ms = t1.elapsed().as_secs_f64() * 1e3;
        if let Some(k) = req.keep {
            let mut held = tokens.clone();
            held.extend_from_slice(&out);
            self.kept = Some((k, held));
        }

        let vocab = self.model.vocab();
        let mut bytes = Vec::new();
        let mut think_end = None;
        for (i, &t) in out.iter().enumerate() {
            let piece = vocab.token_to_piece(LlamaToken(t), true, None);
            bytes.extend_from_slice(&piece);
            // The closing tag ends in this token where the text's tail,
            // this token and a tag's length before it, holds it.
            let tail = &bytes[bytes.len().saturating_sub(piece.len() + 8)..];
            if think_end.is_none() && tail.windows(8).any(|w| w == b"</think>") {
                think_end = Some(i + 1);
            }
        }
        let generated = String::from_utf8_lossy(&bytes).into_owned();
        let (thinking, answer) = split_thinking(&generated, req.thinking);
        let n_thinking = if req.thinking {
            think_end.unwrap_or(out.len())
        } else {
            0
        };
        let template = match &req.prompt {
            Prompt::Chat { tools, .. } => Some(TemplateId {
                sha256: self.template_sha.clone(),
                renderer: template::RENDERER.to_string(),
                thinking: req.thinking,
                tools: tools.len(),
            }),
            Prompt::Text { .. } => None,
        };
        let envelope = Envelope {
            schema: SCHEMA.to_string(),
            model: self.pin.clone(),
            runtime: self.runtime.clone(),
            context: self.context.clone(),
            template,
            prompt: PromptId {
                text_sha256: sha256_hex(text.as_bytes()),
                tokens_sha256: tokens_digest(&tokens),
                tokens: tokens.len(),
                reused,
            },
            sampler: SamplerId {
                chain: req.sampler.clone(),
                seed: req.seed,
            },
            grammar: req.grammar.as_ref().map(|g| GrammarId {
                sha256: grammar_sha.clone().unwrap_or_default(),
                root: g.root.clone(),
                start: g.start.clone(),
                began_at,
            }),
            generation: GenerationId {
                thinking: n_thinking,
                answer: out.len() - n_thinking,
                tokens: out,
                stop,
                prompt_ms,
                generation_ms,
            },
        };
        Ok(Generation {
            thinking,
            answer,
            logprobs: req.logprobs.map(|_| logprobs),
            envelope,
        })
    }
}
