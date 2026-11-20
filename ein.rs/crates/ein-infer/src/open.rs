//! **The pass that writes a state's `open` facts** at its fixpoint
//! (Q-M1f.566, Q-M1f.569, Q-M1f.570).
//!
//! An `open` fact is a fact of the kernel's relation `open`, held in the part
//! of a state no fork inherits (`ein_core::Kb::write_open`): a fork drops
//! every one, and this pass writes the child's at the child's own fixpoint.
//! So no rule deletes a fact, and what a state holds under `open` is a
//! function of the state. Two kinds:
//!
//! - **`(open R)`, what the state owes**: one fact for each relation an
//!   instance is owed of, read off the obligations' tally
//!   ([`crate::obligations::tally`]), which counts each instance as before;
//!   a state that owes nothing holds none, a state of $R$ among them.
//! - **`(open F)`, a hypothesis** (Q-M1f.573), by three writers: every match
//!   of a rule that concludes one, an hrule among them, whether or not the
//!   state holds or denies `F`, since a rule concludes what its match gives;
//!   the facts that would pay what the state owes, read off each activated
//!   obligation's guard ([`crate::oblgen::payments`]), those the state
//!   neither holds nor denies; and, where root held no `open` fact and
//!   activated no obligation, the blind enumerator's guesses, the facts still
//!   undecided. Identical facts are one (Q-M1f.570).
//!
//! A dead state is written as any state is (Q-M1f.566): it owes what the
//! tally counts there.
//!
//! **Root's `open` facts are $H$'s source**: $H$ is the hypotheses root holds,
//! less those root holds or denies ([`crate::hypgen`] builds it through the
//! one filter pipeline, and [`write_root`] writes what each writer wrote
//! there). The program's own `(open …)` facts are root's alone.
//!
//! **What a state above root holds that root does not** is the in-flight
//! check of Q-M1f.574: a hypothesis first held above root makes the program
//! not monotone. The pass reports each one it writes ([`Writers::first_above`]),
//! and the run says so at once, once ([`set_in_flight_hook`]); the answer
//! stands, and nothing is claimed of it.
//!
//! **The blind enumerator at a state above root** is root's raw list, the
//! facts it proposed there, held to the state: those it neither holds nor
//! denies, of a relation it does not close. That is the enumerator's own
//! list at the state, since what it guesses about, the program's names and
//! declared relations, is the same at every state (the alive-set invariant,
//! [`crate::invariant`]), and it costs no enumeration per node.
//!
//! **A rule that reads an `open` fact** (Q-M1f.569) sees them: the pass
//! writes, then the saturation resumes over what it wrote, and the two repeat
//! until neither adds anything ([`Writers::reads_open`]). Such a program is
//! one the static check flags ([`crate::monotone`]'s `open-read` site).


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use ein_core::{FactId, Kb, Prov, Symbol, Terms, Value};
use rustc_hash::FxHashSet;

use crate::compile::CompileError;
use crate::hrule::Hrules;
use crate::hypgen::{HypGenStats, Written};
use crate::obligations::Owes;
use crate::saturator::{Saturator, Session};

/// What the pass writes at every state, read once, at root.
pub struct Writers {
    /// The plans of the rules that conclude a hypothesis, hrules among them,
    /// compiled at root ([`Hrules::new`]).
    pub(crate) rules: Hrules,
    /// The blind enumerator's raw list at root, where it writes: root held no
    /// `open` fact and activated no obligation. `None` where it does not.
    pub(crate) blind: Option<Box<[FactId]>>,
    /// What root's activated obligations propose for a debt a commitment can
    /// bring ([`crate::oblgen::LaterDebt`]), which every state re-reads as it
    /// re-reads the blind enumerator's list.
    pub(crate) later: Box<[FactId]>,
    /// Some rule's `:match` reads `open`: the pass and the saturation repeat.
    pub(crate) reads_open: bool,
    /// What root's rules refute beside a fact ([`crate::contradiction::Pairs`]),
    /// the filter's.
    pub(crate) pairs: crate::contradiction::Pairs,
    /// The hypotheses root holds under `open`.
    pub(crate) at_root: FxHashSet<FactId>,
    /// The first hypothesis a state above root held that root does not, as
    /// text; set once (Q-M1f.574).
    first_above: OnceLock<String>,
    said: AtomicBool,
}

