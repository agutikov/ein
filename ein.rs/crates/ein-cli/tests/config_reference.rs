//! [`docs/kernel/configuration.md`] — the configuration reference, pinned.
//!
//! M1e [S1e.5.1](../../../../plans/m1e_review_processing/p1e.5_documentation_and_other/s1e.5.1_config_reference.md).
//! The page enumerates three surfaces — the 17 `(config …)` flags, the CLI
//! options, and the `EIN_*` environment — and a reference page is the most
//! drift-prone shape there is: a list of names and defaults that nothing runs.
//! [`docs/api/rust.md`](../../../../docs/api/rust.md) is the precedent that
//! works, and this file is the same trade with a different generator.
//!
//! Six claims, one test each, and each is a claim the page makes in prose:
//!
//! 1. [`the_defaults_block_is_the_binarys_own_dump_config`] — the page's
//!    generated block **is** `ein solve --dump-config` on a program with no
//!    `(config …)` head. A flag added to `FIELDS` appears there or this fails.
//!    *Edit nothing by hand: `EIN_BLESS=1` rewrites the block in place.*
//! 2. [`every_flag_has_a_row_and_no_row_is_orphaned`] — the judgement table
//!    outside the markers lists the same flags, in the same order, with the
//!    same types and the same defaults. This is T3's cheap guard, and it is
//!    what makes the prose columns safe to hand-write.
//! 3. [`the_cli_counts_come_from_the_golden`] — the per-subcommand option
//!    counts are read off `golden/help_shape.txt`, which already owns the
//!    surface (`help_surface.rs`). The page cites an owner rather than
//!    competing with one.
//! 4. [`the_shipped_environment_set_is_what_the_page_lists`] — every
//!    `EIN_*` name a *shipping* crate reads from the process environment has a
//!    row, and every row names one. The classification is the result: a grep
//!    census lists `EIN_RS`, which is a Python local.
//! 5. [`the_two_inert_flags_are_still_inert`] — `print-alive` and
//!    `candidate-order-seed` are read by no code path, which is the page's
//!    one novel claim about behaviour, so it is held by behaviour.
//! 6. [`the_surface_table_agrees_with_the_sections_it_summarises`] — the
//!    three-row table at the top of the page repeats three numbers the
//!    sections below own. Two hundred lines apart inside one file is still a
//!    parallel copy.
//!
//! [`docs/kernel/configuration.md`]: ../../../../docs/kernel/configuration.md

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_core::config::{FIELDS, FieldKind};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("crates/<crate>/ is three below the root")
        .to_path_buf()
}

fn page_path() -> PathBuf {
    repo_root().join("docs/kernel/configuration.md")
}

fn page() -> String {
    std::fs::read_to_string(page_path()).expect("docs/kernel/configuration.md")
}

/// The scratch directory this file owns. One per process, so a parallel run of
/// another test suite cannot collide with it.
fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-config-reference-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn write_fixture(tag: &str, body: &str) -> PathBuf {
    let path = scratch().join(format!("{tag}.ein"));
    std::fs::write(&path, body).expect("writes");
    path
}

