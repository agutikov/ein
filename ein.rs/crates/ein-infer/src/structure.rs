//! The structure of the hypothesis set `H`, read from the program's own
//! lattice, with nobody declaring it.
//!
//! Every object here is a statement about nodes [`crate::region`] defines:
//! root's fixpoint with a commitment written in and saturated, by the node
//! function every route and the oracle share, reached through
//! [`crate::explore`]'s node cache (`Space::eval`). So each is true of the
//! lattice `R` is a region of, and none is a second semantics.
//!
//! ### The pair layer
//!
//! [`PairLayer`] is what the nodes of one candidate and of two decide at
//! root, over `H` as root builds it:
//!
//! | object | read from | definition |
//! |---|---|---|
//! | **dead alone** | the node `{a}` | it is dead |
//! | **implies** | the node `{a}`, alive | `b` is in its closure: committing `a` brings `b` |
//! | **equivalent** | two alive single nodes | one closure |
//! | **excludes** | the node `{a, b}`, both alive alone, neither implying the other | it is dead |
//! | **interacts** | the same node, alive | its fixpoint holds a fact that neither `{a}`'s nor `{b}`'s does |
//!
//! The excluded pairs are the members of `N`, the minimal dead sets, that
//! hold two candidates: a dead pair whose two members are each alive alone,
//! over an alive root. `ein-infer/tests/pair_layer.rs` holds the two to each
//! other on every corpus program whose describe exhausts, where `N` is read
//! off the maximal alive states by duality and shares no code with this
//! module past the node.
//!
//! ### What needs no saturation
//!
//! The singles are read first, all of them. On a program the static check
//! calls monotone two licences then decide a pair without its node:
//!
//! - **L1, dead is upward-closed.** A pair with a member dead alone is dead.
//!   It is not an exclusion: its dead member says all there is to say.
//! - **L3, a commitment's state is its closure's.** A pair one of whose
//!   members implies the other is the node of the one that implies: alive,
//!   and with no fact of its own.
//!
//! A pair is skipped by one of the two and by nothing else: not by the
//! relation of its members, not by their arguments. Under a dead root every
//! node is dead (L0, on any program), so every candidate is dead alone and
//! no node is read.
//!
//! ### A program the static check flags
//!
//! There neither licence holds, so **every pair is saturated**, and the layer
//! is still a set of true statements about root's nodes; nothing above root
//! follows from them ([`PairLayer::domain`]). Two more lists say where the
//! licences would have read a node wrongly: [`PairLayer::against_l1`], a pair
//! alive over a member dead alone, and [`PairLayer::against_l3`], a pair one
//! of whose members implies the other whose node is not that member's.
//!
//! ### What it costs, and where it stops
//!
//! Root's node, `n` singles, and the pairs the licences leave, each one
//! saturation; the budget is the routes', facts beyond root summed over the
//! fixpoints computed, checked before each step by the cache's own
//! projection. A stopped layer says what it read: the lists hold the pairs
//! saturated, [`Pairs::left`] counts the rest, and the singles not read are
//! [`PairLayer::singles_left`]. Nothing here is written anywhere a route
//! reads: the layer reads nodes and moves no answer.
//!
//! ### The theory's share
//!
//! At root a pair two declarations exclude and a pair two clues exclude are
//! the same thing, a dead node. [`Theory`] tells them apart by reading the
//! same universe a second time, in **the small KB**: the program with the
//! puzzle's statements taken out, *rules, ontology and the two facts*. A
//! pair dead there is **of the theory**; a pair root excludes that is alive
//! there is **of the puzzle**.
//!
//! **The small KB is the program over fewer of its facts.** It has the
//! loaded program's registries, every rule, declaration and import, and of
//! the facts a file writes those a rule keeps ([`small_kb`]); `(written T)`
//! is stated again as the loader states it. It is built over the same
//! [`Terms`], so a candidate is one fact in both, and it is what loading the
//! program's text with the other facts deleted gives. Its nodes are the node
//! function's, over its own saturated root and the universe **root** built
//! (`region::prepare_over`): the small KB's own generator is not asked, since
//! what it would propose is another program's `H`.
//!
//! **Its pair layer is a [`PairLayer`]** over root's candidates
//! ([`Theory::small`]). [`PairLayer::excluded`] there are the exclusions the
//! theory makes alone, over **all** of `H`: a candidate dead alone at root is
//! alive alone in a small KB that drops what killed it, and has its row and
//! its column like any other. [`PairLayer::dead_alone`] there are the
//! candidates the theory kills with no clue.
//!
//! **Which statements are the puzzle's** nothing declares, and
//! [`small_kb`] reads it off the candidates. A fact of a relation a
//! candidate is of is a given, and is dropped. Past that it depends on what
//! built `H`. Where the program's own hrules or obligations did, the
//! relations that rung reads are what the universe is made of: a fact of one
//! is kept, and every other fact that names one of the names the candidates'
//! relations hold is dropped. Where the blind enumerator did, nothing says
//! what the universe is made of, and only a fact that names a candidates'
//! relation with such a name, or two such names, is dropped. A fact that
//! names none of them is kept: a declaration that names a relation and no
//! object speaks of the relation whole. The rule reads no relation by what
//! it is called and no `:source`. It is held by what must be true of any
//! right one (`ein-infer/tests/theory_share.rs`): a declared bijection's
//! rows and columns are exactly the theory's pairs, two programs one clue
//! apart have one theory, one puzzle in two encodings has one theory, and
//! the small KB derives no more than the program. What it cannot tell:
//!
//! - **a clue written as a rule** is kept, as every rule is
//!   ([`Theory::rules_naming_objects`] lists the rules that name an object);
//! - **a clue that names none of the candidates' objects** is kept: a gate
//!   between two wires no candidate holds. It derives nothing once the
//!   statements that tie its names to an object are gone;
//! - **under the blind enumerator**, which proposes facts of every open
//!   relation over every name, a type's name and a relation's are
//!   candidates' objects, so the statements that type an object go with the
//!   clues, and a clue that names one object is kept as a type is;
//! - **a table a module states over the candidates' objects**, a digit
//!   table over the digits a letter may be, is dropped with the clues. The
//!   rules that read it have nothing to join it with once the clues are
//!   gone, and nothing in a loaded fact says which file wrote it.
//!
//! **A smaller program derives no more**, unless a rule reads the absence of
//! a statement it dropped. So [`Theory::against_root`], what is dead in the
//! small KB and alive at root, and [`Theory::small_beyond_root`], what the
//! small KB's fixpoint holds that root's does not, are both empty where the
//! rule fits the program, and name the program where it does not.
//!
//! ### The support of a death
//!
//! The second reading of the same question asks no definition. The node of
//! a pair root excludes is dead by a derivation, and
//! [`crate::explain::support_frontier`] grounds that derivation out on the
//! facts the program **states**, following root's activators, which a core
//! leaves to the program: a [`Support`]. A pair whose support lies inside
//! the small KB is of the theory, and otherwise the support names the
//! statements it is the puzzle's by. A support is an explanation, as a core
//! is: sufficient, not least, and it may vary with rule order. Each costs
//! its node a second saturation, since the routes' cache keeps no fixpoint.
//!
//! **Each reading holds the other** ([`Theory::readings`]). A pair the small
//! KB kills whose support names a statement the small KB drops died another
//! way there; a pair alive in the small KB whose support lies inside it is a
//! disagreement. And a support is checked ([`Check`]): the small KB with the
//! statements the support names put back, and the pair committed, is dead,
//! by a node that holds no fact the program's own node of the pair does not.
//! The check puts back one thing more, every statement the small KB drops of
//! **a relation a rule reads the absence of** ([`Stated::read_absent`]): a
//! firing that passed an `absent` rests on the whole of what the program
//! states of that relation, no premise records it, and a program that
//! states half a row of houses derives what the program does not.

use std::collections::BTreeMap;
use std::time::Instant;

