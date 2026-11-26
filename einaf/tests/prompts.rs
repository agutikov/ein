//! **The prompts' Ein is Ein**: every program a prompt file of EinAf's shows,
//! each fenced block marked `lisp`, loads through the bounded call and is not
//! refused; and the probe's plan quotes only programs that load, each held
//! to its family rule.

use einaf::kernel::{self, Bounds, Outcome, Question, Source};
use einaf::probe;

fn blocks(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut lines = text.lines();
    while let Some(l) = lines.next() {
        if l.trim_start().starts_with("```lisp") {
            let mut block = Vec::new();
            for l in lines.by_ref() {
                if l.trim_start().starts_with("```") {
                    break;
                }
                block.push(l);
            }
            out.push(block.join("\n"));
        }
    }
    out
}

fn assert_loads(text: &str, name: &str) {
    let f = kernel::call(
        Source::Text {
            text,
            name,
            base_dir: None,
        },
        &[Question::NoSearch],
        Bounds::default(),
    )
    .pop()
    .expect("one object");
    let invalid = f
        .outcome
        .iter()
        .flatten()
        .any(|r| r.outcome == Outcome::Invalid);
    assert!(!invalid, "{name} is refused:\n{text}\n{:#?}", f.diagnostics);
}

#[test]
fn every_program_the_excerpt_shows_loads() {
    let b = blocks(probe::EXCERPT);
    assert_eq!(b.len(), 4, "the excerpt's blocks");
    for (i, block) in b.iter().enumerate() {
        assert_loads(block, &format!("ein-excerpt.v1.md block {}", i + 1));
    }
}

#[test]
fn every_program_the_steps_excerpt_shows_loads() {
    let b = blocks(einaf::prompts::EXCERPT.text);
    assert_eq!(b.len(), 4, "the excerpt's blocks");
    for (i, block) in b.iter().enumerate() {
        assert_loads(block, &format!("ein-excerpt.v2.md block {}", i + 1));
        assert!(
            !block.contains(['`', '<', '>', '|']),
            "a program the excerpt shows holds a character no grammar lets the model write"
        );
    }
}

#[test]
fn every_template_renders_with_its_slots_and_no_other() {
    use einaf::prompts::{STEP1, STEP2, STEP2B, STEP3, SYSTEM, WHOLE};
    use std::collections::BTreeMap;
    let fill = |names: &[&'static str]| -> BTreeMap<&'static str, String> {
        names.iter().map(|n| (*n, format!("<{n}>"))).collect()
    };
    let cases: [(einaf::prompts::Template, &[&'static str]); 6] = [
        (SYSTEM, &["excerpt"]),
        (STEP1, &["examples", "text", "answer_line"]),
        (STEP2, &["catalogue", "examples", "text", "program"]),
        (
            STEP2B,
            &["markers", "catalogue", "examples", "text", "program"],
        ),
        (STEP3, &["catalogue", "examples", "text", "program"]),
        (WHOLE, &["catalogue", "examples", "text", "answer_line"]),
    ];
    for (t, names) in cases {
        let text = t.render(&fill(names)).expect("renders");
        assert!(
            !text.contains("%%"),
            "{}: a note of the template is rendered",
            t.file
        );
        assert!(!text.contains("{{"), "{}", t.file);
        let mut more = fill(names);
        more.insert("unknown", String::new());
        assert!(
            t.render(&more).is_err(),
            "{}: a slot it has not is refused",
            t.file
        );
        if let Some(first) = names.first() {
            let mut fewer = fill(names);
            fewer.remove(first);
            assert!(
                t.render(&fewer).is_err(),
                "{}: a slot not filled is refused",
                t.file
            );
        }
    }
}

#[test]
fn the_plan_quotes_only_programs_that_load() {
    let plan = probe::plan();
    assert_eq!(plan.len(), 87, "the plan's runs");
    let mut quoted: Vec<&str> = plan
        .iter()
        .flat_map(|p| p.examples.iter().map(String::as_str))
        .collect();
    quoted.sort();
    quoted.dedup();
    for path in quoted {
        let f = kernel::call(
            Source::File(&probe::root().join(path)),
            &[Question::NoSearch],
            Bounds::default(),
        )
        .pop()
        .expect("one object");
        assert!(
            !f.outcome
                .iter()
                .flatten()
                .any(|r| r.outcome == Outcome::Invalid),
            "{path} is refused"
        );
    }
}

/// **The parse alone** (`einaf::kernel::parse`), on an answer read two ways:
/// whole, a fenced program is the parse's two errors at the fence's lines,
/// each a finding of the kind `parse` with its place and its repair; by its
/// fenced block, the program, which parses and loads nothing.
#[test]
fn a_fenced_answer_read_whole_is_refused_at_its_fence_lines_only() {
    let answer = "```ein\n(relation older Person Person :why \"a {?3} is older than a {?4}\")\n(older Ann Bob)\n```\n";
    let whole = kernel::parse(answer, "answer.ein");
    assert_eq!(whole.forms, None);
    let places: Vec<(u32, u32)> = whole
        .findings
        .iter()
        .map(|f| {
            assert_eq!(f.kind, "parse");
            assert!(f.message.contains("repair: Delete"), "{}", f.message);
            let p = f.place.as_ref().expect("a place");
            (p.line, p.col)
        })
        .collect();
    assert_eq!(places, [(1, 1), (4, 1)]);
    let fenced = einaf::model::first_fenced_block(answer).expect("a block");
    let parsed = kernel::parse(&fenced, "fenced.ein");
    assert_eq!(parsed.forms, Some(2));
    assert!(parsed.findings.is_empty());
}
