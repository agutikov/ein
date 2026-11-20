//! The argument surface — `ein`, `render`, `show`, `solve`, `oracle`, `test`,
//! and `saturate`'s own.
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
/// includes SMT siblings and E-cores, and more threads than physical P-cores
/// need not be faster
/// ([03-design/08-parallelism.md § 1](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#1-the-contract)), so
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

/// `-n/--solutions` — a count of **one or more** (Q-M1e.24).
///
/// Zero is refused for the reason [`jobs_spec`] refuses `--jobs 0`: a
/// stop-after of zero has no reading a user could mean. *Record nothing* is
/// no question about R, and *no limit* is leaving `-n` out, so accepting it
/// would make `-n 0` another spelling of some other count. A negative is
/// refused with it.
///
/// The non-integer message stays [`int_arg`]'s, because
/// [`07-diagnostics.md` §2](../../../../docs/ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes)
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

/// `--region-limit`'s `n` — **not negative**, a shape `int_arg` alone would
/// let through.
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
/// refused (Q-M1e.17): a clamp with `.max(1)` would log every node for `-g=0`
/// and `-g=-3`, as `-g 1` does, with no word.
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
/// Several options name a file or a directory the run will write
/// ([`07-diagnostics.md` § 2.4](../../../../docs/ein/user/04-reference/07-diagnostics.md#24-an-artefact-flags-failure)),
/// and an empty string would reach each: a file write would fail with a bare
/// `No such file or directory`, and a directory would **succeed** —
/// `create_dir_all("")` is `Ok` on Linux, so `""/<file>` resolves to the
/// *current directory* and the run would silently drop its files into the
/// caller's cwd.
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

/// `--fold`, on every view that draws facts and on `ein solve`'s trace: a
/// fact of three arrows, a relation and two arguments, that nothing drawn
/// points at is drawn as one arrow from its first argument to its second,
/// and a fact of two arrows as a loop on its argument, labelled with the
/// relation's name (Q-M1f.425); by default every fact is drawn in the
/// canonical form, an octagon with no name in it whose first arrow is at its
/// relation (Q-M1f.419). One builder, so the views cannot describe it
/// differently. It changes a drawing and nothing else.
fn fold() -> Arg {
    Arg::new("fold")
        .long("fold")
        .action(ArgAction::SetTrue)
        .help(
            "draw a fact of three arrows (a relation and two arguments) that \
             nothing drawn points at as one arrow from its first argument to its \
             second, and a fact of two arrows as a loop on its argument, \
             labelled with the relation. By default every fact is an octagon \
             with no name in it, its arrow 1 at the relation's atom and 2, 3, … \
             at its arguments",
        )
}

/// `--in-flight`, on every command that builds the states of a search:
/// `solve`, `test`, `oracle` and `render lattice`. The pass that writes a
/// state's `open` facts runs at root, and above it only where a state is
/// printed, saved or uses them (Q-M1f.655); the option runs it at every state
/// the run builds, so that a hypothesis first held above root is found in
/// flight wherever it is (Q-M1f.656). It changes no R and no printed state,
/// only what the run checks and what it costs. One builder, so the four
/// commands describe it alike.
pub fn in_flight() -> Arg {
    Arg::new("in-flight")
        .long("in-flight")
        .action(ArgAction::SetTrue)
        .help(
            "write every state's open facts, not only root's and those of the \
             states printed or saved, and check at each whether it holds a \
             hypothesis root does not, which makes the program not monotone, \
             found in flight. It changes no R and no printed state; it costs \
             the pass at every state",
        )
}

