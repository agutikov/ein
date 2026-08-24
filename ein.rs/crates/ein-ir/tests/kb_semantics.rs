//! T1a.10.2.2 — **the KB after load**: what the loader built, where each fact
//! came from, and which library it read to get there.
//!
//! Replaces the semantics half of `ein.py/tests/kb/`, which is being deleted
//! with its implementation: `test_entities.py` (a fact's origin),
//! `test_imports.py` (the three import tiers, the reserved-name guard),
//! `test_layers.py` (the fact view's filters), `test_load_negative.py` (the
//! `.expected` corpus's portability), `test_provenance.py` (`walk_premises`
//! and the derivation DAG), `test_stdlib_resolution.py` (where `std.*` comes
//! from), `test_store.py` and `test_store_indexing.py` (the registries and the
//! seven cross-reference indexes).
//!
//! It lives in `ein-ir` rather than in `ein-core` for one reason: every claim
//! here is about a KB *that was loaded from source*, and `ein-core` has no
//! parser. The Python originals had the same shape — `tests/kb/conftest.py`'s
//! fixtures are `KnowledgeBase.from_ir(parse(text))`.
//!
//! Two things the port deliberately does **not** translate. The Python suite
//! reached into `kb._facts_by_relation`, `kb.names`, `kb._nogoods` and
//! `_cached_macro_names`; none of those names exists here, and a test that
//! poked the Rust equivalents would pin this build rather than the language.
//! What each of them protected is asserted through [`ein_core::shape`], the
//! `Kb` reads, and the walks instead. And the Python fixtures that drove
//! `$EIN_STDLIB` with `monkeypatch` are driven through
//! [`Resolver::with_stdlib`] here: mutating the process environment from a
//! test that runs in parallel with others reading it is unsound in Rust 2024,
//! and the injection seam exists precisely so a test need not.

use ein_core::{
    BitSet, FactId, IntId, Justifications, Kb, Prov, ProvKind, Symbol, Tag, Terms, Value,
    build_derivation_dag, shape, unsat_core, walk_premises, walks::is_frontier,
};
use ein_corpus::{corpus_files, repo_root};
use ein_ir::imports::Resolver;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::stdlib::{self, MARKER, Source};
use ein_ir::{Ast, dump_canonical, load, load_file, parse};
use std::path::{Path, PathBuf};

// ── Fixtures ───────────────────────────────────────────────────────

/// Parse and load inline source with no base directory — `from_ir`.
///
/// Fixtures here author rule provenance with `:rule R :using (via (p a b) …)`.
/// The wrapper head — `via` throughout this file, `p` in
/// `examples/broken/load/derivation_cycle.ein` — is read by nothing: `:using`
/// takes one value, and the headless list `((p a b) …)` the engine's own dumps
/// would prefer is [not in the
/// grammar](../../../../docs/kernel/ir/03-ein-lang/05_inspirations.md). Every
/// *inner* S-form is interned as a premise proposition, which is why a premise
/// may name a fact this KB never held.
fn kb_of(text: &str) -> (Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
    let kb = load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (terms, kb)
}

/// The load error inline source produces, with no base directory.
fn load_error(text: &str) -> String {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
    match load(&mut ast, &mut terms, &forms, None) {
        Ok(_) => panic!("expected a KBLoadError, but it loaded"),
        Err(e) => e.0,
    }
}

/// The one fact rendering `(rel arg arg)` names, or a panic naming what is
/// there — an assertion on a missing fact should say which facts exist.
fn fact(terms: &Terms, kb: &Kb, compact: &str) -> FactId {
    kb.facts()
        .find(|&f| terms.compact(f) == compact)
        .unwrap_or_else(|| {
            let all: Vec<String> = kb.facts().map(|f| terms.compact(f)).collect();
            panic!("no {compact} among {all:?}")
        })
}

/// Every fact of the KB, rendered and sorted — the readable shape of a model.
fn rendered(terms: &Terms, ids: impl Iterator<Item = FactId>) -> Vec<String> {
    let mut out: Vec<String> = ids.map(|f| terms.compact(f)).collect();
    out.sort();
    out
}

fn names<V>(terms: &Terms, registry: &ein_core::Registry<V>) -> Vec<String> {
    let mut out: Vec<String> = registry
        .iter()
        .map(|(n, _)| terms.sym(n).to_string())
        .collect();
    out.sort();
    out
}

/// A private directory for a fixture project. Named after the process so two
/// concurrent `cargo test` runs cannot collide, and cleared on entry so a
/// crashed run leaves nothing behind for the next one to read.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-kb-semantics-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    path
}

// ── A fact's origin ────────────────────────────────────────────────

