//! **`ein saturate --dump` lists every rule of the program, each kind in a
//! section of its own.**
//!
//! A program's rules are of three kinds, and the dump has a section for each
//! kind the program loads a rule of:
//!
//! | heading | the rules | beside each |
//! |---|---|---|
//! | `;; Rules — the saturation agenda's (N)` | the rules a saturation fires | its stratum, or `no plan compiled` where the run compiled none of it |
//! | `;; Obligation rules (N)` | the rules read on the fixpoint, which derive nothing | how many facts headed by its name the saturated knowledge base holds for its parameters |
//! | `;; Hypothesis rules (N)` | the rules that propose the candidates, which no saturation reads | how many tuples `(solve :hrules …)` writes for its parameters |
//!
//! Every line is the dump's one form for a rule, `;;   NAME  :params (…)`
//! and then what the section says of it. A rule without parameters is bound
//! by no activator, and an obligation rule or a hypothesis rule without them
//! has nothing after its parameters. A rule stands in one section, and a
//! heading's count is the number of lines under it.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const AGENDA: &str = ";; Rules — the saturation agenda's";
const OBLIGATION: &str = ";; Obligation rules";
const HYPOTHESIS: &str = ";; Hypothesis rules";

fn dump(file: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["saturate", file, "--dump"])
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    assert!(
        out.status.success(),
        "ein saturate {file} --dump: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A rule section of the dump: the count its heading states and the lines
/// under it, each without its `;;   ` margin. `None` where the dump has no
/// such section.
fn section(dump: &str, heading: &str) -> Option<(usize, Vec<String>)> {
    let mut lines = dump.lines().skip_while(|l| !l.starts_with(heading));
    let head = lines.next()?;
    let count = head
        .strip_prefix(heading)
        .and_then(|rest| rest.trim().strip_prefix('('))
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("the heading states no count: {head:?}"));
    let rules = lines
        .take_while(|l| l.starts_with(";;   "))
        .map(|l| l[5..].to_string())
        .collect();
    Some((count, rules))
}

/// The lines of a section the dump must have, its count held to them.
fn rules(dump: &str, heading: &str) -> Vec<String> {
    let (count, rules) =
        section(dump, heading).unwrap_or_else(|| panic!("no section {heading:?}:\n{dump}"));
    assert_eq!(count, rules.len(), "{heading}: its count and its lines");
    rules
}

/// A line's rule name.
fn name(line: &str) -> &str {
    line.split("  ")
        .next()
        .expect("a rule line starts with a name")
}

/// **The seats program**: its two obligation rules stand in their section,
/// each with the one activator the program states, and in no other; the
/// agenda's section is headed as the agenda's; and a program with no
/// hypothesis rule has no section for one.
#[test]
fn an_obligation_rule_is_listed_in_a_section_of_its_own() {
    let out = dump("examples/lattice/05_owed_seats_bounded.ein");
    assert_eq!(
        rules(&out, OBLIGATION),
        [
            "total-owed  :params (?R ?isa)  1 activator",
            "surjective-owed  :params (?R ?isa)  1 activator",
        ]
    );
    let agenda = rules(&out, AGENDA);
    assert!(
        agenda.contains(&"functional  :params (?R)  stratum 0".to_string()),
        "{agenda:?}"
    );
    for line in &agenda {
        assert!(
            !name(line).ends_with("-owed"),
            "an obligation rule in the agenda's section: {line}"
        );
    }
    assert!(section(&out, HYPOTHESIS).is_none(), "{out}");
    assert!(
        !out.lines().any(|l| l.starts_with(";; Rules (")),
        "a rule section headed as every rule's:\n{out}"
    );
}

/// **An obligation rule a program loads and nothing activates** is listed
/// with no activator: the program imports the module for `total-owed`, and
/// the module's other obligation rule comes with it.
#[test]
fn an_obligation_rule_nothing_activates_says_so() {
    let out = dump("tests/region/04_owing.ein");
    assert_eq!(
        rules(&out, OBLIGATION),
        [
            "total-owed  :params (?R ?isa)  1 activator",
            "std.algebra.surjective-owed  :params (?R ?isa)  no activator",
        ]
    );
}

