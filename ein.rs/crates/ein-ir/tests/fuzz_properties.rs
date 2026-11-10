//! The frontend fuzzer, on **self-checkable properties** — T1a.1.1.4's
//! generator.
//!
//! The grammar admits every tokenization; recursive descent commits to one.
//! That is the one structural risk in the parser
//! ([design/04](../../../../docs/history/m1a_rust/design/04_ir_frontend.md) §1),
//! and the fuzzer probes it by generating malformed and nearly-well-formed
//! inputs. It is the generator, the minimiser, the seed corpus, and three
//! properties:
//!
//! 1. **It never panics.** Generated input either parses or is *diagnosed* —
//!    a recursive-descent parser that unwrapped an assumption would show up
//!    here as a crash rather than as a wrong answer.
//! 2. **What parses round-trips.** `dump_canonical → parse → dump_canonical`
//!    is a fixed point on every mutation that parses. This is the strongest
//!    self-checkable property the frontend has, and it is the one that
//!    catches a dumper/parser disagreement.
//! 3. **The recorded findings still parse the way they parsed.** Every seed
//!    and every checked-in `corpus/fuzz_findings/*.ein` against a table —
//!    which is what makes a find a regression test rather than a souvenir.
//! 4. **The generated parser's refusal is the same twice, within its budget**
//!    (M1f S1f.5.5 F11, T1f.5.5.3): generated input the generated parser
//!    refuses is parsed again and reports the same, byte for byte, and its
//!    recovery took at most `lrpar`'s step budget, all of it only where the
//!    report says it was cut.
//! 5. **No accepted program carries a keyword its form does not list** (M1f
//!    S1f.5.11, T1f.5.11.2): each keyword pair an accepted program holds is a
//!    form's own, on the form's list, and no term or nested fact holds one.
//! 6. **No firing meets an unbound variable** (M1f S1f.5.10, P1f.5's F13):
//!    every plan the compiler builds for an accepted program's rules and
//!    hrules reads, in each conclusion, only registers each disjunct binds,
//!    so a loader that lets one through shows here. It compiles with
//!    `ein-infer`, a dev-dependency since S1f.5.10.
//! 7. **The compiler refuses nothing the loader accepts** (M1f S1f.5.21, the
//!    close review's finding 1): every plan of an accepted program's rules,
//!    hrules and obligations, one per activator, and every query's goal
//!    compiles, but where a macro's body wrote a primitive at another arity
//!    (S1f.5.24's); a nested `(or …)`, a clause past the register file and a
//!    goal's relation variable are the loader's, and the compiler's arms for
//!    them are invariants, which a panic here would show.
//!
//! Budget is `EIN_FUZZ_ITERS` (default 2 000, which fits the per-commit tier);
//! the nightly tier raises it. `EIN_FUZZ_SEED` moves the stream. Both are
//! deterministic: the same pair reproduces the same run, which is what makes a
//! find replayable.

use std::path::PathBuf;

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::dump::{dump_canonical, dump_compact};
use ein_ir::{Ast, Node, NodeId, parse};

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
    // S1f.5.11's keywords (T1f.5.11.2): one a fact takes, and four no form
    // does, so the stream writes a pair beside a form's own and inside a term.
    ":source",
    ":where",
    ":cardinality",
    ":mode",
    ":k",
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
    // `.ein` only. Any text is a legal fuzz seed, but the seed corpus is a
    // checked-in table, and a table that moved when a README was edited would
    // be a table nobody trusts — so the seed list is a function of the
    // fixtures.
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
/// Every mutation either parses or comes back as a `ParseError` — there is
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
/// find a regression test. The table checks each against the answer it was
/// recorded with.
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

