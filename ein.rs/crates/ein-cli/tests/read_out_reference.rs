//! [`docs/kernel/inference/read_out.md`] — the read-out reference, pinned.
//!
//! M1f [S1f.5.16](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md),
//! T1f.5.16.3. The page defines every row `ein solve` and `ein oracle` print:
//! its label, what it says, what its numbers count, and the summary field that
//! holds the same number. A reference page is the most drift-prone shape there
//! is, so its labels, its summary paths and its numbers are held here, both
//! ways, against the printer's own list,
//! [`ein_render::read_out::Row`], and against real runs: the blocks of the
//! examples page, [`docs/kernel/inference/read_out_examples.md`], which
//! `read_out_examples.rs` holds to the binary byte for byte. What no test
//! reads is a row's *meaning*; the page quotes no printed value, so that only
//! that column can drift. [`configuration.md`] with `config_reference.rs` is
//! the precedent.
//!
//! 1. [`every_row_label_has_a_row_and_no_row_is_orphaned`]: each row table
//!    the page marks (`<!-- rows: NAME -->`) lists the labels of
//!    [`Row::TABLES`]' table of that name, in printing order, and every
//!    [`Row`] is in a table.
//! 2. [`every_row_is_printed_by_an_example`]: every label is printed at its
//!    layout by a block of the examples page, each block's rows in their
//!    table's order; the clock rows (`-s`'s `wall`, `-t`'s table) by one run
//!    with the times masked, held to the page's masked block.
//! 3. [`every_answer_field_of_the_summary_has_a_row`]: every leaf path of
//!    `schema`, `source`, `asked`, `region`, `verdict`, `queries` and `claim`
//!    over the examples' summaries is named by the page, and every path the
//!    page names is in a summary; the owners table names every top-level key.
//! 4. [`each_row_prints_its_summary_fields_number`]: on every example run,
//!    every row whose summary column names a field prints that field's
//!    number — *one label, one number*.
//!
//! [`docs/kernel/inference/read_out.md`]: ../../../../docs/kernel/inference/read_out.md
//! [`docs/kernel/inference/read_out_examples.md`]: ../../../../docs/kernel/inference/read_out_examples.md
//! [`configuration.md`]: ../../../../docs/kernel/configuration.md

use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;
use std::sync::OnceLock;

use ein_corpus::{Scratch, repo_root};
use ein_render::read_out::{Layout, Row};
use serde_json::Value as J;

/// The page, at one path, as `embedding.rs` reads `rust.md`: S1f.20.5's
/// rule moves a page and its test together.
const PAGE: &str = "docs/kernel/inference/read_out.md";
const EXAMPLES: &str = "docs/kernel/inference/read_out_examples.md";

/// The seven keys the page defines; the others' owners are in its table.
const ANSWER_KEYS: [&str; 7] = [
    "schema", "source", "asked", "region", "verdict", "queries", "claim",
];

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

// ── The page's tables ───────────────────────────────────────────────

/// The cells of a markdown table row, split at each `|` no backslash escapes.
fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut prev = ' ';
    for c in inner.chars() {
        if c == '|' && prev != '\\' {
            out.push(cur.trim().to_string());
            cur.clear();
        } else {
            cur.push(c);
        }
        prev = c;
    }
    out.push(cur.trim().to_string());
    out
}

/// Every `` `code` `` span of a cell.
fn spans(cell: &str) -> Vec<String> {
    cell.split('`')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, s)| s.to_string())
        .collect()
}

/// The body rows of the table between `<!-- open -->` and `<!-- close -->`.
fn table(page: &str, open: &str, close: &str) -> Vec<Vec<String>> {
    let (_, rest) = page
        .split_once(open)
        .unwrap_or_else(|| panic!("{PAGE} has no `{open}` marker"));
    let (body, _) = rest
        .split_once(close)
        .unwrap_or_else(|| panic!("{PAGE}: `{open}` is not closed"));
    body.lines()
        .filter(|l| l.trim_start().starts_with('|'))
        .skip(2)
        .map(cells)
        .collect()
}

/// A row table: per row, its labels and its summary paths.
struct Entry {
    labels: Vec<String>,
    paths: Vec<String>,
}