/// A fact's origin is a **three-way** split, not a two-way one — and the third
/// case is the one that keeps catching people out — and the declaration's own
/// companion facts are in it.
///
/// `is_given` and `is_derived` replaced the old `Layer` enum, and they are not
/// complements: a `:source`-kind record *with* a source id is given, any
/// non-`:source` kind is derived, and both a missing record and a `:source`
/// record with no id are **neither**. That third bucket is where a puzzle's
/// background ontology lives, which is why `kb_to_ein_text` emits it first and
/// why `--print-final-facts` drops it — a two-way reading would classify the
/// whole ontology as authored input.
///
/// The frontier is a *fourth* reading of the same records and deliberately
/// disagrees with both: a hypothesis is derived and is still a walk terminal.
#[test]
fn a_facts_origin_is_a_three_way_split_over_its_provenance() {
    let (mut terms, mut kb) = kb_of(
        r#"
        (relation r T T)
        (r a b :source "(1)")
        (r c d :rule step :using (via (r a b)))
        (r e f)
        "#,
    );
    // The two kinds the surface language cannot author, added through the
    // engine's own API — a fork introduces one and contradicts the other.
    let rel = terms.intern_text("r").expect("room");
    for (args, prov) in [
        (["g", "h"], Some(Prov::from_hypothesis(1, None))),
        (["i", "j"], Some(Prov::rejected(1, None))),
        (["k", "l"], None),
    ] {
        let args: Vec<Value> = args
            .iter()
            .map(|a| terms.value_text(a).expect("room"))
            .collect();
        let prov = prov.map(|p| terms.provs.push(p));
        kb.add_and_index_fact(&mut terms, rel, &args, prov)
            .expect("room");
    }

    let split = |f: FactId| match kb.primary(f) {
        None => (false, false),
        Some(p) => {
            let prov = terms.provs.get(p);
            (
                prov.kind == ProvKind::Source && prov.source.is_some(),
                prov.kind != ProvKind::Source,
            )
        }
    };
    let bucket = |want: (bool, bool)| rendered(&terms, kb.facts().filter(|&f| split(f) == want));

    assert_eq!(bucket((true, false)), ["(r a b)"], "given");
    assert_eq!(
        bucket((false, true)),
        ["(r c d)", "(r g h)", "(r i j)"],
        "derived — a rule firing, a hypothesis and a rejected hypothesis alike"
    );
    assert_eq!(
        bucket((false, false)),
        ["(r e f)", "(r k l)", "(relation r T T)", "(relation r)"],
        "background — an unannotated fact, one with no record at all, and the \
         two companion facts `(relation r T T)` emits about itself"
    );
    assert!(
        !kb.facts().any(|f| split(f) == (true, true)),
        "given and derived are exclusive"
    );

    // The two view filters read the same records and select the same two
    // buckets — `by_rule` is empty before reasoning only because nothing
    // carries rule provenance, not because the view knows about phases.
    let source = terms.syms.get("(1)").expect("interned");
    let step = terms.syms.get("step").expect("interned");
    let view = kb.all_facts(&terms);
    assert_eq!(rendered(&terms, view.by_source(source)), ["(r a b)"]);
    assert_eq!(rendered(&terms, view.by_rule(step)), ["(r c d)"]);
    assert_eq!(view.by_rule(rel).count(), 0, "no rule is named `r`");

    // What the split looks like from outside: the rendered record is the
    // discriminator, and the no-record fact has no line at all.
    let text = shape(&kb, &terms);
    assert!(text.contains("source='(1)'"), "{text}");
    assert!(text.contains("source=None"), "{text}");
    assert!(
        text.contains("rule=step using=[('r', ('a', 'b'))]"),
        "{text}"
    );
    assert_eq!(
        text.lines().filter(|l| l.starts_with("PROV ")).count(),
        5,
        "five of the six facts carry a record; `(r k l)` carries none:\n{text}"
    );

    // …and the frontier is not the given/derived split: the hypothesis is
    // derived and still terminal, the rule firing is derived and is not.
    assert!(is_frontier(&kb, &terms, fact(&terms, &kb, "(r g h)")));
    assert!(is_frontier(&kb, &terms, fact(&terms, &kb, "(r k l)")));
    assert!(!is_frontier(&kb, &terms, fact(&terms, &kb, "(r c d)")));
}

// ── Imports ────────────────────────────────────────────────────────

/// The **rule form** of a program, expanded — the only rendering that shows
/// whether a macro head was recognised, because an unrecognised one survives
/// expansion as an ordinary relational form instead of raising.
fn expanded_rule(text: &str) -> String {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
    let resolved = Resolver::new()
        .resolve_imports(&mut ast, &forms, None)
        .expect("resolves");
    let macros = collect_macros(&ast, &resolved);
    let expanded = expand_rule_clauses(&mut ast, &resolved, &macros).expect("expands");
    let rule = expanded
        .iter()
        .copied()
        .find(|&f| ast.head_name(f) == Some("rule"))
        .expect("a (rule …) form");
    dump_canonical(&ast, &[rule])
}

/// A whole-module import renames every definition, and the renamed head is
/// still a macro head.
///
/// The renaming half is easy to see; the invocable half is not, because a
/// macro head that stopped being recognised does not fail — the expander
/// leaves the form alone and it loads as a relational premise named
/// `std.macro.forall`, which matches nothing and quietly makes the rule
/// unfireable. So the assertion is not "it expanded" but "it expanded to the
/// *same body* the flat import produces", which is the only statement that
/// distinguishes an expansion from a passthrough.
#[test]
fn a_whole_module_import_qualifies_every_name_and_the_qualified_head_still_expands() {
    let body = |head: &str| {
        format!(
            r#"
            (relation player T T) (relation beats T T)
            (rule r ()
              :match (and (player ?p)
                          ({head} ?q (and (player ?q) (neq ?p ?q)) (beats ?p ?q)))
              :assert (ok ?p) :why "u")
            "#
        )
    };
    let flat = format!("(import std.macro :symbols (forall))\n{}", body("forall"));
    let whole = format!("(import std.macro)\n{}", body("std.macro.forall"));

    let (terms, kb) = kb_of(&whole);
    assert_eq!(
        names(&terms, &kb.program().macros),
        ["std.macro.forall", "std.macro.unknown"],
        "a bare (import M) prefixes every definition, `unknown` included"
    );
    let (flat_terms, flat_kb) = kb_of(&flat);
    assert_eq!(
        names(&flat_terms, &flat_kb.program().macros),
        ["forall"],
        ":symbols is flat, and pulls only the closure of what was listed"
    );

    assert_eq!(
        expanded_rule(&whole),
        expanded_rule(&flat),
        "the qualified head must expand to the body the flat one does"
    );
    // Non-vacuity: the shared body is an expansion, not the invocation.
    assert!(expanded_rule(&whole).contains("absent"), "forall's body");
    assert!(!expanded_rule(&whole).contains("forall"), "no residue");
}

