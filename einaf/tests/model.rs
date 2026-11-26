//! **The model's runtime, offline**: what EinAf's gate step holds of it with
//! no llama.cpp and no model. Every call under `fixtures/model/` was made by
//! the binding, `einaf generate --model …`, and is replayed here:
//!
//! - **replay**: a recorded request is answered by the call that recorded it,
//!   field for field, and a request no call recorded is the failure
//!   `not-recorded`, with its digest;
//! - **the envelope**, written and read back, is itself, and its prompt is the
//!   one EinAf renders: the template the record keeps, rendered offline over
//!   the request, has the rendered text's digest;
//! - **each typed failure from a recorded cause**: a model's file that does not
//!   load, a template that does not render, a prompt longer than its context,
//!   a grammar that does not parse and a grammar that leaves no token, each
//!   replayed as itself;
//! - **the probe, replayed**: a line of the probe's record, its call answered
//!   by replay and its answer judged again through the kernel, is judged as it
//!   was.

use std::path::PathBuf;

use einaf::model::replay::Replay;
use einaf::model::{Failure, Prompt, Recorded, Runtime, sha256_hex, template};
use einaf::probe::{self, Line};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/model")
}

fn calls() -> (String, Vec<Recorded>) {
    let text = std::fs::read_to_string(fixtures().join("calls.jsonl")).expect("the calls");
    let calls = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("a recorded call"))
        .collect();
    (text, calls)
}

#[test]
fn a_recorded_call_is_replayed_field_for_field() {
    let (text, calls) = calls();
    let mut replay = Replay::from_jsonl(&text).expect("a record");
    assert_eq!(replay.len(), calls.len(), "a digest twice");
    let generations = calls.iter().filter(|c| c.generation.is_some()).count();
    assert!(generations >= 4, "{generations} generations recorded");
    for c in &calls {
        assert_eq!(c.digest, c.request.digest(), "the digest is the request's");
        assert_eq!(replay.generate(&c.request), c.result());
    }
    let mut other = calls[0].request.clone();
    other.seed += 1;
    assert_eq!(
        replay.generate(&other),
        Err(Failure::NotRecorded {
            digest: other.digest()
        })
    );
}

#[test]
fn the_envelope_is_read_back_as_itself_and_its_prompt_is_einafs_rendering() {
    let (_, calls) = calls();
    let template_text =
        std::fs::read_to_string(fixtures().join("chat_template.jinja")).expect("the template");
    let mut rendered = 0;
    for c in calls.iter().filter(|c| c.generation.is_some()) {
        let json = serde_json::to_string(c).expect("JSON");
        let back: Recorded = serde_json::from_str(&json).expect("read back");
        assert_eq!(&back, c);
        let g = c.generation.as_ref().expect("a generation");
        let e = &g.envelope;
        assert_eq!(e.schema, einaf::model::SCHEMA);
        assert_eq!(
            e.model.sha256,
            "sha256:a9d0a3e1d8c91732a3812becd31f8d4a8ff57414903f226306069f0437e2ac50"
        );
        assert_eq!(e.model.architecture, "qwen35");
        assert_eq!(e.model.quantization, "Q4_K_M");
        assert_eq!(e.runtime.binding, "llama-cpp-2");
        assert_eq!(
            e.generation.tokens.len(),
            e.generation.thinking + e.generation.answer
        );
        assert_eq!(e.sampler.chain, c.request.sampler);
        if let Prompt::Chat { messages, tools } = &c.request.prompt {
            let t = e.template.as_ref().expect("a chat's template");
            assert_eq!(t.sha256, sha256_hex(template_text.as_bytes()));
            let text = template::render(&template_text, messages, tools, c.request.thinking)
                .expect("renders");
            assert_eq!(e.prompt.text_sha256, sha256_hex(text.as_bytes()));
            rendered += 1;
        }
    }
    assert!(rendered >= 3, "{rendered} chats rendered");
}

#[test]
fn each_typed_failure_is_replayed_from_its_recorded_cause() {
    let (text, calls) = calls();
    let mut replay = Replay::from_jsonl(&text).expect("a record");
    let mut seen = Vec::new();
    for c in calls.iter().filter(|c| c.failure.is_some()) {
        let f = c.failure.clone().expect("a failure");
        assert_eq!(replay.generate(&c.request), Err(f.clone()));
        let kind = serde_json::to_value(&f).expect("JSON")["failure"]
            .as_str()
            .expect("a kind")
            .to_string();
        match &f {
            Failure::PromptTooLong { prompt, context } => assert!(prompt >= &(*context as usize)),
            Failure::Template { message } => assert!(message.contains("No messages provided")),
            Failure::GrammarParse { .. } | Failure::GrammarDeadEnd { .. } => {
                assert!(c.request.grammar.is_some())
            }
            Failure::ModelLoad { path, .. } => assert!(!path.is_empty()),
            other => panic!("an unexpected failure recorded: {other}"),
        }
        assert!(!f.to_string().is_empty());
        seen.push(kind);
    }
    seen.sort();
    assert_eq!(
        seen,
        [
            "grammar-dead-end",
            "grammar-parse",
            "model-load",
            "prompt-too-long",
            "template"
        ]
    );
}

#[test]
fn a_probe_line_replayed_is_judged_as_it_was() {
    let text = std::fs::read_to_string(fixtures().join("probe_lines.jsonl")).expect("the lines");
    let mut replay = Replay::from_jsonl(&text).expect("a record");
    let lines: Vec<Line> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("a line"))
        .collect();
    assert!(!lines.is_empty());
    let untimed = |mut l: Line| {
        for j in &mut l.judged {
            for f in &mut j.feedback {
                f.work.run_us = None;
                f.work.load_us = 0;
                f.work.ask_us = 0;
            }
        }
        l
    };
    for line in lines {
        let again = probe::run_one(&mut replay, &line.planned);
        assert_eq!(untimed(again), untimed(line));
    }
}
