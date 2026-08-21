//! The argument surface — `ein`, `render`, `solve`, and `saturate`'s own.
//!
//! One module so the whole surface can be *introspected* rather than scraped:
//! [Q-M1a.13](../../../../plans/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity)
//! resolved `--help` layout and usage-error text onto the normalisation list,
//! and what replaces the byte diff is a structural comparison of exactly these
//! builders against `argparse`'s parser objects. Layout is `clap`'s business;
//! every option, short key, metavar, default, `choices` value and exclusive
//! group here is ein.py's, and the help strings are its help strings.

use clap::{Arg, ArgAction, ArgGroup, Command, builder::PossibleValuesParser};

/// `type=int` — CPython's `int()`, which takes surrounding whitespace and PEP
/// 515 `_` separators that Rust's `FromStr` does not. Accepting less would be
/// a difference in *behaviour*, which is on the exact list.
fn py_int(s: &str) -> Result<i64, String> {
    ein_core::python_int(s).ok_or_else(|| format!("invalid int value: '{s}'"))
}

/// `type=float`, same argument.
fn py_float(s: &str) -> Result<f64, String> {
    ein_core::python_float(s).ok_or_else(|| format!("invalid float value: '{s}'"))
}

fn flag(short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .action(ArgAction::SetTrue)
        .help(help)
}

/// The `--rule-mode` option, shared by `render rules` and `render rule` —
/// ein.py shares the same `dict` of keyword arguments between the two.
fn rule_mode() -> Arg {
    Arg::new("rule-mode")
        .long("rule-mode")
        .value_name("RULE_MODE")
        .value_parser(PossibleValuesParser::new(["sidebyside", "overlay"]))
        .default_value("sidebyside")
        .help(
            "rule rendering: 'sidebyside' LHS|RHS clusters (default) \
             or 'overlay' (LHS solid + RHS dashed)",
        )
}

/// `--events` / `--events-level` — `cli/_events.add_arguments`, added to
/// *both* `solve` and `saturate`.
fn event_args(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("events")
            .long("events")
            .value_name("FILE.jsonl")
            .help(
                "record the engine's step-by-step event log to FILE \
                 (one JSON object per line). Off by default and \
                 additive: stdout, stderr and the exit code are \
                 unchanged. Schema: \
                 docs/kernel/inference/events.md.",
            ),
    )
    .arg(
        Arg::new("events-level")
            .long("events-level")
            .value_name("EVENTS_LEVEL")
            .value_parser(PossibleValuesParser::new(["normal", "verbose"]))
            .default_value("normal")
            .help(
                "(--events) `verbose` adds redundant firings and \
                 pre-candidate hypothesis skips — ~6x the volume, and \
                 what T2 comparisons run at, since a dropped \
                 redundant firing is exactly what a port loses.",
            ),
    )
}

/// `ein render` — DOT only, no SVG.
fn render_command() -> Command {
    Command::new("render")
        .about("DOT for rules / constraints (S1.6.1)")
        .subcommand_required(true)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("rules")
                .about("one DOT digraph per rule")
                .arg(Arg::new("file").required(true))
                .arg(rule_mode()),
        )
        .subcommand(
            Command::new("rule")
                .about("a single rule's DOT, by name")
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("name")
                        .long("name")
                        .value_name("NAME")
                        .required(true)
                        .help("rule name to render"),
                )
                .arg(rule_mode()),
        )
        .subcommand(
            Command::new("constraints")
                .about("constraint-scope DOT (structural properties)")
                .arg(Arg::new("file").required(true)),
        )
        .subcommand(
            Command::new("lattice")
                .about("commitment-lattice / proof-DAG DOT (runs a solve, S1.6.3)")
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("view")
                        .long("view")
                        .value_name("VIEW")
                        .value_parser(PossibleValuesParser::new(["full", "solution"]))
                        .default_value("solution")
                        .help(
                            "'solution' survivors + pruned siblings (default) or \
                             'full' every commitment (falls back to 'solution' — \
                             solve doesn't store the per-commitment SetNode DAG)",
                        ),
                )
                // Note: 3 here, not `solve`'s 5.
                .arg(
                    Arg::new("max-set-size")
                        .long("max-set-size")
                        .value_name("MAX_SET_SIZE")
                        .value_parser(py_int)
                        .default_value("3")
                        .help("commitment-set depth cap (default 3)"),
                ),
        )
}