/// `--no-require-why`, on every command that loads a program. A `:why` is
/// optional in the grammar and required by the loader by default, and this
/// option, the CLI's spelling of a negative (`--no-search`, `--no-diagrams`),
/// loads a program without them (Q-M1f.62). It sets
/// [`ein_ir::LoadOptions::require_why`] false and nothing else, so it decides
/// whether a program loads and never what a loaded one answers. Every view
/// that loads a program takes it, the three that draw a program as written
/// since they read it through the loader first (Q-M1f.71). One builder, as
/// [`rule_mode`] and [`event_args`] are, so the eleven commands cannot
/// describe it differently.
fn no_require_why() -> Arg {
    Arg::new("no-require-why")
        .long("no-require-why")
        .action(ArgAction::SetTrue)
        .help(
            "load a program in which a relation, rule or macro carries \
             no :why \"…\". By default such a program is refused, one line per \
             form. It decides whether a program loads, never what a loaded one \
             answers",
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
                 docs/ein/ein.rs/05-events.md.",
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

/// `ein show` — what a program holds, listed as Ein text: stdout is forms a
/// `.ein` file could hold and nothing else. A group, as `ein render` is, so
/// that another thing to show is one more subcommand; `hyps` is the one
/// there is.
fn show_command() -> Command {
    Command::new("show")
        .about("list what a program holds, as Ein text (render draws, show lists)")
        .subcommand_required(true)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("hyps")
                .about(
                    "the hypotheses: the program's hypothesis set H as Ein facts, one \
                     candidate per line as the fact is written, every candidate, in the \
                     universe's order, and nothing else on stdout; a note on stderr says \
                     which writers built H and how many it holds. It is the set every route \
                     searches over, and listing it costs root's saturation and the \
                     generator, no search. It asks about H, not about R, and changes no \
                     answer (docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md, § 1.3)",
                )
                .arg(Arg::new("file").required(true))
                .arg(no_require_why()),
        )
}

/// `ein render` — DOT only, no SVG.
fn render_command() -> Command {
    Command::new("render")
        .about("DOT for rules / constraints")
        .subcommand_required(true)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("rules")
                .about("one DOT digraph per rule")
                .arg(Arg::new("file").required(true))
                .arg(rule_mode())
                .arg(fold())
                .arg(no_require_why()),
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
                .arg(rule_mode())
                .arg(fold())
                .arg(no_require_why()),
        )
        .subcommand(
            Command::new("constraints")
                .about(
                    "the facts about relations: every fact root holds, at its fixpoint, \
                     with a relation of the program directly under an argument arrow, \
                     drawn as `render kb` draws a fact, with what it points at",
                )
                .arg(Arg::new("file").required(true))
                .arg(fold())
                .arg(no_require_why()),
        )
        // Two renderers a command reaches as every renderer is (Q-M1e.20).
        .subcommand(
            Command::new("program")
                .about(
                    "the program as written, in three parts: its knowledge base as \
                     one DOT digraph, drawn as `render kb` draws one; a digraph per \
                     rule; the query. Read through the loader first, so a program it \
                     refuses is refused, as every command refuses it; drawn as \
                     parsed, its imports unresolved",
                )
                .arg(Arg::new("file").required(true))
                .arg(fold())
                .arg(rule_mode())
                .arg(no_require_why()),
        )
        .subcommand(
            Command::new("kb")
                .about(
                    "a KB as one DOT digraph, entities fused across forms: FILE \
                     loaded and saturated. A --dump-answer state file is a FILE, \
                     so a model draws the same way root does. --goal or --query \
                     draws a goal's slice of it",
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
                    Arg::new("goal")
                        .long("goal")
                        .value_name("GOAL")
                        .conflicts_with("query")
                        .help(
                            "draw the goal's slice: the facts of the KB drawn that answer \
                             GOAL, written as a (query :goal …) writes it, with what they \
                             point at. Held to what a query's :goal is held to, and \
                             refused as one would be",
                        ),
                )
                .arg(
                    Arg::new("query")
                        .long("query")
                        .value_name("N")
                        .value_parser(clap::value_parser!(u64).range(1..))
                        .help(
                            "draw the slice of the goal of the program's N-th (query …), \
                             from 1: the facts of the KB drawn that answer it, with what \
                             they point at; its :over and :expect are not read",
                        ),
                )
                .arg(fold())
                .arg(no_require_why()),
        )
        // The cube an `ein oracle --dump` directory holds, drawn from the
        // files, so the picture is of the run whose answer it shows; and a
        // region run on it, from the run's own dumps.
        .subcommand(
            Command::new("region")
                .about(
                    "the solution region as a Hasse diagram: DIR is an `ein oracle \
                     --dump` directory, drawn with a `solve` run on it when --visited \
                     or --answer names its dumps \
                     (docs/ein/user/04-reference/12-rendering.md)",
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
                     green, owing amber, dead red",
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
                .arg(no_require_why())
                .arg(in_flight()),
        )
        // The hypothesis space: `ein_infer::structure` reads it and
        // `ein_render::structure` spells it, the command's name among what it
        // spells. `ein solve` computes none of it.
        .subcommand(
            Command::new(ein_render::structure::COMMAND)
                .about(
                    "the hypothesis space of the program: the candidates of its hypothesis \
                     set H and the structure of that set, read from the program's own \
                     lattice with nobody declaring it: which candidates exclude each other, \
                     which of those pairs the theory makes alone, the groups (pick at most \
                     one), the owed groups and grids (exactly one), and what couples the \
                     questions (what each is: docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md; \
                     the command: docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md, § 2.3). \
                     Prints a read-out, or the --view asked. It reads nodes and changes no \
                     answer",
                )
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("view")
                        .long("view")
                        .value_name("VIEW")
                        .value_parser(PossibleValuesParser::new(H_SPACE_VIEWS))
                        .default_value("summary")
                        .help(format!(
                            "what stdout holds: 'summary' (default) a row an object, what it \
                             cost and whatever was stopped, cut or left unread; a graph as \
                             DOT, 'exclusion' the pairs that exclude, the theory's and the \
                             puzzle's in two strokes, 'cover' the groups and grids, \
                             'coupling' the questions' hypergraph, 'one-step' its edges read \
                             one application back; or 'json', the whole document, which \
                             --dump writes as {}",
                            ein_render::structure::DOCUMENT
                        )),
                )
                .arg(
                    Arg::new("dump")
                        .long("dump")
                        .value_name("DIR")
                        .value_parser(artefact_path)
                        .help(format!(
                            "also write the hypothesis space to DIR: {} and the four \
                             graphs, exclusion.dot, cover.dot, coupling.dot and \
                             coupling_one_step.dot. stdout is the same with it",
                            ein_render::structure::DOCUMENT
                        )),
                )
                .arg(
                    Arg::new("supports")
                        .long("supports")
                        .action(ArgAction::SetTrue)
                        .help(
                            "also read the support of every pair root excludes, the stated \
                             facts its death rests on: they label the puzzle's pairs and \
                             name what each tie rests on. One more saturation and a search \
                             per pair",
                        ),
                )
                .arg(
                    Arg::new("region-budget")
                        .long("region-budget")
                        .value_name("FACTS")
                        .value_parser(region_budget_spec)
                        .help(format!(
                            "the budget of root's pair layer and of the small KB's, each: \
                             facts beyond its root, as `ein solve --region-budget`'s (default \
                             2^{}). A layer that reaches it is listed as far as it was read, \
                             and the read-out and the document say what was not",
                            ein_infer::downset::DEFAULT_BUDGET.trailing_zeros()
                        )),
                )
                .arg(
                    Arg::new("jobs")
                        .short('j')
                        .long("jobs")
                        .value_name("JOBS")
                        .value_parser(jobs_spec)
                        .default_value("1")
                        .help(
                            "threads for each step's nodes, or 'auto' (default: 1). The same \
                             structure, byte for byte",
                        ),
                )
                .arg(no_require_why()),
        )
}

