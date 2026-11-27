//! Minimal explanation over the AND/OR proof graph — the ATMS label search.
//!
//! Ein's provenance is an AND/OR **graph**, not a tree: a fact is an OR-node
//! over the derivations the saturator recorded for it, and each derivation is
//! an AND-node over its premises. This module answers the question that
//! structure exists to answer — *what is the smallest set of given facts that
//! forces this conclusion?* — by computing an ATMS label for each fact: the
//! subset-minimal environments it follows from.
//!
//! ### Why it is not a walk
//!
//! [`ein_core::walks::unsat_core`] walks **one** justification per fact, so
//! what it returns is minimal only over the derivations recorded first —
//! flipping two rules' `:priority` flipped the reported core between `{C, Y}`
//! and `{A, B, Y}` while `{C, Y}` still existed. Choosing the best
//! *combination* of justifications is the minimum-axiom-set problem,
//! worst-case exponential, so it needs a real search and an explicit budget.
//!
//! ### What it searches over
//!
//! **Every justification the store holds for a fact**,
//! [`ein_core::Kb::justifications`]: the primary, up to
//! [`ein_core::kb::MAX_ALT_JUSTIFICATIONS`] alternatives kept by **premise
//! count**, and, beside them, the arrival with the **smallest closure**
//! ([`ein_core::Kb::best`], Q-M1e2.11). The promise is
//! the smallest frontier over **every derivation the program records**, and
//! the third is what keeps a full list from hiding the derivation this search
//! would choose: premise count is local and frontier size transitive, so a
//! one-premise step over a deep chain is retained ahead of a two-premise step
//! over givens. `examples/ein-bugs/alt-cap-core.ein` and its `-reordered`
//! twin, its rules written in another order, report the same 2-fact core.
//!
//! Two limits (Q-M1e2.11):
//!
//! - **A saving between two facts, behind a full list, is not offered.** The
//!   derivation a target needs can be its own fact's worst, and no list of
//!   fixed length keeps every such one:
//!   `examples/ein-bugs/wrong-answers/alt-cap-shared.ein` reports 3 facts
//!   where every derivation gives 2, a known gap: no search
//!   re-derives what a full list refused. [`Explanation::exhausted`] does
//!   not see it.
//! - **`:record-alternative-justifications false` records no alternative**,
//!   so the search is offered each fact's first derivation alone and the core
//!   can be another: the one config flag that may change which core is
//!   named, and nothing else
//!   ([06-config.md](../../../../docs/ein/user/04-reference/06-config.md), Q-M1e2.30).
//!
//! It is not a soundness question either way: a larger frontier is still a
//! real explanation.
//!
//! ### A search its budget cut says so
//!
//! **On every surface that prints its core** ([`CoreStatus`]). The search
//! keeps 64 environments per fact and runs 64 rounds
//! ([`ExplanationBudget`]); past either it stops with a real frontier that
//! may not be the smallest, and [`Explanation::cut`] names the cap. The two
//! callers that take a core a surface prints, `region::node_in_region` and
//! `region::Nodes::refutation`, carry the status with the frontier, so a cut
//! core is never printed as the core:
//! `examples/ein-bugs/wrong-answers/core-search-cut.ein`'s `(A k)` has 256
//! environments, and its root core is ten facts where eight suffice,
//! qualified as cut. The search keeps its budget and a
//! cut core says so; the exact core past the budget is not computed. In the
//! corpus `zebra.ein`'s cuts are at the rounds and the `zebra2` family's at
//! the environments.
//!
//! ### A search the run's clock stops
//!
//! **The budget bounds the search's size, not its time**: a round folds
//! every justification of every fact whose label moved, each over up to 64
//! environments a premise, so a dead state whose link several rules derive,
//! each derivation recorded, can fold millions of environments within its
//! 64 rounds. A search a bounded run takes, a core a question about `R`
//! prints, reads the run's clock ([`contradiction_frontier_by`]), the
//! deadline its node's saturation took (Q-M2.16): every
//! [`CLOCK_EVERY`] units of its work, a fact the closure reads, a
//! justification folded or an environment made, and past the deadline it
//! stops with [`Deadline`], which the run reports as it reports the clock
//! anywhere, no answer. It writes nothing on the way, so a search stopped
//! leaves nothing another reader sees. With no deadline it is the search,
//! work for work.
//!
//! ### What a derivation rests on
//!
//! **Its premises, and the activator its plan was compiled from where a node
//! derived or committed it** ([`ein_core::walks::node_activator`]). A plan
//! of a rule with parameters is compiled from an
//! activator, `(rule args…)`, and a firing records it
//! ([`ein_core::Prov::activator`]); the AND-node is the premises with that
//! activator beside them. Root's activators, stated or derived, are the
//! program's, as the plans root compiles from them are its rules, and are
//! followed into no core (Q-M1e2.12). So a core is a set of facts from
//! which, with root's plans, saturation derives the contradiction, and it
//! names the commitment a plan only a node compiles rests on:
//! `examples/ein-bugs/wrong-answers/activator-chain-no-absent.ein`'s dead
//! `{(p a)}` reports `(p a) (seed a)`, and `core-activator-guard-only.ein`'s,
//! whose refuting plan matches no fact, `(p a)`.
//!
//! **And the facts its universals read** ([`ein_core::Prov::universal`]). A
//! `forall` is `(absent (and G (absent B)))`, and one that held
//! rests on facts: for each match of `G`, the fact `B` matched. `total`'s
//! `(false)` rests on the `(not (R a b))` that exclude every `b`, and an
//! elimination's conclusion on the exclusions of every other value. Each is
//! folded in at its **own smallest environment**, beside the premises the
//! search folds over their whole labels (Q-M1e2.15). Folding those over their
//! whole labels too is the exact search, which costs orders of magnitude
//! more on a member of N and is cut by its budget; a core it would give
//! smaller is the promise's second residual, beside the cap's:
//! `examples/ein-bugs/wrong-answers/forall-core-false.ein`'s dead `{(tag s1),
//! (tag s2)}` reports `(item a) (tag s1) (tag s2)`.
//!
//! ### What a contradiction rests on
//!
//! **Its facts, together** ([`contradiction_frontier`]): a pair's `X` and
//! `(not X)` as one AND-node, the smallest union over both sides'
//! environments; and what the firing of a rule that states `:ein
//! inconsistent` rests on, which is what a derivation rests on (above): the
//! premises it matched, the activator a node derived or committed, and the
//! facts its universals read. Of a state that holds `(false)`, that is the
//! `(false)`, which `std.core`'s rule matched. A node's core is the
//! smallest over its contradictions. A core of a pair's positive side alone
//! would leave out the derivation of `(not X)`, where a commitment typically
//! is: `pair-core-root-given.ein`'s dead `{(q a)}` would report
//! `(p a)`, a fact the program states.
//!
//! ### Which rules a core's derivation ends in
//!
//! **The last step of a derivation from the core** ([`Explanation::steps`]):
//! for each fact of the contradiction the core was taken over, a recorded
//! justification whose premises, activator and universal reads the core
//! derives, the first in the store's order where there are several
//! (Q-M1e2.21). Where the core was taken over a firing that ended the state,
//! the step is that firing, its rule the one that refuted, unless the rule is
//! of the module every program gets ([`ein_core::Rule::implicit`]): an
//! explanation names the rules a program wrote or imported, so it reads
//! through such a rule to the steps of what it matched, and a state dead by a
//! `(false)` is refuted by the rule that concluded it.
//! A dead node's `refuted by` is those steps' rules, never the
//! first contradiction's primaries, which need not be a step the core's
//! derivation takes: `examples/ein-bugs/alt-cap-core.ein` prints `refuted by
//! wide` beside the core `(w1 X) (w2 X)`, which only `wide` refutes.
//!
//! ### The algorithm
//!
//! A least fixpoint from the frontier upward. A `source` / `hypothesis` /
//! un-provenanced fact is a leaf, labelled `{{itself}}`; a rule provenance
//! with **no** premises is a synthetic engine writeback whose contract is that
//! provenance walks ground out on it, labelled `{∅}`. An interior node folds
//! each justification's premises' labels by union, then unions in each fact
//! its universals read at that fact's smallest environment, and keeps the
//! subset-minimal result.
//!
//! Labels start empty and only improve, which is what makes a **cyclic**
//! justification graph safe by construction: a fact can never ground itself,
//! because at the moment its own label is still empty it contributes nothing.
//! That is not a corner case — once re-derivations are recorded, `(R a b)` and
//! `(R b a)` justify each other through a symmetric relation's rule
//! (`std.algebra`'s `symmetric`) in any ordinary puzzle.
//!
//! ### Where a rewrite can change the answer
//!
//! **This is the part of the engine where a "cleaner" rewrite most easily
//! changes the answer**, so the loop structure, the wave ordering and the
//! domination test are load-bearing as they stand. Two things carry the
//! determinism:
//!
//! - **`rank`** — assigned once, in `repr` order over the graph's
//!   `(relation_name, args)` id *tuples*, because `repr` is a total order over
//!   any fact, nested ones included, and computing it in an inner loop is far
//!   too slow. Every tie-break and every deterministic iteration goes through
//!   it. An environment **is** a sorted vector of ranks, so `minimise`'s sort
//!   key is the vector itself and union is a merge.
//! - **`recorded_fallback`'s key** is `repr` of the *whole fact*, not of the
//!   id tuple — a different rendering of the same thing, and both are
//!   observable.

