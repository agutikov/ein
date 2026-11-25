//! The hypothesis mechanisms a program declares, read at a state
//! ([04-hypothesis-generation.md § 6](../../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#6-scoping-and-the-mechanisms-judged-together)).
//!
//! Three writers of a hypothesis, `(open F)` (Q-M1f.566, Q-M1f.570): a rule
//! that concludes one, with the `(open …)` the program writes; an obligation, activated, which writes `(open R)` where an
//! instance is owed and `(open F)` for what would pay it; and the blind
//! enumerator, which runs only where root holds no `open` fact and no
//! obligation is activated ([`crate::hypgen`]). Three declarations configure
//! the blind enumerator, a white list and a black list (Q-M1f.578):
//!
//! | | mechanism | read here as |
//! |---|---|---|
//! | H | a rule that concludes `(open F)`, activated by a fact of its name | [`Mechanisms::proposals`]: each active plan and the relation each hypothesis is of |
//! | O | an obligation rule, `:assert (open ?R)` | [`Mechanisms::obligations`]: each obligation plan the state's activators compile, and the relation it owes |
//! | HR | `(solve :hypothesis-relations …)` | [`Mechanisms::listed`] |
//! | NH | `(solve :no-hypothesis …)` | [`Mechanisms::excluded`] |
//! | C | `(__closed__ R)`, stated or derived | [`Mechanisms::closed`] |
//! | | the `open` facts the state holds | [`Mechanisms::open`] |
//!
//! [`read`] reads; it decides nothing and writes nothing but the plans it
//! compiles into the session's memo, which a later reader of the same memo
//! would compile alike.
//!
//! ### The check
//!
//! [`check`] judges the program on root's fixpoint, at load (Q-M1f.245),
//! **against the `open` facts root holds** (Q-M1f.578): where root holds
//! one, a declaration of the blind enumerator that disagrees with them is a
//! **conflict**, which refuses the program (Q-M1f.578):
//!
//! | the declaration | a conflict where root holds an `open` fact |
//! |---|---|
//! | C | of a relation it closes |
//! | NH | of a relation it names |
//! | HR | of a relation it leaves out, or none of a relation it names |
//!
//! An `open` fact is of the relation it owes, `(open R)`, or of its
//! hypothesis's, `(open F)`. Beside `open` facts that agree with them the
//! lists filter nothing, since the blind enumerator does not run there, and
//! say so nowhere. Where root holds none, they are the blind enumerator's,
//! and two of them still contradict each other: a relation in both lists,
//! and a listed relation that is closed. A **redundant** name, one a
//! `:no-hypothesis` names that another declaration already takes out, is a
//! `note:`.
//!
//! The lines carry no prefix: `ein-cli` prints a conflict as a load error and
//! a note as `note:`, on every command, before the command reads anything
//! else.
//!
//! **`(solve :no-search)`** (Q-M1f.254), [`Mechanisms::no_search`], states
//! that the lattice is root alone: no hypothesis is proposed
//! ([`crate::hypgen::declares_no_search`]). Beside an obligation it is
//! silent, the obligation judging root alone, which is its use; beside a
//! closure silent too. Beside a rule that concludes a hypothesis it is one
//! note, *the rule proposes no candidate*, and beside each list one, *the
//! list filters no candidate*, which replace the notes above. It relaxes no
//! conflict: the `open` facts root holds are written beside it, and every
//! conflict stands (Q-M1f.254).

use ein_core::{Diagnostic, FactId, Kb, Kind, Place, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};
use rustc_hash::FxHashSet;

use crate::compile::{CompileError, SharedMemo};
use crate::events::Events;
use crate::saturator::{Saturator, Session};

/// One plan of a generating mechanism: the rule it is compiled from, the
/// activator's arguments (empty for a rule without parameters), and the
/// relation it is about — the one a hypothesis `(open F)` is a fact of, or
/// the one an obligation's `(open ?R)` owes. `None` where the plan names
/// none: a hypothesis a variable holds, `(open ?f)`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
    /// The activator fact itself, which the state holds. `None` for a rule
    /// without parameters.
    pub activator_fact: Option<FactId>,
    pub relation: Option<Symbol>,
}

