//! The read-out as text: what `ein solve` and `ein oracle` print — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md),
//! the layout of
//! [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md),
//! from [`ein_infer::read_out::ReadOut`].
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
use ein_ir::expect::Over;

/// The rule under the header.
pub const RULE: &str = "──────────────────────────────────────────────────────────────";

/// The most lines stdout's listings print together.
pub const LIST_LINES: usize = 100;

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

/// How many of `blocks` fit in `left` lines: all, or whole blocks while they
/// fit and at least one. The read-out's listings are cut by it, and since
/// M1e2 S1e2.3.3 the forms printed after the read-out too.
pub fn fit(left: usize, blocks: &[Vec<String>]) -> usize {
    let mut lines = 0usize;
    for (i, b) in blocks.iter().enumerate() {
        lines = lines.saturating_add(b.len());
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

    fn row(&mut self, label: &str, value: impl AsRef<str>) {
        let l = row_text(label, value);
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
        self.listed += blocks[..shown].iter().map(Vec::len).sum::<usize>();
        shown
    }
}

fn row_text(label: &str, value: impl AsRef<str>) -> String {
    format!("  {label:<18}{}", value.as_ref())
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
    out.row("asked", asked);

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
        out.row("(solve :expect)", label);
        for l in &claim.lines {
            out.row("", l);
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
            "lattice",
            "monotone (the static check) — R is convex: no hole",
        );
        // `-y` (M1e2 S1e2.1c.5): the spot check, unbounded, held to the
        // theorem — a path that differed would have been the run's error.
        if let Some(p) = spot {
            out.line(sub_row("paths", paths_text(p)));
        }
        return;
    }
    let holes = holes.or_else(|| warning.and_then(|w| w.status.holes()));
    let h = match holes {
        Some(0) => "no hole in this cube".to_string(),
        Some(h) => plural(h, "hole", "holes"),
        None => "holes not known: the cube was not enumerated".to_string(),
    };
    out.row(
        "lattice",
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
}

// ── The warning, as rows ────────────────────────────────────────────

/// How many sites, and how many stale nodes, the warning names on stdout
/// before a `…` row says how many more `--dump-answer` writes (the
/// proposal's § 0, approved 2026-11-03).
pub const WARNING_ROWS: usize = 4;

/// A row under the `lattice` row, at the hole rows' indent.
fn sub_row(label: &str, value: impl AsRef<str>) -> String {
    format!("    {label:<16}{}", value.as_ref())
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
        out.line(sub_row("outside", t));
    }
}

/// The warning's static half, as rows under its first: `assumes`, a `site`
/// per site (at most [`WARNING_ROWS`] unless `uncut`), each `cycle`, each
/// `order` — what `ein solve --monotone` prints, and every flagged read-out's
/// `lattice` block.
pub fn static_rows(m: &Monotonicity, uncut: bool, hint: &str) -> Vec<String> {
    let mut out = vec![sub_row("assumes", ASSUMES)];
    let stakes = &m.detector.stakes;
    for (i, s) in stakes.iter().enumerate() {
        if i == WARNING_ROWS && !uncut {
            out.push(sub_row(
                "…",
                format!(
                    "{} more sites: {hint} writes every one",
                    group((stakes.len() - WARNING_ROWS) as u64)
                ),
            ));
            break;
        }
        out.push(sub_row("site", s.text()));
    }
    for c in &m.detector.cycles {
        out.push(sub_row("cycle", c.text()));
    }
    for o in ein_infer::detector::order_texts(&m.detector.order) {
        out.push(sub_row("order", o));
    }
    out
}

