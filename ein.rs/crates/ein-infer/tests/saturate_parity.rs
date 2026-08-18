//! S1a.3.3 acceptance — **T2**: the two engines take the same steps.
//!
//! The comparison is the `--events` protocol itself
//! ([`conformance/EVENTS.md`](../../../../conformance/EVENTS.md)) at
//! `verbose`, so every firing is reported including the redundant ones: a
//! dropped redundant firing is exactly the kind of difference a port
//! introduces, and it is invisible at `normal`.
//!
//! `n` is compared as a **position, not a field**. One extra event on either
//! side renumbers every line after it, and a differ that reported all of them
//! would bury the one difference that caused them — so the first *structural*
//! difference is what gets named, with its four preceding events for context.

use ein_core::Terms;
use ein_ir::{Ast, load_file};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use std::path::Path;

fn rust_events(path: &Path) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(
        match ein_infer::saturate_events(&ast, &mut terms, &mut kb) {
            Ok(log) => Answer::Ok(log),
            Err(e) => Answer::Err {
                kind: "SaturateError".into(),
                msg: e.to_string(),
            },
        },
    )
}

#[test]
fn the_whole_corpus_saturates_the_same_way() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_whole_corpus_saturates_the_same_way");
    };
    let (mut bad, mut compared, mut events) = (Vec::new(), 0, 0usize);
    for path in &corpus_files() {
        let Some(got) = rust_events(path) else {
            continue;
        };
        let want = py.file("saturate-events", path);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                compared += 1;
                events += a.lines().count();
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { .. }, Answer::Err { .. }) => {}
            _ => bad.push(format!(
                "{name}\n  rs: {}\n  py: {}",
                brief(&got),
                brief(&want)
            )),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} files differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("T2: {compared} files, {events} events, 0 differences");
    assert!(
        compared >= 50 && events >= 3000,
        "only {compared} files / {events} events compared"
    );
}

fn brief(a: &Answer) -> String {
    match a {
        Answer::Ok(s) => format!("{} events", s.lines().count()),
        Answer::Err { kind, msg } => format!("{kind}: {msg}"),
    }
}

/// The first differing line, with the four before it from each side — the
/// context `ein-conformance diff` prints, for the same reason.
fn first_difference(a: &str, b: &str) -> String {
    let (ours, theirs): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for (i, (x, y)) in ours.iter().zip(theirs.iter()).enumerate() {
        if x != y {
            let from = i.saturating_sub(4);
            let context: Vec<String> = ours[from..i].iter().map(|l| format!("    {l}")).collect();
            return format!(
                "  at event {i}:\n{}\n  ein.py: {y}\n  ein.rs: {x}",
                context.join("\n")
            );
        }
    }
    let (extra, side) = if ours.len() > theirs.len() {
        (ours.get(theirs.len()), "ein.rs")
    } else {
        (theirs.get(ours.len()), "ein.py")
    };
    format!(
        "  same prefix, different length: ein.py {} events, ein.rs {}\n  \
         first extra ({side}): {extra:?}",
        theirs.len(),
        ours.len(),
    )
}
