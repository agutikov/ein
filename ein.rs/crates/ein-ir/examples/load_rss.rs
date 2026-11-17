//! Peak RSS of a load: the smallest program that answers what a load costs in
//! memory. It loads a `.ein` and reports what the kernel says it cost.
//!
//! ```sh
//! cargo run --release --example load_rss -- examples/zebra2.ein
//! ```
//!
//! Both numbers are worth having. `peak` is what a user's machine sees; `delta`
//! is the RSS the load itself added, which is the data model's cost
//! ([03-design/03-data-model.md](../../../../docs/ein/ein.rs/03-design/03-data-model.md)).

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