fn row_table(page: &str, name: &str) -> Vec<Entry> {
    table(page, &format!("<!-- rows: {name} -->"), "<!-- /rows -->")
        .into_iter()
        .map(|c| {
            assert_eq!(
                c.len(),
                6,
                "{PAGE}, table `{name}`: a row of {} cells: {c:?}",
                c.len()
            );
            Entry {
                labels: spans(&c[0]),
                paths: spans(&c[4]).into_iter().filter(|p| is_path(p)).collect(),
            }
        })
        .collect()
}

/// Whether a code span names a summary field.
fn is_path(s: &str) -> bool {
    let root = s.split(['.', '[']).next().unwrap_or("");
    !s.contains(' ')
        && [
            "schema",
            "source",
            "asked",
            "region",
            "verdict",
            "queries",
            "claim",
            "work",
            "monotonicity",
            "leftover",
            "root",
            "config",
        ]
        .contains(&root)
}

/// A page path with its row indices read as every element: `[k]`, `[q]` and
/// `[i]` are `[]`, and a list a row prints one element of (`region.core[k]`)
/// is the list's path.
fn normal(p: &str) -> String {
    let p = p
        .replace("[k]", "[]")
        .replace("[q]", "[]")
        .replace("[i]", "[]");
    p.strip_suffix("[]").map_or(p.clone(), str::to_string)
}

/// The table of `Row::TABLES` a row is in, for a run of the given question.
fn tables_of(r: Row) -> Vec<&'static str> {
    Row::TABLES
        .iter()
        .filter(|(_, rows)| rows.contains(&r))
        .map(|(n, _)| *n)
        .collect()
}

// ── 1 ───────────────────────────────────────────────────────────────

/// **Every label has a row, and no row is orphaned**, in printing order: each
/// marked table is its [`Row::TABLES`] entry, label for label, and every
/// [`Row`] the printer can print is in a table, so a label added to the
/// printer with no row fails here, and so does a row the printer no longer
/// has.
#[test]
fn every_row_label_has_a_row_and_no_row_is_orphaned() {
    let page = read(PAGE);
    let mut failures = Vec::new();
    for (name, rows) in Row::TABLES {
        let want: Vec<Vec<String>> = rows.iter().map(|r| r.patterns()).collect();
        let got: Vec<Vec<String>> = row_table(&page, name)
            .into_iter()
            .map(|e| e.labels)
            .collect();
        if want != got {
            failures.push(format!(
                "table `{name}`:\n    the printer: {want:?}\n    the page:    {got:?}"
            ));
        }
    }
    for r in Row::ALL {
        if tables_of(*r).is_empty() {
            failures.push(format!("{r:?} (`{}`) is in no table", r.label()));
        }
    }
    let marked = page.matches("<!-- rows: ").count();
    if marked != Row::TABLES.len() {
        failures.push(format!(
            "{marked} marked tables on the page, {} in Row::TABLES",
            Row::TABLES.len()
        ));
    }
    assert!(failures.is_empty(), "{PAGE}:\n{}", failures.join("\n"));
}

// ── The examples, run once ──────────────────────────────────────────

/// One example block's command, run with `--json-summary`.
struct Run {
    caption: String,
    stdout: String,
    summary: J,
}

/// The `(command, args)` of every whole-run block of the examples page.
fn captions(page: &str) -> Vec<(String, Vec<String>)> {
    let lines: Vec<&str> = page.lines().collect();
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let Some(rest) = l.strip_prefix("`ein ") else {
            continue;
        };
        let Some(end) = rest.find('`') else { continue };
        let after = &rest[end + 1..];
        if !(after.is_empty() || after.starts_with(" — ")) || lines.get(i + 2) != Some(&"```text")
        {
            continue;
        }
        let mut words = rest[..end].split_whitespace().map(str::to_string);
        let cmd = words.next().expect("a command");
        let args: Vec<String> = words.collect();
        // The scratch variant is no corpus file; `answer.txt` is a run
        // another block already makes.
        if args.iter().any(|a| a == "--dump-answer")
            || !repo_root().join(args.last().expect("a file")).exists()
        {
            continue;
        }
        out.push((cmd, args));
    }
    out
}

fn ein(args: &[String]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs")
}

