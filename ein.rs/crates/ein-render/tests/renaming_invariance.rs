//! **The kernel reads no theory's name.**
//!
//! The engine expects the minimal shape of a program, its relations and
//! rules, and gives a fixed meaning to its own names alone, those
//! `ein_core::terms::Kernel` holds (`Terms::new` interns exactly them), and
//! to the three a keyword takes, `:over`'s (`ein_ir::expect::Over::NAMES`). Every
//! other name, `is-a`, `symmetric`, `bijective`, `slot-partition` and the
//! program's own, is a theory's or a program's, and means what the rules that
//! read it make it mean. So a program with every such name renamed is the same
//! program: it answers the same, its claims hold or fail the same, and it is
//! drawn the same, apart from the names.
//!
//! This holds it over the corpus. Each program is run twice: as written, and
//! with **every name that is not the kernel's renamed**, by one bijection
//! onto fresh names for the run, in its own text, in every local module it
//! imports and in the whole standard library, written once renamed and read
//! by the renamed programs' resolver, so a library's rule and the name a
//! program writes of it are renamed alike. The renaming goes through the
//! lexer every parse goes through (`ein_ir::grammar::name_spans`): a `NAME`
//! token is renamed, and a word of the grammar (`relation`, `model`, `step`,
//! …), a keyword, a variable, a string and a comment are not, being no name.
//! A module's name and an import's `:as` stay, being where a file is found
//! rather than what a program says; a name under them is renamed.
//!
//! **The bijection** appends [`SUFFIX`] to a name. It is order-preserving:
//! the suffix opens with `!`, the least character a name may hold, and no
//! name of the corpus holds one, so a name before another, the kernel's
//! among them, stays before it, and every list the engine sorts by name keeps
//! its order. A program that writes a name with a `!` cannot be held, and the
//! test names it.
//!
//! **What is compared**, for each program, each run on a load of its own:
//! the loaded KB's shape, root saturation's event stream, the plans, the
//! match, the NAF map, the explanation, the frontend's parse and compact dump,
//! every DOT view, and the answer under describe as documents
//! (`corpus_ops::answer`: the summary's blocks, the claims' verdicts among
//! them, `nogoods.json` and `nodes.json`). The renamed run's text has the
//! suffix taken out and the scratch paths put back, and then both are read the
//! same way: a DOT node id hashed from a name (`ein_render::dot_util::hashed_id`)
//! is numbered by where it first appears, a colour hashed from a name
//! (`ein_render::palette::hash_color`) is one colour, the column of a place,
//! `LINE:COL`, is masked, and a run of whitespace is one space, since a
//! printer breaks a line at a width a longer name moves. The read-out's text is
//! not compared, for the same reason; its content is the summary's.
//!
//! **A drawing that still reads a theory's name** is listed in [`READS`], with
//! the name it reads: it is not compared, and a listed view that no longer
//! moves on any program fails the run, so the list says what is left and no
//! more.
//!
//! **Which programs**: every corpus program outside `tests/`, whose programs
//! are each one standard-library rule's claim and are renamed with the
//! library in the nightly; under `EIN_CORPUS_SLOW=1` every corpus program;
//! either way this run's shard of them (`EIN_TEST_SHARD`). Each runs within the
//! sweep's budget and cube limit (`corpus_ops::SWEEP_BUDGET`, `SWEEP_LIMIT`),
//! under the default `--region-budget`, one program a unit of the pool.
//!
//! ```text
//! cargo test -p ein-render --test renaming_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-render --test renaming_invariance
//! ```

mod corpus_ops;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use corpus_ops::Op;
use ein_core::{Sources, Symbol, Terms};
use ein_corpus::{Scratch, corpus_files, pool, repo_root, shard, under_tests};
use ein_ir::grammar::{name_spans, opens_saved, parse};
use ein_ir::imports::Resolver;
use ein_ir::stdlib::{MARKER, Source};
use ein_render::shape::all_views;

/// What the bijection appends to a name: `!` first, the least character of a
/// name, so the renaming keeps every order by name.
const SUFFIX: &str = "!~";

