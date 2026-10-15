//! The argument surface — `ein`, `render`, `solve`, `test`, and `saturate`'s own.
//!
//! One module so the whole surface can be *introspected* rather than scraped:
//! `--help` layout is `clap`'s business and is not pinned byte for byte, and
//! what pins the surface instead is a structural rendering of exactly these
//! builders ([`crate::help_shape`]) — every option, short key, metavar,
//! default, `choices` value, exclusive group and help string.

use clap::{Arg, ArgAction, ArgGroup, Command, builder::PossibleValuesParser};

/// An integer argument, through [`ein_core::parse_int`]: surrounding
/// whitespace is ignored, a sign is allowed, and a `_` may sit between two
/// digits (`1_000`) — the syntax `(config …)` coerces with, which Rust's
/// `FromStr` alone does not accept.
fn int_arg(s: &str) -> Result<i64, String> {
    ein_core::parse_int(s).ok_or_else(|| format!("invalid int value: '{s}'"))
}

/// `-j/--jobs` — a positive integer, or the word `auto`.
///
/// `auto` parses to **0**, which [`crate::solve`] resolves through
/// [`std::thread::available_parallelism`]. It is a sentinel and not a
/// spelling: `--jobs 0` is refused, because a user who types it means "none"
/// far more often than "all of them", and a flag with two ways to say the same
/// thing has two ways to be misread.
///
/// What `auto` promises is the *machine's* answer, not the best one:
/// `available_parallelism` counts logical CPUs, and on a hybrid CPU that
/// includes SMT siblings and E-cores. P1a.7's measurements say more threads
/// than physical P-cores is not faster
/// ([scaling.md](../../../../docs/history/m1a_rust/measurements/scaling.md)), so
/// `auto` is a convenience and `--jobs N` is what a measurement uses.
fn jobs_spec(s: &str) -> Result<i64, String> {
    if s.eq_ignore_ascii_case("auto") {
        return Ok(0);
    }
    match ein_core::parse_int(s) {
        Some(n) if n >= 1 => Ok(n),
        Some(n) => Err(format!(
            "invalid job count: '{n}' (expected 1 or more, or 'auto')"
        )),
        None => Err(format!(
            "invalid job count: '{s}' (expected an integer or 'auto')"
        )),
    }
}

/// `-n/--solutions` — a count of **one or more**.
///
/// Zero is refused for the reason [`jobs_spec`] refuses `--jobs 0`: it has two
/// readings and the engine implements a third. `stop_after` is compared with
/// `>=` *after* a model is recorded (`ein_infer`'s `commit_entering`), so
/// `Some(0)` — and, before this validator, every negative, which the CLI
/// clamped onto it — cuts at the **first** model and is `-n 1` under another
/// name. Neither reading a user could mean is available: *record nothing* is
/// not what it does, and *no limit* is already spelled `--exhaustive`.
///
/// Refusing them moves no recorded output: no golden under `tests/golden/` is
/// a `solve` invocation at all, and every `-n` in the corpus is `-n 3`. M1e
/// [S1e.1.5](../../../../docs/history/m1e_review_processing/README.md#s1e15---n-0-q7),
/// the review's `Q7`.
///
/// The non-integer message stays [`int_arg`]'s, because
/// [`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)
/// quotes it verbatim, with `--solutions` as its example.
fn solutions_spec(s: &str) -> Result<i64, String> {
    match int_arg(s)? {
        n if n >= 1 => Ok(n),
        n => Err(format!(
            "invalid solution count: '{n}' (expected 1 or more, or --exhaustive)"
        )),
    }
}

/// `--region-limit`'s `n` — **not negative**, the
/// [S5](../../../../plans/m1e2_correctness/README.md#p1e23--surfaces-that-misreport-the-answer-proposed)
/// shape `int_arg` alone would let through.
fn region_limit_spec(s: &str) -> Result<i64, String> {
    match int_arg(s)? {
        n if (0..=63).contains(&n) => Ok(n),
        n => Err(format!(
            "invalid region limit: '{n}' (expected 0 to 63 candidates)"
        )),
    }
}

/// An artefact flag's path — **not empty**.
///
/// M1e [S1e.3.5](../../../../docs/history/m1e_review_processing/README.md#s1e35--error-handling-medium),
/// `EH-M1`'s neighbour and found by its probe. Five options name a file or a
/// directory the run will write, and an empty string reached all five: three
/// of them then failed with a bare `No such file or directory` and exit 0,
/// and **`--dump-states ""` succeeded** — `create_dir_all("")` is `Ok` on
/// Linux, so `""/00_timeline.jsonl` resolved to the *current directory* and
/// the run silently dropped four files into the caller's cwd.
///
/// Refused here rather than at the write, for `solutions_spec`'s reason: an
/// empty path is a usage error, the CLI is where usage errors are answered,
/// and a policy that lives in two places is a policy with two answers.
fn artefact_path(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("expected a path, got an empty string".to_string());
    }
    Ok(s.to_string())
}