/// The dynamic rows: `status`, `state` and its `stale` rows, `paths`.
pub fn dynamic_rows(w: &Warning, uncut: bool, hint: &str) -> Vec<String> {
    let mut out = Vec::new();
    out.push(sub_row("status", status_text(&w.status)));
    let st = &w.state;
    out.push(sub_row("state", state_text(st)));
    if !st.stale.is_empty() {
        let k = st.stale.len() as u64;
        for (i, s) in st.stale.iter().enumerate() {
            if i == WARNING_ROWS && !uncut {
                out.push(sub_row(
                    "…",
                    format!(
                        "{} more: {hint} writes them",
                        group(k - WARNING_ROWS as u64)
                    ),
                ));
                break;
            }
            out.push(sub_row(
                "stale",
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
        out.push(sub_row("paths", paths_text(p)));
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
            format!(" · status non-monotone (exercised, by {})", by.join(", "))
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
            "non-monotone here (exercised, by {}) — the cube: {}",
            by.join(", "),
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
        "asked",
        "is the program monotone? — the static check, on root's fixpoint",
    );
    let m = &c.monotonicity;
    if m.is_monotone() {
        out.row(
            "monotone",
            "yes — no site: dead and discharged are upward-closed, so R has no hole and its two \
             lines describe it",
        );
    } else {
        out.row(
            "monotone",
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
        out.line(sub_row("bit", bit));
    }
    out.row("universe", plural(c.n as u64, "candidate", "candidates"));
    outside_row(&mut out, &c.root.outside);
    out.row(
        "route",
        "none — a question about the program: root's fixpoint, then the static check",
    );
    out.row("work", "1 saturation, root's · 0 facts beyond root");
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
        let label = if n_q > 1 {
            format!("query {} of {n_q}", qi + 1)
        } else {
            "query".to_string()
        };
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
        "claims",
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
    let cube = d.route == Route::Cube;
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
    out.row("states (k_s)", format!("{}{lb}", group(k_s)));
    // R itself is decided when it is the whole cube; only its states are not.
    let decided = sp.is_some_and(|p| p.cube);
    let lb_c = if decided { "" } else { lb.as_str() };
    out.row("commitments (k_c)", format!("{}{lb_c}", group(d.k_c)));
    match (&a.stopped, d.exhausted) {
        (_, true) => out.row(
            "exhausted",
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
                    "exhausted",
                    format!(
                        "no — R is decided: ∅ and ⊤ are both in it, so it is the whole cube (L1, \
                         L2), and only its states are not all counted. The {why}"
                    ),
                );
            } else {
                out.row("exhausted", format!("no — the {why}"));
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
                "exhausted",
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
                "exhausted",
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
            out.row("exhausted", "no — --max-time");
        }
    }
    let root = &d.root;
    if k_s == 0 && d.exhausted {
        if !d.any_alive {
            out.row(
                "empty",
                "every node is dead — root's own fixpoint derives (false)",
            );
        } else {
            out.row(
                "empty",
                "every alive node owes — no commitment pays every debt",
            );
        }
    } else if root.in_r() {
        out.row("∅", "in R — root is alive and owes nothing");
    } else if !root.alive {
        out.row("∅", "dead — root's own fixpoint derives (false)");
    } else {
        out.row(
            "∅",
            format!(
                "not in R — root owes {}",
                plural(root.owes as u64, "instance", "instances")
            ),
        );
    }

    if !count && (k_s > 0 || !d.exhausted) {
        let n_min = d.states.iter().filter(|s| s.minimal).count() as u64;
        let n_max = d.states.iter().filter(|s| s.maximal).count() as u64;
        let min_note = if root.in_r() {
            if d.owing_met {
                " — ∅".to_string()
            } else {
                " — ∅, the trivial line: no obligation is stated".to_string()
            }
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
            out.row(
                "minimal line",
                format!("{}{min_note}", plural(n_min, "state", "states")),
            );
        } else if let Some(p) = sp {
            out.row(
                "minimal line",
                match p.up_to {
                    Some(k) => format!(
                        "≥ {} — complete up to states of {}",
                        plural(n_min, "state", "states"),
                        plural(k as u64, "candidate", "candidates")
                    ),
                    None => format!("≥ {}", plural(n_min, "state", "states")),
                },
            );
        } else if let Some(l) = d.complete_up_to {
            out.row(
                "minimal line",
                format!(
                    "≥ {} — complete up to commitments of {}",
                    plural(n_min, "state", "states"),
                    plural(l as u64, "candidate", "candidates")
                ),
            );
        } else {
            out.row(
                "minimal line",
                format!(
                    "≥ {} — every member met in layer order with nothing in R below it",
                    plural(n_min, "state", "states")
                ),
            );
        }
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
            out.row(
                "maximal line",
                format!("{}{note}", plural(n_max, "state", "states")),
            );
        } else if cube {
            out.row(
                "maximal line",
                "not known — without a licence it needs the whole cube",
            );
        } else if top {
            out.row(
                "maximal line",
                "1 state — ⊤, every candidate: no node is dead, the trivial line, certified by \
                 one saturation",
            );
        } else if let Some(p) = sp
            && n_max > 0
        {
            out.row(
                "maximal line",
                format!(
                    "≥ {} — all those of up to {}",
                    plural(n_max, "state", "states"),
                    plural(p.up_to.unwrap_or(0) as u64, "candidate", "candidates")
                ),
            );
        } else if n_max > 0 {
            out.row(
                "maximal line",
                format!(
                    "≥ {} — all those a commitment of up to {} reaches",
                    plural(n_max, "state", "states"),
                    plural(
                        d.complete_up_to.unwrap_or(1).saturating_sub(1) as u64,
                        "candidate",
                        "candidates"
                    )
                ),
            );
        } else {
            out.row("maximal line", "not reached");
        }
        match &d.shape {
            Some(shape) => {
                let c = &shape.components;
                let note = if c.len() > 1 && c.iter().all(|&s| s == 1) {
                    ", each a single commitment".to_string()
                } else if c.len() > 1 {
                    format!(
                        ", with {} commitments",
                        c.iter().map(|s| group(*s)).collect::<Vec<_>>().join(" + ")
                    )
                } else {
                    String::new()
                };
                out.row("components", format!("{}{note}", group(c.len() as u64)));
                match &shape.subcubes {
                    Some(dims) => out.row("sub-cubes", subcube_text(dims)),
                    None => out.row("sub-cubes", "not known: past the interval search's bound"),
                }
                let merged: Vec<&ein_infer::described::State> =
                    d.states.iter().filter(|s| s.commitments > 1).collect();
                if !d.exhausted {
                    out.row("merged", "not known: the run did not meet every state");
                } else if merged.is_empty() {
                    out.row(
                        "merged",
                        "none — every state is reached by one commitment, so k_c = k_s",
                    );
                } else {
                    let g: u64 = merged.iter().map(|s| s.generators.len() as u64).sum();
                    out.row(
                        "merged",
                        format!(
                            "{} reached by more than one commitment, with {} minimal",
                            plural(merged.len() as u64, "state", "states"),
                            group(g)
                        ),
                    );
                }
                out.row(
                    "N",
                    plural(d.nogoods, "minimal dead set", "minimal dead sets"),
                );
                if let Some(e) = shape.discharging {
                    out.row(
                        "E",
                        plural(e, "minimal discharging set", "minimal discharging sets"),
                    );
                }
            }
            None => {
                out.row("components", "not known: R was not exhausted");
                out.row("sub-cubes", "not known");
                out.row(
                    "N",
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
                format!("dead: {}", h.core.join(" "))
            } else {
                format!("owes {}", h.owes)
            };
            out.row(
                "  minimal hole",
                format!("{} — {what}", name(&cands, &h.set)),
            );
        }
    }
    if n == 0 {
        out.row(
            "universe",
            "no candidate: R is {∅} or empty, decided by root alone",
        );
    } else if dead_alone_known(d) {
        out.row(
            "universe",
            format!(
                "{}, {} dead alone",
                plural(n, "candidate", "candidates"),
                group(d.dead_alone)
            ),
        );
    } else {
        out.row(
            "universe",
            format!(
                "{}, dead alone not known: the run stopped before it visited each alone",
                plural(n, "candidate", "candidates")
            ),
        );
    }
    outside_row(out, &a.root.outside);
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
    out.row("route", route);
    out.row(
        "work",
        work_text(d.work.saturations, d.work.facts, d.work.layers),
    );
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
    let k = r.states.len() as u64;
    let label = match a.mode.which {
        Which::All => "states of R",
        Which::Minimal => "minimal line",
        Which::Maximal => "maximal line",
    };
    if a.mode.ask == Ask::Single {
        match a.verdict {
            Some(Verdict::Solution) => {
                out.row("verdict", format!("Solution — {line} is one state"))
            }
            Some(Verdict::Ambiguity) => out.row(
                "verdict",
                format!("Ambiguity — {line} holds two states or more; two of them below"),
            ),
            Some(v @ (Verdict::Contradiction | Verdict::Open)) => {
                let why = if v == Verdict::Contradiction {
                    "R is empty: every node is dead"
                } else {
                    "R is empty: every alive node owes"
                };
                out.row("verdict", format!("{} — {why}", v.word()));
            }
            None => out.row("verdict", "not decided — the run stopped first"),
        }
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
            "decided",
            if matches!(a.verdict, Some(Verdict::Contradiction | Verdict::Open)) {
                format!("yes — {line} is empty")
            } else if a.exhausted {
                format!("yes — {line} holds no other state")
            } else {
                format!("yes — two states settle it; the rest of {line} was not visited")
            },
        ),
        None if a.exhausted => out.row("exhausted", format!("yes — this is all of {line}")),
        None => out.row(
            "exhausted",
            "no — stopped as asked; each state shown is certified, nothing is said of the rest",
        ),
        Some(Stopped::Budget { spent, projected }) => {
            out.row(
                if a.mode.ask == Ask::Single {
                    "decided"
                } else {
                    "exhausted"
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
                    "decided"
                } else {
                    "exhausted"
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
        Some(Stopped::Deadline) => out.row("exhausted", "no — --max-time"),
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
    out.row("universe", plural(a.n() as u64, "candidate", "candidates"));
    outside_row(out, &a.root.outside);
    out.row(
        "route",
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
    out.row("work", work_text(a.saturations, a.facts, None));
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
            group(c),
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
                for c in &d.root.core {
                    out.row("unsat core", c);
                }
                for c in &d.root.sources {
                    out.row("source", c);
                }
                if !d.root.refuted_by.is_empty() {
                    out.row("refuted by", d.root.refuted_by.join(", "));
                }
            }
            out.blank();
            out.line("  (false)");
        }
        Listing::Owing(frontier) => {
            out.blank();
            for f in frontier {
                out.row(
                    "owing",
                    format!(
                        "{} — owes {}",
                        name(cands, &f.node.set),
                        group(f.node.owes as u64)
                    ),
                );
                for w in &f.node.owed {
                    out.row("", format!("  {w}"));
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
            out.line(
                "  ;; the line members this run certified. Not a (region …): read back, it \
                 would claim these are all of R.",
            );
            let listed: Vec<(usize, &Listed)> = r.listed().collect();
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
            group(c),
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
        let label = if n_q > 1 {
            format!("query {} of {n_q}", qi + 1)
        } else {
            "query".to_string()
        };
        query_block(out, r, q, &label);
        let Some(claim) = &q.claim else {
            continue;
        };
        let why = claim.lines.first().cloned().unwrap_or_default();
        match claim.outcome {
            Outcome::Held => out.row(":expect", "holds"),
            Outcome::Failed => {
                out.row(":expect", format!("FAILED — {why}"));
                for l in claim.lines.iter().skip(1) {
                    out.row("", l);
                }
                out.stderr
                    .push(format!("{}: {label} :expect FAILED — {why}", o.file));
                code = 1;
            }
            Outcome::NotChecked => {
                out.row(":expect", format!("NOT CHECKED — {why}"));
                out.stderr.push(format!(
                    "note: {}: {label} :expect NOT CHECKED — {why}",
                    o.file
                ));
            }
        }
    }
    code
}

/// One query's block: its goal, the set it is over, its classes, and the
/// `:expect` that states them.
fn query_block(out: &mut Out, r: &ReadOut, q: &Query, label: &str) {
    let a = &r.answer;
    let k = r.states.len() as u64;
    let over = match (r.described(), q.over) {
        _ if !q.as_asked => format!("the {} listed", plural(k, "state", "states")),
        (Some(_), Over::All) if a.route == Route::Cube && !a.exhausted => {
            format!("the {} met", plural(k, "member", "members"))
        }
        (Some(_), over) => over.words().to_string(),
        (None, over) => over.words().to_string(),
    };
    {
        out.blank();
        out.row(label, &q.goal);
        if q.classes.is_empty() && q.exact {
            let empty = if r.states.is_empty() {
                "R is empty: no state to answer from"
            } else {
                "it holds no state"
            };
            out.row("over", format!("{over} — {empty}"));
            out.row("as :expect", "(false)");
            return;
        }
        let n_states = q.classes.iter().map(|c| c.states.len()).sum::<usize>() as u64;
        let tail = if q.exact { "" } else { "  (a lower bound)" };
        out.row(
            "over",
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
            let head = format!("class {} of {}", ci + 1, q.classes.len());
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
                b.push(row_text(
                    &format!("  {head}"),
                    format!("{where_} — the goal has no row: undetermined here"),
                ));
                blocks.push(b);
                continue;
            }
            b.push(row_text(&format!("  {head}"), where_));
            for (ri, row) in c.rows.iter().enumerate() {
                let bound: Vec<String> = row.iter().map(|(k, v)| format!("?{k} = {v}")).collect();
                if bound.is_empty() {
                    b.push(row_text("", "  the goal holds: it has no variable"));
                } else {
                    b.push(row_text("", format!("  {}", bound.join(" · "))));
                }
                if let Some(t) = c.rendered.get(ri) {
                    b.push(row_text("", format!("    “{t}”")));
                }
            }
            blocks.push(b);
        }
        let shown = out.blocks(&blocks);
        if shown < q.classes.len() {
            out.row(
                "",
                format!(
                    "… {}: {} writes them to DIR/answer.txt",
                    plural(
                        (q.classes.len() - shown) as u64,
                        "more class",
                        "more classes"
                    ),
                    out.hint
                ),
            );
        }
        if !q.as_asked {
            out.row(
                "as :expect",
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
                "as :expect",
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
                "as :expect",
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
                "as :expect",
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
            out.row(if li == 0 { "as :expect" } else { "" }, l);
        }
    }
}