/// One `open` fact a state holds: the fact, the relation it is of, and what
/// wrote it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Open {
    pub fact: FactId,
    /// The relation it owes, `(open R)`, or its hypothesis's, `(open F)`.
    pub relation: Symbol,
}

/// What a program declares, at one state.
#[derive(Clone, Default, Debug)]
pub struct Mechanisms {
    /// Every rule the program declares that concludes a hypothesis, in load
    /// order, active or not.
    pub hypothesis_rules: Vec<Symbol>,
    /// The active plans of those, in the generator's order, one per
    /// hypothesis each states.
    pub proposals: Vec<Plan>,
    /// Every obligation rule the program declares, in load order.
    pub obligation_rules: Vec<Symbol>,
    /// The obligation plans this state's activators compile, in report order.
    /// Non-empty, the obligations write; an obligation rule in
    /// `obligation_rules` with no plan here writes nothing.
    pub obligations: Vec<Plan>,
    /// `:hypothesis-relations`, as the generator reads it: `None` where the
    /// program writes none.
    pub listed: Option<FxHashSet<Symbol>>,
    /// `:no-hypothesis`, as the generator reads it.
    pub excluded: FxHashSet<Symbol>,
    /// Every `R` with `(__closed__ R)` at this state.
    pub closed: FxHashSet<Symbol>,
    /// `(solve :no-search)`: the program states its lattice is root alone,
    /// so no writer of H runs.
    pub no_search: bool,
    /// The `open` facts the state holds, in the order written: at root's
    /// fixpoint, once the pass has written them ([`crate::open`]).
    pub open: Vec<Open>,
}

impl Mechanisms {
    /// The relations the active rules that conclude a hypothesis propose
    /// facts of.
    pub fn proposed(&self) -> FxHashSet<Symbol> {
        self.proposals.iter().filter_map(|p| p.relation).collect()
    }

    /// The relations an obligation of this state can owe.
    pub fn owable(&self) -> FxHashSet<Symbol> {
        self.obligations.iter().filter_map(|p| p.relation).collect()
    }
}

/// Read what the program declares at `s`'s state: written declarations as
/// they were loaded, and closures and activators as the state holds them.
pub fn read(s: &mut Session<'_>) -> Result<Mechanisms, CompileError> {
    let program = s.kb.program_arc();
    let hypothesis_rules: Vec<Symbol> = program.hypotheses.iter().map(|(n, _)| n).collect();
    let proposals = if hypothesis_rules.is_empty() {
        Vec::new()
    } else {
        crate::hypothesis_rules::HypothesisRules::new(s)?
            .proposals(s.terms)
            .iter()
            .map(|(p, relation)| Plan {
                rule: p.rule,
                activator: p.activator_args.clone(),
                activator_fact: p.activator,
                relation: *relation,
            })
            .collect()
    };
    let open: Vec<Open> =
        s.kb.open_facts()
            .iter()
            .filter_map(|&fact| {
                let arg = *s.terms.facts.args(fact).first()?;
                let relation = match arg.as_fact() {
                    Some(f) => s.terms.facts.rel(f),
                    None => arg.as_sym()?,
                };
                Some(Open { fact, relation })
            })
            .collect();
    let obligation_rules: Vec<Symbol> = program.obligations.iter().map(|(n, _)| n).collect();
    let obligations = obligation_plans(s)?
        .iter()
        .map(|p| Plan {
            rule: p.rule,
            activator: p.activator_args.clone(),
            activator_fact: p.activator,
            relation: crate::obligations::open_argument(p, s.terms),
        })
        .collect();
    let (listed, excluded, closed) = crate::hypgen::scoping(s);
    Ok(Mechanisms {
        hypothesis_rules,
        proposals,
        obligation_rules,
        obligations,
        listed,
        excluded,
        closed,
        no_search: crate::hypgen::declares_no_search(&program, s.ast),
        open,
    })
}