/// A float argument, through [`ein_core::parse_float`]: [`int_arg`]'s
/// whitespace and `_` rules, plus `inf` / `nan`, `.5`, `1.` and exponents.
fn float_arg(s: &str) -> Result<f64, String> {
    ein_core::parse_float(s).ok_or_else(|| format!("invalid float value: '{s}'"))
}

fn flag(short: char, long: &'static str, help: &'static str) -> Arg {
    Arg::new(long)
        .short(short)
        .long(long)
        .action(ArgAction::SetTrue)
        .help(help)
}

/// The `--rule-mode` option, shared by `render rules` and `render rule`.
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

/// `--events` / `--events-level`, added to *both* `solve` and `saturate`.
fn event_args(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("events")
            .long("events")
            .value_name("FILE.jsonl")
            .value_parser(artefact_path)
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
                 pre-candidate hypothesis skips — ~6x the volume.",
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
        // M1e2 S1e2.1.3 — the two renderers Q-M1e.20 found produced, tested
        // and reachable from no command.
        .subcommand(
            Command::new("program")
                .about(
                    "the program as written: one DOT digraph per group — rules, \
                     ontology, facts, the query. Parsed, never loaded, so a \
                     program the loader refuses still draws",
                )
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("levi")
                        .long("levi")
                        .action(ArgAction::SetTrue)
                        .help(
                            "draw unary and binary facts as Levi-bipartite \
                             octagons rather than labelled arrows",
                        ),
                )
                .arg(rule_mode()),
        )
        .subcommand(
            Command::new("kb")
                .about(
                    "a KB as one DOT digraph, entities fused across forms: FILE \
                     loaded and saturated. A --dump-answer state file is a FILE, \
                     so a model draws the same way root does",
                )
                .arg(Arg::new("file").required(true))
                .arg(Arg::new("since").long("since").value_name("FILE").help(
                    "load and saturate FILE too, and draw at penwidth 3 \
                             every fact it lacks — a model against its \
                             root.ein is what the commitment added",
                ))
                .arg(
                    Arg::new("colour-by")
                        .long("colour-by")
                        .value_name("COLOUR_BY")
                        .value_parser(PossibleValuesParser::new(["relation", "origin", "none"]))
                        .default_value("relation")
                        .help("edge colour: per relation (default), per origin, or none"),
                )
                .arg(
                    Arg::new("no-types")
                        .long("no-types")
                        .action(ArgAction::SetTrue)
                        .help("leave out the type boxes and their is-a edges"),
                )
                .arg(
                    Arg::new("no-instances")
                        .long("no-instances")
                        .action(ArgAction::SetTrue)
                        .help("leave out the instance ovals and every fact edge"),
                ),
        )
        // M1e2 S1e2.1a.2 — the cube a `--dump-region` directory holds, drawn
        // from the files, so the picture is of the run whose answer it shows.
        .subcommand(
            Command::new("region")
                .about(
                    "the solution region as a Hasse diagram: DIR is an `ein solve \
                     --dump-region` directory, drawn with that run's walk on it \
                     (docs/kernel/ir/03-ein-lang/04_dot_rendering.md)",
                )
                .arg(Arg::new("dir").required(true))
                .arg(
                    Arg::new("view")
                        .long("view")
                        .value_name("VIEW")
                        .value_parser(PossibleValuesParser::new([
                            "auto", "full", "border", "lines", "table",
                        ]))
                        .default_value("auto")
                        .help(
                            "'full' every node of the cube; 'border' R, its holes and \
                             every node one edge from it; 'lines' the root, both lines, \
                             the holes and the recorded nodes; 'table' every node as \
                             markdown, at any n; 'auto' (default) full up to n = 6, \
                             border up to 64 nodes, else lines",
                        ),
                ),
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
                        .value_parser(int_arg)
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
                .value_parser(solutions_spec)
                .default_value("1")
                .help("stop after N distinct solutions (default: 1)"),
        )
        .arg(flag(
            'e',
            "exhaustive",
            "do not stop at the first model. Certification also needs the \
             lattice to end within --max-set-size, which `exhausted` in \
             --stats reports (slower)",
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
                .value_parser(int_arg)
                .default_value("5")
                .help("commitment-set depth cap (default: 5)"),
        )
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(float_arg)
                .help("abort after N wall-clock seconds"),
        )
        .arg(
            Arg::new("max-enterings")
                .short('E')
                .long("max-enterings")
                .value_name("MAX_ENTERINGS")
                .value_parser(int_arg)
                .help("abort after N commitment tries"),
        )
        // T1a.7.2.1. The *contract* — what `--jobs N` promises, and the
        // `--unordered` opt-out — is S1a.7.5's; this is the knob the engine
        // already takes, so that the promise can be measured.
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("JOBS")
                .value_parser(jobs_spec)
                .default_value("1")
                .help(
                    "threads for a layer's enterings, or 'auto' \
                     (default: 1). Same verdict, same models, same counters",
                ),
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
             saturates to the same kb (1 + k + one per alive parent, \
             so up to 2k+1 each; adds compile/enqueue/fire/quiesce \
             events to --events)",
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
                .value_parser(int_arg)
                .help("pin the --shuffle RNG seed for a reproducible permutation"),
        )
        // ── progress / diagnostics ──
        .arg(flag(
            'v',
            "verbose",
            "stream per-layer + per-entering progress to stderr",
        ))
        .arg(
            Arg::new("layer-progress")
                .long("layer-progress")
                .action(ArgAction::SetTrue)
                .help(
                    "stream the per-layer census to stderr and nothing per \
                     entering: three lines a layer — alive, then what the join \
                     proposed and the clauses filtered, then what entering \
                     them killed, learned and found",
                ),
        )
        .arg(
            Arg::new("progress-every")
                .short('g')
                .long("progress-every")
                .value_name("PROGRESS_EVERY")
                .value_parser(int_arg)
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
                .value_parser(artefact_path)
                .help(
                    "persist the engine dump tree (root snapshot per layer + \
                     timeline) to DIR",
                ),
        )
        .arg(
            Arg::new("dump-lattice")
                .long("dump-lattice")
                .value_name("DIR")
                .value_parser(artefact_path)
                // One dumper per solve: each of these chooses a different one.
                .conflicts_with_all(["dump-states", "verbose", "layer-progress", "timing"])
                .help(
                    "persist every commitment the search tested, with its \
                     firings and outcome, to DIR (enterings/, \
                     proof_summary.json — docs/kernel/inference/lattice_dump.md). \
                     One dumper per solve, so not with --dump-states, -v, \
                     --layer-progress or -t",
                ),
        )
        .arg(
            Arg::new("dump-answer")
                .long("dump-answer")
                .value_name("DIR")
                .value_parser(artefact_path)
                .help(
                    "write the answer as files to DIR: the program, root, one \
                     loadable file per model and open state with the \
                     commitment that reached it, every refuted commitment, \
                     and answer.json (= --json-summary). DIR is made before \
                     the search; past that, stdout, stderr and the exit code \
                     are unchanged.",
                ),
        )
        // M1e2 S1e2.1a.1 — the oracle's door, its limit, and its states.
        .arg(
            Arg::new("dump-region")
                .long("dump-region")
                .value_name("DIR")
                .value_parser(artefact_path)
                .help(
                    "write the solution region to DIR: every subset of the root \
                     candidates saturated by the program's rules alone, with no \
                     search machinery (region.json, region.md), and this run's \
                     walk over the lattice (walk.json) — \
                     docs/kernel/inference/region_dump.md. Runs after the solve; \
                     stdout, the exit code and the other artefacts are unchanged.",
                ),
        )
        .arg(
            Arg::new("region-limit")
                .long("region-limit")
                .value_name("N")
                .value_parser(region_limit_spec)
                .default_value("16")
                .help(
                    "(--dump-region) the most root candidates the region is \
                     computed for: 2^N saturations. Above it the directory holds \
                     the walk and a note says why; 17-19 warns, 20 and more \
                     warns with the projected cost.",
                ),
        )
        .arg(
            Arg::new("region-states")
                .long("region-states")
                .action(ArgAction::SetTrue)
                .help(
                    "(--dump-region) also keep every node's state beyond \
                     root's, as text, in region.json",
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
        // ── the model set's projection (M1d S1d.3.3) ──
        .arg(
            Arg::new("models")
                .long("models")
                .value_name("FORM")
                .value_parser(PossibleValuesParser::new(["list", "key", "grid"]))
                .default_value("list")
                .help(
                    "how to print the models: 'list' (a block per model, the \
                     default); 'key' (the smallest set of slots that tells \
                     them apart, and the table of combinations that occur — an \
                     ambiguous verdict only); 'grid' (each model and open \
                     state as a table, the anchor type's objects down the \
                     side, where the program's declarations license one). \
                     Changes stdout and nothing recorded",
                ),
        )
        .arg(
            Arg::new("diff-models")
                .long("diff-models")
                .value_name("FILE")
                .help(
                    "after the answer, compare its models with the ones FILE \
                     lists ('closes R …', then 'model' and one fact per line) \
                     and print what matches, what is missing, what is \
                     spurious and what is one fact off. Changes stdout and \
                     nothing recorded",
                ),
        )
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
                .value_parser(artefact_path)
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
                .value_parser(artefact_path)
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

/// `ein test` — M1c
/// [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test).
///
/// **Small on purpose.** The only engine knobs here are the three that decide
/// whether a run *can finish* — the lattice depth cap and the two budgets —
/// because everything else `solve` offers is a way to ask a different
/// question, and this command asks the one the file already asked. In
/// particular there is no `-n` and no `--exhaustive`: an expectation is a
/// claim about the exhausted answer, so exhausting is the behaviour rather
/// than a flag ([`crate::test`]).
///
/// The two file-only flags are here because the stage's acceptance asks for
/// them by name: a failing expectation is exactly when someone wants the
/// stream. Each still names one path, so each is refused on a selection that
/// is more than one run.
fn test_command() -> Command {
    let cmd = Command::new("test")
        .about("run the expectations the programs state about themselves (M1c)")
        .arg(
            Arg::new("path")
                .required(true)
                .num_args(1..)
                .value_name("PATH")
                .help("a .ein file, or a directory to walk for them"),
        )
        .arg(
            Arg::new("max-set-size")
                .short('m')
                .long("max-set-size")
                .value_name("MAX_SET_SIZE")
                .value_parser(int_arg)
                .default_value("5")
                .help("commitment-set depth cap (default: 5)"),
        )
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(float_arg)
                .help(
                    "abort a query after N wall-clock seconds. An abort is an \
                     ERROR, never a pass and never a failure: nothing was \
                     established either way",
                ),
        )
        .arg(
            Arg::new("max-enterings")
                .short('E')
                .long("max-enterings")
                .value_name("MAX_ENTERINGS")
                .value_parser(int_arg)
                .help("abort a query after N commitment tries (as --max-time)"),
        )
        .arg(flag(
            'v',
            "verbose",
            "one line per query, held ones included, with the verdict and k",
        ))
        .arg(flag(
            'q',
            "quiet",
            "the summary line, and whatever was not ok",
        ))
        .group(
            ArgGroup::new("volume")
                .args(["verbose", "quiet"])
                .multiple(false),
        );
    let cmd = event_args(cmd);
    cmd.arg(
        Arg::new("json-summary")
            .long("json-summary")
            .value_name("FILE.json")
            .value_parser(artefact_path)
            .help(
                "write the structured run summary (verdict, model, every \
                 engine counter, the root-saturation shape, the resolved \
                 config) to FILE as JSON. Additive, and refused when the \
                 selection is more than one run.",
            ),
    )
    .arg(
        Arg::new("json-report")
            .long("json-report")
            .value_name("FILE.json")
            .value_parser(artefact_path)
            .help(
                "write one row per (query …) of the whole selection — the \
                 claim's shape, the relations its :goal closes, the outcome, \
                 and whether the search exhausted — to FILE as JSON. \
                 Additive. Unlike --json-summary it is the report of the run \
                 rather than of one query, so it takes any selection.",
            ),
    )
}

/// `ein saturate`'s **own** parser, with its own `prog`.
///
/// Registered in the top-level tree as a bare subcommand purely so it appears
/// in `ein --help`; the dispatcher intercepts `saturate` before `clap` ever
/// reaches it.
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
                    .value_parser(int_arg)
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
                    .value_parser(int_arg)
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
    /// `ein kb` — P1a.8's surface. Registered only when the `einb` feature is
    /// on, so a build without the container does not advertise it.
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
        // `clap`'s own is disabled: the flag is registered below and handled
        // in `run` before the parser, because this command requires a
        // subcommand and `--version` has none.
        .disable_version_flag(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("version")
                .long("version")
                .short('V')
                .action(ArgAction::SetTrue)
                .help("engine version, event protocol, features, stdlib hash"),
        )
        .subcommand(render_command())
        .subcommand(solve_command())
        .subcommand(test_command())
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
