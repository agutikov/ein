//! The model's subcommands of `einaf`: `render`, `generate`, `model` and
//! `probe`. A call is answered by the model, through the binding (the
//! `llama` feature), or by a record (`--replay`), which needs no model.

use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use einaf::model::replay::Replay;
use einaf::model::{Prompt, Recorded, Request, Runtime, template};
use einaf::probe::{self, Line, Planned};

use crate::usage_error;

/// What answers a call, as the options name it.
struct RuntimeArgs {
    model: Option<PathBuf>,
    ctx: u32,
    verbose: bool,
    replay: Option<PathBuf>,
}

impl RuntimeArgs {
    fn new() -> Self {
        RuntimeArgs {
            model: None,
            ctx: 65_536,
            verbose: false,
            replay: None,
        }
    }

    /// Take a runtime option at `a`; `false` where `a` is none.
    fn take(&mut self, a: &str, it: &mut std::slice::Iter<'_, String>) -> Result<bool, String> {
        match a {
            "--model" => self.model = Some(it.next().ok_or("--model takes a path")?.into()),
            "--replay" => self.replay = Some(it.next().ok_or("--replay takes a path")?.into()),
            "--ctx" => {
                let v = it.next().ok_or("--ctx takes a number")?;
                self.ctx = v
                    .parse()
                    .map_err(|_| format!("--ctx {v}: not a number of tokens"))?;
            }
            "--verbose" => self.verbose = true,
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn open(&self) -> Result<Box<dyn Runtime>, Opened> {
        if let Some(path) = &self.replay {
            let replay = Replay::from_file(path).map_err(Opened::Usage)?;
            eprintln!("einaf: replaying {} recorded calls", replay.len());
            return Ok(Box::new(replay));
        }
        let model = self
            .model
            .clone()
            .or_else(|| std::env::var_os("EINAF_MODEL").map(PathBuf::from))
            .ok_or_else(|| {
                Opened::Usage("no runtime: --model PATH, $EINAF_MODEL or --replay FILE".into())
            })?;
        self.load(model)
    }

    #[cfg(feature = "llama")]
    fn load(&self, model: PathBuf) -> Result<Box<dyn Runtime>, Opened> {
        use einaf::model::llama::{Llama, Options};
        let mut options = Options::new(model, self.ctx);
        options.verbose = self.verbose;
        let llama = Llama::load(&options).map_err(Opened::Failed)?;
        eprintln!(
            "einaf: {} {} ({}), {} layers on {} ({}), a context of {} tokens",
            llama.pin().architecture,
            llama.pin().size_label,
            llama.pin().sha256,
            llama.runtime().gpu_layers,
            llama.runtime().backend,
            llama.runtime().device,
            llama.context().n_ctx
        );
        Ok(Box::new(llama))
    }

    #[cfg(not(feature = "llama"))]
    fn load(&self, model: PathBuf) -> Result<Box<dyn Runtime>, Opened> {
        Err(Opened::Usage(format!(
            "{}: this einaf is built without the model's runtime; build it with \
             `--features cuda` (or `llama`), or answer from a record with --replay",
            model.display()
        )))
    }
}

/// Why no runtime was opened: the model's file did not load, a failure of
/// every call, which `generate` records; or the options name none.
enum Opened {
    #[cfg_attr(not(feature = "llama"), allow(dead_code))]
    Failed(einaf::model::Failure),
    Usage(String),
}

impl std::fmt::Display for Opened {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opened::Failed(failure) => write!(f, "{failure}"),
            Opened::Usage(s) => f.write_str(s),
        }
    }
}

fn read_request(path: &str) -> Result<Request, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))
}