use ein_core::entities::{Loc, NameCategory};
use ein_core::{FactId, Kb, Program, Prov, ProvId, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::downset;
use crate::error::SolveError;
use crate::events::{Events, sexpr};
use crate::explain::CoreStatus;
use crate::explore::{self, Ask, Licence, Mode, Seen, Space, Stopped, Which};
use crate::region::{self, Candidate, Cores};

/// The largest step, as the routes'.
const BATCH: u64 = 1024;

/// What [`PairLayer::domain`] says of a program the static check flags.
pub const FLAGGED: &str = "the static check flags this program: every statement of the pair layer \
                           is true of root's nodes, and nothing above root follows from them";

/// How the layer is asked.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Facts beyond root, summed over every fixpoint the layer computes: the
    /// routes' budget, in the routes' unit.
    pub budget: u64,
    /// Threads the node function fans out over. The layer is the same at any
    /// count.
    pub jobs: usize,
    /// Checked before each step, as the budget is.
    pub deadline: Option<Instant>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            budget: downset::DEFAULT_BUDGET,
            jobs: 1,
            deadline: None,
        }
    }
}

/// The pairs of `H`, by what decided each.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pairs {
    /// `n (n − 1) / 2`.
    pub all: u64,
    /// Not saturated: a member is dead alone, so the pair is dead (L1; L0
    /// under a dead root).
    pub dead_member: u64,
    /// Not saturated: one member implies the other, so the pair is that
    /// member's node (L3).
    pub implied: u64,
    /// Read through the node function.
    pub saturated: u64,
    /// Neither licence decides them and the layer stopped before their
    /// nodes.
    pub left: u64,
}

/// The work the layer did, in the read-out's units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    /// Fixpoints computed: root's node, the singles and the pairs saturated.
    pub saturations: u64,
    /// Facts beyond root's, summed over them.
    pub facts: u64,
}

/// What one candidate and two decide at root. Every candidate is its index
/// in [`Self::candidates`], the universe in rendered-text order, and every
/// pair `(a, b)` has `a < b` unless its field says otherwise; each list is
/// ascending, so the layer does not depend on the interner's ids or on the
/// job count.
#[derive(Clone, Debug)]
pub struct PairLayer {
    pub candidates: Vec<Candidate>,
    /// The static verdict: L1 and L3 hold, and decided the pairs they could.
    pub monotone: bool,
    /// Root's own fixpoint derives `(false)`: every candidate is dead alone
    /// without a visit, and no pair is read.
    pub root_dead: bool,
    /// Singles the layer stopped before. While it is not 0,
    /// [`Self::dead_alone`] holds the dead singles met and every list after
    /// it is empty.
    pub singles_left: u64,
    /// The candidates root's own fixpoint holds: committing one is
    /// committing nothing. They are in no implication and in no equivalence
    /// class below.
    pub at_root: Vec<u32>,
    /// The candidates whose own node is dead.
    pub dead_alone: Vec<u32>,
    /// `(a, b)`, in either order of index: `a` is alive alone and `b`, which
    /// is not `a` and not at root, is in the closure of `{a}`.
    pub implies: Vec<(u32, u32)>,
    /// The classes of two or more candidates alive alone with one closure,
    /// each ascending, in the order of their first members.
    pub equivalent: Vec<Vec<u32>>,
    /// The pairs that exclude: each member alive alone, neither implying the
    /// other, the pair dead.
    pub excluded: Vec<(u32, u32)>,
    /// The pairs that interact: as [`Self::excluded`]'s but alive, with a
    /// fact in the pair's fixpoint that neither member's own holds.
    pub interacting: Vec<(u32, u32)>,
    /// On a flagged program: the pairs alive over a member dead alone.
    pub against_l1: Vec<(u32, u32)>,
    /// On a flagged program: the pairs one of whose members implies the
    /// other and whose node is not that member's state, each with whether
    /// it is dead.
    pub against_l3: Vec<(u32, u32, bool)>,
    pub pairs: Pairs,
    /// The licences that decided a pair without its node: none on a flagged
    /// program.
    pub licences: Vec<Licence>,
    pub work: Work,
    /// Why the layer stopped before its last node, when it did.
    pub stopped: Option<Stopped>,
}

impl PairLayer {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Candidate `b`, as the program writes it.
    pub fn text(&self, b: u32) -> &str {
        &self.candidates[b as usize].text
    }

    /// Every single and every pair the licences leave was read.
    pub fn complete(&self) -> bool {
        self.stopped.is_none()
    }

    /// What the layer's statements are about, where that is not the whole
    /// lattice: [`FLAGGED`] on a program the static check flags.
    pub fn domain(&self) -> Option<&'static str> {
        (!self.monotone).then_some(FLAGGED)
    }

    /// How many candidates are in at least one excluded pair.
    pub fn in_an_excluded_pair(&self) -> usize {
        let mut seen = vec![false; self.n()];
        for &(a, b) in &self.excluded {
            seen[a as usize] = true;
            seen[b as usize] = true;
        }
        seen.into_iter().filter(|&x| x).count()
    }
}

