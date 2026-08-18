//! Peak RSS of a load — the memory half of P1a.2's acceptance.
//!
//! design/03 §10 asks for "peak RSS ≤ 1/5 of ein.py's", and there is no CLI
//! yet to measure it through, so this is the smallest program that answers the
//! question: load a `.ein` and report what the kernel says it cost.
//!
//! ```sh
//! cargo run --release --example load_rss -- examples/zebra2.ein
//! ```
//!
//! Both numbers are worth having. `peak` is what a user's machine sees, and it
//! flatters the port by the size of a Python interpreter; `delta` is the RSS
//! the load itself added, which is the data-model comparison and is the one
//! design/03 is really making a claim about.

use ein_core::Terms;
use ein_ir::{Ast, load_file};

fn field(name: &str) -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status
        .lines()
        .find(|l| l.starts_with(name))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/zebra2.ein".to_string());
    let before = field("VmRSS:");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = match load_file(&mut ast, &mut terms, std::path::Path::new(&path)) {
        Ok(kb) => kb,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(1);
        }
    };
    let after = field("VmRSS:");
    let peak = field("VmHWM:");
    println!(
        "{{\"file\": \"{path}\", \"facts\": {}, \"relations\": {}, \"rules\": {}, \
         \"rss_before_kb\": {before}, \"rss_after_kb\": {after}, \
         \"delta_kb\": {}, \"peak_kb\": {peak}, \"fact_store_bytes\": {}}}",
        kb.n_facts(),
        kb.program().relations.len(),
        kb.program().rules.len(),
        after.saturating_sub(before),
        terms.facts.footprint(),
    );
}