use ein_core::{FactId, Kb, ProvId, Terms};
use rustc_hash::{FxHashMap, FxHashSet};

/// Anytime budget for the label search.
///
/// Minimum-cardinality source frontier over an AND/OR graph is worst-case
/// exponential — it is why ATMS labels blow up — so every axis along which the
/// search can explode is capped. Every cap is *sound*: truncation only ever
/// discards environments, and each survivor is still a real set of derivation
/// leaves. A truncated search may miss a smaller one, which is what
/// [`Explanation::exhausted`] reports and [`Explanation::cut`] names, and
/// every surface that prints a core says
/// ([`CoreStatus`]). Every caller that takes a core a surface prints uses the
/// default, and no option changes it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExplanationBudget {
    /// Environments kept per label, smallest first — the dominant knob.
    pub max_environments: usize,
    /// Fixpoint iterations before giving up.
    pub max_rounds: usize,
    /// Drop environments larger than this. `None` keeps all sizes; setting it
    /// turns the search into "is there an explanation of at most N givens?".
    pub max_env_size: Option<usize>,
    /// Refuse to search a premise closure larger than this, returning the
    /// recorded-derivation frontier instead of hanging.
    pub max_facts: usize,
}

impl Default for ExplanationBudget {
    fn default() -> Self {
        ExplanationBudget {
            max_environments: 64,
            max_rounds: 64,
            max_env_size: None,
            max_facts: 20_000,
        }
    }
}

/// How many units of a search's work, a fact its closure reads, a
/// justification folded or an environment made, it does between two reads
/// of the clock (the module's § A search the run's clock stops), a power of
/// two. An environment, made and kept or dropped, costs about half a
/// microsecond where the labels are full, so a search reads the clock every
/// few milliseconds at most, and stops within as long past its deadline; one
/// read is a share of this many units' cost that no measurement sees.
pub const CLOCK_EVERY: u64 = 1 << 12;

/// The run's clock passed while a search ran ([`contradiction_frontier_by`]):
/// no core is taken, and the run is the clock's abort,
/// [`crate::error::SolveError::Deadline`], wherever it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Deadline;

impl std::fmt::Display for Deadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("--max-time reached inside a core's search")
    }
}

/// The deadline a search reads, and the work it has done since its last
/// read.
struct Clock {
    deadline: Option<std::time::Instant>,
    work: u64,
}

impl Clock {
    /// The clock a search reads: `None`, no deadline, and the search runs to
    /// its own end, as it always did.
    fn until(deadline: Option<std::time::Instant>) -> Clock {
        Clock { deadline, work: 0 }
    }

    /// `units` more of the search's work: every [`CLOCK_EVERY`] the clock is
    /// read, and a deadline past ends the search.
    #[inline]
    fn tick(&mut self, units: usize) -> Result<(), Deadline> {
        let Some(d) = self.deadline else {
            return Ok(());
        };
        self.work += units as u64;
        if self.work >= CLOCK_EVERY {
            self.work = 0;
            if std::time::Instant::now() >= d {
                return Err(Deadline);
            }
        }
        Ok(())
    }
}