/// **The obligations activated at `s`'s state**: one plan per obligation rule
/// and activator the state holds, a rule without parameters always, in report
/// order ([`crate::obligations`]); none where the program loads no
/// obligation rule. An obligation rule nothing activates is loaded and has no
/// plan here.
///
/// The one reading of O: [`read`]'s [`Mechanisms::obligations`]; the
/// obligations write a share of H where this is not empty at the state H is
/// built at, root ([`crate::oblgen`]). The plans are
/// compiled into the session's memo, as every reader of obligation plans
/// compiles them.
pub(crate) fn obligation_plans(
    s: &mut Session<'_>,
) -> Result<Vec<std::sync::Arc<crate::plan::Plan>>, CompileError> {
    if s.kb.program().obligations.is_empty() {
        return Ok(Vec::new());
    }
    let memo = s.memo.clone();
    crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)
}

// ── The check ──────────────────────────────────────────────────────

/// What [`check`] found: the conflicts, which refuse the program, and the
/// notes, which do not, each a finding of [`Kind::Mechanisms`]
/// (`07-diagnostics.md` § 2.26), an error or a note, placed where the
/// program writes what it is about. Each message is a line with its position
/// first where the program writes one, and no prefix.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Report {
    pub conflicts: Vec<Diagnostic>,
    pub notes: Vec<Diagnostic>,
}

/// **Could two mechanisms meet in this program?** Read off the loaded
/// program, before any saturation, so that a program declaring at most one
/// pays nothing: a rule that concludes a hypothesis, an obligation rule, an
/// `(open …)` fact the program writes, `:hypothesis-relations` and
/// `:no-hypothesis` are each present or not as written, and a closure is
/// possible wherever `__closed__` is written, a fact's head or argument or
/// any word of a rule's `:assert`. Two of them, or one and a possible
/// closure, and [`check`] has something to read.
pub fn may_conflict(kb: &Kb, terms: &Terms, ast: &Ast) -> bool {
    let program = kb.program();
    let lists = |key: &str| {
        program.solve().is_some_and(|solve| {
            solve
                .kw_pairs
                .iter()
                .any(|&pair| keyword_value(ast, NodeId(pair.0), key).is_some())
        })
    };
    let present = [
        !program.hypotheses.is_empty(),
        !program.obligations.is_empty(),
        !kb.open_facts().is_empty(),
        lists(crate::hypgen::HYPOTHESIS_RELATIONS),
        lists(crate::hypgen::NO_HYPOTHESIS),
        lists(crate::hypgen::NO_SEARCH),
    ]
    .iter()
    .filter(|&&b| b)
    .count();
    if present >= 2 {
        return true;
    }
    if present == 0 {
        return false;
    }
    let closed = terms.kernel.closed;
    if kb.facts_of(closed).next().is_some() || kb.name_as_arg(closed).next().is_some() {
        return true;
    }
    program
        .every_rule()
        .filter_map(|(_, r)| r.assert_.as_ref())
        .any(|a| writes_name(ast, NodeId(a.expr.0), ein_core::terms::CLOSED))
}

