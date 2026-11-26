//! **The grammars held to the parser**, with no model.
//!
//! - The whole grammar's file is what `einaf grammar write` writes, and its
//!   manifest names it by digest.
//! - **The corpus through EinAf's acceptor** (test 1): every `.ein` file under
//!   `examples/`, `stdlib/` and `tests/` is accepted by the whole grammar,
//!   from its root `program`, exactly when `ein_ir::parse`, reached through
//!   `einaf::kernel::parse`, returns no error; the refusals no context-free
//!   grammar states are named (a text nested past 1 000 parentheses, a byte
//!   that is not UTF-8). And every shape of the kernel's banked table of
//!   refused shapes, `ein-ir/tests/golden/grammar_table.txt`, is refused by
//!   the grammar.
//! - **The grammar's own texts through the parser** (test 2): texts sampled
//!   from the whole grammar, a stated number from a stated seed, each parses;
//!   those of the root `root`, and of each step's grammar, hold none of
//!   [`einaf::grammar::EXCLUDED`] but in a sentence of the input, and no `/`
//!   outside a string or a comment.
//! - **The root a model writes under** refuses the chat template's markers,
//!   the fence, a block comment and those characters in a string or a
//!   comment, and accepts the contract's programs and every corpus file the
//!   parser takes that holds none of them, its comments emptied or as
//!   written.
//!
//! `--nocapture` prints the counts and what each test cost.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::time::Instant;

use einaf::grammar::gbnf::{Compiled, Rng, Verdict};
use einaf::grammar::{MANIFEST_FILE, WHOLE_FILE, whole};
use einaf::kernel;

fn package() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn compiled() -> Compiled {
    Compiled::parse(&einaf::grammar::whole_text()).expect("the whole grammar reads")
}

/// **The file under `einaf/grammars/` is what the command writes**, and the
/// manifest names it by its digest.
#[test]
fn the_grammar_file_is_what_einaf_grammar_write_writes() {
    let want = einaf::grammar::whole_text();
    let have = std::fs::read_to_string(package().join(WHOLE_FILE)).unwrap_or_default();
    assert!(
        have == want,
        "{WHOLE_FILE} is not what `einaf grammar write` writes from ein.l and ein.y: \
         run `cargo run --manifest-path einaf/Cargo.toml -- grammar write`"
    );
    let manifest = std::fs::read_to_string(package().join(MANIFEST_FILE)).unwrap_or_default();
    assert_eq!(
        manifest,
        einaf::grammar::manifest_text(&want),
        "{MANIFEST_FILE} does not name the grammar by its digest"
    );
    assert!(
        want.contains("Do not edit"),
        "the file says it is generated"
    );
}

