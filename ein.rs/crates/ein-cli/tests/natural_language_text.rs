//! **Natural-language text reaches every surface that carries it, byte for
//! byte**: a string literal is Unicode with escapes, so a program can emit
//! any text, and the examples hold a couple to show it.
//!
//! A string is natural-language text, written only after `:why`, `:source`
//! and `:goal-text` (Q-M1e2.23). `tests/text/01_unicode.ein` writes each of
//! the three with Cyrillic, CJK, an emoji and every escape a string reads
//! (`\n \t \r \" \\` and `\u{…}`), and one `:source` with a raw newline. Each
//! surface that carries a text is held to the text the program wrote, as that
//! surface writes text:
//!
//! | surface | which text | as |
//! |---|---|---|
//! | the read-out, `ein solve` and `--dump-answer`'s `answer.txt` | `:goal-text`, filled | the characters, in `“…”`, each line after a line break at the sentence's column |
//! | `--trace` | a rule's `:why`, filled at its firing, a premise's `:source`, and the derived fact's relation's `:why`, filled | the characters, each line break (`\n`, `\r`) a markdown hard break and the line after it in the same block, `> ` in the `:why`'s quote (Q-M1f.109) |
//! | `ein saturate --dump` | `:source`, `:goal-text` | quoted, as the printer quotes |
//! | a `--dump-answer` state file | all three, in the program it carries | as the program writes them, byte for byte, and it reads back to the same texts |
//! | `ein render program` | `:goal-text` | quoted, inside a DOT label |
//! | a `.einb` | all three | `ein solve` of it prints what `ein solve` of the `.ein` does |
//!
//! Which surfaces carry a text is a fixed set (Q-M1f.12): `ein render
//! rules`, `--events` and the JSON summary carry none, and this test does
//! not add one.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};
use ein_ir::{Ast, Node};

const PROGRAM: &str = "tests/text/01_unicode.ein";

/// The texts the program writes, as the parser reads them.
const RELATION_WHY: &str = "Жилец {?1} живёт в доме {?2} — «日本語», 🦓";
const HOUSED_WHY: &str = "{?1} 有房子 ✅";
const SOURCE_ANN: &str = "Условие (1): Анна живёт в первом доме 🏠\n二行目\tタブ\r\"引用\" \\ 🦓";
const SOURCE_BOB: &str = "条件 (2): ボブは二番目の家に住んでいる\n— сырая новая строка";
const RULE_WHY: &str = "{?p} 有房子, потому что живёт в {?h} 🦓\nвторая\tстрока\r\"цитата\" \\ Жук";
const GOAL_TEXT: &str = "Жилец {?who} — 住所 🦓\nстрока\tтаб\r\"цитата\" \\ 🦓";

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The read-out's sentence for `who`: the `:goal-text` filled, in `“…”`, each
/// line after a line break started at the sentence's column, 25, where its
/// first character is printed (Q-M1f.19): the
/// text's `\n` and its `\r` each start a line, and nothing else moves.
fn sentence(who: &str) -> String {
    let filled = GOAL_TEXT.replace("{?who}", who);
    let lines: Vec<&str> = filled.split(['\n', '\r']).collect();
    format!("“{}”", lines.join(&format!("\n{}", " ".repeat(25))))
}

/// Every string a program's text holds, in the order the parser met them.
fn texts(path: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(path).expect("readable");
    let mut ast = Ast::new();
    ein_ir::parse(&mut ast, &text, Some(&path.to_string_lossy())).expect("it parses");
    let mut out = Vec::new();
    for i in 0..ast.arena_sizes().0 {
        if let Node::Str(s) = ast.node(ein_ir::NodeId(i as u32)) {
            out.push(ast.sym(s).to_string());
        }
    }
    out
}

/// The program's texts are the six above: the parser reads every escape and
/// the raw newline as the characters they stand for.
#[test]
fn the_program_writes_the_six_texts() {
    let mut got = texts(&repo_root().join(PROGRAM));
    got.sort();
    let mut want = [
        RELATION_WHY,
        HOUSED_WHY,
        SOURCE_ANN,
        SOURCE_BOB,
        RULE_WHY,
        GOAL_TEXT,
    ]
    .map(String::from);
    want.sort();
    assert_eq!(got, want);
}

/// The read-out prints the `:goal-text` filled, for each row of the class,
/// each line of it inside the row, and `answer.txt` is the read-out; the claim
/// holds (`ein test`, exit 0).
#[test]
fn the_read_out_prints_the_goal_text_as_written() {
    let r = ein(&["solve", PROGRAM]);
    assert_eq!(r.code, 0, "{}", r.err);
    for who in ["Ann", "Bob"] {
        let sentence = sentence(who);
        assert!(r.out.contains(&sentence), "no {sentence:?}:\n{}", r.out);
    }
    // No line of the read-out starts outside a row: each is the header, the
    // rule, blank, or indented.
    for l in r.out.lines().skip(2) {
        assert!(
            l.is_empty() || l.starts_with("  "),
            "a line outside every row: {l:?}"
        );
    }
    let scratch = Scratch::new("ein-nl-text-answer");
    let dir = scratch.path().join("answer");
    let d = ein(&["solve", PROGRAM, "--dump-answer", &dir.to_string_lossy()]);
    assert_eq!(d.code, 0, "{}", d.err);
    let answer = std::fs::read_to_string(dir.join("answer.txt")).expect("answer.txt");
    for who in ["Ann", "Bob"] {
        let sentence = sentence(who);
        assert!(answer.contains(&sentence), "answer.txt: no {sentence:?}");
    }
    let t = ein(&["test", PROGRAM]);
    assert_eq!(t.code, 0, "{}{}", t.out, t.err);
}