/// `ein solve` — the one solver command. Every option carries a short key
/// except the three file-only ones.
fn solve_command() -> Command {
    let cmd = Command::new("solve")
        .about("solve a puzzle: print the solution(s) or the unsat core")
        .arg(Arg::new("file").required(true))
        // ── stop policy (single / N / exhaustive) ──
        .arg(
            Arg::new("solutions")
                .short('n')
                .long("solutions")
                .value_name("N")
                .value_parser(py_int)
                .default_value("1")
                .help("stop after N distinct solutions (default: 1)"),
        )
        .arg(flag(
            'e',
            "exhaustive",
            "exhaust the lattice — certify unique / ambiguous / unsat (slower)",
        ))
        .group(
            ArgGroup::new("stop")
                .args(["solutions", "exhaustive"])
                .multiple(false),
        )
        // ── engine knobs ──
        .arg(
            Arg::new("max-set-size")
                .short('m')
                .long("max-set-size")
                .value_name("MAX_SET_SIZE")
                .value_parser(py_int)
                .default_value("5")
                .help("commitment-set depth cap (default: 5)"),
        )
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(py_float)
                .help("abort after N wall-clock seconds"),
        )
        .arg(
            Arg::new("max-enterings")
                .short('E')
                .long("max-enterings")
                .value_name("MAX_ENTERINGS")
                .value_parser(py_int)
                .help("abort after N commitment tries"),
        )
        .arg(flag(
            'L',
            "no-lookahead",
            "disable hypgen's one-step lookahead (forces deaths \
             through the no-good learning path)",
        ))
        .arg(flag(
            'K',
            "no-kill-cache",
            "disable the lookahead-kill (not h) cache (pairs with \
             --no-lookahead to exercise the raw loop)",
        ))
        .arg(
            Arg::new("lattice-order")
                .short('o')
                .long("lattice-order")
                .value_name("LATTICE_ORDER")
                .value_parser(PossibleValuesParser::new(["lex", "score-sum"]))
                .help(
                    "within-layer candidate ordering (S1.5b.26): 'lex' \
                     (canonical-tuple sort, the deterministic default) or \
                     'score-sum' (S1.5a.7 hypgen scoring summed per set)",
                ),
        )
        .arg(flag(
            'y',
            "lattice-sanity-check",
            "S1.5b.27 regression: for every alive size-k≥2 \
             commitment, verify every (k-1)-subset parent path \
             saturates to the same kb (costs k+1 saturations each)",
        ))
        // ── search order (shuffle invariance — S1.5b.31) ──
        .arg(flag(
            'z',
            "shuffle",
            "shuffle the within-layer commitment order (a per-layer \
             random permutation). Fresh seed each run unless --seed \
             pins it; the seed used is printed to stderr. Changes the \
             TRAVERSAL order, not the verdict — the answer is \
             shuffle-invariant.",
        ))
        .arg(
            Arg::new("seed")
                .short('d')
                .long("seed")
                .value_name("SEED")
                .value_parser(py_int)
                .help("pin the --shuffle RNG seed for a reproducible permutation"),
        )
        // ── progress / diagnostics ──
        .arg(flag(
            'v',
            "verbose",
            "stream per-layer + per-entering progress to stderr",
        ))
        .arg(
            Arg::new("progress-every")
                .short('g')
                .long("progress-every")
                .value_name("PROGRESS_EVERY")
                .value_parser(py_int)
                .default_value("100")
                .help(
                    "under --verbose, log every N-th entering (default 100; \
                     set 1 to log every entering)",
                ),
        )
        .arg(
            Arg::new("dump-states")
                .short('D')
                .long("dump-states")
                .value_name("DIR")
                .help(
                    "persist the engine dump tree (root snapshot per layer + \
                     timeline) to DIR",
                ),
        )
        .arg(flag(
            'c',
            "dump-config",
            "print the resolved SolverConfig before solving",
        ))
        .arg(flag(
            'H',
            "hyp-stats",
            "print the root-hypothesis preview (total + per-relation \
             breakdown + hypgen filter report)",
        ))
        .arg(flag(
            't',
            "timing",
            "print a per-phase wall-clock table: parse, kb load, \
             compile, root saturation, hypothesis search (enterings, \
             layers, saturations, per-hypothesis avg), and totals",
        ))
        // ── extra stdout ──
        .arg(flag(
            's',
            "stats",
            "print engine counters (k, enterings, layers, wall)",
        ))
        .arg(flag(
            'p',
            "print-final-state",
            "dump each solution's derived facts (for an \
             unsat verdict, the unsat-core facts instead)",
        ))
        .arg(flag(
            'P',
            "print-final-positive",
            "like --print-final-state but drops the (not …) facts",
        ))
        .arg(flag(
            'f',
            "print-final-hfacts",
            "dump only the hypothesis-target (query :hrules) facts \
             per solution",
        ));

    // ── event log (file only) ──
    let cmd = event_args(cmd);

    cmd
        // ── machine-readable summary (file only) ──
        .arg(
            Arg::new("json-summary")
                .long("json-summary")
                .value_name("FILE.json")
                .help(
                    "write the structured run summary (verdict, model, \
                     every engine counter, the root-saturation shape, the \
                     resolved config) to FILE as JSON. Additive: stdout, \
                     stderr and the exit code are unchanged.",
                ),
        )
        // ── markdown trace (file only) ──
        .arg(
            Arg::new("trace")
                .short('r')
                .long("trace")
                .value_name("FILE.md")
                .help(
                    "write the self-contained markdown derivation trace to \
                     FILE (a file, not stdout)",
                ),
        )
        .arg(flag(
            'G',
            "no-diagrams",
            "(--trace) suppress all inline dot blocks",
        ))
        .arg(flag(
            'F',
            "full-kb-snapshots",
            "(--trace) append a whole-KB snapshot of the final state",
        ))
        .arg(flag(
            'R',
            "reorder",
            "(--trace) cluster steps by target entity instead of \
             engine order",
        ))
        .arg(flag(
            'l',
            "relevant",
            "(--trace) prune to the goal-relevant slice",
        ))
}

