//! **What a run interns, and when.**
//!
//! [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)
//! has to make [`Terms`] shareable across workers, and the interner is the
//! part of it that cannot simply be wrapped: [`Interner::text`] hands out a
//! `&str` borrowed from the arena, and no lock returns a borrow that outlives
//! its guard. A sharded `RwLock` — [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)'s
//! sketch — would therefore have to change every read site in the engine, not
//! the write sites.
//!
//! It does not have to, because of what this file measures: **the symbol table
//! does not grow during the search**. An engine that never grows a table can
//! share it by `&`, with no lock at all and no change to a single reader.
//!
//! Two groups of names could arrive after root saturation, and each is
//! interned before it:
//!
//! | group | names | interned by |
//! |---|---|---|
//! | the engine's own | `<lookahead-dies-immediately>` (19 files), `<forced-positive>` (4), `<monotonic-unconditional>` (4) | [`ein_core::terms::ENGINE`], interned by `Terms::new` with the rest of the kernel vocabulary |
//! | a rule's constants | `Ann`, from `(hrule guess … :assert (seat Ann ?v))` in `examples/ein-bugs/mixed-type-hypothesis.ein` — a name no fact mentions, so the compiler was the first to see it | `ein_ir::from_ir::intern_program_names`, a load pass over the registered rules and the query |
//!
//! `ENGINE` holds **five**. The other two land during *root saturation*,
//! which is single-threaded and so never a hazard.
//!
//! # What this asserts, and what it deliberately does not
//!
//! The claim is about the **search**, because that is the only region the
//! worker threads share anything across. So each file is loaded, saturated at
//! root, marked,
//! and then solved: growth before the mark is the loader's and the root
//! saturation's business and is single-threaded by construction.
//!
//! It is a corpus-scale measurement, not a proof. The one shape that could
//! be outside it is a pattern head `(?rel ?a ?b)` whose `?rel` binds to a
//! name no text of the program holds, and no program can write one: a run of
//! digits is a name the program has, and a relation parameter bound to a fact
//! is refused, by the loader where the activator is written down and by the
//! compiler where a rule derives it, before anything is interned
//! ([07-diagnostics.md § 2.6](../../../../docs/ein/user/04-reference/07-diagnostics.md#26-a-relation-named-by-a-number),
//! Q-M1e.21). A worker that cannot
//! intern can only fall back, which is
//! [03-design/08-parallelism.md § 2](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#2-what-runs-in-parallel)'s
//! to build and this file's to keep honest.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, parse};

/// The whole corpus is swept, so a run has to be bounded: a describe that
/// stops at a small budget, and a flagged program's cube only where it is
/// small. The question is *whether* the search interns, which the first nodes
/// answer as well as the last. The search is a route's.
fn options() -> explore::Options {
    let mut o = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    o.budget = 20_000;
    o.limit = 10;
    o
}

#[test]
fn the_search_interns_no_names() {
    let mut grew: Vec<String> = Vec::new();
    let (mut solved, mut skipped) = (0usize, 0usize);
    // On the corpus sweeps' pool, a program a unit: whether it
    // was solved or skipped, and what its search interned, read in order.
    let files = corpus_files();
    let found = ein_corpus::pool::map(
        "interning",
        &files,
        |p| {
            (
                p.strip_prefix(repo_root())
                    .unwrap_or(p)
                    .display()
                    .to_string(),
                String::new(),
            )
        },
        |path| -> Result<Option<String>, ()> {
            let rel = path.strip_prefix(repo_root()).unwrap_or(path).to_path_buf();
            let text = std::fs::read_to_string(path).map_err(|_| ())?;
            let mut ast = Ast::new();
            let forms = parse(&mut ast, &text, path.to_str()).map_err(|_| ())?;
            let mut terms = Terms::new();
            let mut kb =
                ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).map_err(|_| ())?;
            // Root saturation, then the mark. Everything after it is the
            // search: H built at root's fixpoint, and every node the route
            // saturates.
            ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut Events::off())
                .map_err(|_| ())?;
            let syms = terms.syms.len();
            explore::run(&mut kb, &mut terms, &ast, &options()).map_err(|_| ())?;
            Ok((terms.syms.len() != syms).then(|| {
                let names: Vec<&str> = (syms..terms.syms.len())
                    .map(|i| terms.syms.text(ein_core::Symbol(i as u32)))
                    .collect();
                format!("{} interned {names:?}", rel.display())
            }))
        },
    );
    for r in found {
        match r {
            Err(()) => skipped += 1,
            Ok(g) => {
                solved += 1;
                grew.extend(g);
            }
        }
    }

    assert!(
        solved >= 90,
        "only {solved} corpus files reached a solve ({skipped} skipped) — the sweep stopped looking"
    );
    assert!(
        grew.is_empty(),
        "{} of {solved} files grew an intern table during the search, so the table \
         cannot be shared by `&`:\n  {}",
        grew.len(),
        grew.join("\n  ")
    );
    eprintln!("interning: {solved} files solved ({skipped} skipped), 0 grew a table after root");
}

/// The five engine names are interned by `Terms::new`, before a program can
/// reach the table — which is what makes the sweep above a statement about
/// *programs* rather than about the order the engine happened to warm up in.
#[test]
fn the_engine_names_are_interned_before_any_program_is() {
    let terms = Terms::new();
    for name in ein_core::terms::ENGINE {
        assert!(
            terms.syms.get(name).is_some(),
            "{name} is in ENGINE but `Terms::new` does not intern it"
        );
    }
    let k = &terms.kernel;
    let by_field = [
        k.forced_positive,
        k.lookahead_dies,
        k.monotonic_unconditional,
        k.query_rule,
        k.closed,
    ];
    for (sym, name) in by_field.iter().zip(ein_core::terms::ENGINE) {
        assert_eq!(
            terms.sym(*sym),
            name,
            "a `Kernel` field and its `ENGINE` entry disagree"
        );
    }
}

/// A constant that appears only inside a rule is interned at **load**, not at
/// first compile. The fixture is the one file in the corpus that had one.
#[test]
fn a_rule_only_constant_is_interned_at_load() {
    let path = repo_root().join("examples/ein-bugs/mixed-type-hypothesis.ein");
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut terms = Terms::new();
    let _kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    // `Ann` is named by the `guess` hrule's `:assert` and by the query goal,
    // and by no fact — so before `intern_program_names` it first existed when
    // the hypothesis loop compiled the rule.
    assert!(
        terms.syms.get("Ann").is_some(),
        "a constant that only a rule mentions is still unknown after load"
    );
}