/// **The inventory names every hand-written rule**, with the rule of `ein.l`
/// or `ein.y` it stands for.
#[test]
fn the_inventory_names_every_hand_written_rule() {
    let readme = std::fs::read_to_string(package().join("grammars/README.md"))
        .expect("the grammars' inventory");
    let words = whole::lexical(whole::EIN_L).expect("ein.l reads").words;
    assert_eq!(words.len(), 29, "the inventory counts the words no name is");
    let root_words = words
        .iter()
        .filter(|w| w.chars().all(root_name_char))
        .count();
    assert_eq!(
        root_words, 26,
        "the inventory counts the words no name the model writes is"
    );
    for (rule, stands_for, _) in whole::HAND_WRITTEN {
        let cell = if rule.starts_with('(') || rule.starts_with("the ") {
            rule.to_string()
        } else {
            rule.split(", ")
                .map(|r| format!("`{r}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        assert!(
            readme.contains(&format!("| {cell} |")),
            "the inventory has no row for {rule} ({stands_for})"
        );
    }
}

/// A corpus file's verdicts.
struct Read {
    path: String,
    /// The parser: `Ok(())`, or the kinds of its findings.
    parser: Result<(), Vec<String>>,
    /// The line and column of the parser's first finding.
    first: Option<(usize, usize)>,
    /// The acceptor's verdict, or why it gave none.
    grammar: Result<Verdict, String>,
    bytes: usize,
}

/// The refusals no context-free grammar states, by the parser's kind.
const NOT_THE_GRAMMARS: [&str; 2] = ["nesting-limit", "unreadable"];

/// **Test 1: every corpus file, accepted by the grammar exactly when the
/// parser accepts it**, the named refusals aside.
#[test]
fn the_grammar_accepts_a_corpus_file_exactly_when_the_parser_does() {
    let t0 = Instant::now();
    let c = compiled();
    let root = ein_corpus::repo_root();
    let files = ein_corpus::corpus_files();
    let reads: Vec<Read> = ein_corpus::pool::map(
        "grammar-corpus",
        &files,
        |p: &PathBuf| (p.display().to_string(), "grammar".to_string()),
        |p: &PathBuf| {
            let rel = p.strip_prefix(&root).unwrap_or(p).display().to_string();
            let bytes = std::fs::read(p).unwrap_or_default();
            match String::from_utf8(bytes.clone()) {
                Err(_) => Read {
                    path: rel,
                    parser: Err(vec!["unreadable".to_string()]),
                    first: None,
                    grammar: Err("not UTF-8".to_string()),
                    bytes: bytes.len(),
                },
                Ok(text) => {
                    let parsed = kernel::parse(&text, &rel);
                    let parser = if parsed.forms.is_some() {
                        Ok(())
                    } else {
                        Err(parsed.findings.iter().map(|f| f.kind.clone()).collect())
                    };
                    Read {
                        path: rel,
                        parser,
                        first: parsed
                            .findings
                            .first()
                            .and_then(|f| f.place.as_ref())
                            .map(|p| (p.line as usize, p.col as usize)),
                        grammar: c.accepts(whole::PROGRAM, &text),
                        bytes: text.len(),
                    }
                }
            }
        },
    );
    let (mut both, mut neither, mut named) = (0, 0, Vec::new());
    let (mut same_line, mut same_place) = (0, 0);
    let mut disagree = Vec::new();
    for r in &reads {
        let accepted = matches!(r.grammar, Ok(Verdict::Accepted));
        match (&r.parser, accepted) {
            (Ok(()), true) => both += 1,
            (Err(_), false) if r.grammar.is_ok() => {
                neither += 1;
                if let (Ok(Verdict::Refused { line, column, .. }), Some((l, c))) =
                    (&r.grammar, r.first)
                {
                    same_line += usize::from(*line == l);
                    same_place += usize::from(*line == l && *column == c);
                }
            }
            (Err(kinds), _) if kinds.iter().all(|k| NOT_THE_GRAMMARS.contains(&k.as_str())) => {
                named.push(format!("{} ({})", r.path, kinds.join(", ")))
            }
            _ => disagree.push(format!(
                "{}: the parser {}, the grammar {:?}",
                r.path,
                match &r.parser {
                    Ok(()) => "accepts".to_string(),
                    Err(k) => format!("refuses ({})", k.join(", ")),
                },
                r.grammar
            )),
        }
    }
    let bytes: usize = reads.iter().map(|r| r.bytes).sum();
    println!(
        "test 1: {} corpus files ({bytes} bytes): {both} accepted by both, {neither} refused by both \
         ({same_line} at the parser's first error's line, {same_place} at its column too), \
         {} refused by the parser alone for a reason that is no grammar's: {:?}; {:.1} s",
        reads.len(),
        named.len(),
        named,
        t0.elapsed().as_secs_f64()
    );
    assert!(
        disagree.is_empty(),
        "the grammar and the parser part ways on {} files:\n{}",
        disagree.len(),
        disagree.join("\n")
    );
    assert!(both > 800, "the corpus is read: {both} files accepted");
}

/// The rows of the kernel's banked table of refused shapes: each row's
/// program, as the table prints it.
fn table_rows() -> Vec<(String, String)> {
    let path = ein_corpus::repo_root().join("ein.rs/crates/ein-ir/tests/golden/grammar_table.txt");
    let text = std::fs::read_to_string(&path).expect("the grammar's table is banked");
    let mut rows = Vec::new();
    let mut head: Option<String> = None;
    let mut program = String::new();
    let mut in_program = false;
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            head = Some(h.to_string());
            program.clear();
            in_program = true;
        } else if in_program {
            if let Some(l) = line.strip_prefix("    ") {
                program.push_str(l);
                program.push('\n');
            } else {
                in_program = false;
                rows.push((head.clone().expect("a row's head"), program.clone()));
            }
        }
    }
    rows
}

/// **Every shape the kernel's table refuses at the parse, the grammar
/// refuses too.**
#[test]
fn every_shape_of_the_kernels_table_is_refused_by_the_grammar() {
    let c = compiled();
    let rows = table_rows();
    assert!(rows.len() > 300, "the table is read: {} rows", rows.len());
    let mut taken = Vec::new();
    for (head, program) in &rows {
        assert!(
            kernel::parse(program, "<row>").forms.is_none(),
            "the parser takes a row of its own table: {head}"
        );
        if c.accepts(whole::PROGRAM, program)
            .expect("the acceptor runs")
            .accepted()
        {
            taken.push(format!("{head}\n{program}"));
        }
    }
    println!(
        "the table: {} rows, each refused by the grammar",
        rows.len()
    );
    assert!(
        taken.is_empty(),
        "the grammar takes {} rows the parser refuses:\n{}",
        taken.len(),
        taken.join("\n")
    );
}