/// `einaf render --template FILE REQUEST.json`.
pub fn render(args: &[String]) -> ExitCode {
    let mut template_path = None;
    let mut request = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--template" => template_path = it.next().cloned(),
            s if s.starts_with('-') => return usage_error(&format!("unknown option {s}")),
            s => request = Some(s.to_string()),
        }
    }
    let (Some(t), Some(r)) = (template_path, request) else {
        return usage_error("render takes --template FILE and a REQUEST.json");
    };
    let result = std::fs::read_to_string(&t)
        .map_err(|e| format!("{t}: {e}"))
        .and_then(|text| {
            let req = read_request(&r)?;
            match &req.prompt {
                Prompt::Chat { messages, tools } => {
                    template::render(&text, messages, tools, req.thinking)
                        .map_err(|e| e.to_string())
                }
                Prompt::Text { text } => Ok(text.clone()),
            }
        });
    match result {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

/// `einaf generate RUNTIME REQUEST.json…`: each request in order, by one
/// runtime, so that a request may continue the context the one before it
/// kept; a record line each.
pub fn generate(args: &[String]) -> ExitCode {
    let mut rt = RuntimeArgs::new();
    let mut requests = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match rt.take(a, &mut it) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(e) => return usage_error(&e),
        }
        match a.as_str() {
            s if s.starts_with('-') => return usage_error(&format!("unknown option {s}")),
            s => requests.push(s.to_string()),
        }
    }
    if requests.is_empty() {
        return usage_error("generate takes a REQUEST.json");
    }
    let mut reqs = Vec::new();
    for r in &requests {
        match read_request(r) {
            Ok(req) => reqs.push(req),
            Err(e) => return usage_error(&e),
        }
    }
    let mut runtime = match rt.open() {
        Ok(r) => r,
        Err(Opened::Failed(failure)) => {
            // The model did not load: every call fails so, and is recorded.
            for (path, req) in requests.iter().zip(&reqs) {
                let recorded = Recorded::new(req, &Err(failure.clone()));
                println!(
                    "{}",
                    serde_json::to_string(&recorded).expect("a record is JSON")
                );
                eprintln!("einaf: {path}: the call failed: {failure}");
            }
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    for (path, req) in requests.iter().zip(&reqs) {
        let result = runtime.generate(req);
        let recorded = Recorded::new(req, &result);
        println!(
            "{}",
            serde_json::to_string(&recorded).expect("a record is JSON")
        );
        if let Err(f) = &result {
            eprintln!("einaf: {path}: the call failed: {f}");
        }
    }
    ExitCode::SUCCESS
}

/// `einaf grammar --json-schema FILE`: the GBNF grammar llama.cpp's own
/// conversion writes for a JSON schema, by the binding's llama.cpp.
#[cfg(feature = "llama")]
pub fn grammar(args: &[String]) -> ExitCode {
    let path = match args {
        [flag, path] if flag == "--json-schema" => path,
        _ => return usage_error("grammar takes --json-schema FILE"),
    };
    let schema = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {path}: {e}");
            return ExitCode::from(1);
        }
    };
    match einaf::model::llama::json_schema_to_grammar(&schema) {
        Ok(g) => {
            print!("{g}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

/// `einaf grammar llama --model PATH [--grammar FILE] [--root NAME]
/// [--list FILE] [--special] FILE...`: **each file read by llama.cpp's own
/// grammar sampler**, token by token with the model's vocabulary, and by
/// EinAf's acceptor; a JSON line a file, with whether the two agree, and a
/// count on stderr. With `--special` a file is tokenised with the model's
/// special tokens parsed, so that `</think>` is the one token the model
/// generates, and its line adds the token ids and the special tokens among
/// them. Exits 0 where every file agrees.
#[cfg(feature = "llama")]
pub fn grammar_llama(args: &[String]) -> ExitCode {
    use einaf::grammar::gbnf::{Compiled, Verdict};
    use einaf::model::llama::Vocab;
    let (mut model, mut grammar, mut root) = (None, None, "program".to_string());
    let mut special = false;
    let mut files: Vec<String> = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--model" => model = it.next().map(PathBuf::from),
            "--special" => special = true,
            "--grammar" => grammar = it.next().cloned(),
            "--root" => match it.next() {
                Some(r) => root = r.clone(),
                None => return usage_error("--root takes a rule's name"),
            },
            "--list" => match it.next().map(std::fs::read_to_string) {
                Some(Ok(t)) => {
                    files.extend(t.lines().filter(|l| !l.is_empty()).map(str::to_string))
                }
                _ => return usage_error("--list takes a file of paths"),
            },
            s if s.starts_with('-') => return usage_error(&format!("unknown option {s}")),
            s => files.push(s.to_string()),
        }
    }
    let Some(model) = model.or_else(|| std::env::var_os("EINAF_MODEL").map(PathBuf::from)) else {
        return usage_error("llama takes --model PATH, or $EINAF_MODEL");
    };
    let text = match &grammar {
        Some(p) => match std::fs::read_to_string(p) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("error: {p}: {e}");
                return ExitCode::from(1);
            }
        },
        None => einaf::grammar::whole_text(),
    };
    let compiled = match Compiled::parse(&text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: the grammar: {e}");
            return ExitCode::from(1);
        }
    };
    let vocab = match Vocab::load(&model) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    let (mut agree, mut differ) = (0, 0);
    for f in &files {
        let input = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{f}: unreadable: {e}");
                continue;
            }
        };
        let ours = compiled.accepts(&root, &input);
        let tokens = if special {
            vocab.special_tokens(&input)
        } else {
            vocab.tokens(&input)
        };
        let theirs = vocab.read_tokens(&text, &root, &input, &tokens);
        let same = match (&ours, &theirs) {
            (Ok(Verdict::Accepted), Ok(r)) => r.refused.is_none() && r.can_end,
            (Ok(Verdict::Refused { at, .. }), Ok(r)) => match r.refused {
                Some((_, off)) => off <= *at && *at < off + r.refused_len.max(1),
                None => *at == input.len() && !r.can_end,
            },
            _ => false,
        };
        if same {
            agree += 1;
        } else {
            differ += 1;
        }
        let einaf = match &ours {
            Ok(Verdict::Accepted) => serde_json::json!({"accepted": true}),
            Ok(Verdict::Refused { at, line, column }) => {
                serde_json::json!({"accepted": false, "at": at, "line": line, "column": column})
            }
            Err(e) => serde_json::json!({"error": e}),
        };
        let llama = match &theirs {
            Ok(r) => serde_json::to_value(r).expect("JSON"),
            Err(e) => serde_json::json!({"error": e}),
        };
        let mut line =
            serde_json::json!({"file": f, "agree": same, "einaf": einaf, "llama": llama});
        if special {
            // The special tokens the text holds, by index, id and piece.
            let held: Vec<serde_json::Value> = tokens
                .iter()
                .enumerate()
                .filter_map(|(i, &id)| {
                    let (piece, special) = vocab.piece(id);
                    special.then(|| serde_json::json!({"index": i, "id": id, "piece": piece}))
                })
                .collect();
            line["ids"] = serde_json::json!(tokens);
            line["special"] = serde_json::json!(held);
        }
        println!("{line}");
    }
    eprintln!(
        "einaf grammar llama: {} files, {agree} read alike, {differ} not",
        files.len()
    );
    if differ == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// `einaf grammar llama`: built with the `llama` feature only.