/// The pair layer of `root`, a **freshly loaded, unsaturated** program
/// nothing else will use, as [`crate::explore::run`] asks. Root is saturated
/// once and never written after.
pub fn pair_layer(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<PairLayer, SolveError> {
    with_space(root, terms, ast, opts, None, pair_layer_in)
}

/// `read` over the prepared space of `root`, a freshly loaded program: the
/// routes' node cache, at the layer's budget, keeping no node's record. Over
/// the universe root's generator builds, or over the one the caller names
/// ([`region::prepare_over`]).
fn with_space<R>(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    universe: Option<&[Candidate]>,
    read: impl FnOnce(&mut Space<'_>) -> Result<R, SolveError>,
) -> Result<R, SolveError> {
    let prepared = match universe {
        None => region::prepare(root, terms, ast)?,
        Some(named) => region::prepare_over(root, terms, ast, named)?,
    };
    let mut events = Events::off();
    let space = explore::Options {
        budget: opts.budget,
        limit: 0,
        jobs: opts.jobs,
        deadline: opts.deadline,
        cores: Cores::READ_OUT,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let mut sp = Space::open(root, terms, ast, prepared, &space, &mut events);
    read(&mut sp)
}

/// One single's node, as the layer keeps it.
struct Single {
    dead: bool,
    /// Its state, as the cache numbers it.
    class: u32,
    closure: Box<[u32]>,
}

impl Single {
    fn holds(&self, b: u32) -> bool {
        self.closure.binary_search(&b).is_ok()
    }
}

/// The next step's size: the largest the budget affords, and one where it
/// affords none, so that the cache's own refusal is the stop reported.
fn step(sp: &Space<'_>, want: u64) -> usize {
    want.min(BATCH).min(sp.affordable().max(1)) as usize
}

/// A fact of `pair` that neither `a` nor `b` holds: each a state, its facts
/// beyond root's ascending.
fn has_own_fact(pair: &[FactId], a: &[FactId], b: &[FactId]) -> bool {
    let (mut i, mut j) = (0, 0);
    pair.iter().any(|f| {
        while i < a.len() && a[i] < *f {
            i += 1;
        }
        while j < b.len() && b[j] < *f {
            j += 1;
        }
        !(i < a.len() && a[i] == *f) && !(j < b.len() && b[j] == *f)
    })
}

/// How the licences read a pair before its node.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Before {
    /// A member is dead alone.
    DeadMember,
    /// One member implies the other.
    Implied,
    /// Neither.
    Open,
}

fn before(singles: &[Single], a: u32, b: u32) -> Before {
    let (x, y) = (&singles[a as usize], &singles[b as usize]);
    if x.dead || y.dead {
        Before::DeadMember
    } else if x.holds(b) || y.holds(a) {
        Before::Implied
    } else {
        Before::Open
    }
}

/// [`pair_layer`] over a program's prepared space, the routes' node cache:
/// a node the space has computed is read and not saturated again, and the
/// layer's nodes are the space's from then on. [`PairLayer::work`] is what
/// the layer added to the space's.
pub(crate) fn pair_layer_in(sp: &mut Space<'_>) -> Result<PairLayer, SolveError> {
    let licensed = sp.is_monotone();
    layer(sp, licensed)
}

/// The layer over `sp`, L1 and L3 deciding the pairs they can when
/// `licensed` and every pair saturated otherwise.
fn layer(sp: &mut Space<'_>, licensed: bool) -> Result<PairLayer, SolveError> {
    let n = sp.n();
    let monotone = sp.is_monotone();
    let (saturations, facts) = sp.work();
    let mut out = PairLayer {
        candidates: sp.candidates().to_vec(),
        monotone,
        root_dead: sp.root_is_dead(),
        singles_left: 0,
        at_root: Vec::new(),
        dead_alone: Vec::new(),
        implies: Vec::new(),
        equivalent: Vec::new(),
        excluded: Vec::new(),
        interacting: Vec::new(),
        against_l1: Vec::new(),
        against_l3: Vec::new(),
        pairs: Pairs {
            all: (n as u64) * (n as u64).saturating_sub(1) / 2,
            ..Pairs::default()
        },
        licences: Vec::new(),
        work: Work::default(),
        stopped: None,
    };
    read(sp, &mut out, licensed)?;
    let (s, f) = sp.work();
    out.work = Work {
        saturations: s - saturations,
        facts: f - facts,
    };
    Ok(out)
}

fn read(sp: &mut Space<'_>, out: &mut PairLayer, licensed: bool) -> Result<(), SolveError> {
    let n = out.n() as u32;
    if out.root_dead {
        // A fork only adds: every node holds root's contradiction.
        out.dead_alone = (0..n).collect();
        out.pairs.dead_member = out.pairs.all;
        out.licences = vec![Licence::ForkOnlyAdds];
        return Ok(());
    }

    // Root's node, for the candidates its own fixpoint holds.
    let root = match sp.eval(&[Box::default()])? {
        Ok(seen) => seen.into_iter().next().expect("one node"),
        Err(stop) => {
            out.stopped = Some(stop);
            out.singles_left = u64::from(n);
            out.pairs.left = out.pairs.all;
            return Ok(());
        }
    };
    out.at_root = root.closure.to_vec();

    // The singles, all of them, before any pair.
    let mut singles: Vec<Single> = Vec::with_capacity(n as usize);
    while (singles.len() as u32) < n {
        let from = singles.len() as u32;
        let take = step(sp, u64::from(n - from)) as u32;
        let sets: Vec<Box<[u32]>> = (from..from + take).map(|b| Box::from([b])).collect();
        match sp.eval(&sets)? {
            Ok(seen) => singles.extend(seen.into_iter().map(|s: Seen| Single {
                dead: s.dead,
                class: s.class,
                closure: s.closure,
            })),
            Err(stop) => {
                out.stopped = Some(stop);
                break;
            }
        }
    }
    out.dead_alone = (0..singles.len() as u32)
        .filter(|&b| singles[b as usize].dead)
        .collect();
    if (singles.len() as u32) < n {
        out.singles_left = u64::from(n) - singles.len() as u64;
        out.pairs.left = out.pairs.all;
        return Ok(());
    }

    // Implication and equivalence, off the singles' closures.
    let held = out.at_root.clone();
    let at_root = |b: u32| held.binary_search(&b).is_ok();
    let mut classes: BTreeMap<&[u32], Vec<u32>> = BTreeMap::new();
    for a in 0..n {
        let single = &singles[a as usize];
        if single.dead || at_root(a) {
            continue;
        }
        out.implies.extend(
            single
                .closure
                .iter()
                .filter(|&&b| b != a && !at_root(b))
                .map(|&b| (a, b)),
        );
        classes.entry(&single.closure).or_default().push(a);
    }
    out.equivalent = classes.into_values().filter(|c| c.len() > 1).collect();
    out.equivalent.sort();

    // The pairs, in the universe's order, a step at a time: the licences
    // first, then the node of each pair they leave.
    let mut pairs = (0..n).flat_map(|a| (a + 1..n).map(move |b| (a, b)));
    let mut skipped = [false; 2];
    loop {
        let want = step(sp, BATCH);
        let mut chunk: Vec<(u32, u32, Before)> = Vec::with_capacity(want);
        while chunk.len() < want {
            let Some((a, b)) = pairs.next() else {
                break;
            };
            let read = before(&singles, a, b);
            match read {
                Before::DeadMember if licensed => {
                    out.pairs.dead_member += 1;
                    skipped[0] = true;
                }
                Before::Implied if licensed => {
                    out.pairs.implied += 1;
                    skipped[1] = true;
                }
                _ => chunk.push((a, b, read)),
            }
        }
        if chunk.is_empty() {
            break;
        }
        let sets: Vec<Box<[u32]>> = chunk.iter().map(|&(a, b, _)| Box::from([a, b])).collect();
        let seen = match sp.eval(&sets)? {
            Ok(seen) => seen,
            Err(stop) => {
                out.stopped = Some(stop);
                out.pairs.left = chunk.len() as u64;
                // The licences still decide what they decide of the rest.
                for (a, b) in pairs.by_ref() {
                    match before(&singles, a, b) {
                        Before::DeadMember if licensed => out.pairs.dead_member += 1,
                        Before::Implied if licensed => out.pairs.implied += 1,
                        _ => out.pairs.left += 1,
                    }
                }
                break;
            }
        };
        out.pairs.saturated += chunk.len() as u64;
        let states = sp.states();
        for (&(a, b, read), node) in chunk.iter().zip(&seen) {
            let (x, y) = (&singles[a as usize], &singles[b as usize]);
            match read {
                // Unlicensed, read against what L1 would have said.
                Before::DeadMember => {
                    if !node.dead {
                        out.against_l1.push((a, b));
                    }
                }
                // And against L3: the node of whichever member implies.
                Before::Implied => {
                    let is = |s: &Single| !node.dead && node.class == s.class;
                    if (x.holds(b) && !is(x)) || (y.holds(a) && !is(y)) {
                        out.against_l3.push((a, b, node.dead));
                    }
                }
                Before::Open if node.dead => out.excluded.push((a, b)),
                Before::Open => {
                    if has_own_fact(
                        states[node.class as usize],
                        states[x.class as usize],
                        states[y.class as usize],
                    ) {
                        out.interacting.push((a, b));
                    }
                }
            }
        }
    }
    if skipped[0] {
        out.licences.push(Licence::DeadUpward);
    }
    if skipped[1] {
        out.licences.push(Licence::Closure);
    }
    Ok(())
}

// ── The theory's share ────────────────────────────────────────────────

/// How a fact came to be among what the program states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// A fact a file writes: the program's own, or a module's it imports.
    Fact,
    /// The fact a `(relation …)` declaration stores.
    Declaration,
    /// `(written T)`, the loader's, for a fact nested in what a file writes
    /// or in a rule's conclusion.
    Written,
}

impl Origin {
    pub fn as_str(self) -> &'static str {
        match self {
            Origin::Fact => "fact",
            Origin::Declaration => "declaration",
            Origin::Written => "written",
        }
    }
}

/// One fact the loaded program holds before any saturation, and what the
/// small KB does with it.
#[derive(Clone, Debug)]
pub struct Statement {
    pub fact: FactId,
    /// As the program writes it.
    pub text: String,
    /// Its `:source`, for a reader. Nothing here reads it to decide.
    pub source: Option<String>,
    pub origin: Origin,
    /// The small KB keeps it.
    pub kept: bool,
    /// The clause of the rule that said so.
    pub clause: &'static str,
    /// It is a fact of a relation a rule reads the absence of
    /// ([`Stated::read_absent`]): where the small KB drops it, every check of
    /// a support puts it back.
    pub read_absent: bool,
}

/// What a rule for the small KB says of one stated fact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Verdict {
    pub kept: bool,
    pub clause: &'static str,
}

impl Verdict {
    /// The theory's: the small KB keeps it.
    pub fn theory(clause: &'static str) -> Verdict {
        Verdict { kept: true, clause }
    }

    /// The puzzle's: the small KB drops it.
    pub fn puzzle(clause: &'static str) -> Verdict {
        Verdict {
            kept: false,
            clause,
        }
    }
}

