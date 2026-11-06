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
//! ([`ein_core::Kb::best`], M1e2 S1e2.2.11, Q-M1e.15's (3)). The promise is
//! the smallest frontier over **every derivation the program records**, and
//! the third is what keeps a full list from hiding the derivation this search
//! would choose: premise count is local and frontier size transitive, so a
//! one-premise step over a deep chain is retained ahead of a two-premise step
//! over givens. `examples/ein-bugs/alt-cap-core.ein` and its `-reordered`
//! twin, one `:priority` apart, reported a 3-fact core against a 2-fact one
//! until S1e2.2.11 and report the same 2-fact core since.
//!
//! Two limits, each a ruling of the user's (2026-11-04, Q-M1e2.11):
//!
//! - **A saving between two facts, behind a full list, is not offered.** The
//!   derivation a target needs can be its own fact's worst, and no list of
//!   fixed length keeps every such one:
//!   `examples/ein-bugs/wrong-answers/alt-cap-shared.ein` reports 3 facts
//!   where every derivation gives 2. It is `W10/alt-cap-shared`, open and
//!   deferred to F22, which re-derives what a full list refused when this
//!   search reaches it. [`Explanation::exhausted`] does not see it.
//! - **`:record-alternative-justifications false` records no alternative**,
//!   so the search is offered each fact's first derivation alone and the core
//!   can be another: the one config flag that may change which core is
//!   named, and nothing else (`configuration.md` § 2.5).
//!
//! It is not a soundness question either way: a larger frontier is still a
//! real explanation.
//!
//! ### A search its budget cut says so
//!
//! **On every surface that prints its core** ([`CoreStatus`], M1e2
//! S1e2.2.23). The search keeps 64 environments per fact and runs 64 rounds
//! ([`ExplanationBudget`]); past either it stops with a real frontier that
//! may not be the smallest, and [`Explanation::cut`] names the cap. Until
//! S1e2.2.23 the two callers that take a core a surface prints,
//! `region::node_in_region` and `region::Nodes::refutation`, kept the
//! frontier and dropped the rest, so a cut core was printed as the core
//! (W23): `examples/ein-bugs/wrong-answers/core-search-cut.ein`'s `(A k)` has
//! 256 environments, and its root core was ten facts where eight suffice. The
//! user ruled on 2026-11-05 (Q-M1e2.16) that the search keeps its budget and a
//! cut core says so; the exact core past the budget is F22's. The corpus cuts
//! 19 of the 5 442 cores `--dump-answer` reports for N, `zebra.ein`'s at the
//! rounds and the `zebra2` family's at the environments, and a budget sixteen
//! times larger gives each the same core (S1e2.2.23 T1, 2026-11-05).
//!
//! ### What a derivation rests on
//!
//! **Its premises, and the activator its plan was compiled from where a node
//! derived or committed it** ([`ein_core::walks::node_activator`], M1e2
//! S1e2.2.19). A plan of a rule with parameters is compiled from an
//! activator, `(rule args…)`, and a firing records it
//! ([`ein_core::Prov::activator`]); the AND-node is the premises with that
//! activator beside them. Root's activators, stated or derived, are the
//! program's, as the plans root compiles from them are its rules, and are
//! followed into no core: the user's ruling of 2026-11-04 on Q-M1e2.12, (c).
//! So a core is a set of facts from which, with root's plans, saturation
//! derives the contradiction, and it names the commitment a plan only a node
//! compiles rests on. Until S1e2.2.19 no derivation recorded its activator,
//! and such a commitment was in no core (W19):
//! `examples/ein-bugs/wrong-answers/forbid-chain-no-absent.ein`'s dead
//! `{(p a)}` reported `(seed a)`, a fact root holds, and
//! `core-activator-guard-only.ein`'s, whose refuting plan matches no fact,
//! reported nothing; they report `(p a) (seed a)` and `(p a)`.
//!
//! **And the facts its universals read** ([`ein_core::Prov::universal`], M1e2
//! S1e2.2.21). A `forall` is `(absent (and G (absent B)))`, and one that held
//! rests on facts: for each match of `G`, the fact `B` matched. `total`'s
//! `(false)` rests on the `(not (R a b))` that exclude every `b`, and an
//! elimination's conclusion on the exclusions of every other value. Each is
//! folded in at its **own smallest environment**, beside the premises the
//! search folds over their whole labels: the user's ruling of 2026-11-05 on
//! Q-M1e2.15, (b). Folding those over their whole labels too is the exact
//! search, and it cost ×2 500 a member of N on `zebra2-hints` (3 ms → 7.5 s,
//! cut by its budget); a core it would give smaller (13 of that program's 73
//! N cores, S1e2.2.21 T1) is the promise's second residual, F22's beside the
//! cap's. Until S1e2.2.21 no record named them:
//! `examples/ein-bugs/wrong-answers/forall-core-false.ein`'s dead `{(tag s1),
//! (tag s2)}` reported `(item a)`, root's, and reports `(item a) (tag s1)
//! (tag s2)`.
//!
//! ### What a contradiction rests on
//!
//! **Its facts, together** ([`contradiction_frontier`], M1e2 S1e2.2.21): a
//! direct ⊥'s `(false …)` fact, and a pair's `X` and `(not X)` as one
//! AND-node, the smallest union over both sides' environments. A node's core
//! is the smallest over its contradictions. Until S1e2.2.21 each contradiction
//! was explained by its witness alone, a pair's positive side, and the core
//! left out the derivation of `(not X)`, where a commitment typically is
//! (W21): `pair-core-root-given.ein`'s dead `{(q a)}` reported `(p a)`, a
//! fact the program states, and `zebra2.ein`'s N 201 of 366 such cores.
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
//! `(R b a)` justify each other through the symmetric mirror in any ordinary
//! puzzle.
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

