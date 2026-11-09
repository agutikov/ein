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
            "invalid solution count: '{n}' (expected 1 or more; leave -n out to describe all of R)"
        )),
    }
}

/// `2^24 = 16777216`: the default budget as the help states it, from the one
/// constant that defines it ([`ein_infer::downset::DEFAULT_BUDGET`]).
fn default_budget() -> String {
    let b = ein_infer::downset::DEFAULT_BUDGET;
    format!("2^{} = {b}", b.trailing_zeros())
}

/// `--region-budget`'s count — one or more: a budget of nothing would be a
/// run that answers nothing, which is not a question anyone asks.
fn region_budget_spec(s: &str) -> Result<i64, String> {
    match int_arg(s)? {
        n if n >= 1 => Ok(n),
        n => Err(format!(
            "invalid region budget: '{n}' (expected 1 or more facts)"
        )),
    }
}

/// `--region-limit`'s `n` — **not negative**, the
/// [S5](../../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/README.md#the-re-take-2026-11-02)
/// shape `int_arg` alone would let through.
fn region_limit_spec(s: &str) -> Result<i64, String> {
    match int_arg(s)? {
        n if (0..=63).contains(&n) => Ok(n),
        n => Err(format!(
            "invalid region limit: '{n}' (expected 0 to 63 candidates)"
        )),
    }
}