/// A text as the trace's markdown writes it: each line break, `\n` and `\r`
/// alike, a hard break, two spaces before it, and the line after it led by
/// `lead`, its block's marker (Q-M1f.109): the
/// characters otherwise, byte for byte.
fn markdown(text: &str, lead: &str) -> String {
    text.split(['\n', '\r'])
        .collect::<Vec<_>>()
        .join(&format!("  \n{lead}"))
}

/// `--trace` prints the rule's `:why` filled at each firing, as a quote, and
/// each premise's `:source` beside it, each line of each a line of the
/// rendered page: a line break written as it stands would be a soft break
/// to markdown.
#[test]
fn the_trace_prints_the_rules_why_and_the_premises_source() {
    let scratch = Scratch::new("ein-nl-text-trace");
    let trace = scratch.path().join("trace.md");
    let r = ein(&["solve", PROGRAM, "--trace", &trace.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    let page = std::fs::read_to_string(&trace).expect("the trace");
    for (p, h, source) in [("Ann", "H1", SOURCE_ANN), ("Bob", "H2", SOURCE_BOB)] {
        let why = format!(
            "\n> {}\n",
            markdown(&RULE_WHY.replace("{?p}", p).replace("{?h}", h), "> ")
        );
        assert!(page.contains(&why), "the trace: no {why:?}");
        let source = markdown(source, "");
        assert!(page.contains(&source), "the trace: no {source:?}");
        // What the derived fact says, its relation's `:why`.
        let derives = format!("Derives `housed({p})` — {}", HOUSED_WHY.replace("{?1}", p));
        assert!(page.contains(&derives), "the trace: no {derives:?}");
    }
}

/// `ein saturate --dump` prints each fact's `:source` and the query's
/// `:goal-text` quoted, as the one printer quotes a text.
#[test]
fn the_dump_quotes_the_source_and_the_goal_text() {
    let r = ein(&["saturate", PROGRAM, "--dump"]);
    assert_eq!(r.code, 0, "{}", r.err);
    let q = ein_core::text::quote;
    for line in [
        format!("  (lives Ann H1 :source {})\n", q(SOURCE_ANN)),
        format!("  (lives Bob H2 :source {})\n", q(SOURCE_BOB)),
        format!(";;   :goal-text  {}\n", q(GOAL_TEXT)),
    ] {
        assert!(r.out.contains(&line), "no line {line:?}:\n{}", r.out);
    }
}

/// A `--dump-answer` state file carries the program as written, every text
/// as the program writes it, and its program reads back to the same six
/// texts.
#[test]
fn a_state_file_carries_every_text_as_written() {
    let scratch = Scratch::new("ein-nl-text-state");
    let dir = scratch.path().join("answer");
    let r = ein(&["solve", PROGRAM, "--dump-answer", &dir.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.err);
    let state = dir.join("models").join("01.ein");
    let file = std::fs::read_to_string(&state).expect("the state file");
    let program = std::fs::read_to_string(repo_root().join(PROGRAM)).expect("the program");
    assert!(
        file.contains(&format!("(program\n{program}\n)\n")),
        "the state file carries the program byte for byte"
    );
    let mut ast = Ast::new();
    ein_ir::saved::parse_text(&mut ast, &file, Some(&state.to_string_lossy()))
        .expect("its program parses");
    let mut got: Vec<String> = (0..ast.arena_sizes().0)
        .filter_map(|i| match ast.node(ein_ir::NodeId(i as u32)) {
            Node::Str(s) => Some(ast.sym(s).to_string()),
            _ => None,
        })
        .collect();
    got.sort();
    let mut want = texts(&repo_root().join(PROGRAM));
    want.sort();
    assert_eq!(got, want);
}

/// `ein render program` draws the query's `:goal-text` quoted, inside the
/// query's DOT label, where `\` and `"` are escaped once more.
#[test]
fn the_program_view_draws_the_goal_text() {
    let r = ein(&["render", "program", PROGRAM]);
    assert_eq!(r.code, 0, "{}", r.err);
    let label = ein_core::text::quote(GOAL_TEXT)
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    assert!(
        r.out.contains(&format!(":goal-text {label}")),
        "no {label}:\n{}",
        r.out
    );
}

/// A `.einb` holds the texts: `ein solve` of it prints what `ein solve` of the
/// `.ein` does, but for the file it names.
#[cfg(feature = "einb")]
#[test]
fn a_container_holds_the_texts() {
    let scratch = Scratch::new("ein-nl-text-einb");
    let einb = scratch.path().join("unicode.einb");
    let einb = einb.to_string_lossy();
    let s = ein(&["kb", "save", PROGRAM, &einb]);
    assert_eq!(s.code, 0, "{}", s.err);
    let (a, b) = (ein(&["solve", PROGRAM]), ein(&["solve", &einb]));
    assert_eq!((a.code, b.code), (0, 0), "{}{}", a.err, b.err);
    let headless = |out: &str| out.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(headless(&a.out), headless(&b.out));
    assert!(b.out.contains(&sentence("Ann")));
}