/// `ein render h-space --view`'s words: the read-out, each of
/// [`ein_render::structure::GRAPHS`], and the document.
const H_SPACE_VIEWS: [&str; 6] = [
    "summary",
    "exclusion",
    "cover",
    "coupling",
    "one-step",
    "json",
];

/// `ein solve` — the one command for R
/// ([`05-cli.md` § 2.1](../../../../docs/ein/user/04-reference/05-cli.md#21-ein-solve)).
/// It describes R by default, and `--minimal`, `--maximal`, `--single` and
/// `-n N` each ask one question about it; `--count` asks how many states and
/// commitments R holds.
///
/// The retired options and levers stay in the parser, **hidden**, so that a
/// stale argv is refused with the
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
        .arg(no_require_why())
        .arg(in_flight())
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
        // R's two counts and nothing listed (Q-M1f.218). Its refusals name
        // what answers the other
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
        // Two routes for describe and `--count`, the tree and the classes by
        // default, an option forcing either: it chooses a route and never R.
        .arg(
            Arg::new("describe-route")
                .long("describe-route")
                .value_name("ROUTE")
                .value_parser(PossibleValuesParser::new(["tree", "states"]))
                .default_value("tree")
                .conflicts_with_all([
                    "solutions",
                    "single",
                    "minimal",
                    "maximal",
                    "monotone",
                    "no-search",
                ])
                .help(
                    "describe's and --count's route on a monotone program: 'tree', R's \
                     states by the obligation tree and each state's class, where root owes \
                     an instance a licence takes (elsewhere its walk is the state route's, \
                     which is taken), N and an empty R's owing frontier not computed; or \
                     'states', every alive state once, N and the frontier among them. It \
                     changes no part of R",
                ),
        )
        // A question about the program.
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
        // A flag that declines to discover R, never one that changes it
        // (Q-M1e2.30, Q-M1f.258).
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([
                    "solutions",
                    "single",
                    "minimal",
                    "maximal",
                    "count",
                    "monotone",
                ])
                .help(
                    "root alone: saturate root and report it, alive or dead, \
                     discharged or what it owes, and do not search R. It changes no \
                     R, and states none: a claim root does not decide is not \
                     checked. A bare `ein solve FILE` describes R",
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
        // The route workers' count, the same under every route.
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
        // ── the listed states' projection ──
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
                     saved file per state (its program and the choices made, \
                     which load to it), the owing frontier, N, and each \
                     form asked for (-H, -p, -P, -f) \
                     uncut under forms/ (docs/ein/user/04-reference/10-answer-dump.md). \
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
                     (docs/ein/ein.rs/06-lattice-dump.md). DIR is made before \
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
        // ── retired levers ──
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
        .arg(fold().help(
            "(--trace) draw a fact of three arrows (a relation and two \
                 arguments) as one arrow from its first argument to its second, \
                 and a fact of two as a loop, labelled with the relation, in \
                 every drawing of the trace but at a step's premises and \
                 conclusions; by default every fact is an octagon with no name \
                 in it",
        ))
}