/// A search that took no deadline ran to its end.
fn unbounded(e: Result<Explanation, Deadline>) -> Explanation {
    e.unwrap_or_else(|_| unreachable!("a search with no deadline is never stopped by one"))
}

/// Which of [`ExplanationBudget`]'s caps a search stopped at: none when it
/// reached its fixpoint inside every one.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Cut {
    /// A label, or a justification's fold, had more than
    /// [`ExplanationBudget::max_environments`] subset-minimal environments,
    /// and the larger ones were dropped.
    pub environments: bool,
    /// The fixpoint was still moving after [`ExplanationBudget::max_rounds`].
    pub rounds: bool,
    /// The premise closure passed [`ExplanationBudget::max_facts`].
    pub facts: bool,
}

impl Cut {
    /// Some cap was hit.
    pub fn any(&self) -> bool {
        self.environments || self.rounds || self.facts
    }
}

/// What a core's search established, carried beside every core a surface
/// prints. It says whether the search was cut, and at which cap, and
/// no more: [`CoreStatus::WithinBudget`] does not say the core is the
/// smallest over every derivation the program records, which the two
/// residuals keep it from being (the module note). A dead set is never
/// printed without a core: every member of `N` is given its core after the
/// walk (Q-M1f.26).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CoreStatus {
    /// The search reached its fixpoint inside every cap of its budget.
    WithinBudget,
    /// The search stopped at a cap: the core is a real frontier, and a
    /// smaller one may exist.
    Cut(Cut),
}

/// How a text surface writes a core that holds no fact (Q-M1f.48).
///
/// A contradiction that needs no fact, `(rule r () :match () :assert
/// (false))`'s, has the empty set for its core: `[]` in every JSON surface,
/// which Q-M1e2.27 tells from `null`, *no core*. A text surface prints a core
/// fact by fact, so the empty set would print nothing, which reads as *no
/// core*; it prints `∅`, the read-out's spelling of the empty set (*∅, root
/// itself*), where a node is dead and its core was taken ([`core_text`]).
pub const EMPTY_CORE: &str = "∅";

/// What the read-out and the trace say beside [`EMPTY_CORE`].
pub const EMPTY_CORE_GLOSS: &str = "the empty set: the refutation rests on no fact";

/// A taken core as a text surface writes it: its facts joined by `sep`, or
/// [`EMPTY_CORE`] where it holds none.
pub fn core_text(core: &[String], sep: &str) -> String {
    if core.is_empty() {
        EMPTY_CORE.to_string()
    } else {
        core.join(sep)
    }
}

impl CoreStatus {
    /// The status of the core `e` names.
    pub fn of(e: &Explanation) -> CoreStatus {
        if e.cut.any() {
            CoreStatus::Cut(e.cut)
        } else {
            CoreStatus::WithinBudget
        }
    }

    /// The word a JSON surface writes: `within-budget`, or `cut-` and the
    /// caps the search stopped at, in [`Cut`]'s order, `cut-environments`,
    /// `cut-rounds`, `cut-facts` or two or three of them joined by `-`
    /// (`cut-environments-rounds`). A reader that asks only whether a core
    /// was cut reads the prefix; the caps' sizes are the default budget's,
    /// which no option changes.
    pub fn word(&self) -> String {
        match self {
            CoreStatus::WithinBudget => "within-budget".to_string(),
            CoreStatus::Cut(c) => {
                let mut w = String::from("cut");
                for (hit, name) in [
                    (c.environments, "-environments"),
                    (c.rounds, "-rounds"),
                    (c.facts, "-facts"),
                ] {
                    if hit {
                        w.push_str(name);
                    }
                }
                w
            }
        }
    }

    /// [`Self::word`] read back, as `ein render region` reads a dump; `None`
    /// for any word it does not write.
    pub fn from_word(word: &str) -> Option<CoreStatus> {
        if word == "within-budget" {
            return Some(CoreStatus::WithinBudget);
        }
        let mut cut = Cut::default();
        for name in word.strip_prefix("cut-")?.split('-') {
            match name {
                "environments" => cut.environments = true,
                "rounds" => cut.rounds = true,
                "facts" => cut.facts = true,
                _ => return None,
            }
        }
        let status = CoreStatus::Cut(cut);
        (status.word() == word).then_some(status)
    }

    /// The caps a cut search stopped at, under the default budget, as a
    /// text surface names them: `64 environments per fact`; `None` inside
    /// its budget.
    pub fn cut_at(&self) -> Option<String> {
        let CoreStatus::Cut(c) = self else {
            return None;
        };
        let b = ExplanationBudget::default();
        let mut at: Vec<String> = Vec::new();
        if c.environments {
            at.push(format!("{} environments per fact", b.max_environments));
        }
        if c.rounds {
            at.push(format!("{} rounds", b.max_rounds));
        }
        if c.facts {
            at.push(format!("{} facts", grouped(b.max_facts)));
        }
        Some(at.join(" and "))
    }

    /// What a text surface prints beside a cut core, `cut at its budget of
    /// 64 environments per fact: a smaller core may exist`; `None` inside its
    /// budget, where a surface prints nothing it did not print before.
    pub fn qualifier(&self) -> Option<String> {
        self.cut_at()
            .map(|at| format!("cut at its budget of {at}: a smaller core may exist"))
    }
}

