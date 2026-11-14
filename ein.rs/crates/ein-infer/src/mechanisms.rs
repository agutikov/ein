//! The hypothesis mechanisms a program declares, read at a state — M1f
//! [S1f.6.34](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.34_hypothesis_mechanisms_in_conflict.md).
//!
//! Five declarations steer what a commitment may be about, and a sixth
//! mechanism, the blind enumerator, is what runs where none of the generating
//! ones does ([`crate::hypgen`]'s ladder):
//!
//! | | mechanism | read here as |
//! |---|---|---|
//! | H | `(hrule …)`, activated by `(solve :hrules …)` | [`Mechanisms::proposals`]: each active hrule plan and the relation its `:assert` proposes |
//! | O | an obligation rule, `:assert (open ?R)` | [`Mechanisms::obligations`]: each obligation plan the state's activators compile, and the relation it owes |
//! | HR | `(solve :hypothesis-relations …)` | [`Mechanisms::listed`] |
//! | NH | `(solve :no-hypothesis …)` | [`Mechanisms::excluded`] |
//! | C | `(__closed__ R)`, stated or derived | [`Mechanisms::closed`] |
//!
//! An hrule is **active** where the generator would compile a plan of it: one
//! without parameters always, one with parameters once per `:hrules`
//! activator of its arity ([`crate::hrule::Hrules::new`]). An obligation is
//! one whose activator the state holds ([`crate::obligations`]'s plans), so a
//! *derived* activator, `bijective-setup`'s `(total color-loc is-a)`, is read
//! only at a fixpoint, as a derived closure is: the reader is meant for
//! root's, before any search.
//!
//! [`read`] reads; it decides nothing and writes nothing but the plans it
//! compiles into the session's memo, which a later reader of the same memo
//! would compile alike.
//!
//! ### The check
//!
//! [`check`] judges the pairs on root's fixpoint, the user's ruling of
//! 2026-11-13 on E-47 (*"report conflict if any hypothesis mechanism
//! constradicts itself"*) and their table of the same day. A **conflict** is a
//! pair whose two declarations say opposite things of one relation; a **dead**
//! list is one that filters no candidate, and a **redundant** name one that
//! says what another declaration already says:
//!
//! | pair | conflict | note |
//! |---|---|---|
//! | H + O | never: H generates, O judges | — |
//! | H + HR | an active hrule proposes a relation the list leaves out | the list is dead |
//! | H + NH | an active hrule proposes a relation the list names | the list is dead |
//! | H + C | an active hrule proposes a closed relation | — |
//! | O + HR | no hrule, and an obligation owes a relation the list leaves out | — |
//! | O + NH | no hrule, and an obligation owes a relation the list names | — |
//! | O + C | an obligation owes a closed relation | — |
//! | HR + NH | a relation in both | no hrule: NH names one HR leaves out |
//! | HR + C | a listed relation is closed | — |
//! | NH + C | never | no hrule: NH names a closed relation |
//!
//! An obligation is read as a plan root's activators compile, whether or not
//! root owes it, as an hrule is read as active whether or not its `:match`
//! has a row. Under an hrule the two lists filter nothing (E-52), so their
//! dead note replaces a redundancy note about them. The lines carry no
//! prefix: `ein-cli` prints a conflict as a load error and a note as `note:`,
//! on every command, before the command reads anything else.
//!
//! **`(solve :no-search)`** (M1f
//! [S1f.6.36](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.36_no_search.md)),
//! [`Mechanisms::no_search`], states that the lattice is root alone: no rung
//! runs ([`crate::hypgen::declares_no_search`]). Beside an obligation it is
//! silent, the obligation judging root alone, which is its use; beside a
//! closure silent too. Beside an hrule it is one note, *the hrule proposes no
//! candidate*, and beside each list one, *the list filters no candidate*,
//! which replace the notes above. It relaxes no conflict: the table's are read
//! as declared, as an obligation is, and stand (Q-M1f.254).

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};
use rustc_hash::FxHashSet;

use crate::compile::{CompileError, SharedMemo, asserted_relation};
use crate::events::Events;
use crate::saturator::{Saturator, Session};