/// Every example block's run, with its summary: run once for tests 2–4.
fn runs() -> &'static [Run] {
    static RUNS: OnceLock<Vec<Run>> = OnceLock::new();
    RUNS.get_or_init(|| {
        let tmp = Scratch::new("ein-read-out-reference");
        captions(&read(EXAMPLES))
            .into_iter()
            .enumerate()
            .map(|(n, (cmd, args))| {
                let j = tmp.join(format!("{n}.json"));
                let mut argv = vec![cmd.clone()];
                argv.extend(args.iter().cloned());
                argv.push("--json-summary".into());
                argv.push(j.to_string_lossy().into());
                let out = ein(&argv);
                let summary = std::fs::read_to_string(&j)
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or(J::Null);
                Run {
                    caption: format!("ein {cmd} {}", args.join(" ")),
                    stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
                    summary,
                }
            })
            .collect()
    })
}

/// A row as stdout prints it: its layout, its label, its value. `None` for a
/// line that is no row: the header, the listing, a continuation.
fn parse_row(l: &str) -> Option<(Layout, String, String)> {
    if let Some(rest) = l.strip_prefix("the cube") {
        return Some((Layout::Line, "the cube".into(), rest.trim().into()));
    }
    let (layout, body, width) = match l.strip_prefix("    ") {
        Some(b) => (Layout::Block, b, 16),
        None => (Layout::Row, l.strip_prefix("  ")?, 18),
    };
    // `(solve :expect)` is the one label that opens like the listing.
    let claim = Row::SolveExpect.label();
    if let Some(value) = body.strip_prefix(claim) {
        return Some((layout, claim.to_string(), value.trim_start().to_string()));
    }
    if body.starts_with([' ', '(', ';']) {
        return None;
    }
    // A numbered label, `query 2 of 3`, `class 10 of 12`: its words.
    let w: Vec<&str> = body.splitn(5, ' ').collect();
    let numbered = w.len() >= 4
        && matches!(w[0], "query" | "class")
        && w[2] == "of"
        && [w[1], w[3]]
            .iter()
            .all(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
    let label = if numbered {
        w[..4].join(" ")
    } else {
        // Padded to its width; a longer label is followed by a space.
        let chars: Vec<char> = body.chars().collect();
        let head: String = chars.iter().take(width).collect();
        if chars.len() <= width || head.ends_with(' ') {
            head.trim_end().to_string()
        } else {
            body.split(' ').next().unwrap_or("").to_string()
        }
    };
    let value = body[label.len()..].trim_start().to_string();
    Some((layout, label, value))
}

/// The [`Row`] a printed `(layout, label)` is, `query 2 of 3` and
/// `class 1 of 4` read as their pattern.
fn row_of(layout: Layout, label: &str) -> Option<Row> {
    let pattern = numbered_pattern(label);
    Row::ALL
        .iter()
        .copied()
        .find(|r| r.layout() == layout && r.patterns().contains(&pattern))
}

/// `query 2 of 3` → `query i of n`; anything else as it is.
fn numbered_pattern(label: &str) -> String {
    let w: Vec<&str> = label.split(' ').collect();
    if w.len() == 4
        && w[2] == "of"
        && w[1].chars().all(|c| c.is_ascii_digit())
        && w[3].chars().all(|c| c.is_ascii_digit())
    {
        format!("{} i of n", w[0])
    } else {
        label.to_string()
    }
}

/// The question a run answered, as the page's tables split them.
fn question(s: &J) -> &'static str {
    if s["asked"]["ask"] == "monotone" {
        "monotone"
    } else if !s["region"]["k_s"].is_null() || s["asked"]["ask"] == "count" {
        "describe"
    } else {
        "mode"
    }
}

// ── 2 ───────────────────────────────────────────────────────────────

/// The `wall` and `timing` rows of a run with every time masked.
fn masked(stdout: &str) -> String {
    let mut out = Vec::new();
    let mut on = false;
    for l in stdout.lines() {
        if l.starts_with("  wall") {
            on = true;
        }
        if !on {
            continue;
        }
        // `12.34 ms` → `… ms`, the padding before it to one space.
        let mut s = String::new();
        let mut rest = l;
        while let Some(at) = rest.find(" ms") {
            let head = &rest[..at];
            let digits = head.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
            s.push_str(digits.trim_end());
            s.push_str(" … ms");
            rest = &rest[at + 3..];
        }
        s.push_str(rest);
        out.push(s);
    }
    out.join("\n")
}