#[cfg(not(feature = "llama"))]
pub fn grammar_llama(_: &[String]) -> ExitCode {
    eprintln!(
        "error: this einaf is built without the model's runtime; build it with `--features cuda`"
    );
    ExitCode::from(1)
}

#[cfg(not(feature = "llama"))]
pub fn grammar(_: &[String]) -> ExitCode {
    eprintln!(
        "error: this einaf is built without the model's runtime; build it with `--features cuda`"
    );
    ExitCode::from(1)
}

/// `einaf model --model PATH`: the model loaded, and what it is.
pub fn model(args: &[String]) -> ExitCode {
    let mut rt = RuntimeArgs::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match rt.take(a, &mut it) {
            Ok(true) => {}
            Ok(false) => return usage_error(&format!("unknown option {a}")),
            Err(e) => return usage_error(&e),
        }
    }
    model_info(&rt)
}

#[cfg(feature = "llama")]
fn model_info(rt: &RuntimeArgs) -> ExitCode {
    use einaf::model::llama::{Llama, Options};
    let Some(model) = rt
        .model
        .clone()
        .or_else(|| std::env::var_os("EINAF_MODEL").map(PathBuf::from))
    else {
        return usage_error("model takes --model PATH, or $EINAF_MODEL");
    };
    let mut options = Options::new(model, rt.ctx);
    options.verbose = rt.verbose;
    match Llama::load(&options) {
        Ok(l) => {
            let info = serde_json::json!({
                "model": l.pin(),
                "runtime": l.runtime(),
                "context": l.context(),
                "template_sha256": einaf::model::sha256_hex(l.template().as_bytes()),
            });
            println!("{}", serde_json::to_string_pretty(&info).expect("JSON"));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

#[cfg(not(feature = "llama"))]
fn model_info(_: &RuntimeArgs) -> ExitCode {
    eprintln!(
        "error: this einaf is built without the model's runtime; build it with `--features cuda`"
    );
    ExitCode::from(1)
}

/// `einaf probe plan|run`.
pub fn probe(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("plan") => probe_plan(&args[1..]),
        Some("run") => probe_run(&args[1..]),
        _ => usage_error("probe takes `plan` or `run`"),
    }
}

fn probe_plan(args: &[String]) -> ExitCode {
    let mut out: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" => out = it.next().cloned(),
            s => return usage_error(&format!("unknown argument {s}")),
        }
    }
    let mut text = String::new();
    for p in probe::plan() {
        text.push_str(&serde_json::to_string(&p).expect("a plan is JSON"));
        text.push('\n');
    }
    match out {
        Some(path) => {
            if let Err(e) = std::fs::write(&path, text) {
                eprintln!("error: {path}: {e}");
                return ExitCode::from(1);
            }
        }
        None => print!("{text}"),
    }
    ExitCode::SUCCESS
}