/// Importing a module whole re-qualifies what that module imported **flat**,
/// not only what it defined.
///
/// The interesting case for a library author: `mid` pulls `forall` in flat and
/// wraps it, so `forall` is one of `mid`'s own names by the time an importer
/// sees it, and the importer gets `mid.forall` alongside `mid.wrap`. If
/// re-export were shallow, `mid.wrap`'s body would still name the bare
/// `forall` its importer never imported and the composition would silently
/// stop expanding one level in.
#[test]
fn importing_a_module_whole_requalifies_what_it_imported_flat() {
    let dir = scratch("reexport");
    write(
        &dir,
        "mid.ein",
        "(import std.macro :symbols (forall))\n(macro wrap (?g ?b) (forall ?x ?g ?b))\n",
    );
    let main = write(
        &dir,
        "main.ein",
        "(import mid)\n\
         (relation player T T) (relation beats T T)\n\
         (rule r () :match (and (player ?p) (mid.wrap (player ?p) (beats ?p ?p)))\n\
         \x20 :assert (ok ?p) :why \"w\")\n",
    );

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &main).expect("the project loads");
    assert_eq!(
        names(&terms, &kb.program().macros),
        ["mid.forall", "mid.wrap"],
        "both the definition and the re-export are re-qualified"
    );

    let text = std::fs::read_to_string(&main).expect("readable");
    let forms = parse(&mut ast, &text, main.to_str()).expect("parses");
    let resolved = Resolver::new()
        .resolve_imports(&mut ast, &forms, main.parent())
        .expect("resolves");
    let macros = collect_macros(&ast, &resolved);
    let expanded = expand_rule_clauses(&mut ast, &resolved, &macros).expect("expands");
    let rule = expanded
        .iter()
        .copied()
        .find(|&f| ast.head_name(f) == Some("rule"))
        .expect("a (rule …) form");
    let rule = dump_canonical(&ast, &[rule]);
    assert!(
        rule.contains("absent") && !rule.contains("mid."),
        "the composition expands through both levels:\n{rule}"
    );
}