use ein_core::repr::{ReprValue, repr};
use ein_core::{FactId, Kb, Terms};
use rustc_hash::{FxHashMap, FxHashSet};

/// Anytime budget for the label search.
///
/// Minimum-cardinality source frontier over an AND/OR graph is worst-case
/// exponential — it is why ATMS labels blow up — so every axis along which the
/// search can explode is capped. Every cap is *sound*: truncation only ever
/// discards environments, and each survivor is still a real set of derivation
/// leaves. A truncated search may miss a smaller one, which is what
/// [`Explanation::exhausted`] reports and [`Explanation::cut`] names, and
/// since M1e2 S1e2.2.23 every surface that prints a core says
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

/// Which of [`ExplanationBudget`]'s caps a search stopped at: none when it
/// reached its fixpoint inside every one (M1e2 S1e2.2.23, W23).
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
/// prints (M1e2 S1e2.2.23, W23). It says whether the search was cut, and at
/// which cap, and no more: [`CoreStatus::WithinBudget`] does not say the core
/// is the smallest over every derivation the program records, which F22's
/// residuals keep it from being (the module note). Where a dead set is printed
/// with no core, it says why ([`CoreStatus::NotKept`], M1e2 S1e2.3.9).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CoreStatus {
    /// The search reached its fixpoint inside every cap of its budget.
    WithinBudget,
    /// The search stopped at a cap: the core is a real frontier, and a
    /// smaller one may exist.
    Cut(Cut),
    /// The run kept no core for this dead set: a describing run past
    /// `--region-limit` keeps each node it met as its two bits, so a member
    /// of `N` it met is printed with no core, and with this status where the
    /// core would be (M1e2 S1e2.3.9, P1e2.3's N6). Until then it was printed
    /// with an empty core and no status, which reads as *refuted by nothing*.
    NotKept,
}

