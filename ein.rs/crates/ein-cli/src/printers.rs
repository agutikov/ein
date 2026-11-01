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
        (
            "enable-pre-branch-lookahead",
            b(cfg.enable_pre_branch_lookahead),
        ),
        (
            "enable-lookahead-kill-cache",
            b(cfg.enable_lookahead_kill_cache),
        ),
        ("warn-derived-naf", b(cfg.warn_derived_naf)),
        ("lattice-sanity-check", b(cfg.lattice_sanity_check)),
        ("enable-path-nogoods", b(cfg.enable_path_nogoods)),
        (
            "record-alternative-justifications",
            b(cfg.record_alternative_justifications),
        ),
        ("enable-fail-fast-fork", b(cfg.enable_fail_fast_fork)),
    ];
    println!("config (resolved)");
    for (name, shown) in fields {
        println!("  {name:<32} {shown}");
    }
}
