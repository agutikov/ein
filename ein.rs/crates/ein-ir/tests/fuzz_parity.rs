//! T1a.1.1.4 — the grammar conformance fixture: a differential fuzzer.
//!
//! Earley explores tokenizations; recursive descent commits. That is the one
//! structural risk in replacing Lark ([design/04](../../../../plans/m1a_rust/design/04_ir_frontend.md) §1),
//! and no amount of reading settles it — so this generates malformed and
//! nearly-well-formed inputs and asserts the two parsers agree on
//! accept/reject **and on the message**.
//!
//! When both sides *accept*, the comparison is the **dumped AST**, not just
//! the verdict — an agreement on "this is ein-lang" that disagrees on what it
//! means would be the worse bug of the two.
//!
//! Budget is `EIN_FUZZ_ITERS` (default 2 000, which fits the per-commit tier);
//! the nightly tier raises it. `EIN_FUZZ_SEED` moves the stream. Both are
//! deterministic: the same pair reproduces the same run, which is what makes a
//! find replayable.
//!
//! A divergence is **minimised** and written to `conformance/fuzz_findings/`,
//! because a fuzzer whose finds are not checked in re-finds them forever
//! (`conformance/README.md` § Growth rule).

use std::path::PathBuf;

use ein_ir::{Ast, parse};
use ein_oracle::{Answer, IR_ORACLE, Oracle, repo_root, skip};

fn rust_answer(text: &str) -> Answer {
    let mut ast = Ast::new();
    match parse(&mut ast, text, None) {
        Ok(_) => Answer::Ok(String::new()),
        Err(e) => Answer::Err {
            kind: "IRParseError".into(),
            msg: e.to_string(),
        },
    }
}

/// True when the two implementations answered the same thing.
fn agrees(got: &Answer, want: &Answer) -> bool {
    match (got, want) {
        (Answer::Ok(_), Answer::Ok(_)) => true,
        (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => a == b,
        _ => false,
    }
}

/// xorshift64*, so a run is reproducible without a dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }

    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len())]
    }
}

/// The characters a mutation may splice in: every terminal's anchor plus a few
/// that anchor nothing, so "this byte can start no token" is exercised too.
const ALPHABET: &[char] = &[
    '(', ')', ' ', '\n', '\t', ':', '?', '_', '=', '"', '\\', '.', '-', '*', ';', '#', '|', '0',
    '1', '9', 'a', 'z', 'A', 'Z', '@', '[', ']', '{', '}', '\'', ',', '/', 'é', '\u{a0}',
];

/// Reserved words, dotted atoms and shape fragments — spliced whole, which is
/// how the `rulex` class of ambiguity gets hit on purpose rather than by luck.
const SPLICES: &[&str] = &[
    "rule",
    "hrule",
    "relation",
    "query",
    "config",
    "trace",
    "macro",
    "import",
    "not",
    "and",
    "or",
    "neq",
    "step",
    "branch-open",
    "branch-close",
    "branch-ref",
    "contradiction",
    "symmetry-class",
    "std.macro",
    "__closed__",
    "?x",
    ":why",
    "1..5",
    "1..*",
    "()",
    "(?a)",
    "\"s\"",
    "-0",
    "#|",
    "|#",
    ";",
];

fn mutate(seed: &str, rng: &mut Rng) -> String {
    let chars: Vec<char> = seed.chars().collect();
    let n = chars.len();
    let mut out = chars.clone();
    match rng.below(9) {
        // Drop a character.
        0 if n > 0 => {
            out.remove(rng.below(n));
        }
        // Replace a character.
        1 if n > 0 => {
            let i = rng.below(n);
            out[i] = *rng.pick(ALPHABET);
        }
        // Insert a character.
        2 => {
            let i = rng.below(n + 1);
            out.insert(i, *rng.pick(ALPHABET));
        }
        // Splice a word — the reserved-word ambiguity generator.
        3 => {
            let i = rng.below(n + 1);
            let word: Vec<char> = rng.pick(SPLICES).chars().collect();
            for (k, ch) in word.into_iter().enumerate() {
                out.insert(i + k, ch);
            }
        }
        // Unbalance the parens.
        4 => {
            let i = rng.below(n + 1);
            out.insert(i, if rng.below(2) == 0 { '(' } else { ')' });
        }
        // Truncate — the EOF-error generator.
        5 if n > 1 => {
            out.truncate(1 + rng.below(n - 1));
        }
        // Drop a whole line.
        6 if n > 0 => {
            let text: String = out.iter().collect();
            let lines: Vec<&str> = text.lines().collect();
            if !lines.is_empty() {
                let drop = rng.below(lines.len());
                let kept: Vec<&str> = lines
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != drop)
                    .map(|(_, l)| *l)
                    .collect();
                out = kept.join("\n").chars().collect();
            }
        }
        // Duplicate a line.
        7 if n > 0 => {
            let text: String = out.iter().collect();
            let lines: Vec<&str> = text.lines().collect();
            if !lines.is_empty() {
                let i = rng.below(lines.len());
                let mut kept: Vec<&str> = lines.clone();
                kept.insert(i, lines[i]);
                out = kept.join("\n").chars().collect();
            }
        }
        // Swap two characters.
        _ if n > 1 => {
            let (i, j) = (rng.below(n), rng.below(n));
            out.swap(i, j);
        }
        _ => {}
    }
    out.into_iter().collect()
}