/// **The generated parser's refusal is the same twice, and within its budget**
/// (M1f S1f.5.5 F11, T1f.5.5.3).
///
/// Every generated input, through the generated parser twice in this one
/// process, whose `HashSet`s each draw their own hash keys: a refusal reports
/// the same errors and repairs, byte for byte, both times, which the
/// unpatched `lrpar` did not, since it ranked equal repairs in a set's order.
/// Recovery is bounded by a count of search steps rather than the clock, so
/// the property also holds under any load; and a refusal took at most
/// `RECOVERY_STEP_BUDGET` steps, all of them only where the report says it
/// was cut there. The hand-written parser's verdict is not compared: where
/// the two languages differ is T1f.5.5.9's to attribute.
///
/// It reads every fourth input of the stream, and at most 10 000 of it,
/// spread evenly: a refusal costs the generated parser milliseconds, most of
/// them lexing, and three in four of these inputs are refused, so the
/// nightly's 1 000 000 would take an hour a seed. 500 inputs per commit, in
/// about three seconds of a debug build, and 10 000 a seed in the nightly.
#[test]
fn a_generated_refusal_is_the_same_twice_and_within_its_budget() {
    use ein_ir::grammar::{self, RECOVERY_STEP_BUDGET};
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let stride = (iters / 10_000).max(4);
    let (mut read, mut refused) = (0usize, 0usize);
    for i in 0..iters {
        let text = generated(&seeds, &mut rng);
        if i % stride != 0 {
            continue;
        }
        read += 1;
        let once = grammar::parse(&mut Ast::new(), &text, None);
        let twice = grammar::parse(&mut Ast::new(), &text, None);
        match (once, twice) {
            (Ok(_), Ok(_)) => {}
            (Err(a), Err(b)) => {
                refused += 1;
                assert_eq!(
                    a.to_string(),
                    b.to_string(),
                    "iteration {i} (EIN_FUZZ_SEED={seed}) reported two ways:\n---\n{text}\n---"
                );
                assert!(
                    a.recovery_steps <= RECOVERY_STEP_BUDGET
                        && (!a.cut || a.recovery_steps == RECOVERY_STEP_BUDGET),
                    "iteration {i} (EIN_FUZZ_SEED={seed}): {} steps, cut {}:\n---\n{text}\n---",
                    a.recovery_steps,
                    a.cut
                );
            }
            _ => panic!(
                "iteration {i} (EIN_FUZZ_SEED={seed}) was refused once and accepted once:\n---\n{text}\n---"
            ),
        }
    }
    eprintln!("fuzz: the generated parser refused {refused} of {read} read, each the same twice");
    assert!(
        refused >= read / 20,
        "only {refused} of {read} inputs were refused, so the property is untested"
    );
}

/// The keywords a form takes: S1f.5.11's § The lists, as the user ruled them on
/// 2026-11-07 (Q-M1e2.28), less `:priority`, which S1f.5.15 removed before the
/// swap, and with a macro's `:why`, the user's ruling of 2026-11-08. A form is
/// named by its head: every declarator's word is reserved, so no fact is headed
/// by one, and `not` heads a stored negation, a fact. A trace takes none, and
/// [`STEP`] is what each of its steps takes.
fn listed(head: &str) -> &'static [&'static str] {
    match head {
        "relation" | "macro" => &["why"],
        "rule" | "hrule" => &["match", "assert", "why"],
        "query" => &["goal", "goal-text", "over", "expect"],
        "solve" => &["hrules", "hypothesis-relations", "no-hypothesis", "expect"],
        "config" => &["lattice-sanity-check", "record-alternative-justifications"],
        "import" => &["symbols", "as"],
        "trace" => &[],
        _ => &["source", "rule", "using"],
    }
}

/// What a trace's `step` takes.
const STEP: &[&str] = &["rule", "using", "derives", "source", "assumes"];

/// `node` holds no keyword and no keyword pair, at any depth: it is a term, a
/// nested fact, a name or a list, never a form's clause.
fn no_keyword_below(ast: &Ast, node: NodeId) -> Result<(), String> {
    match ast.node(node) {
        Node::KwPair { .. } | Node::Keyword(_) => Err(format!(
            "holds the keyword pair {} inside a term",
            dump_compact(ast, node)
        )),
        Node::SForm { head, args } => {
            no_keyword_below(ast, head)?;
            ast.args(args)
                .iter()
                .try_for_each(|&a| no_keyword_below(ast, a))
        }
        _ => Ok(()),
    }
}

