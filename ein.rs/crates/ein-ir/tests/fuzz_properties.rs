//! The frontend fuzzer, on **self-checkable properties** — T1a.1.1.4's
//! generator, after its oracle.
//!
//! Earley explores tokenizations; recursive descent commits. That is the one
//! structural risk in replacing Lark ([design/04](../../../../plans/m1a_rust/design/04_ir_frontend.md) §1),
//! and the fuzzer settled it by generating malformed and nearly-well-formed
//! inputs and asserting the two parsers agreed on accept/reject **and on the
//! message**. That arm is
//! [accepted loss L1](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#6-accepted-loss):
//! the input space no fixture covers had exactly one owner and it is gone.
//!
//! What survives is the generator, the minimiser, the seed corpus, and three
//! properties that need one implementation:
//!
//! 1. **It never panics.** Generated input either parses or is *diagnosed* —
//!    a recursive-descent parser that unwrapped an assumption would show up
//!    here as a crash rather than as a wrong answer.
//! 2. **What parses round-trips.** `dump_canonical → parse → dump_canonical`
//!    is a fixed point on every mutation that parses. This is the strongest
//!    self-checkable property the frontend has, and it is the one that would
//!    have caught a dumper/parser disagreement without a second engine.
//! 3. **The recorded findings still parse the way they parsed.** Every seed
//!    and every checked-in `corpus/fuzz_findings/*.ein` against a table —
//!    which is what makes a find a regression test rather than a souvenir.
//!
//! Property 2 is not weaker than the diff in every direction: the diff
//! compared ein.rs's answer to lark's, and a *shared* misunderstanding of the
//! grammar would have satisfied it. A round-trip is a claim about this
//! engine's two halves agreeing, and no oracle could have made it.
//!
//! Budget is `EIN_FUZZ_ITERS` (default 2 000, which fits the per-commit tier);
//! the nightly tier raises it. `EIN_FUZZ_SEED` moves the stream. Both are
//! deterministic: the same pair reproduces the same run, which is what makes a
//! find replayable.

use std::path::PathBuf;

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::dump::dump_canonical;
use ein_ir::{Ast, parse};

/// One parse, as the text a table can hold.
fn answer(text: &str) -> String {
    let mut ast = Ast::new();
    match parse(&mut ast, text, None) {
        Ok(forms) => format!("ok {} form(s)", forms.len()),
        Err(e) => format!("refused {e}"),
    }
}

/// `dump_canonical → parse → dump_canonical`, or the reason it could not be
/// taken. `None` for input that does not parse — there is nothing to
/// round-trip.
fn round_trip(text: &str) -> Option<Result<(), String>> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, None).ok()?;
    let once = dump_canonical(&ast, &forms);
    Some(match parse(&mut ast, &once, Some("<dumped>")) {
        Err(e) => Err(format!("a dump does not re-parse: {e}")),
        Ok(again) => {
            let twice = dump_canonical(&ast, &again);
            if twice == once {
                Ok(())
            } else {
                Err(format!(
                    "not a fixed point:\n--- once\n{once}\n--- twice\n{twice}"
                ))
            }
        }
    })
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

