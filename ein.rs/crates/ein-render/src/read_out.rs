//! The read-out as text: what `ein solve` and `ein oracle` print — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md),
//! the layout [`read_out.md`](../../../../docs/kernel/inference/read_out.md) defines row by row since M1f
//! S1f.5.16, from [`ein_infer::read_out::ReadOut`]; every label is a [`Row`].
//!
//! [`render`] returns stdout, the stderr lines and the exit code, and prints
//! nothing itself. Three rules shape it beyond the rows:
//!
//! - **The listings share 100 lines** ([`LIST_LINES`], the user's ruling of
//!   2026-10-16, the diff's § 4.1). R's states, a line's members, each query's
//!   classes and `as :expect` are printed in that order, each in whole items
//!   while the lines last and at least one. A cut region is not wrapped in
//!   `(region …)`, which would claim it is all of R. [`Options::uncut`] is
//!   what `--dump-answer` writes as `answer.txt`: every listing whole.
//! - **Every answer exits 0** (the diff's § 9). A claim the run refutes exits
//!   1; one it could not check, and a partial run, write a `note:`.
//! - **No verdict word without an expectation**: the four words are
//!   `--single`'s alone.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use ein_infer::described::Described;
use ein_infer::detector::{ASSUMES, Paths, State, Status, Warning};
use ein_infer::expect::Outcome;
use ein_infer::explore::{Ask, Checked, Licence, Route, Stopped, Verdict, Which};
use ein_infer::monotone::Monotonicity;
use ein_infer::read_out::{Listed, Listing, Query, ReadOut};
use ein_infer::region::Class;
use ein_infer::scope::{Outside, Why};
use ein_infer::types_as_objects::TypesAsObjects;
use ein_ir::expect::Over;

/// The rule under the header.
pub const RULE: &str = "──────────────────────────────────────────────────────────────";

/// The most lines stdout's listings print together.
pub const LIST_LINES: usize = 100;

/// Where a row is printed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    /// At two spaces, its value at column 20.
    Row,
    /// A block's row, at four spaces, its value at column 20.
    Block,
    /// `-t`'s table, under its line `timing`: the phase at two spaces padded
    /// to ten, the time right-aligned in twelve, then what the phase did.
    Timing,
    /// A line of its own at the left margin: `ein oracle`'s `the cube`.
    Line,
}

/// Declares [`Row`]: each variant with its label and its layout, and
/// [`Row::ALL`], every variant, so that no label can be added without being
/// in the list.
macro_rules! rows {
    ($($(#[$doc:meta])* $v:ident => $label:literal, $layout:ident;)*) => {
        /// **Every label the read-out prints**, `ein solve`'s and `ein
        /// oracle`'s, `-s`'s and `-t`'s — M1f S1f.5.16's T1f.5.16.3. A row is
        /// printed only through one ([`line()`], and the read-out's private
        /// printers), so a label the reference page,
        /// `docs/kernel/inference/read_out.md`, lacks cannot be printed
        /// without being here, and `ein-cli/tests/read_out_reference.rs`
        /// holds every one to a row of the page's [`Row::TABLES`] table, both
        /// ways and in printing order. Two rows share a label, `…`, and two
        /// are numbered where they print ([`Row::numbered`]).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Row {
            $($(#[$doc])* $v,)*
        }

        impl Row {
            /// Every row, in declaration order.
            pub const ALL: &'static [Row] = &[$(Row::$v),*];

            /// The label as printed; a numbered row's first word.
            pub const fn label(self) -> &'static str {
                match self {
                    $(Row::$v => $label,)*
                }
            }

            /// Where it is printed.
            pub const fn layout(self) -> Layout {
                match self {
                    $(Row::$v => Layout::$layout,)*
                }
            }
        }
    };
}

rows! {
    /// The question.
    Asked => "asked", Row;
    /// R's size in states.
    StatesKs => "states (k_s)", Row;
    /// R's size in commitments.
    CommitmentsKc => "commitments (k_c)", Row;
    /// Whether the run classified every commitment, or listed all of the
    /// part asked.
    Exhausted => "exhausted", Row;
    /// Which kind of empty R is.
    Empty => "empty", Row;
    /// Where root, ∅, stands.
    Root => "∅", Row;
    /// An obligation instance root owes, where root is alive and not in R.
    Owed => "owed", Block;
    /// The minimal line, described or listed.
    MinimalLine => "minimal line", Row;
    /// The maximal line, described or listed.
    MaximalLine => "maximal line", Row;
    /// `-n N`'s count of the states of R it lists.
    StatesOfR => "states of R", Row;
    /// `--single`'s word.
    Verdict => "verdict", Row;
    /// Whether `--single`'s word is certified.
    Decided => "decided", Row;
    /// R's components.
    Components => "components", Row;
    /// R's maximal sub-cubes.
    SubCubes => "sub-cubes", Row;
    /// The states more than one commitment reaches.
    Merged => "merged", Row;
    /// The minimal dead sets.
    N => "N", Row;
    /// The minimal discharging sets.
    E => "E", Row;
    /// The static check, and the warning's block under it.
    Lattice => "lattice", Row;
    /// `--monotone`'s answer.
    Monotone => "monotone", Row;
    /// The rules' negative cycles.
    Strata => "strata", Row;
    /// A minimal hole of the cube.
    MinimalHole => "minimal hole", Block;
    /// H, the universe.
    Universe => "universe", Row;
    /// The route, and its licences.
    Route => "route", Row;
    /// What the run cost.
    Work => "work", Row;
    /// What a node is taken to be on a flagged program.
    Assumes => "assumes", Block;
    /// A site of the static check.
    Site => "site", Block;
    /// The sites past [`WARNING_ROWS`].
    MoreSites => "…", Block;
    /// A cycle through an `absent` over D.
    Cycle => "cycle", Block;
    /// An order finding.
    Order => "order", Block;
    /// `--monotone`'s word on the cube's half.
    Bit => "bit", Block;
    /// Whether a site bit on the cube.
    Status => "status", Block;
    /// The stale facts of the nodes the run computed.
    State => "state", Block;
    /// A node holding a stale fact.
    Stale => "stale", Block;
    /// The stale nodes past [`WARNING_ROWS`].
    MoreStale => "…", Block;
    /// The parent-path spot check.
    Paths => "paths", Block;
    /// A cycle of the rules' dependency graph through an `absent`.
    NegativeCycle => "negative cycle", Block;
    /// A relation H leaves out.
    Outside => "outside", Block;
    /// A type the blind enumerator binds as an object.
    Types => "types", Block;
    /// A fact of root's core.
    UnsatCore => "unsat core", Row;
    /// A cut core search.
    CoreSearch => "core search", Row;
    /// A core fact's `:source`.
    Source => "source", Row;
    /// The rules of the core's last step.
    RefutedBy => "refuted by", Row;
    /// A node of the owing frontier.
    Owing => "owing", Row;
    /// A query's goal: `query`, or `query i of n`.
    Query => "query", Row;
    /// The set a query's classes are over.
    Over => "over", Row;
    /// A projection class: `class i of n`.
    Class => "class", Block;
    /// The claim that states the classes.
    AsExpect => "as :expect", Row;
    /// A query's own claim, checked.
    Expect => ":expect", Row;
    /// The program's `(solve … :expect …)`, checked.
    SolveExpect => "(solve :expect)", Row;
    /// `--count`'s claims, none checked.
    Claims => "claims", Row;
    /// `-s`: the wall clock.
    Wall => "wall", Row;
    /// `-t`: the load.
    TimingLoad => "load", Timing;
    /// `-t`: the route.
    TimingRoute => "route", Timing;
    /// `-t`: the read-out.
    TimingReadOut => "read-out", Timing;
    /// `-t`: the print.
    TimingPrint => "print", Timing;
    /// `-t`: the sum.
    TimingTotal => "total", Timing;
    /// `ein oracle`'s line on the cube.
    TheCube => "the cube", Line;
}

impl Row {
    /// The label as the page writes it: a numbered row by its pattern.
    pub fn patterns(self) -> Vec<String> {
        match self {
            Row::Query => vec!["query".to_string(), "query i of n".to_string()],
            Row::Class => vec!["class i of n".to_string()],
            r => vec![r.label().to_string()],
        }
    }

    /// The label of the `i`-th of `n`: `query` where the program has one
    /// query and `query i of n` where it has several; `class i of n` always.
    pub fn numbered(self, i: usize, n: usize) -> String {
        match self {
            Row::Query if n <= 1 => self.label().to_string(),
            r => format!("{} {i} of {n}", r.label()),
        }
    }

    /// **The reference page's row tables**, by their marker
    /// (`<!-- rows: NAME -->`), each in printing order: a bare `ein solve`
    /// with `--count` and `ein oracle`, a mode, `--monotone`, the three
    /// blocks, why R is empty, the queries, the claims, `-s` and `-t`, and
    /// `ein oracle`'s line.
    pub const TABLES: &'static [(&'static str, &'static [Row])] = &[
        (
            "describe",
            &[
                Row::Asked,
                Row::StatesKs,
                Row::CommitmentsKc,
                Row::Exhausted,
                Row::Empty,
                Row::Root,
                Row::Owed,
                Row::MinimalLine,
                Row::MaximalLine,
                Row::Components,
                Row::SubCubes,
                Row::Merged,
                Row::N,
                Row::E,
                Row::Lattice,
                Row::Strata,
                Row::MinimalHole,
                Row::Universe,
                Row::Route,
                Row::Work,
            ],
        ),
        (
            "mode",
            &[
                Row::Asked,
                Row::Verdict,
                Row::StatesOfR,
                Row::MinimalLine,
                Row::MaximalLine,
                Row::Decided,
                Row::Exhausted,
                Row::Lattice,
                Row::Strata,
                Row::Universe,
                Row::Route,
                Row::Work,
            ],
        ),
        (
            "monotone",
            &[
                Row::Asked,
                Row::Monotone,
                Row::Universe,
                Row::Route,
                Row::Work,
            ],
        ),
        (
            "lattice",
            &[
                Row::Assumes,
                Row::Site,
                Row::MoreSites,
                Row::Cycle,
                Row::Order,
                Row::Bit,
                Row::Status,
                Row::State,
                Row::Stale,
                Row::MoreStale,
                Row::Paths,
            ],
        ),
        ("strata", &[Row::NegativeCycle]),
        ("universe", &[Row::Outside, Row::Types]),
        (
            "empty",
            &[
                Row::UnsatCore,
                Row::CoreSearch,
                Row::Source,
                Row::RefutedBy,
                Row::Owing,
            ],
        ),
        (
            "queries",
            &[
                Row::Query,
                Row::Over,
                Row::Class,
                Row::AsExpect,
                Row::Expect,
            ],
        ),
        ("claims", &[Row::SolveExpect, Row::Claims]),
        (
            "clock",
            &[
                Row::Wall,
                Row::TimingLoad,
                Row::TimingRoute,
                Row::TimingReadOut,
                Row::TimingPrint,
                Row::TimingTotal,
            ],
        ),
        ("oracle", &[Row::TheCube]),
    ];
}

/// What `ein solve`'s cut lines name as the writer of the rest.
const DUMP_FLAG: &str = "--dump-answer DIR";

/// Where a cut listing's rest is, as its line names it: `--dump-answer DIR`
/// on `ein solve`. `ein oracle` writes no answer directory and refuses the
/// flag, so its lines name the command that writes one, over the same R: the
/// forms' since M1e2 S1e2.3.3, the read-out's own since S1e2.3.7 (N4, the
/// user's ruling of 2026-11-03, which added no flag to `ein oracle`).
pub fn dump_hint(command: &str) -> &'static str {
    if command == "oracle" {
        "ein solve FILE --dump-answer DIR"
    } else {
        DUMP_FLAG
    }
}