/// What a rule for the small KB may read: the program as loaded and as root
/// leaves it, by nothing a relation is called.
pub struct Stated<'a> {
    /// Root's fixpoint.
    pub root: &'a Kb,
    pub terms: &'a Terms,
    /// `H`, in the universe's order.
    pub candidates: &'a [Candidate],
    /// **The candidates' relations**: the relations a candidate of `H` is
    /// of, and, where the program's own hrules or its obligations built `H`,
    /// those that rung can still propose a fact of
    /// ([`crate::hypgen::eligible_relations`]). The second half is what
    /// keeps a relation here once root has decided every fact of it and no
    /// candidate of it is left.
    pub hypothesised: FxHashSet<Symbol>,
    /// **The candidates' objects**: every name among the arguments, at any
    /// depth, of a candidate, or of a fact of a candidates' relation that
    /// root's fixpoint holds. Not of one it denies: a rule that excludes
    /// over a whole type denies facts of names no candidate holds.
    pub objects: FxHashSet<Symbol>,
    /// **The relations a rule reads the absence of**: those a saturation
    /// rule of root's plans reads inside an `absent`, and those such a
    /// relation is derived from, the activators of the rules that derive it
    /// among them. What a rule concludes from the absence of a fact rests
    /// on the whole of what the program states of that relation, and no
    /// premise of the firing records it.
    pub read_absent: FxHashSet<Symbol>,
    /// **The program's own rung built `H`**: its hrules, or what would
    /// discharge its obligations. Not the blind enumerator, which a program
    /// with neither, or with an obligation the ladder declines, falls to.
    pub declared: bool,
    /// The relations read by the rung of the ladder that built `H`: the
    /// program's hrules' premises where it declares one, the triggers and
    /// witnesses of the obligations root owes where the candidates are what
    /// would discharge them, and none under the blind enumerator, which
    /// reads names and no relation.
    pub generator_reads: FxHashSet<Symbol>,
}

impl Stated<'_> {
    /// The relation `fact` is of; a stored negation's is the relation of
    /// the fact it denies.
    pub fn relation(&self, fact: FactId) -> Symbol {
        subject(self.terms, fact)
    }

    /// Every name among `fact`'s arguments, at any depth, once each, in the
    /// order written.
    pub fn names(&self, fact: FactId) -> Vec<Symbol> {
        let mut out = Vec::new();
        names_of(self.terms, fact, &mut out);
        out
    }

    /// `fact` is headed by the name of a rule or of an obligation: it binds
    /// that rule's parameters.
    pub fn is_activator(&self, fact: FactId) -> bool {
        let (rel, _) = self.terms.fact(fact);
        let program = self.root.program();
        program.rules.contains(rel) || program.obligations.contains(rel)
    }

    /// What kind of name `name` is in this program.
    pub fn category(&self, name: Symbol) -> NameCategory {
        self.root.category(self.terms, name)
    }
}

/// The relation `fact` is of, through a negation.
fn subject(terms: &Terms, fact: FactId) -> Symbol {
    let (rel, args) = terms.fact(fact);
    if rel == terms.kernel.not
        && let Some(inner) = args.first().and_then(|a| a.as_fact())
    {
        return terms.fact(inner).0;
    }
    rel
}

/// Every name among `fact`'s arguments, at any depth, appended to `out`
/// where it does not hold it.
fn names_of(terms: &Terms, fact: FactId, out: &mut Vec<Symbol>) {
    for &a in terms.fact(fact).1 {
        if let Some(s) = a.as_sym() {
            if !out.contains(&s) {
                out.push(s);
            }
        } else if let Some(f) = a.as_fact() {
            names_of(terms, f, out);
        }
    }
}

/// The program as loaded: what [`Loaded::over`] states a program over fewer
/// of its facts from.
struct Loaded {
    /// The registries, copied, with nothing computed from them.
    program: Program,
    /// Every fact the loaded program holds, in the order the loader stored
    /// them, each with its record.
    facts: Vec<(FactId, Option<ProvId>)>,
    /// The terms the loader states `(written T)` for from the rules' text.
    conclusions: Vec<(Value, Option<Loc>)>,
}

impl Loaded {
    /// Of `root`, **before** anything saturates it.
    fn of(root: &Kb, terms: &mut Terms, ast: &Ast) -> Result<Loaded, SolveError> {
        let mut program = root.program().clone();
        program.domain.take();
        let conclusions =
            ein_ir::written_in_conclusions(ast, terms, &program, &mut FxHashSet::default())
                .map_err(|e| SolveError::Unsupported(e.to_string()))?;
        Ok(Loaded {
            program,
            facts: root.facts().map(|f| (f, root.primary(f))).collect(),
            conclusions,
        })
    }

    /// **The program over the facts `keep` keeps**: the loaded program's
    /// registries, every rule, declaration and import among them, and of
    /// its facts those `keep` says, in the order written, each with its own
    /// record. `(written T)` is the loader's and is stated again as the
    /// loader states it, for each fact nested in a fact kept and each one a
    /// rule's conclusion holds (`ein_ir::written_in`), so `keep` is not asked
    /// about one. The KB is what loading the program's text with the other
    /// facts deleted gives, unsaturated, over the same [`Terms`]: a fact of
    /// either is the same [`FactId`] in both.
    fn over(&self, terms: &mut Terms, keep: &dyn Fn(FactId) -> bool) -> Kb {
        let written = terms.kernel.written;
        let step = terms.kernel.written_step;
        let mut kb = Kb::new(self.program.clone());
        let mut kept: Vec<(FactId, Option<ProvId>)> = Vec::new();
        let mut records: FxHashMap<FactId, ProvId> = FxHashMap::default();
        for &(f, prov) in &self.facts {
            let (rel, args) = terms.fact(f);
            if rel == written {
                records.extend(prov.map(|p| (f, p)));
                continue;
            }
            if !keep(f) {
                continue;
            }
            let args = args.to_vec();
            kb.add_fact(terms, rel, &args, prov)
                .expect("a fact the program holds");
            kept.push((f, prov));
        }
        let mut seen: FxHashSet<Value> = FxHashSet::default();
        for (f, prov) in kept {
            for t in ein_ir::written_in(terms, f, &mut seen) {
                let id = terms
                    .intern_fact(written, &[t])
                    .expect("a term the program writes");
                // The loaded program's own record where it read the term in
                // this fact, and a record of the same step where it read it
                // in one the KB leaves out.
                let prov = match records.get(&id) {
                    Some(&p) if *terms.provs.get(p).premises == [f] => p,
                    _ => {
                        let loc = prov.and_then(|p| terms.provs.get(p).loc);
                        terms.provs.push(Prov::from_rule(step, Box::new([f]), loc))
                    }
                };
                kb.add_fact(terms, written, &[t], Some(prov))
                    .expect("a term the program writes");
            }
        }
        for &(t, loc) in &self.conclusions {
            if !seen.insert(t) {
                continue;
            }
            let id = terms
                .intern_fact(written, &[t])
                .expect("a term the program writes");
            let prov = match records.get(&id) {
                Some(&p) if terms.provs.get(p).premises.is_empty() => p,
                _ => terms.provs.push(Prov::from_rule(step, Box::new([]), loc)),
            };
            kb.add_fact(terms, written, &[t], Some(prov))
                .expect("a term the program writes");
        }
        kb.rebuild_indexes(terms);
        kb
    }
}

/// Which nodes of one candidate and of two a layer found dead, by the node
/// or by the licence that decided it: a reader for a comparison of two
/// layers over one universe.
pub struct Deaths {
    root_dead: bool,
    alone: Vec<bool>,
    pairs: FxHashSet<(u32, u32)>,
    alive_over_dead: FxHashSet<(u32, u32)>,
}

impl Deaths {
    pub fn of(l: &PairLayer) -> Deaths {
        let mut alone = vec![false; l.n()];
        for &b in &l.dead_alone {
            alone[b as usize] = true;
        }
        let mut pairs: FxHashSet<(u32, u32)> = l.excluded.iter().copied().collect();
        pairs.extend(l.against_l3.iter().filter(|x| x.2).map(|&(a, b, _)| (a, b)));
        Deaths {
            root_dead: l.root_dead,
            alone,
            pairs,
            alive_over_dead: l.against_l1.iter().copied().collect(),
        }
    }