/// `20 000`: digit groups of three, the read-out's spelling.
fn grouped(n: usize) -> String {
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

/// A minimal-cardinality set of given facts that forces the target.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Explanation {
    /// The explanation — `source` / `hypothesis` / given facts whose joint
    /// truth derives the target through *some* recorded derivation. Empty when
    /// the target is not derivable from the frontier at all.
    pub frontier: Vec<FactId>,
    /// The fact explained; for a contradiction search, the witness that won.
    pub target: Option<FactId>,
    /// True iff no budget was hit anywhere: `frontier` is a true minimum
    /// **over the justifications the store holds** ([`ein_core::Kb::justifications`]).
    /// False means "sound, but possibly not smallest", and [`Self::cut`]
    /// says which cap; a core's [`CoreStatus`] carries both to every surface
    /// that prints it.
    ///
    /// It reports [`ExplanationBudget`] and nothing else. Which derivations
    /// the store holds is decided upstream, by the alternatives cap and the
    /// smallest-closure arrival kept beside it, and this flag does not see
    /// it: where a saving between two facts sits behind a full list,
    /// `exhausted = true` beside a core larger than the smallest over every
    /// derivation (`alt-cap-shared.ein`; see the module note).
    pub exhausted: bool,
    /// Which caps the search stopped at; `exhausted == !cut.any()`.
    pub cut: Cut,
    pub rounds: usize,
    pub facts_considered: usize,
    /// The last step of the derivation [`Self::frontier`] was taken over:
    /// for each fact of the winning group (a pair's `X` then its `(not X)`,
    /// what a rule of the module every program gets matched where its firing
    /// ended the state, or [`explain`]'s target), the
    /// justification through which the frontier derives it, the first the
    /// store records where it derives it through several (`steps_of`), or
    /// the primary of a fact the frontier holds itself (a given's or a
    /// hypothesis's). Past `max_facts`, each fact's primary, the walk's
    /// derivation. Where the group is the firing of a program's own rule that
    /// ended the state, that firing alone. What a dead node's `refuted by`
    /// names (Q-M1e2.21); empty where no target got a label.
    pub steps: Vec<ProvId>,
}

impl Explanation {
    fn empty() -> Explanation {
        Explanation {
            exhausted: true,
            ..Explanation::default()
        }
    }

    pub fn len(&self) -> usize {
        self.frontier.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frontier.is_empty()
    }
}

/// An environment: frontier facts, as **ranks**, sorted ascending.
///
/// Keeping the sorted rank vector *as* the representation makes the sort key
/// the vector itself, union a merge and the subset test a merge — and,
/// because ranks are unique per fact, loses nothing a set would keep.
type Env = Vec<u32>;

/// One justification: what a derivation rests on, as the search folds it.
struct AndNode {
    /// Its premises and the activator a node derived or committed, then the
    /// facts its universals read: `facts[..searched]` and `facts[searched..]`.
    facts: Vec<FactId>,
    /// How many of `facts` are folded over their whole labels; each one after
    /// is folded at its smallest environment (Q-M1e2.15).
    searched: usize,
    /// The justification it is ([`Explanation::steps`]).
    prov: ProvId,
}

/// The AND/OR premise closure of a set of targets.
struct Graph {
    /// `fid → justifications`, each an AND-node.
    just: FxHashMap<FactId, Vec<AndNode>>,
    /// `fid → the facts whose justifications name it` — the propagation edges.
    consumers: FxHashMap<FactId, FxHashSet<FactId>>,
    /// Terminals with a fixed label.
    seed: FxHashMap<FactId, Vec<Env>>,
    /// `fid → a stable integer`, assigned once in `repr` order.
    rank: FxHashMap<FactId, u32>,
    /// `rank → fid`, for turning an environment back into facts.
    by_rank: Vec<FactId>,
    truncated: bool,
}

impl Graph {
    /// `(env.len(), sorted ranks)` — and since an [`Env`] *is* the sorted
    /// ranks, comparing `(len, env)` is that key.
    fn cmp_env(a: &Env, b: &Env) -> std::cmp::Ordering {
        a.len().cmp(&b.len()).then_with(|| a.cmp(b))
    }
}

/// `repr` of the `(relation_name, args)` id **tuple** — what the ranks are
/// ordered by. Not `repr` of the whole fact; that one is
/// [`recorded_fallback`]'s key and they are different strings.
fn fact_id_repr(terms: &Terms, id: FactId) -> String {
    terms.repr_identity(id)
}

/// What a search grounds out on, and which activators it follows.
#[derive(Clone, Copy)]
enum Reach<'a> {
    /// A core's: the engine's givens and a node's commitments, and the
    /// activator a node derived or committed
    /// ([`ein_core::walks::node_activator`]).
    Core,
    /// A support's ([`support_frontier`]): those, every fact of the set,
    /// and the activator of every firing, root's among them.
    Support(&'a FxHashSet<FactId>),
}

fn build_graph(
    kb: &Kb,
    terms: &Terms,
    targets: &[FactId],
    budget: &ExplanationBudget,
    reach: Reach<'_>,
    clock: &mut Clock,
) -> Result<Graph, Deadline> {
    let mut g = Graph {
        just: FxHashMap::default(),
        consumers: FxHashMap::default(),
        seed: FxHashMap::default(),
        rank: FxHashMap::default(),
        by_rank: Vec::new(),
        truncated: false,
    };
    // The targets are pushed in order and popped from the end, so the last
    // target is expanded first. Only `max_facts` truncation can see that, but
    // it can, so the order is fixed.
    let mut stack: Vec<FactId> = Vec::new();
    for &t in targets {
        if !g.just.contains_key(&t) && !g.seed.contains_key(&t) {
            stack.push(t);
            g.just.insert(t, Vec::new());
        }
    }
    while let Some(fid) = stack.pop() {
        clock.tick(1)?;
        if !kb.contains(fid) {
            // A dangling premise id — the case `walk_premises` skips. Ground
            // it out rather than killing the justification that named it, so
            // the closure agrees with the recorded walk.
            g.seed.insert(fid, vec![Vec::new()]);
            g.just.remove(&fid);
            continue;
        }
        let stated = matches!(reach, Reach::Support(stated) if stated.contains(&fid));
        if stated || ein_core::walks::is_frontier(kb, terms, fid) {
            // A placeholder env; the rank it holds is filled in below, once
            // every node has one.
            g.seed.insert(fid, vec![vec![u32::MAX]]);
            g.just.remove(&fid);
            continue;
        }
        let mut ands: Vec<AndNode> = Vec::new();
        for p in kb.justifications(fid) {
            let prov = terms.provs.get(p);
            if prov.kind != ein_core::ProvKind::Rule {
                continue;
            }
            // Its premises, and the activator its plan was compiled from
            // where a node derived or committed it (Q-M1e2.12).
            let mut facts = prov.premises.to_vec();
            let activator = match reach {
                Reach::Core => ein_core::walks::node_activator(kb, prov),
                Reach::Support(_) => prov.activator,
            };
            if let Some(a) = activator
                && !facts.contains(&a)
            {
                facts.push(a);
            }
            let searched = facts.len();
            // Then what its universals read (Q-M1e2.15).
            for &u in prov.universal.iter() {
                if !facts.contains(&u) {
                    facts.push(u);
                }
            }
            if facts.is_empty() {
                ands.clear(); // a synthetic ground-out wins
                break;
            }
            ands.push(AndNode {
                facts,
                searched,
                prov: p,
            });
        }
        if ands.is_empty() {
            g.seed.insert(fid, vec![Vec::new()]);
            g.just.remove(&fid);
            continue;
        }
        for and_node in &ands {
            for &pid in &and_node.facts {
                g.consumers.entry(pid).or_default().insert(fid);
                if !g.just.contains_key(&pid) && !g.seed.contains_key(&pid) {
                    g.just.insert(pid, Vec::new());
                    stack.push(pid);
                }
            }
        }
        g.just.insert(fid, ands);
        if g.just.len() + g.seed.len() > budget.max_facts {
            g.truncated = true;
            break;
        }
    }

    // Every node of `just` and `seed`, sorted by its id tuple's `repr`.
    // determinism-ok: collected and sorted by `repr` before anything reads it.
    let mut nodes: Vec<FactId> = g.just.keys().chain(g.seed.keys()).copied().collect();
    nodes.sort_unstable();
    nodes.dedup();
    let mut keyed: Vec<(String, FactId)> = nodes
        .into_iter()
        .map(|f| (fact_id_repr(terms, f), f))
        .collect();
    keyed.sort();
    g.by_rank = keyed.iter().map(|(_, f)| *f).collect();
    g.rank = keyed
        .iter()
        .enumerate()
        .map(|(i, (_, f))| (*f, i as u32))
        .collect();
    // The self-referencing seeds now know their own rank.
    // determinism-ok: an in-place patch of entries that do not interact.
    for (fid, envs) in g.seed.iter_mut() {
        for e in envs.iter_mut() {
            if e.as_slice() == [u32::MAX] {
                e[0] = g.rank[fid];
            }
        }
    }
    Ok(g)
}

