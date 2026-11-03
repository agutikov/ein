//! `ein solve -c`'s printer: the resolved config.

use ein_core::SolverConfig;

/// `-c/--dump-config` — each flag a program can set
/// (`ein_core::config::FLAGS`), resolved, in declaration order, with the name
/// kebab-cased into a 32-column field.
///
/// Note this is *not* `config::rendered_fields`, which spells a bool the
/// KB-shape dump's way (`True`); `--dump-config` lowers it.
pub fn print_resolved_config(cfg: &SolverConfig) {
    let b = |v: bool| if v { "true" } else { "false" }.to_string();
    let fields: Vec<(&str, String)> = vec![
        ("lattice-sanity-check", b(cfg.lattice_sanity_check)),
        (
            "record-alternative-justifications",
            b(cfg.record_alternative_justifications),
        ),
    ];
    println!("config (resolved)");
    for (name, shown) in fields {
        println!("  {name:<32} {shown}");
    }
}