/// **Every label is printed by an example**, at its layout, so every row the
/// page defines is one the binary prints in a real run; and each block's rows
/// come in their table's order, which is how the tables' order is held to the
/// printer's. The clock rows are held by one run with the times masked,
/// against the page's masked block.
#[test]
fn every_row_is_printed_by_an_example() {
    let mut printed: BTreeSet<(String, String)> = BTreeSet::new();
    let mut failures = Vec::new();
    for run in runs() {
        let table = question(&run.summary);
        let mut last: BTreeMap<&str, usize> = BTreeMap::new();
        for l in run.stdout.lines().skip(2) {
            let Some((layout, label, _)) = parse_row(l) else {
                continue;
            };
            let Some(r) = row_of(layout, &label) else {
                failures.push(format!("{}: a row no Row is: {l:?}", run.caption));
                continue;
            };
            printed.insert((format!("{:?}", r.layout()), r.patterns()[0].clone()));
            // Order: within each table the row is in, its place never goes
            // back, but to the table's first row (the next query's block).
            for (name, rows) in Row::TABLES {
                if ["describe", "mode", "monotone"].contains(name) && *name != table {
                    continue;
                }
                let from = *last.get(name).unwrap_or(&0);
                // By label: the two `…` rows share theirs.
                let pattern = numbered_pattern(&label);
                let places: Vec<usize> = rows
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x.layout() == layout && x.patterns().contains(&pattern))
                    .map(|(i, _)| i)
                    .collect();
                let Some(&first) = places.first() else {
                    continue;
                };
                match places.iter().find(|p| **p >= from) {
                    Some(&p) => {
                        last.insert(name, p);
                    }
                    None if first == 0 => {
                        last.insert(name, 0);
                    }
                    None => failures.push(format!(
                        "{}: `{label}` after `{}` breaks table `{name}`'s order",
                        run.caption,
                        rows[from].label()
                    )),
                }
            }
        }
    }
    for r in Row::ALL {
        if matches!(r.layout(), Layout::Timing) || *r == Row::Wall {
            continue;
        }
        if !printed.contains(&(format!("{:?}", r.layout()), r.patterns()[0].clone())) {
            failures.push(format!(
                "{r:?} (`{}`) is printed by no block of {EXAMPLES}",
                r.label()
            ));
        }
    }
    // The clock rows: one run, the times masked, against the page's block.
    let page = read(PAGE);
    let (_, rest) = page
        .split_once("<!-- masked: ")
        .expect("the page's masked block");
    let (cmd, rest) = rest.split_once(" -->").expect("its marker closes");
    let want = rest
        .split_once("```text\n")
        .and_then(|(_, b)| b.split_once("\n```"))
        .map(|(b, _)| b.to_string())
        .expect("its block");
    let args: Vec<String> = cmd
        .trim_start_matches("ein ")
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let tmp = Scratch::new("ein-read-out-clock");
    let j = tmp.join("s.json");
    let mut argv = args.clone();
    argv.push("--json-summary".into());
    argv.push(j.to_string_lossy().into());
    let out = String::from_utf8_lossy(&ein(&argv).stdout).into_owned();
    let got = masked(&out);
    if got != want {
        failures.push(format!(
            "the masked run `{cmd}` prints\n{got}\n  and the page's block is\n{want}"
        ));
    }
    for r in Row::ALL
        .iter()
        .filter(|r| matches!(r.layout(), Layout::Timing) || **r == Row::Wall)
    {
        let head = match r.layout() {
            Layout::Timing => format!("  {:<10}", r.label()),
            _ => format!("  {:<18}", r.label()),
        };
        if !out.lines().any(|l| l.starts_with(&head)) {
            failures.push(format!("the masked run prints no `{}` row", r.label()));
        }
    }
    // `-t`'s `route` saturations are `work`'s.
    let s: J =
        serde_json::from_str(&std::fs::read_to_string(&j).expect("the summary")).expect("JSON");
    let sats = s["work"]["saturations"].as_u64().expect("work.saturations");
    let route = out
        .lines()
        .skip_while(|l| *l != "timing")
        .find(|l| l.starts_with(&format!("  {:<10}", Row::TimingRoute.label())))
        .unwrap_or("");
    if !numbers(route).contains(&sats) {
        failures.push(format!(
            "-t's route row {route:?} is not work.saturations {sats}"
        ));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

// ── 3 ───────────────────────────────────────────────────────────────

/// Every path of a summary value, `[]` for an array's elements and `?var`
/// for a binding row's variable; and which of them are leaves.
fn paths(v: &J, at: &str, all: &mut BTreeSet<String>) {
    all.insert(at.to_string());
    match v {
        J::Object(m) => {
            let row = at.ends_with("rows[]");
            for (k, x) in m {
                let key = if row { "?var" } else { k.as_str() };
                paths(x, &format!("{at}.{key}"), all);
            }
        }
        J::Array(a) if a.iter().any(J::is_object) => {
            for x in a {
                paths(x, &format!("{at}[]"), all);
            }
        }
        _ => {}
    }
}

/// Every path the page names: its row tables' summary column and its fields
/// table, `[k]`, `[q]` and `[i]` read as `[]`.
fn page_paths(page: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (name, _) in Row::TABLES {
        for e in row_table(page, name) {
            out.extend(e.paths.iter().map(|p| normal(p)));
        }
    }
    for c in table(page, "<!-- fields -->", "<!-- /fields -->") {
        for p in spans(&c[0]).into_iter().filter(|p| is_path(p)) {
            out.insert(normal(&p));
        }
    }
    out
}

/// **Every answer field of the summary has a row**: every leaf path of the
/// seven keys over the examples' summaries is named by the page, in a row's
/// summary column or its table of the fields no row prints; every path the
/// page names is in a summary; and the owners table names every top-level key
/// a summary has, each key it names one a summary has.
#[test]
fn every_answer_field_of_the_summary_has_a_row() {
    let page = read(PAGE);
    let mut all = BTreeSet::new();
    let mut top = BTreeSet::new();
    for run in runs() {
        let Some(m) = run.summary.as_object() else {
            panic!("{}: no summary", run.caption);
        };
        for (k, v) in m {
            top.insert(k.clone());
            paths(v, k, &mut all);
        }
    }
    let leaves: BTreeSet<&String> = all
        .iter()
        .filter(|p| {
            ANSWER_KEYS.contains(&p.split(['.', '[']).next().unwrap_or(""))
                && !all.iter().any(|q| {
                    q.len() > p.len()
                        && q.starts_with(p.as_str())
                        && (q[p.len()..].starts_with('.') || q[p.len()..].starts_with("[]"))
                })
        })
        .collect();
    let named = page_paths(&page);
    let mut failures = Vec::new();
    for p in &leaves {
        if !named.contains(*p) {
            failures.push(format!("the summary's `{p}` is named nowhere on the page"));
        }
    }
    for p in &named {
        if !all.contains(p) {
            failures.push(format!(
                "the page names `{p}`, which no example's summary has"
            ));
        }
    }
    let owners: BTreeSet<String> = table(&page, "<!-- owners -->", "<!-- /owners -->")
        .into_iter()
        .flat_map(|c| spans(&c[0]))
        .collect();
    if owners != top {
        failures.push(format!(
            "the owners table names {owners:?}; a summary's keys are {top:?}"
        ));
    }
    assert!(failures.is_empty(), "{PAGE}:\n{}", failures.join("\n"));
}

// ── 4 ───────────────────────────────────────────────────────────────

/// The counts a row's value prints: each run of digits that is not part of
/// a name (`L1`, `C1`, `House-4`, `2nd`), with `261 982`'s groups of three
/// joined.
fn numbers(value: &str) -> Vec<u64> {
    let c: Vec<char> = value.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < c.len() {
        if !c[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let named = i > 0 && (c[i - 1].is_alphanumeric() || matches!(c[i - 1], '_' | '-' | '.'));
        let mut j = i;
        while j < c.len() && c[j].is_ascii_digit() {
            j += 1;
        }
        let mut digits: String = c[i..j].iter().collect();
        if j - i <= 3 {
            while j + 4 <= c.len()
                && c[j] == ' '
                && c[j + 1..j + 4].iter().all(char::is_ascii_digit)
                && c.get(j + 4).is_none_or(|x| !x.is_ascii_digit())
            {
                digits.extend(&c[j + 1..j + 4]);
                j += 4;
            }
        }
        let glued = c.get(j).is_some_and(|x| x.is_alphabetic());
        if !named
            && !glued
            && let Ok(n) = digits.parse()
        {
            out.push(n);
        }
        i = j;
    }
    out
}

/// Whether a row's value prints the field's value `v`: an integer, or an
/// array's length, among its numbers (a zero may print as a word); each
/// string of a list, or a string, in it; a flag as `yes` or `no`.
fn holds(value: &str, v: &J) -> Result<(), String> {
    let ok = match v {
        J::Null => true,
        J::Bool(b) => value.starts_with(if *b { "yes" } else { "no" }),
        J::Number(n) => {
            let n = n.as_u64().ok_or("not a count")?;
            n == 0 || numbers(value).contains(&n)
        }
        J::String(s) => value.contains(s.as_str()),
        J::Array(a) if !a.is_empty() && a.iter().all(J::is_string) => {
            a.iter().all(|s| value.contains(s.as_str().unwrap_or("")))
        }
        J::Array(a) => a.is_empty() || numbers(value).contains(&(a.len() as u64)),
        J::Object(_) => return Err("names an object, not a number".into()),
    };
    if ok {
        Ok(())
    } else {
        Err(format!("does not print {v}"))
    }
}

/// The value at a page path in one run's summary, its indices the row's:
/// `[k]` its place among the rows of its label, `[q]` the query it is under,
/// `[i]` its own number less one; `[]` every element.
fn at(s: &J, path: &str, k: usize, q: usize, i: usize) -> J {
    let mut cur = vec![s.clone()];
    for part in path.split('.') {
        let (key, idx) = match part.find('[') {
            Some(b) => (&part[..b], Some(&part[b..])),
            None => (part, None),
        };
        let mut next = Vec::new();
        for c in &cur {
            let v = c.get(key).cloned().unwrap_or(J::Null);
            match idx {
                None => next.push(v),
                Some(ix) => {
                    let n = match ix {
                        "[k]" => Some(k),
                        "[q]" => Some(q),
                        "[i]" => Some(i),
                        _ => None,
                    };
                    match (n, v) {
                        (Some(n), J::Array(a)) => next.push(a.get(n).cloned().unwrap_or(J::Null)),
                        (None, J::Array(a)) => next.extend(a),
                        _ => next.push(J::Null),
                    }
                }
            }
        }
        cur = next;
    }
    if cur.len() == 1 {
        cur.pop().unwrap_or(J::Null)
    } else {
        // A fan's strings: each must be printed.
        J::Array(cur)
    }
}

/// **Each row prints its summary field's number**, on every example run:
/// *one label, one number*, held for every row whose summary column names a
/// field. `read_out::each_count_is_printed_once_and_is_the_summarys` held it
/// for `k_s` and `k_c` alone.
#[test]
fn each_row_prints_its_summary_fields_number() {
    let page = read(PAGE);
    let cell: BTreeMap<(&str, Row), Vec<String>> = Row::TABLES
        .iter()
        .flat_map(|(name, rows)| {
            let entries = row_table(&page, name);
            rows.iter()
                .zip(entries)
                .map(move |(r, e)| ((*name, *r), e.paths))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut failures = Vec::new();
    let mut checked = 0usize;
    for run in runs() {
        let table = question(&run.summary);
        let mut seen: BTreeMap<Row, usize> = BTreeMap::new();
        let mut query = 0usize;
        for l in run.stdout.lines().skip(2) {
            let Some((layout, label, value)) = parse_row(l) else {
                continue;
            };
            let Some(r) = row_of(layout, &label) else {
                continue;
            };
            let k = *seen.entry(r).and_modify(|n| *n += 1).or_insert(0);
            if r == Row::Query {
                query = k;
            }
            let i = label
                .split(' ')
                .nth(1)
                .and_then(|n| n.parse::<usize>().ok())
                .map_or(0, |n| n - 1);
            let name = tables_of(r)
                .into_iter()
                .find(|n| !["describe", "mode", "monotone"].contains(n) || *n == table);
            let Some(paths) = name.and_then(|n| cell.get(&(n, r))) else {
                continue;
            };
            for p in paths {
                checked += 1;
                if let Err(e) = holds(&value, &at(&run.summary, p, k, query, i)) {
                    failures.push(format!("{}: `{label}` {e}, `{p}`'s:\n    {l}", run.caption));
                }
            }
        }
    }
    assert!(checked > 500, "{checked} row fields checked");
    assert!(failures.is_empty(), "{PAGE}:\n{}", failures.join("\n"));
}
