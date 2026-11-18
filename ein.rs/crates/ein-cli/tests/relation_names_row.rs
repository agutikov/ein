//! **A relation's name the blind enumerator binds as an object is one
//! sentence, on every surface that reports the universe.**
//!
//! Where the blind enumerator built H and a candidate names a relation only a
//! rule concludes, the candidate stays and the read-out says so
//! (`ein_infer::relation_names`): every question prints the same `relation
//! names` row under `universe`, `ein oracle` and its dump say it, and the
//! summary carries it as `root.hypgen.relation_names_as_objects`. A program
//! with none prints no row and writes no key, so its read-out and its summary
//! are what they are without this.
//!
//! ```text
//! cargo test -p ein-cli --test relation_names_row
//! ```

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

/// `liquid` heads the fact the rule `liquids` derives; no file declares it
/// or writes a fact of it, so it is an object, and `(tasty liquid)` a
/// candidate beside `(tasty Drink)` and `(tasty Tea)`.
const DERIVED: &str = "\
(relation is-a Thing Type :why \"{?3} is a {?4}\")
(is-a Tea Drink)
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{?x} is a drink, so a liquid\")
(relation tasty Thing :why \"{?3} is tasty\")
(solve :no-hypothesis (is-a))
";

/// The same program with `liquid` declared, and scoped out as `is-a` is: a
/// declared relation is no object.
const DECLARED: &str = "\
(relation is-a Thing Type :why \"{?3} is a {?4}\")
(is-a Tea Drink)
(relation liquid Thing :why \"{?3} is a liquid\")
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{?x} is a drink, so a liquid\")
(relation tasty Thing :why \"{?3} is tasty\")
(solve :no-hypothesis (is-a liquid))
";

const SENTENCE: &str = "liquid — a relation only a rule concludes, and 1 candidate of tasty names \
                        it as an object: no file declares it or writes a fact of it";

fn run(args: &[&str]) -> (Option<i32>, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// The `relation names` rows a read-out prints.
fn rows(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.strip_prefix("    relation names  "))
        .map(str::to_string)
        .collect()
}

fn json(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("the file was written"))
        .expect("it parses")
}

fn strings(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .expect("a list")
        .iter()
        .map(|o| o.as_str().expect("a name").to_string())
        .collect()
}

/// Every question prints the row, the summary carries it, and `ein oracle`
/// and its dump say it; the program that declares the relation prints no row
/// and writes no key.
#[test]
fn a_relations_name_bound_as_an_object_is_one_sentence_on_every_surface() {
    let tmp = Scratch::new("ein-relation-names");
    for (name, src, named) in [("derived", DERIVED, true), ("declared", DECLARED, false)] {
        let file = tmp.join(format!("{name}.ein"));
        std::fs::write(&file, src).expect("the program is written");
        let file = file.to_str().expect("a UTF-8 path");
        let want: Vec<String> = if named {
            vec![SENTENCE.to_string()]
        } else {
            Vec::new()
        };
        let questions: [&[&str]; 8] = [
            &[],
            &["--maximal"],
            &["--minimal"],
            &["--single"],
            &["-n", "1"],
            &["--count"],
            &["--monotone"],
            &["--no-search"],
        ];
        for (i, flags) in questions.iter().enumerate() {
            let summary = tmp.join(format!("{name}-{i}.json"));
            let mut args = vec!["solve", file];
            args.extend_from_slice(flags);
            args.extend_from_slice(&["--json-summary", summary.to_str().expect("UTF-8")]);
            let (code, out) = run(&args);
            assert_eq!(code, Some(0), "ein {args:?}:\n{out}");
            assert_eq!(rows(&out), want, "ein solve {name} {flags:?}:\n{out}");
            let hypgen = &json(&summary)["root"]["hypgen"];
            let key = &hypgen["relation_names_as_objects"];
            if named {
                assert_eq!(strings(&key["objects"]), ["liquid"], "{name} {flags:?}");
                assert_eq!(strings(&key["relations"]), ["tasty"], "{name} {flags:?}");
                assert_eq!(key["candidates"].as_u64(), Some(1), "{name} {flags:?}");
            } else {
                assert!(
                    key.is_null(),
                    "{name} {flags:?}: a program with none writes no key: {hypgen}"
                );
            }
        }
        let dump = tmp.join(format!("{name}-cube"));
        let (code, out) = run(&["oracle", file, "--dump", dump.to_str().expect("UTF-8")]);
        assert_eq!(code, Some(0), "ein oracle {name}:\n{out}");
        assert_eq!(rows(&out), want, "ein oracle {name}:\n{out}");
        let md = std::fs::read_to_string(dump.join("region.md")).expect("region.md");
        assert_eq!(
            md.contains(&format!("**Relation names as objects**: {SENTENCE}.")),
            named,
            "{name}: region.md:\n{md}"
        );
        assert_eq!(
            md.contains("**Relation names as objects**"),
            named,
            "{name}: region.md"
        );
        let region = json(&dump.join("region.json"));
        let key = &region["relation_names_as_objects"];
        if named {
            assert_eq!(strings(&key["objects"]), ["liquid"], "{name}: region.json");
            assert_eq!(key["candidates"].as_u64(), Some(1), "{name}: region.json");
        } else {
            assert!(key.is_null(), "{name}: region.json writes no key");
        }
    }
}

/// **The corpus's one program**: `features/01_not_and_absent.ein` derives
/// `explicitly-dislikes` by its rule `see-stored-neg` and declares it
/// nowhere, and the row counts the candidates that name it. `--monotone`
/// reads root and H and runs no search, so the count is of H whole.
#[test]
fn the_corpus_program_that_derives_an_undeclared_relation_is_named() {
    let tmp = Scratch::new("ein-relation-names-corpus");
    let summary = tmp.join("summary.json");
    let (code, out) = run(&[
        "solve",
        "examples/features/01_not_and_absent.ein",
        "--monotone",
        "--json-summary",
        summary.to_str().expect("UTF-8"),
    ]);
    assert_eq!(code, Some(0), "{out}");
    let key = &json(&summary)["root"]["hypgen"]["relation_names_as_objects"];
    assert_eq!(strings(&key["objects"]), ["explicitly-dislikes"]);
    let n = key["candidates"].as_u64().expect("a count");
    let of = strings(&key["relations"]).join(", ");
    assert_eq!(
        rows(&out),
        [format!(
            "explicitly-dislikes — a relation only a rule concludes, and {n} candidates of {of} \
             name it as an object: no file declares it or writes a fact of it"
        )],
        "{out}"
    );
    // The row is about H, and says nothing of what else H holds.
    assert!(
        n > 0
            && n < json(&summary)["root"]["hypgen"]["emitted"]
                .as_u64()
                .expect("H's size"),
        "{n} of H's candidates"
    );
}