/// `form`'s own keyword pairs are on `list`, and nothing below them or beside
/// them holds one; a trace's steps each carry [`STEP`]'s. Counts the pairs.
fn keywords_listed(
    ast: &Ast,
    form: NodeId,
    list: &[&str],
    pairs: &mut usize,
) -> Result<(), String> {
    let head = ast.head_name(form).unwrap_or("?");
    for &a in ast.form_args(form) {
        match ast.node(a) {
            Node::KwPair { key, value } => {
                *pairs += 1;
                let Node::Keyword(k) = ast.node(key) else {
                    return Err(format!("({head} …) holds a pair whose key is no keyword"));
                };
                if !list.contains(&ast.sym(k)) {
                    return Err(format!(
                        "({head} …) carries :{}, which it does not list (one of {list:?})",
                        ast.sym(k)
                    ));
                }
                no_keyword_below(ast, value)?;
            }
            _ if head == "trace" => {
                if ast.head_name(a) != Some("step") {
                    return Err(format!("(trace …) holds {}", dump_compact(ast, a)));
                }
                keywords_listed(ast, a, STEP, pairs)?;
            }
            _ => no_keyword_below(ast, a)?,
        }
    }
    Ok(())
}

/// **No accepted program carries a keyword its form does not list** (M1f
/// S1f.5.11, T1f.5.11.2).
///
/// The user, 2026-11-07: *"there are fixed set of allowed kwargs, if parser sees
/// :k or :G :notes or whatever else unknown kwarg - it is an error"*. Each form
/// takes the keywords [`listed`] names, as its tokens in `ein.y`, and a term or
/// a nested fact takes none, so every program the parser accepts holds keyword
/// pairs only as a form's own clauses, each on its form's list. Until the swap
/// only `query`, `solve` and `config` refused one they did not know; a pair in a
/// `:match` or a macro call was dropped (Q32), and S1f.5.11's T1f.5.11.2
/// retired the drops. Every input of the stream the parser accepts is read, and
/// the pairs it holds are counted, so a stream that carried few would fail here
/// rather than pass empty: at least one pair for four accepted programs, where
/// the per-commit stream carries 261 in 442 (`cargo test -p ein-ir --test
/// fuzz_properties`, 2026-11-09). The generator splices `:source`, `:where`,
/// `:cardinality`, `:mode` and `:k` since this property came, so the refused
/// side of the stream writes a pair inside a term too.
#[test]
fn no_accepted_program_carries_a_keyword_its_form_does_not_list() {
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let (mut accepted, mut pairs) = (0usize, 0usize);
    for i in 0..iters {
        let text = generated(&seeds, &mut rng);
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, None) else {
            continue;
        };
        accepted += 1;
        for &form in &forms {
            let list = listed(ast.head_name(form).unwrap_or(""));
            if let Err(why) = keywords_listed(&ast, form, list, &mut pairs) {
                panic!("iteration {i} (EIN_FUZZ_SEED={seed}): {why}:\n---\n{text}\n---");
            }
        }
    }
    eprintln!(
        "fuzz: {pairs} keyword pairs in {accepted} accepted programs, each on its form's list"
    );
    assert!(
        pairs >= accepted / 4,
        "{pairs} keyword pairs in {accepted} accepted programs, so the property is barely tested"
    );
}