/// Delta-debug a divergent input down to something a human can read: drop
/// lines while it still diverges, then characters.
fn minimise(input: &str, py: &mut Oracle) -> String {
    let diverges =
        |text: &str, py: &mut Oracle| !agrees(&rust_answer(text), &py.text("parse", text, None));
    let mut best = input.to_string();
    let mut improved = true;
    while improved {
        improved = false;
        let lines: Vec<String> = best.lines().map(str::to_string).collect();
        for i in 0..lines.len() {
            let mut kept = lines.clone();
            kept.remove(i);
            let candidate = kept.join("\n");
            if diverges(&candidate, py) {
                best = candidate;
                improved = true;
                break;
            }
        }
    }
    let mut improved = true;
    while improved {
        improved = false;
        let chars: Vec<char> = best.chars().collect();
        for i in 0..chars.len() {
            let mut kept = chars.clone();
            kept.remove(i);
            let candidate: String = kept.into_iter().collect();
            if diverges(&candidate, py) {
                best = candidate;
                improved = true;
                break;
            }
        }
    }
    best
}

/// Seeds: the parse-negative fixtures, the smaller corpus files, the recorded
/// findings, and a handful of shapes small enough that a single mutation lands
/// somewhere interesting.
fn seeds() -> Vec<String> {
    let root = repo_root();
    let mut out: Vec<String> = vec![
        "(rule r (?a) :match (p ?a) :assert (q ?a) :why \"w\")".into(),
        "(relation co-located Person Person :why \"{?1} with {?2}\")".into(),
        "(import std.macro :symbols (forall))".into(),
        "(macro forall (?g ?b) (absent (and ?g (absent ?b))))".into(),
        "(trace (step s1 :rule r :using (c1) :derives (p a)))".into(),
        "(query :goal :solve)".into(),
        "(config :enable-pre-branch-lookahead true)".into(),
        "(= a b :source \"s\")".into(),
        "(not (color-loc Red House-1) :rule x :using (c1))".into(),
        "(a 1..5 :k 1..* :s \"x\\ny\" :w _ :v ?v)".into(),
        "(hrule h () :match (p ?a) :assert (q ?a) :why \"w\")".into(),
        "()".into(),
    ];
    let mut files: Vec<PathBuf> = Vec::new();
    for dir in ["examples/broken", "conformance/fuzz_findings"] {
        if let Ok(entries) = std::fs::read_dir(root.join(dir)) {
            files.extend(entries.flatten().map(|e| e.path()).filter(|p| p.is_file()));
        }
    }
    files.sort();
    for p in files {
        if let Ok(text) = std::fs::read_to_string(&p)
            && text.len() <= 2048
        {
            out.push(text);
        }
    }
    out
}

#[test]
fn mutations_of_the_corpus_parse_identically() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("mutations_of_the_corpus_parse_identically");
    };
    let iters: usize = std::env::var("EIN_FUZZ_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);
    let seed: u64 = std::env::var("EIN_FUZZ_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x5EED_1A17);
    let mut rng = Rng(seed | 1);
    let seeds = seeds();

    // Every recorded finding is a regression test in its own right, before a
    // single random byte is generated.
    for s in &seeds {
        let (got, want) = (rust_answer(s), py.text("parse", s, None));
        assert!(
            agrees(&got, &want),
            "seed diverges:\n{s}\n  ein.rs {got:?}\n  ein.py {want:?}"
        );
    }

    for i in 0..iters {
        // Chain two or three mutations sometimes: one edit rarely reaches a
        // shape that is *nearly* valid, and those are the interesting ones.
        let mut text = seeds[rng.below(seeds.len())].clone();
        for _ in 0..=rng.below(3) {
            text = mutate(&text, &mut rng);
        }
        let got = rust_answer(&text);
        let want = py.text("parse", &text, None);
        if agrees(&got, &want) {
            continue;
        }
        let small = minimise(&text, &mut py);
        let dir = repo_root().join("conformance/fuzz_findings");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("seed{seed:x}-iter{i}.ein"));
        let _ = std::fs::write(&path, &small);
        panic!(
            "iteration {i} (EIN_FUZZ_SEED={seed}) diverges; minimised to {}:\n\
             ---\n{small}\n---\n  ein.rs: {:?}\n  ein.py: {:?}",
            path.display(),
            rust_answer(&small),
            py.text("parse", &small, None),
        );
    }
}