impl Writers {
    /// The first hypothesis a state above root held that root does not, as
    /// the fact's text: `Some` once a run built such a state (Q-M1f.574).
    pub fn first_above(&self) -> Option<&str> {
        self.first_above.get().map(String::as_str)
    }

    /// The static check flagged the program: its warning is said already,
    /// before the run, and the in-flight check records what it finds and
    /// says nothing more.
    pub(crate) fn said_already(&self) {
        self.said.store(true, Ordering::Relaxed);
    }
}

/// The writers, shared by every state of a run: what a root's snapshot
/// carries to the nodes that resume it.
pub type Shared = Arc<Writers>;

/// The hook a front end gives the in-flight check, called once, the first
/// time a run builds a state that holds a hypothesis root does not, with that
/// fact's text: it says so at once (Q-M1f.574). `ein` prints the warning
/// there.
pub type Hook = Box<dyn Fn(&str) + Send + Sync>;

static HOOK: Mutex<Option<Hook>> = Mutex::new(None);

/// Set, or clear with `None`, the in-flight hook ([`Hook`]).
/// **The `open` facts the program wrote**, of those `kb` holds: each with
/// the provenance the loader gives a written fact, a source's, where the
/// pass gives its own a rule's. A root a run has prepared holds the pass's
/// too, and preparing it again reads the program's alone, so the writers
/// and $H$ are what they were.
pub fn written_by_program(kb: &Kb, terms: &Terms) -> Vec<FactId> {
    kb.open_facts()
        .iter()
        .copied()
        .filter(|&o| {
            kb.primary(o)
                .is_none_or(|p| terms.provs.get(p).kind == ein_core::prov::ProvKind::Source)
        })
        .collect()
}

pub fn set_in_flight_hook(hook: Option<Hook>) {
    *HOOK.lock().expect("no hook panicked") = hook;
}

/// **Root's `open` facts**, at root's fixpoint, which `sat` reached: $H$'s
/// writers run, each candidate handed to `f` as
/// [`crate::hypgen::generate_config_free`] hands it, with `stats` its
/// counters; what root owes is read ([`crate::obligations::tally`],
/// narrated into `s.events`); and the pass writes root's `(open …)` facts.
/// Where a rule reads `open`, the saturation resumes over them and the two
/// repeat until neither adds anything. Returns the writers every state above
/// root reads, and what root owes.
pub fn at_root(
    s: &mut Session<'_>,
    sat: &mut Saturator,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> std::ops::ControlFlow<()>,
) -> Result<(Shared, Owes), crate::error::SolveError> {
    let mut written = Written::default();
    crate::hypgen::generate_written(s, stats, &mut written, f)?;
    let blind = if written.blind_ran {
        // The blind enumerator's raw list, every fact it proposes at root
        // before the filters: what a state above root re-reads.
        let mut raw: Vec<FactId> = Vec::new();
        let mut probe = HypGenStats::default();
        let mut silent = crate::events::Events::off();
        let mut q = Session {
            kb: &mut *s.kb,
            terms: &mut *s.terms,
            ast: s.ast,
            events: &mut silent,
            memo: s.memo.clone(),
        };
        crate::hypgen::generate_blind_raw(&mut q, &mut probe, &mut |fact| {
            raw.push(fact);
            std::ops::ControlFlow::Continue(())
        })?;
        Some(raw.into_boxed_slice())
    } else {
        None
    };
    let later: Box<[FactId]> = stats.later.iter().flat_map(|l| l.added.iter().copied()).collect();
    let mut writers = writers_at_root(s, &written, blind, later)?;
    // What the writers give at root by the one reading every state takes,
    // beside what $H$'s pass recorded: root's hypotheses, which the in-flight
    // check reads every state above it against.
    for (_, f) in hypotheses_at(s, &writers)? {
        writers.at_root.insert(f);
    }
    let writers = Arc::new(writers);
    let memo = s.memo.clone();
    let mut owes = crate::obligations::tally(s.kb, s.terms, s.ast, &memo, s.events)?;
    loop {
        let wrote = write(s, &writers, &owes, false)?;
        if !writers.reads_open || wrote.is_empty() {
            return Ok((writers, owes));
        }
        sat.feed(&wrote);
        sat.saturate(s, None, &mut |_| {})?;
        owes = crate::obligations::tally(
            s.kb,
            s.terms,
            s.ast,
            &memo,
            &mut crate::events::Events::off(),
        )?;
    }
}