fn ein(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

// ── 1. The generated block ──────────────────────────────────────────

/// The markers around the generated block. HTML comments, so they survive
/// every markdown renderer and are invisible in the rendered page.
const BEGIN: &str = "<!-- generated: ein solve --dump-config -->\n```text\n";
const END: &str = "\n```\n<!-- /generated -->";

/// A program with **no** `(config …)` head, so `--dump-config` resolves to the
/// defaults and nothing else. Two facts, because a program with none has
/// nothing for the loader to do and this way the run is a real solve.
const NO_CONFIG: &str = "(relation p T)\n(p A)\n";

/// `--dump-config`'s block: the header line and the indented lines under it.
fn dump_config_block() -> String {
    let f = write_fixture("no-config", NO_CONFIG);
    let out = ein(&["solve", "--dump-config", &f.to_string_lossy()]);
    let mut lines = out.lines().skip_while(|l| *l != "config (resolved)");
    let head = lines.next().expect("--dump-config printed no config block");
    let mut block = String::from(head);
    for l in lines.take_while(|l| l.starts_with("  ")) {
        block.push('\n');
        block.push_str(l);
    }
    block
}

/// The page's generated region **is** the binary's `--dump-config` output.
///
/// This is the flag list, in declaration order, with every default — so a
/// flag added to [`FIELDS`], a renamed one and a changed default all fail
/// here. It is the strongest of the five because the text is not derived from
/// the source, it is the shipped program's own answer.
///
/// ```sh
/// EIN_BLESS=1 cargo test -p ein-cli --test config_reference
/// ```
#[test]
fn the_defaults_block_is_the_binarys_own_dump_config() {
    let want = dump_config_block();
    assert!(
        want.lines().count() == FIELDS.len() + 1,
        "--dump-config printed {} lines for {} flags",
        want.lines().count(),
        FIELDS.len()
    );

    let text = page();
    let (before, rest) = text
        .split_once(BEGIN)
        .expect("docs/kernel/configuration.md has no generated-block opening marker");
    let (got, after) = rest
        .split_once(END)
        .expect("docs/kernel/configuration.md has no generated-block closing marker");

    if got == want {
        return;
    }
    if std::env::var("EIN_BLESS").as_deref() == Ok("1") {
        let fresh = format!("{before}{BEGIN}{want}{END}{after}");
        std::fs::write(page_path(), fresh).expect("rewrites the page");
        return;
    }
    panic!(
        "docs/kernel/configuration.md's generated block is not \
         `ein solve --dump-config`'s output.\n\
         Re-bank it with `EIN_BLESS=1 cargo test -p ein-cli --test \
         config_reference`; never edit the block by hand.\n\n\
         --- page ---\n{got}\n--- binary ---\n{want}\n"
    );
}

// ── 2. The judgement table ──────────────────────────────────────────

/// One row of the page's judgement table, as far as this test reads it.
struct Row {
    flag: String,
    kind: String,
    default: String,
}

/// The rows of the one markdown table whose first column header is `flag`.
///
/// Deliberately not a general markdown parser: the page has exactly one such
/// table and a second would be a page that says the same thing twice.
fn judgement_rows(text: &str) -> Vec<Row> {
    let cells = |line: &str| -> Vec<String> {
        line.trim()
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(|c| c.trim().trim_matches('`').trim().to_string())
            .collect()
    };
    let mut rows = Vec::new();
    let mut in_table = false;
    for line in text.lines() {
        let is_row = line.trim_start().starts_with('|');
        if !is_row {
            in_table = false;
            continue;
        }
        let c = cells(line);
        if !in_table {
            // The header, then the `|---|` rule, then the rows.
            in_table = c.first().is_some_and(|h| h == "flag");
            continue;
        }
        if c.first().is_some_and(|f| f.starts_with("---")) {
            continue;
        }
        if c.len() >= 3 {
            rows.push(Row {
                flag: c[0].clone(),
                kind: c[1].clone(),
                default: c[2].clone(),
            });
        }
    }
    rows
}

/// T3's cheap guard: the prose half of the table lists the same flags, in the
/// same order, with the same types and the same defaults as the generated
/// half.
///
/// It catches what the diff above cannot — a flag documented in the block and
/// forgotten in the judgement table, which is the shape that would let a knob
/// ship with a default and no *"does it change the answer"* answer. That
/// column is the page's reason to exist.
#[test]
fn every_flag_has_a_row_and_no_row_is_orphaned() {
    let text = page();
    let rows = judgement_rows(&text);
    assert!(
        !rows.is_empty(),
        "found no table in docs/kernel/configuration.md whose first column is `flag`"
    );

    let want: Vec<&str> = FIELDS.iter().map(|(n, _)| *n).collect();
    let got: Vec<&str> = rows.iter().map(|r| r.flag.as_str()).collect();
    assert_eq!(
        got, want,
        "the judgement table's flags are not `FIELDS`, in declaration order"
    );

    for ((name, kind), row) in FIELDS.iter().zip(&rows) {
        let expected = match kind {
            FieldKind::Bool => "bool",
            FieldKind::Int => "int",
            FieldKind::Float => "float",
            FieldKind::Str => "str",
        };
        assert_eq!(row.kind, expected, "{name}: the type column drifted");
    }

    // …and the default column, against the generated block rather than
    // against `SolverConfig::default()`: the block is what a reader compares
    // the row to, so the row has to agree with *it*.
    let block = dump_config_block();
    for (row, line) in rows.iter().zip(block.lines().skip(1)) {
        let (name, value) = line.trim().split_once(char::is_whitespace).expect("a pair");
        assert_eq!(row.flag, name, "the block and the table are out of step");
        assert_eq!(
            row.default,
            value.trim(),
            "{name}: the table's default is not the block's"
        );
    }
}

// ── 3. The CLI counts ───────────────────────────────────────────────

/// A small number as the page spells it. Prose, not a table cell, so a digit
/// would read wrong; and the page has to state the number somehow or the
/// parser count is unpinned.
fn spell(n: usize) -> String {
    const WORDS: [&str; 21] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    WORDS
        .get(n)
        .map_or_else(|| n.to_string(), |w| (*w).to_string())
}

/// `(command, options)` from the help-shape golden — the file `help_surface.rs`
/// already owns.
fn golden_option_counts() -> Vec<(String, usize)> {
    let text = std::fs::read_to_string(
        repo_root().join("ein.rs/crates/ein-cli/tests/golden/help_shape.txt"),
    )
    .expect("the help-shape golden");
    let mut out: Vec<(String, usize)> = Vec::new();
    for line in text.lines() {
        if let Some(cmd) = line.strip_prefix("COMMAND ") {
            out.push((cmd.to_string(), 0));
        } else if line.starts_with("  OPTION ")
            && let Some(last) = out.last_mut()
        {
            last.1 += 1;
        }
    }
    out
}

/// The page's CLI counts are the golden's, per subcommand and in total.
///
/// The page does not re-enumerate the options: `--help` and the golden are the
/// surface's owners and a third list would be [AR-M1]'s parallel copy. What it
/// states is a *shape* — how many options each subcommand has — and that is a
/// number, so it is read off the owner rather than typed.
///
/// [AR-M1]: ../../../../plans/m1e_review_processing/README.md
#[test]
fn the_cli_counts_come_from_the_golden() {
    let text = page();
    let counts = golden_option_counts();

    for (cmd, n) in &counts {
        if *n == 0 {
            continue;
        }
        let want = format!("| `{cmd}` | {n} |");
        assert!(
            text.contains(&want),
            "docs/kernel/configuration.md has no row `{want}` — the golden says \
             {cmd} takes {n} options"
        );
    }

    let parsers = counts.len();
    assert!(
        text.contains(&format!("across {} parsers", spell(parsers))),
        "the page does not say the golden has {parsers} parsers"
    );

    let total: usize = counts.iter().map(|(_, n)| n).sum();
    assert!(
        text.contains(&format!("**{total}** options")),
        "the page does not state the golden's total of {total} options"
    );
    // § 1's summary table repeats the total, and a summary that disagrees with
    // the section it summarises is worse than no summary.
    assert!(
        text.contains(&format!("| CLI options | **{total}** |")),
        "§ 1's surface table does not say {total}"
    );
}

// ── 4. The environment census ───────────────────────────────────────

/// The crates that ship. `ein-corpus` and `ein-parity` are `publish = false`
/// and reach no binary, so what they read is the *harness*'s environment and
/// belongs in the page's second class, not its first.
const SHIPPING: [&str; 6] = [
    "ein-core",
    "ein-ir",
    "ein-infer",
    "ein-einb",
    "ein-render",
    "ein-cli",
];

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
}