/// Delta-debug a failing input down to something a human can read: drop lines
/// while it still fails, then characters.
///
/// The predicate is the caller's, which is the only change S1a.10.2 made here:
/// it used to be "the two parsers disagree" and it is now "the property still
/// does not hold".
fn minimise(input: &str, fails: impl Fn(&str) -> bool) -> String {
    let mut best = input.to_string();
    let mut improved = true;
    while improved {
        improved = false;
        let lines: Vec<String> = best.lines().map(str::to_string).collect();
        for i in 0..lines.len() {
            let mut kept = lines.clone();
            kept.remove(i);
            let candidate = kept.join("\n");
            if fails(&candidate) {
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
            if fails(&candidate) {
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
    // `.ein` only. The differential arm took every file in these directories —
    // any text is a legal fuzz seed — but S1a.10.2 turned the seed corpus into
    // a checked-in table, and a table that moved when a README was edited
    // would be a table nobody trusts. What is lost is a few `.expected` and
    // `.md` files as *mutation* seeds; what is gained is that the seed list is
    // a function of the fixtures.
    let mut files: Vec<PathBuf> = Vec::new();
    for dir in ["examples/broken", "corpus/fuzz_findings"] {
        if let Ok(entries) = std::fs::read_dir(root.join(dir)) {
            files.extend(
                entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "ein")),
            );
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

/// The generator's budget and stream, as the environment sets them.
fn budget() -> (usize, u64) {
    let iters: usize = std::env::var("EIN_FUZZ_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);
    let seed: u64 = std::env::var("EIN_FUZZ_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x5EED_1A17);
    (iters, seed)
}

/// One generated input: a seed, mutated one to three times.
fn generated(seeds: &[String], rng: &mut Rng) -> String {
    let mut text = seeds[rng.below(seeds.len())].clone();
    for _ in 0..=rng.below(3) {
        text = mutate(&text, rng);
    }
    text
}

/// **Generated input is answered, never survived.**
///
/// Every mutation either parses or comes back as an `IRParseError` — there is
/// no third outcome, and in a recursive-descent parser the third outcome is a
/// panic. The counters are reported because a generator that stopped producing
/// *rejected* input would leave the diagnostic paths untested while still
/// passing: a run that accepted everything would be a run that mutated
/// nothing.
#[test]
fn generated_input_is_always_parsed_or_diagnosed() {
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let (mut accepted, mut refused) = (0usize, 0usize);
    for _ in 0..iters {
        let text = generated(&seeds, &mut rng);
        let mut ast = Ast::new();
        match parse(&mut ast, &text, None) {
            Ok(_) => accepted += 1,
            Err(_) => refused += 1,
        }
    }
    eprintln!("fuzz: {accepted} accepted, {refused} refused of {iters}");
    assert_eq!(accepted + refused, iters);
    assert!(
        accepted >= iters / 20 && refused >= iters / 20,
        "the stream is one-sided: {accepted} accepted, {refused} refused"
    );
}

/// **Everything that parses round-trips through the dumper.**
///
/// `parse(dump(parse(x)))` dumps to the same text as `dump(parse(x))`. A
/// failure is minimised before it is reported and written to
/// `corpus/fuzz_findings/`, because a fuzzer whose finds are not checked in
/// re-finds them forever (`corpus/README.md` § Growth rule).
#[test]
fn everything_that_parses_round_trips_through_the_dumper() {
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let mut round_tripped = 0usize;
    for i in 0..iters {
        let text = generated(&seeds, &mut rng);
        match round_trip(&text) {
            None => continue,
            Some(Ok(())) => round_tripped += 1,
            Some(Err(why)) => {
                let small = minimise(&text, |t| matches!(round_trip(t), Some(Err(_))));
                let dir = repo_root().join("corpus/fuzz_findings");
                let _ = std::fs::create_dir_all(&dir);
                let path = dir.join(format!("roundtrip-seed{seed:x}-iter{i}.ein"));
                let _ = std::fs::write(&path, &small);
                panic!(
                    "iteration {i} (EIN_FUZZ_SEED={seed}) does not round-trip; \
                     minimised to {}:\n---\n{small}\n---\n{why}",
                    path.display()
                );
            }
        }
    }
    assert!(
        round_tripped >= iters / 20,
        "only {round_tripped} of {iters} mutations parsed, so the property is untested"
    );
}

/// **Every seed and every recorded finding parses the way it parsed.**
///
/// The seed corpus is the twelve hand-written shapes plus every file under
/// `examples/broken/` and `corpus/fuzz_findings/` — which is what makes a
/// find a regression test. The differential arm checked each against lark on
/// every run; the table checks each against the answer lark gave, blessed
/// while that arm was green.
///
/// The table is keyed by the seed's own text, so a finding that is added,
/// removed or edited changes the file and has to be blessed on purpose.
#[test]
fn every_seed_and_finding_parses_the_way_it_was_recorded() {
    let seeds = seeds();
    // Twelve hand-written shapes, the four parse-negative fixtures and the two
    // recorded findings. A floor rather than an equality, because a new
    // finding is *supposed* to arrive here.
    assert!(seeds.len() >= 18, "only {} seeds", seeds.len());
    let mut out = String::new();
    for seed in &seeds {
        out.push_str(&format!("=== {seed:?}\n  {}\n", answer(seed)));
        // The seeds are also the smallest round-trip corpus there is, and
        // unlike the mutations they are all *meant* to be well-formed or
        // meant to be rejected for a recorded reason.
        if let Some(Err(why)) = round_trip(seed) {
            panic!("a seed does not round-trip:\n{seed}\n{why}");
        }
    }
    if let Some(msg) = golden(&golden_path("ein-ir", "fuzz_seeds.txt"), &out) {
        panic!("{msg}");
    }
}