/// `ein test`
/// ([`05-cli.md` § 2.3](../../../../docs/ein/user/04-reference/05-cli.md#23-ein-test)).
///
/// **Small on purpose.** The only engine knobs here are the ones that decide
/// whether a run *can finish*, the two budgets, because everything else
/// `solve` offers is a way to ask a different question, and this command
/// asks the one the file already asked. In particular there is no `-n`, no
/// `--single` and no line flag: an expectation is a claim about a whole part
/// of R, so which question decides it is read off the claim's shape and is
/// the runner's to ask, never a flag's ([`crate::test`]).
///
/// The two file-only flags are here because a failing expectation is exactly
/// when someone wants the stream. Each still names one path, so each is
/// refused on a selection that
/// is more than one run. `--diff` explains a claim, its models against what
/// the run found, and decides nothing.
fn test_command() -> Command {
    let cmd = Command::new("test")
        .about("run the expectations the programs state about themselves")
        .arg(
            Arg::new("path")
                .required(true)
                .num_args(1..)
                .value_name("PATH")
                .help("a .ein file, or a directory to walk for them"),
        )
        .arg(no_require_why())
        .arg(in_flight())
        .arg(
            Arg::new("region-budget")
                .long("region-budget")
                .value_name("FACTS")
                .value_parser(region_budget_spec)
                .help(format!(
                    "the budget of each question a file is asked, as `ein solve \
                     --region-budget`'s (default {}). A claim no question decided \
                     within it is NOT CHECKED",
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
        // Root alone, as `ein solve --no-search` reads it (Q-M1f.258).
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .action(ArgAction::SetTrue)
                .help(
                    "ask each file root alone, as `ein solve --no-search` does: a \
                     claim root decides is held or FAILED as the full run decides \
                     it, and every other is NOT CHECKED. R is not searched",
                ),
        )
        // Two retired options, refused as `solve`'s are.
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
            "one line per claim, held ones included, with the question it was read off \
             and what the run found",
        ))
        .arg(flag(
            'q',
            "quiet",
            "the summary line, and whatever was not ok",
        ))
        // A claim's models against what its run found: an explanation after
        // the claim's line, which moves no verdict and no exit code.
        .arg(
            Arg::new("diff")
                .long("diff")
                .value_name("LEVEL")
                .num_args(0..=1)
                .default_missing_value("stats")
                .value_parser(["stats", "full"])
                .help(
                    "after the line of each claim checked, compare the models it \
                     lists with the states, classes or owing nodes the run found, \
                     over what the claim's check compares: `stats` (a bare --diff) \
                     how many match, are near and how many facts off, are missing \
                     and are extra; `full` also each model against the one it \
                     matches or is nearest, its facts the found one lacks (-) and \
                     the found one's beyond it (+). A held claim's diff is all \
                     matches. It moves no verdict and no exit code; under -q, only \
                     the claims printed",
                ),
        )
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
                "write the file's run, the last question it was asked, as \
                 ein-summary/2 to FILE, as `ein solve` does. Additive, and \
                 refused when the selection is more than one file.",
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
                 closes, the outcome, the question it was read off, what \
                 the run found, and under --diff the claim's diff — to FILE \
                 as ein-test-report/6. Additive. \
                 Unlike --json-summary it is the report of the selection \
                 rather than of one run, so it takes any selection.",
            ),
    )
}