/// The most minimal commitments a member's comment names; the rest are
/// counted. A `(region …)` lists them all in its `(merged …)`.
const MINIMAL_SHOWN: usize = 4;

/// How the read-out is printed.
#[derive(Clone, Copy, Debug)]
pub struct Options<'a> {
    /// `solve` or `oracle`: the header's first word.
    pub command: &'a str,
    /// The arguments as given, for the header.
    pub argv: &'a str,
    /// The file, for the stderr lines.
    pub file: &'a str,
    /// `--region-budget`, as typed.
    pub budget: u64,
    /// Every listing whole: what `--dump-answer` writes as `answer.txt`.
    pub uncut: bool,
}

/// What the run prints, and how it exits.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text {
    pub stdout: String,
    pub stderr: Vec<String>,
    pub code: i32,
    /// The states stdout printed, as indices into `ReadOut::states` — what
    /// `--trace` traces, and what the forms after the read-out draw from
    /// (M1e2 S1e2.3.3).
    pub shown: Vec<usize>,
    /// The lines the listings spent of [`LIST_LINES`]: what is left of them
    /// is the forms' (M1e2 S1e2.3.3, the user's ruling (a) of 2026-11-03).
    pub listed: usize,
}

impl Text {
    /// What the read-out left of [`LIST_LINES`] for the forms printed after
    /// it.
    pub fn left(&self) -> usize {
        LIST_LINES.saturating_sub(self.listed)
    }
}

/// The lines `block` prints: one per entry, and one more per line break an
/// entry holds, a text's ([`hang`]) — what the 100 lines count (M1f
/// S1f.5.16's T1f.5.16.5).
fn lines_of(block: &[String]) -> usize {
    block.iter().map(|l| 1 + l.matches('\n').count()).sum()
}

/// How many of `blocks` fit in `left` lines: all, or whole blocks while they
/// fit and at least one. The read-out's listings are cut by it, and since
/// M1e2 S1e2.3.3 the forms printed after the read-out too.
pub fn fit(left: usize, blocks: &[Vec<String>]) -> usize {
    let mut lines = 0usize;
    for (i, b) in blocks.iter().enumerate() {
        lines = lines.saturating_add(lines_of(b));
        if lines > left {
            return i.max(1);
        }
    }
    blocks.len()
}

/// `261 982`: digit groups of three, the corpus pages' spelling.
pub fn group(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

/// A count of commitments, as the read-out prints one: grouped, and *or more*
/// at [`ein_infer::count::CAP`], 2⁶³ − 1, the greatest a run reports (M1f
/// S1f.6.25, E-22). The run counted every commitment; the number is past
/// what a surface carries.
pub fn commitments(n: u64) -> String {
    if n >= ein_infer::count::CAP {
        format!("{} or more", group(n))
    } else {
        group(n)
    }
}

fn plural(n: u64, one: &str, many: &str) -> String {
    format!("{} {}", group(n), if n == 1 { one } else { many })
}

/// `(model a b c)` broken at `width` with a hanging indent.
pub fn wrapped(head: &str, items: &[String], indent: usize, width: usize) -> Vec<String> {
    if items.is_empty() {
        return vec![format!("{}({head})", " ".repeat(indent))];
    }
    let mut lines = Vec::new();
    let mut cur = format!("{}({head}", " ".repeat(indent));
    let hang = indent + head.len() + 2;
    let mut first = true;
    for it in items {
        if !first && cur.chars().count() + 1 + it.chars().count() > width {
            lines.push(cur);
            cur = format!("{}{it}", " ".repeat(hang));
        } else {
            cur.push(' ');
            cur.push_str(it);
        }
        first = false;
    }
    cur.push(')');
    lines.push(cur);
    lines
}

/// The text being built, and the listing lines spent.
struct Out {
    text: String,
    stderr: Vec<String>,
    listed: usize,
    uncut: bool,
    shown: Vec<usize>,
    /// What a cut line names as the writer of the rest: [`dump_hint`].
    hint: &'static str,
}

impl Out {
    fn line(&mut self, l: impl AsRef<str>) {
        self.text.push_str(l.as_ref());
        self.text.push('\n');
    }

    fn blank(&mut self) {
        self.text.push('\n');
    }

    /// A row: `r`'s label, then its value at column 20.
    fn row(&mut self, r: Row, value: impl AsRef<str>) {
        let l = line(r, value);
        self.line(l);
    }

    /// A row's continuation: a value at column 20 under no label.
    fn more(&mut self, value: impl AsRef<str>) {
        let l = more_text(value);
        self.line(l);
    }

    /// What is left of [`LIST_LINES`].
    fn left(&self) -> usize {
        if self.uncut {
            usize::MAX
        } else {
            LIST_LINES.saturating_sub(self.listed)
        }
    }

    /// How many of `blocks` fit in what is left: all, or whole blocks while
    /// they fit and at least one.
    fn fit(&self, blocks: &[Vec<String>]) -> usize {
        fit(self.left(), blocks)
    }

    /// Print what [`Self::fit`] allows, count it, and return how many.
    fn blocks(&mut self, blocks: &[Vec<String>]) -> usize {
        let shown = self.fit(blocks);
        for l in blocks[..shown].iter().flatten() {
            self.text.push_str(l);
            self.text.push('\n');
        }
        self.listed += blocks[..shown].iter().map(|b| lines_of(b)).sum::<usize>();
        shown
    }
}

/// Unicode's mandatory line breaks (UAX #14's BK, CR, LF and NL), `\r\n`
/// read as one: where a text a program wrote starts a new line.
fn split_breaks(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut it = text.char_indices().peekable();
    while let Some((i, c)) = it.next() {
        if matches!(
            c,
            '\n' | '\r' | '\u{0B}' | '\u{0C}' | '\u{85}' | '\u{2028}' | '\u{2029}'
        ) {
            out.push(&text[start..i]);
            let mut end = i + c.len_utf8();
            if c == '\r'
                && let Some(&(j, '\n')) = it.peek()
            {
                it.next();
                end = j + 1;
            }
            start = end;
        }
    }
    out.push(&text[start..]);
    out
}

/// `value`, printed at column `col`, with each further line of a text it
/// holds started where its first character is: past its indent and an
/// opening `“` — M1f S1f.5.16's T1f.5.16.5, Q-M1f.19 (a). A `:goal-text`
/// filled for a class's row, and an obligation's `:why` under `owing`, are
/// the program's text, and a line break in one is the writer's; until then
/// its next line started at column 0, outside every row.
fn hang(value: &str, col: usize) -> String {
    let lines = split_breaks(value);
    if lines.len() == 1 {
        return value.to_string();
    }
    let lead = value.chars().take_while(|&c| c == ' ' || c == '“').count();
    let pad = " ".repeat(col + lead);
    lines.join(&format!("\n{pad}"))
}

/// `label` padded to `width` at `indent`, then `value`, its further lines
/// started where it starts ([`hang`]). Private: a label reaches it only
/// through a [`Row`].
///
/// One space at least separates a label from its value, at every width (M1f
/// S1f.5.6, W42): a label as long as its column, `class 100 of 277` at a
/// block's sixteen, was printed with none, so `class 100 of 277` and `28
/// states` read as `class 100 of 27728 states`. Such a label's value starts
/// one past it, and every other row's at its column.
fn padded(indent: usize, width: usize, label: &str, value: &str) -> String {
    let width = width.max(label.chars().count() + 1);
    let col = indent + width;
    format!("{}{label:<width$}{}", " ".repeat(indent), hang(value, col))
}

/// `r` printed with `value`, at its layout: a row at two spaces, a block's
/// row at four, each value at column 20. `ein-cli` prints `-s`'s `wall` row
/// through it.
pub fn line(r: Row, value: impl AsRef<str>) -> String {
    match r.layout() {
        Layout::Row => padded(2, 18, r.label(), value.as_ref()),
        Layout::Block => padded(4, 16, r.label(), value.as_ref()),
        Layout::Timing | Layout::Line => {
            unreachable!("{r:?} has its own layout, printed by `ein-cli`")
        }
    }
}

/// A numbered row, `query 2 of 3` or `class 1 of 4`, at `r`'s layout.
fn numbered_line(r: Row, i: usize, n: usize, value: impl AsRef<str>) -> String {
    let label = r.numbered(i, n);
    match r.layout() {
        Layout::Block => padded(4, 16, &label, value.as_ref()),
        _ => padded(2, 18, &label, value.as_ref()),
    }
}

/// A row's continuation: `value` at column 20, under no label.
fn more_text(value: impl AsRef<str>) -> String {
    padded(2, 18, "", value.as_ref())
}

fn which_words(w: Which) -> &'static str {
    match w {
        Which::All => "all of R",
        Which::Minimal => "the minimal line",
        Which::Maximal => "the maximal line",
    }
}