/// `ein saturate`'s **own** parser, with its own `prog`.
///
/// Registered in the top-level tree as a bare subcommand purely so it appears
/// in `ein --help`; the dispatcher intercepts `saturate` before `clap` ever
/// reaches it, which is `_DELEGATED`'s two-level behaviour.
pub fn saturate_command() -> Command {
    event_args(
        Command::new("saturate")
            .bin_name("ein saturate")
            .about("Benchmark + state dump for the Saturator.")
            .arg(Arg::new("file").required(true).help("path to a .ein file"))
            .arg(
                Arg::new("dump")
                    .long("dump")
                    .action(ArgAction::SetTrue)
                    .help("after the benchmark, print the saturated KB grouped by origin"),
            )
            .arg(
                Arg::new("max-steps")
                    .long("max-steps")
                    .value_name("MAX_STEPS")
                    .value_parser(py_int)
                    .help(
                        "hard cap on saturator firings (raises SaturatorStepLimitError \
                         when exceeded); useful for runaway-debugging on a fresh \
                         input. Default: no cap.",
                    ),
            )
            .arg(
                Arg::new("progress-every")
                    .long("progress-every")
                    .value_name("PROGRESS_EVERY")
                    .value_parser(py_int)
                    .default_value("500")
                    .help(
                        "log a one-line progress sample every N steps \
                         (0 disables; default: 500).",
                    ),
            ),
    )
}

/// The whole tree.
pub fn command() -> Command {
    /// `ein kb` — P1a.8's surface, and the only subcommand with no ein.py
    /// counterpart. Registered only when the `einb` feature is on, so a build
    /// without the container does not advertise it.
    #[cfg(feature = "einb")]
    fn kb_command() -> Command {
        Command::new("kb")
            .about("the .einb binary knowledge-base container (P1a.8)")
            .subcommand_required(true)
            .subcommand(
                Command::new("save")
                    .about("load FILE and write it as a .einb container")
                    .arg(Arg::new("file").required(true))
                    .arg(Arg::new("out").required(true).value_name("OUT.einb"))
                    .arg(flag(
                        's',
                        "saturate",
                        "bank the least fixpoint too, not just the loaded KB",
                    )),
            )
    }

    let cmd = Command::new("ein")
        .about("Graph-based relation algebra solver for Zebra-style logic puzzles.")
        .subcommand_required(true)
        .arg_required_else_help(false)
        .disable_version_flag(true)
        // ein.py has no `help` subcommand and no `--version`.
        .disable_help_subcommand(true)
        .subcommand(render_command())
        .subcommand(solve_command())
        .subcommand(
            // Bare — never parsed. Its flags live in `saturate_command`.
            Command::new("saturate")
                .about("saturate to a least fixpoint + phase timings / state dump")
                .disable_help_flag(true)
                .allow_hyphen_values(true)
                .arg(
                    Arg::new("argv")
                        .num_args(0..)
                        .allow_hyphen_values(true)
                        .hide(true),
                ),
        );
    #[cfg(feature = "einb")]
    let cmd = cmd.subcommand(kb_command());
    cmd
}