/// **A hypothesis rule with parameters is listed with the tuples its
/// `(solve :hrules …)` writes for it**, and an obligation rule with the
/// activators the saturated knowledge base holds, here the two a rule
/// derives at root and no file states.
#[test]
fn a_hypothesis_rule_is_listed_with_its_activators() {
    let scratch = Scratch::new("ein-saturate-dump-activators");
    let file = scratch.path().join("activators.ein");
    std::fs::write(
        &file,
        r#"(relation is-a X T :why "{?3} is a {?4}")
(relation seat Guest Chair :why "{?3} sits on {?4}")
(relation near Guest Chair :why "{?3} is near {?4}")
(is-a Ann Guest) (is-a C1 Chair)
(rule filled (?R)
  :match  (and (is-a ?c Chair) (absent (?R ?g ?c)))
  :assert (open ?R)
  :why    "{?c} is empty.")
(rule every-relation-is-filled ()
  :match  (relation ?R Guest Chair)
  :assert (filled ?R)
  :why    "{?R} leaves no chair empty.")
(hrule guess-of (?R)
  :match  (and (is-a ?g Guest) (is-a ?c Chair))
  :assert (?R ?g ?c)
  :why    "suppose {?R} of {?g} and {?c}.")
(solve :hrules (guess-of (seat) (near)))
"#,
    )
    .expect("the program is written");
    let out = dump(&file.display().to_string());
    assert_eq!(
        rules(&out, HYPOTHESIS),
        ["guess-of  :params (?R)  2 activators"]
    );
    assert_eq!(
        rules(&out, OBLIGATION),
        ["filled  :params (?R)  2 activators"]
    );
    // The program's rule, and the one every program gets.
    assert_eq!(
        rules(&out, AGENDA),
        [
            "every-relation-is-filled  :params ()  stratum 0",
            "std.core.inconsistent  :params ()  :ein inconsistent"
        ]
    );
}

/// **A rule without parameters has nothing after them** in the two sections
/// that count activators, and a hypothesis rule with parameters for which
/// the program writes no `:hrules` tuple has no activator. Every rule of the
/// program stands in exactly one section.
#[test]
fn every_rule_stands_in_one_section() {
    let scratch = Scratch::new("ein-saturate-dump-rules");
    let file = scratch.path().join("kinds.ein");
    std::fs::write(
        &file,
        r#"(relation is-a X T :why "{?3} is a {?4}")
(relation seat Guest Chair :why "{?3} sits on {?4}")
(relation taken Chair :why "{?3} is taken")
(is-a Ann Guest) (is-a C1 Chair)
(rule takes ()
  :match  (seat ?g ?c)
  :assert (taken ?c)
  :why    "{?g} sits on {?c}, so it is taken.")
(rule seated ()
  :match  (and (is-a ?g Guest) (absent (seat ?g ?c)))
  :assert (open seat)
  :why    "{?g} has no chair.")
(rule filled (?R)
  :match  (and (is-a ?c Chair) (absent (?R ?g ?c)))
  :assert (open ?R)
  :why    "{?c} is empty.")
(hrule guess ()
  :match  (and (is-a ?g Guest) (is-a ?c Chair))
  :assert (seat ?g ?c)
  :why    "suppose {?g} sits on {?c}.")
(hrule guess-of (?R)
  :match  (and (is-a ?g Guest) (is-a ?c Chair))
  :assert (?R ?g ?c)
  :why    "suppose {?R} of {?g} and {?c}.")
(filled seat)
"#,
    )
    .expect("the program is written");
    let out = dump(&file.display().to_string());
    let agenda = rules(&out, AGENDA);
    let obligation = rules(&out, OBLIGATION);
    let hypothesis = rules(&out, HYPOTHESIS);
    // The program's rule, and the one every program gets.
    assert_eq!(
        agenda,
        [
            "takes  :params ()  stratum 0",
            "std.core.inconsistent  :params ()  :ein inconsistent"
        ]
    );
    assert_eq!(
        obligation,
        ["seated  :params ()", "filled  :params (?R)  1 activator"]
    );
    assert_eq!(
        hypothesis,
        ["guess  :params ()", "guess-of  :params (?R)  no activator"]
    );
    let mut listed: Vec<&str> = agenda
        .iter()
        .chain(&obligation)
        .chain(&hypothesis)
        .map(|l| name(l))
        .collect();
    listed.sort_unstable();
    assert_eq!(
        listed,
        [
            "filled",
            "guess",
            "guess-of",
            "seated",
            "std.core.inconsistent",
            "takes"
        ]
    );
}