fn name(cands: &[String], set: &[u32]) -> String {
    if set.is_empty() {
        return "∅".to_string();
    }
    let parts: Vec<&str> = set.iter().map(|&b| cands[b as usize].as_str()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn set_facts(cands: &[String], set: &[u32]) -> Vec<String> {
    let mut v: Vec<String> = set.iter().map(|&b| cands[b as usize].clone()).collect();
    v.sort();
    v
}

/// Print `r` as `ein solve` (or `ein oracle`) does.
pub fn render(r: &ReadOut, o: &Options) -> Text {
    let mut out = Out {
        text: String::new(),
        stderr: Vec::new(),
        listed: 0,
        uncut: o.uncut,
        shown: Vec::new(),
        hint: dump_hint(o.command),
    };
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let line = if a.mode.which == Which::All {
        "R"
    } else {
        which_words(a.mode.which)
    };
    let asked = match a.mode.ask {
        Ask::Describe if a.mode.which == Which::All => "all of R — describe".to_string(),
        Ask::Describe => format!("{line} — every state on it"),
        Ask::Find(n) => format!("{} of {line} (-n {n})", plural(n, "state", "states")),
        Ask::Single => format!("is {line} one state? (--single)"),
        Ask::Count => "how many states and commitments R holds (--count)".to_string(),
    };
    out.line(format!("{} · {}", o.command, o.argv));
    out.line(RULE);
    out.row(Row::Asked, asked);

    match r.described() {
        Some(d) => describe_rows(&mut out, r, d, o),
        None => mode_rows(&mut out, r, o, line),
    }
    listing(&mut out, r, &cands);
    if a.mode.ask == Ask::Count {
        count_claims(&mut out, r, o);
        return Text {
            stdout: out.text,
            stderr: out.stderr,
            code: 0,
            shown: out.shown,
            listed: out.listed,
        };
    }
    let mut code = queries(&mut out, r, o);

    if let Some(claim) = &r.claim {
        out.blank();
        let label = match claim.outcome {
            Outcome::Held => "holds",
            Outcome::Failed => "FAILED",
            Outcome::NotChecked => "NOT CHECKED",
        };
        out.row(Row::SolveExpect, label);
        for l in &claim.lines {
            out.more(l);
        }
        let why = claim.lines.first().cloned().unwrap_or_default();
        match claim.outcome {
            Outcome::Held => {}
            Outcome::Failed => {
                out.stderr
                    .push(format!("{}: (solve :expect) FAILED — {why}", o.file));
                code = 1;
            }
            Outcome::NotChecked => out.stderr.push(format!(
                "note: {}: (solve :expect) NOT CHECKED — {why}",
                o.file
            )),
        }
    }
    Text {
        stdout: out.text,
        stderr: out.stderr,
        code,
        shown: out.shown,
        listed: out.listed,
    }
}

/// Whether the description visited every candidate alone, so that its count
/// of those dead alone is the count: an exhausted run, one complete past layer
/// 1, or a dead root, which leaves every one dead alone without a visit.
pub fn dead_alone_known(d: &Described) -> bool {
    d.exhausted
        || d.complete_up_to.is_some_and(|k| k >= 1)
        || d.states_partial.is_some_and(|p| p.up_to.is_some())
        || !d.root.alive
}

/// The `note:` a partial run writes.
fn partial_note(out: &mut Out, o: &Options, why: String) {
    out.stderr
        .push(format!("note: {}: partial — {why}", o.file));
}

/// The `lattice` row, and on a flagged program the warning under it — M1e2
/// S1e2.1b.5, the wording the user approved on 2026-11-03: what is assumed,
/// each site with what it puts at stake, the cycles and order findings, then
/// whether a site bit, the status and the state apart, and the parent paths.
/// A monotone program's one row is what it was. `holes` is the description's
/// count where it has one; a whole cube's status counts them too.
fn lattice_block(
    out: &mut Out,
    m: &Monotonicity,
    warning: Option<&Warning>,
    spot: Option<&Paths>,
    holes: Option<u64>,
) {
    if m.is_monotone() {
        out.row(
            Row::Lattice,
            "monotone (the static check) — R is convex: no hole",
        );
        // `-y` (M1e2 S1e2.1c.5): the spot check, unbounded, held to the
        // theorem — a path that differed would have been the run's error.
        if let Some(p) = spot {
            out.line(sub_row(Row::Paths, paths_text(p)));
        }
        strata_block(out, m);
        return;
    }
    let holes = holes.or_else(|| warning.and_then(|w| w.status.holes()));
    let h = match holes {
        Some(0) => "no hole in this cube".to_string(),
        Some(h) => plural(h, "hole", "holes"),
        None => "holes not known: the cube was not enumerated".to_string(),
    };
    out.row(
        Row::Lattice,
        format!(
            "not monotone: {} (the static check) — {h}",
            plural(m.sites.len() as u64, "site", "sites")
        ),
    );
    let (uncut, hint) = (out.uncut, out.hint);
    for l in static_rows(m, uncut, hint) {
        out.line(l);
    }
    if let Some(w) = warning {
        for l in dynamic_rows(w, uncut, hint) {
            out.line(l);
        }
    }
    strata_block(out, m);
}

/// The `strata` row, on a program with no stratification only — M1f
/// S1f.5.15's T4, the static non-stratifiability diagnostic: each negative
/// cycle of the rules' dependency graph over every relation, by its relations
/// and the rules on its negative edges. A diagnostic, not a refusal: the
/// answer above stands. It says what the graph licenses and nothing about this
/// run: root's fixpoint *may* depend on the boundary's admission order, and
/// *need not* be unique (`syntax/rule-guards.ein`'s cycle never fires, and its
/// fixpoint is unique). A stratified program prints nothing, as it did.
fn strata_block(out: &mut Out, m: &Monotonicity) {
    if m.negative_cycles.is_empty() {
        return;
    }
    out.row(
        Row::Strata,
        format!(
            "not stratifiable: {} through an absent — root's fixpoint may depend on the \
             boundary's admission order, and need not be unique",
            plural(m.negative_cycles.len() as u64, "cycle", "cycles")
        ),
    );
    for c in &m.negative_cycles {
        out.line(sub_row(Row::NegativeCycle, c.stratum_text()));
    }
}

// ── The warning, as rows ────────────────────────────────────────────

/// How many sites, and how many stale nodes, the warning names on stdout
/// before a `…` row says how many more `--dump-answer` writes (the
/// proposal's § 0, approved 2026-11-03).
pub const WARNING_ROWS: usize = 4;

/// A row of a block: under `lattice`, `strata` or `universe`, at the hole
/// rows' indent.
fn sub_row(r: Row, value: impl AsRef<str>) -> String {
    debug_assert_eq!(r.layout(), Layout::Block, "{r:?} is no block's row");
    line(r, value)
}

/// What the universe leaves out, as one sentence — M1e2 S1e2.2.8 (W6), the
/// user's ruling of 2026-11-03 on Q-M1e2.3, (a) *scoped, and the read-out
/// says so*: the relations [`ein_infer::scope`] names, why the rung that
/// built H proposes none of their facts, and what R holds of them. `None`
/// when it leaves nothing out. The read-out prints it as the `outside` row
/// under `universe`, `ein test` beside each claim it prints, and `ein oracle
/// --dump`'s `region.md` under its candidate count.
pub fn outside_text(outside: &[Outside]) -> Option<String> {
    let first = outside.first()?;
    let one = outside.len() == 1;
    let it = if one { "it" } else { "them" };
    let why = match first.why {
        Why::NotOwed => format!("no obligation owes {it}"),
        Why::NoHrule => format!("no hrule proposes {it}"),
        Why::Arity if one => format!(
            "it has arity {}, and the blind enumerator proposes arity 1 and 2 only",
            first.arity
        ),
        Why::Arity => {
            "they have arity 3 or more, and the blind enumerator proposes arity 1 and 2 only"
                .to_string()
        }
    };
    let names: Vec<&str> = outside.iter().map(|o| o.relation.as_str()).collect();
    Some(format!(
        "{} — no candidate: {why}. R holds of {it} only what the program states and saturation \
         derives",
        names.join(", ")
    ))
}

/// The `outside` row under `universe`, when the universe leaves a relation
/// out: every question prints it, since every question is over the same H.
fn outside_row(out: &mut Out, outside: &[Outside]) {
    if let Some(t) = outside_text(outside) {
        out.line(sub_row(Row::Outside, t));
    }
}

/// What the untyped rung bound as an object, as one sentence — M1e2
/// S1e2.2.10 (C1), the user's ruling of 2026-11-03 on Q-M1e.12, (a) *the
/// fixture, with the warning*: the names on the right of an `is-a` that a
/// candidate of the blind enumerator's H names, how many candidates and of
/// which relations, and why. `None` when it bound none. The read-out prints it
/// as the `types` row under `universe`, `ein test` beside each claim it
/// prints, and `ein oracle --dump`'s `region.md` under its candidate count.
pub fn types_text(t: &TypesAsObjects) -> Option<String> {
    if t.is_empty() {
        return None;
    }
    let one = t.objects.len() == 1;
    let (it, one_of) = if one { ("it", "it") } else { ("them", "one") };
    let name = if t.candidates == 1 { "names" } else { "name" };
    Some(format!(
        "{} — on the right of an is-a, and {} of {} {name} {one_of} as an object: the blind \
         enumerator is untyped, and no signature names {it}",
        t.objects.join(", "),
        plural(t.candidates as u64, "candidate", "candidates"),
        t.relations.join(", "),
    ))
}

/// The `types` row under `universe`, when the untyped rung bound a type as
/// an object: every question prints it, since every question is over the
/// same H.
fn types_row(out: &mut Out, t: &TypesAsObjects) {
    if let Some(text) = types_text(t) {
        out.line(sub_row(Row::Types, text));
    }
}

/// The warning's static half, as rows under its first: `assumes`, a `site`
/// per site (at most [`WARNING_ROWS`] unless `uncut`), each `cycle`, each
/// `order` — what `ein solve --monotone` prints, and every flagged read-out's
/// `lattice` block.
pub fn static_rows(m: &Monotonicity, uncut: bool, hint: &str) -> Vec<String> {
    let mut out = vec![sub_row(Row::Assumes, ASSUMES)];
    let stakes = &m.detector.stakes;
    for (i, s) in stakes.iter().enumerate() {
        if i == WARNING_ROWS && !uncut {
            out.push(sub_row(
                Row::MoreSites,
                format!(
                    "{} more sites: {hint} writes every one",
                    group((stakes.len() - WARNING_ROWS) as u64)
                ),
            ));
            break;
        }
        out.push(sub_row(Row::Site, s.text()));
    }
    for c in &m.detector.cycles {
        out.push(sub_row(Row::Cycle, c.text()));
    }
    for o in ein_infer::detector::order_texts(&m.detector.order) {
        out.push(sub_row(Row::Order, o));
    }
    out
}

/// The dynamic rows: `status`, `state` and its `stale` rows, `paths`.
pub fn dynamic_rows(w: &Warning, uncut: bool, hint: &str) -> Vec<String> {
    let mut out = Vec::new();
    out.push(sub_row(Row::Status, status_text(&w.status)));
    let st = &w.state;
    out.push(sub_row(Row::State, state_text(st)));
    if !st.stale.is_empty() {
        let k = st.stale.len() as u64;
        for (i, s) in st.stale.iter().enumerate() {
            if i == WARNING_ROWS && !uncut {
                out.push(sub_row(
                    Row::MoreStale,
                    format!(
                        "{} more: {hint} writes them",
                        group(k - WARNING_ROWS as u64)
                    ),
                ));
                break;
            }
            out.push(sub_row(
                Row::Stale,
                format!(
                    "{}, {}: {}",
                    s.node,
                    class_words(s.class),
                    s.facts.join(" · ")
                ),
            ));
        }
    }
    if let Some(p) = &w.paths {
        out.push(sub_row(Row::Paths, paths_text(p)));
    }
    out
}

/// The `state` row: how many of the nodes the run computed hold a stale fact.
pub fn state_text(st: &State) -> String {
    let where_ = if st.cube {
        format!("the cube's {} nodes", group(st.nodes))
    } else {
        format!("the {} the run computed", plural(st.nodes, "node", "nodes"))
    };
    if st.stale.is_empty() {
        return format!("no stale fact in {where_}");
    }
    let k = st.stale.len() as u64;
    format!(
        "stale facts — {} of {where_} {} one: a fact whose absent guard the node's own fixpoint \
         falsifies",
        group(k),
        if k == 1 { "holds" } else { "hold" }
    )
}

/// `ein test`'s one line beside a claim on a flagged program (the proposal's
/// § 4.3): the sites, what is assumed, the status, the state and, where one
/// differs, the parent paths.
pub fn warning_line(m: &Monotonicity, w: Option<&Warning>, root_dead: bool) -> String {
    let mut s = format!(
        "not monotone: {} · every node a fork of root{}; what a node is here is open (F21)",
        plural(m.sites.len() as u64, "site", "sites"),
        if root_dead {
            ", so a dead root kills every node (L0)"
        } else {
            ""
        }
    );
    let Some(w) = w else { return s };
    s.push_str(&match &w.status {
        Status::Exercised { by, .. } => {
            format!(
                " · status non-monotone (exercised, {})",
                ein_infer::detector::by_words(by)
            )
        }
        Status::Conservative => " · status monotone (conservative)".to_string(),
        Status::PastLimit { n, .. } | Status::NotRead { n } | Status::RootDead { n, .. } => {
            format!(" · status not known (2^{n})")
        }
    });
    let k = w.state.stale.len() as u64;
    s.push_str(&if k == 0 {
        " · no stale fact".to_string()
    } else {
        format!(
            " · {} {} a stale fact",
            plural(k, "node", "nodes"),
            if k == 1 { "holds" } else { "hold" }
        )
    });
    if let Some(p) = &w.paths
        && !p.differ.is_empty()
    {
        s.push_str(&format!(
            " · {} of {} reach another state",
            group(p.differ.len() as u64),
            plural(p.parents, "parent path", "parent paths")
        ));
    }
    s
}

/// `in R`, `owing` or `dead`: a stale node's class.
fn class_words(c: Class) -> &'static str {
    match c {
        Class::R => "in R",
        Class::Owing => "owing",
        Class::Dead => "dead",
    }
}

/// The cube's counts, `1 hole · 1 dead node with an alive superset · …`.
pub fn violation_counts(holes: u64, dead: u64, discharged: u64) -> String {
    format!(
        "{} · {} · {}",
        plural(holes, "hole", "holes"),
        plural(
            dead,
            "dead node with an alive superset",
            "dead nodes with an alive superset"
        ),
        plural(
            discharged,
            "discharged node with an owing superset",
            "discharged nodes with an owing superset"
        )
    )
}

/// The `status` row: whether a flagged site bit, in the status.
pub fn status_text(s: &Status) -> String {
    match s {
        Status::Exercised {
            by,
            holes,
            dead_not_upward_closed,
            discharged_not_upward_closed,
        } => format!(
            "non-monotone here (exercised, {}) — the cube: {}",
            ein_infer::detector::by_words(by),
            violation_counts(
                *holes,
                *dead_not_upward_closed,
                *discharged_not_upward_closed
            )
        ),
        Status::Conservative => "monotone here (conservative) — the cube: no hole, no dead node \
                                 with an alive superset, no discharged node with an owing superset"
            .to_string(),
        Status::PastLimit { n, limit } => {
            format!("not known — the cube is 2^{n} nodes, past --region-limit {limit}")
        }
        Status::NotRead { n } => format!(
            "not known — this run stopped before it read the cube's 2^{n} nodes: `ein oracle \
             FILE` reads them, and so does a bare `ein solve FILE`"
        ),
        Status::RootDead { n, limit } if *n <= *limit as usize && *n < 64 => format!(
            "not known — root is dead, so every node is (L0), and the run read root alone: \
             `ein oracle FILE` reads the cube's 2^{n} nodes"
        ),
        Status::RootDead { n, limit } => format!(
            "not known — root is dead, so every node is (L0), and the cube is 2^{n} nodes, past \
             --region-limit {limit}"
        ),
    }
}

/// The `paths` row: what the spot check read, and no more. Under `-y`
/// (`bound` `None`, M1e2 S1e2.1c.5) it says it read every parent.
pub fn paths_text(p: &Paths) -> String {
    let text = paths_words(p);
    if p.bound.is_none() {
        format!("{text} (-y: every one-step parent)")
    } else {
        text
    }
}

fn paths_words(p: &Paths) -> String {
    if p.states == 0
        && let Some((met, cost)) = p.unprinted
    {
        return format!(
            "not checked — the read-out prints no state of two commitments or more; its {} \
                 met would cost up to {} saturations",
            plural(met, "member", "members"),
            group(cost)
        );
    }
    if p.parents == 0 {
        return "no state the read-out prints has an alive parent to check".to_string();
    }
    if p.differ.is_empty() {
        return format!(
            "every parent path checked reaches the same state — {} of {}",
            plural(p.parents, "parent", "parents"),
            plural(p.states, "printed state", "printed states")
        );
    }
    let d = p.differ.len() as u64;
    format!(
        "path-dependent — {} of {} checked reach{} another state: {}",
        group(d),
        plural(p.parents, "parent path", "parent paths"),
        if d == 1 { "es" } else { "" },
        p.differ[0]
    )
}

/// `ein solve FILE --monotone`: a question about the program, answered by
/// root's fixpoint and the static check — the proposal's § 2.2, approved
/// 2026-11-03. The cube's half is not computed (option (i)): the `bit` row
/// names what computes it. Every verdict exits 0.
pub fn render_monotone(c: &Checked, o: &Options, limit: u32) -> Text {
    let mut out = Out {
        text: String::new(),
        stderr: Vec::new(),
        listed: 0,
        uncut: o.uncut,
        shown: Vec::new(),
        hint: dump_hint(o.command),
    };
    out.line(format!("{} · {}", o.command, o.argv));
    out.line(RULE);
    out.row(
        Row::Asked,
        "is the program monotone? — the static check, on root's fixpoint",
    );
    let m = &c.monotonicity;
    if m.is_monotone() {
        out.row(
            Row::Monotone,
            "yes — no site: dead and discharged are upward-closed, so R has no hole and its two \
             lines describe it",
        );
    } else {
        out.row(
            Row::Monotone,
            format!("no — {}", plural(m.sites.len() as u64, "site", "sites")),
        );
        for l in static_rows(m, o.uncut, out.hint) {
            out.line(l);
        }
        let n = c.n;
        let bit = if n > limit as usize || n >= 64 {
            format!("not computed — the cube is 2^{n} nodes, past --region-limit {limit}")
        } else if c.root_dead {
            // A bare `ein solve` decides a dead root by L0 and reads root
            // alone, so it does not compute the cube here.
            format!(
                "not computed — the cube is 2^{n} = {} saturations: `ein oracle FILE` computes it",
                group(1u64 << n)
            )
        } else {
            format!(
                "not computed — the cube is 2^{n} = {} saturations: `ein oracle FILE` computes it, \
                 and so does a bare `ein solve FILE`",
                group(1u64 << n)
            )
        };
        out.line(sub_row(Row::Bit, bit));
    }
    out.row(Row::Universe, plural(c.n as u64, "candidate", "candidates"));
    outside_row(&mut out, &c.root.outside);
    types_row(&mut out, &c.root.types_as_objects);
    out.row(
        Row::Route,
        "none — a question about the program: root's fixpoint, then the static check",
    );
    out.row(Row::Work, "1 saturation, root's · 0 facts beyond root");
    Text {
        stdout: out.text,
        stderr: out.stderr,
        code: 0,
        shown: out.shown,
        listed: out.listed,
    }
}

fn licences(l: &[Licence]) -> String {
    l.iter().map(|l| l.label()).collect::<Vec<_>>().join(" · ")
}

/// `--count`'s one row for the claims the program states, when it states
/// any: none is checked, since `--count` reads no state, and none is skipped
/// silently (M1e2 T1e2.1c.4.6). stderr gets one `note:` per claim, as a claim
/// describe could not check does (`read_out.md` § 9).
fn count_claims(out: &mut Out, r: &ReadOut, o: &Options) {
    let n_q = r.queries.len();
    let mut notes: Vec<String> = Vec::new();
    for (qi, q) in r.queries.iter().enumerate() {
        let Some(claim) = &q.claim else { continue };
        let label = Row::Query.numbered(qi + 1, n_q);
        let why = claim.lines.first().cloned().unwrap_or_default();
        notes.push(format!(
            "note: {}: {label} :expect NOT CHECKED — {why}",
            o.file
        ));
    }
    let on_queries = notes.len() as u64;
    if let Some(claim) = &r.claim {
        let why = claim.lines.first().cloned().unwrap_or_default();
        notes.push(format!(
            "note: {}: (solve :expect) NOT CHECKED — {why}",
            o.file
        ));
    }
    if notes.is_empty() {
        return;
    }
    let mut parts: Vec<String> = Vec::new();
    match on_queries {
        0 => {}
        1 => parts.push("a query's :expect".to_string()),
        k => parts.push(format!("{}' :expect", plural(k, "query", "queries"))),
    }
    if r.claim.is_some() {
        parts.push("the (solve :expect)".to_string());
    }
    let k = notes.len() as u64;
    out.blank();
    out.row(
        Row::Claims,
        format!(
            "not checked — {}, {}: --count reads no state, so it checks no claim; a bare `ein \
             solve FILE` checks {}",
            plural(k, "claim", "claims"),
            parts.join(" and "),
            if k == 1 { "it" } else { "them" }
        ),
    );
    out.stderr.extend(notes);
}

/// The `minimal line` row's value, read off a description: describe's, and
/// since M1e2 S1e2.2.26 (W27) a line mode's past `--region-limit` on a
/// flagged program, read off the same description, so the two say the same.
fn minimal_line(d: &Described) -> String {
    let root = &d.root;
    let sp = d.states_partial;
    let n_min = d.states.iter().filter(|s| s.minimal).count() as u64;
    // Where ∅ is in R the line is {∅}, and the `∅` row above says why: root
    // is alive and owes nothing. No clause beside it: *no obligation is
    // stated* read whether a node the run met owes, not what the program
    // states (M1f S1f.5.6, S-17 and F1, the user's ruling of 2026-11-07).
    let min_note = if root.in_r() {
        " — ∅".to_string()
    } else if d.exhausted {
        let smallest = d
            .minimal_commitments
            .iter()
            .map(|s| s.len())
            .min()
            .unwrap_or(0);
        format!(
            ", from {} — the least guess is {}",
            plural(
                d.minimal_commitments.len() as u64,
                "commitment",
                "commitments"
            ),
            plural(smallest as u64, "candidate", "candidates")
        )
    } else {
        String::new()
    };
    if d.exhausted || root.in_r() {
        format!("{}{min_note}", plural(n_min, "state", "states"))
    } else if let Some(p) = sp {
        match p.up_to {
            Some(k) => format!(
                "≥ {} — complete up to states of {}",
                plural(n_min, "state", "states"),
                plural(k as u64, "candidate", "candidates")
            ),
            None => format!("≥ {}", plural(n_min, "state", "states")),
        }
    } else if let Some(l) = d.complete_up_to {
        format!(
            "≥ {} — complete up to commitments of {}",
            plural(n_min, "state", "states"),
            plural(l as u64, "candidate", "candidates")
        )
    } else {
        format!(
            "≥ {} — every member met in layer order with nothing in R below it",
            plural(n_min, "state", "states")
        )
    }
}

/// The `maximal line` row's value, read off a description: describe's, and
/// a line mode's past the limit ([`minimal_line`]).
fn maximal_line(d: &Described) -> String {
    let n = d.n() as u64;
    let sp = d.states_partial;
    let cube = d.route == Route::Cube;
    let n_max = d.states.iter().filter(|s| s.maximal).count() as u64;
    let top = d
        .states
        .iter()
        .any(|s| s.maximal && s.closure.len() as u64 == n && n > 0);
    if d.exhausted {
        let note = if !d.dead_met && top {
            " — ⊤, the trivial line: no restriction bites"
        } else {
            ""
        };
        format!("{}{note}", plural(n_max, "state", "states"))
    } else if cube {
        "not known — without a licence it needs the whole cube".to_string()
    } else if top {
        "1 state — ⊤, every candidate: no node is dead, the trivial line, certified by one \
         saturation"
            .to_string()
    } else if let Some(p) = sp
        && n_max > 0
    {
        format!(
            "≥ {} — all those of up to {}",
            plural(n_max, "state", "states"),
            plural(p.up_to.unwrap_or(0) as u64, "candidate", "candidates")
        )
    } else if n_max > 0 {
        format!(
            "≥ {} — all those a commitment of up to {} reaches",
            plural(n_max, "state", "states"),
            plural(
                d.complete_up_to.unwrap_or(1).saturating_sub(1) as u64,
                "candidate",
                "candidates"
            )
        )
    } else {
        "not reached".to_string()
    }
}

/// The rows of a description, exhausted or partial. Under `--count` (M1e2
/// T1e2.1c.4.6) the rows that qualify R's two counts and no others: the
/// counts, `exhausted`, `∅` or `empty`, `lattice` with its warning,
/// `universe`, `route` and `work`, each the same text describe prints. The
/// lines, components, sub-cubes, merged states, N, E and the minimal holes
/// are describe's alone.
fn describe_rows(out: &mut Out, r: &ReadOut, d: &Described, o: &Options) {
    let a = &r.answer;
    let count = a.mode.ask == Ask::Count;
    let n = d.n() as u64;
    let k_s = d.k_s();
    let sp = d.states_partial;
    let lb = if d.exhausted {
        String::new()
    } else if let Some(p) = sp {
        match p.up_to {
            Some(k) => format!(
                "   (a lower bound: complete up to states of {})",
                plural(k as u64, "candidate", "candidates")
            ),
            None => "   (a lower bound: the run stopped before it extended a state)".to_string(),
        }
    } else if let Some(l) = d.complete_up_to {
        format!(
            "   (a lower bound: complete up to commitments of {})",
            plural(l as u64, "candidate", "candidates")
        )
    } else {
        "   (a lower bound: the run stopped before R was exhausted)".to_string()
    };
    out.row(Row::StatesKs, format!("{}{lb}", group(k_s)));
    // R itself is decided when it is the whole cube; only its states are not.
    let decided = sp.is_some_and(|p| p.cube);
    let lb_c = if decided { "" } else { lb.as_str() };
    // Past the cap the count is a floor for another reason than a partial
    // run's, and says which.
    let past = if d.k_c >= ein_infer::count::CAP {
        "   (2^63 − 1: the run counts no further)"
    } else {
        ""
    };
    out.row(
        Row::CommitmentsKc,
        format!("{}{past}{lb_c}", commitments(d.k_c)),
    );
    match (&a.stopped, d.exhausted) {
        (_, true) => out.row(
            Row::Exhausted,
            "yes — every commitment is classified: this is all of R",
        ),
        (Some(Stopped::Budget { spent, projected }), false) if sp.is_some() => {
            let p = sp.expect("a state-route partial");
            let (size, next) = p.stopped_at;
            let why = format!(
                "state route stopped before the states of {}: their {} extensions project \
                 {} facts on top of the {} spent, past --region-budget {}",
                plural(size as u64, "candidate", "candidates"),
                group(next),
                group(*projected),
                group(*spent),
                o.budget
            );
            if p.cube {
                out.row(
                    Row::Exhausted,
                    format!(
                        "no — R is decided: ∅ and ⊤ are both in it, so it is the whole cube (L1, \
                         L2), and only its states are not all counted. The {why}"
                    ),
                );
            } else {
                out.row(Row::Exhausted, format!("no — the {why}"));
            }
            partial_note(
                out,
                o,
                format!(
                    "the state route stopped before the states of {} at --region-budget {}",
                    plural(size as u64, "candidate", "candidates"),
                    o.budget
                ),
            );
        }
        (Some(Stopped::Budget { spent, projected }), false) => {
            let (layer, next) = d
                .stopped_before
                .unwrap_or((d.complete_up_to.map_or(0, |l| l + 1), 0));
            out.row(
                Row::Exhausted,
                format!(
                    "no — route 1 stopped before layer {layer}: its {} commitments project {} \
                     facts on top of the {} spent, past --region-budget {}",
                    group(next),
                    group(*projected),
                    group(*spent),
                    o.budget
                ),
            );
            partial_note(
                out,
                o,
                format!(
                    "route 1 stopped before layer {layer} at --region-budget {}",
                    o.budget
                ),
            );
        }
        (Some(Stopped::Limit { n, limit }), false) => {
            out.row(
                Row::Exhausted,
                format!(
                    "no — not monotone, and the question needs the whole cube: 2^{n}, past \
                     --region-limit {limit}"
                ),
            );
            partial_note(
                out,
                o,
                format!("the question needs 2^{n} saturations, past --region-limit {limit}"),
            );
        }
        (_, false) => {
            out.row(Row::Exhausted, "no — --max-time");
        }
    }
    let root = &d.root;
    if k_s == 0 && d.exhausted {
        if !d.any_alive {
            out.row(
                Row::Empty,
                "every node is dead — root's own fixpoint derives (false)",
            );
        } else {
            out.row(
                Row::Empty,
                "every alive node owes — no commitment pays every debt",
            );
        }
    } else if root.in_r() {
        out.row(Row::Root, "in R — root is alive and owes nothing");
    } else if !root.alive {
        out.row(Row::Root, "dead — root's own fixpoint derives (false)");
    } else {
        out.row(
            Row::Root,
            format!(
                "not in R — root owes {}",
                plural(root.owes as u64, "instance", "instances")
            ),
        );
        if !count {
            owed_rows(out, &root.owed);
        }
    }

    if !count && (k_s > 0 || !d.exhausted) {
        out.row(Row::MinimalLine, minimal_line(d));
        out.row(Row::MaximalLine, maximal_line(d));
        match &d.shape {
            Some(shape) => {
                let c = &shape.components;
                let note = if c.len() > 1 && c.iter().all(|&s| s == 1) {
                    ", each a single commitment".to_string()
                } else if c.len() > 1 {
                    format!(
                        ", with {} commitments",
                        c.iter()
                            .map(|s| commitments(*s))
                            .collect::<Vec<_>>()
                            .join(" + ")
                    )
                } else {
                    String::new()
                };
                out.row(Row::Components, format!("{}{note}", group(c.len() as u64)));
                match &shape.subcubes {
                    Some(dims) => out.row(Row::SubCubes, subcube_text(dims)),
                    None => out.row(Row::SubCubes, "not known: past the interval search's bound"),
                }
                let merged: Vec<&ein_infer::described::State> =
                    d.states.iter().filter(|s| s.commitments > 1).collect();
                if !d.exhausted {
                    out.row(Row::Merged, "not known: the run did not meet every state");
                } else if merged.is_empty() {
                    out.row(
                        Row::Merged,
                        "none — every state is reached by one commitment, so k_c = k_s",
                    );
                } else {
                    let g: u64 = merged.iter().map(|s| s.generators.len() as u64).sum();
                    out.row(
                        Row::Merged,
                        format!(
                            "{} reached by more than one commitment, with {} minimal",
                            plural(merged.len() as u64, "state", "states"),
                            group(g)
                        ),
                    );
                }
                out.row(
                    Row::N,
                    plural(d.nogoods, "minimal dead set", "minimal dead sets"),
                );
                if let Some(e) = shape.discharging {
                    out.row(
                        Row::E,
                        plural(e, "minimal discharging set", "minimal discharging sets"),
                    );
                }
            }
            None => {
                out.row(Row::Components, "not known: R was not exhausted");
                out.row(Row::SubCubes, "not known");
                out.row(
                    Row::N,
                    match (sp, d.complete_up_to) {
                        (Some(p), _) if p.nogoods_known => {
                            "0 minimal dead sets — ⊤ is alive, so no node is dead".to_string()
                        }
                        (Some(_), _) => format!(
                            "≥ {} — those the run met with every one-down subset alive",
                            plural(d.nogoods, "minimal dead set", "minimal dead sets")
                        ),
                        (None, Some(l)) => format!(
                            "≥ {} — complete up to size {l}",
                            plural(d.nogoods, "minimal dead set", "minimal dead sets")
                        ),
                        (None, None) => format!(
                            "≥ {} — those met in layer order",
                            plural(d.nogoods, "minimal dead set", "minimal dead sets")
                        ),
                    },
                );
            }
        }
    }
    let holes = d.shape.as_ref().and_then(|s| s.holes.as_ref());
    lattice_block(
        out,
        &d.monotonicity,
        a.warning.as_deref(),
        r.spot.as_ref(),
        holes.map(|h| h.0),
    );
    if let Some((holes, minimal)) = holes
        && *holes > 0
        && !count
    {
        let cands: Vec<String> = d.candidates.iter().map(|c| c.text.clone()).collect();
        for h in minimal.iter().take(4) {
            let what = if h.dead {
                match h.core_status.and_then(|c| c.cut_at()) {
                    Some(at) => format!(
                        "dead: {} (core search cut at its budget of {at})",
                        ein_infer::explain::core_text(&h.core, " ")
                    ),
                    None if h.core_status.is_some() => {
                        format!("dead: {}", ein_infer::explain::core_text(&h.core, " "))
                    }
                    None => format!("dead: {}", h.core.join(" ")),
                }
            } else {
                format!("owes {}", h.owes)
            };
            out.row(
                Row::MinimalHole,
                format!("{} — {what}", name(&cands, &h.set)),
            );
        }
    }
    if n == 0 {
        out.row(
            Row::Universe,
            "no candidate: R is {∅} or empty, decided by root alone",
        );
    } else if dead_alone_known(d) {
        out.row(
            Row::Universe,
            format!(
                "{}, {} dead alone",
                plural(n, "candidate", "candidates"),
                group(d.dead_alone)
            ),
        );
    } else {
        out.row(
            Row::Universe,
            format!(
                "{}, dead alone not known: the run stopped before it visited each alone",
                plural(n, "candidate", "candidates")
            ),
        );
    }
    outside_row(out, &a.root.outside);
    types_row(out, &a.root.types_as_objects);
    let route = match (d.route, o.command) {
        (Route::DownSet, _) => format!("1, the alive down-set — {}", licences(&a.licences)),
        (Route::States, _) => format!(
            "the state route, every alive state once — {}",
            licences(&a.licences)
        ),
        (Route::Tree, _) => format!(
            "the obligation tree, over states — {}",
            licences(&a.licences)
        ),
        (Route::Cube, "oracle") => "2, the cube — no licence: asked for by name".to_string(),
        (Route::Cube, _) => {
            "2, the cube — no licence: the static check flags the program".to_string()
        }
    };
    out.row(Row::Route, route);
    out.row(
        Row::Work,
        work_text(d.work.saturations, d.work.facts, d.work.layers),
    );
}

/// The most `owed` rows stdout prints under `∅` before a line says how many
/// more `--dump-answer` writes: the warning's cut ([`WARNING_ROWS`]).
pub const OWED_ROWS: usize = 4;

/// The `owed` rows under `∅`: each obligation instance an alive root owes,
/// through its rule's `:why`, where root is not in R and R is not exhausted
/// and empty — M1f S1f.6.26, E-19, the user's reply 30 (a) of 2026-11-12.
/// Until then the `∅` row counted them and nothing on stdout named one, where
/// an empty R's `owing` rows did: the debt of a root beside a state that pays
/// it, a problem's named gap beside the supposition that closes it, was in
/// `--events`' `owe` lines alone.
///
/// A block under a row, as the warning's is: at most [`OWED_ROWS`] on stdout,
/// then one line that counts the rest and names what writes them, and every
/// one in `--dump-answer`'s `answer.txt`. It does not take the listings'
/// [`LIST_LINES`], which are R's (Q-M1f.220). `--count` prints none.
fn owed_rows(out: &mut Out, owed: &[String]) {
    for (i, w) in owed.iter().enumerate() {
        if i == OWED_ROWS && !out.uncut {
            out.more(format!(
                "… {}: {} writes every one, to DIR/answer.txt",
                plural(
                    (owed.len() - OWED_ROWS) as u64,
                    "more instance",
                    "more instances"
                ),
                out.hint
            ));
            break;
        }
        out.line(sub_row(Row::Owed, w));
    }
}

fn work_text(saturations: u64, facts: u64, layers: Option<usize>) -> String {
    let mut s = format!(
        "{} · {} beyond root",
        plural(saturations, "saturation", "saturations"),
        plural(facts, "fact", "facts")
    );
    if let Some(l) = layers {
        let _ = write!(s, " · {}", plural(l as u64, "layer", "layers"));
    }
    s
}

/// `3, each of dimension 2` or `63, 4 of dimension 17, 59 of dimension 16`.
fn subcube_text(dims: &[usize]) -> String {
    let mut by: BTreeMap<usize, u64> = BTreeMap::new();
    for &d in dims {
        *by.entry(d).or_default() += 1;
    }
    let total = dims.len() as u64;
    let what: Vec<String> = if by.len() == 1 {
        let d = by.keys().next().copied().unwrap_or(0);
        vec![if total == 1 {
            format!("of dimension {d}")
        } else {
            format!("each of dimension {d}")
        }]
    } else {
        by.iter()
            .rev()
            .map(|(d, c)| format!("{} of dimension {d}", group(*c)))
            .collect()
    };
    if total == 0 {
        "0".to_string()
    } else {
        format!("{}, {}", group(total), what.join(", "))
    }
}

/// The rows of a mode: a line, an `-n` cut, `--single`.
fn mode_rows(out: &mut Out, r: &ReadOut, o: &Options, line: &str) {
    let a = &r.answer;
    // The states listed: past the limit a line mode meets members it does
    // not list (M1e2 S1e2.2.26).
    let k = r.listed().count() as u64;
    let label = match a.mode.which {
        Which::All => Row::StatesOfR,
        Which::Minimal => Row::MinimalLine,
        Which::Maximal => Row::MaximalLine,
    };
    if a.mode.ask == Ask::Single {
        match a.verdict {
            Some(Verdict::Solution) => {
                out.row(Row::Verdict, format!("Solution — {line} is one state"))
            }
            Some(Verdict::Ambiguity) => out.row(
                Row::Verdict,
                format!("Ambiguity — {line} holds two states or more; two of them below"),
            ),
            Some(v @ (Verdict::Contradiction | Verdict::Open)) => {
                let why = if v == Verdict::Contradiction {
                    "R is empty: every node is dead"
                } else {
                    "R is empty: every alive node owes"
                };
                out.row(Row::Verdict, format!("{} — {why}", v.word()));
            }
            None => out.row(Row::Verdict, "not decided — the run stopped first"),
        }
    } else if let Some(d) = a.partial.as_deref() {
        // A line mode past `--region-limit` on a flagged program says what
        // describe says of its line, from the same description (W27).
        out.row(
            label,
            match a.mode.which {
                Which::Maximal => maximal_line(d),
                _ => minimal_line(d),
            },
        );
    } else {
        let q = if a.exhausted {
            if k == 0 {
                if a.root_dead {
                    " — none: R is empty, every node is dead".to_string()
                } else {
                    " — none: R is empty, every alive node owes".to_string()
                }
            } else {
                " — all of them".to_string()
            }
        } else if a.stopped.is_some() {
            "   (a lower bound: the run stopped first)".to_string()
        } else {
            format!(", as asked — {line} may hold more")
        };
        out.row(label, format!("{}{q}", plural(k, "state", "states")));
    }
    match &a.stopped {
        None if a.mode.ask == Ask::Single => out.row(
            Row::Decided,
            if matches!(a.verdict, Some(Verdict::Contradiction | Verdict::Open)) {
                format!("yes — {line} is empty")
            } else if a.exhausted {
                format!("yes — {line} holds no other state")
            } else {
                format!("yes — two states settle it; the rest of {line} was not visited")
            },
        ),
        None if a.exhausted => out.row(Row::Exhausted, format!("yes — this is all of {line}")),
        None => out.row(
            Row::Exhausted,
            "no — stopped as asked; each state shown is certified, nothing is said of the rest",
        ),
        Some(Stopped::Budget { spent, projected }) => {
            out.row(
                if a.mode.ask == Ask::Single {
                    Row::Decided
                } else {
                    Row::Exhausted
                },
                format!(
                    "no — stopped at --region-budget {}: {} facts spent, the next step projects {}",
                    o.budget,
                    group(*spent),
                    group(*projected)
                ),
            );
            partial_note(out, o, format!("stopped at --region-budget {}", o.budget));
        }
        Some(Stopped::Limit { n, limit }) => {
            out.row(
                if a.mode.ask == Ask::Single {
                    Row::Decided
                } else {
                    Row::Exhausted
                },
                format!(
                    "no — not monotone, and the question needs the whole cube: 2^{n}, past \
                     --region-limit {limit}"
                ),
            );
            partial_note(
                out,
                o,
                format!("the question needs 2^{n} saturations, past --region-limit {limit}"),
            );
        }
        Some(Stopped::Deadline) => out.row(Row::Exhausted, "no — --max-time"),
    }
    // A dead root leaves no node of R, so no hole (L0).
    let holes = if a
        .licences
        .contains(&ein_infer::explore::Licence::ForkOnlyAdds)
    {
        Some(0)
    } else {
        a.shape.as_ref().and_then(|s| s.holes).map(|h| h as u64)
    };
    lattice_block(
        out,
        &a.monotonicity,
        a.warning.as_deref(),
        r.spot.as_ref(),
        holes,
    );
    out.row(
        Row::Universe,
        plural(a.n() as u64, "candidate", "candidates"),
    );
    outside_row(out, &a.root.outside);
    types_row(out, &a.root.types_as_objects);
    out.row(
        Row::Route,
        format!(
            "{} — {}",
            match a.route {
                Route::DownSet => "1, the alive down-set",
                Route::Cube => "2, the cube",
                // A mode's cut of the route: it meets only the states its
                // question needs (M1e2 S1e2.1c.4).
                Route::States => "the state route, each state at most once",
                Route::Tree => "the obligation tree, over states",
            },
            licences(&a.licences)
        ),
    );
    out.row(Row::Work, work_text(a.saturations, a.facts, None));
}

fn on_lines(s: &Listed) -> &'static str {
    match (s.minimal, s.maximal) {
        (true, true) => " · minimal and maximal",
        (true, false) => " · minimal",
        (false, true) => " · maximal",
        _ => "",
    }
}