/// One plan of a generating mechanism: the rule it is compiled from, the
/// activator's arguments (empty for a rule without parameters), and the
/// relation it is about — the one an hrule's `:assert` proposes a fact of,
/// or the one an obligation's `(open ?R)` owes. `None` where the plan names
/// none: a bare `(open)`, or an hrule whose `:assert` is no fact's template.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
    /// The activator fact itself: an obligation's, which the state holds; an
    /// hrule's synthetic `(h args…)`, which it does not. `None` for a rule
    /// without parameters.
    pub activator_fact: Option<FactId>,
    pub relation: Option<Symbol>,
}

/// What a program declares, at one state.
#[derive(Clone, Default, Debug)]
pub struct Mechanisms {
    /// Every `(hrule …)` the program declares, in load order, active or not.
    /// Non-empty is what takes the ladder's first rung.
    pub hrules: Vec<Symbol>,
    /// The active hrule plans, in the generator's order.
    pub proposals: Vec<Plan>,
    /// Every obligation rule the program declares, in load order.
    pub obligation_rules: Vec<Symbol>,
    /// The obligation plans this state's activators compile, in report order.
    pub obligations: Vec<Plan>,
    /// `:hypothesis-relations`, as the generator reads it: `None` where the
    /// program writes none.
    pub listed: Option<FxHashSet<Symbol>>,
    /// `:no-hypothesis`, as the generator reads it.
    pub excluded: FxHashSet<Symbol>,
    /// Every `R` with `(__closed__ R)` at this state.
    pub closed: FxHashSet<Symbol>,
    /// `(solve :no-search)`: the program states its lattice is root alone,
    /// so no rung runs (M1f S1f.6.36).
    pub no_search: bool,
}

impl Mechanisms {
    /// The relations the active hrules propose facts of.
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
    let hrules: Vec<Symbol> = program.hrules.iter().map(|(n, _)| n).collect();
    let proposals = if hrules.is_empty() {
        Vec::new()
    } else {
        crate::hrule::Hrules::new(s)?
            .plans()
            .iter()
            .map(|p| Plan {
                rule: p.rule,
                activator: p.activator_args.clone(),
                activator_fact: p.activator,
                relation: asserted_relation(p, s.terms),
            })
            .collect()
    };
    let obligation_rules: Vec<Symbol> = program.obligations.iter().map(|(n, _)| n).collect();
    let obligations = if obligation_rules.is_empty() {
        Vec::new()
    } else {
        let memo = s.memo.clone();
        crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?
            .iter()
            .map(|p| Plan {
                rule: p.rule,
                activator: p.activator_args.clone(),
                activator_fact: p.activator,
                relation: crate::obligations::open_argument(p, s.terms),
            })
            .collect()
    };
    let (listed, excluded, closed) = crate::hypgen::scoping(s);
    Ok(Mechanisms {
        hrules,
        proposals,
        obligation_rules,
        obligations,
        listed,
        excluded,
        closed,
        no_search: crate::hypgen::declares_no_search(&program, s.ast),
    })
}

// ── The check ──────────────────────────────────────────────────────

/// What [`check`] found: the conflicts, which refuse the program, and the
/// notes, which do not. Each a line with its position first where the
/// program writes one, and no prefix.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Report {
    pub conflicts: Vec<String>,
    pub notes: Vec<String>,
}

/// **Could two mechanisms meet in this program?** Read off the loaded
/// program, before any saturation, so that a program declaring at most one
/// pays nothing: an hrule, an obligation rule, `:hypothesis-relations` and
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
        !program.hrules.is_empty(),
        !program.obligations.is_empty(),
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
        .rules
        .values()
        .chain(program.hrules.values())
        .chain(program.obligations.values())
        .filter_map(|r| r.assert_.as_ref())
        .any(|a| writes_name(ast, NodeId(a.expr.0), ein_core::terms::CLOSED))
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
    let m = read(&mut s).ok()?;
    Some(judge(&m, s.kb, s.terms, s.ast))
}