/// The views that still read a theory's name, each with the name it reads
/// and where: not compared, and each must still move on some program. None
/// does: no view draws a relation otherwise than another (Q-M1f.421), and
/// none reads `is-a`, `type`, `instance`, `relation` or `std.macro`'s
/// `forall` by its name, so every view is compared.
const READS: &[(&str, &str)] = &[];

/// The surfaces compared, beside the answer ([`corpus_ops::answer`]). The
/// frontend's `resolve` and `expand` read the checkout's standard library by
/// their own resolver, and the read-out's text is the summary's in a layout;
/// every other op of [`corpus_ops::ops`] that renders no search is here.
fn surfaces() -> Vec<Op> {
    let mut out = vec![
        Op::Ir("parse"),
        Op::Ir("dump-compact"),
        Op::Load,
        Op::Saturate,
        Op::Plan,
        Op::Match,
        Op::Naf,
        Op::Explain(true),
        Op::Explain(false),
    ];
    out.extend(all_views().into_iter().map(Op::Dot));
    out
}

/// The language's own names: the kernel's, what `Terms::new` interns
/// (`ein_core::terms::Kernel`), and the names a keyword takes, `:over`'s
/// (`ein_ir::expect::Over::NAMES`) and `:ein`'s, the engine's words
/// (`ein_core::EngineWord`), which the loader reads in that one place as it
/// reads a keyword.
fn kernel_names() -> BTreeSet<String> {
    let terms = Terms::new();
    (0..terms.syms.len() as u32)
        .map(|i| terms.syms.text(Symbol(i)).to_string())
        .chain(ein_ir::expect::Over::NAMES.map(str::to_string))
        .chain(ein_core::EngineWord::ALL.map(|w| w.as_str().to_string()))
        .collect()
}

/// The `NAME` token after `head` in `text`, each: the module an `(import …)`
/// or a saved file's `(module …)` names, and the alias an `:as` gives.
fn names_after(text: &str, head: &str) -> Vec<String> {
    name_spans(text)
        .into_iter()
        .filter(|s| text[..s.start].trim_end().ends_with(head))
        .map(|s| text[s].to_string())
        .collect()
}

/// The bijection, one for the run: every program and every module renamed
/// alike, so a library's rule and the name a program writes of it stay one
/// name.
struct Renaming {
    /// Kept: the kernel's names, every module's name and every alias, read
    /// off the corpus and the standard library.
    kept: BTreeSet<String>,
}

impl Renaming {
    fn of(kernel: &BTreeSet<String>, texts: &[String], modules: &[String]) -> Renaming {
        let mut kept = kernel.clone();
        kept.extend(modules.iter().cloned());
        for t in texts {
            for head in ["(import", "(module", ":as"] {
                kept.extend(names_after(t, head));
            }
        }
        Renaming { kept }
    }

    /// `text` with every name renamed but the kept ones; `None` where a name
    /// already holds the suffix's first character, which the order rests on.
    fn text(&self, text: &str) -> Result<String, String> {
        let mut out = String::with_capacity(text.len() + text.len() / 8);
        let mut last = 0;
        for s in name_spans(text) {
            let name = &text[s.clone()];
            if name.contains('!') {
                return Err(format!(
                    "the name {name} holds a `!`, which the bijection's order rests on"
                ));
            }
            if self.kept.contains(name) {
                continue;
            }
            out.push_str(&text[last..s.end]);
            out.push_str(SUFFIX);
            last = s.end;
        }
        out.push_str(&text[last..]);
        Ok(out)
    }
}

/// One spelling of a program: its path, the directory its imports are found
/// from, and the standard library its resolver reads.
struct Spelling {
    path: PathBuf,
    stdlib: Source,
}