/// Every `EIN_*` name a shipping crate reads from the **process** environment.
///
/// The lookback is what makes this a census of reads rather than of mentions:
/// `EIN_STDLIB` appears in a doc comment three lines above the call that reads
/// it, and `EIN_BLESS` appears in a `cargo test` invocation inside one.
fn env_names_read_by_shipping_crates() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for crate_name in SHIPPING {
        let mut files = Vec::new();
        rust_files(
            &repo_root()
                .join("ein.rs/crates")
                .join(crate_name)
                .join("src"),
            &mut files,
        );
        for f in files {
            let text = std::fs::read_to_string(&f).expect("a source file");
            for (at, _) in text.match_indices("\"EIN_") {
                let back = text[at.saturating_sub(24)..at].replace(char::is_whitespace, "");
                if !back.ends_with("env::var(") && !back.ends_with("env::var_os(") {
                    continue;
                }
                let name: String = text[at + 1..]
                    .chars()
                    .take_while(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || *c == '_')
                    .collect();
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }
    }
    names.sort();
    names
}

/// The page's *read by the shipped binary* class is exactly that set.
///
/// Both directions. A new `std::env::var("EIN_…")` in a shipping crate fails
/// here until it has a row; a row for a name nothing reads fails here too,
/// which is the half that matters — the census this page replaces listed
/// `EIN_RENDER_LEVI`, which no code has ever read.
#[test]
fn the_shipped_environment_set_is_what_the_page_lists() {
    let text = page();
    let read = env_names_read_by_shipping_crates();
    assert!(
        read.len() >= 5,
        "the scanner found only {read:?} — it stopped seeing the call sites"
    );

    // The page's first class is the section between its two headings; a name
    // listed in a *later* class must not count as a row here.
    let class_a = text
        .split_once("### Read by the shipped binary")
        .expect("the page has no `Read by the shipped binary` section")
        .1
        .split_once("\n### ")
        .expect("that section has no successor")
        .0;

    for name in &read {
        assert!(
            class_a.contains(&format!("`{name}`")),
            "{name} is read by a shipping crate and has no row in the page's first class"
        );
    }
    for line in class_a
        .lines()
        .filter(|l| l.trim_start().starts_with("| `EIN_"))
    {
        let name: String = line
            .trim_start()
            .trim_start_matches("| `")
            .chars()
            .take_while(|c| *c != '`')
            .collect();
        assert!(
            read.contains(&name),
            "the page lists {name} as read by the shipped binary; no shipping crate reads it"
        );
    }
}

