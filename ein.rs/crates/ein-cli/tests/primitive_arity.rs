//! **Every kernel meta-primitive, at every arity** — M1e
//! [S1e.1.6](../../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.6_coverage_gaps.md)
//! T1e.1.6.2, the review's `Q9`.
//!
//! The review found one process panic from a well-formed program —
//! `(eq ?x)` — and had no dedicated pass over the surface it came from. This
//! is that pass, and it found the rule that predicts the whole class:
//!
//! > **The grammar shape-pins four of the seven kernel meta-primitives and
//! > leaves three unpinned.** `NotForm`, `NeqForm`, `AndForm` and `OrForm` are
//! > productions with a fixed or bounded arity
//! > ([`00_ebnf.md` §2](../../../../docs/kernel/ir/03-ein-lang/00_ebnf.md));
//! > `eq`, `absent` and `false` are ordinary `GenericList`s, and the only
//! > thing between them and the engine is a runtime `assert!`.
//!
//! Every cell that panics or silently misbehaves is one of the unpinned
//! three; every cell of the pinned four is a positioned parse error. That is
//! not a coincidence to be re-derived — it is what these two tests hold.
//!
//! **This pins today's behaviour, including the wrong parts.** Seven of the
//! twenty-one cells are defects — two panics and **five silent** ones — and
//! all seven are asserted as they are, so a fix has to come through here:
//! `(absent)` silently retires its rule where `(absent ?x)` is a *CompileError*
//! saying exactly that, and `(absent a b …)` / `(eq a b c …)` drop every
//! argument past the ones they read — firing a guard weaker than the one
//! written, with no diagnostic. The two `(eq)` panics are
//! [`CO-H1`](../../../../plans/m1e_review_processing/README.md), whose fix is
//! [S1e.2.1](../../../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md)'s.
//!
//! It is a subprocess sweep rather than an in-process one because two of the
//! outcomes are **process** outcomes: an exit code of 101 and a panic message
//! on stderr are not things a library call can report.

use std::path::PathBuf;
use std::process::Command;

