//! **Every kernel meta-primitive, at every arity.**
//!
//! A primitive whose arity nothing pins reaches the engine at any arity, and
//! the class is predicted by one rule:
//!
//! > **The grammar shape-pins the kernel meta-primitives.** `NotForm`,
//! > `NeqForm`, `AndForm` and `OrForm`, `eq`, `absent` and `false` are
//! > productions with a fixed or bounded arity
//! > ([`01-grammar.md` §2](../../../../docs/ein/user/04-reference/01-grammar.md#2-phrase-grammar)).
//!
//! A primitive left to a runtime `assert!` would panic on `(eq)` or `(eq ?x)`,
//! and one that read only its first arguments would retire `(absent)`'s rule
//! without a word and drop every argument of `(absent a b …)` or
//! `(eq a b c …)` past the ones it reads, firing a guard weaker than the one
//! written. Each of those cells is a parse error, positioned, and this file
//! asserts it cell by cell, beside the cells that fire or stay silent as a
//! reader expects. The shapes have `.expected` fixtures under
//! [`examples/broken/`](../../../../examples/broken/).
//!
//! It is a subprocess sweep rather than an in-process one because two of the
//! outcomes are **process** outcomes: an exit code of 101 and a panic message
//! on stderr are not things a library call can report.

use std::path::PathBuf;
use std::process::Command;

/// What one `(primitive …)` in a `:match` did to the program around it.
#[derive(Debug, PartialEq, Eq)]
enum Cell {
    /// A positioned `file:line:col: unexpected …` — the grammar refused it
    /// at the token it found.
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
/// Tagged per test, not per process: the tests below run as threads of one
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
        "(relation p T :why \"p holds of {{?3}}\")\n(relation q T :why \"q holds of {{?3}}\")\n\
         (rule r ()\n  :match (and (p ?x) {guard})\n  :assert (q ?x) :why \"p holds of {{?x}} and the guard beside it holds of {{?x}}, so q holds of {{?x}}\")\n(p A)\n"
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
        let at = format!("{}:", path.display());
        return if both
            .lines()
            .any(|l| l.starts_with(&at) && l.contains(": unexpected "))
        {
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
/// [`01-grammar.md` §2](../../../../docs/ein/user/04-reference/01-grammar.md#2-phrase-grammar)'s
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

/// **`eq` and `absent` are pinned by the grammar.**
///
/// *A reader expects* is what a reader of the program expects, and *now* what
/// this test asserts; *unpinned* is what a primitive left to the engine
/// would do:
///
/// | guard | a reader expects | unpinned | now |
/// |---|---|---|---|
/// | `(eq)` · `(eq ?x)` | a diagnostic | **panic**, exit 101 | a parse error |
/// | `(eq ?x A)`, `A` bound to `?x` | fires | fires | fires |
/// | `(eq ?x A B)` with `A ≠ B` | a diagnostic, or silence | **fires** — everything past the second argument dropped | a parse error |
/// | `(absent)` | a diagnostic — `(absent ?x)` gets one | **silent**: the rule can never fire, and nothing says so | a parse error |
/// | `(absent (q ?x))`, `q` empty | fires | fires | fires |
/// | `(absent (q ?x) (p ?x))` with `p` non-empty | a diagnostic, or silence | **fires** — everything past the first argument dropped | a parse error |
///
/// The paper grammar states each primitive's shape (`eq` takes two terms,
/// `absent` one premise), so no program reaches a compiler refusal of these
/// shapes.
///
/// The dropped-argument rows are the worse pair: a panic is loud, and a guard
/// that quietly evaluates a weaker condition than the one written is a wrong
/// answer with a success exit code.
///
/// `false` takes no argument and the grammar shapes it too (Q-M1e.18): a
/// premise is `(false)`, and `(false …)` with an argument is a parse error.
/// This sweep does not take it.
#[test]
fn the_unpinned_primitives_are_refused_at_a_wrong_arity() {
    let s = Scratch::new("unpinned");
    let cases: [(&str, Cell); 11] = [
        // Below its arity: a parse error, where a runtime `assert!` would
        // take the process down.
        ("(eq)", Cell::ParseError),
        ("(eq ?x)", Cell::ParseError),
        // The control: at its own arity the guard works, in both directions.
        ("(eq ?x A)", Cell::Fires),
        ("(eq ?x B)", Cell::Silent),
        // …and above it, the tail was dropped. `A ≠ B`, and the rule fired.
        ("(eq ?x A B)", Cell::ParseError),
        ("(eq ?x A B C)", Cell::ParseError),
        // `(absent ?x)` is a CompileError naming this exact hazard — "the
        // guard would fail against every KB". With no argument at all there
        // was no diagnostic and the rule was retired in silence.
        ("(absent)", Cell::ParseError),
        // The control, both directions.
        ("(absent (q ?x))", Cell::Fires),
        ("(absent (p ?x))", Cell::Silent),
        // …and past the first argument, dropped: `(p A)` is present, so a
        // guard reading both of these must not pass. It did.
        ("(absent (q ?x) (p ?x))", Cell::ParseError),
        ("(absent (q ?x) (p ?x) (p ?x))", Cell::ParseError),
    ];
    for (i, (guard, want)) in cases.iter().enumerate() {
        assert_eq!(
            &cell(&s, 100 + i, guard),
            want,
            "{guard} — if this cell moved, say so in the commit rather than \
             re-blessing it"
        );
    }
}

/// **A refused cell says where.** `corpus_cli::every_refusal_carries_a_diagnostic` is a rule
/// the repo enforces on refusals, and a panic is a refusal that carries a
/// stack trace instead.
///
/// The four shapes have `.expected` fixtures under `examples/broken/`, which
/// is where the message is pinned byte for byte. What is pinned *here* is
/// the part a fixture cannot see: that the diagnostic reaches **stderr** of a
/// real process with a non-zero exit, and that it says where: each is a
/// parse error, `file:line:col: unexpected …`, at the token where the shape
/// stops being the primitive's.
#[test]
fn a_refused_arity_names_its_position() {
    let s = Scratch::new("positioned");
    for (i, (guard, col, found)) in [
        ("(eq ?x)", 28, "`)`"),
        ("(eq ?x A B)", 31, "`B`: a name"),
        ("(absent)", 29, "`)`"),
        ("(absent (q ?x) (p ?x))", 37, "`(`"),
    ]
    .iter()
    .enumerate()
    {
        let src = format!(
            "(relation p T)\n(relation q T)\n\
             (rule r ()\n  :match (and (p ?x) {guard})\n  :assert (q ?x))\n(p A)\n"
        );
        let path = s.0.join(format!("pos{i}.ein"));
        std::fs::write(&path, src).expect("the cell is written");
        let out = Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(["saturate", &path.display().to_string()])
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(1), "{guard}: {stderr}");
        let want = format!("{}:4:{col}: unexpected {found}", path.display());
        assert!(
            stderr.lines().any(|l| l == want),
            "{guard}: the diagnostic does not say where ({want}): {stderr}"
        );
    }
}
