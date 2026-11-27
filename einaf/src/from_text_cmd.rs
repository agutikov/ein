//! `einaf from-text`: a text to a program and the short answer that
//! program prints, one shot ([`einaf::from_text`]); the answer on stdout,
//! each call and each step's judgement, with its times, on stderr.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use einaf::answer;
use einaf::catalogue::Size;
use einaf::formalize::{Cut, Input, Mode, RequestMode};
use einaf::from_text::{self, Config, FromText};
use einaf::model::Recorded;
use einaf::prompts;

use crate::analysis_cmd::{self, AnalysisArgs};
use crate::model_cmd::{Opened, RuntimeArgs};
use crate::usage_error;

/// `einaf from-text TEXT RUNTIME [options]`.
pub fn from_text(args: &[String]) -> ExitCode {
    let mut rt = RuntimeArgs::new();
    let mut analysis = AnalysisArgs::new();
    let mut config = Config::default();
    let (mut text, mut id) = (None::<PathBuf>, None::<String>);
    let (mut out, mut explain, mut record) = (None::<PathBuf>, None::<PathBuf>, None::<PathBuf>);
    let mut gold = None::<String>;
    let mut line = "generic".to_string();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match rt.take(a, &mut it) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(e) => return usage_error(&e),
        }
        match analysis.take(a, &mut it) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(e) => return usage_error(&e),
        }
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} takes a value"));
        let o = &mut config.options;
        let r: Result<(), String> = (|| {
            match a.as_str() {
                "--id" => id = Some(value("--id")?),
                "--answer-line" => line = value("--answer-line")?,
                "--mode" => {
                    let v = value("--mode")?;
                    o.mode = Mode::parse(&v).ok_or(format!("--mode {v}: steps or whole"))?;
                }
                "--no-grammar" => o.grammar = false,
                "--request-mode" => {
                    let v = value("--request-mode")?;
                    o.request = RequestMode::parse(&v)
                        .ok_or(format!("--request-mode {v}: thinking-off"))?;
                }
                "--catalogue" => {
                    let v = value("--catalogue")?;
                    o.catalogue =
                        Size::parse(&v).ok_or(format!("--catalogue {v}: whole or minimal"))?;
                }
                "--module-dir" => config.module_dir = Some(value("--module-dir")?.into()),
                "--max-tokens" => {
                    let v = value("--max-tokens")?;
                    o.max_tokens = v.parse().map_err(|_| format!("--max-tokens {v}"))?;
                }
                "-T" | "--max-time" => {
                    let v = value("-T")?;
                    o.clock_s = v.parse().map_err(|_| format!("-T {v}"))?;
                }
                "--seed" => {
                    let v = value("--seed")?;
                    o.seed = v.parse().map_err(|_| format!("--seed {v}"))?;
                }
                "--cut" => {
                    let v = value("--cut")?;
                    o.cut = Cut::parse(&v).ok_or(format!("--cut {v}: end or keep-forms"))?;
                }
                "--no-format" => config.format = false,
                "--out" => out = Some(value("--out")?.into()),
                "--explain" => explain = Some(value("--explain")?.into()),
                "--record" => record = Some(value("--record")?.into()),
                "--gold-answer" => gold = Some(value("--gold-answer")?),
                s if s.starts_with('-') => return Err(format!("unknown option {s}")),
                s if text.is_none() => text = Some(s.into()),
                s => return Err(format!("one text only: {s}")),
            }
            Ok(())
        })();
        if let Err(e) = r {
            return usage_error(&e);
        }
    }
    let Some(text_path) = text else {
        return usage_error("from-text takes a TEXT file");
    };
    let Some(answer_line) = prompts::answer_line(&line) else {
        return usage_error(&format!("--answer-line {line}: zebra, word or generic"));
    };
    let text = match std::fs::read_to_string(&text_path) {
        Ok(t) => t,
        Err(e) => return usage_error(&format!("{}: {e}", text_path.display())),
    };
    let id = id.unwrap_or_else(|| {
        text_path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "text".to_string())
    });
    config.analysis = match analysis.open() {
        Ok(a) => a,
        Err(e) => return usage_error(&e),
    };
    if let Some(a) = &config.analysis
        && let Err(e) = einaf::formalize::fits(a, &text, config.options)
    {
        return usage_error(&e);
    }
    let mut runtime = match rt.open() {
        Ok(r) => r,
        Err(Opened::Failed(f)) => {
            eprintln!("error: the model did not load: {f}");
            return ExitCode::from(1);
        }
        Err(e) => return usage_error(&e.to_string()),
    };
    let input = Input {
        id,
        text,
        answer_line,
    };
    let t0 = Instant::now();
    let mut calls: Vec<Recorded> = Vec::new();
    let ft = match from_text::run(runtime.as_mut(), &input, &config, &mut calls) {
        Ok(ft) => ft,
        Err(e) => return usage_error(&e),
    };
    eprint!("{}", progress(&ft));
    eprintln!("einaf from-text: {:.1} s", t0.elapsed().as_secs_f64());
    let scored = gold.as_deref().map(|g| answer::score(&ft.answer, g));
    if let Some(s) = &scored {
        eprintln!(
            "against the gold answer \"{}\": {} ({})",
            s.gold, s.verdict, s.why
        );
    }
    let written = (|| -> Result<(), String> {
        if let (Some(path), Some(p)) = (&out, &ft.run.program) {
            std::fs::write(path, p).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        if let Some(path) = &explain {
            let md = match from_text::explain(&ft, &config) {
                Some(Ok(md)) => md,
                Some(Err(e)) => format!("# No trace\n\n{e}\n"),
                None => "# No trace\n\nNo program was written.\n".to_string(),
            };
            std::fs::write(path, md).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        if let Some(dir) = &record {
            from_text::write_record(dir, &ft, &config, &calls, scored.as_ref())
                .map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        Ok(())
    })();
    print!("{}", ft.answer.print());
    if let Err(e) = written {
        eprintln!("error: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

/// **The run in lines**, for stderr: each call with its tokens, its stop and
/// its time; each step's parse and judgement at root; the complete program
/// under each question asked, with the time of each.
pub fn progress(ft: &FromText) -> String {
    let run = &ft.run;
    let mut s = String::new();
    if let Some(a) = &run.analysis {
        s.push_str(&format!("{}\n", analysis_cmd::line(a)));
    }
    for st in &run.steps {
        let parsed = match &st.parsed {
            Some(p) if p.forms.is_some() => format!("{} forms", p.forms.unwrap_or(0)),
            Some(p) => format!(
                "does not parse: {}",
                p.findings.first().map(|f| f.message.as_str()).unwrap_or("")
            ),
            None => "no output".to_string(),
        };
        s.push_str(&format!(
            "step {:<5} {} prompt tokens, {} generated ({}, {:.1} s), {}",
            st.step,
            st.prompt_tokens.unwrap_or(0),
            st.generated_tokens.unwrap_or(0),
            st.stop.as_deref().or(st.failure.as_deref()).unwrap_or("-"),
            st.generation_ms.unwrap_or(0.0) / 1e3,
            parsed
        ));
        if let Some(c) = &st.cut {
            s.push_str(&format!(
                "; cut by the budget: {} forms kept, {} repeats dropped",
                c.kept, c.repeats
            ));
        }
        if let Some(j) = &st.judged
            && run.program.as_ref() != st.program.as_ref()
        {
            let root = j
                .feedback
                .first()
                .and_then(|f| f.outcome.as_ref())
                .and_then(|o| o.first())
                .map(|r| r.outcome.word().to_string())
                .unwrap_or_else(|| "no outcome".to_string());
            s.push_str(&format!(
                "; at root {root}; H from {} ({}); {} findings",
                j.root.writer.as_deref().unwrap_or("none"),
                j.root.candidates,
                j.check.findings.len()
            ));
        }
        s.push('\n');
    }
    if let Some(why) = &run.stopped {
        s.push_str(&format!("stopped: {why}\n"));
    }
    if let Some(c) = &run.complete {
        s.push_str(&format!(
            "the program: H from {} ({}); {} findings of the check\n",
            c.root.writer.as_deref().unwrap_or("none"),
            c.root.candidates,
            c.check.findings.len()
        ));
    }
    for f in &ft.asked {
        let outcome = f
            .outcome
            .as_ref()
            .and_then(|o| o.first())
            .map(|r| format!("{} ({})", r.outcome.word(), r.why))
            .unwrap_or_else(|| "no outcome".to_string());
        s.push_str(&format!(
            "  {}: {outcome}, {:.1} s\n",
            f.asked.question,
            (f.work.load_us + f.work.ask_us) as f64 / 1e6
        ));
    }
    s.push_str(&format!(
        "the answer: {} (from {})\n",
        ft.answer.case.word(),
        ft.answer.from
    ));
    s
}
