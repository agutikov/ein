//! T1a.7.1.0 — **what a run interns, and when.**
//!
//! [S1a.7.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md)
//! has to make [`Terms`] shareable across workers, and the interner is the
//! part of it that cannot simply be wrapped: [`Interner::text`] hands out a
//! `&str` borrowed from the arena, and no lock returns a borrow that outlives
//! its guard. A sharded `RwLock` — [design/08
//! §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)'s
//! sketch — would therefore have to change every read site in the port, not
//! the write sites.
//!
//! It does not have to, because of what this file measures: **the symbol table
//! and the integer pool do not grow during the search.** An engine that never
//! grows a table can share it by `&`, with no lock at all and no change to a
//! single reader.
//!
//! That was not true when the question was first asked (2026-08-22). **Four
//! names arrived after root saturation, on 24 of the 90 corpus files that
//! reach a solve**, in two groups, and both are now closed:
//!
//! | group | names | closed by |
//! |---|---|---|
//! | the engine's own | `<lookahead-dies-immediately>` (19 files), `<forced-positive>` (4), `<monotonic-unconditional>` (4) | [`ein_core::terms::ENGINE`], interned by `Terms::new` with the rest of the kernel vocabulary |
//! | a rule's constants | `Ann`, from `(hrule guess … :assert (seat Ann ?v))` in `examples/ein-bugs/mixed-type-hypothesis.ein` — a name no fact mentions, so the compiler was the first to see it | `ein_ir::from_ir::intern_program_names`, a load pass over the registered rules and the query |
//!
//! `ENGINE` holds **eight**. The other five land during *root saturation*,
//! which is single-threaded and so never a hazard — but `__symmetric__` was
//! interned by every `Saturator::new` on 94 of the files, and `a` / `b`, the
//! names the native mirror reports a firing's bindings under, were interned
//! **per mirror firing**. Both are now a field read.
//!
//! # What this asserts, and what it deliberately does not
//!
//! The claim is about the **search**, because that is the only region P1a.7
//! shares anything across. So each file is loaded, saturated at root, marked,
//! and then solved: growth before the mark is the loader's and the root
//! saturation's business and is single-threaded by construction.
//!
//! It is a corpus-scale measurement, not a proof. One shape is known to be
//! outside it and is named rather than hidden — a pattern head `(?rel ?a ?b)`
//! whose `?rel` binds to an **integer**, whose decimal text the compiler
//! interns as a symbol. Nothing in the corpus has one. What makes that
//! survivable is that the consequence of being wrong is not a race but a
//! *missing name*: a worker that cannot intern can only fall back, which is
//! [S1a.7.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)'s
//! to build and this file's to keep honest.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, parse};

/// The whole corpus is swept, so a solve has to be bounded: this is the same
/// budget `solve_shape` uses for the shape digests, and for the same reason.
/// The question is *whether* the search interns, which the first enterings
/// answer as well as the last.
const MAX_ENTERINGS: u64 = 60;

#[test]
fn the_search_interns_no_names() {
    let mut grew: Vec<String> = Vec::new();
    let (mut solved, mut skipped) = (0usize, 0usize);
    for path in corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_path_buf();
        let Ok(text) = std::fs::read_to_string(&path) else {
            skipped += 1;
            continue;
        };
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
            skipped += 1;
            continue;
        };
        let mut terms = Terms::new();
        let Ok(mut kb) = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()) else {
            skipped += 1;
            continue;
        };
        // Root saturation, then the mark. Everything after it is the search.
        let mut events = Events::off();
        if ein_infer::saturate_events(&ast, &mut terms, &mut kb).is_err() {
            skipped += 1;
            continue;
        }
        let (syms, ints) = (terms.syms.len(), terms.ints.len());
        let opts = SolveOptions {
            stop_after: None,
            max_enterings: Some(MAX_ENTERINGS),
            on_budget: ein_infer::solve::OnBudget::Verdict,
            ..SolveOptions::default()
        };
        if solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).is_err() {
            skipped += 1;
            continue;
        }
        solved += 1;
        if terms.syms.len() != syms {
            let names: Vec<&str> = (syms..terms.syms.len())
                .map(|i| terms.syms.text(ein_core::Symbol(i as u32)))
                .collect();
            grew.push(format!("{} interned {names:?}", rel.display()));
        }
        if terms.ints.len() != ints {
            grew.push(format!(
                "{} interned {} integer literal(s)",
                rel.display(),
                terms.ints.len() - ints
            ));
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

/// The eight engine names are interned by `Terms::new`, before a program can
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
        k.symmetric,
        k.mirror_a,
        k.mirror_b,
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