    /// The node `{a}` is dead.
    pub fn single(&self, a: u32) -> bool {
        self.root_dead || self.alone[a as usize]
    }

    /// The node `{a, b}` is dead: the layer read it dead, or a member is
    /// dead alone and no node the layer read says otherwise. A pair a
    /// stopped layer left is not dead here.
    pub fn pair(&self, a: u32, b: u32) -> bool {
        let p = (a.min(b), a.max(b));
        if self.root_dead {
            return true;
        }
        if self.alone[a as usize] || self.alone[b as usize] {
            return !self.alive_over_dead.contains(&p);
        }
        self.pairs.contains(&p)
    }
}

/// How [`theory`] is asked.
#[derive(Clone, Copy, Debug)]
pub struct TheoryOptions {
    /// The budget, the jobs and the deadline of **each** layer: root's and
    /// the small KB's are read at it separately, and so is each check.
    pub layer: Options,
    /// Read root's own singles and pairs. Without them root is read as far
    /// as its universe, which is all the small KB's layer needs: root's
    /// [`PairLayer`] holds the candidates and says it stopped there
    /// ([`Stopped::NoSearch`]), nothing is compared with it, and no support
    /// is read.
    pub root_pairs: bool,
    /// Read the support of every pair root excludes.
    pub supports: bool,
    /// Check each support read ([`Check`]): the small KB with the
    /// statements the support names put back, and the pair committed, is
    /// dead.
    pub check: bool,
}

impl Default for TheoryOptions {
    fn default() -> Self {
        TheoryOptions {
            layer: Options::default(),
            root_pairs: true,
            supports: false,
            check: false,
        }
    }
}

/// What the small KB, with the statements a support names put back, says of
/// the pair: the check *rules, ontology and the two facts*, run on a
/// support. Every statement the small KB drops of a relation a rule reads
/// the absence of is put back beside them ([`Statement::read_absent`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    /// The pair's node is dead there.
    pub dead: bool,
    /// Its root is dead there, before the pair.
    pub root_dead: bool,
    /// Facts the pair's node holds there that the program's own node of the
    /// pair does not. At 0 the check's node is inside the program's, so its
    /// death is the program's; anything else is a fact derived from the
    /// **absence** of a statement the check left out, and the death may be
    /// another.
    pub beyond: u64,
    /// Why it was not read, where it was not: a refusal, or the budget.
    pub unread: Option<String>,
}

/// The stated facts one excluded pair's death rests on
/// ([`crate::explain::support_frontier`]).
#[derive(Clone, Debug, Default)]
pub struct Support {
    /// The search labelled the death. `false` where its closure passed the
    /// budget's count of facts first, and the lists below are empty.
    pub read: bool,
    /// The stated facts, as places in [`Theory::stated`], ascending.
    pub stated: Vec<u32>,
    /// The members of the pair the derivation rests on.
    pub commitments: Vec<u32>,
    /// A fact of the frontier that is neither, as text. None is expected.
    pub other: Vec<String>,
    /// The rules whose firing ends the derivation.
    pub rules: Vec<String>,
    /// What the search established, as a core's.
    pub status: Option<CoreStatus>,
    /// The check, where it was asked.
    pub check: Option<Check>,
}

/// What a comparison of the small KB with root found against *a smaller
/// program derives no more*.
#[derive(Clone, Debug, Default)]
pub struct Against {
    /// Candidates dead alone in the small KB and alive alone at root.
    pub singles: Vec<u32>,
    /// Pairs dead in the small KB and alive at root.
    pub pairs: Vec<(u32, u32)>,
}

impl Against {
    pub fn is_empty(&self) -> bool {
        self.singles.is_empty() && self.pairs.is_empty()
    }
}

/// The work the supports and their checks cost, beside the two layers'.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SupportWork {
    /// Supports read, each a node saturated a second time.
    pub supports: u64,
    /// Programs the checks built, one per distinct support, each a root
    /// saturated.
    pub check_roots: u64,
    /// Fixpoints the checks computed past their roots.
    pub check_saturations: u64,
    /// Facts beyond each check's own root, summed.
    pub check_facts: u64,
}

/// **The theory's share** of a program's pair layer: what *rules, ontology
/// and the two facts* decide, with the rest of what the program states taken
/// out.
#[derive(Clone, Debug)]
pub struct Theory {
    /// Root's pair layer: the program as written.
    pub root: PairLayer,
    /// Every fact the program states, in the order the loader stored them,
    /// each with whether the small KB keeps it.
    pub stated: Vec<Statement>,
    /// **The small KB's pair layer over the same universe**:
    /// [`PairLayer::excluded`] are the pairs the theory excludes alone and
    /// [`PairLayer::dead_alone`] the candidates it kills alone, every index
    /// a place in root's [`PairLayer::candidates`]. `Err` where the small KB
    /// has no layer: the message is the engine's, a refusal by *forbid* or a
    /// rule that does not compile.
    pub small: Result<PairLayer, String>,
    /// Facts the small KB's own fixpoint holds that root's does not. A
    /// program over fewer statements derives no more unless a rule reads the
    /// absence of one, so anything but 0 names such a program.
    pub small_beyond_root: u64,
    /// What is dead in the small KB and alive at root.
    pub against_root: Against,
    /// One per pair of root's [`PairLayer::excluded`], in its order, where
    /// supports were asked; empty otherwise.
    pub supports: Vec<Support>,
    pub support_work: SupportWork,
    /// The rung of the ladder that built `H`, by its word
    /// ([`crate::oblgen::Mode::as_str`]).
    pub rung: &'static str,
    /// The saturation rules whose text names one of the candidates' objects,
    /// by name, sorted. The small KB keeps every rule, so what such a rule
    /// says of that object is the theory's whatever it is.
    pub rules_naming_objects: Vec<String>,
}

/// How the two readings of one root-excluded pair meet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Readings {
    /// Dead in the small KB, and the support root recorded lies inside it.
    pub theory_both: u64,
    /// Dead in the small KB, and root's recorded derivation rests on a
    /// statement the small KB drops: a support is sufficient, not least,
    /// and the small KB reaches the death another way.
    pub theory_by_another_way: u64,
    /// Alive in the small KB, and the support names a statement it drops:
    /// the pair is the puzzle's, by those statements.
    pub puzzle_both: u64,
    /// **Alive in the small KB though the support lies inside it**: the two
    /// readings disagree.
    pub disagree: u64,
    /// No support was read for the pair.
    pub unread: u64,
}

impl Theory {
    /// The pairs the theory excludes alone: each member alive alone in the
    /// small KB, the pair dead there. Empty where the small KB has no layer.
    pub fn excluded(&self) -> &[(u32, u32)] {
        self.small.as_ref().map_or(&[], |l| &l.excluded)
    }

    /// The stated facts the small KB drops, as places in [`Self::stated`].
    pub fn dropped(&self) -> Vec<u32> {
        (0..self.stated.len() as u32)
            .filter(|&k| !self.stated[k as usize].kept)
            .collect()
    }

    /// Of root's excluded pairs, in their order: is each dead in the small
    /// KB, *of the theory*? All `false` where the small KB has no layer.
    pub fn of_the_theory(&self) -> Vec<bool> {
        match &self.small {
            Ok(l) => {
                let dead = Deaths::of(l);
                self.root
                    .excluded
                    .iter()
                    .map(|&(a, b)| dead.pair(a, b))
                    .collect()
            }
            Err(_) => vec![false; self.root.excluded.len()],
        }
    }

    /// The two readings against each other, over root's excluded pairs.
    /// All zero where no support was asked.
    pub fn readings(&self) -> Readings {
        let mut r = Readings::default();
        for (theory, s) in self.of_the_theory().into_iter().zip(&self.supports) {
            if !s.read {
                r.unread += 1;
                continue;
            }
            let inside = s.stated.iter().all(|&k| self.stated[k as usize].kept);
            match (theory, inside) {
                (true, true) => r.theory_both += 1,
                (true, false) => r.theory_by_another_way += 1,
                (false, false) => r.puzzle_both += 1,
                (false, true) => r.disagree += 1,
            }
        }
        r
    }
}