/// **A state's `open` facts above root**, at the fixpoint `sat` reached:
/// what it owes is read ([`crate::obligations::tally`], narrated into
/// `s.events`), and the pass writes what `writers` write there. Where a rule
/// reads `open`, the saturation resumes over them and the two repeat until
/// neither adds anything, each firing handed to `sink`. Returns what the
/// state owes.
pub(crate) fn at_state(
    s: &mut Session<'_>,
    sat: &mut Saturator,
    writers: &Writers,
    sink: &mut dyn FnMut(&crate::firing::Firing),
) -> Result<Owes, crate::error::SolveError> {
    let memo = s.memo.clone();
    let mut owes = crate::obligations::tally(s.kb, s.terms, s.ast, &memo, s.events)?;
    loop {
        let wrote = write(s, writers, &owes, true)?;
        if !writers.reads_open || wrote.is_empty() {
            return Ok(owes);
        }
        sat.feed(&wrote);
        sat.saturate(s, None, sink)?;
        owes = crate::obligations::tally(
            s.kb,
            s.terms,
            s.ast,
            &memo,
            &mut crate::events::Events::off(),
        )?;
    }
}

/// The writers, read at root: the plans of the rules that conclude a
/// hypothesis, the blind enumerator's raw list where it writes, and the
/// hypotheses root holds under `open`.
fn writers_at_root(
    s: &mut Session<'_>,
    written: &Written,
    blind: Option<Box<[FactId]>>,
    later: Box<[FactId]>,
) -> Result<Writers, CompileError> {
    let rules = Hrules::new(s)?;
    let plans = crate::monotone::plans_of(&s.memo);
    let pairs = crate::contradiction::Pairs::of(plans.iter().map(|p| &**p), s.terms);
    let open = s.terms.kernel.open;
    let program = s.kb.program_arc();
    let reads_open = program
        .rules
        .values()
        .chain(program.obligations.values())
        .chain(program.hypotheses.values())
        .chain(program.hrules.values())
        .any(|r| {
            r.match_
                .as_ref()
                .is_some_and(|m| m.relation_names.contains(&open))
        })
        // A rule whose relation parameter an activator binds to `open`: its
        // plan reads it, as the static check's `open-read` site does.
        || plans.iter().any(|p| {
            crate::monotone::reads_of(p, s.terms)
                .iter()
                .any(|(r, _)| r.name == open)
        });
    let mut at_root: FxHashSet<FactId> = written
        .rules
        .iter()
        .chain(&written.obligations)
        .chain(&written.blind)
        .copied()
        .collect();
    for &o in s.kb.open_facts() {
        if let Some(f) = s.terms.facts.args(o).first().and_then(|v| v.as_fact()) {
            at_root.insert(f);
        }
    }
    // Every `(open F)` a state above root can hold, of a monotone program,
    // is one of root's, and every `(open R)` one of a relation an obligation
    // activated at root owes: numbered here, so a worker's pass finds each
    // numbered.
    let mut kinds: Vec<FactId> = at_root.iter().copied().collect();
    if let Some(raw) = &blind {
        kinds.extend(raw.iter().copied());
    }
    // determinism-ok: numbering order; a fact's id is never observed.
    kinds.sort_unstable();
    for f in kinds {
        let _ = s.terms.intern_fact(open, &[Value::fact(f)]);
    }
    let memo = s.memo.clone();
    for plan in crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)? {
        if let Some(r) = crate::obligations::open_argument(&plan, s.terms) {
            let _ = s.terms.intern_fact(open, &[Value::sym(r)]);
        }
    }
    Ok(Writers {
        rules,
        blind,
        later,
        reads_open,
        pairs,
        at_root,
        first_above: OnceLock::new(),
        said: AtomicBool::new(false),
    })
}