/// `ein oracle` — the region by brute force: every
/// subset of the root candidates saturated, nothing skipped. It is the
/// reference every `ein solve` mode is held to, and it costs `2ⁿ` on every
/// program, so it runs only when asked for by name
/// ([`05-cli.md` § 2.2](../../../../docs/ein/user/04-reference/05-cli.md#22-ein-oracle)).
fn oracle_command() -> Command {
    Command::new("oracle")
        .about("the solution region by brute force: every subset of the root candidates saturated")
        .arg(Arg::new("file").required(true))
        .arg(no_require_why())
        .arg(in_flight())
        // Refused with a sentence (`crate::oracle::REFUSED`), as a retired
        // option of `solve` is: the oracle's one question is R (Q-M1f.258).
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .hide(true)
                .action(ArgAction::SetTrue),
        )
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
                     (docs/ein/user/04-reference/11-region-dump.md), which `ein render \
                     region DIR` draws",
                ),
        )
        .arg(
            Arg::new("states")
                .long("states")
                .action(ArgAction::SetTrue)
                .help(
                    "(--dump) also keep every node's state, as text, in region.json: its facts \
                     beyond root's and every open fact it holds, root's too",
                ),
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
        // The four `ein solve` takes for the read-out.
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
            .arg(no_require_why())
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
    /// `ein kb`, the container's surface. Registered only when the `einb` feature is
    /// on, so a build without the container does not advertise it.
    #[cfg(feature = "einb")]
    fn kb_command() -> Command {
        Command::new("kb")
            .about("the .einb binary knowledge-base container")
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
                    .arg(no_require_why()),
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
        .subcommand(show_command())
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