/// One saturation plan, as the rule's inputs read it.
struct Read {
    rule: Symbol,
    /// Each relation it reads, with the `absent` depth it reads it at.
    reads: Vec<(Symbol, u32)>,
    writes: Vec<Symbol>,
}

/// What the rule reads of root that only its space holds.
struct RootRead {
    obligation_reads: FxHashSet<Symbol>,
    read_absent: FxHashSet<Symbol>,
    rung: crate::oblgen::Mode,
}

impl RootRead {
    fn of(sp: &Space<'_>) -> RootRead {
        let terms = sp.terms();
        let program = sp.root().program();
        let mut obligation_reads = FxHashSet::default();
        let mut saturation: Vec<Read> = Vec::new();
        for p in sp.plans() {
            let reads = || -> Vec<(Symbol, u32)> {
                crate::monotone::reads_of(p, terms)
                    .into_iter()
                    .map(|(r, depth)| (r.name, depth))
                    .collect()
            };
            if program.rules.contains(p.rule) {
                let writes: Vec<Symbol> = crate::monotone::writes_of(p, terms)
                    .into_iter()
                    .map(|r| r.name)
                    .collect();
                saturation.push(Read {
                    rule: p.rule,
                    reads: reads(),
                    writes,
                });
            } else if program.obligations.contains(p.rule) {
                obligation_reads.extend(reads().into_iter().map(|(r, _)| r));
            }
        }
        // The relations a saturation rule reads the absence of, and those
        // such a relation is derived from: a plan that concludes one makes
        // what it reads, and the activator it was compiled from, one too.
        let mut read_absent: FxHashSet<Symbol> = saturation
            .iter()
            .flat_map(|p| p.reads.iter().filter(|(_, d)| d % 2 == 1).map(|&(r, _)| r))
            .collect();
        loop {
            let before = read_absent.len();
            for p in &saturation {
                if p.writes.iter().any(|w| read_absent.contains(w)) {
                    read_absent.insert(p.rule);
                    read_absent.extend(p.reads.iter().filter(|(_, d)| d % 2 == 0).map(|&(r, _)| r));
                }
            }
            if read_absent.len() == before {
                break;
            }
        }
        RootRead {
            read_absent,
            obligation_reads,
            rung: sp.rung(),
        }
    }
}

/// [`theory_by`] under [`small_kb`], the one rule the owner applies.
pub fn theory(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &TheoryOptions,
) -> Result<Theory, SolveError> {
    theory_by(root, terms, ast, opts, &small_kb)
}