/// The number of texts test 2 samples, and its seed.
const SAMPLES: usize = 2_000;
const SEED: u64 = 20_261_126;
/// How deep a sampled text nests, in rules, and how long it grows before
/// every rule takes its shortest alternative.
const DEPTH: u32 = 24;
const SIZE: usize = 1_200;

/// **Test 2: every text sampled from the whole grammar parses**, from each
/// of its two roots.
#[test]
fn every_text_sampled_from_the_whole_grammar_parses() {
    let t0 = Instant::now();
    let c = compiled();
    let mut rng = Rng::new(SEED);
    let mut refused = Vec::new();
    let mut chars = 0;
    let mut seen = Seen::default();
    for i in 0..SAMPLES {
        let root = if i % 2 == 0 {
            whole::ROOT
        } else {
            whole::PROGRAM
        };
        let text = c
            .sample(root, &mut rng, DEPTH, SIZE, einaf::grammar::FAVOURED)
            .expect("the root derives a text");
        chars += text.len();
        seen.read(&text);
        assert!(
            c.accepts(root, &text)
                .expect("the acceptor runs")
                .accepted(),
            "{text}"
        );
        if root == whole::ROOT
            && let Some(e) = excluded_outside(&text, &[])
        {
            refused.push(format!("{text}\n  {e}"));
        }
        let parsed = kernel::parse(&text, "<sample>");
        if parsed.forms.is_none() {
            refused.push(format!(
                "{text}\n  {}",
                parsed
                    .findings
                    .iter()
                    .map(|f| f.message.clone())
                    .collect::<Vec<_>>()
                    .join("\n  ")
            ));
        }
    }
    println!(
        "test 2: {SAMPLES} texts from the seed {SEED}, {chars} characters, {} refused or holding a \
         character the root leaves out; {:.1} s\n  \
         what they hold: {} words no name is, standing as tokens; {} names that begin with \
         one or are one cut short; {} tokens glued to a paren; {} line comments, {} block comments",
        refused.len(),
        t0.elapsed().as_secs_f64(),
        seen.words,
        seen.near,
        seen.glued,
        seen.lines,
        seen.blocks
    );
    assert!(
        seen.words > 1_000 && seen.near > 1_000 && seen.glued > 1_000 && seen.blocks > 100,
        "the samples reach the hand-written rules"
    );
    assert!(
        refused.is_empty(),
        "{} sampled texts do not parse:\n{}",
        refused.len(),
        refused
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n---\n")
    );
}

/// The words that are tokens of their own: the ten never a name and the
/// fifteen a position gives a meaning to.
const WORDS: [&str; 25] = [
    "and", "or", "neq", "rule", "query", "solve", "config", "trace", "macro", "import", "not",
    "relation", "step", "eq", "absent", "false", "true", "open", "model", "region", "merged",
    "owing", "program", "module", "commit",
];

/// What the sampled texts hold of the hand-written rules, read by a scan of
/// each text's tokens.
#[derive(Default)]
struct Seen {
    /// A word of [`WORDS`] standing as a token.
    words: usize,
    /// A name that begins with such a word, or is one cut short by at least
    /// one character and at least two long.
    near: usize,
    /// A token other than a paren with a paren just before or after it.
    glued: usize,
    lines: usize,
    blocks: usize,
}

impl Seen {
    fn read(&mut self, text: &str) {
        let b = text.as_bytes();
        let mut i = 0;
        let word = |c: u8| !(c.is_ascii_whitespace() || b"();#\"\x1c\x1d\x1e\x1f".contains(&c));
        while i < b.len() {
            match b[i] {
                b';' => {
                    self.lines += 1;
                    while i < b.len() && b[i] != b'\n' {
                        i += 1;
                    }
                }
                b'#' if b.get(i + 1) == Some(&b'|') => {
                    self.blocks += 1;
                    i += 2;
                    while i + 1 < b.len() && !(b[i] == b'|' && b[i + 1] == b'#') {
                        i += 1;
                    }
                    i += 2;
                }
                b'"' => {
                    i += 1;
                    while i < b.len() && b[i] != b'"' {
                        i += if b[i] == b'\\' { 2 } else { 1 };
                    }
                    i += 1;
                }
                c if word(c) => {
                    let start = i;
                    while i < b.len() && word(b[i]) {
                        i += 1;
                    }
                    let t = &text[start..i];
                    if WORDS.contains(&t) {
                        self.words += 1;
                    } else if WORDS
                        .iter()
                        .any(|w| t.starts_with(w) || (t.len() >= 2 && w.starts_with(t)))
                    {
                        self.near += 1;
                    }
                    let before = start.checked_sub(1).map(|j| b[j]);
                    if matches!(before, Some(b'(' | b')')) || matches!(b.get(i), Some(b'(' | b')'))
                    {
                        self.glued += 1;
                    }
                }
                _ => i += 1,
            }
        }
    }
}