/// The pairs, judged: § The check's table, in its order.
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
    let h = !m.hrules.is_empty();
    let mut out = Report::default();
    let mut conflict = |line: String| {
        if !out.conflicts.contains(&line) {
            out.conflicts.push(line);
        }
    };
    let no_list = |rel: &str| {
        format!(
            "no list filters an hrule's candidates, so the program says both that {rel} is \
             guessed and that it is not"
        )
    };
    let stuck = |rel: &str| {
        format!(
            "an obligation is paid by facts of the relation it owes, so a state that owes {rel} \
             and may not be given one is stuck"
        )
    };

    // H + HR, H + NH, H + C.
    for p in &m.proposals {
        let Some(r) = p.relation else { continue };
        let rel = name(r);
        if let Some((value, _)) = &hr
            && left_out(r)
        {
            conflict(format!(
                "{}solve: its :hypothesis-relations leaves out {rel}, and {} proposes facts of \
                 {rel} — {}: drop the list, which filters nothing where an hrule is declared, or \
                 the hrule's {rel}",
                w.at(*value),
                w.hrule(p),
                no_list(&rel),
            ));
        }
        if let Some(node) = in_list(&nh, &rel) {
            conflict(format!(
                "{}solve: its :no-hypothesis names {rel}, and {} proposes facts of {rel} — {}: \
                 drop {rel} from the list, which filters nothing where an hrule is declared, or \
                 the hrule's {rel}",
                w.at(node),
                w.hrule(p),
                no_list(&rel),
            ));
        }
        if m.closed.contains(&r) {
            let at = kb
                .program()
                .hrules
                .get(p.rule)
                .and_then(|rule| rule.assert_.as_ref())
                .map_or(String::new(), |a| w.at(NodeId(a.expr.0)));
            conflict(format!(
                "{at}{} proposes facts of {rel}, and {} says {rel} is closed — a closed relation \
                 is one no commitment extends, and each fact the hrule proposes is a commitment: \
                 drop the closure, or the hrule's {rel}",
                w.hrule(p),
                w.closure(r),
            ));
        }
    }
    // O + HR, O + NH (no hrule), O + C.
    for p in &m.obligations {
        let Some(r) = p.relation else { continue };
        let rel = name(r);
        if !h {
            if let Some((value, _)) = &hr
                && left_out(r)
            {
                conflict(format!(
                    "{}solve: its :hypothesis-relations leaves out {rel}, and {} owes {rel} — {}: \
                     add {rel} to the list",
                    w.at(*value),
                    w.obligation(p),
                    stuck(&rel),
                ));
            }
            if let Some(node) = in_list(&nh, &rel) {
                conflict(format!(
                    "{}solve: its :no-hypothesis names {rel}, and {} owes {rel} — {}: drop {rel} \
                     from the list",
                    w.at(node),
                    w.obligation(p),
                    stuck(&rel),
                ));
            }
        }
        if m.closed.contains(&r) {
            conflict(format!(
                "{}{} owes {rel}, and {} says {rel} is closed — an obligated relation is not \
                 closed: {}; drop the closure, or the obligation",
                w.written(p.activator_fact)
                    .or_else(|| w.written(w.closure_fact(r)))
                    .unwrap_or_default(),
                w.obligation(p),
                w.closure(r),
                stuck(&rel),
            ));
        }
    }
    // HR + NH, HR + C.
    if let (Some((_, listed)), Some((_, named))) = (&hr, &nh) {
        for (node, r) in named {
            if listed.iter().any(|(_, l)| l == r) {
                conflict(format!(
                    "{}solve: its :no-hypothesis names {r}, and its :hypothesis-relations names \
                     {r} too — the one says guess it, the other never: drop {r} from one",
                    w.at(*node),
                ));
            }
        }
    }
    if let Some((_, listed)) = &hr {
        for (node, r) in listed {
            let Some(sym) = terms.syms.get(r) else {
                continue;
            };
            if m.closed.contains(&sym) {
                conflict(format!(
                    "{}solve: its :hypothesis-relations names {r}, and {} says {r} is closed — a \
                     listed relation is one the program guesses, and a closed one it never does: \
                     drop {r} from the list, or the closure",
                    w.at(*node),
                    w.closure(sym),
                ));
            }
        }
    }
    if !out.conflicts.is_empty() {
        return out;
    }

    // The notes: a dead list under an hrule; with none, a redundant name.
    let hrules = || {
        m.hrules
            .iter()
            .map(|&h| format!("'{}'", terms.sym(h)))
            .collect::<Vec<_>>()
            .join(", ")
    };
    // `(solve :no-search)` (M1f S1f.6.36): no rung runs, so an hrule proposes
    // nothing and a list filters nothing, each a note; beside an obligation
    // it is silent, the obligation judging root alone, which is its use. The
    // conflicts above are read as declared, as an obligation is, and stand.
    if m.no_search {
        let at = list(crate::hypgen::NO_SEARCH).map_or(String::new(), |(at, _)| w.at(at));
        if h {
            let (one, word) = if m.hrules.len() == 1 {
                ("hrule", "proposes")
            } else {
                ("hrules", "propose")
            };
            out.notes.push(format!(
                "{at}solve: its :no-search states the lattice is root alone, so {one} {} {word} \
                 no candidate",
                hrules(),
            ));
        }
        for (key, l) in [
            (crate::hypgen::HYPOTHESIS_RELATIONS, &hr),
            (crate::hypgen::NO_HYPOTHESIS, &nh),
        ] {
            let Some((value, _)) = l else { continue };
            out.notes.push(format!(
                "{}solve: its :{key} filters no candidate — the program states :no-search, \
                 whose lattice is root alone",
                w.at(*value),
            ));
        }
        return out;
    }
    for (key, l) in [
        (crate::hypgen::HYPOTHESIS_RELATIONS, &hr),
        (crate::hypgen::NO_HYPOTHESIS, &nh),
    ] {
        let Some((value, _)) = l else { continue };
        if h {
            out.notes.push(format!(
                "{}solve: its :{key} filters no candidate — the program declares hrule {}, and \
                 no list filters an hrule's candidates",
                w.at(*value),
                hrules(),
            ));
        }
    }
    if !h && let Some((_, named)) = &nh {
        for (node, r) in named {
            let sym = terms.syms.get(r);
            if hr.is_some() && sym.is_none_or(left_out) {
                out.notes.push(format!(
                    "{}solve: its :no-hypothesis names {r}, which its :hypothesis-relations \
                     already leaves out",
                    w.at(*node),
                ));
            } else if let Some(sym) = sym
                && m.closed.contains(&sym)
            {
                out.notes.push(format!(
                    "{}solve: its :no-hypothesis names {r}, which {} already closes",
                    w.at(*node),
                    w.closure(sym),
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

    /// Where a fact is written, `FILE:LINE:COL: `; `None` for a fact a rule
    /// derived, or one the program's text does not show.
    fn written(&self, fact: Option<FactId>) -> Option<String> {
        let fact = fact?;
        let prov = self.terms.provs.get(self.kb.primary(fact)?);
        if prov.rule.is_some() {
            return None;
        }
        self.form_of(fact).map(|node| self.at(node))
    }

    /// The form that writes `fact`: the first of the program's forms whose
    /// head and arguments are the fact's names, a fact being stated with no
    /// position of its own (`defined_behaviour.md` § 1.5) while its form's
    /// words carry theirs. `None` for a fact with a nested argument.
    fn form_of(&self, fact: FactId) -> Option<NodeId> {
        let (rel, args) = self.terms.facts.get(fact);
        let rel = self.terms.sym(rel);
        let args: Vec<&str> = args
            .iter()
            .map(|v| v.as_sym().map(|s| self.terms.sym(s)))
            .collect::<Option<_>>()?;
        (0..self.ast.arena_sizes().0 as u32)
            .map(NodeId)
            .find_map(|node| {
                let Node::SForm { head, .. } = self.ast.node(node) else {
                    return None;
                };
                let a = self.ast.form_args(node);
                (self.ast.atom_name(head) == Some(rel)
                    && a.len() == args.len()
                    && a.iter()
                        .zip(&args)
                        .all(|(&n, &want)| self.ast.atom_name(n) == Some(want)))
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

    /// `hrule 'h'`, and its activator as `:hrules` writes it.
    fn hrule(&self, p: &Plan) -> String {
        let h = self.terms.sym(p.rule);
        if p.activator.is_empty() {
            format!("hrule '{h}'")
        } else {
            let args: Vec<String> = p.activator.iter().map(|&v| self.terms.display(v)).collect();
            format!("hrule '{h}' ({})", args.join(" "))
        }
    }

    /// `obligation 'o', activated by (o a b), which rule 'r' derives,`.
    fn obligation(&self, p: &Plan) -> String {
        let o = self.terms.sym(p.rule);
        match p.activator_fact {
            Some(f) => {
                let by = self.sourced(f);
                let comma = if by.ends_with(',') { "" } else { "," };
                format!("obligation '{o}', activated by {by}{comma}")
            }
            None => format!("obligation '{o}'"),
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