/// The theory's share of `root`, a **freshly loaded, unsaturated** program
/// nothing else will use, with the small KB what `rule` says it is: root's
/// pair layer, the small KB's over the same universe, and where
/// [`TheoryOptions::supports`] asks, the support of every pair root
/// excludes. `rule` is asked once about each fact a file writes; a
/// declaration's fact is kept, and `(written T)` follows the facts it is
/// read in.
///
/// [`theory`] is this under [`small_kb`]. A caller that names another rule
/// measures an alternative, as `ein-infer/examples/pair_layer.rs` does;
/// nothing in the engine does.
pub fn theory_by(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &TheoryOptions,
    rule: &dyn Fn(&Stated<'_>, FactId) -> Verdict,
) -> Result<Theory, SolveError> {
    let loaded = Loaded::of(root, terms, ast)?;
    let stated_set: FxHashSet<FactId> = loaded.facts.iter().map(|&(f, _)| f).collect();

    // Root's layer, what only its space holds, the supports, and for a
    // check each excluded pair's own node.
    let (root_pairs, want, check) = (
        opts.root_pairs,
        opts.root_pairs && opts.supports,
        opts.root_pairs && opts.supports && opts.check,
    );
    let (root_layer, read, supports, nodes) =
        with_space(root, terms, ast, &opts.layer, None, |sp| {
            let l = if root_pairs {
                pair_layer_in(sp)?
            } else {
                universe_alone(sp)
            };
            let read = RootRead::of(sp);
            let sets: Vec<Box<[u32]>> =
                l.excluded.iter().map(|&(a, b)| Box::from([a, b])).collect();
            let supports = if want && !sets.is_empty() {
                sp.supports(&sets, &stated_set)?
            } else {
                Vec::new()
            };
            // Each a node the layer read, so the cache answers.
            let nodes: Vec<FxHashSet<FactId>> = if check && !sets.is_empty() {
                match sp.eval(&sets)? {
                    Ok(read) => {
                        let states = sp.states();
                        read.iter()
                            .map(|s| states[s.class as usize].iter().copied().collect())
                            .collect()
                    }
                    Err(_) => Vec::new(),
                }
            } else {
                Vec::new()
            };
            Ok((l, read, supports, nodes))
        })?;

    // What the rule reads.
    let mut hypothesised: FxHashSet<Symbol> = root_layer
        .candidates
        .iter()
        .map(|c| subject(terms, c.fact))
        .collect();
    let declared = matches!(
        read.rung,
        crate::oblgen::Mode::Hrules | crate::oblgen::Mode::Obligations
    );
    let generator_reads = {
        let mut events = Events::off();
        let mut s = crate::saturator::Session {
            kb: root,
            terms,
            ast,
            events: &mut events,
            memo: crate::compile::SharedMemo::default(),
        };
        if declared {
            hypothesised.extend(crate::hypgen::eligible_relations(&mut s)?);
        }
        match read.rung {
            crate::oblgen::Mode::Hrules => crate::hrule::Hrules::new(&mut s)?
                .plans()
                .iter()
                .flat_map(|p| crate::monotone::reads_of(p, s.terms))
                .map(|(r, _)| r.name)
                .collect(),
            crate::oblgen::Mode::Obligations => read.obligation_reads,
            _ => FxHashSet::default(),
        }
    };
    let objects: FxHashSet<Symbol> = {
        let mut named: Vec<Symbol> = Vec::new();
        for c in &root_layer.candidates {
            names_of(terms, c.fact, &mut named);
        }
        // determinism-ok: builds a set; no order reaches it.
        for &rel in &hypothesised {
            for f in root.facts_of(rel) {
                names_of(terms, f, &mut named);
            }
        }
        named.into_iter().collect()
    };
    let view = Stated {
        root,
        terms,
        candidates: &root_layer.candidates,
        hypothesised,
        objects,
        read_absent: read.read_absent,
        declared,
        generator_reads,
    };

    // The rule, once a fact a file writes.
    let mut stated: Vec<Statement> = Vec::with_capacity(loaded.facts.len());
    for &(fact, prov) in &loaded.facts {
        let rel = terms.fact(fact).0;
        let (origin, verdict) = if rel == terms.kernel.written {
            (Origin::Written, Verdict::theory("written"))
        } else if rel == terms.kernel.relation {
            (
                Origin::Declaration,
                Verdict::theory("a declaration is kept"),
            )
        } else {
            (Origin::Fact, rule(&view, fact))
        };
        let record = prov.map(|p| terms.provs.get(p));
        stated.push(Statement {
            fact,
            text: sexpr(terms, fact),
            source: record
                .and_then(|p| p.source)
                .map(|s| terms.sym(s).to_string()),
            origin,
            kept: verdict.kept,
            clause: verdict.clause,
            read_absent: origin == Origin::Fact && view.read_absent.contains(&subject(terms, fact)),
        });
    }
    let rules_naming_objects: Vec<String> = {
        let mut rules: Vec<String> = crate::invariant::rule_constants(root, terms, ast)
            .into_iter()
            .filter(|c| c.which == "rule" && view.objects.contains(&c.name))
            .map(|c| terms.sym(c.rule).to_string())
            .collect();
        rules.sort();
        rules.dedup();
        rules
    };
    drop(view);
    let place: FxHashMap<FactId, u32> = stated
        .iter()
        .enumerate()
        .map(|(k, s)| (s.fact, k as u32))
        .collect();

    // The small KB, and its layer over root's universe.
    let kept: FxHashSet<FactId> = stated.iter().filter(|s| s.kept).map(|s| s.fact).collect();
    let mut small_kb = loaded.over(terms, &|f| kept.contains(&f));
    for s in stated.iter_mut().filter(|s| s.origin == Origin::Written) {
        s.kept = small_kb.contains(s.fact);
        s.clause = if s.kept {
            "written in a fact the small KB keeps, or in a rule"
        } else {
            "written only in facts the small KB drops"
        };
    }
    let small = with_space(
        &mut small_kb,
        terms,
        ast,
        &opts.layer,
        Some(&root_layer.candidates),
        |sp| {
            let licensed = sp.is_monotone();
            layer(sp, licensed)
        },
    );
    let small_beyond_root = match &small {
        Ok(_) => small_kb.facts().filter(|&f| !root.contains(f)).count() as u64,
        Err(_) => 0,
    };
    let mut against_root = Against::default();
    if let Ok(l) = &small
        && root_pairs
    {
        let (here, there) = (Deaths::of(l), Deaths::of(&root_layer));
        let n = l.n() as u32;
        against_root.singles = (0..n)
            .filter(|&a| here.single(a) && !there.single(a))
            .collect();
        for a in 0..n {
            for b in a + 1..n {
                if here.pair(a, b) && !there.pair(a, b) {
                    against_root.pairs.push((a, b));
                }
            }
        }
    }
    let small = small.map_err(|e| e.to_string());

    // The supports, as places in what is stated.
    let mut support_work = SupportWork::default();
    let candidate: FxHashMap<FactId, u32> = root_layer
        .candidates
        .iter()
        .enumerate()
        .map(|(b, c)| (c.fact, b as u32))
        .collect();
    let mut supports: Vec<Support> = supports
        .into_iter()
        .map(|s| {
            support_work.supports += 1;
            let Some(s) = s else {
                return Support::default();
            };
            let mut out = Support {
                read: s.read,
                rules: s.rules,
                status: Some(s.status),
                ..Support::default()
            };
            for f in s.frontier {
                if let Some(&k) = place.get(&f) {
                    out.stated.push(k);
                } else if let Some(&b) = candidate.get(&f) {
                    out.commitments.push(b);
                } else {
                    out.other.push(sexpr(terms, f));
                }
            }
            out.stated.sort_unstable();
            out.commitments.sort_unstable();
            out.other.sort();
            out
        })
        .collect();

    // The check: the program over each distinct support, with each of its
    // pairs.
    if check && nodes.len() == supports.len() {
        let mut by_support: BTreeMap<Vec<u32>, Vec<usize>> = BTreeMap::new();
        for (k, s) in supports.iter().enumerate() {
            if s.read {
                let dropped: Vec<u32> = s
                    .stated
                    .iter()
                    .copied()
                    .filter(|&k| !stated[k as usize].kept)
                    .collect();
                by_support.entry(dropped).or_default().push(k);
            }
        }
        // What every check has beside the small KB: the statements it
        // drops of a relation a rule reads the absence of.
        let whole: FxHashSet<FactId> = stated
            .iter()
            .filter(|s| s.read_absent && !s.kept)
            .map(|s| s.fact)
            .collect();
        for (set, places) in by_support {
            let named: FxHashSet<FactId> = set.iter().map(|&k| stated[k as usize].fact).collect();
            let mut kb = loaded.over(terms, &|f| {
                kept.contains(&f) || named.contains(&f) || whole.contains(&f)
            });
            let sets: Vec<Box<[u32]>> = places
                .iter()
                .map(|&k| {
                    let (a, b) = root_layer.excluded[k];
                    Box::from([a, b])
                })
                .collect();
            let got = with_space(
                &mut kb,
                terms,
                ast,
                &opts.layer,
                Some(&root_layer.candidates),
                |sp| {
                    if sp.root_is_dead() {
                        return Ok((Ok(None), sp.work()));
                    }
                    // Each node as (dead, its facts beyond its own root).
                    let seen = sp.eval(&sets)?.map(|seen| {
                        let states = sp.states();
                        let read: Vec<(bool, Vec<FactId>)> = seen
                            .iter()
                            .map(|s| (s.dead, states[s.class as usize].to_vec()))
                            .collect();
                        Some(read)
                    });
                    Ok((seen, sp.work()))
                },
            );
            support_work.check_roots += 1;
            let unread = |why: String| -> Vec<Check> {
                vec![
                    Check {
                        dead: false,
                        root_dead: false,
                        beyond: 0,
                        unread: Some(why),
                    };
                    places.len()
                ]
            };
            let checks: Vec<Check> = match got {
                Err(e) => unread(e.to_string()),
                Ok((seen, (saturations, facts))) => {
                    support_work.check_saturations += saturations;
                    support_work.check_facts += facts;
                    // What the check's own root holds that the program's
                    // root does not, and so no node of the program's need.
                    let at_root: Vec<FactId> = kb.facts().filter(|&f| !root.contains(f)).collect();
                    let beyond = |k: usize, own: &[FactId]| -> u64 {
                        at_root
                            .iter()
                            .chain(own.iter().filter(|&&f| !root.contains(f)))
                            .filter(|f| !nodes[k].contains(f))
                            .count() as u64
                    };
                    match seen {
                        Ok(None) => places
                            .iter()
                            .map(|&k| Check {
                                dead: true,
                                root_dead: true,
                                beyond: beyond(k, &[]),
                                unread: None,
                            })
                            .collect(),
                        Ok(Some(read)) => places
                            .iter()
                            .zip(read)
                            .map(|(&k, (dead, own))| Check {
                                dead,
                                root_dead: false,
                                beyond: beyond(k, &own),
                                unread: None,
                            })
                            .collect(),
                        Err(stop) => unread(format!("{stop:?}")),
                    }
                }
            };
            for (k, c) in places.into_iter().zip(checks) {
                supports[k].check = Some(c);
            }
        }
    }

    Ok(Theory {
        root: root_layer,
        stated,
        small,
        small_beyond_root,
        against_root,
        supports,
        support_work,
        rung: read.rung.as_str(),
        rules_naming_objects,
    })
}

/// Root's layer read as far as its universe: the candidates, the static
/// verdict and whether root is dead, with every single and every pair left
/// and the stop said to be by asking.
fn universe_alone(sp: &Space<'_>) -> PairLayer {
    let n = sp.n() as u64;
    PairLayer {
        candidates: sp.candidates().to_vec(),
        monotone: sp.is_monotone(),
        root_dead: sp.root_is_dead(),
        singles_left: n,
        at_root: Vec::new(),
        dead_alone: Vec::new(),
        implies: Vec::new(),
        equivalent: Vec::new(),
        excluded: Vec::new(),
        interacting: Vec::new(),
        against_l1: Vec::new(),
        against_l3: Vec::new(),
        pairs: Pairs {
            all: n * n.saturating_sub(1) / 2,
            left: n * n.saturating_sub(1) / 2,
            ..Pairs::default()
        },
        licences: Vec::new(),
        work: Work::default(),
        stopped: Some(Stopped::NoSearch),
    }
}

/// **The rule for the small KB**: which of the facts a file writes are the
/// puzzle's. It reads the program as loaded and as root leaves it, and no
/// relation by what it is called.
///
/// The small KB keeps every rule, every declaration and every import, and of
/// the facts a file writes it drops **(a) a fact of a candidates' relation**,
/// stated or denied ([`Stated::hypothesised`]): a given. Of the others:
///
/// - where **the program's own rung built `H`** ([`Stated::declared`]), the
///   relations that rung reads are what the universe is made of, and a fact
///   of one is kept ([`Stated::generator_reads`]): it says what kind of
///   thing an object is. **(b) Every other fact that names one of the
///   candidates' objects** ([`Stated::objects`]) is dropped: it says
///   something of that object which the universe does not need;
/// - where **the blind enumerator built `H`**, nothing says what the
///   universe is made of, and the rule drops less: **(c) a fact that names a
///   candidates' relation together with one of the candidates' objects**,
///   which speaks of that relation's facts at one of its arguments, and
///   **(d) a fact that names two or more of the candidates' objects**, which
///   relates them. A fact that names one object and no candidates' relation
///   is kept, as a type is.
///
/// A fact that names none of the candidates' objects is kept under either:
/// a declaration that names a relation and no object speaks of the relation
/// whole.
pub fn small_kb(p: &Stated<'_>, fact: FactId) -> Verdict {
    let rel = p.relation(fact);
    if p.hypothesised.contains(&rel) {
        return Verdict::puzzle("(a) a fact of a candidates' relation");
    }
    let names = p.names(fact);
    let objects = names.iter().filter(|n| p.objects.contains(n)).count();
    if objects == 0 {
        return Verdict::theory("names none of the candidates' objects");
    }
    if p.declared {
        if p.generator_reads.contains(&rel) {
            return Verdict::theory("a fact of a relation the rung that built H reads");
        }
        return Verdict::puzzle("(b) names one of the candidates' objects");
    }
    if names.iter().any(|n| p.hypothesised.contains(n)) {
        return Verdict::puzzle(
            "(c) names a candidates' relation and one of the candidates' objects",
        );
    }
    if objects >= 2 {
        return Verdict::puzzle("(d) names two or more of the candidates' objects");
    }
    Verdict::theory("names one of the candidates' objects and no candidates' relation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{load, parse};

    fn layer_of(rel: &str, licensed: bool) -> PairLayer {
        let (mut kb, mut terms, ast) = fresh(rel);
        with_space(&mut kb, &mut terms, &ast, &Options::default(), None, |sp| {
            layer(sp, licensed)
        })
        .expect("the layer")
    }

    fn fresh(rel: &str) -> (Kb, Terms, Ast) {
        let path = ein_corpus::repo_root().join(rel);
        let text = std::fs::read_to_string(&path).expect("the program");
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
        (kb, terms, ast)
    }

    /// Programs that state a negation, nest a fact in a fact, write a
    /// numeral in a rule's conclusion, import a module that states facts,
    /// and state nothing at all beyond declarations.
    const SHAPES: [&str; 6] = [
        "examples/zebra2-hints.ein",
        "examples/features/05_stdlib_domain_elim.ein",
        "examples/beyond_arith/equation_two_unknowns.ein",
        "examples/beyond_arith/cryptarithm.ein",
        "examples/arith/add.ein",
        "examples/lattice/02_genuine_3set_death.ein",
    ];

    /// **The program over every fact it states is the program**: the same
    /// facts in the same order, `(written …)` among them, each with a record
    /// of the same step from the same premises.
    #[test]
    fn the_program_over_all_its_facts_is_the_program() {
        for rel in SHAPES {
            let (kb, mut terms, ast) = fresh(rel);
            let loaded = Loaded::of(&kb, &mut terms, &ast).expect("loaded");
            let again = loaded.over(&mut terms, &|_| true);
            let facts = |kb: &Kb| -> Vec<FactId> { kb.facts().collect() };
            assert_eq!(facts(&again), facts(&kb), "{rel}");
            for f in kb.facts() {
                let record = |kb: &Kb| {
                    kb.primary(f).map(|p| {
                        let p = terms.provs.get(p);
                        (p.kind, p.rule, p.premises.clone(), p.source)
                    })
                };
                assert_eq!(record(&again), record(&kb), "{rel}: {}", sexpr(&terms, f));
            }
            // And over none of them, the declarations' and the rules' own
            // statements are left.
            let none = loaded.over(&mut terms, &|_| false);
            assert!(
                none.facts().all(|f| {
                    let p = none.primary(f).map(|p| terms.provs.get(p));
                    terms.fact(f).0 == terms.kernel.written
                        && p.is_some_and(|p| p.premises.is_empty())
                }),
                "{rel}"
            );
        }
    }

    /// **A small KB that keeps every statement decides what the program
    /// decides**: its layer over root's universe is root's layer, list for
    /// list, on a monotone program and on a flagged one. Root's universe is
    /// then named to a second root and not generated by it, and the nodes
    /// are the same.
    #[test]
    fn a_small_kb_that_keeps_everything_is_the_program() {
        for rel in [
            "examples/zebra2-hints.ein",
            "examples/branching/08_hypothesis_relation_whitelist.ein",
            "examples/beyond_arith/cryptarithm.ein",
            "examples/saturation/square-unique/cul-de-sac.ein",
        ] {
            let (mut kb, mut terms, ast) = fresh(rel);
            let t = theory_by(
                &mut kb,
                &mut terms,
                &ast,
                &TheoryOptions::default(),
                &|_, _| Verdict::theory("everything"),
            )
            .expect("the theory's share");
            let small = t.small.as_ref().expect("a layer");
            let said = |l: &PairLayer| {
                format!(
                    "{:?}",
                    (
                        (&l.at_root, &l.dead_alone, &l.implies, &l.equivalent),
                        (&l.excluded, &l.interacting, &l.against_l1, &l.against_l3),
                        (l.monotone, l.root_dead, l.pairs, &l.licences, l.work),
                    )
                )
            };
            assert!(t.root.pairs.saturated > 0, "{rel}");
            assert_eq!(said(small), said(&t.root), "{rel}");
            assert!(t.dropped().is_empty() && t.against_root.is_empty(), "{rel}");
            assert_eq!(t.small_beyond_root, 0, "{rel}");
        }
    }

    /// **A support follows root's activators and a core does not.** A pair
    /// two clues exclude is dead by a rule whose plan an activator the
    /// program states compiled: the support names that statement, and the
    /// core of the same node names commitments and givens alone.
    #[test]
    fn a_support_names_the_statements_a_core_leaves_to_the_program() {
        let (mut kb, mut terms, ast) = fresh("examples/zebra2-hints.ein");
        let opts = TheoryOptions {
            supports: true,
            ..TheoryOptions::default()
        };
        let t = theory(&mut kb, &mut terms, &ast, &opts).expect("the theory's share");
        let of_theory = t.of_the_theory();
        let puzzle: Vec<&Support> = t
            .supports
            .iter()
            .zip(&of_theory)
            .filter(|&(_, &theory)| !theory)
            .map(|(s, _)| s)
            .collect();
        assert!(!puzzle.is_empty());
        for s in &puzzle {
            assert!(s.read && s.other.is_empty());
            let dropped: Vec<&Statement> = s
                .stated
                .iter()
                .map(|&k| &t.stated[k as usize])
                .filter(|x| !x.kept)
                .collect();
            assert!(
                !dropped.is_empty(),
                "a pair of the puzzle names a statement of it"
            );
        }
        // Some support names an activator, which no core holds.
        let activators = puzzle
            .iter()
            .flat_map(|s| &s.stated)
            .filter(|&&k| {
                let rel = terms.fact(t.stated[k as usize].fact).0;
                kb.program().rules.contains(rel)
            })
            .count();
        assert!(activators > 0, "no support names an activator");
    }

    /// **The licences decide a pair as its node does.** A monotone program's
    /// layer read with no licence saturates every pair, finds none alive over
    /// a member dead alone and none that is not the node of the member that
    /// implies, and lists the excluded and the interacting pairs the licensed
    /// layer lists.
    #[test]
    fn the_licences_decide_a_pair_as_its_node_does() {
        for rel in [
            "examples/branching/02_one_dead_one_alive.ein",
            "examples/branching/08_hypothesis_relation_whitelist.ein",
            "examples/zebra2-hints.ein",
        ] {
            let with = layer_of(rel, true);
            let without = layer_of(rel, false);
            assert!(
                with.monotone && with.complete() && without.complete(),
                "{rel}"
            );
            assert!(
                with.pairs.dead_member > 0 && with.pairs.implied > 0,
                "{rel}"
            );
            assert_eq!(without.pairs.saturated, without.pairs.all, "{rel}");
            assert!(without.licences.is_empty(), "{rel}");
            assert_eq!(without.against_l1, [], "{rel}: against L1");
            assert_eq!(without.against_l3, [], "{rel}: against L3");
            assert_eq!(with.excluded, without.excluded, "{rel}");
            assert_eq!(with.interacting, without.interacting, "{rel}");
            assert_eq!(with.dead_alone, without.dead_alone, "{rel}");
            assert_eq!(with.implies, without.implies, "{rel}");
        }
    }
}