/// The program at `s` loaded as every command loads it (`ein_ir::saved`'s
/// `load_text`), but with the standard library `s.stdlib`: the forms, the AST
/// and `Terms`, and the KB or the load's refusal. `None` where it does not
/// parse.
fn load(
    s: &Spelling,
) -> Option<(
    ein_ir::Ast,
    Terms,
    Result<ein_core::Kb, ein_ir::KbLoadError>,
)> {
    let text = std::fs::read_to_string(&s.path).ok()?;
    let mut ast = ein_ir::Ast::new();
    let mut terms = Terms::new();
    let name = s.path.to_str();
    if opens_saved(&text) {
        // A saved file reads no other file.
        let opts = ein_ir::LoadOptions::default();
        let kb = match ein_ir::saved::load_text(&mut ast, &mut terms, &text, name, None, 0, &opts) {
            Ok(l) => Ok(l.kb),
            Err(ein_ir::saved::TextError::Parse(_)) => return None,
            Err(ein_ir::saved::TextError::Load(e)) => Err(e),
        };
        return Some((ast, terms, kb));
    }
    let forms = parse(&mut ast, &text, name).ok()?;
    let resolver = Resolver::with_stdlib(s.stdlib.clone());
    let kb = ein_ir::from_ir::load_query_by(
        &resolver,
        &mut ast,
        &mut terms,
        &forms,
        s.path.parent(),
        0,
        &ein_ir::LoadOptions::default(),
    )
    .map(|mut kb| {
        kb.program_mut().sources = Some(Arc::new(Sources {
            program: text.clone(),
            modules: resolver.modules_read(),
        }));
        kb
    });
    Some((ast, terms, kb))
}

/// Every surface of one spelling, by name, the answer last.
fn rendered(s: &Spelling, ops: &[Op]) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let loaded = load(s);
    for &op in ops {
        let text = if !op.loads() {
            corpus_ops::run_unloaded(&s.path, op)
        } else {
            loaded.as_ref().and_then(|(ast, terms, kb)| {
                let mut terms = terms.clone();
                corpus_ops::run_loaded(ast, &mut terms, kb.clone(), &s.path, op, 1)
            })
        };
        out.push((op.to_string(), text));
    }
    let answer = loaded.and_then(|(ast, mut terms, kb)| {
        let mut kb = kb.ok()?;
        Some(
            match corpus_ops::answer(&ast, &mut terms, &mut kb, &s.path) {
                Ok(t) => t,
                Err(e) => format!("<refused> {e}"),
            },
        )
    });
    out.push(("answer".to_string(), answer));
    out
}

/// Both spellings read one way: the renamed one's suffix out and its scratch
/// paths put back; then, on both, a hashed DOT id numbered by where it first
/// appears, a hashed colour one colour, a column of a place in a text masked,
/// and a run of whitespace one space, since a printer breaks a line at a width.
fn normalised(text: &str, back: Option<&[(String, String)]>) -> String {
    let mut t = text.to_string();
    if let Some(paths) = back {
        t = t.replace(SUFFIX, "");
        for (from, to) in paths {
            t = t.replace(from.as_str(), to.as_str());
        }
    }
    let mut out = String::with_capacity(t.len());
    let mut ids: Vec<String> = Vec::new();
    let b = t.as_bytes();
    let mut i = 0;
    let hex = |c: u8| c.is_ascii_digit() || (b'a'..=b'f').contains(&c);
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    while i < b.len() {
        // `#rrggbb`: a colour.
        if b[i] == b'#'
            && i + 7 <= b.len()
            && b[i + 1..i + 7].iter().all(|&c| c.is_ascii_hexdigit())
            && (i + 7 == b.len() || !word(b[i + 7]))
        {
            out.push_str("#colour");
            i += 7;
            continue;
        }
        // `LINE:COL`, a place in a text: a column moves with the length of
        // the names before it on its line, a line does not.
        if b[i].is_ascii_digit() && (i == 0 || !b[i - 1].is_ascii_digit()) {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            let mut k = j + 1;
            while k < b.len() && b[k].is_ascii_digit() {
                k += 1;
            }
            if j < b.len() && b[j] == b':' && k > j + 1 {
                out.push_str(&t[i..j]);
                out.push_str(":<col>");
                i = k;
                continue;
            }
            out.push_str(&t[i..j]);
            i = j;
            continue;
        }
        // A word that ends in ten hex digits: a hashed id.
        if word(b[i]) && (i == 0 || !word(b[i - 1])) {
            let mut j = i;
            while j < b.len() && word(b[j]) {
                j += 1;
            }
            let w = &t[i..j];
            if w.len() >= 10 && w.as_bytes()[w.len() - 10..].iter().all(|&c| hex(c)) {
                let n = ids.iter().position(|x| x == w).unwrap_or_else(|| {
                    ids.push(w.to_string());
                    ids.len() - 1
                });
                out.push_str(&format!("{}<id {n}>", &w[..w.len() - 10]));
            } else {
                out.push_str(w);
            }
            i = j;
            continue;
        }
        // Whitespace, a line's end among it: a printer breaks a line at a
        // width, which a longer name moves.
        if b[i].is_ascii_whitespace() {
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push(' ');
            continue;
        }
        let c = t[i..].chars().next().expect("a char");
        out.push(c);
        i += c.len_utf8();
    }
    out
}