/// What a text surface prints in place of a core the run did not keep
/// ([`CoreStatus::NotKept`]).
pub const NOT_KEPT: &str = "not kept: the run stopped at `--region-limit`";

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
    /// (`cut-environments-rounds`), or `not-kept` beside a `null` core
    /// ([`CoreStatus::NotKept`]). A reader that asks only whether a core
    /// was cut reads the prefix; the caps' sizes are the default budget's,
    /// which no option changes.
    pub fn word(&self) -> String {
        match self {
            CoreStatus::WithinBudget => "within-budget".to_string(),
            CoreStatus::NotKept => "not-kept".to_string(),
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
        if word == "not-kept" {
            return Some(CoreStatus::NotKept);
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

    /// What a text surface prints in place of a core the run did not keep,
    /// [`NOT_KEPT`]; `None` for a core a search took.
    pub fn not_kept(&self) -> Option<&'static str> {
        matches!(self, CoreStatus::NotKept).then_some(NOT_KEPT)
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
    /// that prints it (M1e2 S1e2.2.23).
    ///
    /// It reports [`ExplanationBudget`] and nothing else. Which derivations
    /// the store holds is decided upstream, by the alternatives cap and the
    /// smallest-closure arrival kept beside it (M1e2 S1e2.2.11), and this
    /// flag does not see it: where a saving between two facts sits behind a
    /// full list, `exhausted = true` beside a core larger than the smallest
    /// over every derivation (`W10/alt-cap-shared`, F22's; see the module
    /// note).
    pub exhausted: bool,
    /// Which caps the search stopped at; `exhausted == !cut.any()`.
    pub cut: Cut,
    pub rounds: usize,
    pub facts_considered: usize,
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
    /// is folded at its smallest environment (M1e2 S1e2.2.21, Q-M1e2.15 (b)).
    searched: usize,
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
    let (rel, args) = terms.facts.get(id);
    repr(&ReprValue::Tuple(vec![
        ReprValue::Str(terms.sym(rel).to_string()),
        ReprValue::Tuple(args.iter().map(|a| terms.repr_value(*a)).collect()),
    ]))
}

fn build_graph(kb: &Kb, terms: &Terms, targets: &[FactId], budget: &ExplanationBudget) -> Graph {
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
        if !kb.contains(fid) {
            // A dangling premise id — the case `walk_premises` skips. Ground
            // it out rather than killing the justification that named it, so
            // the closure agrees with the recorded walk.
            g.seed.insert(fid, vec![Vec::new()]);
            g.just.remove(&fid);
            continue;
        }
        if ein_core::walks::is_frontier(kb, terms, fid) {
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
            // where a node derived or committed it (M1e2 S1e2.2.19).
            let mut facts = prov.premises.to_vec();
            if let Some(a) = ein_core::walks::node_activator(kb, prov)
                && !facts.contains(&a)
            {
                facts.push(a);
            }
            let searched = facts.len();
            // Then what its universals read (M1e2 S1e2.2.21).
            for &u in prov.universal.iter() {
                if !facts.contains(&u) {
                    facts.push(u);
                }
            }
            if facts.is_empty() {
                ands.clear(); // a synthetic ground-out wins
                break;
            }
            ands.push(AndNode { facts, searched });
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
    g
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
/// one union at the end (M1e2 S1e2.2.21, the user's ruling of 2026-11-05 on
/// Q-M1e2.15, (b)). Folding those over their whole labels too is the exact
/// search, and it cost ×2 500 a member of N on `zebra2-hints` (3 ms → 7.5 s,
/// cut by its budget; S1e2.2.21 T1); the saving it would find is F22's.
fn fold(
    and_node: &[FactId],
    searched: usize,
    labels: &FxHashMap<FactId, Vec<Env>>,
    budget: &ExplanationBudget,
) -> (Vec<Env>, bool) {
    let mut envs: Vec<Env> = vec![Vec::new()];
    let mut truncated = false;
    let mut read: Env = Vec::new();
    for pid in &and_node[searched..] {
        let Some(plabel) = labels.get(pid).filter(|l| !l.is_empty()) else {
            return (Vec::new(), false); // not derivable yet
        };
        read = union(&read, &plabel[0]);
    }
    for pid in &and_node[..searched] {
        let Some(plabel) = labels.get(pid).filter(|l| !l.is_empty()) else {
            return (Vec::new(), false); // premise not derivable yet
        };
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
            return (Vec::new(), truncated);
        }
    }
    if !read.is_empty() {
        let merged: Vec<Env> = envs.iter().map(|e| union(e, &read)).collect();
        let (next, cut) = minimise(merged, budget);
        truncated |= cut;
        envs = next;
    }
    (envs, truncated)
}

/// Least-fixpoint label propagation — `(labels, rounds, cut)`.
fn propagate(g: &Graph, budget: &ExplanationBudget) -> (FxHashMap<FactId, Vec<Env>>, usize, Cut) {
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
                let (envs, cut) = fold(&and_node.facts, and_node.searched, &labels, budget);
                truncated |= cut;
                candidates.extend(envs);
            }
            if candidates.is_empty() {
                continue;
            }
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
    (labels, rounds, cut)
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
    let groups: Vec<Vec<FactId>> = targets.iter().map(|&t| vec![t]).collect();
    explain_groups(kb, terms, &groups, budget)
}

/// The smallest explanation of any of `contradictions` — **a contradiction's
/// facts explained together**: a pair's `X` and its `(not X)` as one
/// AND-node, the smallest union over both sides' environments, and a direct
/// ⊥'s `(false …)` fact alone ([`crate::contradiction::Contradiction::sides`]).
/// The smallest across contradictions wins, as in [`explain`], each one's
/// [`crate::contradiction::Contradiction::witness`] its tie-break and the
/// [`Explanation::target`].
///
/// M1e2 S1e2.2.21 (W21). Until then each contradiction was explained by its
/// witness alone, a pair's positive side, and the core left out the
/// derivation of `(not X)`, where a commitment typically is:
/// `examples/ein-bugs/wrong-answers/pair-core-root-given.ein`'s dead
/// `{(q a)}` reported `(p a)`, a fact root states, and `zebra2.ein`'s N 201
/// of 366 such cores. *Jointly* is the stage's default 2: the stage file's
/// *each side at its smallest* differs on four corpus programs and is never
/// smaller.
pub fn contradiction_frontier(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
    budget: &ExplanationBudget,
) -> Explanation {
    let groups: Vec<Vec<FactId>> = contradictions.iter().map(|c| c.sides()).collect();
    explain_groups(kb, terms, &groups, budget)
}

/// [`explain`] over **groups**, each a conjunction of facts explained
/// together, the first fact of each its target and its tie-break.
fn explain_groups(
    kb: &Kb,
    terms: &Terms,
    groups: &[Vec<FactId>],
    budget: &ExplanationBudget,
) -> Explanation {
    if groups.is_empty() {
        return Explanation::empty();
    }
    // Every group's other facts, then the targets, in group order: the
    // closure is walked from the end, so the targets are expanded first, as
    // they were when each group was its target alone. Only `max_facts`
    // truncation can see the order.
    let mut roots: Vec<FactId> = groups.iter().flat_map(|g| g[1..].iter().copied()).collect();
    roots.extend(groups.iter().map(|g| g[0]));
    let g = build_graph(kb, terms, &roots, budget);
    let (labels, rounds, mut cut) = propagate(&g, budget);

    let mut best: Option<((usize, Env, u32), FactId, Env)> = None;
    for group in groups {
        let t = group[0];
        let env = if let [one] = group.as_slice() {
            let Some(label) = labels.get(one).filter(|l| !l.is_empty()) else {
                continue;
            };
            label[0].clone() // smallest, already sorted
        } else {
            let (envs, folded) = fold(group, group.len(), &labels, budget);
            cut.environments |= folded;
            let Some(env) = envs.into_iter().next() else {
                continue;
            };
            env
        };
        let key = (
            env.len(),
            env.clone(),
            g.rank.get(&t).copied().unwrap_or(u32::MAX),
        );
        if best.as_ref().is_none_or(|(b, _, _)| key < *b) {
            best = Some((key, t, env));
        }
    }
    let considered = g.just.len() + g.seed.len();
    let Some((_, target, env)) = best else {
        if g.truncated {
            // The closure blew the `max_facts` valve before any target got a
            // label. Degrade to the pre-S1.21.7 answer rather than returning
            // nothing or, worse, passing derived facts off as givens.
            return fallback_groups(kb, terms, groups, rounds, considered);
        }
        return Explanation {
            exhausted: !cut.any(),
            cut,
            rounds,
            facts_considered: considered,
            ..Explanation::default()
        };
    };
    let frontier: Vec<FactId> = env
        .iter()
        .map(|&r| g.by_rank[r as usize])
        .filter(|&f| kb.contains(f))
        .collect();
    Explanation {
        frontier,
        target: Some(target),
        exhausted: !cut.any(),
        cut,
        rounds,
        facts_considered: considered,
    }
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
    let groups: Vec<Vec<FactId>> = targets.iter().map(|&t| vec![t]).collect();
    fallback_groups(kb, terms, &groups, rounds, considered)
}

/// [`recorded_fallback`] for `contradictions`, each one's facts walked
/// together, a pair's two sides (M1e2 S1e2.2.21): what [`contradiction_frontier`]
/// degrades to past `max_facts`. `pub` for `shape::explain_shape`, as
/// [`recorded_fallback`] is.
pub fn recorded_contradiction_fallback(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[crate::contradiction::Contradiction],
) -> Explanation {
    let groups: Vec<Vec<FactId>> = contradictions.iter().map(|c| c.sides()).collect();
    fallback_groups(kb, terms, &groups, 0, 0)
}

fn fallback_groups(
    kb: &Kb,
    terms: &Terms,
    groups: &[Vec<FactId>],
    rounds: usize,
    considered: usize,
) -> Explanation {
    let mut best: Option<((usize, String), FactId, Vec<FactId>)> = None;
    for group in groups {
        let t = group[0];
        let mut core =
            ein_core::walks::unsat_core(kb, terms, group, ein_core::walks::Justifications::Primary);
        core.sort_unstable();
        core.dedup();
        if core.is_empty() {
            continue;
        }
        // The sorted whole-fact `repr`s joined by a space — not the id
        // tuple's.
        let mut reprs: Vec<String> = core.iter().map(|&f| repr(&terms.repr_fact(f))).collect();
        reprs.sort();
        let key = (core.len(), reprs.join(" "));
        if best.as_ref().is_none_or(|(b, _, _)| key < *b) {
            best = Some((key, t, core));
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
        Some((_, target, frontier)) => Explanation {
            frontier,
            target: Some(target),
            exhausted: false,
            cut,
            rounds,
            facts_considered: considered,
        },
    }
}

/// The smallest explanation of any contradiction in `kb`.
///
/// `None` explains every contradiction the detector finds, each one's facts
/// together ([`contradiction_frontier`]; until M1e2 S1e2.2.21 each by its
/// witness alone). `Some(facts)` explains each of `facts` alone, as
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
/// the smallest of them exactly the culprit. The four numbers are
/// `explain_semantics::the_injected_contradiction_fans_out_and_the_union_overstates_it`'s
/// since M1e `MA-M4`, because this comment and `solve.rs`'s said two different
/// things and neither was current.
pub fn smallest_contradiction_frontier(
    kb: &Kb,
    terms: &Terms,
    witnesses: Option<&[FactId]>,
) -> Vec<FactId> {
    minimal_contradiction_frontier(kb, terms, witnesses, &ExplanationBudget::default()).frontier
}