/// What one `(primitive …)` in a `:match` did to the program around it.
#[derive(Debug, PartialEq, Eq)]
enum Cell {
    /// A positioned `file:line:col: unexpected input` — the grammar refused it.
    ParseError,
    /// A load or compile diagnostic, exit 1 — refused later, but refused.
    Refused,
    /// `thread 'main' panicked`, exit 101.
    Panic,
    /// Accepted, and the rule concluded.
    Fires,
    /// Accepted, and the rule never fired.
    Silent,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// A directory one test owns and deletes.
///
/// Tagged per test, not per process: the two tests below run as threads of one
/// binary, and an untagged directory means the second `new()` deletes the
/// first test's files out from under it — which reads as a *refusal* in a
/// sweep whose whole subject is what gets refused.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let dir =
            std::env::temp_dir().join(format!("ein-primitive-arity-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Scratch(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// One rule, one guard, one fact — and `(q A)` iff the rule fired.
///
/// `saturate` rather than `solve` on purpose: `q` is a declared relation, so
/// the blind enumerator would *guess* `(q A)` and every silent cell would read
/// as a firing one. The deductive closure has no such second source.
fn cell(scratch: &Scratch, n: usize, guard: &str) -> Cell {
    let src = format!(
        "(relation p T)\n(relation q T)\n\
         (rule r ()\n  :match (and (p ?x) {guard})\n  :assert (q ?x))\n(p A)\n"
    );
    let path = scratch.0.join(format!("cell{n}.ein"));
    std::fs::write(&path, src).expect("the cell is written");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["saturate", &path.display().to_string(), "--dump"])
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let (stdout, stderr) = (
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let both = format!("{stderr}{stdout}");
    if both.contains("panicked at") {
        return Cell::Panic;
    }
    if !out.status.success() {
        return if both.contains("unexpected input") {
            Cell::ParseError
        } else {
            Cell::Refused
        };
    }
    if stdout.contains("(q A :rule r)") {
        Cell::Fires
    } else {
        Cell::Silent
    }
}

/// **The four the grammar pins refuse a wrong arity, positioned.**
///
/// `not`, `and`, `or` and `neq` are productions in
/// [`00_ebnf.md` §2](../../../../docs/kernel/ir/03-ein-lang/00_ebnf.md)'s
/// *Kernel meta-primitives (shape-pinned)* block, so a wrong arity is a
/// **parse** error with a line and a column — the cheapest diagnostic the
/// engine has, and the one a generated program most needs.
///
/// `or` is the exception that proves the block is about *shape*: `(or …)` at
/// any arity ≥ 1 parses and is then refused by the compiler, because a nested
/// `(or …)` in a premise is a semantic refusal rather than a shape one. It is
/// still a diagnostic and still exit 1, which is all this test claims.
#[test]
fn the_shape_pinned_primitives_refuse_a_wrong_arity() {
    let s = Scratch::new("pinned");
    let cases: [(&str, Cell); 10] = [
        ("(not)", Cell::ParseError),
        ("(not (p ?x) (p ?x))", Cell::ParseError),
        ("(not (p ?x) (p ?x) (p ?x))", Cell::ParseError),
        ("(and)", Cell::ParseError),
        ("(neq)", Cell::ParseError),
        ("(neq ?x)", Cell::ParseError),
        ("(neq ?x A B)", Cell::ParseError),
        ("(neq ?x A B C)", Cell::ParseError),
        ("(or)", Cell::ParseError),
        ("(or (p ?x))", Cell::Refused),
    ];
    for (i, (guard, want)) in cases.iter().enumerate() {
        assert_eq!(
            &cell(&s, i, guard),
            want,
            "{guard} — a pinned primitive at a wrong arity must be refused, \
             and this is the property that makes the unpinned three a class"
        );
    }
}

/// **The three the grammar does not pin do something else, and seven cells of
/// it are wrong — two loudly and five in silence.**
///
/// Read the table as the sweep produced it — *want* is what a reader of the
/// program would expect, *got* is what this test asserts:
///
/// | guard | a reader expects | today |
/// |---|---|---|
/// | `(eq)` · `(eq ?x)` | a diagnostic | **panic**, exit 101 — `CO-H1` |
/// | `(eq ?x A)`, `A` bound to `?x` | fires | fires |
/// | `(eq ?x A B)` with `A ≠ B` | a diagnostic, or silence | **fires** — everything past the second argument is dropped |
/// | `(absent)` | a diagnostic — `(absent ?x)` gets one | **silent**: the rule can never fire, and nothing says so |
/// | `(absent (q ?x))`, `q` empty | fires | fires |
/// | `(absent (q ?x) (p ?x))` with `p` non-empty | a diagnostic, or silence | **fires** — everything past the first argument is dropped |
/// | `(false …)` in a `:match` | silence | silence, at every arity |
///
/// The two `(eq)` rows are the review's finding. The two dropped-argument rows
/// are this sweep's, and they are the worse pair: a panic is loud, and a guard
/// that quietly evaluates a weaker condition than the one written is a wrong
/// answer with a success exit code.
#[test]
fn the_unpinned_primitives_drop_arguments_and_two_of_them_panic() {
    let s = Scratch::new("unpinned");
    let cases: [(&str, Cell); 11] = [
        // CO-H1: below its arity, the runtime assertion is the only check.
        ("(eq)", Cell::Panic),
        ("(eq ?x)", Cell::Panic),
        // The control: at its own arity the guard works, in both directions.
        ("(eq ?x A)", Cell::Fires),
        ("(eq ?x B)", Cell::Silent),
        // …and above it, the tail is dropped. `A ≠ B`, and the rule fires.
        ("(eq ?x A B)", Cell::Fires),
        ("(eq ?x A B C)", Cell::Fires),
        // `(absent ?x)` is a CompileError naming this exact hazard — "the
        // guard would fail against every KB". With no argument at all there is
        // no diagnostic and the rule is retired in silence.
        ("(absent)", Cell::Silent),
        // The control, both directions.
        ("(absent (q ?x))", Cell::Fires),
        ("(absent (p ?x))", Cell::Silent),
        // …and past the first argument, dropped: `(p A)` is present, so a
        // guard reading both of these must not pass.
        ("(absent (q ?x) (p ?x))", Cell::Fires),
        ("(absent (q ?x) (p ?x) (p ?x))", Cell::Fires),
    ];
    for (i, (guard, want)) in cases.iter().enumerate() {
        assert_eq!(
            &cell(&s, 100 + i, guard),
            want,
            "{guard} — this test pins today's behaviour, defects included; if \
             this cell moved, say so in the commit rather than re-blessing it"
        );
    }
}
