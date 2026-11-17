//! **Hold a file of rows to their recorded answers.**
//!
//! A file of the `ein_corpus::regressions` format records, for each program
//! and run, what the engine answers today (`[case.now]`), the answer the
//! definitions give (`[case.right]`) and the witness's region
//! (`[case.region]`). This holds every row of the file it is given, as
//! `corpus/regressions.toml`'s tests hold the project's:
//!
//! - with no run, that the file is well formed, and that each row's
//!   `[case.now]` answers its `[case.right]` — or, with `--open`, that no row's
//!   does, by the method, which is what a file of known wrong answers holds;
//! - each row's `[case.now]` through the binary given by `--ein`;
//! - each row's `[case.region]` through the library; `--bless` rewrites a
//!   block that differs instead.
//!
//! Both are `tests/held/`'s, which `tests/regressions.rs` holds the project's
//! file by.
//!
//! It prints what it held and every difference, and exits 1 on any.
//!
//! ```text
//! cargo run -p ein-cli --example held_answers -- --ein target/release/ein FILE
//! cargo run -p ein-cli --example held_answers -- --ein target/release/ein --open FILE
//! ```

#[path = "../tests/held/mod.rs"]
mod held;

use std::path::PathBuf;
use std::process::ExitCode;

use ein_corpus::Scratch;
use ein_corpus::regressions::{Held, Regressions};

fn main() -> ExitCode {
    let mut ein: Option<PathBuf> = None;
    let mut file: Option<PathBuf> = None;
    let mut open = false;
    let mut bless = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--ein" => ein = args.next().map(PathBuf::from),
            "--open" => open = true,
            "--bless" => bless = true,
            "-h" | "--help" => {
                println!("usage: held_answers --ein BIN [--open] [--bless] FILE");
                return ExitCode::SUCCESS;
            }
            _ if file.is_none() && !a.starts_with('-') => file = Some(PathBuf::from(a)),
            _ => {
                eprintln!("held_answers: unexpected argument {a:?}");
                return ExitCode::from(2);
            }
        }
    }
    let (Some(ein), Some(file)) = (ein, file) else {
        eprintln!("usage: held_answers --ein BIN [--open] [--bless] FILE");
        return ExitCode::from(2);
    };
    let ein = std::fs::canonicalize(&ein).unwrap_or(ein);
    let path = std::fs::canonicalize(&file).unwrap_or(file);
    let rows = match Regressions::load(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("held_answers: {e}");
            return ExitCode::from(2);
        }
    };
    let held = if open { Held::Open } else { Held::Right };
    let mut bad = rows.defects(held);
    bad.extend(match held {
        Held::Right => rows.not_right(),
        Held::Open => rows.right_by_the_method(),
    });
    let scratch = Scratch::new("ein-held-answers");
    let ran = held::hold(&ein, &rows.case, &scratch);
    bad.extend(ran.bad);
    let (regions, region_bad) = held::regions::check(&path, bless);
    bad.extend(region_bad);
    println!(
        "{}: {} rows, {} runs, {} regions{}",
        path.display(),
        rows.case.len(),
        ran.runs,
        regions,
        if open {
            ", each still answered wrong"
        } else {
            ", each answered right"
        }
    );
    if bad.is_empty() {
        ExitCode::SUCCESS
    } else {
        for b in &bad {
            println!("\n{b}");
        }
        println!("\n{} differences", bad.len());
        ExitCode::FAILURE
    }
}