/// The first conclusion variable a plan of `text`'s rules or hrules reads
/// that one of its disjuncts leaves unbound, as `rule 'r': ?y`; `None` where
/// `text` does not load, or every plan binds what its conclusions read.
///
/// A disjunct binds the activator's seed (the registers before the first free
/// variable's, `ein_infer::plan`'s register spaces), every register a relation
/// premise's slots hold at any depth, and every register an `eq` or `neq`
/// guard reads, which it binds where nothing else does (M1f S1f.5.7). An
/// `(absent …)`, lifted or nested, binds nothing outside it. A rule with
/// parameters is compiled only from an activator, which seeds every one, a
/// nested fact too since S1f.5.18, so a conclusion reads a parameter as a
/// constant of the plan; one no activator activates compiles no plan. **A
/// parameter was excepted until S1f.5.18**, which seeded a nested fact (W33's
/// cause; W58 was the stop it made where only the conclusion read the
/// parameter), and until then the plan of a rule with parameters and no
/// activator was compiled here with none, its parameters unbound.
fn unbound_conclusion(text: &str) -> Option<String> {
    use ein_infer::plan::{GuardArgKind, Plan, Slot, Step};
    fn regs(plan: &Plan, slot: Slot, out: &mut Vec<u16>) {
        match slot {
            Slot::Reg(r) => out.push(r),
            Slot::Nested { slots, .. } => {
                for &s in plan.slots(slots) {
                    regs(plan, s, out);
                }
            }
            Slot::Const(_) | Slot::Opaque(_) => {}
        }
    }
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, None).ok()?;
    let mut terms = ein_core::Terms::new();
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).ok()?;
    let program = kb.program();
    let rules: Vec<ein_core::Rule> = program
        .rules
        .values()
        .chain(program.hrules.values())
        .cloned()
        .collect();
    for rule in &rules {
        for activator in ein_infer::activators_for(&kb, &terms, rule) {
            // A plan the compiler refuses is its own diagnosed outcome.
            let Ok(plan) = ein_infer::compile_rule(&ast, &mut terms, rule, activator) else {
                continue;
            };
            let mut read: Vec<u16> = Vec::new();
            for &slot in plan.asserts.iter() {
                regs(&plan, slot, &mut read);
            }
            for d in plan.disjuncts.iter() {
                let mut bound: Vec<u16> = (0..plan.seed.len() as u16).collect();
                for step in plan.steps(d.steps) {
                    match *step {
                        Step::Rel(r) => {
                            for &s in plan.slots(r.slots) {
                                regs(&plan, s, &mut bound);
                            }
                        }
                        Step::Guard { args, .. } => {
                            for a in plan.guard_args(args) {
                                if let GuardArgKind::Reg(r) = a.kind {
                                    bound.push(r);
                                }
                            }
                        }
                        Step::Absent { .. } => {}
                    }
                }
                for &r in &read {
                    let name = plan.reg_names[r as usize];
                    if !bound.contains(&r) {
                        return Some(format!(
                            "rule '{}': ?{}",
                            terms.sym(rule.name),
                            terms.sym(name)
                        ));
                    }
                }
            }
        }
    }
    None
}

/// **No firing meets an unbound variable** (M1f S1f.5.10, P1f.5's F13).
///
/// The loader refuses a rule or an hrule whose conclusion writes a variable
/// some way of satisfying its `:match` leaves unbound (the user's ruling of
/// 2026-11-07, *"yes, file F13 with the refusal"*). So, for every generated
/// program the loader accepts, every plan the compiler builds for its rules
/// and hrules reads, in each conclusion, only registers each disjunct binds
/// ([`unbound_conclusion`]). A firing builds its conclusion from the registers
/// a match through one disjunct bound, so this holds every firing at once: an
/// hrule's, whose template built no fact and proposed nothing in silence
/// until S1f.5.10, and a rule's that no state fires among them. It holds the
/// loader's reading of what binds to the compiler's (Q-M1f.40, taken by
/// default on the user's instruction of 2026-11-09). Until S1f.5.10 the loader
/// did not check it, and the run stopped at the first firing that met one,
/// `ein_infer::firing::FireError::UnboundVar`.
///
/// The programs of the stream the loader refuses with the line are counted,
/// as are those it accepts, so a stream that reached neither would say so.
#[test]
fn no_firing_meets_an_unbound_variable() {
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let (mut loaded, mut refused) = (0usize, 0usize);
    for i in 0..iters {
        let text = generated(&seeds, &mut rng);
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, None) else {
            continue;
        };
        match ein_ir::load(&mut ast, &mut ein_core::Terms::new(), &forms, None) {
            Ok(_) => loaded += 1,
            Err(e) if e.0.contains("which no premise or parameter binds") => refused += 1,
            Err(_) => continue,
        }
        if let Some(why) = unbound_conclusion(&text) {
            let small = minimise(&text, |t| unbound_conclusion(t).is_some());
            panic!(
                "iteration {i} (EIN_FUZZ_SEED={seed}): {why} reaches a conclusion unbound; \
                 minimised:\n---\n{small}\n---"
            );
        }
    }
    eprintln!(
        "fuzz: {loaded} programs loaded, every conclusion bound; {refused} refused for a \
         conclusion variable nothing binds"
    );
    assert!(
        loaded >= iters / 20,
        "only {loaded} of {iters} programs loaded, so the property is barely tested"
    );
}