/// A state's title — its number, its size, its lines, and the commitment
/// that reaches it or how many do — as the trace heads its section.
pub fn state_title(i: usize, k: usize, s: &Listed, cands: &[String]) -> String {
    let c = state_comment(i, k, s, cands);
    let t = c.trim_start_matches(";; ");
    let mut chars = t.chars();
    match chars.next() {
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// A state's comment in R's listing: its number, its size, its lines, and
/// the commitment that reaches it or how many do.
fn state_comment(i: usize, k: usize, s: &Listed, cands: &[String]) -> String {
    let what = if s.closure.is_empty() {
        "∅, root itself".to_string()
    } else {
        plural(s.closure.len() as u64, "candidate", "candidates")
    };
    let from = match s.commitments {
        Some(1) | None if s.closure.is_empty() => String::new(),
        Some(1) | None => format!(" · from {}", name(cands, &s.rep)),
        Some(c) => format!(
            " · {} commitments, {} minimal",
            commitments(c),
            group(s.generators.len() as u64)
        ),
    };
    format!(";; state {} of {k} — {what}{}{from}", i + 1, on_lines(s))
}

fn listing(out: &mut Out, r: &ReadOut, cands: &[String]) {
    match &r.listing {
        Listing::False => {
            out.blank();
            if let Some(d) = r.described() {
                // The empty set, where the core was taken and holds no fact
                // (M1f S1f.5.9, Q-M1f.48).
                if d.root.core.is_empty() && d.root.core_status.is_some() {
                    out.row(
                        Row::UnsatCore,
                        format!(
                            "{}, {}",
                            ein_infer::explain::EMPTY_CORE,
                            ein_infer::explain::EMPTY_CORE_GLOSS
                        ),
                    );
                }
                for c in &d.root.core {
                    out.row(Row::UnsatCore, c);
                }
                if let Some(q) = d.root.core_status.and_then(|c| c.qualifier()) {
                    out.row(Row::CoreSearch, q);
                }
                for c in &d.root.sources {
                    out.row(Row::Source, c);
                }
                if !d.root.refuted_by.is_empty() {
                    out.row(Row::RefutedBy, d.root.refuted_by.join(", "));
                }
            }
            out.blank();
            out.line("  (false)");
        }
        Listing::Owing(frontier) => {
            out.blank();
            for f in frontier {
                out.row(
                    Row::Owing,
                    format!(
                        "{} — owes {}",
                        name(cands, &f.node.set),
                        group(f.node.owes as u64)
                    ),
                );
                for w in &f.node.owed {
                    out.more(format!("  {w}"));
                }
            }
            out.blank();
            let items: Vec<String> = frontier.iter().map(|f| model_text(&f.model)).collect();
            let mut lines = wrapped("region (owing", &items, 2, 78);
            if let Some(last) = lines.last_mut() {
                last.push(')');
            }
            for l in lines {
                out.line(l);
            }
        }
        Listing::Region => {
            out.blank();
            region(out, &r.states, cands);
        }
        Listing::Certified { others } => {
            out.blank();
            let listed: Vec<(usize, &Listed)> = r.listed().collect();
            // A line mode that certifies no member met (`--maximal` past the
            // limit, M1e2 S1e2.2.26) has no line member to introduce; a
            // description always certifies one when it met one.
            if !listed.is_empty() || *others == 0 {
                out.line(
                    "  ;; the line members this run certified. Not a (region …): read back, it \
                     would claim these are all of R.",
                );
            }
            members(out, &listed, cands, None);
            if *others > 0 {
                out.line(format!(
                    "  ;; {} met {} not certified on a line, and not listed: {} writes each \
                     to DIR/models/",
                    plural(*others, "other member", "other members"),
                    if *others == 1 { "is" } else { "are" },
                    out.hint
                ));
            }
        }
        Listing::Counted => {}
        Listing::Members => {
            if !r.states.is_empty() {
                out.blank();
                let tag = match r.answer.mode.which {
                    Which::All => None,
                    Which::Minimal => Some("minimal"),
                    Which::Maximal => Some("maximal"),
                };
                let all: Vec<(usize, &Listed)> = r.states.iter().enumerate().collect();
                members(out, &all, cands, tag);
            }
        }
    }
}

fn model_text(facts: &[String]) -> String {
    if facts.is_empty() {
        "(model)".to_string()
    } else {
        format!("(model {})", facts.join(" "))
    }
}

/// All of R as one `(region …)`, or what fits of it, outside one.
fn region(out: &mut Out, states: &[Listed], cands: &[String]) {
    let k = states.len();
    // One block per state, then one per `(merged …)`, at the region's indent.
    let mut blocks: Vec<Vec<String>> = Vec::new();
    for (i, s) in states.iter().enumerate() {
        let mut b = vec![format!("    {}", state_comment(i, k, s, cands))];
        b.extend(wrapped("model", &s.model, 4, 78));
        blocks.push(b);
    }
    for (i, s) in states.iter().enumerate() {
        let Some(c) = s.commitments.filter(|&c| c > 1) else {
            continue;
        };
        let mut b = vec![format!(
            "    ;; state {} is reached by {} commitments; its {} minimal",
            i + 1,
            commitments(c),
            group(s.generators.len() as u64)
        )];
        let to = wrapped("model", &s.model, 12, 78);
        b.push(format!("    (merged {}", to[0].trim_start()));
        b.extend(to[1..].iter().cloned());
        let mut g: Vec<Vec<String>> = s.generators.iter().map(|g| set_facts(cands, g)).collect();
        g.sort();
        if g.len() == 1 {
            let lines = wrapped("model", &g[0], 12, 78);
            let last = lines.len() - 1;
            for (j, l) in lines.iter().enumerate() {
                b.push(format!("{l}{}", if j == last { ")" } else { "" }));
            }
        } else {
            for (j, m) in g.iter().enumerate() {
                let lines = wrapped("model", m, 16, 78);
                let n_lines = lines.len();
                for (li, l) in lines.into_iter().enumerate() {
                    let l = if j == 0 && li == 0 {
                        format!("{}(or {}", " ".repeat(12), l.trim_start())
                    } else {
                        l
                    };
                    let close = if j + 1 == g.len() && li + 1 == n_lines {
                        "))"
                    } else {
                        ""
                    };
                    b.push(format!("{l}{close}"));
                }
            }
        }
        blocks.push(b);
    }
    let total = 2 + blocks.iter().map(Vec::len).sum::<usize>() - 1;
    if total <= out.left() {
        out.listed += total;
        out.shown = (0..k).collect();
        out.line("  (region");
        let mut lines: Vec<String> = blocks.into_iter().flatten().collect();
        if let Some(last) = lines.last_mut() {
            last.push(')');
        }
        for l in lines {
            out.line(l);
        }
        return;
    }
    // Past stdout's limit: whole blocks while they fit, outside a `(region
    // …)`, since a cut wrapped in one would claim to be all of R.
    let blocks: Vec<Vec<String>> = blocks
        .into_iter()
        .map(|b| b.into_iter().map(|l| l[2..].to_string()).collect())
        .collect();
    out.line(format!(
        "  ;; R as one (region …) is {} lines, past stdout's {LIST_LINES}: what fits is below, \
         not wrapped in a (region …)",
        group(total as u64)
    ));
    let shown = out.blocks(&blocks);
    out.shown = (0..shown.min(k)).collect();
    let n_merged = blocks.len() - k;
    let mut rest: Vec<String> = Vec::new();
    if shown < k {
        rest.push(plural((k - shown) as u64, "more state", "more states"));
    }
    let merged_shown = shown.saturating_sub(k);
    if merged_shown < n_merged {
        rest.push(format!(
            "{} (merged …)",
            group((n_merged - merged_shown) as u64)
        ));
    }
    out.line(format!(
        "  ;; not shown: {} — {} writes the whole (region …) to DIR/answer.txt",
        rest.join(" and "),
        out.hint
    ));
}

/// States listed as members, not as a region: a line, `-n`'s cut, `--single`,
/// a partial run's certified members.
fn members(out: &mut Out, states: &[(usize, &Listed)], cands: &[String], line: Option<&str>) {
    let k = states.len();
    let mut blocks: Vec<Vec<String>> = Vec::new();
    for (i, (_, s)) in states.iter().enumerate() {
        let what = if s.closure.is_empty() {
            "∅, root itself".to_string()
        } else {
            plural(s.closure.len() as u64, "candidate", "candidates")
        };
        let from = if s.closure.is_empty() || s.generators.is_empty() {
            String::new()
        } else if s.generators.len() > 1 || s.generators.first().is_some_and(|g| *g != s.rep) {
            let mut g: Vec<String> = s
                .generators
                .iter()
                .take(MINIMAL_SHOWN)
                .map(|g| name(cands, g))
                .collect();
            if s.generators.len() > MINIMAL_SHOWN {
                g.push(format!(
                    "… {} more",
                    group((s.generators.len() - MINIMAL_SHOWN) as u64)
                ));
            }
            format!(" · minimal {}", g.join(" | "))
        } else {
            format!(" · from {}", name(cands, &s.rep))
        };
        let on = match line {
            Some(l) => format!(" · {l}"),
            None => on_lines(s).to_string(),
        };
        let mut b = vec![format!("  ;; state {} of {k} — {what}{on}{from}", i + 1)];
        b.extend(wrapped("model", &s.model, 2, 78));
        blocks.push(b);
    }
    let shown = out.blocks(&blocks);
    out.shown = states[..shown].iter().map(|(i, _)| *i).collect();
    if shown < k {
        out.line(format!(
            "  ;; … {}: {} writes the listing uncut, to DIR/answer.txt",
            plural((k - shown) as u64, "more state", "more states"),
            out.hint
        ));
    }
}

/// Each query's block, then its `:expect` checked under it; returns the exit
/// code a refuted claim takes.
fn queries(out: &mut Out, r: &ReadOut, o: &Options) -> i32 {
    let mut code = 0;
    let n_q = r.queries.len();
    for (qi, q) in r.queries.iter().enumerate() {
        let label = Row::Query.numbered(qi + 1, n_q);
        query_block(out, r, q, qi + 1, n_q);
        let Some(claim) = &q.claim else {
            continue;
        };
        let why = claim.lines.first().cloned().unwrap_or_default();
        match claim.outcome {
            Outcome::Held => out.row(Row::Expect, "holds"),
            Outcome::Failed => {
                out.row(Row::Expect, format!("FAILED — {why}"));
                for l in claim.lines.iter().skip(1) {
                    out.more(l);
                }
                out.stderr
                    .push(format!("{}: {label} :expect FAILED — {why}", o.file));
                code = 1;
            }
            Outcome::NotChecked => {
                out.row(Row::Expect, format!("NOT CHECKED — {why}"));
                out.stderr.push(format!(
                    "note: {}: {label} :expect NOT CHECKED — {why}",
                    o.file
                ));
            }
        }
    }
    code
}

/// Why a query over an empty R has no state to answer from: which kind of
/// empty R is, as root says it — M1f S1f.6.26, the row half of E-18. A dead
/// root leaves every node dead (L0), and an alive root of an empty R owes,
/// since one that owed nothing would be in R. Until then the row read *R is
/// empty: no state to answer from* on both, so a query could not tell a
/// contradiction from a debt. The claim under it is not changed: `as :expect
/// (false)` on both, as a query's `(false)` holds on both (Q-M1f.219).
fn empty_words(a: &ein_infer::explore::Answer) -> String {
    if a.root_dead {
        "R is empty, root is dead: no state to answer from".to_string()
    } else if a.root_owes > 0 {
        format!(
            "R is empty, root is alive and owes {}: no state to answer from",
            plural(a.root_owes as u64, "instance", "instances")
        )
    } else {
        // No run reaches this: an alive root that owes nothing is in R.
        "R is empty: no state to answer from".to_string()
    }
}

/// One query's block: its goal, the set it is over, its classes, and the
/// `:expect` that states them.
fn query_block(out: &mut Out, r: &ReadOut, q: &Query, qi: usize, n_q: usize) {
    let a = &r.answer;
    let k = r.states.len() as u64;
    let over = match (r.described(), q.over) {
        // The states listed: a line mode past the limit meets members it
        // does not list (M1e2 S1e2.2.26).
        _ if !q.as_asked => format!(
            "the {} listed",
            plural(r.listed().count() as u64, "state", "states")
        ),
        (Some(_), Over::All) if a.route == Route::Cube && !a.exhausted => {
            format!("the {} met", plural(k, "member", "members"))
        }
        (Some(_), over) => over.words().to_string(),
        (None, over) => over.words().to_string(),
    };
    {
        out.blank();
        out.line(numbered_line(Row::Query, qi, n_q, &q.goal));
        if q.classes.is_empty() && q.exact {
            let empty = if r.states.is_empty() {
                empty_words(a)
            } else {
                "it holds no state".to_string()
            };
            out.row(Row::Over, format!("{over} — {empty}"));
            out.row(Row::AsExpect, "(false)");
            return;
        }
        let n_states = q.classes.iter().map(|c| c.states.len()).sum::<usize>() as u64;
        let tail = if q.exact { "" } else { "  (a lower bound)" };
        out.row(
            Row::Over,
            format!(
                "{over} — {} → {}{tail}",
                plural(n_states, "state", "states"),
                plural(
                    q.classes.len() as u64,
                    "projection class",
                    "projection classes"
                )
            ),
        );
        let mut blocks: Vec<Vec<String>> = Vec::new();
        for (ci, c) in q.classes.iter().enumerate() {
            let mut b: Vec<String> = Vec::new();
            let mut which: Vec<String> = c
                .states
                .iter()
                .take(12)
                .map(|i| (i + 1).to_string())
                .collect();
            if c.states.len() > 12 {
                which.push("…".to_string());
            }
            // State numbers point into the listing, so only a complete
            // listing gets them: one that is all of the part the run asked,
            // whether or not that part is the set the query is about.
            let complete = q.exact || r.described().map_or(r.answer.exhausted, |d| d.exhausted);
            let where_ = if complete {
                format!(
                    "{} ({})",
                    plural(c.states.len() as u64, "state", "states"),
                    which.join(", ")
                )
            } else {
                plural(c.states.len() as u64, "state", "states")
            };
            if c.rows.is_empty() {
                b.push(numbered_line(
                    Row::Class,
                    ci + 1,
                    q.classes.len(),
                    format!("{where_} — the goal has no row: undetermined here"),
                ));
                blocks.push(b);
                continue;
            }
            b.push(numbered_line(Row::Class, ci + 1, q.classes.len(), where_));
            for (ri, row) in c.rows.iter().enumerate() {
                let bound: Vec<String> = row.iter().map(|(k, v)| format!("?{k} = {v}")).collect();
                if bound.is_empty() {
                    // A goal with a variable binds it in every row it has, an
                    // `eq` or `neq` guard included since M1f S1f.5.7, unless
                    // the variable is an `(absent …)`'s own; until then a
                    // guard's free variable bound nothing, and `(neq w ?y)`'s
                    // one row said it had no variable. A name holds no `?`
                    // (`ein.l`'s name rule), so the goal's text tells.
                    b.push(more_text(if q.goal.contains('?') {
                        "  the goal holds: it binds no variable"
                    } else {
                        "  the goal holds: it has no variable"
                    }));
                } else {
                    b.push(more_text(format!("  {}", bound.join(" · "))));
                }
                if let Some(t) = c.rendered.get(ri) {
                    b.push(more_text(format!("    “{t}”")));
                }
            }
            blocks.push(b);
        }
        let shown = out.blocks(&blocks);
        if shown < q.classes.len() {
            out.more(format!(
                "… {}: {} writes them to DIR/answer.txt",
                plural(
                    (q.classes.len() - shown) as u64,
                    "more class",
                    "more classes"
                ),
                out.hint
            ));
        }
        if !q.as_asked {
            out.row(
                Row::AsExpect,
                format!(
                    "not printed: these classes are over the states listed, and the query asks \
                     about {}",
                    q.over.words()
                ),
            );
            return;
        }
        if !q.exact {
            out.row(
                Row::AsExpect,
                "not printed: these classes are a lower bound, and the claim would say they are all",
            );
            return;
        }
        if shown < q.classes.len() {
            // `ein oracle` writes no DIR, so it names what does.
            let by = if out.hint == DUMP_FLAG {
                String::new()
            } else {
                format!(", which {} writes", out.hint)
            };
            out.row(
                Row::AsExpect,
                format!(
                    "not printed: stdout lists part of the classes; it is in DIR/answer.txt{by}"
                ),
            );
            return;
        }
        // The query's `:expect`, one class per line: `(model …)` for one,
        // `(or …)` for several, `(false)` for none.
        let mut lines: Vec<String> = Vec::new();
        if q.classes.is_empty() {
            lines.push("(false)".to_string());
        } else if q.classes.len() == 1 {
            lines = wrapped("model", &q.classes[0].facts, 0, 58);
        } else {
            for (ci, c) in q.classes.iter().enumerate() {
                let mut m = wrapped("model", &c.facts, 4, 58);
                if ci == 0 {
                    m[0] = format!("(or {}", m[0].trim_start());
                }
                if ci + 1 == q.classes.len()
                    && let Some(last) = m.last_mut()
                {
                    last.push(')');
                }
                lines.extend(m);
            }
        }
        // A one-line claim is no longer than the note that would replace it.
        if lines.len() > out.left().max(1) {
            out.row(
                Row::AsExpect,
                format!(
                    "{}, past what is left of stdout's {LIST_LINES}: {} writes it to \
                     DIR/answer.txt",
                    plural(lines.len() as u64, "line", "lines"),
                    out.hint
                ),
            );
            return;
        }
        out.listed += lines.len();
        for (li, l) in lines.iter().enumerate() {
            if li == 0 {
                out.row(Row::AsExpect, l);
            } else {
                out.more(l);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A text's line breaks are Unicode's mandatory ones, `\r\n` one break;
    /// each further line starts at the column the text's first character is
    /// printed at (M1f S1f.5.16's T1f.5.16.5).
    #[test]
    fn a_texts_line_breaks_keep_the_rows_column() {
        assert_eq!(
            split_breaks("a\r\nb\rc\nd\u{2028}e"),
            ["a", "b", "c", "d", "e"]
        );
        assert_eq!(split_breaks("one line"), ["one line"]);
        assert_eq!(hang("x", 20), "x");
        assert_eq!(
            more_text("    “a\nb”"),
            format!("{}    “a\n{}b”", " ".repeat(20), " ".repeat(25))
        );
        assert_eq!(
            sub_row(Row::Site, "a\r\nb"),
            format!("    site            a\n{}b", " ".repeat(20))
        );
        // The listing counts the lines a block prints, not its entries.
        assert_eq!(lines_of(&["a\nb".to_string(), "c".to_string()]), 3);
        assert_eq!(
            fit(2, &[vec!["a\nb".to_string()], vec!["c".to_string()]]),
            1
        );
    }

    /// A label never touches its value, at every width (M1f S1f.5.6, W42):
    /// one shorter than its column has its value at column 20, and one as
    /// long or longer has one space after it, its value's further lines at
    /// that value's column. Until S1f.5.6 `class 100 of 277` printed
    /// `class 100 of 27728 states` for 28 states.
    #[test]
    fn a_label_never_touches_its_value() {
        for (indent, width) in [(2, 18), (4, 16)] {
            for n in 0..=width + 4 {
                let label = "x".repeat(n);
                let line = padded(indent, width, &label, "7 states");
                let rest = &line[indent + n..];
                assert!(
                    rest.starts_with(' ') && rest.trim_start() == "7 states",
                    "{line:?}"
                );
                let col = line.len() - "7 states".len();
                assert_eq!(col, indent + width.max(n + 1), "{line:?}");
            }
        }
        assert_eq!(
            numbered_line(Row::Class, 100, 277, "28 states"),
            "    class 100 of 277 28 states"
        );
        assert_eq!(
            numbered_line(Row::Class, 99, 277, "28 states"),
            "    class 99 of 277 28 states"
        );
        assert_eq!(
            padded(4, 16, "class 100 of 277", "a\nb"),
            format!("    class 100 of 277 a\n{}b", " ".repeat(21))
        );
    }
}