/// **Write `s`'s state's `open` facts**: `(open R)` for each relation `owes`
/// holds an instance of, and `(open F)` for each hypothesis a writer gives
/// there (the module's list), by one reading at root and at every state
/// above it. Where `above`, a hypothesis root does not hold is the in-flight
/// check's (Q-M1f.574). Returns the facts it wrote that the state did not
/// hold; empty where a lent table could not number one, which the fan-out
/// reads off `Terms::refused`.
fn write(
    s: &mut Session<'_>,
    writers: &Writers,
    owes: &Owes,
    above: bool,
) -> Result<Vec<FactId>, CompileError> {
    let open = s.terms.kernel.open;
    let hypotheses = hypotheses_at(s, writers)?;
    let mut facts: Vec<(FactId, Symbol)> = Vec::new();
    // `(open R)`, once per relation owed, in first-owed order.
    for o in owes.instances() {
        let Some(r) = o.relation else { continue };
        let Ok(id) = s.terms.intern_fact(open, &[Value::sym(r)]) else {
            return Ok(Vec::new());
        };
        if !facts.iter().any(|&(f, _)| f == id) {
            facts.push((id, o.rule));
        }
    }
    let mut seen: FxHashSet<FactId> = FxHashSet::default();
    for (rule, f) in hypotheses {
        if !seen.insert(f) {
            continue;
        }
        if above && !writers.at_root.contains(&f) {
            first_above(writers, s.terms, f);
        }
        let Ok(id) = s.terms.intern_fact(open, &[Value::fact(f)]) else {
            return Ok(Vec::new());
        };
        facts.push((id, rule.unwrap_or(open)));
    }
    let fresh: Vec<(FactId, Symbol)> = facts
        .into_iter()
        .filter(|&(id, _)| !s.kb.contains(id))
        .collect();
    let mut records: Vec<(FactId, ein_core::ProvId)> = Vec::with_capacity(fresh.len());
    for &(id, rule) in &fresh {
        let prov = Prov::from_rule(rule, Box::new([]), None);
        records.push((id, s.terms.provs.push(prov)));
    }
    let ids: Vec<FactId> = fresh.iter().map(|&(id, _)| id).collect();
    s.kb.write_open(s.terms, &ids, |id| {
        records.iter().find(|&&(f, _)| f == id).map(|&(_, p)| p)
    });
    Ok(ids)
}

/// The hypotheses each writer gives at `s`'s state (the module's list), with
/// the rule that wrote each where one did: every match of a rule that
/// concludes one; what would pay what the state owes; what root proposed for
/// a debt a commitment can bring; and the blind enumerator's list, where it
/// writes; each but a rule's held to the state, a fact it neither holds nor
/// denies.
fn hypotheses_at(
    s: &mut Session<'_>,
    writers: &Writers,
) -> Result<Vec<(Option<Symbol>, FactId)>, CompileError> {
    let mut out: Vec<(Option<Symbol>, FactId)> = Vec::new();
    let _ = writers.rules.candidates_by(s, &mut |_, rule, f| {
        out.push((Some(rule), f));
        std::ops::ControlFlow::Continue(())
    });
    if !s.kb.program().obligations.is_empty() {
        for (rule, f) in crate::oblgen::payments(s)? {
            if !s.kb.contains(f) && !writers.pairs.denies(s.kb, s.terms, f) {
                out.push((Some(rule), f));
            }
        }
    }
    for &f in writers.later.iter() {
        if !s.kb.contains(f) && !writers.pairs.denies(s.kb, s.terms, f) {
            out.push((None, f));
        }
    }
    if let Some(raw) = &writers.blind {
        let closed = closed_relations(s);
        for &f in raw.iter() {
            if !s.kb.contains(f)
                && !writers.pairs.denies(s.kb, s.terms, f)
                && !closed.contains(&s.terms.facts.rel(f))
            {
                out.push((None, f));
            }
        }
    }
    Ok(out)
}

fn closed_relations(s: &Session<'_>) -> FxHashSet<Symbol> {
    let mut out = FxHashSet::default();
    for f in s.kb.facts_of(s.terms.kernel.closed) {
        if let [arg] = s.terms.facts.args(f)
            && let Some(rel) = arg.as_sym()
        {
            out.insert(rel);
        }
    }
    out
}

/// A hypothesis a state above root holds that root does not: recorded once,
/// and said at once, once, through the hook (Q-M1f.574).
fn first_above(writers: &Writers, terms: &Terms, f: FactId) {
    let text = crate::events::sexpr(terms, f);
    let _ = writers.first_above.set(text.clone());
    if !writers.said.swap(true, Ordering::Relaxed)
        && let Some(hook) = HOOK.lock().expect("no hook panicked").as_ref()
    {
        hook(&text);
    }
}
