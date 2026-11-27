//! **The catalogue holds what it states**: every entry program is loaded and
//! asked through the bounded call, its root built through
//! `einaf::kernel::root`, and held to the entry's stated effect: the library
//! rules root fires or owes, the writer of H, the outcome under `--single`
//! and the answer query's lines; every selecting fact of it active by the
//! contract's check; the rule whose words the prompt quotes present in the
//! generated part; the text with no digest of the library; and the catalogue
//! the grammars are rendered on narrower than the library's.

use std::collections::BTreeSet;
use std::time::Duration;

use einaf::catalogue::{self, Catalogue, ENTRIES, Size};
use einaf::check;
use einaf::grammar::steps;
use einaf::kernel::{self, Bounds, Outcome, Part, Question, Source};

fn bounds() -> Bounds {
    Bounds {
        clock: Some(Duration::from_secs(60)),
        ..Bounds::default()
    }
}

#[test]
fn every_entry_is_run_by_the_kernel_and_shows_its_stated_effect() {
    let generated = catalogue::generated();
    let whole = Catalogue::new(Size::Whole);
    for e in ENTRIES {
        let source = Source::Text {
            text: e.program,
            name: e.id,
            base_dir: None,
        };
        let f = kernel::call(source, &[Question::Single(Part::All)], bounds())
            .pop()
            .expect("one object");
        let outcome = f.outcome.as_ref().expect("an outcome")[0].outcome;
        let expected = match e.outcome {
            "one" => Outcome::One,
            "inconsistent" => Outcome::Inconsistent,
            other => panic!("{}: an outcome the test does not read: {other}", e.id),
        };
        assert_eq!(outcome, expected, "{}: the outcome under --single", e.id);
        let lines: Vec<String> = f
            .queries
            .iter()
            .flatten()
            .filter(|q| q.goal_text.is_some())
            .flat_map(|q| q.classes.iter().flat_map(|c| c.lines.clone()))
            .collect();
        assert_eq!(lines, e.answer, "{}: the answer query's lines", e.id);

        let root = kernel::root(source, bounds());
        assert!(
            root.refused.is_empty(),
            "{}: refused {:?}",
            e.id,
            root.refused
        );
        assert_eq!(
            root.writer.as_deref(),
            Some(e.writer),
            "{}: the writer of H",
            e.id
        );
        let named: BTreeSet<&str> = root
            .fired
            .iter()
            .chain(root.owed.iter())
            .map(|f| f.rule.as_str())
            .collect();
        for rule in e.fires {
            assert!(
                named.contains(rule),
                "{}: root names no firing or owe of {rule}: {named:?}",
                e.id
            );
        }

        // Every selection of the entry is active, by the check's two readings.
        let checked = check::check(&check::Input {
            program: e.program,
            name: e.id,
            step: 3,
            text: "",
            catalogue: &whole,
            generated: &generated,
            step1: None,
            root: &root,
        });
        let selection: Vec<_> = checked
            .findings
            .iter()
            .filter(|f| f.kind.number() == 2 || f.kind.number() == 3)
            .collect();
        assert!(selection.is_empty(), "{}: {selection:#?}", e.id);

        if let Some(rule) = e.rule {
            assert!(
                generated.rule(e.module, rule).is_some()
                    || generated.rule("std.algebra", rule).is_some(),
                "{}: the generated part has no rule {rule} of {}",
                e.id,
                e.module
            );
        }
    }
}

#[test]
fn the_catalogue_s_text_is_cut_from_the_entry_programs() {
    let whole = Catalogue::new(Size::Whole);
    let minimal = Catalogue::new(Size::Minimal);
    let text = whole.text();
    for e in ENTRIES {
        assert!(
            text.contains(&e.shown()),
            "{}: its lines are not shown",
            e.id
        );
        assert!(
            !e.shown().contains(['`', '<', '>', '|']),
            "{}: a program a prompt shows holds a character no grammar lets the model write",
            e.id
        );
    }
    // The library's digest is the record's, and no prompt shows it: a
    // change to the library moves what the model is shown only where it
    // moves the own words of a rule the text quotes.
    let manifest = whole.manifest.as_deref().expect("the library's digest");
    assert!(
        !text.contains(manifest),
        "the catalogue's text shows the library's digest"
    );
    assert!(
        !text.contains("sha256:"),
        "the catalogue's text shows a digest"
    );
    assert!(minimal.entries.iter().all(|e| e.minimal()));
    assert!(minimal.entries.len() < whole.entries.len());
    assert!(minimal.text().len() < text.len());
}

#[test]
fn the_grammars_catalogue_is_the_entries_and_narrower_than_the_library() {
    let narrow = Catalogue::new(Size::Whole).grammar();
    let library = steps::Catalogue::library();
    let modules: BTreeSet<&str> = narrow.modules.iter().map(|m| m.module.as_str()).collect();
    assert_eq!(
        modules,
        [
            "std.algebra",
            "std.bijection",
            "std.contradiction",
            "std.math.arith",
            "std.math.decimal",
            "std.math.decimal-arith",
            "std.slots",
        ]
        .into_iter()
        .collect()
    );
    for m in &narrow.modules {
        let whole = library.module(&m.module).expect("a module of the library");
        assert!(m.symbols.is_subset(&whole.symbols), "{}", m.module);
        assert!(m.symbols.len() <= whole.symbols.len());
    }
    let algebra = narrow.module("std.algebra").expect("std.algebra");
    for a in [
        "transitive",
        "symmetric",
        "includes",
        "converse",
        "compose",
        "top",
        "meet",
        "functional",
        "injective",
        "total-owed",
    ] {
        assert!(algebra.activators.contains_key(a), "{a}");
    }
    assert_eq!(algebra.activators.len(), 10);
    let slots = narrow.module("std.slots").expect("std.slots");
    assert_eq!(
        slots
            .unlisted
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["slot-partition", "slot-spatial"]
    );
    let calc = narrow
        .module("std.math.decimal-arith")
        .expect("the calculator");
    assert!(calc.values.contains("gt"));
    assert_eq!(calc.symbols.len(), 10);
}

#[test]
fn the_generated_part_holds_every_module_and_its_rules() {
    let g = catalogue::generated();
    assert!(
        g.manifest
            .as_deref()
            .is_some_and(|m| m.starts_with("sha256:"))
    );
    assert!(g.modules.iter().any(|m| m.module == "std.core"));
    let t = g.rule("std.algebra", "transitive").expect("transitive");
    assert_eq!(t.params, ["rel"]);
    assert!(g.is_check("functional"));
    assert!(!g.is_check("transitive"));
}