// ── The step grammars ───────────────────────────────────────────────

use einaf::grammar::steps::{self, Catalogue, Params, Step};
use einaf::kernel::{Shape, Written};

/// The texts the contract's steps by hand are written for.
const TEXTS: [&str; 6] = ["youngest", "zebra", "seating", "apples", "together", "more"];

/// The top-level forms of a text, as written: each `(` … `)` at depth 0,
/// comments and strings skipped.
fn spans(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let (mut out, mut depth, mut start, mut i) = (Vec::new(), 0usize, 0usize, 0usize);
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'#' if b.get(i + 1) == Some(&b'|') => {
                while i + 1 < b.len() && !(b[i] == b'|' && b[i + 1] == b'#') {
                    i += 1;
                }
                i += 2;
                continue;
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
            }
            b'(' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    out.push(text[start..=i].to_string());
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

fn contract_file(dir: &str, file: &str) -> String {
    std::fs::read_to_string(package().join("contract").join(dir).join(file))
        .unwrap_or_else(|e| panic!("{dir}/{file}: {e}"))
}

fn canonical(form: &str) -> String {
    kernel::shapes(form, "<form>").expect("a hand-written form parses")[0]
        .canonical
        .clone()
}

/// **What a step wrote by hand**: the forms of its program that no earlier
/// step's program holds, as written, in order; where `markers`, each rule
/// after the marker the file writes directly above it.
fn part(dir: &str, step: usize, markers: bool) -> String {
    let before: BTreeSet<String> = if step == 1 {
        BTreeSet::new()
    } else {
        spans(&contract_file(dir, &format!("step{}.ein", step - 1)))
            .iter()
            .map(|f| canonical(f))
            .collect()
    };
    let file = contract_file(dir, &format!("step{step}.ein"));
    let mut out = Vec::new();
    for f in spans(&file) {
        if before.contains(&canonical(&f)) {
            continue;
        }
        if markers && f.starts_with("(rule") {
            let at = file.find(&f).expect("a form is in its file");
            let above = file[..at].trim_end_matches('\n');
            let line = above[above.rfind('\n').map_or(0, |i| i + 1)..].to_string();
            assert!(
                line.starts_with(steps::MARKER),
                "{dir}/step{step}.ein: no marker directly above {}",
                f.lines().next().unwrap_or("")
            );
            out.push(line);
        }
        out.push(f);
    }
    out.join("\n")
}

fn params(dir: &str, program_of: Option<usize>, catalogue: &Catalogue) -> Params {
    Params {
        sentences: Some(einaf::contract::sentences(&contract_file(dir, "text.txt"))),
        catalogue: catalogue.clone(),
        program: program_of.map_or_else(Default::default, |n| {
            kernel::vocabulary(&contract_file(dir, &format!("step{n}.ein")), dir)
                .expect("a step's program parses")
        }),
    }
}

/// A step's grammar, rendered for a text's hand-written steps: step 1 on
/// the input's sentences; step 2 on the catalogue and step 1's program;
/// steps 2b and 3 on the program of steps 1 and 2.
fn rendered(dir: &str, step: Step, catalogue: &Catalogue) -> steps::Rendered {
    let program = match step {
        Step::One => None,
        Step::Two => Some(1),
        Step::TwoRules | Step::Three => Some(2),
    };
    steps::render(step, &params(dir, program, catalogue))
}

/// **Each step's grammar accepts that step's hand-written part**, rendered
/// on its vocabulary, for every text of the contract's steps by hand; step
/// 2's second call is held to the rules step 3 wrote, each after a marker.
#[test]
fn each_steps_grammar_accepts_the_steps_written_by_hand() {
    let catalogue = Catalogue::library();
    let mut refused = Vec::new();
    let mut n = 0;
    for dir in TEXTS {
        for step in Step::ALL {
            let text = match step {
                Step::One => part(dir, 1, false),
                Step::Two => part(dir, 2, false),
                Step::Three => part(dir, 3, true),
                Step::TwoRules => spans(&part(dir, 3, false))
                    .into_iter()
                    .filter(|f| f.starts_with("(rule"))
                    .map(|f| {
                        let name = f.split_whitespace().nth(1).unwrap_or("").to_string();
                        format!(";; missing: {name}\n{f}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            };
            if text.is_empty() {
                continue;
            }
            n += 1;
            let g = rendered(dir, step, &catalogue);
            let c = Compiled::parse(&g.text).expect("a step's grammar reads");
            match c.accepts(&g.root, &text).expect("the acceptor runs") {
                Verdict::Accepted => {}
                Verdict::Refused { at, line, column } => refused.push(format!(
                    "{dir}, step {}: refused at {line}:{column}, {:?}, in\n{text}",
                    step.word(),
                    &text[at..text.len().min(at + 30)]
                )),
            }
        }
    }
    println!("the steps by hand: {n} parts, each accepted by its step's grammar");
    assert!(refused.is_empty(), "{}", refused.join("\n\n"));
}

/// Whether a text's forms are of `step`'s grammar's, in its order.
fn of_its_step(step: Step, forms: &[einaf::kernel::Form]) -> Result<(), String> {
    // Each form's part, in the step's order.
    let mut parts: Vec<u8> = Vec::new();
    for f in forms {
        let p = match (step, &f.shape) {
            (
                Step::One,
                Shape::Fact {
                    source: Some(_),
                    written: Written::Holds | Written::Negated,
                    ..
                },
            )
            | (Step::One, Shape::Query { .. }) => 0,
            (
                Step::One,
                Shape::Relation { .. }
                | Shape::Fact {
                    written: Written::Holds,
                    ..
                },
            ) => 1,
            (Step::Two, Shape::Relation { .. }) => 0,
            (Step::Two, Shape::Import { .. }) => 1,
            // An ontology fact before the imports, an activating fact after
            // them: either is a fact.
            (
                Step::Two,
                Shape::Fact {
                    written: Written::Holds,
                    ..
                },
            ) => *parts.last().unwrap_or(&0),
            (Step::TwoRules, Shape::Rule { .. }) => 0,
            (
                Step::Three,
                Shape::Fact {
                    written: Written::Holds,
                    ..
                },
            ) => 0,
            (Step::Three, Shape::Rule { .. }) => 1,
            (
                Step::Three,
                Shape::Solve
                | Shape::Fact {
                    written: Written::Open,
                    ..
                },
            ) => 2,
            _ => return Err(format!("step {} writes no {}", step.word(), f.canonical)),
        };
        parts.push(p);
    }
    if step == Step::One {
        let queries = forms
            .iter()
            .filter(|f| {
                matches!(
                    f.shape,
                    Shape::Query {
                        goal_text: true,
                        ..
                    }
                )
            })
            .count();
        if queries != 1 {
            return Err(format!("{queries} answer queries"));
        }
        if !forms
            .iter()
            .any(|f| matches!(f.shape, Shape::Relation { .. }))
        {
            return Err("no declaration".to_string());
        }
    }
    if parts.windows(2).any(|w| w[0] > w[1]) && step != Step::One {
        return Err("its parts out of order".to_string());
    }
    if step == Step::One {
        let first_decl = forms
            .iter()
            .position(|f| matches!(f.shape, Shape::Relation { .. }));
        let last_statement = forms.iter().rposition(|f| {
            matches!(
                f.shape,
                Shape::Query { .. }
                    | Shape::Fact {
                        written: Written::Negated,
                        ..
                    }
            )
        });
        if let (Some(d), Some(s)) = (first_decl, last_statement)
            && s > d
        {
            return Err("a statement after the ontology".to_string());
        }
    }
    Ok(())
}

/// How many texts test 2 samples from each step's grammar, and its seed.
const STEP_SAMPLES: usize = 100;

/// **Test 2 of each step's grammar**: every text sampled from it parses,
/// and is of the forms its step writes, in its order; rendered for every
/// text of the steps by hand.
#[test]
fn every_text_sampled_from_a_steps_grammar_parses_and_is_of_its_step() {
    let t0 = Instant::now();
    let catalogue = Catalogue::library();
    let mut rng = Rng::new(SEED);
    let mut bad = Vec::new();
    let mut n = 0;
    for dir in TEXTS {
        for step in Step::ALL {
            let g = rendered(dir, step, &catalogue);
            let c = Compiled::parse(&g.text).expect("a step's grammar reads");
            let sentences = einaf::contract::sentences(&contract_file(dir, "text.txt"));
            for _ in 0..STEP_SAMPLES {
                let text = c
                    .sample(&g.root, &mut rng, 12, 1_500, &[])
                    .expect("the root derives a text");
                n += 1;
                if let Some(e) = excluded_outside(&text, &sentences) {
                    bad.push(format!("{dir}, step {}: {e}\n{text}", step.word()));
                }
                if text.trim().is_empty() {
                    continue;
                }
                match kernel::shapes(&text, "<sample>") {
                    Err(f) => bad.push(format!(
                        "{dir}, step {}: does not parse: {}\n{text}",
                        step.word(),
                        f.iter()
                            .map(|f| f.message.clone())
                            .collect::<Vec<_>>()
                            .join("; ")
                    )),
                    Ok(forms) => {
                        if let Err(e) = of_its_step(step, &forms) {
                            bad.push(format!("{dir}, step {}: {e}\n{text}", step.word()));
                        }
                    }
                }
            }
        }
    }
    println!(
        "test 2 of the steps' grammars: {n} texts, {STEP_SAMPLES} a grammar from the seed {SEED}, {} refused or \
         holding a character the model's grammars leave out; {:.1} s",
        bad.len(),
        t0.elapsed().as_secs_f64()
    );
    assert!(
        bad.is_empty(),
        "{}",
        bad.iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n---\n")
    );
}

// ── The root a model writes under ───────────────────────────────────

use einaf::grammar::EXCLUDED;

/// Whether a character may stand in a name or a variable the model writes.
fn root_name_char(c: char) -> bool {
    whole::ROOT_NAME_CHARS
        .iter()
        .any(|&(a, b)| a <= c && c <= b)
}

/// **Where a text holds a character the model's grammars leave out**: one of
/// [`EXCLUDED`] anywhere but in a sentence of `sentences` quoted as a string,
/// or `/` outside a string and a comment.
fn excluded_outside(text: &str, sentences: &[String]) -> Option<String> {
    let mut t = text.to_string();
    for s in sentences {
        let quoted = format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
        t = t.replace(&quoted, "\"\"");
    }
    if let Some(c) = t.chars().find(|c| EXCLUDED.contains(c)) {
        return Some(format!("holds {c:?} outside a sentence of the input"));
    }
    let (b, mut i) = (t.as_bytes(), 0);
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            b'/' => return Some("holds `/` outside a string and a comment".to_string()),
            _ => i += 1,
        }
    }
    None
}

/// The forms of `ein.y`'s `Form` and `TopForm` the contract's root does not
/// take, by their head.
const NOT_THE_CONTRACTS: [&str; 6] = ["config", "trace", "macro", "program", "module", "commit"];

/// **Why the root refuses a text the parser takes**, read off its tokens:
/// a form the contract does not admit, a block comment, or the first
/// character a name, a variable, a string or a line comment holds that the
/// model's characters leave out; `None` where it holds none of these.
fn outside_the_root(text: &str) -> Option<String> {
    for f in spans(text) {
        let head = f[1..]
            .split(|c: char| c.is_whitespace() || "()\";".contains(c))
            .next()
            .unwrap_or("");
        if NOT_THE_CONTRACTS.contains(&head) {
            return Some(format!("a `{head}` form"));
        }
    }
    let (b, mut i) = (text.as_bytes(), 0);
    let excluded = |s: &str, what: &str| {
        s.chars()
            .find(|c| EXCLUDED.contains(c))
            .map(|c| format!("{c:?} in a {what}"))
    };
    while i < b.len() {
        match b[i] {
            b';' => {
                let start = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                if let Some(e) = excluded(&text[start..i], "comment") {
                    return Some(e);
                }
            }
            b'#' if b.get(i + 1) == Some(&b'|') => return Some("a block comment".to_string()),
            b'"' => {
                let start = i;
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
                if let Some(e) = excluded(&text[start..i.min(b.len())], "string") {
                    return Some(e);
                }
            }
            b'(' | b')' => i += 1,
            c if c.is_ascii_whitespace() => i += 1,
            _ => {
                let start = i;
                while i < b.len()
                    && !(b[i].is_ascii_whitespace()
                        || b"();\"".contains(&b[i])
                        || (b[i] == b'#' && b.get(i + 1) == Some(&b'|')))
                {
                    i += 1;
                }
                let tok = &text[start..i];
                let body = tok.strip_prefix('?').unwrap_or(tok);
                if !tok.starts_with(':')
                    && let Some(c) = body.chars().find(|&c| !root_name_char(c))
                {
                    return Some(format!("{c:?} in the name `{tok}`"));
                }
            }
        }
    }
    None
}

/// **A text with its comments emptied**: each line comment its `;` alone,
/// each block comment a space; nothing else of it changed.
fn comments_emptied(text: &str) -> String {
    let (b, mut i) = (text.as_bytes(), 0);
    let mut out = String::with_capacity(text.len());
    let mut from = 0;
    while i < b.len() {
        match b[i] {
            b';' => {
                out.push_str(&text[from..=i]);
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                from = i;
            }
            b'#' if b.get(i + 1) == Some(&b'|') => {
                out.push_str(&text[from..i]);
                out.push(' ');
                i += 2;
                while i + 1 < b.len() && !(b[i] == b'|' && b[i + 1] == b'#') {
                    i += 1;
                }
                i = (i + 2).min(b.len());
                from = i;
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    out.push_str(&text[from.min(text.len())..]);
    out
}

/// **The root refuses what would let the model write the chat template's
/// markers or the fence**: a name of `<`, `>`, `/` or backticks, a block
/// comment, and `<`, `>`, `|` or a backtick in a string or a comment; each
/// text a program Ein's parser takes, and the root `program` too. And each
/// step's grammar refuses a comment line holding a marker where it takes
/// one with none.
#[test]
fn the_root_refuses_the_chat_templates_markers_the_fence_and_a_block_comment() {
    let c = compiled();
    let refused = [
        "(p </think>)",
        "(p <tool_call>)",
        "(p ``` x)",
        "(p ```\nx)",
        "(p a/b)",
        "(query :goal (p ?x/y))",
        "(p a #| a comment |# b)",
        "#| a comment |#\n(p a)",
        "(relation p T :why \"{?3} is <p>\")",
        "(relation p T :why \"{?3} is `p`\")",
        "(relation p T :why \"{?3} | p\")",
        "(p a :source \"</think>\")",
        "(p a) ; </think>",
        ";; ```\n(p a)",
    ];
    for t in refused {
        assert!(
            kernel::parse(t, "<text>").forms.is_some(),
            "the parser takes {t:?}"
        );
        assert!(
            c.accepts(whole::PROGRAM, t).unwrap().accepted(),
            "`program` takes {t:?}"
        );
        assert!(
            !c.accepts(whole::ROOT, t).unwrap().accepted(),
            "`root` refuses {t:?}"
        );
    }
    let taken = [
        "(p a.b *x+ =y- _z)",
        "(query :goal (p ?v.w+ ?_x))",
        "(relation p T :why \"{?3} is p/q, {braces} and all\")",
        "(p a) ; a comment, a/b",
    ];
    for t in taken {
        assert!(
            c.accepts(whole::ROOT, t).unwrap().accepted(),
            "`root` takes {t:?}"
        );
    }
    // A step's comment lines, and its marker.
    let catalogue = Catalogue::library();
    for step in [Step::One, Step::Three] {
        let g = rendered("zebra", step, &catalogue);
        let c = Compiled::parse(&g.text).expect("a step's grammar reads");
        let body = part(
            "zebra",
            if step == Step::One { 1 } else { 3 },
            step == Step::Three,
        );
        let ok = |t: &str| c.accepts(&g.root, t).unwrap().accepted();
        assert!(ok(&format!(";; a comment\n{body}")), "step {}", step.word());
        assert!(!ok(&format!(";; </think>\n{body}")), "step {}", step.word());
        assert!(!ok(&format!(";; `x`\n{body}")), "step {}", step.word());
    }
    // A marker holding one, in the step 3 of each text that writes a rule.
    let mut markers = 0;
    for dir in TEXTS {
        let body = part(dir, 3, true);
        let marked = body.replacen(";; missing: ", ";; missing: <x> ", 1);
        if marked == body {
            continue;
        }
        let g = rendered(dir, Step::Three, &catalogue);
        let c = Compiled::parse(&g.text).expect("step 3's grammar reads");
        assert!(c.accepts(&g.root, &body).unwrap().accepted(), "{dir}");
        assert!(!c.accepts(&g.root, &marked).unwrap().accepted(), "{dir}");
        markers += 1;
    }
    assert!(markers > 0, "a text's step 3 writes a rule");
    println!(
        "the root refuses {} texts `program` takes, and takes {}; each step's comment and marker hold no marker",
        refused.len(),
        taken.len()
    );
}

/// **The root takes the contract's programs and every corpus file the parser
/// takes that holds none of what it leaves out**, and refuses the rest:
/// each file as written, and with its comments emptied, which the hand-written
/// programs fill with backticks. A file the parser refuses is test 1's.
#[test]
fn the_root_accepts_the_contracts_programs_and_the_corpus_in_its_characters() {
    let t0 = Instant::now();
    let c = compiled();
    let repo = ein_corpus::repo_root();
    let mut files: Vec<PathBuf> = ein_corpus::corpus_files();
    let contract = package().join("contract");
    let mut programs: Vec<PathBuf> = Vec::new();
    for dir in std::fs::read_dir(&contract).expect("the contract's programs") {
        let dir = dir.expect("a directory").path();
        for f in std::fs::read_dir(&dir).expect("a text's programs") {
            let f = f.expect("a file").path();
            if f.extension().is_some_and(|e| e == "ein") {
                programs.push(f);
            }
        }
    }
    programs.sort();
    assert!(programs.len() >= 18, "the contract's programs are read");
    let n_corpus = files.len();
    files.extend(programs.iter().cloned());
    // Each file: its name, whether a contract's program, and for each
    // reading (as written, comments emptied) why the root should refuse it
    // and whether it does.
    type Reading = (Option<String>, bool);
    let reads: Vec<Option<(String, bool, Reading, Reading)>> = ein_corpus::pool::map(
        "grammar-root",
        &files,
        |p: &PathBuf| (p.display().to_string(), "root".to_string()),
        |p: &PathBuf| {
            let rel = p.strip_prefix(&repo).unwrap_or(p).display().to_string();
            let text = std::fs::read_to_string(p).ok()?;
            kernel::parse(&text, &rel).forms.as_ref()?;
            let emptied = comments_emptied(&text);
            assert!(
                kernel::parse(&emptied, &rel).forms.is_some(),
                "{rel}: the parser takes it with its comments emptied"
            );
            let read = |t: &str| -> Reading {
                (
                    outside_the_root(t),
                    c.accepts(whole::ROOT, t)
                        .expect("the acceptor runs")
                        .accepted(),
                )
            };
            Some((rel, p.starts_with(&contract), read(&text), read(&emptied)))
        },
    );
    let mut wrong = Vec::new();
    let mut tally: BTreeMap<(bool, &str), Vec<String>> = BTreeMap::new();
    let mut parsed = 0;
    for (rel, of_contract, written, emptied) in reads.iter().flatten() {
        parsed += usize::from(!of_contract);
        for (reading, (why, accepted)) in [("as written", written), ("comments emptied", emptied)] {
            if why.is_none() != *accepted {
                wrong.push(format!(
                    "{rel}, {reading}: the root {} it, the scan finds {why:?}",
                    if *accepted { "takes" } else { "refuses" }
                ));
            }
            let kind = match why.as_deref() {
                None => "taken",
                Some(w) if w.ends_with("form") => "a form the contract does not admit",
                Some("a block comment") => "a block comment",
                Some(w) if w.ends_with("comment") => "a character left out, in a comment",
                Some(w) if w.ends_with("string") => "a character left out, in a string",
                Some(_) => "a character left out, in a name",
            };
            let label = if reading == "as written" {
                "as written: "
            } else {
                "emptied: "
            };
            let key = (*of_contract, kind);
            let entry = tally.entry(key).or_default();
            entry.push(format!(
                "{label}{rel}{}",
                why.as_ref().map(|w| format!(" ({w})")).unwrap_or_default()
            ));
        }
    }
    let count = |of_contract: bool, label: &str, kind: &str| {
        tally
            .get(&(of_contract, kind))
            .map_or(0, |v| v.iter().filter(|l| l.starts_with(label)).count())
    };
    let kinds = [
        "taken",
        "a form the contract does not admit",
        "a block comment",
        "a character left out, in a comment",
        "a character left out, in a string",
        "a character left out, in a name",
    ];
    for (of_contract, what, n) in [
        (false, "corpus files the parser takes", parsed),
        (true, "contract's programs", programs.len()),
    ] {
        for label in ["as written: ", "emptied: "] {
            let parts: Vec<String> = kinds
                .iter()
                .map(|k| format!("{} {k}", count(of_contract, label, k)))
                .collect();
            println!("the root on the {n} {what}, {label}{}", parts.join("; "));
        }
    }
    for kind in [
        "a character left out, in a string",
        "a character left out, in a name",
    ] {
        for l in tally.get(&(false, kind)).into_iter().flatten() {
            if l.starts_with("emptied: ") {
                println!("  refused, {l}");
            }
        }
    }
    for l in tally
        .get(&(true, "a character left out, in a comment"))
        .into_iter()
        .flatten()
    {
        println!("  refused, {l}");
    }
    println!(
        "  of {n_corpus} corpus files, {} the parser refuses are test 1's; {:.1} s",
        n_corpus - parsed,
        t0.elapsed().as_secs_f64()
    );
    assert!(
        wrong.is_empty(),
        "the root and the scan part ways on {} readings:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
    assert_eq!(
        count(true, "emptied: ", "taken"),
        programs.len(),
        "every contract program is taken with its comments emptied"
    );
    assert!(
        count(false, "emptied: ", "taken") > 750,
        "the corpus's forms are read"
    );
}