/// A file-relative import resolves against the **importing file's** directory,
/// and the two tiers behave differently on the same module.
///
/// Worth pinning together because the tiers are not two spellings of one
/// thing: whole-module qualifies and takes everything, `:symbols` stays flat
/// and takes only the closure of what was named — so `unused`, which nothing
/// references, never arrives. A resolver that pulled the module and then
/// filtered would pass the first half and fail the second.
#[test]
fn a_file_relative_import_resolves_against_the_importing_file() {
    let dir = scratch("relative");
    write(
        &dir,
        "lib.ein",
        "(relation knows T T)\n\
         (rule sym (?r) :match (?r ?a ?b) :assert (?r ?b ?a) :why \"s\")\n",
    );
    let main = write(&dir, "main.ein", "(import lib)\n(relation x T T)\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &main).expect("the project loads");
    assert!(names(&terms, &kb.program().relations).contains(&"lib.knows".to_string()));
    assert!(names(&terms, &kb.program().rules).contains(&"lib.sym".to_string()));

    let dir = scratch("relative-symbols");
    write(
        &dir,
        "lib.ein",
        "(macro twice (?x) (and ?x ?x))\n(macro unused (?y) (not ?y))\n",
    );
    let main = write(&dir, "main.ein", "(import lib :symbols (twice))\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &main).expect("the project loads");
    assert_eq!(
        names(&terms, &kb.program().macros),
        ["twice"],
        "flat, and `unused` stays behind"
    );
}

/// The one loader diagnostic that cannot be a `.ein` fixture.
///
/// It fires only when `base_dir` is `None`, and loading a *file* always
/// supplies one — so `examples/broken/load/` cannot hold it and
/// [its README](../../../../examples/broken/load/README.md) records the gap.
/// That makes this test the corpus entry: it is the only place the message is
/// checked at all, in either implementation.
#[test]
fn a_file_relative_import_without_a_base_directory_is_refused() {
    assert_eq!(
        load_error("(import mymod.x)"),
        "(import mymod.x) — file-relative import needs a base directory \
         (load from a file path) at None"
    );
    // …and a `std.*` module needs no base, which is what makes the check a
    // property of the *module name* rather than of loading from a string.
    let (terms, kb) = kb_of("(import std.macro :symbols (forall))");
    assert_eq!(names(&terms, &kb.program().macros), ["forall"]);
}

/// `:symbols` pulls the listed names **plus their name-reference closure**,
/// transitively through the imported module's own imports.
///
/// This is the property that makes the stdlib usable at all: a puzzle names
/// one entry rule and gets a working stack. It is also the property most
/// likely to rot silently — a closure that stopped following the module's own
/// imports would still load `zebra2`, because `zebra2` lists three entry
/// points and the missing rules would simply never fire. So the fixture names
/// exactly one rule and asserts on eight it did not name.
#[test]
fn a_symbols_import_pulls_the_name_reference_closure() {
    let (terms, kb) = kb_of(
        "(import std.bijection :symbols (bijective-setup))\n\
         (relation color-of T T)\n",
    );
    let rules = names(&terms, &kb.program().rules);
    for pulled in [
        "domain-elimination",
        "functional",
        "functional-negative",
        "injective",
        "injective-negative",
        "range-elimination",
        "surjective",
        "total",
    ] {
        assert!(
            rules.contains(&pulled.to_string()),
            "auto-closure did not pull {pulled}; it pulled {rules:?}"
        );
    }
    assert!(
        names(&terms, &kb.program().macros).contains(&"forall".to_string()),
        "`forall` rode in through std.bijection's own import of std.macro"
    );
}

/// The reserved-kernel-name guard fires on **declarators only**, and matches
/// whole names.
///
/// Both halves are easy to get wrong in the same direction. A guard applied to
/// every form would reject `(not (likes A B))`, which is an ordinary fact
/// whose head happens to be kernel vocabulary — `not`-headed facts are how the
/// engine records negative knowledge, so rejecting them would break the
/// negation model. A guard written as a prefix test would reject `eq-elim` and
/// `absent-of`, which are just names that begin with kernel words.
///
/// The names that can *reach* the guard are `absent`, `eq`, `false` and
/// `relation` — the other four in [`ein_core::RESERVED`] (`and`, `neq`, `not`,
/// `or`) are among the eleven words `SYMBOL`'s negative lookahead rejects, so
/// `(hrule not …)` is a parse error and never becomes a load error at all.
#[test]
fn the_reserved_name_guard_is_on_declarators_and_matches_whole_names() {
    let (terms, kb) = kb_of("(not (likes A B) :source \"(1)\") (relation likes T T)");
    let not = terms.syms.get("not").expect("interned");
    assert_eq!(
        rendered(&terms, kb.facts().filter(|&f| terms.facts.rel(f) == not)).len(),
        1,
        "a reserved head is legal on a fact"
    );

    let (terms, kb) = kb_of(
        "(rule eq-elim () :match (x ?a) :assert (y ?a) :why \"ok\")\n\
         (relation absent-of T T)\n\
         (relation x T T) (relation y T T)\n",
    );
    assert!(names(&terms, &kb.program().rules).contains(&"eq-elim".to_string()));
    assert!(names(&terms, &kb.program().relations).contains(&"absent-of".to_string()));

    // Non-vacuity: the guard is live on the whole names those two extend.
    for (declarator, source) in [
        (
            "rule",
            "(rule eq () :match (x ?a) :assert (y ?a) :why \"w\")",
        ),
        ("relation", "(relation absent T T)"),
        (
            "hrule",
            "(hrule false () :match (x ?a) :assert (y ?a) :why \"w\")",
        ),
    ] {
        let msg = load_error(source);
        assert!(
            msg.contains("shadows a reserved kernel name"),
            "{declarator}: {msg}"
        );
    }
}

// ── The load-negative corpus ───────────────────────────────────────

/// No `.expected` may carry an absolute path from the machine that wrote it.
///
/// The failure this prevents is invisible in the run that causes it: a
/// message whose path was captured verbatim passes on the machine that
/// blessed it and fails in every other checkout, including CI's. Placeholders
/// are the fix, and they only work if their use is enforced rather than
/// remembered — which is the whole reason this is a test and not a note in
/// the corpus README.
#[test]
fn no_load_negative_expectation_carries_a_machine_specific_path() {
    let root = repo_root();
    let dir = root.join("examples/broken/load");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the load-negative corpus")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "expected"))
        .collect();
    files.sort();
    assert!(
        files.len() >= 29,
        "only {} .expected files — the glob stopped matching",
        files.len()
    );

    let root = root.to_str().expect("utf-8");
    let mut with_placeholders = 0;
    for path in &files {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
        assert!(
            !text.contains(root),
            "{}: holds {root} — use {{FILE}} / {{DIR}} / {{STDLIB}}",
            path.display()
        );
        if text.contains("{FILE}") || text.contains("{DIR}") || text.contains("{STDLIB}") {
            with_placeholders += 1;
        }
    }
    assert!(
        with_placeholders >= 5,
        "only {with_placeholders} fixtures use a placeholder — either the \
         messages stopped naming paths or the convention was abandoned"
    );
}

// ── Provenance walks ───────────────────────────────────────────────

/// The diamond: `s1,s2 → d1`, `s2 → d2`, `d1,d2 → top`. Frontier(top) =
/// {s1,s2}, and the two `d`s are the intermediates a walk has to pass through
/// without collecting.
fn diamond() -> (Terms, Kb) {
    kb_of(
        r#"
        (relation r T T) (relation d T T) (relation top T T)
        (r A B :source "(1)")
        (r B C :source "(2)")
        (d A C :rule compose :using (via (r A B) (r B C)))
        (d C A :rule flip :using (via (r B C)))
        (top A A :rule join :using (via (d A C) (d C A)))
        "#,
    )
}

/// `keep` decides **membership**, never descent.
///
/// The natural misreading is that a walk stops where its predicate stops
/// holding — that is how a filter usually behaves, and it is what a
/// short-circuiting implementation would give you for free. Here it would
/// break the unsat core: the frontier predicate is false on every rule-derived
/// fact, so a walk that stopped at a failing `keep` would never reach a source
/// at all. The two halves below are the same walk with two predicates: one
/// collects only the intermediates, the other collects only what lies beyond
/// them.
#[test]
fn keep_decides_membership_without_stopping_the_walk() {
    let (terms, kb) = diamond();
    let top = fact(&terms, &kb, "(top A A)");
    let rel_is = |name: &str| terms.syms.get(name).expect("interned");
    let (d, r) = (rel_is("d"), rel_is("r"));

    let mut visited = BitSet::new();
    let intermediates = walk_premises(
        &kb,
        &terms,
        top,
        &|_, terms: &Terms, f| terms.facts.rel(f) == d,
        &mut visited,
        Justifications::Primary,
    );
    assert_eq!(
        rendered(&terms, intermediates.into_iter()),
        ["(d A C)", "(d C A)"],
        "a non-leaf predicate selects the rule-kind intermediates"
    );

    let mut visited = BitSet::new();
    let beyond = walk_premises(
        &kb,
        &terms,
        top,
        &|_, terms: &Terms, f| terms.facts.rel(f) == r,
        &mut visited,
        Justifications::Primary,
    );
    assert_eq!(
        rendered(&terms, beyond.into_iter()),
        ["(r A B)", "(r B C)"],
        "and the same walk still reaches what lies *below* facts it did not keep"
    );

    // A shared `visited` unions across roots rather than re-walking: the two
    // intermediates together have the same frontier `top` does.
    let mut visited = BitSet::new();
    let mut union = Vec::new();
    for name in ["(d A C)", "(d C A)"] {
        union.extend(walk_premises(
            &kb,
            &terms,
            fact(&terms, &kb, name),
            &is_frontier,
            &mut visited,
            Justifications::Primary,
        ));
    }
    assert_eq!(rendered(&terms, union.into_iter()), ["(r A B)", "(r B C)"]);
}

/// The unsat core and the derivation DAG must answer the same question the
/// same way.
///
/// They are two walks over one graph written at different times for different
/// consumers — the core feeds the no-good clause, the DAG feeds the rendered
/// explanation — and nothing in either implementation forces them to agree.
/// A drift would be invisible in the ordinary case and would surface as an
/// explanation that names a premise the no-good does not, which reads as an
/// engine bug rather than as a walk bug.
#[test]
fn the_unsat_core_frontier_is_the_derivation_dags_sources() {
    let (terms, kb) = diamond();
    let top = fact(&terms, &kb, "(top A A)");

    let core = unsat_core(&kb, &terms, &[top], Justifications::Primary);
    let dag = build_derivation_dag(&kb, &terms, top, Justifications::Primary);
    assert_eq!(
        rendered(&terms, core.into_iter()),
        rendered(&terms, dag.sources(&kb, &terms).into_iter()),
    );
    // Non-vacuity: both are the two sources, and neither is everything.
    assert_eq!(
        rendered(&terms, dag.sources(&kb, &terms).into_iter()),
        ["(r A B)", "(r B C)"]
    );
    assert_eq!(dag.nodes.len(), 5, "the DAG saw the intermediates too");
}

// ── Where `std.*` comes from ───────────────────────────────────────

/// The resolved stdlib root is the **only** one consulted — the checkout being
/// right there changes nothing.
///
/// This is what `$EIN_STDLIB` buys: point it at a
/// directory and that directory is the standard library, entire. The claim is
/// asserted through the injection seam rather than through the environment,
/// because `stdlib::resolve` maps the variable onto [`Source::Override`] in
/// three lines that return before the checkout walk, and mutating a
/// process-wide variable from one test while others read it is unsound. The
/// ranking those three lines implement is asserted directly below, in
/// whichever configuration the run happens to be in.
#[test]
fn the_resolved_stdlib_root_is_the_only_one_consulted() {
    let dir = scratch("override");
    write(&dir, "macro.ein", "(macro only-here (?p) (rel ?p))\n");

    assert_eq!(
        Resolver::with_stdlib(Source::Override(dir.clone())).stdlib_macro_names(),
        ["only-here"],
        "the override's macro.ein, and nothing from the checkout"
    );
    let checkout = repo_root().join("stdlib");
    assert_eq!(
        Resolver::with_stdlib(Source::Checkout(checkout.clone())).stdlib_macro_names(),
        ["forall", "unknown"],
        "…which is not what the checkout says, so the previous line means something"
    );

    // The ranking itself: an override outranks the checkout, and the checkout
    // outranks the embedded copy. Only one of the two branches is exercised
    // per run, and which one is a property of the environment, not of a choice
    // this test makes.
    let from = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match std::env::var_os("EIN_STDLIB") {
        Some(over) => assert_eq!(
            stdlib::resolve(&from),
            Source::Override(PathBuf::from(over)),
            "an override wins from inside the checkout"
        ),
        None => match stdlib::resolve(&from) {
            Source::Checkout(found) => {
                assert!(found.join(MARKER).is_file(), "{}", found.display());
                assert_eq!(found, checkout);
            }
            other => panic!("expected the checkout, got {other:?}"),
        },
    }
}

/// An override is honoured whatever it points at, and a miss is reported
/// rather than papered over.
///
/// A directory called `stdlib/` proves nothing — `MANIFEST.sha256` is what
/// identifies the checkout during the *walk*, and an explicit source skips the
/// walk entirely, so an empty directory is a perfectly good (and empty)
/// standard library. The consequence is the part worth pinning: a puzzle
/// loaded through it fails naming the path it looked at, instead of quietly
/// falling back to the checkout or to the embedded copy and reporting success
/// for a program the operator never pointed at.
#[test]
fn an_override_is_honoured_whatever_it_points_at() {
    let empty = scratch("empty-stdlib");
    assert!(!empty.join(MARKER).is_file(), "no marker, on purpose");
    assert!(
        Source::Override(empty.clone()).modules().is_empty(),
        "and no modules either"
    );

    let zebra2 = repo_root().join("examples/zebra2.ein");
    let text = std::fs::read_to_string(&zebra2).expect("readable");
    let mut ast = Ast::new();
    let forms = parse(&mut ast, &text, zebra2.to_str()).expect("parses");
    let err = Resolver::with_stdlib(Source::Override(empty.clone()))
        .resolve_imports(&mut ast, &forms, zebra2.parent())
        .expect_err("an empty stdlib cannot answer zebra2's imports");
    assert_eq!(
        err.0,
        format!(
            "(import std.algebra) — module not found at {}/algebra.ein (None)",
            empty.display()
        )
    );
    // The same file through the ordinary resolver loads, so the failure above
    // is the override's and not the puzzle's.
    let mut terms = Terms::new();
    let kb = load_file(&mut Ast::new(), &mut terms, &zebra2).expect("zebra2 loads normally");
    assert!(kb.n_facts() > 30 && !kb.program().rules.is_empty());
}

/// The module set is re-read on every resolution, not memoised for the life of
/// the process.
///
/// This is the entire reason a checkout outranks a packaged copy: edit a
/// stdlib module, re-run, see the change. ein.py had to say so explicitly
/// because its `stdlib_macro_names` was an `lru_cache` and the tests had to
/// clear it; here there is no cache to clear, which is exactly the kind of
/// property that gets quietly reintroduced by someone optimising a filesystem
/// read out of a hot path. The same [`Resolver`] is used for both calls, so a
/// cache anywhere — per process, per resolver — fails this.
#[test]
fn editing_a_module_takes_effect_without_a_rebuild() {
    let dir = scratch("editable");
    let macro_ein = write(&dir, "macro.ein", "(macro forall (?p) (rel ?p))\n");
    let resolver = Resolver::with_stdlib(Source::Override(dir.clone()));
    assert_eq!(resolver.stdlib_macro_names(), ["forall"]);

    std::fs::write(
        &macro_ein,
        "(macro forall (?p) (rel ?p))\n(macro unknown (?p) (rel ?p))\n",
    )
    .expect("writable");
    assert_eq!(
        resolver.stdlib_macro_names(),
        ["forall", "unknown"],
        "the second call re-read the file the first one read"
    );
}

// ── The store: registries, indexes, snapshots ──────────────────────

/// A `Terms` whose id space is a permutation of `after`'s.
///
/// Reverse assignment order for names and integer literals, a run of junk
/// facts ahead of everything so `FactId` 0 is not special, and the facts
/// themselves re-interned in reverse *within each nesting depth* — depth-blind
/// would be illegal, since `(not (R a b))` cannot be interned before the fact
/// it wraps. The kernel names keep ids 0–17 in both runs because `Terms::new`
/// interns them before a caller can reach the table.
fn permuted(after: &Terms) -> Terms {
    let mut terms = Terms::new();
    let junk = terms.intern_text("@kb-semantics-probe").expect("room");
    for k in 0..7 {
        let arg = terms.value_text(&format!("@probe-{k}")).expect("room");
        terms.intern_fact(junk, &[arg]).expect("room");
    }
    for i in (0..after.syms.len()).rev() {
        terms
            .intern_text(after.syms.text(Symbol(i as u32)))
            .expect("room");
    }
    for i in (0..after.ints.len()).rev() {
        terms
            .value_int(after.ints.text(IntId(i as u32)))
            .expect("room");
    }

    let n = after.facts.len();
    let mut depth: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        let (_, args) = after.facts.get(FactId(i as u32));
        let d = args
            .iter()
            .filter_map(|v| v.as_fact())
            .map(|f| depth[f.0 as usize] + 1)
            .max()
            .unwrap_or(0);
        depth.push(d);
    }
    let mut moved: Vec<Option<FactId>> = vec![None; n];
    for level in 0..=depth.iter().copied().max().unwrap_or(0) {
        for i in (0..n).rev().filter(|&i| depth[i] == level) {
            let (rel, args) = after.facts.get(FactId(i as u32));
            let rel_text = after.syms.text(rel).to_string();
            let args: Vec<Value> = args
                .iter()
                .map(|&v| match v.tag() {
                    Tag::Sym => {
                        let text = after.syms.text(Symbol(v.payload())).to_string();
                        terms.value_text(&text).expect("room")
                    }
                    Tag::Int => {
                        let text = after.ints.text(IntId(v.payload())).to_string();
                        terms.value_int(&text).expect("room")
                    }
                    Tag::Fact => Value::fact(moved[v.payload() as usize].expect("a lower depth")),
                })
                .collect();
            let rel = terms.intern_text(&rel_text).expect("room");
            moved[i] = Some(terms.intern_fact(rel, &args).expect("room"));
        }
    }
    terms
}

/// The loaded KB's whole shape — registries, the seven cross-reference
/// indexes, every fact's provenance — is a function of the **file**, and of
/// nothing else about the run.
///
/// This is what replaces `load_parity.rs`'s oracle sweep once ein.py is gone.
/// The oracle answered "is the shape right"; nothing in a single-implementation
/// repo can answer that, and pretending otherwise is the trap
/// [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
/// names. What survives is the half that is still falsifiable and is the half
/// that actually catches bugs: **nothing in that text may depend on which
/// integer a name happened to be assigned.** `Symbol` has no `Ord` and every
/// observable sort goes through `Interner::rank` precisely so this holds; a
/// map iterated in id order, a `min_by_key` over a `Value`, a set rendered in
/// insertion order all break it, and all of them are invisible in a run where
/// the ids arrive in source order.
#[test]
fn the_kb_shape_does_not_move_when_the_id_space_is_permuted() {
    let files = corpus_files();
    assert!(files.len() >= 90, "only {} corpus files", files.len());
    let (mut loaded, mut rejected, mut bad) = (0usize, 0usize, Vec::new());
    let mut weakest: Option<(usize, usize)> = None;

    for path in &files {
        let mut terms = Terms::new();
        let base = load_file(&mut Ast::new(), &mut terms, path)
            .map(|kb| shape(&kb, &terms))
            .map_err(|e| e.0);

        let mut shuffled = permuted(&terms);
        let kernel = Terms::new().syms.len();
        let permutable = terms.syms.len() - kernel;
        let stayed = (kernel..terms.syms.len())
            .filter(|&i| {
                shuffled.syms.get(terms.syms.text(Symbol(i as u32))) == Some(Symbol(i as u32))
            })
            .count();
        if permutable > 0 && weakest.is_none_or(|(m, p)| (permutable - stayed) * p < m * permutable)
        {
            weakest = Some((permutable - stayed, permutable));
        }

        let again = load_file(&mut Ast::new(), &mut shuffled, path)
            .map(|kb| shape(&kb, &shuffled))
            .map_err(|e| e.0);
        match (&base, &again) {
            (Ok(a), Ok(b)) => {
                loaded += 1;
                if a != b {
                    bad.push(format!("{}\n{}", path.display(), first_difference(a, b)));
                }
            }
            (Err(a), Err(b)) => {
                rejected += 1;
                if a != b {
                    bad.push(format!("{}\n  plain: {a}\n  permuted: {b}", path.display()));
                }
            }
            _ => bad.push(format!("{}: loaded in only one id space", path.display())),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} corpus files move when the ids do:\n\n{}",
        bad.len(),
        files.len(),
        bad.iter().take(5).cloned().collect::<Vec<_>>().join("\n\n")
    );
    assert!(
        loaded >= 60 && rejected >= 20,
        "{loaded} loaded, {rejected} rejected"
    );
    // A permutation that permuted nothing compares a run against itself.
    let (moved, permutable) = weakest.expect("some file interned a name");
    assert!(
        moved * 2 >= permutable,
        "the weakest permutation moved {moved} of {permutable} ids"
    );
}

fn first_difference(got: &str, want: &str) -> String {
    for (i, (a, b)) in got.lines().zip(want.lines()).enumerate() {
        if a != b {
            return format!("  line {}\n    plain:    {a}\n    permuted: {b}", i + 1);
        }
    }
    format!(
        "  same {} lines, then {} vs {}",
        got.lines().count().min(want.lines().count()),
        got.lines().count(),
        want.lines().count()
    )
}

/// The two canonical puzzles' registries and cross-reference indexes hold
/// exactly what the source declared — the content the shape digest above
/// freezes, spelled out where a diff can be read.
///
/// The subject is the *derived* half of a load, the part no line of source
/// states outright: which relations are open-world (nobody declares
/// `symmetric`; it vivifies because a property fact names it), which rules a
/// relation is subject to (a union of "named in a pattern" and "named by an
/// application fact"), and which facts are rule applications (any fact whose
/// head is a rule name). Those three indexes are what the saturator plans
/// against, so an error in them is a wrong answer rather than a wrong report.
#[test]
fn the_zebra_registries_and_indexes_hold_what_the_puzzle_declared() {
    let mut terms = Terms::new();
    let kb = load_file(
        &mut Ast::new(),
        &mut terms,
        &repo_root().join("examples/zebra.ein"),
    )
    .expect("zebra.ein loads");
    let sym = |n: &str| terms.syms.get(n).unwrap_or_else(|| panic!("{n} interned"));

    let declared: Vec<&str> = {
        let mut v: Vec<&str> = kb
            .program()
            .relations
            .iter()
            .filter(|(_, r)| r.declared)
            .map(|(n, _)| terms.sym(n))
            .collect();
        v.sort();
        v
    };
    assert_eq!(
        declared,
        ["co-located", "instance", "next-to", "right-of", "type"],
        "five declared relations; `type` / `instance` are ordinary ones here"
    );
    let open_world: Vec<&str> = kb
        .program()
        .relations
        .iter()
        .filter(|(_, r)| !r.declared)
        .map(|(n, _)| terms.sym(n))
        .collect();
    for tag in ["symmetric", "includes", "slot-partition", "slot-spatial"] {
        assert!(open_world.contains(&tag), "{tag} should have vivified");
    }
    assert!(
        !open_world.contains(&"instance"),
        "a declared relation never appears as open-world"
    );
    assert_eq!(
        kb.program()
            .relations
            .get(sym("co-located"))
            .expect("declared")
            .signature
            .iter()
            .map(|&s| terms.sym(s))
            .collect::<Vec<_>>(),
        ["Attribute", "Attribute"],
        "a signature is opaque type-name atoms; nothing resolves them"
    );

    // Which rules a relation is subject to — the union index.
    let rules_of = |rel: &str| {
        let mut v: Vec<&str> = kb
            .rules_of_relation(sym(rel))
            .iter()
            .map(|&r| terms.sym(r))
            .collect();
        v.sort();
        v
    };
    assert_eq!(rules_of("right-of"), ["includes"]);
    assert_eq!(rules_of("next-to"), ["includes", "symmetric"]);

    // Rule applications, by rule — the authored activators.
    let apps_of = |rule: &str| rendered(&terms, kb.rule_apps_by_rule(sym(rule)));
    assert_eq!(
        apps_of("symmetric"),
        ["(symmetric co-located)", "(symmetric next-to)"]
    );
    assert_eq!(apps_of("includes"), ["(includes right-of next-to)"]);
    for reflective in ["slot-locate", "slot-occupied", "slot-elimination"] {
        assert!(
            apps_of(reflective).is_empty(),
            "{reflective} is activated reflectively, so it carries none at load"
        );
    }
    // A fact is a rule application iff its head names a rule.
    let is_app = |compact: &str| {
        let f = fact(&terms, &kb, compact);
        kb.rule_apps_by_rule(terms.facts.rel(f)).any(|a| a == f)
    };
    assert!(is_app("(symmetric co-located)"));
    assert!(!is_app("(co-located Norwegian House-1)"));

    // The slot machinery's two carrier facts, whose arguments *are* the
    // property: one partition, two spatial relations.
    let spatial: Vec<String> = kb
        .facts_of(sym("slot-spatial"))
        .map(|f| terms.display(terms.facts.args(f)[1]))
        .collect();
    assert_eq!(spatial.len(), 2);
    assert!(spatial.contains(&"right-of".to_string()) && spatial.contains(&"next-to".to_string()));
    assert_eq!(
        rendered(&terms, kb.facts_of(sym("slot-partition"))),
        ["(slot-partition co-located instance type Attribute House)"]
    );

    // zebra2 states membership with `is-a` alone.
    let mut terms = Terms::new();
    let kb = load_file(
        &mut Ast::new(),
        &mut terms,
        &repo_root().join("examples/zebra2.ein"),
    )
    .expect("zebra2.ein loads");
    let heads: Vec<&str> = kb
        .facts()
        .map(|f| terms.sym(terms.facts.rel(f)))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    assert!(heads.contains(&"is-a"));
    assert!(!heads.contains(&"type") && !heads.contains(&"instance"));
    assert!(
        kb.program()
            .relations
            .get(terms.syms.get("is-a").expect("interned"))
            .is_some_and(|r| r.declared)
    );

    // An undeclared head vivifies open-world, and a re-derived fact lands in
    // the extent exactly once — the F-KB-3 regression, which the saturator's
    // firing-key dedup used to mask.
    let (mut terms, mut kb) = kb_of("(is-a A Foo)\n(mystery-relation A B :source \"(1)\")\n");
    let mystery = terms.syms.get("mystery-relation").expect("vivified");
    assert!(
        kb.program()
            .relations
            .get(mystery)
            .is_some_and(|r| !r.declared)
    );
    assert_eq!(kb.facts_of(mystery).count(), 1);
    let args: Vec<Value> = ["A", "B"]
        .iter()
        .map(|a| terms.value_text(a).expect("room"))
        .collect();
    let again = kb
        .add_and_index_fact(&mut terms, mystery, &args, None)
        .expect("room");
    assert!(!again.is_new(), "the second derivation deduplicates");
    assert_eq!(kb.facts_of(mystery).count(), 1, "and indexes once");
    assert_eq!(kb.facts().filter(|&f| f == again.id()).count(), 1);
}

/// `examples/zebra.ein` loads to 71 facts, 18 of them given and none derived.
///
/// The count is the puzzle's own arithmetic and is worth spelling out because
/// most of it is *not* in the file as facts: 5 relation declarations each also
/// emit a companion `(relation R)` membership fact, and those cover exactly the
/// declared relations — an auto-vivified property-tag carrier gets no
/// declaration, so it gets no membership fact either. That asymmetry is the
/// part a refactor breaks: vivify a membership fact for `symmetric` and the
/// count silently becomes 75 with nothing else looking wrong.
///
/// "None derived" is the other half, and it is the load-time invariant the
/// whole trace rests on: everything the engine later attributes to a rule was
/// put there by the engine.
#[test]
fn zebra_loads_seventy_one_facts_eighteen_of_them_given_and_none_derived() {
    let mut terms = Terms::new();
    let kb = load_file(
        &mut Ast::new(),
        &mut terms,
        &repo_root().join("examples/zebra.ein"),
    )
    .expect("zebra.ein loads");

    assert_eq!(kb.n_facts(), 71);
    let kinds = |want: ProvKind, with_source: bool| {
        kb.facts()
            .filter(|&f| {
                kb.primary(f).is_some_and(|p| {
                    let prov = terms.provs.get(p);
                    prov.kind == want && prov.source.is_some() == with_source
                })
            })
            .count()
    };
    assert_eq!(
        kinds(ProvKind::Source, true),
        18,
        "the 14 numbered conditions plus condition (1)'s four spatial facts"
    );
    assert_eq!(
        kb.facts()
            .filter(|&f| kb
                .primary(f)
                .is_some_and(|p| terms.provs.get(p).kind != ProvKind::Source))
            .count(),
        0,
        "nothing is derived before the engine runs"
    );
    let rules: Vec<Symbol> = kb.program().rules.keys().collect();
    assert!(
        rules.len() >= 10,
        "std.slots and std.algebra brought rules in"
    );
    for r in rules {
        assert_eq!(
            kb.all_facts(&terms).by_rule(r).count(),
            0,
            "{} attributed a fact at load time",
            terms.sym(r)
        );
    }

    let membership: Vec<String> = {
        let relation = terms.syms.get("relation").expect("interned");
        let mut v: Vec<String> = kb
            .facts_of(relation)
            .filter(|&f| terms.facts.args(f).len() == 1)
            .map(|f| terms.display(terms.facts.args(f)[0]))
            .collect();
        v.sort();
        v
    };
    assert_eq!(
        membership,
        ["co-located", "instance", "next-to", "right-of", "type"],
        "one companion fact per *declaration*, and none for a vivified tag"
    );
}

/// A snapshot still resolves its own premises after the source has moved on.
///
/// This is what makes a satisfying branch reportable: `solve` records the
/// branch's KB and keeps searching, so by the time anything renders the
/// explanation the root has grown facts and rebuilt its indexes underneath.
/// The premise ids in a provenance record are *propositions*, not pointers,
/// and every walk resolves them against belief — so the failure mode is not a
/// dangling reference but a **changed answer**: a snapshot that shared the
/// source's layers would report the frontier of a KB the branch never had.
#[test]
fn a_snapshot_still_resolves_its_premises_after_the_source_moves_on() {
    let (mut terms, mut kb) = kb_of(
        r#"
        (relation p T T) (relation q T T)
        (p a b :source "(1)")
        (q a b :rule p-to-q :using (via (p a b)))
        "#,
    );
    let derived = fact(&terms, &kb, "(q a b)");
    let snap = kb.snapshot();
    let before = rendered(
        &terms,
        build_derivation_dag(&snap, &terms, derived, Justifications::Primary)
            .sources(&snap, &terms)
            .into_iter(),
    );
    assert_eq!(before, ["(p a b)"]);
    let n_at_snapshot = snap.n_facts();

    // The source moves on: a new source fact, a second derivation of `q` that
    // rests on it, and a full index rebuild.
    let p = terms.intern_text("p").expect("room");
    let q = terms.intern_text("q").expect("room");
    let (c, d) = (
        terms.value_text("c").expect("room"),
        terms.value_text("d").expect("room"),
    );
    let prov = terms.provs.push(Prov::from_source(None, None));
    let new = kb
        .add_and_index_fact(&mut terms, p, &[c, d], Some(prov))
        .expect("room")
        .id();
    let late = terms.intern_text("late").expect("room");
    let prov = terms
        .provs
        .push(Prov::from_rule(late, Box::new([new]), None));
    kb.add_and_index_fact(&mut terms, q, &[c, d], Some(prov))
        .expect("room");
    kb.rebuild_indexes(&terms);

    assert_eq!(snap.n_facts(), n_at_snapshot, "the snapshot did not grow");
    assert!(!snap.contains(new), "and does not believe the new premise");
    assert!(kb.contains(new), "…which the source does");
    assert_eq!(
        rendered(
            &terms,
            build_derivation_dag(&snap, &terms, derived, Justifications::Primary)
                .sources(&snap, &terms)
                .into_iter(),
        ),
        before,
        "the snapshot's derivation walk is unchanged"
    );
}