/// What one program came to: each surface that differs, `(surface, the
/// first line that differs)`, or why it cannot be held.
enum Held {
    Same,
    Moved(Vec<(String, String)>),
    Cannot(String),
}

/// The standard library renamed, written once under `dir`: each module at
/// its own path, beside the marker that makes the directory a standard
/// library. What each renamed program's resolver reads, the modules it does
/// not import among them, as `std.macro`, whose macros' names the loader reads
/// to tell a call of one it did not import.
fn renamed_stdlib(renaming: &Renaming, dir: &Path) -> Result<(), String> {
    let src = repo_root().join("stdlib");
    std::fs::write(dir.join(MARKER), "").map_err(|e| e.to_string())?;
    for f in ein_corpus::ein_files_under(&src) {
        let text = std::fs::read_to_string(&f).map_err(|e| e.to_string())?;
        let to = dir.join(f.strip_prefix(&src).expect("under the stdlib"));
        std::fs::create_dir_all(to.parent().expect("a directory")).map_err(|e| e.to_string())?;
        std::fs::write(&to, renaming.text(&text)?).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// The renamed spelling of `path`, written under `scratch`: the program at
/// its own name and each local module it imports, its own imports' too, at
/// the path its name gives from the program's directory; the standard library
/// is `stdlib`'s, renamed once. And the pairs that put the scratch paths back.
fn renamed(
    path: &Path,
    renaming: &Renaming,
    stdlib: &Path,
    scratch: &Path,
) -> Result<(Spelling, Vec<(String, String)>), String> {
    let root = repo_root();
    let dir = path.parent().expect("a file's directory");
    let prog_dir = scratch.join("program");
    std::fs::create_dir_all(&prog_dir).map_err(|e| e.to_string())?;
    let at = prog_dir.join(path.file_name().expect("a file name"));
    let mut todo: Vec<(PathBuf, PathBuf)> = vec![(path.to_path_buf(), at.clone())];
    let mut done: BTreeSet<PathBuf> = BTreeSet::new();
    while let Some((from, to)) = todo.pop() {
        if !done.insert(from.clone()) {
            continue;
        }
        if from.is_dir() {
            // A module's name that names a directory: one there too.
            std::fs::create_dir_all(&to).map_err(|e| e.to_string())?;
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&from) else {
            continue;
        };
        std::fs::create_dir_all(to.parent().expect("a directory")).map_err(|e| e.to_string())?;
        std::fs::write(&to, renaming.text(&text)?).map_err(|e| e.to_string())?;
        if opens_saved(&text) {
            continue;
        }
        for module in names_after(&text, "(import") {
            if module.starts_with("std.") {
                continue;
            }
            let rel: PathBuf = module.split('.').collect::<PathBuf>().with_extension("ein");
            if dir.join(&rel).exists() {
                todo.push((dir.join(&rel), prog_dir.join(&rel)));
            }
        }
    }
    let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let mut back = vec![];
    for (from, to) in [
        (prog_dir.as_path(), dir.to_path_buf()),
        (stdlib, root.join("stdlib")),
    ] {
        for f in [from.to_path_buf(), canon(from)] {
            back.push((f.display().to_string(), canon(&to).display().to_string()));
        }
    }
    back.sort_by_key(|(f, _)| std::cmp::Reverse(f.len()));
    back.dedup();
    let spelling = Spelling {
        path: at,
        stdlib: Source::Override(stdlib.to_path_buf()),
    };
    Ok((spelling, back))
}

/// Where two normalised renderings first differ, with what is around it.
fn first_line_that_differs(a: &str, b: &str) -> String {
    let at = a
        .char_indices()
        .zip(b.chars())
        .find(|((_, x), y)| x != y)
        .map_or(a.len().min(b.len()), |((i, _), _)| i);
    let around = |t: &str| {
        let from = t.floor_char_boundary(at.saturating_sub(100));
        let to = t.ceil_char_boundary((at + 100).min(t.len()));
        t[from..to].to_string()
    };
    format!(
        "at {at} of {} against {}\n      written: …{}…\n      renamed: …{}…",
        a.len(),
        b.len(),
        around(a),
        around(b)
    )
}

/// One program: both spellings rendered, normalised and compared.
fn held(path: &Path, rel: &Path, renaming: &Renaming, stdlib: &Path, ops: &[Op]) -> Held {
    let scratch = Scratch::new("ein-renaming");
    let (spelling, back) = match renamed(path, renaming, stdlib, scratch.path()) {
        Ok(r) => r,
        Err(why) => return Held::Cannot(format!("{}: {why}", rel.display())),
    };
    let written = Spelling {
        path: path.to_path_buf(),
        stdlib: ein_ir::stdlib::resolve_default(),
    };
    let a = rendered(&written, ops);
    let b = rendered(&spelling, ops);
    let mut moved = Vec::new();
    for ((op, x), (_, y)) in a.into_iter().zip(b) {
        let x = x.map(|x| normalised(&x, None));
        let y = y.map(|y| normalised(&y, Some(&back)));
        if x != y {
            let (x, y) = (x.unwrap_or_default(), y.unwrap_or_default());
            moved.push((op, first_line_that_differs(&x, &y)));
        }
    }
    if moved.is_empty() {
        Held::Same
    } else {
        Held::Moved(moved)
    }
}

#[test]
fn every_program_answers_alike_with_every_name_the_kernel_does_not_own_renamed() {
    let root = repo_root();
    let every = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files: Vec<PathBuf> = corpus_files()
        .into_iter()
        .filter(|f| every || !under_tests(f.strip_prefix(&root).unwrap_or(f)))
        .collect();
    let files = shard::keep(files, |f| {
        f.strip_prefix(&root).unwrap_or(f).display().to_string()
    });
    let ops = surfaces();
    let rel = |f: &PathBuf| f.strip_prefix(&root).unwrap_or(f).to_path_buf();
    // One bijection for the run: what it keeps is read off every corpus
    // program and every module of the standard library.
    let stdlib_files = ein_corpus::ein_files_under(&root.join("stdlib"));
    let texts: Vec<String> = corpus_files()
        .iter()
        .chain(&stdlib_files)
        .filter_map(|f| std::fs::read_to_string(f).ok())
        .collect();
    let modules: Vec<String> = stdlib_files
        .iter()
        .map(|f| {
            let rel = f
                .strip_prefix(root.join("stdlib"))
                .expect("under the stdlib");
            let parts: Vec<String> = rel
                .with_extension("")
                .iter()
                .map(|c| c.to_string_lossy().into_owned())
                .collect();
            format!("std.{}", parts.join("."))
        })
        .collect();
    let renaming = Renaming::of(&kernel_names(), &texts, &modules);
    let stdlib = Scratch::new("ein-renaming-stdlib");
    renamed_stdlib(&renaming, stdlib.path()).expect("the standard library renamed");
    let results = pool::map(
        "renaming_invariance",
        &files,
        |f| (rel(f).display().to_string(), String::new()),
        |f| held(f, &rel(f), &renaming, stdlib.path(), &ops),
    );
    let listed: BTreeSet<&str> = READS.iter().map(|(v, _)| *v).collect();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut bad: Vec<String> = Vec::new();
    let mut cannot: Vec<String> = Vec::new();
    let mut same = 0usize;
    for (f, r) in files.iter().zip(results) {
        match r {
            Held::Same => same += 1,
            Held::Cannot(why) => cannot.push(why),
            Held::Moved(moved) => {
                let mut left = Vec::new();
                for (op, line) in moved {
                    if listed.contains(op.as_str()) {
                        seen.insert(op);
                    } else {
                        left.push(format!("  [{op}] {line}"));
                    }
                }
                if left.is_empty() {
                    same += 1;
                } else {
                    bad.push(format!("{}\n{}", rel(f).display(), left.join("\n")));
                }
            }
        }
    }
    eprintln!(
        "renaming_invariance: {} programs, {same} alike, {} moved, {} cannot be held",
        files.len(),
        bad.len(),
        cannot.len()
    );
    assert!(
        cannot.is_empty(),
        "programs the renaming cannot hold:\n{}",
        cannot.join("\n")
    );
    assert!(
        bad.is_empty(),
        "{} programs answer otherwise with every name the kernel does not own renamed — \
         the engine reads a theory's or a program's name:\n{}",
        bad.len(),
        bad.iter().take(20).cloned().collect::<Vec<_>>().join("\n")
    );
    // A whole run reads every listed view on some program that moves it; a
    // shard may read none.
    if ein_corpus::shard::current().is_none() {
        let stale: Vec<&str> = listed
            .iter()
            .filter(|v| !seen.contains(**v))
            .copied()
            .collect();
        assert!(
            stale.is_empty(),
            "listed as reading a theory's name, and moved on no program: {stale:?} — \
             the view reads none now, so take it out of READS"
        );
    }
}

/// **The renaming catches a name read.** A rendering that reads `is-a` by
/// name, drawn here as a stand-in, gives one answer for the program as written
/// and another renamed: the comparison is not blind to the thing it is for.
#[test]
fn a_name_read_by_name_is_caught() {
    let kernel = kernel_names();
    let text = "(relation is-a T T :why \"{?3} is a {?4}\")\n(is-a Ann Person)\n";
    let renaming = Renaming::of(&kernel, &[text.to_string()], &[]);
    let renamed = renaming.text(text).expect("renames");
    assert!(renamed.contains("(is-a!~ Ann!~ Person!~)"), "{renamed}");
    let reads = |t: &str| {
        if t.contains("(is-a ") {
            "a type edge".to_string()
        } else {
            "a fact".to_string()
        }
    };
    assert_ne!(
        normalised(&reads(text), None),
        normalised(&reads(&renamed), Some(&[])),
        "a rendering that reads a name moves under the renaming"
    );
    assert_eq!(
        normalised(&renamed, Some(&[])),
        normalised(text, None),
        "and the renamed text reads back as written"
    );
}

/// **The bijection keeps every order by name**: two names, the kernel's
/// among them, compare as their renamings do.
#[test]
fn the_renaming_keeps_every_order_by_name() {
    let names = [
        "=",
        "A",
        "B",
        "House-1",
        "House-10",
        "House-2",
        "__closed__",
        "a",
        "a-b",
        "a.b",
        "ab",
        "b",
        "is-a",
        "is-a*",
        "not-red",
        "symmetric",
        "symmetric-negative",
        "z",
    ];
    let kernel = kernel_names();
    let r = |n: &str| {
        if kernel.contains(n) {
            n.to_string()
        } else {
            format!("{n}{SUFFIX}")
        }
    };
    for x in names {
        for y in names {
            assert_eq!(x.cmp(y), r(x).cmp(&r(y)), "{x} against {y}");
        }
    }
}