/// `-g/--progress-every`'s `N` — **0 or more**: 0 logs no node line, as
/// `ein saturate --progress-every 0` disables its progress, and a negative is
/// refused (M1e2 S1e2.3.4, the user's ruling (b) of 2026-11-03, which closed
/// Q-M1e.17). The region modes clamped it with `.max(1)`, so `-g=0` and
/// `-g=-3` logged every node, as `-g 1` does, with no word.
fn progress_every_spec(s: &str) -> Result<i64, String> {
    match int_arg(s)? {
        n if n >= 0 => Ok(n),
        n => Err(format!(
            "invalid progress interval: '{n}' (expected 0 or more nodes; 0 logs none)"
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

/// `--require-why`, on every command that loads a program — M1f S1f.5.14,
/// T1f.5.14.6, on the user's rulings of 2026-11-08: `:why` is optional in the
/// grammar, and a loader flag makes it required, default false. It sets
/// [`ein_ir::LoadOptions::require_why`] and nothing else, so it decides
/// whether a program loads and never what a loaded one answers (*"yes, the
/// flag only decides loading"*). One builder, as [`rule_mode`] and
/// [`event_args`] are, so the seven commands cannot describe it differently.
fn require_why() -> Arg {
    Arg::new("require-why")
        .long("require-why")
        .action(ArgAction::SetTrue)
        .help(
            "refuse to load a program in which a relation, rule, hrule or \
             macro carries no :why \"…\", one line per form. It decides \
             whether a program loads, never what a loaded one answers (M1f \
             T1f.5.14.6)",
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
                )
                .arg(require_why()),
        )
        // M1e2 S1e2.1a.2 — the cube an `ein oracle --dump` directory holds,
        // drawn from the files, so the picture is of the run whose answer it
        // shows; S1e2.1a.7 — a region run on it, from the run's own dumps.
        .subcommand(
            Command::new("region")
                .about(
                    "the solution region as a Hasse diagram: DIR is an `ein oracle \
                     --dump` directory, drawn with a `solve` run on it when --visited \
                     or --answer names its dumps \
                     (docs/kernel/ir/03-ein-lang/04_dot_rendering.md)",
                )
                .arg(Arg::new("dir").required(true))
                .arg(Arg::new("visited").long("visited").value_name("DIR").help(
                    "a `solve --dump-lattice` directory: draw the nodes that run \
                             visited, each read against the cube",
                ))
                .arg(Arg::new("answer").long("answer").value_name("DIR").help(
                    "a `solve --dump-answer` directory: draw the states that run \
                             reported, against the part of R it asked (MISSING, SPURIOUS)",
                ))
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
                             the holes and what the run got wrong; 'table' every node \
                             as markdown, at any n; 'auto' (default) full up to n = 6, \
                             border up to 64 nodes, else lines",
                        ),
                ),
        )
        .subcommand(
            Command::new("lattice")
                .about(
                    "the nodes a describing `ein solve` run visits, as a DAG: R \
                     green, owing amber, dead red (M1e2)",
                )
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("view")
                        .long("view")
                        .value_name("VIEW")
                        .value_parser(PossibleValuesParser::new(["full", "solution"]))
                        .default_value("solution")
                        .help(
                            "'solution' the nodes of R and the nodes one fact above \
                             them that are not (default), or 'full' every node the \
                             run visited",
                        ),
                )
                .arg(
                    Arg::new("region-budget")
                        .long("region-budget")
                        .value_name("FACTS")
                        .value_parser(region_budget_spec)
                        .help(format!(
                            "the run's budget, as `ein solve --region-budget`'s \
                             (default 2^{}); a run that reaches it is drawn as far \
                             as it went",
                            ein_infer::downset::DEFAULT_BUDGET.trailing_zeros()
                        )),
                )
                // The walk's depth cap, retired with `solve -m`.
                .arg(
                    Arg::new("max-set-size")
                        .long("max-set-size")
                        .hide(true)
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(require_why()),
        )
}

/// `ein solve` — the one command for R, `solve_interface.md` § 2.1. Since M1e2
/// S1e2.1a.5's switch it describes R by default, and `--minimal`, `--maximal`,
/// `--single` and `-n N` each ask one question about it; since T1e2.1c.4.6
/// `--count` asks how many states and commitments R holds.
///
/// The nine options the switch retired, and the two levers S1e2.1c.5 retired,
/// stay in the parser, **hidden**, so that a stale argv is refused with the
/// sentence naming what replaced it (`crate::solve::RETIRED`) rather than with
/// clap's "unexpected argument".
fn solve_command() -> Command {
    let retired = |id: &'static str, short: Option<char>, takes_value: bool| {
        let mut a = Arg::new(id).long(id).hide(true);
        if let Some(c) = short {
            a = a.short(c);
        }
        if takes_value {
            a.num_args(1).value_parser(clap::value_parser!(String))
        } else {
            a.action(ArgAction::SetTrue)
        }
    };
    let cmd = Command::new("solve")
        .about(
            "the solution region R: describe it, or ask one question about it \
             (--minimal, --maximal, --single, -n N, --count)",
        )
        .arg(Arg::new("file").required(true))
        .arg(require_why())
        // ── the question: which part of R, and what is asked of it ──
        .arg(
            Arg::new("solutions")
                .short('n')
                .long("solutions")
                .value_name("N")
                .value_parser(solutions_spec)
                .help(
                    "N states of the chosen part of R — all of it, or the line \
                     --minimal or --maximal names — in the route's own order, \
                     the same under every lever and -j. Unset, R is described",
                ),
        )
        .group(
            ArgGroup::new("stop")
                .args(["solutions", "single"])
                .multiple(false),
        )
        .arg(
            Arg::new("minimal")
                .long("minimal")
                .action(ArgAction::SetTrue)
                .help(
                    "the minimal line: the commitments of R that nothing can be \
                     taken from, and their states",
                ),
        )
        .arg(
            Arg::new("maximal")
                .long("maximal")
                .action(ArgAction::SetTrue)
                .help(
                    "the maximal line: the states of R that nothing can be added \
                     to, found by climbing from each member of R",
                ),
        )
        .group(
            ArgGroup::new("which")
                .args(["minimal", "maximal"])
                .multiple(false),
        )
        .arg(
            Arg::new("single")
                .long("single")
                .action(ArgAction::SetTrue)
                .help(
                    "expect exactly one state in the chosen part of R, answered \
                     Solution, Ambiguity, Contradiction or Open; every answer \
                     exits 0, and a run that stopped before it decided says so",
                ),
        )
        // M1e2 T1e2.1c.4.6, the user's ruling of 2026-11-03: R's two counts
        // and nothing listed. Its refusals name what answers the other
        // question (`crate::modes`'s `COUNT_REFUSES`), so they are not
        // clap's conflicts.
        .arg(
            Arg::new("count")
                .long("count")
                .action(ArgAction::SetTrue)
                .help(
                    "how many states and commitments R holds: describe's route, \
                     R's two counts and the rows that qualify them, and nothing \
                     listed. No state is read, so no query is answered and no \
                     claim checked. Over all of R only",
                ),
        )
        // M1e2 S1e2.1b.5, approved 2026-11-03: a question about the program.
        .arg(
            Arg::new("monotone")
                .long("monotone")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([
                    "solutions",
                    "single",
                    "minimal",
                    "maximal",
                    "region-budget",
                    "trace",
                    "dump-lattice",
                    "diff-models",
                    "models",
                    "print-final-state",
                    "print-final-positive",
                    "print-final-hfacts",
                    "hyp-stats",
                    "timing",
                ])
                .help(
                    "is the program monotone? Root's fixpoint and the static check, \
                     and nothing else: the sites, what each puts at stake, the \
                     cycles and order findings. No node is built and R is not \
                     described; every verdict exits 0",
                ),
        )
        .arg(
            Arg::new("region-budget")
                .long("region-budget")
                .value_name("FACTS")
                .value_parser(region_budget_spec)
                .help(format!(
                    "the budget: facts derived beyond root, summed over every \
                     saturation (default {}). A run that reaches it prints what it \
                     established and bounds, never a different answer",
                    default_budget()
                )),
        )
        .arg(
            Arg::new("region-limit")
                .long("region-limit")
                .value_name("N")
                .value_parser(region_limit_spec)
                .default_value("16")
                .help(
                    "the most candidates a program the static check flags gets \
                     the whole cube for: 2^N saturations (default 16). Past it a \
                     question that needs the cube is answered in part",
                ),
        )
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(float_arg)
                .help(
                    "abort after N wall-clock seconds, exit 2; checked before each \
                     step of work. A machine-dependent limit is never an answer",
                ),
        )
        // T1a.7.2.1's knob, the same under every route.
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("JOBS")
                .value_parser(jobs_spec)
                .default_value("1")
                .help(
                    "threads for each step's nodes, or 'auto' (default: 1). The \
                     same read-out, byte for byte",
                ),
        )
        // ── a check: never changes an answer ──
        .arg(flag(
            'y',
            "lattice-sanity-check",
            "the parent-path spot check, unbounded: every one-step parent of \
             every printed state, built and saturated again. A row of its own; \
             a mismatch on a monotone program is an engine error (exit 2), on \
             a flagged one a warning",
        ))
        // ── progress, on stderr ──
        .arg(flag(
            'v',
            "verbose",
            "stream the route's steps to stderr as it takes them, and every \
             --progress-every-th node",
        ))
        .arg(
            Arg::new("layer-progress")
                .long("layer-progress")
                .action(ArgAction::SetTrue)
                .help(
                    "stream the route's steps to stderr as it takes them, one line \
                     per route-1 layer or mode batch: visited, dead, owing, in R, \
                     facts",
                ),
        )
        .arg(
            Arg::new("progress-every")
                .short('g')
                .long("progress-every")
                .value_name("PROGRESS_EVERY")
                .value_parser(progress_every_spec)
                .allow_negative_numbers(true)
                .default_value("100")
                .help(
                    "under --verbose, log every N-th node (default 100; 1 logs \
                     every node, 0 none: the step lines alone). A negative is \
                     refused",
                ),
        )
        // ── what is printed beside the read-out ──
        .arg(flag(
            'c',
            "dump-config",
            "print the resolved config before the read-out",
        ))
        .arg(flag(
            'H',
            "hyp-stats",
            "after the read-out, H, the universe: the candidates by relation, \
             each dead alone marked. Its candidates share stdout's 100 lines \
             with the read-out; --dump-answer DIR writes it whole, as \
             DIR/forms/hyp-stats.txt",
        ))
        .arg(flag(
            't',
            "timing",
            "after the read-out, where the wall clock went: load, the route, the \
             read-out, the print",
        ))
        .arg(flag(
            's',
            "stats",
            "print the wall clock under the read-out",
        ))
        // ── the listed states' projection (M1d S1d.3.3) ──
        .arg(
            Arg::new("models")
                .long("models")
                .value_name("FORM")
                .value_parser(PossibleValuesParser::new(["list", "key", "grid"]))
                .default_value("list")
                .help(models_help(
                    "each as a table, the anchor type's objects down the side, \
                     where the program's declarations license one",
                    "--dump-answer DIR",
                )),
        )
        .arg(
            Arg::new("diff-models")
                .long("diff-models")
                .value_name("FILE")
                .help(
                    "after the read-out, compare the states it showed with the \
                     models FILE lists ('closes R …', then 'model' and its facts, read as a \
                     program's, so a fact may span lines and carry a comment): \
                     what matches, what is missing, what is spurious and \
                     what is one fact off, in what the read-out left of stdout's \
                     100 lines. --dump-answer DIR writes it uncut, over every \
                     listed state, as DIR/forms/diff-models.txt",
                ),
        )
        .arg(flag(
            'p',
            "print-final-state",
            "after the read-out, each state it showed, its facts beyond the \
             program as loaded: root's derivations, its commitment and what that \
             derives, in what the read-out left of stdout's 100 lines. \
             --dump-answer DIR writes every listed state's, as \
             DIR/forms/print-final-state.txt",
        ))
        .arg(flag(
            'P',
            "print-final-positive",
            "like --print-final-state but drops the (not …) facts; \
             DIR/forms/print-final-positive.txt",
        ))
        .arg(flag(
            'f',
            "print-final-hfacts",
            "after the read-out, each state it showed, its facts of the \
             candidates' relations: what its commitment is made of, in what the \
             read-out left of stdout's 100 lines. --dump-answer DIR writes every \
             listed state's, as DIR/forms/print-final-hfacts.txt",
        ))
        // ── files ──
        .arg(
            Arg::new("dump-answer")
                .long("dump-answer")
                .value_name("DIR")
                .value_parser(artefact_path)
                .help(
                    "write the answer as files to DIR: answer.txt (the read-out \
                     uncut), answer.json (= --json-summary), index.json, one \
                     loadable file per state, the owing frontier, N, and each \
                     form asked for (-H, --models, -p, -P, -f, --diff-models) \
                     uncut under forms/ (docs/kernel/inference/answer_dump.md). \
                     DIR is made before the run; past that, stdout, stderr and \
                     the exit code are unchanged.",
                ),
        )
        .arg(
            Arg::new("dump-lattice")
                .long("dump-lattice")
                .value_name("DIR")
                .value_parser(artefact_path)
                .help(
                    "write every node this run visited, in visit order, to DIR: \
                     nodes.json (ein-lattice/2) and nodes.md \
                     (docs/kernel/inference/lattice_dump.md). DIR is made before \
                     the run; past that, stdout, stderr and the exit code are \
                     unchanged.",
                ),
        )
        // ── retired at the switch: refused, with what replaced each ──
        .arg(retired("exhaustive", Some('e'), false))
        .arg(retired("max-set-size", Some('m'), true))
        .arg(retired("max-enterings", Some('E'), true))
        .arg(retired("lattice-order", Some('o'), true))
        .arg(retired("shuffle", Some('z'), false))
        .arg(retired("seed", Some('d'), true))
        .arg(retired("dump-states", Some('D'), true))
        .arg(retired("dump-region", None, true))
        .arg(retired("region-states", None, false))
        // ── retired at M1e2 S1e2.1c.5, the user's ruling of 2026-11-03 ──
        .arg(retired("no-lookahead", Some('L'), false))
        .arg(retired("no-kill-cache", Some('K'), false));

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
                    "write the read-out as ein-summary/2 to FILE: what was asked, \
                     R as the run established it, every state, the queries and \
                     claims, the work, root and the config. Additive: stdout, \
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
                    "write the region trace to FILE: root's derivation, each listed \
                     state's from its first minimal commitment, and N, each \
                     minimal dead set with why (a file, not stdout)",
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
            "(--trace) append each traced state's whole KB",
        ))
        .arg(flag(
            'R',
            "reorder",
            "(--trace) cluster each section's steps by target entity instead of \
             engine order",
        ))
        .arg(flag(
            'l',
            "relevant",
            "(--trace) prune each derivation to the goal-relevant slice",
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
        .arg(require_why())
        .arg(
            Arg::new("region-budget")
                .long("region-budget")
                .value_name("FACTS")
                .value_parser(region_budget_spec)
                .help(format!(
                    "each file's budget, as `ein solve --region-budget`'s (default \
                     {}). A claim the run could not decide within it is NOT CHECKED",
                    default_budget()
                )),
        )
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(float_arg)
                .help(
                    "abort a file after N wall-clock seconds. An abort is an \
                     ERROR, never a pass and never a failure: nothing was \
                     established either way",
                ),
        )
        // The walk's two, retired at M1e2 S1e2.1a.5's switch with `solve`'s.
        .arg(
            Arg::new("max-set-size")
                .short('m')
                .long("max-set-size")
                .hide(true)
                .num_args(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("max-enterings")
                .short('E')
                .long("max-enterings")
                .hide(true)
                .num_args(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(flag(
            'v',
            "verbose",
            "one line per claim, held ones included, with what the run found",
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
                "write the file's run as ein-summary/2 to FILE, as `ein solve` \
                 does. Additive, and refused when the selection is more than \
                 one file.",
            ),
    )
    .arg(
        Arg::new("json-report")
            .long("json-report")
            .value_name("FILE.json")
            .value_parser(artefact_path)
            .help(
                "write one row per claim and per (query …) of the whole \
                 selection — the claim's shape, the relations its :goal \
                 closes, the outcome, and what the run found — to FILE as \
                 ein-test-report/4. Additive. Unlike --json-summary it is the \
                 report of the selection rather than of one run, so it takes \
                 any selection.",
            ),
    )
}

/// `ein oracle` — the region by brute force, M1e2 S1e2.1a.9 T1e2.1a.9.4: every
/// subset of the root candidates saturated, nothing skipped. It is the
/// reference every `ein solve` mode is held to, and it costs `2ⁿ` on every
/// program, so it runs only when asked for by name
/// ([`solve_interface.md` § 2.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#22-ein-oracle)).
fn oracle_command() -> Command {
    Command::new("oracle")
        .about("the solution region by brute force: every subset of the root candidates saturated (M1e2)")
        .arg(Arg::new("file").required(true))
        .arg(require_why())
        .arg(
            Arg::new("region-limit")
                .long("region-limit")
                .value_name("N")
                .value_parser(region_limit_spec)
                .default_value("16")
                .help(
                    "the most root candidates the cube is computed for: 2^N \
                     saturations. Above it the command refuses; 17-19 warns, \
                     20 and more warns with the projected cost",
                ),
        )
        .arg(
            Arg::new("dump")
                .long("dump")
                .value_name("DIR")
                .value_parser(artefact_path)
                .help(
                    "also write the cube to DIR: region.json and region.md \
                     (docs/kernel/inference/region_dump.md), which `ein render \
                     region DIR` draws",
                ),
        )
        .arg(
            Arg::new("states")
                .long("states")
                .action(ArgAction::SetTrue)
                .help("(--dump) also keep every node's state beyond root's, as text, in region.json"),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("JOBS")
                .value_parser(jobs_spec)
                .default_value("1")
                .help("threads, or 'auto' (default: 1). Same cube, byte for byte"),
        )
        .arg(flag(
            's',
            "stats",
            "print the work: saturations and the wall clock of each phase",
        ))
        // The five `ein solve` takes for the read-out — M1e2 T1e2.1a.5.3.
        .arg(
            Arg::new("max-time")
                .short('T')
                .long("max-time")
                .value_name("MAX_TIME")
                .value_parser(float_arg)
                .help(
                    "abort after N wall-clock seconds, exit 2. Checked before each \
                     batch of the cube's nodes; a stopped cube is never an answer",
                ),
        )
        .arg(
            Arg::new("json-summary")
                .long("json-summary")
                .value_name("FILE.json")
                .value_parser(artefact_path)
                .help(
                    "write the read-out as ein-summary/2 to FILE, as `ein solve` \
                     does. Additive: stdout, stderr and the exit code are unchanged",
                ),
        )
        .arg(
            Arg::new("models")
                .long("models")
                .value_name("FORM")
                .value_parser(PossibleValuesParser::new(["list", "key", "grid"]))
                .default_value("list")
                .help(models_help(
                    "each as a table, where the program's declarations license one",
                    "`ein solve FILE --dump-answer DIR`",
                )),
        )
        .arg(flag(
            'p',
            "print-final-state",
            "after the read-out, each state it showed, its facts beyond the \
             program as loaded: root's derivations, its commitment and what that \
             derives, in what the read-out left of stdout's 100 lines. `ein solve \
             FILE -p --dump-answer DIR` writes every listed state's, as \
             DIR/forms/print-final-state.txt",
        ))
        .arg(flag(
            'P',
            "print-final-positive",
            "like --print-final-state but drops the (not …) facts; \
             DIR/forms/print-final-positive.txt",
        ))
}

/// `--models`' help, `grid`'s gloss and where the rest is aside. The key's
/// decline is said with the bound it is priced against (M1e2 S1e2.4.2), read
/// from the constant, so the help cannot state a bound the renderer does not
/// apply. Where the rest is: M1e2 S1e2.3.3, the user's ruling (a) of
/// 2026-11-03.
fn models_help(grid: &str, dump: &str) -> String {
    format!(
        "after the read-out, the states it showed: 'key' (the smallest set of \
         slots that tells them apart, and the table of combinations that occur; \
         declined, with the reason, past a {} MiB pair table or its search \
         budget) or 'grid' ({grid}), in what the read-out left of stdout's 100 \
         lines. {dump} writes the form uncut, over every listed state, as \
         DIR/forms/key.txt or grid.txt. 'list', the default, adds nothing",
        ein_render::models::PAIR_TABLE_BOUND >> 20
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
            .arg(require_why())
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
                        "hard cap on saturator firings. At N, saturation stops before \
                         it checks for a fixpoint, and a '!! saturator step limit hit' \
                         line names the last firing; the state AFTER, the firing \
                         breakdowns and --dump that follow are the KB where it \
                         stopped. Not an error: the exit code is 0. 0 or less stops \
                         before the first firing. Useful for runaway-debugging on a \
                         fresh input. Default: no cap.",
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
                    ))
                    .arg(require_why()),
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
        .subcommand(oracle_command())
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