// ── 5. The two inert flags ──────────────────────────────────────────

/// `print-alive` and `candidate-order-seed` are settable, validated,
/// `--dump-config`-printed, `--json-summary`-echoed, `.einb`-round-tripped —
/// and read by nothing.
///
/// That is the page's one claim about behaviour that no other test covers, and
/// it is the claim most likely to stop being true, because wiring either flag
/// up is a small change nobody would think to re-document. So it is held from
/// the outside: the whole of stdout, on two fixtures with different search
/// shapes, byte for byte.
///
/// `-p` and no `-s`: the stats block carries a wall clock, and a test that
/// compares wall clocks is a test that fails on a busy machine.
#[test]
fn the_two_inert_flags_are_still_inert() {
    let fixtures = [
        "examples/branching/04_two_levels.ein",
        "examples/lattice/02_genuine_3set_death.ein",
    ];
    let flags = [
        "(config :print-alive true)",
        "(config :candidate-order-seed 7)",
    ];

    for path in fixtures {
        let src = std::fs::read_to_string(repo_root().join(path)).expect(path);
        let tag = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("a stem");
        // `(config …)` is last-wins, so appending a block overrides whatever
        // the fixture set — the same way `utils/feature_matrix.py` builds a
        // cell.
        let base = write_fixture(&format!("{tag}-base"), &src);
        let base_out = ein(&["solve", "-e", "-p", &base.to_string_lossy()]);
        assert!(
            base_out.contains("final-state facts"),
            "{path}: the baseline run printed no state\n{base_out}"
        );

        for (i, flag) in flags.iter().enumerate() {
            let alt = write_fixture(&format!("{tag}-{i}"), &format!("{src}\n{flag}\n"));
            let alt_out = ein(&["solve", "-e", "-p", &alt.to_string_lossy()]);
            assert_eq!(
                base_out.replace(&*base.to_string_lossy(), "F"),
                alt_out.replace(&*alt.to_string_lossy(), "F"),
                "{path}: {flag} changed the run. It is documented as inert in \
                 docs/kernel/configuration.md — either the flag was wired up \
                 (update the page and delete this fixture's row) or something \
                 else moved."
            );
        }
    }
}

// ── 6. The summary table ───────────────────────────────────────────

/// § 1's flag count and environment count, against the sections that own them.
///
/// The three-row table at the top is what most readers will take away, and all
/// three of its numbers are stated a second time further down — which is the
/// parallel-copy shape [AR-M1] is about, two hundred lines apart inside one
/// file. The CLI row is [`the_cli_counts_come_from_the_golden`]'s; these are
/// the other two.
///
/// [AR-M1]: ../../../../plans/m1e_review_processing/README.md
#[test]
fn the_surface_table_agrees_with_the_sections_it_summarises() {
    let text = page();
    assert!(
        text.contains(&format!("| `(config …)` flags | **{}** |", FIELDS.len())),
        "§ 1 does not say there are {} flags",
        FIELDS.len()
    );
    let n = env_names_read_by_shipping_crates().len();
    assert!(
        text.contains(&format!("**{n}** read by the shipped binary")),
        "§ 1 does not say {n} environment variables reach the binary"
    );
}