/// Subset-minimal environments, smallest first, capped.
///
/// Discarding a superset is exact — a superset can never be a
/// minimum-cardinality answer, nor lead to one, since union is monotone — so
/// only the cap loses information.
fn minimise(mut envs: Vec<Env>, budget: &ExplanationBudget) -> (Vec<Env>, bool) {
    envs.sort_by(Graph::cmp_env);
    envs.dedup();
    let n = envs.len();
    let mut out: Vec<Env> = Vec::new();
    for e in envs {
        if budget.max_env_size.is_some_and(|m| e.len() > m) {
            continue;
        }
        if out.iter().any(|m| is_subset(m, &e)) {
            continue;
        }
        out.push(e);
        if out.len() >= budget.max_environments {
            let cut = out.len() < n;
            return (out, cut);
        }
    }
    (out, false)
}

/// `a ⊆ b`, both sorted ascending.
fn is_subset(a: &[u32], b: &[u32]) -> bool {
    let mut it = b.iter();
    a.iter().all(|x| it.any(|y| y == x))
}

fn union(a: &[u32], b: &[u32]) -> Env {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

/// Environments of one justification: union one environment per premise.
///
/// A **beam** fold — the working set is re-minimised and capped after each
/// premise — so the cost is bounded by `max_environments` per premise instead
/// of by the full cross-product over all of them.
///
/// `facts[..searched]` are folded so; each fact after them, what the
/// derivation's universals read, contributes its **smallest** environment,
/// one union at the end (Q-M1e2.15). Folding those over their whole labels
/// too is the exact search, which costs orders of magnitude more and is cut by
/// its budget; the saving it would find is not computed.
fn fold(
    and_node: &[FactId],
    searched: usize,
    labels: &FxHashMap<FactId, Vec<Env>>,
    budget: &ExplanationBudget,
    clock: &mut Clock,
) -> Result<(Vec<Env>, bool), Deadline> {
    clock.tick(1)?;
    let mut envs: Vec<Env> = vec![Vec::new()];
    let mut truncated = false;
    let mut read: Env = Vec::new();
    for pid in &and_node[searched..] {
        let Some(plabel) = labels.get(pid).filter(|l| !l.is_empty()) else {
            return Ok((Vec::new(), false)); // not derivable yet
        };
        read = union(&read, &plabel[0]);
    }
    for pid in &and_node[..searched] {
        let Some(plabel) = labels.get(pid).filter(|l| !l.is_empty()) else {
            return Ok((Vec::new(), false)); // premise not derivable yet
        };
        clock.tick(envs.len() * plabel.len())?;
        let mut merged = Vec::with_capacity(envs.len() * plabel.len());
        for e in &envs {
            for pe in plabel {
                merged.push(union(e, pe));
            }
        }
        let (next, cut) = minimise(merged, budget);
        truncated |= cut;
        envs = next;
        if envs.is_empty() {
            return Ok((Vec::new(), truncated));
        }
    }
    if !read.is_empty() {
        clock.tick(envs.len())?;
        let merged: Vec<Env> = envs.iter().map(|e| union(e, &read)).collect();
        let (next, cut) = minimise(merged, budget);
        truncated |= cut;
        envs = next;
    }
    Ok((envs, truncated))
}

/// What [`propagate`] reaches: each fact's label, the rounds it took, and
/// the caps it stopped at.
type Labels = (FxHashMap<FactId, Vec<Env>>, usize, Cut);

/// Least-fixpoint label propagation — `(labels, rounds, cut)`.
fn propagate(g: &Graph, budget: &ExplanationBudget, clock: &mut Clock) -> Result<Labels, Deadline> {
    let mut labels: FxHashMap<FactId, Vec<Env>> = g.seed.clone();
    let mut truncated = false;
    let mut dirty: FxHashSet<FactId> = FxHashSet::default();
    // A fact with no derivable premise simply never acquires a label, so the
    // worklist starts at the terminals' consumers.
    // determinism-ok: accumulated into `dirty`, a set the round sorts by rank.
    for fid in g.seed.keys() {
        if let Some(cs) = g.consumers.get(fid) {
            dirty.extend(cs.iter().copied());
        }
    }
    let mut rounds = 0;
    while !dirty.is_empty() && rounds < budget.max_rounds {
        rounds += 1;
        let wave = std::mem::take(&mut dirty);
        // determinism-ok: sorted by rank right here, which is what makes the
        // wave order a function of the graph rather than of the hash set.
        let mut wave: Vec<FactId> = wave.into_iter().collect();
        wave.sort_by_key(|k| g.rank.get(k).copied().unwrap_or(u32::MAX));
        for fid in wave {
            let Some(ands) = g.just.get(&fid).filter(|a| !a.is_empty()) else {
                continue;
            };
            let mut candidates: Vec<Env> = Vec::new();
            for and_node in ands {
                let (envs, cut) = fold(&and_node.facts, and_node.searched, &labels, budget, clock)?;
                truncated |= cut;
                candidates.extend(envs);
            }
            if candidates.is_empty() {
                continue;
            }
            clock.tick(candidates.len())?;
            let (new, cut) = minimise(candidates, budget);
            truncated |= cut;
            if labels.get(&fid).map(|l| l.as_slice()) != Some(new.as_slice()) {
                labels.insert(fid, new);
                if let Some(cs) = g.consumers.get(&fid) {
                    dirty.extend(cs.iter().copied());
                }
            }
        }
    }
    let cut = Cut {
        environments: truncated,
        rounds: !dirty.is_empty(),
        facts: g.truncated,
    };
    Ok((labels, rounds, cut))
}

/// The smallest recorded-derivation frontier that forces *some* target.
///
/// Each target is explained independently — they are alternatives, not a
/// conjunction, because a caller asking "why is this KB contradictory?" wants
/// the single most legible witness rather than the union over all of them —
/// and the smallest explanation across targets wins. Ties break on the rank
/// order, so the result depends on neither set iteration nor the order in
/// which the rules happened to fire.
pub fn explain(
    kb: &Kb,
    terms: &Terms,
    targets: &[FactId],
    budget: &ExplanationBudget,
) -> Explanation {
    let groups: Vec<Group> = targets.iter().map(|&t| Group::of(vec![t])).collect();
    unbounded(explain_groups(
        kb,
        terms,
        &groups,
        budget,
        Reach::Core,
        &mut Clock::until(None),
    ))
}

/// What one contradiction rests on, as the search explains it: a
/// conjunction of facts explained together, the first its target and its
/// tie-break.
struct Group {
    /// `facts[..searched]` are folded over their whole labels, and each fact
    /// after them at its smallest environment, as a justification's are
    /// ([`AndNode`]).
    facts: Vec<FactId>,
    searched: usize,
    /// The firing that ended the state, where the group is one's
    /// ([`crate::contradiction::Contradiction::Ended`]).
    ended: Option<ProvId>,
}

impl Group {
    /// Facts explained together, each over its whole label.
    fn of(facts: Vec<FactId>) -> Group {
        Group {
            searched: facts.len(),
            facts,
            ended: None,
        }
    }

    /// A contradiction's: a pair's two sides, or what the
    /// firing that ended the state rests on, which is what a derivation
    /// rests on (the module's § What a derivation rests on): its premises,
    /// the activator `reach` follows, and after them the facts its
    /// universals read.
    fn of_contradiction(
        kb: &Kb,
        terms: &Terms,
        c: &crate::contradiction::Contradiction,
        reach: Reach<'_>,
    ) -> Group {
        let Some(p) = c.ended() else {
            return Group::of(c.sides(terms));
        };
        let prov = terms.provs.get(p);
        let mut facts = prov.premises.to_vec();
        let activator = match reach {
            Reach::Core => ein_core::walks::node_activator(kb, prov),
            Reach::Support(_) => prov.activator,
        };
        if let Some(a) = activator
            && !facts.contains(&a)
        {
            facts.push(a);
        }
        let searched = facts.len();
        for &u in prov.universal.iter() {
            if !facts.contains(&u) {
                facts.push(u);
            }
        }
        Group {
            facts,
            searched,
            ended: Some(p),
        }
    }
}

fn groups_of(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
    reach: Reach<'_>,
) -> Vec<Group> {
    contradictions
        .iter()
        .map(|c| Group::of_contradiction(kb, terms, c, reach))
        .collect()
}

/// The smallest explanation of any of `contradictions` — **a contradiction's
/// facts explained together**: a pair's `X` and its `(not X)` as one
/// AND-node, the smallest union over both sides' environments, and what the
/// firing that ended a state rests on (`Group::of_contradiction`).
/// The smallest across contradictions wins, as in [`explain`], each one's
/// [`crate::contradiction::Contradiction::witness`] its tie-break and the
/// [`Explanation::target`].
///
/// Explained by its witness alone, a pair's positive side, the core
/// would leave out the derivation of `(not X)`, where a commitment typically
/// is: `examples/ein-bugs/wrong-answers/pair-core-root-given.ein`'s dead
/// `{(q a)}` would report `(p a)`, a fact root states. The sides are taken
/// *jointly*: each side at its smallest, apart, is never smaller.
pub fn contradiction_frontier(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
    budget: &ExplanationBudget,
) -> Explanation {
    unbounded(contradiction_frontier_by(
        kb,
        terms,
        contradictions,
        budget,
        None,
    ))
}

/// [`contradiction_frontier`] under a run's clock (the module's § A search
/// the run's clock stops): the same search, read against `deadline` every
/// [`CLOCK_EVERY`] units of its work, and [`Deadline`] past it. `None` is
/// [`contradiction_frontier`].
pub fn contradiction_frontier_by(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
    budget: &ExplanationBudget,
    deadline: Option<std::time::Instant>,
) -> Result<Explanation, Deadline> {
    let groups = groups_of(kb, terms, contradictions, Reach::Core);
    explain_groups(
        kb,
        terms,
        &groups,
        budget,
        Reach::Core,
        &mut Clock::until(deadline),
    )
}

/// **The support of a death**: [`contradiction_frontier`]'s search, grounded
/// out on what the program **states** and following every activator.
///
/// A core stops at root's activators, stated or derived: they are the
/// program's, as the plans root compiles from them are its rules, and a core
/// names what a node committed. A support asks the other question, which of
/// the program's own statements a death rests on, so here the activator of
/// every firing is an AND-node's fact beside its premises, root's among
/// them, and a fact of `stated`, the facts the loaded program held before any
/// saturation, is a leaf whatever its record says (a fact written with
/// `:rule` is stated all the same). The frontier is then the stated facts
/// and the commitments from which, with the program's rules and nothing else
/// it states, the recorded derivation reaches the contradiction.
///
/// It is an explanation as a core is one: sufficient, the smallest over the
/// derivations the store holds within the budget, not the least over every
/// derivation, and it may vary with rule order. An `absent` a derivation
/// passed is recorded and not followed, here as in a core: a support says
/// which statements a death needs, not which it needs to be missing. Past
/// [`ExplanationBudget::max_facts`] no support is read ([`Cut::facts`], an
/// empty frontier): the walk a core degrades to follows no root activator.
pub fn support_frontier(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
    budget: &ExplanationBudget,
    stated: &FxHashSet<FactId>,
) -> Explanation {
    let groups = groups_of(kb, terms, contradictions, Reach::Support(stated));
    unbounded(explain_groups(
        kb,
        terms,
        &groups,
        budget,
        Reach::Support(stated),
        &mut Clock::until(None),
    ))
}

/// [`explain`] over **groups**, each a conjunction of facts explained
/// together, the first fact of each its target and its tie-break.
fn explain_groups(
    kb: &Kb,
    terms: &Terms,
    groups: &[Group],
    budget: &ExplanationBudget,
    reach: Reach<'_>,
    clock: &mut Clock,
) -> Result<Explanation, Deadline> {
    if groups.is_empty() {
        return Ok(Explanation::empty());
    }
    // Every group's other facts, then the targets, in group order: the
    // closure is walked from the end, so the targets are expanded first, as
    // they were when each group was its target alone. Only `max_facts`
    // truncation can see the order. A group of no fact, a firing that ended
    // the state and matched none, has neither.
    let mut roots: Vec<FactId> = groups
        .iter()
        .flat_map(|g| g.facts.iter().skip(1).copied())
        .collect();
    roots.extend(groups.iter().filter_map(|g| g.facts.first().copied()));
    let g = build_graph(kb, terms, &roots, budget, reach, clock)?;
    let (labels, rounds, mut cut) = propagate(&g, budget, clock)?;

    let mut best: Option<((usize, Env, u32), usize, Env)> = None;
    for (gi, group) in groups.iter().enumerate() {
        let env = match group.facts.as_slice() {
            [one] if group.searched == 1 => {
                let Some(label) = labels.get(one).filter(|l| !l.is_empty()) else {
                    continue;
                };
                label[0].clone() // smallest, already sorted
            }
            facts => {
                let (envs, folded) = fold(facts, group.searched, &labels, budget, clock)?;
                cut.environments |= folded;
                let Some(env) = envs.into_iter().next() else {
                    continue;
                };
                env
            }
        };
        let key = (
            env.len(),
            env.clone(),
            group
                .facts
                .first()
                .and_then(|t| g.rank.get(t).copied())
                .unwrap_or(u32::MAX),
        );
        if best.as_ref().is_none_or(|(b, _, _)| key < *b) {
            best = Some((key, gi, env));
        }
    }
    let considered = g.just.len() + g.seed.len();
    let Some((_, gi, env)) = best else {
        if g.truncated && matches!(reach, Reach::Core) {
            // The closure blew the `max_facts` valve before any target got a
            // label. Degrade to the pre-S1.21.7 answer rather than returning
            // nothing or, worse, passing derived facts off as givens. A
            // support has no such walk to degrade to and says it was cut.
            return Ok(fallback_groups(kb, terms, groups, rounds, considered));
        }
        return Ok(Explanation {
            exhausted: !cut.any(),
            cut,
            rounds,
            facts_considered: considered,
            ..Explanation::default()
        });
    };
    let frontier: Vec<FactId> = env
        .iter()
        .map(|&r| g.by_rank[r as usize])
        .filter(|&f| kb.contains(f))
        .collect();
    let steps = match groups[gi].ended {
        // The firing that ended the state is the derivation's last step,
        // unless its rule is of the module every program gets, which an
        // explanation reads through to the steps of what it matched.
        Some(p) if !crate::contradiction::ended_by_an_implicit_rule(kb, terms, p) => vec![p],
        _ => steps_of(kb, &g, &groups[gi].facts, &env),
    };
    Ok(Explanation {
        frontier,
        target: groups[gi].facts.first().copied(),
        exhausted: !cut.any(),
        cut,
        rounds,
        facts_considered: considered,
        steps,
    })
}

/// The facts the environment `env` derives through the graph's recorded
/// justifications: its own, every fact whose label is `{∅}` (a synthetic
/// ground-out), and, to a least fixpoint, every fact one of whose
/// justifications rests on facts already in the set — premises, a node's
/// activator and what its universals read alike.
fn derived_from(g: &Graph, env: &Env) -> FxHashSet<FactId> {
    let mut d: FxHashSet<FactId> = env.iter().map(|&r| g.by_rank[r as usize]).collect();
    // determinism-ok: builds a set; the fixpoint does not depend on the order.
    for (f, envs) in &g.seed {
        if envs.iter().any(|e| e.is_empty()) {
            d.insert(*f);
        }
    }
    // determinism-ok: a worklist; the set it closes is the same in any order.
    let mut work: Vec<FactId> = d.iter().copied().collect();
    while let Some(x) = work.pop() {
        let Some(cs) = g.consumers.get(&x) else {
            continue;
        };
        // determinism-ok: membership only; the fixpoint is order-free.
        for &c in cs {
            if d.contains(&c) {
                continue;
            }
            let derivable = g
                .just
                .get(&c)
                .is_some_and(|ands| ands.iter().any(|a| a.facts.iter().all(|f| d.contains(f))));
            if derivable {
                d.insert(c);
                work.push(c);
            }
        }
    }
    d
}

/// [`Explanation::steps`]: for each fact of `group`, the justification
/// through which `env` derives it, **one per fact**: the **first recorded
/// derivation the core supports**, in the store's order
/// ([`ein_core::Kb::justifications`]): the fact's primary where `env`
/// derives it, else the first alternative the store keeps that it derives
/// (fewest premises first), else the smallest-closure arrival. A fact the
/// frontier holds itself, or a ground-out, is its own primary.
///
/// Q-M1e2.21. A core often derives a fact of its contradiction through two
/// rules, and the store's order picks one: arrival in the agenda's order and
/// in commitment order, never the interner. So
/// `:record-alternative-justifications false`, which keeps each fact's
/// primary alone, moves the rule only where it moves the core: where the core
/// is the same, the primaries are a derivation from it. The accepted cost:
/// two programs whose rules are written in another order can name two rules
/// beside one core (Q-M1f.6: a core is an explanation).
fn steps_of(kb: &Kb, g: &Graph, group: &[FactId], env: &Env) -> Vec<ProvId> {
    let d = derived_from(g, env);
    let mut out = Vec::with_capacity(group.len());
    for &f in group {
        let Some(ands) = g.just.get(&f).filter(|a| !a.is_empty()) else {
            // A fact the frontier holds, or a ground-out: its own primary.
            out.extend(kb.primary(f));
            continue;
        };
        // `ands` is the store's order: `build_graph` reads
        // `Kb::justifications`, the primary first.
        let first = ands
            .iter()
            .find(|a| a.facts.iter().all(|x| d.contains(x)))
            .map(|a| a.prov);
        // `env` derives every fact of its group (each label environment was
        // made from recorded steps), so `first` is `Some`; the primary is the
        // floor regardless.
        out.extend(first.or_else(|| kb.primary(f)));
    }
    out
}

/// The pre-S1.21.7 answer: the smallest single-target recorded-**primary**
/// frontier.
///
/// `pub` because `shape::explain_shape` calls it directly: it is only reachable
/// through [`explain`] when the closure blows `max_facts` *and* no target gets
/// a label, and even then its tie-break only decides when two targets tie on
/// core size — which no corpus input separates from plain first-wins.
///
/// Graceful degradation when the AND/OR search cannot run. Always sound — it
/// is a real derivation's leaves — just order-dependent, which is what
/// `exhausted = false` is telling the caller.
pub fn recorded_fallback(
    kb: &Kb,
    terms: &Terms,
    targets: &[FactId],
    rounds: usize,
    considered: usize,
) -> Explanation {
    let groups: Vec<Group> = targets.iter().map(|&t| Group::of(vec![t])).collect();
    fallback_groups(kb, terms, &groups, rounds, considered)
}

/// [`recorded_fallback`] for `contradictions`, each one's facts walked
/// together, a pair's two sides: what [`contradiction_frontier`]
/// degrades to past `max_facts`. `pub` for `shape::explain_shape`, as
/// [`recorded_fallback`] is.
pub fn recorded_contradiction_fallback(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
) -> Explanation {
    let groups = groups_of(kb, terms, contradictions, Reach::Core);
    fallback_groups(kb, terms, &groups, 0, 0)
}

fn fallback_groups(
    kb: &Kb,
    terms: &Terms,
    groups: &[Group],
    rounds: usize,
    considered: usize,
) -> Explanation {
    let mut best: Option<((usize, String), usize, Vec<FactId>)> = None;
    for (gi, group) in groups.iter().enumerate() {
        let mut core = ein_core::walks::unsat_core(
            kb,
            terms,
            &group.facts,
            ein_core::walks::Justifications::Primary,
        );
        core.sort_unstable();
        core.dedup();
        // No frontier is no explanation, but of a firing that ended the
        // state and matched no fact, whose core is the empty set.
        if core.is_empty() && !(group.facts.is_empty() && group.ended.is_some()) {
            continue;
        }
        // The sorted whole-fact `repr`s joined by a space — not the id
        // tuple's.
        let mut reprs: Vec<String> = core.iter().map(|&f| terms.repr_fact_text(f)).collect();
        reprs.sort();
        let key = (core.len(), reprs.join(" "));
        if best.as_ref().is_none_or(|(b, _, _)| key < *b) {
            best = Some((key, gi, core));
        }
    }
    // Reached past `max_facts` alone.
    let cut = Cut {
        facts: true,
        ..Cut::default()
    };
    match best {
        None => Explanation {
            exhausted: false,
            cut,
            rounds,
            facts_considered: considered,
            ..Explanation::default()
        },
        Some((_, gi, frontier)) => Explanation {
            frontier,
            target: groups[gi].facts.first().copied(),
            exhausted: false,
            cut,
            rounds,
            facts_considered: considered,
            // The walk's derivation is each fact's primary, after the firing
            // that ended the state where its rule is the program's.
            steps: match groups[gi].ended {
                Some(p) if !crate::contradiction::ended_by_an_implicit_rule(kb, terms, p) => {
                    vec![p]
                }
                _ => groups[gi]
                    .facts
                    .iter()
                    .filter_map(|&f| kb.primary(f))
                    .collect(),
            },
        },
    }
}

/// The smallest explanation of any contradiction in `kb`.
///
/// `None` explains every contradiction the detector finds, each one's facts
/// together ([`contradiction_frontier`]). `Some(facts)` explains each of
/// `facts` alone, as
/// [`explain`] does; a consistent KB yields an empty [`Explanation`].
pub fn minimal_contradiction_frontier(
    kb: &Kb,
    terms: &Terms,
    witnesses: Option<&[FactId]>,
    budget: &ExplanationBudget,
) -> Explanation {
    match witnesses {
        Some(w) => explain(kb, terms, w, budget),
        None => contradiction_frontier(kb, terms, &crate::contradiction::detect(kb, terms), budget),
    }
}

/// The smallest source frontier that forces a contradiction.
///
/// Sound by construction and NAF-safe: the result is a real derivation's
/// leaves, never a re-saturated guess. It is **not** a subset-minimal MUS — no
/// proper subset is checked for satisfiability, so a smaller logically
/// sufficient set may exist that no recorded derivation exhibits.
///
/// Why not the union: `unsat_core` unions the frontier of *every* witness, and
/// when one cause propagates it fans out into many. `zebra2-bad`'s single
/// injected fact is the worked case — a large witness set, each witness a
/// complete contradiction on its own with a handful of givens behind it, and
/// the smallest of them exactly the culprit. The numbers are
/// `explain_semantics::the_injected_contradiction_fans_out_and_the_union_overstates_it`'s,
/// so that one place states them.
pub fn smallest_contradiction_frontier(
    kb: &Kb,
    terms: &Terms,
    witnesses: Option<&[FactId]>,
) -> Vec<FactId> {
    minimal_contradiction_frontier(kb, terms, witnesses, &ExplanationBudget::default()).frontier
}