/// The first refusal the compiler makes of a plan of `text`, which the loader
/// accepts, as `rule 'r': …`: every rule's, hrule's and obligation's plan, one
/// per activator as the engine compiles them, and every query's goal. `None`
/// where `text` does not load, or every plan compiles, or the one refusal is a
/// macro body's primitive at another arity, which the compiler is the first to
/// read (S1f.5.24's, Q-M1f.67).
fn compile_refusal(text: &str) -> Option<String> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, None).ok()?;
    let mut terms = ein_core::Terms::new();
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).ok()?;
    let program = kb.program();
    let rules: Vec<ein_core::Rule> = program
        .rules
        .values()
        .chain(program.hrules.values())
        .chain(program.obligations.values())
        .cloned()
        .collect();
    let mut refusals: Vec<String> = Vec::new();
    for rule in &rules {
        for activator in ein_infer::activators_for(&kb, &terms, rule) {
            if let Err(e) = ein_infer::compile_rule(&ast, &mut terms, rule, activator) {
                refusals.push(e.0);
            }
        }
    }
    let goals: Vec<NodeId> = program
        .queries
        .iter()
        .filter_map(|q| ein_infer::query_value(&ast, q, "goal"))
        .collect();
    for goal in goals {
        if let Some(e) = ein_infer::verdict::goal_plan_error(&ast, &mut terms, &kb, Some(goal)) {
            refusals.push(e.0);
        }
    }
    refusals
        .into_iter()
        .find(|r| !r.contains(" takes exactly "))
}

/// **The compiler refuses nothing the loader accepts** (M1f S1f.5.21, the
/// close review's finding 1).
///
/// A refusal the compiler made a program reach was met only where a plan was
/// compiled, so a command that never compiled it answered what one that did
/// stopped on; and each was printed in the Python IR's spelling. The loader
/// refuses each since (a nested `(or …)`, W62; a clause past the register
/// file, W63; a goal's relation variable, W64), so for every generated program
/// it accepts, every plan compiles ([`compile_refusal`]), but a macro body's
/// primitive at another arity, which is S1f.5.24's. The compiler's arms for
/// the three are invariants since, so one the loader let through would panic
/// here.
#[test]
fn the_compiler_refuses_nothing_the_loader_accepts() {
    let (iters, seed) = budget();
    let mut rng = Rng(seed | 1);
    let seeds = seeds();
    let mut loaded = 0usize;
    for i in 0..iters {
        let text = generated(&seeds, &mut rng);
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, None) else {
            continue;
        };
        if ein_ir::load(&mut ast, &mut ein_core::Terms::new(), &forms, None).is_err() {
            continue;
        }
        loaded += 1;
        if let Some(why) = compile_refusal(&text) {
            let small = minimise(&text, |t| compile_refusal(t).is_some());
            panic!(
                "iteration {i} (EIN_FUZZ_SEED={seed}): the compiler refuses a program the loader \
                 accepts, {why}; minimised:\n---\n{small}\n---"
            );
        }
    }
    eprintln!("fuzz: {loaded} programs loaded, every plan compiled");
    assert!(
        loaded >= iters / 20,
        "only {loaded} of {iters} programs loaded, so the property is barely tested"
    );
}