fn probe_run(args: &[String]) -> ExitCode {
    let mut rt = RuntimeArgs::new();
    let (mut plan, mut out) = (None, None);
    let mut only: Vec<String> = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match rt.take(a, &mut it) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(e) => return usage_error(&e),
        }
        match a.as_str() {
            "--plan" => plan = it.next().cloned(),
            "--out" => out = it.next().cloned(),
            "--only" => match it.next() {
                Some(p) => only.push(p.clone()),
                None => return usage_error("--only takes a prefix of a run's id"),
            },
            s => return usage_error(&format!("unknown argument {s}")),
        }
    }
    let (Some(plan), Some(out)) = (plan, out) else {
        return usage_error("probe run takes --plan FILE and --out FILE");
    };
    let planned: Vec<Planned> = match std::fs::read_to_string(&plan) {
        Ok(text) => {
            let parsed: Result<Vec<Planned>, _> = text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(serde_json::from_str)
                .collect();
            match parsed {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("error: {plan}: {e}");
                    return ExitCode::from(1);
                }
            }
        }
        Err(e) => {
            eprintln!("error: {plan}: {e}");
            return ExitCode::from(1);
        }
    };
    let done: HashSet<String> = std::fs::read_to_string(&out)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<Line>(l).ok())
        .map(|l| l.planned.id)
        .collect();
    let todo: Vec<&Planned> = planned
        .iter()
        .filter(|p| only.is_empty() || only.iter().any(|o| p.id.starts_with(o.as_str())))
        .filter(|p| !done.contains(&p.id))
        .collect();
    if todo.is_empty() {
        eprintln!("einaf: nothing to run");
        return ExitCode::SUCCESS;
    }
    let mut runtime = match rt.open() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {out}: {e}");
            return ExitCode::from(1);
        }
    };
    for (i, p) in todo.iter().enumerate() {
        let line = probe::run_one(runtime.as_mut(), p);
        let summary = summarise(&line);
        eprintln!("[{}/{}] {} {summary}", i + 1, todo.len(), p.id);
        let json = serde_json::to_string(&line).expect("a line is JSON");
        if let Err(e) = writeln!(file, "{json}").and_then(|_| file.flush()) {
            eprintln!("error: {out}: {e}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}

/// One line of the run's progress: the generation's size and stop, and each
/// reading's parse, load and outcome at root.
fn summarise(line: &Line) -> String {
    let r = &line.recorded;
    let mut s = match (&r.generation, &r.failure) {
        (Some(g), _) => format!(
            "{} tokens ({} thinking), {:?}, {:.1} s:",
            g.envelope.generation.tokens.len(),
            g.envelope.generation.thinking,
            g.envelope.generation.stop,
            g.envelope.generation.generation_ms / 1e3
        ),
        (None, Some(f)) => format!("failed: {f}"),
        (None, None) => "nothing".to_string(),
    };
    for j in &line.judged {
        let word = match (&j.text, &j.parsed, j.loaded) {
            (None, _, _) => "none".to_string(),
            (Some(_), Some(p), None) if p.forms.is_none() => {
                format!("parse error ({})", j.kinds.join(","))
            }
            (Some(_), _, Some(false)) => format!("load error ({})", j.kinds.join(",")),
            (Some(_), _, Some(true)) => {
                let single = j
                    .feedback
                    .get(1)
                    .and_then(|f| f.outcome.as_ref())
                    .and_then(|o| o.first())
                    .map(|o| o.outcome.word())
                    .unwrap_or("?");
                format!("loads, --single {single}")
            }
            _ => "?".to_string(),
        };
        s.push_str(&format!(" {}={word}", j.reading));
    }
    s
}