/// **The judgement a load is followed by** (Q-M1f.245), on every command and
/// for a library caller alike: the loader of `ein-ir` reads no fixpoint, so
/// the conflicts between a program's hypothesis mechanisms, which are read on
/// root's, are judged here, after it. A program that cannot declare two
/// mechanisms ([`may_conflict`]) pays nothing; otherwise `again` loads the
/// program a second time into the arenas it is handed, or copies `ast`,
/// `terms` and `kb` into them, and [`check`] saturates that, leaving the
/// caller's own tables as they were. `Err` holds the conflicts, which refuse
/// the program, each a line `kb load error: ` is printed before; `Ok` the
/// notes, which do not, each printed after `note: ` when the command's run
/// ends (Q-M1f.580). A root that does not saturate, or a plan that does not
/// compile, judges nothing: the run that reads the program reports it.
pub fn judge_loaded(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    again: impl FnOnce(&mut Ast, &mut Terms) -> Option<Kb>,
) -> Result<Vec<Diagnostic>, Vec<Diagnostic>> {
    if !may_conflict(kb, terms, ast) {
        return Ok(Vec::new());
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = again(&mut ast, &mut terms) else {
        return Ok(Vec::new());
    };
    let Some(report) = check(&ast, &mut terms, &mut kb) else {
        return Ok(Vec::new());
    };
    if report.conflicts.is_empty() {
        Ok(report.notes)
    } else {
        Err(report.conflicts)
    }
}

/// Saturate `kb` to root's fixpoint and judge its mechanisms. `kb` is the
/// caller's to throw away: `ein-cli` loads the program a second time for it,
/// so that the command's own tables are not touched. `None` where root does
/// not saturate or a plan does not compile: the command that reads the
/// program reports that itself.
pub fn check(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Option<Report> {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).ok()?;
    sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    // Root's `open` facts, as every command writes them at root's fixpoint.
    let mut stats = crate::hypgen::HypGenStats::default();
    crate::open::at_root(
        &mut s,
        &mut sat,
        &mut stats,
        &mut |_| std::ops::ControlFlow::Continue(()),
        &mut |_| {},
    )
    .ok()?;
    let mut m = read(&mut s).ok()?;
    // What the blind enumerator wrote is what the lists and a closure let it
    // write: they configure it, and its `open` facts are no other writer's.
    if stats.rung.mode == crate::oblgen::Mode::Blind {
        m.open.clear();
    }
    Some(judge(&m, s.kb, s.terms, s.ast))
}

/// The declarations, judged: § The check's table, in its order.
pub fn judge(m: &Mechanisms, kb: &Kb, terms: &Terms, ast: &Ast) -> Report {
    let w = Words { kb, terms, ast };
    let solve = kb.program().solve();
    let list = |key: &str| -> Option<(NodeId, Vec<(NodeId, String)>)> {
        let solve = solve?;
        solve.kw_pairs.iter().find_map(|&pair| {
            keyword_value(ast, NodeId(pair.0), key).map(|value| {
                // `()` keeps no position of its own: the keyword's is the
                // list's then.
                let Node::KwPair { key, .. } = ast.node(NodeId(pair.0)) else {
                    unreachable!("keyword_value read a pair")
                };
                let at = if ast.loc(value).is_some() { value } else { key };
                (at, list_names(ast, value))
            })
        })
    };
    let hr = list(crate::hypgen::HYPOTHESIS_RELATIONS);
    let nh = list(crate::hypgen::NO_HYPOTHESIS);
    let name = |r: Symbol| terms.sym(r).to_string();
    let in_list = |l: &Option<(NodeId, Vec<(NodeId, String)>)>, r: &str| -> Option<NodeId> {
        l.as_ref()?
            .1
            .iter()
            .find(|(_, n)| n == r)
            .map(|&(node, _)| node)
    };
    let left_out = |r: Symbol| m.listed.as_ref().is_some_and(|l| !l.contains(&r));
    let mut out = Report::default();
    let mut conflict = |place: Option<Place>, line: String| {
        if !out.conflicts.iter().any(|d| d.message == line) {
            out.conflicts
                .push(Diagnostic::error(Kind::Mechanisms, line).at(place));
        }
    };
    let note =
        |place: Option<Place>, line: String| Diagnostic::note(Kind::Mechanisms, line).at(place);
    const CONFIGURES: &str = "the two lists and a closure configure the blind enumerator, which \
                              does not run beside an open fact, and one that disagrees with the \
                              open facts root holds is an error";

    // The `open` facts root holds against NH, HR and C (Q-M1f.578): one line
    // per relation, at its first `open` fact.
    let mut said: Vec<Symbol> = Vec::new();
    for o in &m.open {
        if said.contains(&o.relation) {
            continue;
        }
        said.push(o.relation);
        let rel = name(o.relation);
        let held = w.open_fact(o.fact);
        if let Some(node) = in_list(&nh, &rel) {
            conflict(
                w.place(node),
                format!(
                    "{}solve: its :no-hypothesis names {rel}, and root holds {held} — \
                     {CONFIGURES}: drop {rel} from the list",
                    w.at(node),
                ),
            );
        }
        if let Some((value, listed)) = &hr
            && !listed.iter().any(|(_, n)| *n == rel)
        {
            conflict(
                w.place(*value),
                format!(
                    "{}solve: its :hypothesis-relations leaves out {rel}, and root holds {held} — \
                     {CONFIGURES}, so beside them it names exactly their relations: add {rel} to \
                     the list, or drop the list",
                    w.at(*value),
                ),
            );
        }
        if m.closed.contains(&o.relation) {
            // Placed where the closure is written, else where what writes the
            // `open` fact is: an obligation's activator, a rule's `:assert`,
            // the program's own `(open …)`.
            let node = w
                .written(w.closure_fact(o.relation))
                .or_else(|| w.written(Some(o.fact)))
                .or_else(|| {
                    m.obligations
                        .iter()
                        .filter(|p| p.relation == Some(o.relation))
                        .find_map(|p| w.written(p.activator_fact))
                })
                .or_else(|| {
                    m.proposals
                        .iter()
                        .filter(|p| p.relation == Some(o.relation))
                        .find_map(|p| w.assert_of(p.rule))
                });
            let at = node.map_or_else(String::new, |n| w.at(n));
            conflict(
                node.and_then(|n| w.place(n)),
                format!(
                    "{at}root holds {held}, and {} says {rel} is closed — {CONFIGURES}, and a \
                     closed relation is one no commitment extends: drop the closure, or what \
                     writes {}",
                    w.closure(o.relation),
                    terms.fact_text(o.fact),
                ),
            );
        }
    }
    if !m.open.is_empty()
        && let Some((_, listed)) = &hr
    {
        for (node, r) in listed {
            if !m.open.iter().any(|o| terms.sym(o.relation) == r) {
                conflict(
                    w.place(*node),
                    format!(
                        "{}solve: its :hypothesis-relations names {r}, and root holds no open \
                         fact of {r} — {CONFIGURES}, so beside them it names exactly their \
                         relations: drop {r} from the list",
                        w.at(*node),
                    ),
                );
            }
        }
    }
    // HR + NH, HR + C.
    if let (Some((_, listed)), Some((_, named))) = (&hr, &nh) {
        for (node, r) in named {
            if listed.iter().any(|(_, l)| l == r) {
                conflict(
                    w.place(*node),
                    format!(
                        "{}solve: its :no-hypothesis names {r}, and its :hypothesis-relations \
                         names {r} too — the one says guess it, the other never: drop {r} from \
                         one",
                        w.at(*node),
                    ),
                );
            }
        }
    }
    if let Some((_, listed)) = &hr {
        for (node, r) in listed {
            let Some(sym) = terms.syms.get(r) else {
                continue;
            };
            if m.closed.contains(&sym) {
                conflict(
                    w.place(*node),
                    format!(
                        "{}solve: its :hypothesis-relations names {r}, and {} says {r} is closed \
                         — a listed relation is one the program guesses, and a closed one it \
                         never does: drop {r} from the list, or the closure",
                        w.at(*node),
                        w.closure(sym),
                    ),
                );
            }
        }
    }
    if !out.conflicts.is_empty() {
        return out;
    }

    let h = !m.hypothesis_rules.is_empty();
    let rules = || {
        m.hypothesis_rules
            .iter()
            .map(|&h| format!("'{}'", terms.sym(h)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    // `(solve :no-search)`: nothing is proposed, so a rule that concludes a
    // hypothesis proposes no candidate and a list filters nothing, each a
    // note; beside an obligation it is silent, the obligation judging root
    // alone, which is its use. The conflicts above stand.
    if m.no_search {
        let node = list(crate::hypgen::NO_SEARCH).map(|(at, _)| at);
        let at = node.map_or(String::new(), |at| w.at(at));
        let place = node.and_then(|at| w.place(at));
        if h {
            let (one, word) = if m.hypothesis_rules.len() == 1 {
                ("rule", "proposes")
            } else {
                ("rules", "propose")
            };
            out.notes.push(note(
                place,
                format!(
                    "{at}solve: its :no-search states the lattice is root alone, so {one} {} \
                     {word} no candidate",
                    rules(),
                ),
            ));
        }
        for (key, l) in [
            (crate::hypgen::HYPOTHESIS_RELATIONS, &hr),
            (crate::hypgen::NO_HYPOTHESIS, &nh),
        ] {
            let Some((value, _)) = l else { continue };
            out.notes.push(note(
                w.place(*value),
                format!(
                    "{}solve: its :{key} filters no candidate — the program states :no-search, \
                     whose lattice is root alone",
                    w.at(*value),
                ),
            ));
        }
        return out;
    }
    // Where root holds no `open` fact, the lists are the blind enumerator's:
    // a name the other declarations already take out is redundant.
    if m.open.is_empty()
        && let Some((_, named)) = &nh
    {
        for (node, r) in named {
            let sym = terms.syms.get(r);
            if hr.is_some() && sym.is_none_or(left_out) {
                out.notes.push(note(
                    w.place(*node),
                    format!(
                        "{}solve: its :no-hypothesis names {r}, which its :hypothesis-relations \
                         already leaves out",
                        w.at(*node),
                    ),
                ));
            } else if let Some(sym) = sym
                && m.closed.contains(&sym)
            {
                out.notes.push(note(
                    w.place(*node),
                    format!(
                        "{}solve: its :no-hypothesis names {r}, which {} already closes",
                        w.at(*node),
                        w.closure(sym),
                    ),
                ));
            }
        }
    }
    out
}

/// What the lines say of a declaration.
struct Words<'a> {
    kb: &'a Kb,
    terms: &'a Terms,
    ast: &'a Ast,
}

impl Words<'_> {
    /// `FILE:LINE:COL: `, or nothing where the parser kept no position.
    fn at(&self, node: NodeId) -> String {
        match self.ast.loc(node) {
            Some(l) => format!("{}: ", ein_ir::loc_repr(self.ast, Some(l))),
            None => String::new(),
        }
    }

    /// [`Words::at`]'s position, as a finding's place.
    fn place(&self, node: NodeId) -> Option<Place> {
        ein_ir::place(self.ast, self.ast.loc(node))
    }

    /// The form that writes a fact, which [`Words::at`] places; `None` for a
    /// fact a rule derived, or one the program's text does not show.
    fn written(&self, fact: Option<FactId>) -> Option<NodeId> {
        let fact = fact?;
        let prov = self.terms.provs.get(self.kb.primary(fact)?);
        if prov.rule.is_some() {
            return None;
        }
        self.form_of(fact)
    }

    /// The form that writes `fact`: the first of the program's forms whose
    /// head and arguments are the fact's names, a fact being stated with no
    /// position of its own
    /// ([07-diagnostics.md § 1.5](../../../../docs/ein/user/04-reference/07-diagnostics.md#15-loader-messages-about-a-top-level-form-end-in-at-none))
    /// while its form's
    /// words carry theirs. A nested argument, `(open (q b))`'s, is matched
    /// as its form at any depth.
    fn form_of(&self, fact: FactId) -> Option<NodeId> {
        (0..self.ast.arena_sizes().0 as u32)
            .map(NodeId)
            .find_map(|node| {
                let Node::SForm { head, .. } = self.ast.node(node) else {
                    return None;
                };
                self.writes(node, Value::fact(fact))
                    .then(|| {
                        if self.ast.loc(node).is_some() {
                            node
                        } else {
                            head
                        }
                    })
                    .filter(|&at| self.ast.loc(at).is_some())
            })
    }

    /// Does `node` write `value`: an atom its name, a form a fact's relation
    /// and arguments, at any depth?
    fn writes(&self, node: NodeId, value: Value) -> bool {
        if let Some(s) = value.as_sym() {
            return self.ast.atom_name(node) == Some(self.terms.sym(s));
        }
        let Some(fact) = value.as_fact() else {
            return false;
        };
        let Node::SForm { head, .. } = self.ast.node(node) else {
            return false;
        };
        let (rel, args) = self.terms.facts.get(fact);
        let a = self.ast.form_args(node);
        self.ast.atom_name(head) == Some(self.terms.sym(rel))
            && a.len() == args.len()
            && a.iter().zip(args).all(|(&n, &want)| self.writes(n, want))
    }

    /// Where the program writes `fact`, `written at FILE:L:C`; `None` for a
    /// fact its text does not show.
    fn written_at(&self, fact: FactId) -> Option<String> {
        let node = self.form_of(fact)?;
        let l = self.ast.loc(node)?;
        Some(format!(
            "written at {}",
            ein_ir::loc_repr(self.ast, Some(l))
        ))
    }

    /// A fact, and where it comes from: `, which rule 'r' derives,` or
    /// `, written at FILE:LINE:COL,`, the comma closing the clause; the fact
    /// alone where neither is known.
    fn sourced(&self, fact: FactId) -> String {
        let text = self.terms.fact_text(fact);
        let Some(prov) = self.kb.primary(fact).map(|id| self.terms.provs.get(id)) else {
            return text;
        };
        if let Some(rule) = prov.rule {
            return format!("{text}, which rule '{}' derives,", self.terms.sym(rule));
        }
        match self.form_of(fact).and_then(|n| self.ast.loc(n)) {
            Some(l) => format!(
                "{text}, written at {},",
                ein_ir::loc_repr(self.ast, Some(l))
            ),
            None => text,
        }
    }

    fn closure_fact(&self, r: Symbol) -> Option<FactId> {
        self.terms
            .probe_fact(self.terms.kernel.closed, &[Value::sym(r)])
            .filter(|&f| self.kb.contains(f))
    }

    /// `(__closed__ R), which rule 'r' derives,`.
    fn closure(&self, r: Symbol) -> String {
        match self.closure_fact(r) {
            Some(f) => self.sourced(f),
            None => format!("(__closed__ {})", self.terms.sym(r)),
        }
    }

    /// Rule `rule`'s `:assert`, where it is written, which [`Words::at`]
    /// places: a rule that concludes a hypothesis.
    fn assert_of(&self, rule: Symbol) -> Option<NodeId> {
        let r = self.kb.program().hypotheses.get(rule)?;
        let a = r.assert_.as_ref()?;
        self.ast.loc(NodeId(a.expr.0)).map(|_| NodeId(a.expr.0))
    }

    /// An `open` fact root holds, and what wrote it: `(open (p a)), which
    /// rule 'r' writes`, `(open p), which obligation 'o' writes`, or
    /// `(open (p a)), written at FILE:L:C`.
    fn open_fact(&self, fact: FactId) -> String {
        let text = self.terms.fact_text(fact);
        let Some(prov) = self.kb.primary(fact).map(|id| self.terms.provs.get(id)) else {
            return text;
        };
        match prov.rule {
            Some(r) if r == self.terms.kernel.open => format!("{text}, a hypothesis root holds"),
            Some(r) => {
                let program = self.kb.program();
                let kind = if program.obligations.contains(r) {
                    "obligation"
                } else {
                    "rule"
                };
                format!("{text}, which {kind} '{}' writes", self.terms.sym(r))
            }
            None => match prov.loc {
                Some(l) => format!(
                    "{text}, written at {}",
                    ein_ir::loc_repr(
                        self.ast,
                        Some(ein_ir::Loc {
                            file: ein_ir::FileId(l.file),
                            line: l.line,
                            col: l.col,
                        })
                    )
                ),
                None => match self.written_at(fact) {
                    Some(at) => format!("{text}, {at}"),
                    None => format!("{text}, which the program writes"),
                },
            },
        }
    }
}

/// `(solve …)`'s `:KEY` value, where `pair` is that keyword's pair.
fn keyword_value(ast: &Ast, pair: NodeId, key: &str) -> Option<NodeId> {
    let Node::KwPair { key: k, value } = ast.node(pair) else {
        return None;
    };
    let Node::Keyword(name) = ast.node(k) else {
        return None;
    };
    (ast.sym(name) == key).then_some(value)
}

/// A list's names with their nodes, as `hypgen` reads them: a bare name, or a
/// form's head and its atom arguments; the internal names (`()`) none.
fn list_names(ast: &Ast, value: NodeId) -> Vec<(NodeId, String)> {
    let nodes: Vec<NodeId> = match ast.node(value) {
        Node::Atom(_) => vec![value],
        Node::SForm { head, .. } => std::iter::once(head)
            .chain(ast.form_args(value).iter().copied())
            .collect(),
        _ => Vec::new(),
    };
    nodes
        .into_iter()
        .filter_map(|n| ast.atom_name(n).map(|s| (n, s.to_string())))
        .filter(|(_, s)| !ein_core::text::INTERNAL_NAMES.contains(&s.as_str()))
        .collect()
}

/// Does `node` write the name `want`, anywhere in it?
fn writes_name(ast: &Ast, node: NodeId, want: &str) -> bool {
    match ast.node(node) {
        Node::Atom(_) => ast.atom_name(node) == Some(want),
        Node::SForm { head, .. } => {
            writes_name(ast, head, want)
                || ast
                    .form_args(node)
                    .iter()
                    .any(|&a| writes_name(ast, a, want))
        }
        Node::KwPair { value, .. } => writes_name(ast, value, want),
        _ => false,
    }
}
