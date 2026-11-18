//! The structure of the hypothesis set `H`, read from the program's own
//! lattice, with nobody declaring it.
//!
//! Every object here is a statement about nodes [`crate::region`] defines:
//! root's fixpoint with a commitment written in and saturated, by the node
//! function every route and the oracle share, reached through
//! [`crate::explore`]'s node cache (`Space::eval`). So each is true of the
//! lattice `R` is a region of, and none is a second semantics.
//!
//! ### `H`, listed
//!
//! [`hypotheses`] is the hypothesis set itself: the candidates root's
//! generator proposes at root's fixpoint, on whichever rung the program is
//! on, in the universe's canonical order, rendered-text order. It is the set
//! every route searches over, and what it costs is what building `H` costs:
//! root's saturation and the generator's pass, no node. **Every object below
//! is over these candidates, taken by the same function**: a pair layer, the
//! theory's share and [`whole`] open their space over root prepared by it,
//! so a listing of `H` and a structure's candidates cannot disagree.
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
//! the facts a file writes those a rule keeps ([`small_kb`]). It is built over the same
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
//!
//! ### The cover: *pick at most one*
//!
//! [`Cover`] is `H` as the groups the theory makes of it. **A group is a
//! maximal set of two or more candidates that pairwise exclude by the
//! theory alone**: a maximal clique of the graph whose vertices are the
//! candidates of `H`, all of them, and whose edges are the pairs
//! [`Theory::small`] excludes. A state holds at most one member of a group.
//! The groups overlap, a candidate of a bijection being in a row and in a
//! column, so they are a **cover** of the candidates they hold and no
//! partition; and a candidate no pair of the theory holds is in no group
//! ([`Cover::in_no_group`]), which says that no group says anything of it.
//!
//! **The graph is the theory's and not root's.** At root a pair two
//! declarations exclude and a pair two clues exclude are one thing, and
//! once the clues have decided part of a puzzle some candidate outside a
//! row excludes every member of it: the row is a clique and not a maximal
//! one. The maximal cliques of root's exclusion graph are kept beside the
//! groups ([`Cover::root_cliques`]), as what the clues add, and are not
//! called groups.
//!
//! **A candidate dead alone at root is a member like any other**: the
//! small KB drops what killed it, so it has its row and its column there.
//! Each candidate carries what root says of it alone ([`Mark`]), and a
//! group's **open** members are those alive alone at root
//! ([`Cover::open`]). A candidate the theory itself kills alone
//! ([`Cover::killed_alone`]) is in no pair of it and so in no group.
//!
//! **No relation is read to form a group**: not what it is called, not its
//! arguments, not a declaration of it. What a program declares stands
//! beside the cover, and [`Cover::standing`] reads any set of candidates
//! against it: an owed instance's alternatives are a group, are strictly
//! inside one, or do not pairwise exclude.
//!
//! **What *at most one* rests on.** A group's pairs are dead in the small
//! KB. They are dead at root, and so at every node above it on a monotone
//! program, where the small KB derives no more than the program:
//! [`Cover::alive_at_root`] lists the pairs of which that fails, and is
//! empty where the small KB's rule fits the program. On a program the
//! static check flags nothing above root follows ([`Cover::domain`]).
//!
//! **The enumeration is bounded** by [`CLIQUE_STEPS`], 2^20 calls of the
//! search for the cliques of one graph, a constant of this module and no
//! lever: no `(config …)` field, no option and no environment name reaches
//! it. A graph the search stops on has the cliques found so far, each
//! maximal, and says what the bound cut ([`Cut`]): the excluded pairs
//! inside no clique listed, and the candidates of such a pair that no
//! clique listed holds.
//!
//! **The order is canonical.** A group is its members ascending, which is
//! the universe's rendered-text order, and the groups are in the order of
//! those sequences. So a cover the bound did not cut is a function of the
//! program: the same under a permutation of the interner's ids and at any
//! job count, and, as sets of candidates with the module prefixes taken
//! off, under every import form
//! (`ein-infer/tests/structure_invariance.rs`).
//!
//! **Once a program, from its prepared space.** `cover_in` takes the
//! space a route has open and the program as it was loaded (`Loaded::of`,
//! taken before root is saturated, since a prepared space holds root's
//! fixpoint and not what the files wrote). Root's singles and pairs are
//! then the route's own nodes, and the small KB's are read in a space of
//! their own. [`cover`] is the same for a reader that has only the freshly
//! loaded program. No route reads the cover.
//!
//! ### Exactly one: owed groups and grids
//!
//! A group says *at most one*. *Exactly one* needs a *must*, and the engine
//! has one source of it, an **obligation**: a state that owes an instance
//! is not in `R` until a fact witnesses it, and L5's table
//! ([`crate::oblgen::Alternatives`]) reads at root the candidates that can
//! witness each instance root owes, where its premise holds of the
//! instance. [`Owed`] is that table with every instance's witnesses
//! beside, and [`ExactlyOne`] reads it against the cover:
//!
//! - **an owed group** is a group whose members are exactly the
//!   alternatives of an instance L5 licenses ([`Owing::Group`]). Every fact
//!   that can witness the instance is then a member or denied at root, so a
//!   state of `R`, which has paid the instance, holds a member, and holds
//!   no two. A member dead alone at root is an alternative like any other
//!   and is in no alive state, so **every state of `R` holds exactly one of
//!   the group's open members**. That rests on L5's premise, which the
//!   obligation tree checks at every node it reads, and on the group's
//!   pairs being dead at root ([`Cover::alive_at_root`]);
//! - **an owed set that is not a group** is a licensed instance whose
//!   alternatives do not pairwise exclude ([`Owing::NotExcluding`]): *at
//!   least one*;
//! - **a group nothing owes** has no such instance ([`ExactlyOne::unowed`]):
//!   *at most one*, and nothing the engine reads says a member must hold;
//! - **an instance L5 does not license** has no alternatives, only the
//!   reason. Its witnesses are read all the same ([`Witness`]): which is a
//!   candidate, which root denies, and for each the candidates whose own
//!   node holds it, at root and in the small KB. Where the candidates that
//!   bring a witness by the theory alone are exactly a group's members, and
//!   every witness root does not deny is so brought, the instance is
//!   **about** that group ([`Owing::Unlicensed`]). The group is not owed by
//!   it: nothing is concluded from a witness a candidate brings.
//!
//! **A grid** is found from the groups and their members alone
//! ([`Cover::grids`]): a connected component of the theory's exclusion
//! graph in which every candidate is in exactly two groups, the groups are
//! two families no two groups of one of which share a member, and two
//! groups share at most one member. Each family then holds every member of
//! the component once, as rows and columns do, and a member is one cell. No
//! relation is read: one relation's candidates may be several grids, and a
//! component that is no grid says why ([`NotAGrid`]). The grid is over all
//! of the component's candidates, those dead alone at root among them, as
//! a group is.
//!
//! **A bijection** is a grid with as many groups in one family as in the
//! other, every one of them owed ([`ExactlyOne::is_bijection`]): every
//! state of `R` holds exactly one member of each row and of each column.
//! Where one family is owed whole and the other is not, or the families
//! differ in number, [`ExactlyOne::share`] says how many groups each family
//! has and how many of them are owed, which is what a count would read;
//! **nothing here concludes *exactly one*, or anything else, from a
//! count**, and a group nothing owes stays *at most one*.
//!
//! **Nothing is synthesised.** No rule, obligation or closure is added, the
//! table is not handed to the node function, and no route reads any of it.
//!
//! ### Coupling: what ties the questions
//!
//! A group says which candidates answer one question. [`Coupling`] says
//! which questions bear on each other. **A question** is a group of the
//! cover, or a candidate in no group ([`Question`]): *which of these, if
//! any*, and *this one, or not*. **A coupling is a set of two or more
//! questions that bear on each other**: some conflict or some derivation
//! needs a member of each of them, or some debt needs a member of one of
//! them. With the questions as nodes it is a hyperedge ([`Edge`]), which
//! says which of the two it is ([`EdgeKind`]), and the result is the
//! questions' hypergraph: a constraint graph, with what each edge was read
//! from and the statements or the instance it rests on.
//!
//! **Which questions a conflict is about is not in the candidates it
//! reads.** A candidate of a grid is in two groups: `(val D 5)` is a digit
//! of `D` and a letter that is `5`. So one **way** to a conclusion, the
//! candidates one derivation of it reads, has several **descriptions**, a
//! question of each candidate, and the edges are a **cover** of the ways by
//! descriptions: the class of questions that describes the most ways
//! first, a class of one question before any other since a way one
//! question describes is that question's own, and classes that stand
//! equal chosen together. That is what makes a column of a sum the letters
//! it adds and not the digits they may be: every way to the column's
//! contradiction reads a digit of each of its letters, and no two of them
//! the same three digits.
//!
//! **Two things are read.** From root's pair layer, the **ties**
//! ([`Tie`]): the pairs of two candidates alive alone and in no question
//! together that root excludes, of which one implies the other, or that
//! interact. And **the whole node**: root with every candidate alive alone
//! committed, one fork, dead at once and saturated to quiescence all the
//! same, read with every rule application that reached it, redundant ones
//! among them, each with its premises, what its universals read and the
//! activator its plan was compiled from, and beside them every
//! justification the store kept. On a program the static check calls
//! monotone an application made at a node of alive candidates is made at
//! the whole node too, so every derivation of every node is there.
//!
//! **A derived fact's label is one class of questions**: the union of the
//! classes the cover of its ways chose, a way being a choice, for each
//! premise beyond root, of the candidate it is or of its label. The labels
//! are settled in rounds from none, each round reading every fact from the
//! round before, so they do not follow the order the applications came
//! in. A way that reads two candidates no alive state holds together, two
//! of one question or a pair root excludes, is no state's and is not read,
//! but for the pair's own contradiction. The node's edges are then the
//! cover of its ways to a contradiction, an application that concludes
//! `(false)` or a fact beside its negation; the cover of its ways to each
//! committed candidate; and every other fact's label ([`NodeTies`]).
//!
//! **An edge names the puzzle's statements it rests on** ([`Edge::stated`]):
//! of a tie, where supports are asked, those of its death's support; of the
//! whole node, those the last application of each way reads, directly or
//! through the derivation root recorded first for a premise root holds,
//! its activator among them. A statement is the puzzle's where the small
//! KB drops it.
//!
//! **A debt is an edge of its own kind** ([`Debt`]). `R` is the
//! commitments alive and discharged, and an instance root owes ties its
//! payers there as a conflict does: a state of `R` holds one of them. The
//! instances are L5's table's, and one whose payers alive alone lie in two
//! questions or more is an edge of their questions, those that hold the
//! most payers first, its reason the instance. One whose payers are all in
//! one question is no edge: it is that question's own, an owed group's
//! *exactly one*. **Whether L5 licenses the instance does not decide it**:
//! the licence says whether a search may confine itself to the
//! alternatives, not whether root owes, and `R` is discharged either way.
//! A licensed instance's payers are its alternatives. An unlicensed one's
//! are **the payers read**, the candidates whose own node holds one of its
//! witnesses, and the edge says which it is ([`Debt::licensed`]). They are
//! read off root's singles, which a layer that read its singles has; where
//! it stopped before them no unlicensed instance is an edge, and the
//! coupling says so ([`Coupling::payers_read`]) and is not complete.
//!
//! **The components** ([`Coupling::components`]) are of the candidates
//! alive alone: two are together where they share a question or an edge
//! holds a question of each. They are what `R` is the product of its parts
//! over. The components over the conflicts' and the derivations' edges
//! alone stand beside ([`Coupling::conflict_components`]): what no debt
//! joins, and where a debt's payers lie in two of them
//! ([`Coupling::debts_across`]) `R` is no product over those.
//!
//! **What it rests on, and what it does not tell.**
//! `ein-infer/tests/coupling.rs` holds it to a describe wherever one
//! exhausts: every member of `N` lies inside one edge of a conflict or a
//! derivation, and `R` is that product. Past that:
//!
//! - **it may tie more than any state does.** A way through a derived fact
//!   does not know which members of a question the fact was derived from,
//!   so two ways no one alive state makes together are read as one;
//! - **it is coarse where a fact has ways of two kinds.** The label is
//!   their union, since labels kept apart do not settle on a program whose
//!   derivations go round. Where every clue propagates into every other,
//!   as on a grid of bijections, the node's edges come to one edge of every
//!   question the clues reach, and say what the components say. The edges
//!   read one application back are kept beside them
//!   ([`Coupling::one_step`]), off the same node: the finer ones that edge
//!   absorbs;
//! - **an unlicensed debt's edge may be too small.** Its payers are only
//!   those a single candidate's node shows: a witness that needs two
//!   candidates together is not seen. Components too coarse claim no
//!   independence that is false, and components too fine would; nothing
//!   follows for a search from such an edge beyond *do not take these
//!   questions for independent*;
//! - **a tie is of root as the clues leave it.** Where root has applied
//!   them the pairs it excludes are their joint consequences, and an edge
//!   of the pair layer is no one statement's;
//! - **the readings short of the whole node** ([`CouplingReading`]) miss
//!   what they are named for: the pairs alone every conflict of three, the
//!   node read without following a derived premise every conflict through
//!   one, and one application back every conflict two away;
//! - on a program the static check flags nothing above the nodes read
//!   follows ([`Coupling::domain`]).
//!
//! **What it costs** is root's pair layer, the small KB's, and one
//! saturation ([`NodeWork`]). The node is no node of the routes' cache and
//! is charged to no budget; it is not read where its facts pass the
//! layer's budget or its applications [`NODE_APPLICATIONS`]
//! ([`NodeStop`]). The bounds, that one, [`ROUNDS`], [`WAYS`] and
//! [`DESCRIPTIONS`], are constants of the module and no lever, and
//! [`CouplingCut`] says what each cut. `coupling_in` reads it over a
//! route's open space, as `cover_in` reads the cover. No route reads it.

use std::collections::BTreeMap;
use std::time::Instant;

use ein_core::entities::NameCategory;
use ein_core::{FactId, Kb, Program, ProvId, Symbol, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::downset;
use crate::error::SolveError;
use crate::events::{Events, sexpr};
use crate::explain::CoreStatus;
use crate::explore::{self, Ask, Licence, Mode, Seen, Space, Stopped, Which};
use crate::region::{self, Candidate, Cores};

mod coupling;
// The entry a route takes, beside `cover_in` and `exactly_one_in`.
#[allow(unused_imports)]
pub(crate) use coupling::coupling_in;
pub use coupling::{
    COUPLING_FLAGGED, Coupling, CouplingOptions, CouplingReading, Cut as CouplingCut, DESCRIPTIONS,
    Debt, Edge, EdgeKind, NODE_APPLICATIONS, NodeStop, NodeTies, NodeWork, PairTies, Parts,
    Question, ROUNDS, Tie, TieKind, WAYS, Whole, WholeOptions, coupling, theory_coupled, whole,
};
// The coupling a route reads while it runs: the whole node alone.
pub(crate) use coupling::node_coupling_in;
pub use coupling::{NodeCoupling, node_coupling};

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

/// [`pair_layer`], with every node the layer computes handed to `events`'
/// watch ([`crate::events::Watch`]) as a route's nodes are: root's node,
/// each single's and each pair's the licences leave, in the order read, each
/// with its closure and its facts beyond root. For a reader that prices the
/// layer node by node. The layer is the one [`pair_layer`] returns: a watch
/// sees a node and changes none.
pub fn pair_layer_with(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    events: &mut Events,
) -> Result<PairLayer, SolveError> {
    with_space_watched(root, terms, ast, opts, None, events, pair_layer_in)
}

/// **Root prepared, and with it `H`**: `root`, a freshly loaded program,
/// saturated once, and the universe its generator builds at that fixpoint in
/// canonical order, by the preparation every route starts from
/// ([`region::prepare_with`]). The one place this module takes `H` from:
/// [`hypotheses`] returns its candidates, and every space a structure is
/// read in is opened over it.
fn root_prepared(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
) -> Result<region::Prepared, SolveError> {
    region::prepare_with(root, terms, ast, events)
}

/// **The hypothesis set `H` of a program, as root builds it.**
#[derive(Clone, Debug)]
pub struct Hypotheses {
    /// The universe: every candidate the generator proposes at root's
    /// fixpoint, once, in rendered-text order. The order a [`PairLayer`]'s
    /// candidates are in, and every other object's.
    pub candidates: Vec<Candidate>,
    /// The rung of the ladder that built it.
    pub rung: crate::oblgen::Mode,
    /// Root's own fixpoint derives `(false)`. `H` is built all the same.
    pub root_dead: bool,
}

/// [`Hypotheses`] of `root`, a **freshly loaded, unsaturated** program
/// nothing else will use. Root is saturated once, the generator is run at
/// its fixpoint, and **no node is read**: no single, no pair, no small KB.
/// A program the engine refuses to read is refused here as it is by a route.
pub fn hypotheses(root: &mut Kb, terms: &mut Terms, ast: &Ast) -> Result<Hypotheses, SolveError> {
    hypotheses_with(root, terms, ast, &mut Events::off())
}

/// [`hypotheses`], root's saturation and the generator's pass narrated into
/// `events` as a route narrates them: for a reader that counts the work.
pub fn hypotheses_with(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
) -> Result<Hypotheses, SolveError> {
    let prepared = root_prepared(root, terms, ast, events)?;
    Ok(Hypotheses {
        rung: prepared.root.hypgen.rung.mode,
        root_dead: prepared.root_dead,
        candidates: prepared.candidates,
    })
}

/// `read` over the prepared space of `root`, a freshly loaded program: the
/// routes' node cache, at the layer's budget, keeping no node's record. Over
/// the universe root's generator builds ([`root_prepared`]), or over the one
/// the caller names ([`region::prepare_over`]).
fn with_space<R>(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    universe: Option<&[Candidate]>,
    read: impl FnOnce(&mut Space<'_>) -> Result<R, SolveError>,
) -> Result<R, SolveError> {
    with_space_watched(root, terms, ast, opts, universe, &mut Events::off(), read)
}

/// [`with_space`], the space's nodes narrated into `events`.
fn with_space_watched<R>(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    universe: Option<&[Candidate]>,
    events: &mut Events,
    read: impl FnOnce(&mut Space<'_>) -> Result<R, SolveError>,
) -> Result<R, SolveError> {
    let prepared = match universe {
        None => root_prepared(root, terms, ast, &mut Events::off())?,
        Some(named) => region::prepare_over(root, terms, ast, named)?,
    };
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
    let mut sp = Space::open(root, terms, ast, prepared, &space, events);
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
    layer(sp, licensed, true)
}

/// The layer over `sp`, L1 and L3 deciding the pairs they can when
/// `licensed` and every pair saturated otherwise. Without `pairs` it is read
/// as far as its singles: root's node and every candidate's, the
/// implications and the classes, and no pair's node, which the layer says
/// it stopped before by asking ([`Stopped::NoSearch`]).
fn layer(sp: &mut Space<'_>, licensed: bool, pairs: bool) -> Result<PairLayer, SolveError> {
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
    read(sp, &mut out, licensed, pairs)?;
    let (s, f) = sp.work();
    out.work = Work {
        saturations: s - saturations,
        facts: f - facts,
    };
    Ok(out)
}

fn read(
    sp: &mut Space<'_>,
    out: &mut PairLayer,
    licensed: bool,
    with_pairs: bool,
) -> Result<(), SolveError> {
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
    if !with_pairs {
        // The singles alone: the licences decide what they decide, and no
        // pair's node is read.
        for (a, b) in pairs.by_ref() {
            match before(&singles, a, b) {
                Before::DeadMember if licensed => {
                    out.pairs.dead_member += 1;
                    skipped[0] = true;
                }
                Before::Implied if licensed => {
                    out.pairs.implied += 1;
                    skipped[1] = true;
                }
                _ => out.pairs.left += 1,
            }
        }
        out.stopped = Some(Stopped::NoSearch);
    }
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
}

impl Origin {
    pub fn as_str(self) -> &'static str {
        match self {
            Origin::Fact => "fact",
            Origin::Declaration => "declaration",
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
    /// with neither an hrule nor an obligation activated at root, or with an
    /// obligation the ladder declines, falls to.
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
pub fn subject(terms: &Terms, fact: FactId) -> Symbol {
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
pub fn names_of(terms: &Terms, fact: FactId, out: &mut Vec<Symbol>) {
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
/// of its facts from. A prepared space holds root's fixpoint and not the
/// facts the files wrote, so a reader that wants the small KB of a program
/// it is about to prepare takes this first.
pub(crate) struct Loaded {
    /// The registries, copied, with nothing computed from them.
    program: Program,
    /// Every fact the loaded program holds, in the order the loader stored
    /// them, each with its record.
    facts: Vec<(FactId, Option<ProvId>)>,
}

impl Loaded {
    /// Of `root`, **before** anything saturates it.
    pub(crate) fn of(root: &Kb) -> Loaded {
        let mut program = root.program().clone();
        program.domain.take();
        Loaded {
            program,
            facts: root.facts().map(|f| (f, root.primary(f))).collect(),
        }
    }

    /// **The program over the facts `keep` keeps**: the loaded program's
    /// registries, every rule, declaration and import among them, and of
    /// its facts those `keep` says, in the order written, each with its own
    /// record. The KB is what loading the program's text with the other
    /// facts deleted gives, unsaturated, over the same [`Terms`]: a fact of
    /// either is the same [`FactId`] in both.
    fn over(&self, terms: &mut Terms, keep: &dyn Fn(FactId) -> bool) -> Kb {
        let mut kb = Kb::new(self.program.clone());
        for &(f, prov) in &self.facts {
            if !keep(f) {
                continue;
            }
            let (rel, args) = terms.fact(f);
            let args = args.to_vec();
            kb.add_fact(terms, rel, &args, prov)
                .expect("a fact the program holds");
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
    /// What root owes, read beside the two layers: L5's table, each
    /// instance with the facts that can witness it and the candidates whose
    /// own node holds each, at root and in the small KB.
    pub owed: Owed,
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
/// excludes. `rule` is asked once about each fact a file writes, and a
/// declaration's fact is kept.
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
    let loaded = Loaded::of(root);
    let reading = if opts.root_pairs {
        Reading::Pairs
    } else {
        Reading::Universe
    };
    with_space(root, terms, ast, &opts.layer, None, |sp| {
        theory_in(sp, &loaded, reading, opts, rule)
    })
}

/// How far root's own layer is read beside the small KB's.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reading {
    /// As far as its universe: no node.
    Universe,
    /// Root's node and every single's.
    Singles,
    /// The whole layer.
    Pairs,
}

/// [`theory_by`] over **the program's prepared space** and the program as
/// it was loaded: root's layer is read through `sp`, the routes' node
/// cache, as far as `reading` says, so a node the space has computed is not
/// saturated again and the layer's nodes are the space's from then on; the
/// small KB is built over the space's own terms and read in a space of its
/// own, at `opts.layer`'s budget. `loaded` is [`Loaded::of`] the same
/// program, taken before its root was saturated.
fn theory_in(
    sp: &mut Space<'_>,
    loaded: &Loaded,
    reading: Reading,
    opts: &TheoryOptions,
    rule: &dyn Fn(&Stated<'_>, FactId) -> Verdict,
) -> Result<Theory, SolveError> {
    let stated_set: FxHashSet<FactId> = loaded.facts.iter().map(|&(f, _)| f).collect();

    // Root's layer, what only its space holds, the supports, and for a
    // check each excluded pair's own node.
    let root_pairs = reading == Reading::Pairs;
    let (want, check) = (
        root_pairs && opts.supports,
        root_pairs && opts.supports && opts.check,
    );
    let (root_layer, read, supports, nodes, owing, root_owes, held_at_root) = {
        // What root owes, read before any node, as a route reads L5's
        // table: a reason of the table's names a fact only where something
        // has interned it, and a node interns what it derives.
        let owing = sp.owed()?;
        let l = match reading {
            Reading::Pairs => pair_layer_in(sp)?,
            Reading::Singles => {
                let licensed = sp.is_monotone();
                layer(sp, licensed, false)?
            }
            Reading::Universe => universe_alone(sp),
        };
        let read = RootRead::of(sp);
        // Which singles hold each fact that can witness an instance: off
        // the cache, which has the singles where the layer read them.
        let held_at_root = owing.as_ref().and_then(|(_, rows)| holders(sp, rows));
        let sets: Vec<Box<[u32]>> = l.excluded.iter().map(|&(a, b)| Box::from([a, b])).collect();
        let supports = if want && !sets.is_empty() {
            sp.supports(&sets, &stated_set)?
        } else {
            Vec::new()
        };
        // Each a node the layer read, off the cache, which asks no budget: a
        // layer whose last step carried it past its budget has read them
        // all the same. `None` where one was never computed, and then each
        // check is unread and says so.
        let nodes: Option<Vec<FxHashSet<FactId>>> = if check && !sets.is_empty() {
            sp.cached(&sets).map(|read| {
                let states = sp.states();
                read.iter()
                    .map(|s| states[s.class as usize].iter().copied().collect())
                    .collect()
            })
        } else {
            Some(Vec::new())
        };
        (
            l,
            read,
            supports,
            nodes,
            owing,
            sp.root_owes(),
            held_at_root,
        )
    };
    // The space is not asked again: the rest reads root's fixpoint, sealed,
    // and builds its programs over the space's terms.
    let (root, terms, ast) = sp.parts();

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
        // The generator's readers take a session, and read root through
        // it: a branch of the sealed fixpoint, which nothing is written to.
        let mut scratch = root.branch();
        let mut events = Events::off();
        let mut s = crate::saturator::Session {
            kb: &mut scratch,
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
        let (origin, verdict) = if rel == terms.kernel.relation {
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
    let small = with_space(
        &mut small_kb,
        terms,
        ast,
        &opts.layer,
        Some(&root_layer.candidates),
        |sp| {
            let licensed = sp.is_monotone();
            let l = layer(sp, licensed, true)?;
            let held = owing.as_ref().and_then(|(_, rows)| holders(sp, rows));
            Ok((l, held))
        },
    );
    let (small, held_in_theory) = match small {
        Ok((l, held)) => (Ok(l), held),
        Err(e) => (Err(e), None),
    };
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

    // A pair's own node the cache did not hold: each check is unread, and
    // says why, rather than left out.
    let nodes: Vec<FxHashSet<FactId>> = nodes.unwrap_or_else(|| {
        for s in supports.iter_mut().filter(|s| s.read) {
            s.check = Some(Check {
                dead: false,
                root_dead: false,
                beyond: 0,
                unread: Some("the pair's own node is not one root's layer computed".to_string()),
            });
        }
        Vec::new()
    });

    // The check: the program over each distinct support, with each of its
    // pairs.
    if check && !nodes.is_empty() && nodes.len() == supports.len() {
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

    let owed = Owed::read(
        root,
        terms,
        &candidate,
        root_layer.monotone,
        root_owes,
        owing,
        held_at_root,
        held_in_theory,
    );
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
        owed,
    })
}

/// For each instance's witnesses, the candidates alive alone whose own
/// node holds each, ascending: `rows` are L5's table's instances in its
/// order, each with the facts that can witness it. A fact nothing has
/// interned is held by no node.
///
/// **The singles are read off the cache and nothing is computed**
/// ([`Space::cached`]), so the reading asks no budget: a layer that read its
/// singles has its holders read, whatever it spent, its last step having
/// carried it past its budget or a later one having stopped it among its
/// pairs. `None` where a single was never computed, a layer stopped before
/// or among its singles, and there alone.
fn holders(sp: &Space<'_>, rows: &[crate::oblgen::Witnesses]) -> Option<Held> {
    let n = sp.n() as u32;
    let empty = |rows: &[crate::oblgen::Witnesses]| -> Held {
        rows.iter()
            .map(|r| vec![Vec::new(); r.rows.as_ref().map_or(0, Vec::len)])
            .collect()
    };
    if sp.root_is_dead() || n == 0 {
        // Under a dead root no candidate is alive alone.
        return Some(empty(rows));
    }
    let sets: Vec<Box<[u32]>> = (0..n).map(|b| Box::from([b])).collect();
    let singles = sp.cached(&sets)?;
    let states = sp.states();
    let terms = sp.terms();
    Some(
        rows.iter()
            .map(|r| {
                let Ok(witnesses) = &r.rows else {
                    return Vec::new();
                };
                witnesses
                    .iter()
                    .map(|(rel, args)| match terms.probe_fact(*rel, args) {
                        None => Vec::new(),
                        Some(fact) => (0..n)
                            .filter(|&a| {
                                let single = &singles[a as usize];
                                !single.dead
                                    && states[single.class as usize].binary_search(&fact).is_ok()
                            })
                            .collect(),
                    })
                    .collect()
            })
            .collect(),
    )
}

/// Per instance, per witness: the candidates whose own node holds it.
type Held = Vec<Vec<Vec<u32>>>;

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

// ── The cover ─────────────────────────────────────────────────────────

/// **The bound on one enumeration of maximal cliques**: the calls its
/// search makes, each of which lists at most one clique. A constant of the
/// module and no lever. A graph the search stops on says what was cut
/// ([`Cut`]).
pub const CLIQUE_STEPS: u64 = 1 << 20;

/// What [`Cover::domain`] says of a program the static check flags.
pub const COVER_FLAGGED: &str = "the static check flags this program: the members of a group \
                                 exclude pairwise at the small KB's root, and nothing above \
                                 root follows from that";

/// What root says of one candidate alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    /// Root's own fixpoint holds it: committing it is committing nothing.
    AtRoot,
    /// Its own node is dead at root.
    DeadAlone,
    /// Its own node is alive at root: it is open.
    AliveAlone,
    /// Root's singles were not read.
    Unread,
}

impl Mark {
    pub fn as_str(self) -> &'static str {
        match self {
            Mark::AtRoot => "at root",
            Mark::DeadAlone => "dead alone",
            Mark::AliveAlone => "alive alone",
            Mark::Unread => "unread",
        }
    }
}

/// What [`CLIQUE_STEPS`] cut of one enumeration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cut {
    /// Excluded pairs inside no clique listed.
    pub pairs: u64,
    /// Candidates of such a pair that no clique listed holds.
    pub candidates: u64,
}

/// The maximal cliques of one exclusion graph.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cliques {
    /// Each maximal clique of two or more candidates, ascending, and the
    /// list in the order of those sequences.
    pub cliques: Vec<Vec<u32>>,
    /// Calls the search made.
    pub steps: u64,
    /// What the bound cut, where it stopped the search: [`Self::cliques`]
    /// are then the cliques found before it, each maximal, and not all.
    pub cut: Option<Cut>,
}

impl Cliques {
    /// How many of the cliques each of `n` candidates is in.
    pub fn memberships(&self, n: usize) -> Vec<u32> {
        let mut out = vec![0u32; n];
        for c in &self.cliques {
            for &b in c {
                out[b as usize] += 1;
            }
        }
        out
    }
}

/// A set of candidates, a bit each.
#[derive(Clone, PartialEq, Eq)]
struct Bits(Vec<u64>);

impl Bits {
    fn new(n: usize) -> Bits {
        Bits(vec![0; n.div_ceil(64)])
    }

    fn set(&mut self, b: u32) {
        self.0[b as usize / 64] |= 1 << (b % 64);
    }

    fn clear(&mut self, b: u32) {
        self.0[b as usize / 64] &= !(1 << (b % 64));
    }

    fn has(&self, b: u32) -> bool {
        self.0[b as usize / 64] >> (b % 64) & 1 == 1
    }

    fn is_empty(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }

    fn and(&self, other: &Bits) -> Bits {
        Bits(self.0.iter().zip(&other.0).map(|(a, b)| a & b).collect())
    }

    fn count_and(&self, other: &Bits) -> u32 {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(a, b)| (a & b).count_ones())
            .sum()
    }

    /// Its members, ascending.
    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().enumerate().flat_map(|(k, &w)| {
            let mut w = w;
            std::iter::from_fn(move || {
                (w != 0).then(|| {
                    let b = w.trailing_zeros();
                    w &= w - 1;
                    k as u32 * 64 + b
                })
            })
        })
    }
}

/// An exclusion graph over `n` candidates: who excludes whom.
#[derive(Clone)]
struct Graph {
    adjacent: Vec<Bits>,
}

impl Graph {
    fn of(n: usize, edges: &[(u32, u32)]) -> Graph {
        let mut adjacent = vec![Bits::new(n); n];
        for &(a, b) in edges {
            if a != b {
                adjacent[a as usize].set(b);
                adjacent[b as usize].set(a);
            }
        }
        Graph { adjacent }
    }

    fn n(&self) -> usize {
        self.adjacent.len()
    }

    fn joins(&self, a: u32, b: u32) -> bool {
        self.adjacent[a as usize].has(b)
    }

    /// Every pair of `set` is an edge.
    fn is_clique(&self, set: &[u32]) -> bool {
        set.iter()
            .enumerate()
            .all(|(k, &a)| set[k + 1..].iter().all(|&b| self.joins(a, b)))
    }

    /// The connected components of two or more candidates, each ascending,
    /// in the order of their first members.
    fn components(&self) -> Vec<Vec<u32>> {
        let n = self.n();
        let mut seen = vec![false; n];
        let mut out = Vec::new();
        for a in 0..n as u32 {
            if seen[a as usize] || self.adjacent[a as usize].is_empty() {
                continue;
            }
            let mut component = vec![a];
            seen[a as usize] = true;
            let mut at = 0;
            while at < component.len() {
                let v = component[at];
                at += 1;
                for b in self.adjacent[v as usize].iter() {
                    if !seen[b as usize] {
                        seen[b as usize] = true;
                        component.push(b);
                    }
                }
            }
            component.sort_unstable();
            out.push(component);
        }
        out
    }

    /// The maximal cliques of two or more candidates, by at most `bound`
    /// calls of the search: Bron and Kerbosch's, with the pivot that leaves
    /// the fewest branches, the lowest candidate among equals. Every clique
    /// it lists is maximal; where the bound stopped it the list is short and
    /// [`Cliques::cut`] says by what.
    fn maximal_cliques(&self, bound: u64) -> Cliques {
        struct Search<'g> {
            graph: &'g Graph,
            found: Vec<Vec<u32>>,
            steps: u64,
            bound: u64,
            stopped: bool,
        }
        impl Search<'_> {
            /// Every maximal clique that holds `r`, some of `p` and none of
            /// `x`: `p` the candidates joined to all of `r` and not yet
            /// branched on, `x` those already branched on.
            fn extend(&mut self, r: &mut Vec<u32>, mut p: Bits, mut x: Bits) {
                if self.steps == self.bound {
                    self.stopped = true;
                    return;
                }
                self.steps += 1;
                if p.is_empty() {
                    if x.is_empty() {
                        let mut clique = r.clone();
                        clique.sort_unstable();
                        self.found.push(clique);
                    }
                    return;
                }
                let adjacent = &self.graph.adjacent;
                let mut pivot = (0u32, u32::MAX);
                for u in p.iter().chain(x.iter()) {
                    let joined = p.count_and(&adjacent[u as usize]);
                    if pivot.1 == u32::MAX || joined > pivot.0 || (joined == pivot.0 && u < pivot.1)
                    {
                        pivot = (joined, u);
                    }
                }
                let branches: Vec<u32> = p
                    .iter()
                    .filter(|&v| !adjacent[pivot.1 as usize].has(v))
                    .collect();
                for v in branches {
                    let near = &adjacent[v as usize];
                    r.push(v);
                    self.extend(r, p.and(near), x.and(near));
                    r.pop();
                    if self.stopped {
                        return;
                    }
                    p.clear(v);
                    x.set(v);
                }
            }
        }

        let n = self.n();
        let mut start = Bits::new(n);
        for a in 0..n as u32 {
            if !self.adjacent[a as usize].is_empty() {
                start.set(a);
            }
        }
        let mut search = Search {
            graph: self,
            found: Vec::new(),
            steps: 0,
            bound,
            stopped: false,
        };
        if !start.is_empty() {
            search.extend(&mut Vec::new(), start, Bits::new(n));
        }
        let mut cliques = search.found;
        cliques.sort();
        let cut = search.stopped.then(|| {
            let mut covered = vec![Bits::new(n); n];
            let mut held = Bits::new(n);
            for c in &cliques {
                for &a in c {
                    held.set(a);
                    for &b in c {
                        covered[a as usize].set(b);
                    }
                }
            }
            let mut cut = Cut::default();
            for a in 0..n as u32 {
                let near = &self.adjacent[a as usize];
                let left = near.iter().filter(|&b| !covered[a as usize].has(b)).count() as u64;
                // Each pair is met from both of its members.
                cut.pairs += left;
                cut.candidates += u64::from(left > 0 && !held.has(a));
            }
            cut.pairs /= 2;
            cut
        });
        Cliques {
            cliques,
            steps: search.steps,
            cut,
        }
    }
}

/// How a set of candidates stands to the cover ([`Cover::standing`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Fewer than two candidates: there is no pair to exclude.
    Fewer,
    /// It is this group.
    Group(u32),
    /// Pairwise excluding and not maximal: it is strictly inside each of
    /// these groups. None where the bound cut the cover before one of them.
    Inside(Vec<u32>),
    /// **Not pairwise excluding**: `alive` of its pairs the theory does not
    /// exclude. `holds` are the groups it holds whole, of each of which it
    /// is a strict superset.
    NotExcluding { alive: u64, holds: Vec<u32> },
}

/// How [`cover`] is asked.
#[derive(Clone, Copy, Debug)]
pub struct CoverOptions {
    /// The budget, the jobs and the deadline of the small KB's layer, and
    /// of root's where [`cover`] opens the space itself.
    pub layer: Options,
    /// Read root's pairs too: the cover then has [`Cover::root_cliques`]
    /// and [`Cover::alive_at_root`]. Without them root is read as far as
    /// its singles, which the marks need.
    pub root_pairs: bool,
}

impl Default for CoverOptions {
    fn default() -> Self {
        CoverOptions {
            layer: Options::default(),
            root_pairs: true,
        }
    }
}

/// **The cover of `H` by its groups**: the maximal sets of two or more
/// candidates that pairwise exclude by the theory alone, *pick at most
/// one*. Not a partition: a candidate is in as many groups as hold it, and
/// in none where the theory excludes no pair of it.
///
/// Every candidate is its index in [`Self::candidates`], the universe in
/// rendered-text order; a group is its place in [`Self::groups`]. Each
/// list is ascending, so a cover the bound did not cut does not depend on
/// the interner's ids or on the job count.
#[derive(Clone, Debug)]
pub struct Cover {
    pub candidates: Vec<Candidate>,
    /// What root says of each candidate alone.
    pub marks: Vec<Mark>,
    /// The candidates the theory kills alone, dead alone in the small KB:
    /// no pair of the theory holds one, so each is in no group.
    pub killed_alone: Vec<u32>,
    /// The pairs the theory excludes: the graph the groups are cliques of.
    pub excluded: Vec<(u32, u32)>,
    /// **Group to members**: each group's candidates, ascending, and the
    /// groups in the order of those sequences.
    pub groups: Vec<Vec<u32>>,
    /// **Member to groups**: for each candidate, the groups that hold it,
    /// ascending; empty for a candidate in none.
    pub member_of: Vec<Vec<u32>>,
    /// The connected components of the same graph, of two or more
    /// candidates each: what a group is not, wherever one of them is not a
    /// clique.
    pub components: Vec<Vec<u32>>,
    /// Calls the enumeration made, against [`CLIQUE_STEPS`].
    pub steps: u64,
    /// What the bound cut of the groups, where it did.
    pub cut: Option<Cut>,
    /// The static verdict on the program, and on its small KB.
    pub monotone: bool,
    pub small_monotone: bool,
    /// Why the small KB's layer stopped before its last pair, where it
    /// did: the groups are then cliques of the pairs it read.
    pub stopped: Option<Stopped>,
    /// **The theory's pairs whose node is alive at root.** Empty where the
    /// small KB derives no more than the program, which is what makes *at
    /// most one* of a group true of the program's own lattice. `None` where
    /// root's pairs were not all read.
    pub alive_at_root: Option<Vec<(u32, u32)>>,
    /// **The maximal cliques of root's exclusion graph**, beside the
    /// groups and not groups: what the clues add to the theory's edges.
    /// `None` where root's pairs were not all read.
    pub root_cliques: Option<Cliques>,
    /// What reading root's layer added to its space's work, and what the
    /// small KB's layer cost in its own root's terms.
    pub root_work: Work,
    pub small_work: Work,
    graph: Graph,
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph({} candidates)", self.n())
    }
}

impl Cover {
    /// The cover of the universe `root` and `small` share: `root` the
    /// program's own layer, read at least as far as its singles for the
    /// marks, and `small` the small KB's ([`Theory::small`]).
    pub fn of(root: &PairLayer, small: &PairLayer) -> Cover {
        Cover::bounded(root, small, CLIQUE_STEPS)
    }

    fn bounded(root: &PairLayer, small: &PairLayer, bound: u64) -> Cover {
        let n = root.n();
        let mut marks = vec![
            if root.singles_left == 0 {
                Mark::AliveAlone
            } else {
                Mark::Unread
            };
            n
        ];
        if root.singles_left == 0 {
            for &b in &root.at_root {
                marks[b as usize] = Mark::AtRoot;
            }
            for &b in &root.dead_alone {
                marks[b as usize] = Mark::DeadAlone;
            }
        }
        let graph = Graph::of(n, &small.excluded);
        let found = graph.maximal_cliques(bound);
        let mut member_of: Vec<Vec<u32>> = vec![Vec::new(); n];
        for (g, members) in found.cliques.iter().enumerate() {
            for &b in members {
                member_of[b as usize].push(g as u32);
            }
        }
        let whole = root.complete();
        let alive_at_root = whole.then(|| {
            let at_root = Deaths::of(root);
            small
                .excluded
                .iter()
                .copied()
                .filter(|&(a, b)| !at_root.pair(a, b))
                .collect()
        });
        let root_cliques = whole.then(|| Graph::of(n, &root.excluded).maximal_cliques(bound));
        Cover {
            candidates: root.candidates.clone(),
            marks,
            killed_alone: small.dead_alone.clone(),
            excluded: small.excluded.clone(),
            components: graph.components(),
            groups: found.cliques,
            member_of,
            steps: found.steps,
            cut: found.cut,
            monotone: root.monotone,
            small_monotone: small.monotone,
            stopped: small.stopped.clone(),
            alive_at_root,
            root_cliques,
            root_work: root.work,
            small_work: small.work,
            graph,
        }
    }

    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Candidate `b`, as the program writes it.
    pub fn text(&self, b: u32) -> &str {
        &self.candidates[b as usize].text
    }

    /// Group `g`'s members.
    pub fn members(&self, g: u32) -> &[u32] {
        &self.groups[g as usize]
    }

    /// The groups that hold candidate `b`.
    pub fn groups_of(&self, b: u32) -> &[u32] {
        &self.member_of[b as usize]
    }

    /// Group `g`'s **open** members: those alive alone at root.
    pub fn open(&self, g: u32) -> impl Iterator<Item = u32> + '_ {
        self.groups[g as usize]
            .iter()
            .copied()
            .filter(|&b| self.marks[b as usize] == Mark::AliveAlone)
    }

    /// The theory excludes the pair: the two share a group, wherever the
    /// bound did not cut the cover.
    pub fn excludes(&self, a: u32, b: u32) -> bool {
        a != b && self.graph.joins(a, b)
    }

    /// The candidates in no group, ascending: those the theory kills alone
    /// among them, and every candidate no pair of the theory holds.
    pub fn in_no_group(&self) -> Vec<u32> {
        (0..self.n() as u32)
            .filter(|&b| self.member_of[b as usize].is_empty())
            .collect()
    }

    /// Every group was listed and every pair of the small KB's layer read:
    /// the cover is the program's and not what a bound left of it.
    pub fn complete(&self) -> bool {
        self.cut.is_none() && self.stopped.is_none()
    }

    /// What the cover's statements are about, where that is not the whole
    /// lattice: [`COVER_FLAGGED`] on a program the static check flags.
    pub fn domain(&self) -> Option<&'static str> {
        (!self.monotone).then_some(COVER_FLAGGED)
    }

    /// **How a set of candidates stands to the cover**: an owed instance's
    /// alternatives, a declared key's class, any set a reader has from what
    /// the program declares. `set` is read as a set, in any order.
    pub fn standing(&self, set: &[u32]) -> Standing {
        let mut set = set.to_vec();
        set.sort_unstable();
        set.dedup();
        if set.len() < 2 {
            return Standing::Fewer;
        }
        let inside = |g: &Vec<u32>| g.iter().all(|b| set.binary_search(b).is_ok());
        if self.graph.is_clique(&set) {
            // The groups that hold its first member and every other.
            let holds: Vec<u32> = self.member_of[set[0] as usize]
                .iter()
                .copied()
                .filter(|&g| {
                    let members = &self.groups[g as usize];
                    set.iter().all(|b| members.binary_search(b).is_ok())
                })
                .collect();
            return match holds
                .iter()
                .find(|&&g| self.groups[g as usize].len() == set.len())
            {
                Some(&g) => Standing::Group(g),
                None => Standing::Inside(holds),
            };
        }
        let mut alive = 0u64;
        for (k, &a) in set.iter().enumerate() {
            alive += set[k + 1..]
                .iter()
                .filter(|&&b| !self.graph.joins(a, b))
                .count() as u64;
        }
        let mut holds: Vec<u32> = set
            .iter()
            .flat_map(|&b| self.member_of[b as usize].iter().copied())
            .filter(|&g| inside(&self.groups[g as usize]))
            .collect();
        holds.sort_unstable();
        holds.dedup();
        Standing::NotExcluding { alive, holds }
    }
}

impl Theory {
    /// The cover of `H` by the groups this theory makes of it. `None`
    /// where the small KB has no layer: the program has no groups by this
    /// reading.
    pub fn cover(&self) -> Option<Cover> {
        self.small.as_ref().ok().map(|s| Cover::of(&self.root, s))
    }
}

/// The cover of `root`, a **freshly loaded, unsaturated** program nothing
/// else will use, under [`small_kb`]: `cover_in` over a space opened for
/// it, at `opts.layer`'s budget.
pub fn cover(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &CoverOptions,
) -> Result<Cover, SolveError> {
    let loaded = Loaded::of(root);
    with_space(root, terms, ast, &opts.layer, None, |sp| {
        cover_in(sp, &loaded, opts)
    })
}

/// **The cover over a program's prepared space**, computed once a program:
/// root's singles, and its pairs where [`CoverOptions::root_pairs`] asks,
/// are read through `sp`, the routes' node cache, so a node the space has
/// computed is not saturated again and the nodes read are the space's from
/// then on; the small KB is built from `loaded`, [`Loaded::of`] the same
/// program taken before its root was saturated, and its layer read in a
/// space of its own at `opts.layer`'s budget. Nothing is written that a
/// node reads. An error where the small KB has no layer.
pub(crate) fn cover_in(
    sp: &mut Space<'_>,
    loaded: &Loaded,
    opts: &CoverOptions,
) -> Result<Cover, SolveError> {
    let t = theory_in(sp, loaded, reading_of(opts), &theory_asked(opts), &small_kb)?;
    match &t.small {
        Ok(small) => Ok(Cover::of(&t.root, small)),
        Err(e) => Err(SolveError::Unsupported(format!(
            "the small KB has no layer, so the program has no groups: {e}"
        ))),
    }
}

// ── Exactly one: owed groups and grids ────────────────────────────────

/// Why L5 does not license an instance one of whose witnesses is
/// [`WitnessIs::Unproposed`], where its static premise holds.
pub const UNPROPOSED: &str =
    "a fact can witness it that is no candidate and that root does not deny";

/// What root says of one fact that can witness an owed instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WitnessIs {
    /// It is this candidate of `H`.
    Candidate(u32),
    /// Root's fixpoint stores its negative: no alive state holds it.
    Denied,
    /// Neither: no commitment names it, and root does not rule it out.
    Unproposed,
}

impl WitnessIs {
    pub fn as_str(self) -> &'static str {
        match self {
            WitnessIs::Candidate(_) => "a candidate",
            WitnessIs::Denied => "denied at root",
            WitnessIs::Unproposed => "no candidate, not denied",
        }
    }
}

/// One fact that can witness an owed instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Witness {
    /// As the program writes it. It need not be a fact any node holds.
    pub text: String,
    pub is: WitnessIs,
    /// The candidates alive alone at root whose own node holds it,
    /// ascending: it is in the closure of each. A candidate alive alone
    /// holds itself. Empty where no single's node does, and where
    /// [`Owed::singles_read`] is `false`.
    pub held_at_root: Vec<u32>,
    /// The same of the small KB's nodes: the candidates that bring it by
    /// the theory alone. Empty where [`Owed::theory_read`] is `false`.
    pub held_in_theory: Vec<u32>,
}

/// One instance root owes, as L5's table has it, with its witnesses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwedInstance {
    /// Its obligation rule, by the name the program gives it.
    pub rule: String,
    /// The instance as text: the rule with its activator's arguments, then
    /// the match's bindings.
    pub text: String,
    /// **L5's reading**: the candidates that can witness it, ascending,
    /// where its premise holds of the instance
    /// ([`crate::oblgen::Instance::alternatives`]); why not where it does
    /// not, in L5's own words, but that [`UNPROPOSED`] stands for the two
    /// sentences in which it names a witness, since which of the two it
    /// says depends on whether anything has interned the fact.
    pub alternatives: Result<Vec<u32>, String>,
    /// Witnesses L5 left out of the alternatives because root denies them.
    pub denied: u32,
    /// **Every fact that can witness it**, in the order root's scan finds
    /// them, licensed or not; why none is known where the rule names no
    /// witness.
    pub witnesses: Result<Vec<Witness>, String>,
}

/// Why [`Owed`] has no table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoTable {
    /// The static check flags the program: L5 is read on a monotone one.
    Flagged,
    /// The program states no obligation rule.
    NoObligation,
}

impl NoTable {
    pub fn as_str(self) -> &'static str {
        match self {
            NoTable::Flagged => "the static check flags the program",
            NoTable::NoObligation => "the program states no obligation",
        }
    }
}

/// **What root owes**, as the engine reads it: L5's table
/// ([`crate::oblgen::Alternatives`]), which the obligation tree branches on,
/// with every instance's witnesses beside. Read once a program, of root's
/// fixpoint; nothing here is handed to a node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Owed {
    /// Instances root's fixpoint owes, by the node function's own count,
    /// with or without a table.
    pub root_owes: u32,
    /// The table's instances, in its order.
    pub instances: Result<Vec<OwedInstance>, NoTable>,
    /// Root's singles were read, every one: [`Witness::held_at_root`] says
    /// something. `false` where root's layer stopped before its last single,
    /// and where there is no table.
    pub singles_read: bool,
    /// The small KB's singles were read, every one:
    /// [`Witness::held_in_theory`] does. `false` where its layer stopped
    /// before its last single or it has none, and where there is no table.
    pub theory_read: bool,
}

impl Owed {
    #[allow(clippy::too_many_arguments)]
    fn read(
        root: &Kb,
        terms: &Terms,
        candidate: &FxHashMap<FactId, u32>,
        monotone: bool,
        root_owes: u32,
        owing: Option<explore::Owed>,
        held_at_root: Option<Held>,
        held_in_theory: Option<Held>,
    ) -> Owed {
        let Some((table, rows)) = owing else {
            return Owed {
                root_owes,
                instances: Err(if monotone {
                    NoTable::NoObligation
                } else {
                    NoTable::Flagged
                }),
                singles_read: false,
                theory_read: false,
            };
        };
        let instances = table
            .instances
            .iter()
            .zip(rows)
            .enumerate()
            .map(|(k, (i, read))| {
                let (premise, rows) = (read.premise, read.rows);
                let (rule, activator, bindings) = &i.key;
                let rule = terms.sym(*rule).to_string();
                let text = format!(
                    "({rule}{}){}",
                    activator
                        .iter()
                        .map(|&v| format!(" {}", terms.display(v)))
                        .collect::<String>(),
                    bindings
                        .iter()
                        .map(|&(x, v)| format!(" {}={}", terms.sym(x), terms.display(v)))
                        .collect::<String>(),
                );
                let held = |h: &Option<Held>, w: usize| -> Vec<u32> {
                    h.as_ref()
                        .and_then(|h| h[k].get(w))
                        .cloned()
                        .unwrap_or_default()
                };
                let witnesses: Result<Vec<Witness>, String> = rows.map(|rows| {
                    rows.iter()
                        .enumerate()
                        .map(|(w, (rel, args))| {
                            let fact = terms.probe_fact(*rel, args);
                            let shown: Vec<String> =
                                args.iter().map(|&a| terms.display(a)).collect();
                            Witness {
                                text: ein_core::text::form(terms.sym(*rel), &shown),
                                is: match fact {
                                    Some(f) if candidate.contains_key(&f) => {
                                        WitnessIs::Candidate(candidate[&f])
                                    }
                                    Some(f) if root.is_negated(f) => WitnessIs::Denied,
                                    _ => WitnessIs::Unproposed,
                                },
                                held_at_root: held(&held_at_root, w),
                                held_in_theory: held(&held_in_theory, w),
                            }
                        })
                        .collect()
                });
                // L5's verdict, and where it does not license, why, as a
                // function of the program: its own words but where they
                // name a witness, which they do by what has been interned.
                let alternatives = match &i.alternatives {
                    Ok(a) => Ok(a.to_vec()),
                    Err(said) => Err(match (&witnesses, premise) {
                        (Err(why), _) => why.clone(),
                        (Ok(_), Err(why)) => why,
                        (Ok(ws), Ok(())) if ws.iter().any(|w| w.is == WitnessIs::Unproposed) => {
                            UNPROPOSED.to_string()
                        }
                        (Ok(_), Ok(())) => said.clone(),
                    }),
                };
                OwedInstance {
                    rule,
                    text,
                    alternatives,
                    denied: i.excluded,
                    witnesses,
                }
            })
            .collect();
        Owed {
            root_owes,
            instances: Ok(instances),
            singles_read: held_at_root.is_some(),
            theory_read: held_in_theory.is_some(),
        }
    }

    /// The table's instances; none where there is no table.
    pub fn table(&self) -> &[OwedInstance] {
        self.instances.as_ref().map_or(&[], |i| &i[..])
    }
}

/// **A grid**: a connected component of the theory's exclusion graph whose
/// groups are two families, every member of the component in exactly one
/// group of each. A member is one cell, the group of one family its row and
/// the group of the other its column.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    /// The component it is, a place in [`Cover::components`]: its members.
    pub component: u32,
    /// Its two families of groups, each ascending. The first is the family
    /// of the grid's lowest group, which says nothing of which is rows.
    pub families: [Vec<u32>; 2],
}

/// Why a component of the theory's exclusion graph is no [`Grid`], by the
/// first of these that holds of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotAGrid {
    /// The bound cut the cover: its groups are not all listed.
    Cut,
    /// A member is in more than two groups, `most` of them: something ties
    /// its lines together.
    MoreThanTwo { most: u32 },
    /// It is one group: one line, and no second family.
    OneGroup,
    /// `members` of its members are in one group only: their other line
    /// holds one candidate, and is no group.
    InOne { members: u32 },
    /// Every member is in two groups, and the groups do not split into two
    /// families with no member shared inside one.
    NoTwoFamilies,
    /// Two of its groups share two members or more: two cells in one row
    /// and one column.
    SharedTwice,
}

impl NotAGrid {
    pub fn as_str(self) -> &'static str {
        match self {
            NotAGrid::Cut => "the bound cut the cover",
            NotAGrid::MoreThanTwo { .. } => "a member is in more than two groups",
            NotAGrid::OneGroup => "it is one group",
            NotAGrid::InOne { .. } => "a member is in one group only",
            NotAGrid::NoTwoFamilies => "its groups are not two families",
            NotAGrid::SharedTwice => "two groups share two members",
        }
    }
}

/// The grids of a cover, and why each other component is none
/// ([`Cover::grids`]).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grids {
    /// In the order of their components.
    pub grids: Vec<Grid>,
    /// Each component that is no grid, a place in [`Cover::components`],
    /// with why.
    pub not: Vec<(u32, NotAGrid)>,
    /// **Group to grid**: for each group, its grid and which of the grid's
    /// two families it is in; `None` for a group of a component that is no
    /// grid.
    pub of_group: Vec<Option<(u32, u8)>>,
}

impl Cover {
    /// **The grids**, found from the groups and their members alone: each
    /// connected component of the theory's exclusion graph in which every
    /// member is in exactly two groups, the groups split into two families
    /// no two groups of which share a member, and two groups share at most
    /// one member. No relation is read, and no declaration.
    pub fn grids(&self) -> Grids {
        let mut out = Grids {
            of_group: vec![None; self.groups.len()],
            ..Grids::default()
        };
        for (c, members) in self.components.iter().enumerate() {
            match self.grid_of(members) {
                Ok(families) => {
                    let grid = out.grids.len() as u32;
                    for (family, groups) in families.iter().enumerate() {
                        for &g in groups {
                            out.of_group[g as usize] = Some((grid, family as u8));
                        }
                    }
                    out.grids.push(Grid {
                        component: c as u32,
                        families,
                    });
                }
                Err(why) => out.not.push((c as u32, why)),
            }
        }
        out
    }

    /// The two families of the component `members`, or why it has none.
    fn grid_of(&self, members: &[u32]) -> Result<[Vec<u32>; 2], NotAGrid> {
        if self.cut.is_some() {
            return Err(NotAGrid::Cut);
        }
        let in_groups = |b: u32| self.member_of[b as usize].len() as u32;
        let most = members.iter().map(|&b| in_groups(b)).max().unwrap_or(0);
        if most > 2 {
            return Err(NotAGrid::MoreThanTwo { most });
        }
        let mut groups: Vec<u32> = members
            .iter()
            .flat_map(|&b| self.member_of[b as usize].iter().copied())
            .collect();
        groups.sort_unstable();
        groups.dedup();
        if groups.len() < 2 {
            return Err(NotAGrid::OneGroup);
        }
        let ones = members.iter().filter(|&&b| in_groups(b) == 1).count() as u32;
        if ones > 0 {
            return Err(NotAGrid::InOne { members: ones });
        }
        // Two groups are joined where they share a member: every member
        // joins its two.
        let place = |g: u32| groups.binary_search(&g).expect("a group of the component");
        let mut joined: Vec<Vec<usize>> = vec![Vec::new(); groups.len()];
        let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(members.len());
        for &b in members {
            let of = &self.member_of[b as usize];
            let (x, y) = (place(of[0]), place(of[1]));
            joined[x].push(y);
            joined[y].push(x);
            pairs.push((x, y));
        }
        // The lowest group's family is the first; the component is
        // connected, so one walk colours every group.
        let mut family: Vec<Option<u8>> = vec![None; groups.len()];
        family[0] = Some(0);
        let mut queue = vec![0usize];
        let mut at = 0;
        while at < queue.len() {
            let x = queue[at];
            at += 1;
            let other = 1 - family[x].expect("coloured");
            for &y in &joined[x] {
                match family[y] {
                    None => {
                        family[y] = Some(other);
                        queue.push(y);
                    }
                    Some(f) if f != other => return Err(NotAGrid::NoTwoFamilies),
                    Some(_) => {}
                }
            }
        }
        pairs.sort_unstable();
        if pairs.windows(2).any(|w| w[0] == w[1]) {
            return Err(NotAGrid::SharedTwice);
        }
        let of = |f: u8| -> Vec<u32> {
            (0..groups.len())
                .filter(|&k| family[k] == Some(f))
                .map(|k| groups[k])
                .collect()
        };
        Ok([of(0), of(1)])
    }
}

/// How one instance root owes stands to the cover ([`ExactlyOne::owing`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Owing {
    /// **An owed group**: L5 licenses the instance and its alternatives are
    /// exactly this group's members. *Exactly one.*
    Group(u32),
    /// Licensed, its alternatives pairwise excluding and strictly inside
    /// each of these groups: exactly one of the alternatives, and the
    /// groups' other members witness nothing.
    Inside(Vec<u32>),
    /// **An owed set that is not a group**: licensed, and `alive` pairs of
    /// its alternatives are not excluded by the theory. *At least one.*
    /// `holds` are the groups it holds whole.
    NotExcluding { alive: u64, holds: Vec<u32> },
    /// Licensed, with one alternative: every state of `R` holds it.
    One(u32),
    /// Licensed, with no alternative: nothing can pay it.
    None,
    /// **L5 does not license it**, for the reason
    /// [`OwedInstance::alternatives`] gives. `payers` are the candidates
    /// whose own node in the small KB holds one of its witnesses, where
    /// those nodes and the witnesses were read. The instance is `about` a
    /// group where the payers are exactly that group's members and every
    /// witness root does not deny is held by a payer's node. It does not
    /// owe that group: nothing is concluded from a witness a candidate
    /// brings.
    Unlicensed {
        payers: Option<Vec<u32>>,
        about: Option<u32>,
    },
}

impl Owing {
    pub fn as_str(&self) -> &'static str {
        match self {
            Owing::Group(_) => "a group",
            Owing::Inside(_) => "inside a group",
            Owing::NotExcluding { .. } => "not pairwise excluding",
            Owing::One(_) => "one alternative",
            Owing::None => "no alternative",
            Owing::Unlicensed { .. } => "unlicensed",
        }
    }
}

/// What a family of a grid's groups is owed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Share {
    /// The family's groups.
    pub groups: u32,
    /// Of them, owed groups.
    pub owed: u32,
    /// Of the others, those an unlicensed instance is about.
    pub about: u32,
}

/// **Exactly one**: what root owes read against the cover, and the grids.
///
/// *At most one* is a group's, and free. *Exactly one* is an **owed
/// group**'s: a group whose members are exactly a licensed instance's
/// alternatives, so that every state of `R` holds one of its open members
/// and no state two. Nothing here is synthesised: an instance is the
/// program's obligation as L5 reads it, and a group nothing owes stays *at
/// most one* whatever a count would suggest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactlyOne {
    /// What root owes.
    pub owed: Owed,
    /// How each instance of [`Owed::table`] stands to the cover, in its
    /// order.
    pub owing: Vec<Owing>,
    /// **Group to the instances that owe it**: for each group, the licensed
    /// instances whose alternatives are exactly its members, ascending.
    /// Empty for a group nothing owes.
    pub owed_by: Vec<Vec<u32>>,
    /// For each group, the unlicensed instances that are about it.
    pub about: Vec<Vec<u32>>,
    /// The grids of the cover.
    pub grids: Grids,
}

impl ExactlyOne {
    /// `owed` read against `cover`, both of one program.
    pub fn of(cover: &Cover, owed: Owed) -> ExactlyOne {
        let mut owed_by: Vec<Vec<u32>> = vec![Vec::new(); cover.groups.len()];
        let mut about: Vec<Vec<u32>> = vec![Vec::new(); cover.groups.len()];
        let owing = owed
            .table()
            .iter()
            .enumerate()
            .map(|(k, i)| match &i.alternatives {
                Ok(alternatives) => match cover.standing(alternatives) {
                    Standing::Group(g) => {
                        owed_by[g as usize].push(k as u32);
                        Owing::Group(g)
                    }
                    Standing::Inside(groups) => Owing::Inside(groups),
                    Standing::NotExcluding { alive, holds } => Owing::NotExcluding { alive, holds },
                    Standing::Fewer => match alternatives.first() {
                        Some(&b) => Owing::One(b),
                        None => Owing::None,
                    },
                },
                Err(_) => {
                    let payers = match (&i.witnesses, owed.theory_read) {
                        (Ok(witnesses), true) => {
                            let mut payers: Vec<u32> = witnesses
                                .iter()
                                .flat_map(|w| w.held_in_theory.iter().copied())
                                .collect();
                            payers.sort_unstable();
                            payers.dedup();
                            Some(payers)
                        }
                        _ => None,
                    };
                    // Every witness root does not deny is brought by a
                    // payer, and the payers are a group's members.
                    let all_brought = i.witnesses.as_ref().is_ok_and(|ws| {
                        ws.iter()
                            .all(|w| w.is == WitnessIs::Denied || !w.held_in_theory.is_empty())
                    });
                    let group = payers.as_ref().filter(|_| all_brought).and_then(|p| {
                        match cover.standing(p) {
                            Standing::Group(g) => Some(g),
                            _ => None,
                        }
                    });
                    if let Some(g) = group {
                        about[g as usize].push(k as u32);
                    }
                    Owing::Unlicensed {
                        payers,
                        about: group,
                    }
                }
            })
            .collect();
        ExactlyOne {
            owed,
            owing,
            owed_by,
            about,
            grids: cover.grids(),
        }
    }

    /// Group `g` is an owed group: *exactly one*.
    pub fn is_owed(&self, g: u32) -> bool {
        !self.owed_by[g as usize].is_empty()
    }

    /// The owed groups, ascending.
    pub fn owed_groups(&self) -> Vec<u32> {
        (0..self.owed_by.len() as u32)
            .filter(|&g| self.is_owed(g))
            .collect()
    }

    /// **The groups nothing owes**, ascending: *at most one*, and no
    /// instance L5 licenses says a member must hold.
    pub fn unowed(&self) -> Vec<u32> {
        (0..self.owed_by.len() as u32)
            .filter(|&g| !self.is_owed(g))
            .collect()
    }

    /// **The owed sets that are not a group**: the licensed instances whose
    /// alternatives do not pairwise exclude, as places in [`Owed::table`].
    pub fn not_groups(&self) -> Vec<u32> {
        (0..self.owing.len() as u32)
            .filter(|&k| matches!(self.owing[k as usize], Owing::NotExcluding { .. }))
            .collect()
    }

    /// The instances L5 does not license, as places in [`Owed::table`].
    pub fn unlicensed(&self) -> Vec<u32> {
        (0..self.owing.len() as u32)
            .filter(|&k| matches!(self.owing[k as usize], Owing::Unlicensed { .. }))
            .collect()
    }

    /// Of instance `k`'s witnesses that are no candidate and that root does
    /// not deny: how many there are, and how many of them some candidate
    /// alive alone at root brings, its own node holding the witness.
    /// `None` where the witnesses or root's singles were not read.
    pub fn brought(&self, k: u32) -> Option<(u32, u32)> {
        let witnesses = self.owed.table()[k as usize].witnesses.as_ref().ok()?;
        if !self.owed.singles_read {
            return None;
        }
        let unproposed = witnesses.iter().filter(|w| w.is == WitnessIs::Unproposed);
        let (mut all, mut brought) = (0, 0);
        for w in unproposed {
            all += 1;
            brought += u32::from(!w.held_at_root.is_empty());
        }
        Some((all, brought))
    }

    /// What each family of grid `grid` is owed.
    pub fn share(&self, grid: u32) -> [Share; 2] {
        let of = |groups: &Vec<u32>| {
            let mut s = Share {
                groups: groups.len() as u32,
                ..Share::default()
            };
            for &g in groups {
                if self.is_owed(g) {
                    s.owed += 1;
                } else if !self.about[g as usize].is_empty() {
                    s.about += 1;
                }
            }
            s
        };
        let families = &self.grids.grids[grid as usize].families;
        [of(&families[0]), of(&families[1])]
    }

    /// **A bijection**: grid `grid` has as many groups in one family as in
    /// the other and every one of them is an owed group, so every state of
    /// `R` holds exactly one member in each row and in each column. A grid
    /// every group of which is owed and whose families differ in number is
    /// none: its share says so, and nothing is concluded from the count.
    pub fn is_bijection(&self, grid: u32) -> bool {
        let share = self.share(grid);
        share[0].groups == share[1].groups && share.iter().all(|s| s.owed == s.groups)
    }
}

impl Theory {
    /// The cover, and what root owes read against it. `None` where the
    /// small KB has no layer.
    pub fn exactly_one(&self) -> Option<(Cover, ExactlyOne)> {
        let cover = self.cover()?;
        let exact = ExactlyOne::of(&cover, self.owed.clone());
        Some((cover, exact))
    }
}

/// The cover of `root`, a **freshly loaded, unsaturated** program nothing
/// else will use, and what root owes read against it:
/// `exactly_one_in` over a space opened for it, at `opts.layer`'s budget.
pub fn exactly_one(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &CoverOptions,
) -> Result<(Cover, ExactlyOne), SolveError> {
    let loaded = Loaded::of(root);
    with_space(root, terms, ast, &opts.layer, None, |sp| {
        exactly_one_in(sp, &loaded, opts)
    })
}

/// **The cover, the owed groups and the grids over a program's prepared
/// space**, computed once a program, as `cover_in` computes the cover:
/// one reading of root's layer through `sp` and of the small KB's in a
/// space of its own. L5's table is read of root's fixpoint and is not
/// handed to the node function, so a route that takes the table itself
/// reads the same one and nothing a node reports changes.
pub(crate) fn exactly_one_in(
    sp: &mut Space<'_>,
    loaded: &Loaded,
    opts: &CoverOptions,
) -> Result<(Cover, ExactlyOne), SolveError> {
    let t = theory_in(sp, loaded, reading_of(opts), &theory_asked(opts), &small_kb)?;
    t.exactly_one().ok_or_else(|| {
        SolveError::Unsupported(format!(
            "the small KB has no layer, so the program has no groups: {}",
            t.small.as_ref().err().map_or("", String::as_str)
        ))
    })
}

/// How far a cover's options read root.
fn reading_of(opts: &CoverOptions) -> Reading {
    if opts.root_pairs {
        Reading::Pairs
    } else {
        Reading::Singles
    }
}

/// The theory's share a cover asks: no support and no check.
fn theory_asked(opts: &CoverOptions) -> TheoryOptions {
    TheoryOptions {
        layer: opts.layer,
        root_pairs: opts.root_pairs,
        supports: false,
        check: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{load, parse};

    fn layer_of(rel: &str, licensed: bool) -> PairLayer {
        let (mut kb, mut terms, ast) = fresh(rel);
        with_space(&mut kb, &mut terms, &ast, &Options::default(), None, |sp| {
            layer(sp, licensed, true)
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
    /// facts in the same order, each with a record of the same step from the
    /// same premises.
    #[test]
    fn the_program_over_all_its_facts_is_the_program() {
        for rel in SHAPES {
            let (kb, mut terms, _) = fresh(rel);
            let loaded = Loaded::of(&kb);
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
            // And over none of them, nothing is left.
            let none = loaded.over(&mut terms, &|_| false);
            assert_eq!(none.facts().count(), 0, "{rel}");
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

    /// A layer of `n` candidates named `c0`, `c1`, … whose every single is
    /// alive alone but `dead`, and whose excluded pairs are `edges`.
    fn layer_over(n: u32, edges: &[(u32, u32)], dead: &[u32]) -> PairLayer {
        let mut excluded: Vec<(u32, u32)> =
            edges.iter().map(|&(a, b)| (a.min(b), a.max(b))).collect();
        excluded.sort_unstable();
        excluded.dedup();
        PairLayer {
            candidates: (0..n)
                .map(|b| Candidate {
                    fact: FactId(b),
                    text: format!("(c{b})"),
                })
                .collect(),
            monotone: true,
            root_dead: false,
            singles_left: 0,
            at_root: Vec::new(),
            dead_alone: dead.to_vec(),
            implies: Vec::new(),
            equivalent: Vec::new(),
            excluded,
            interacting: Vec::new(),
            against_l1: Vec::new(),
            against_l3: Vec::new(),
            pairs: Pairs::default(),
            licences: Vec::new(),
            work: Work::default(),
            stopped: None,
        }
    }

    /// Every maximal clique of two or more, by trying every set: for a
    /// graph small enough to.
    fn by_every_set(n: u32, graph: &Graph) -> Vec<Vec<u32>> {
        let mut out = Vec::new();
        for bits in 0u32..1 << n {
            let set: Vec<u32> = (0..n).filter(|b| bits >> b & 1 == 1).collect();
            if set.len() < 2 || !graph.is_clique(&set) {
                continue;
            }
            let grows = (0..n)
                .filter(|b| bits >> b & 1 == 0)
                .any(|b| set.iter().all(|&a| graph.joins(a, b)));
            if !grows {
                out.push(set);
            }
        }
        out.sort();
        out
    }

    /// **The enumeration lists every maximal clique, once, in one order.**
    /// On 400 graphs of up to twelve candidates it is held to the cliques
    /// found by trying every set; and the same graph with its candidates
    /// renumbered has the same cliques under the renumbering, so nothing in
    /// the list is the search's.
    #[test]
    fn the_enumeration_lists_every_maximal_clique() {
        let mut rng = crate::mt19937::Mt19937::seeded(20);
        let (mut graphs, mut cliques, mut overlapping) = (0, 0, 0);
        for round in 0..400u32 {
            let n = 2 + round % 11;
            let mut pairs: Vec<(u32, u32)> = (0..n)
                .flat_map(|a| (a + 1..n).map(move |b| (a, b)))
                .collect();
            rng.shuffle(&mut pairs);
            // From sparse to nearly complete.
            pairs.truncate(pairs.len() * (1 + round as usize % 7) / 8);
            let graph = Graph::of(n as usize, &pairs);
            let found = graph.maximal_cliques(CLIQUE_STEPS);
            assert!(found.cut.is_none());
            assert_eq!(found.cliques, by_every_set(n, &graph), "{pairs:?}");
            graphs += 1;
            cliques += found.cliques.len();
            overlapping += usize::from(found.memberships(n as usize).iter().any(|&k| k > 2));

            let mut to: Vec<u32> = (0..n).collect();
            rng.shuffle(&mut to);
            let renumbered: Vec<(u32, u32)> = pairs
                .iter()
                .map(|&(a, b)| (to[a as usize], to[b as usize]))
                .collect();
            let mut again: Vec<Vec<u32>> = Graph::of(n as usize, &renumbered)
                .maximal_cliques(CLIQUE_STEPS)
                .cliques;
            let mut moved: Vec<Vec<u32>> = found
                .cliques
                .iter()
                .map(|c| {
                    let mut c: Vec<u32> = c.iter().map(|&b| to[b as usize]).collect();
                    c.sort_unstable();
                    c
                })
                .collect();
            moved.sort();
            again.sort();
            assert_eq!(again, moved, "{pairs:?} under {to:?}");
        }
        // Not vacuous: graphs with cliques, and with a candidate in three.
        assert!(graphs == 400 && cliques > 1000 && overlapping > 50);
    }

    /// **The bound stops the search and says what it cut.** Five sets of
    /// three, every candidate joined to all outside its own set: 243
    /// maximal cliques, one from each choice. Under the module's bound all
    /// are listed. Under a bound of 60 calls fewer are, each one maximal,
    /// and the cut counts the pairs no listed clique holds.
    #[test]
    fn the_bound_says_what_it_cut() {
        let n = 15u32;
        let edges: Vec<(u32, u32)> = (0..n)
            .flat_map(|a| (a + 1..n).map(move |b| (a, b)))
            .filter(|&(a, b)| a / 3 != b / 3)
            .collect();
        let graph = Graph::of(n as usize, &edges);
        let whole = graph.maximal_cliques(CLIQUE_STEPS);
        assert_eq!((whole.cliques.len(), whole.cut), (243, None));
        assert!(whole.steps < CLIQUE_STEPS);
        assert_eq!(whole.memberships(n as usize), vec![81; 15]);

        let short = graph.maximal_cliques(60);
        assert_eq!(short.steps, 60);
        let cut = short.cut.expect("the bound cut it");
        assert!(!short.cliques.is_empty() && short.cliques.len() < 60);
        for c in &short.cliques {
            assert!(whole.cliques.contains(c), "{c:?} is not maximal");
        }
        let held: FxHashSet<(u32, u32)> = short
            .cliques
            .iter()
            .flat_map(|c| c.iter().flat_map(move |&a| c.iter().map(move |&b| (a, b))))
            .collect();
        let left = edges.iter().filter(|p| !held.contains(p)).count() as u64;
        assert!(left > 0);
        assert_eq!(cut.pairs, left);

        // The cover says so, and is not complete.
        let layer = layer_over(n, &edges, &[]);
        let cover = Cover::bounded(&layer, &layer, 60);
        assert_eq!(cover.cut, Some(cut));
        assert!(!cover.complete() && cover.groups == short.cliques);
        assert!(Cover::of(&layer, &layer).complete());
    }

    /// **The cover's two maps, the marks and how a set stands to it.** A
    /// grid of two rows and three columns, and a candidate nothing
    /// excludes: five groups, each candidate of the grid in two, the seventh
    /// in none; a member dead alone at root is a member, and not open.
    #[test]
    fn a_grid_is_covered_by_its_rows_and_columns() {
        // 0 1 2
        // 3 4 5      and 6, alone.
        let edges = [
            (0, 1),
            (0, 2),
            (1, 2),
            (3, 4),
            (3, 5),
            (4, 5),
            (0, 3),
            (1, 4),
            (2, 5),
        ];
        let small = layer_over(7, &edges, &[]);
        // At root 1 is dead alone, so root excludes no pair of it.
        let at_root: Vec<(u32, u32)> = edges
            .iter()
            .copied()
            .filter(|&(a, b)| a != 1 && b != 1)
            .collect();
        let root = layer_over(7, &at_root, &[1]);
        let c = Cover::of(&root, &small);
        assert_eq!(
            c.groups,
            [
                vec![0, 1, 2],
                vec![0, 3],
                vec![1, 4],
                vec![2, 5],
                vec![3, 4, 5]
            ]
        );
        assert_eq!(c.member_of[0], [0, 1]);
        assert_eq!(c.groups_of(4), [2, 4]);
        assert_eq!(c.in_no_group(), [6]);
        assert_eq!(c.marks[1], Mark::DeadAlone);
        assert_eq!(c.marks[6], Mark::AliveAlone);
        assert_eq!(c.open(0).collect::<Vec<_>>(), [0, 2]);
        assert_eq!(c.open(2).collect::<Vec<_>>(), [4]);
        assert!(c.complete() && c.excludes(0, 3) && !c.excludes(0, 4) && !c.excludes(6, 6));
        // One component, which is no clique: 0 and 4 are in it and do not
        // exclude.
        assert_eq!(c.components, [vec![0, 1, 2, 3, 4, 5]]);
        // Every pair of the theory's is dead at root: by exclusion, or by
        // its member dead alone.
        assert_eq!(c.alive_at_root, Some(Vec::new()));
        // Root's own graph has lost the candidate dead alone.
        let root_cliques = c.root_cliques.as_ref().expect("root's pairs were read");
        assert_eq!(
            root_cliques.cliques,
            [vec![0, 2], vec![0, 3], vec![2, 5], vec![3, 4, 5]]
        );

        assert_eq!(c.standing(&[2, 1, 0]), Standing::Group(0));
        assert_eq!(c.standing(&[5, 3]), Standing::Inside(vec![4]));
        assert_eq!(c.standing(&[4]), Standing::Fewer);
        assert_eq!(c.standing(&[]), Standing::Fewer);
        assert_eq!(
            c.standing(&[0, 1, 2, 6]),
            Standing::NotExcluding {
                alive: 3,
                holds: vec![0]
            }
        );
        assert_eq!(
            c.standing(&[0, 4]),
            Standing::NotExcluding {
                alive: 1,
                holds: vec![]
            }
        );

        // A layer read as far as its universe gives no mark, and root's
        // cliques are not claimed.
        let mut unread = layer_over(7, &[], &[]);
        unread.singles_left = 7;
        unread.stopped = Some(Stopped::NoSearch);
        let c = Cover::of(&unread, &small);
        assert!(c.marks.iter().all(|&m| m == Mark::Unread));
        assert!(c.root_cliques.is_none() && c.alive_at_root.is_none());
        assert_eq!(c.groups.len(), 5);
    }

    /// L5's reading of one instance: its alternatives, or why not.
    type Alternatives = Result<Vec<u32>, String>;

    /// What root owes over a hand-made cover: each instance its
    /// alternatives or a reason, with the witnesses given.
    fn owed_of(instances: Vec<(Alternatives, Vec<Witness>)>) -> Owed {
        Owed {
            root_owes: instances.len() as u32,
            instances: Ok(instances
                .into_iter()
                .enumerate()
                .map(|(k, (alternatives, witnesses))| OwedInstance {
                    rule: "owe".to_string(),
                    text: format!("(owe) k={k}"),
                    alternatives,
                    denied: 0,
                    witnesses: Ok(witnesses),
                })
                .collect()),
            singles_read: true,
            theory_read: true,
        }
    }

    fn witness(is: WitnessIs, at_root: &[u32], in_theory: &[u32]) -> Witness {
        Witness {
            text: "(w)".to_string(),
            is,
            held_at_root: at_root.to_vec(),
            held_in_theory: in_theory.to_vec(),
        }
    }

    /// **A grid is found from the groups alone, and a component that is none
    /// says why.** Two rows of three over three columns, a candidate alone,
    /// and beside them each shape the definition turns away: one group, a
    /// path, a ring of five, two cells in one row and one column, and a
    /// member in three groups.
    #[test]
    fn a_grid_is_two_families_and_every_other_component_says_why() {
        // 0 1 2
        // 3 4 5      a grid: rows {0,1,2} {3,4,5}, columns {0,3} {1,4} {2,5}.
        let mut edges = vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (3, 4),
            (3, 5),
            (4, 5),
            (0, 3),
            (1, 4),
            (2, 5),
        ];
        // 6, alone. 7 8 9: one group.
        edges.extend([(7, 8), (7, 9), (8, 9)]);
        // 10 - 11 - 12: a path, whose ends are in one group each.
        edges.extend([(10, 11), (11, 12)]);
        // 13 .. 17: a ring of five, every member in two groups of two, and
        // no two families.
        edges.extend([(13, 14), (14, 15), (15, 16), (16, 17), (13, 17)]);
        // 18 19 with 20, and with 21; 20 - 22 - 21: every member in two
        // groups, two families, and {18, 19, 20} and {18, 19, 21} share two.
        edges.extend([
            (18, 19),
            (18, 20),
            (19, 20),
            (18, 21),
            (19, 21),
            (20, 22),
            (21, 22),
        ]);
        // 23 with 24, 25 and 26, no two of which exclude: 23 is in three.
        edges.extend([(23, 24), (23, 25), (23, 26)]);
        let small = layer_over(27, &edges, &[]);
        let root = layer_over(27, &edges, &[1]);
        let c = Cover::of(&root, &small);
        let g = c.grids();
        assert_eq!(c.components.len(), 6);
        assert_eq!(
            g.grids,
            [Grid {
                component: 0,
                families: [vec![0, 4], vec![1, 2, 3]],
            }]
        );
        assert_eq!(c.groups[0], [0, 1, 2]);
        assert_eq!(c.groups[4], [3, 4, 5]);
        assert_eq!(g.of_group[..5], [0, 1, 1, 1, 0].map(|f| Some((0, f))));
        assert!(g.of_group[5..].iter().all(Option::is_none));
        assert_eq!(
            g.not,
            [
                (1, NotAGrid::OneGroup),
                (2, NotAGrid::InOne { members: 2 }),
                (3, NotAGrid::NoTwoFamilies),
                (4, NotAGrid::SharedTwice),
                (5, NotAGrid::MoreThanTwo { most: 3 }),
            ]
        );

        // The rows owed, one of them by an instance whose alternatives are
        // the row; a column with a strict subset; a set across two rows; an
        // instance with one alternative, one with none; and two L5 does not
        // license, the first's witnesses brought by the middle column's
        // members and by no other, the second's by one member of it.
        let unproposed =
            |at_root: &[u32], in_theory: &[u32]| witness(WitnessIs::Unproposed, at_root, in_theory);
        let why = || Err("a `w` fact can witness it that no candidate proposes".to_string());
        let x = ExactlyOne::of(
            &c,
            owed_of(vec![
                (Ok(vec![0, 1, 2]), vec![]),
                (Ok(vec![3, 4, 5]), vec![]),
                (Ok(vec![8, 9]), vec![]),
                (Ok(vec![0, 4]), vec![]),
                (Ok(vec![6]), vec![]),
                (Ok(vec![]), vec![]),
                (
                    why(),
                    vec![
                        unproposed(&[], &[1]),
                        unproposed(&[4], &[4]),
                        witness(WitnessIs::Denied, &[], &[]),
                    ],
                ),
                (why(), vec![unproposed(&[4], &[4]), unproposed(&[], &[])]),
            ]),
        );
        assert_eq!(
            x.owing,
            [
                Owing::Group(0),
                Owing::Group(4),
                Owing::Inside(vec![5]),
                Owing::NotExcluding {
                    alive: 1,
                    holds: vec![]
                },
                Owing::One(6),
                Owing::None,
                Owing::Unlicensed {
                    payers: Some(vec![1, 4]),
                    about: Some(2),
                },
                Owing::Unlicensed {
                    payers: Some(vec![4]),
                    about: None,
                },
            ]
        );
        assert_eq!(x.owed_groups(), [0, 4]);
        assert_eq!(x.unowed().len(), c.groups.len() - 2);
        assert_eq!(x.owed_by[0], [0]);
        assert_eq!(x.about[2], [6]);
        assert_eq!(x.not_groups(), [3]);
        assert_eq!(x.unlicensed(), [6, 7]);
        // The first has two witnesses that are no candidate and not denied,
        // one of them brought by a candidate alive alone at root.
        assert_eq!(x.brought(6), Some((2, 1)));
        assert_eq!(x.brought(7), Some((2, 1)));
        // One family owed whole, the other not: no bijection, and the
        // column an unlicensed instance is about is not owed by it.
        assert_eq!(
            x.share(0),
            [
                Share {
                    groups: 2,
                    owed: 2,
                    about: 0
                },
                Share {
                    groups: 3,
                    owed: 0,
                    about: 1
                },
            ]
        );
        assert!(!x.is_bijection(0) && !x.is_owed(2));

        // Every group owed, of two rows and three columns: no bijection,
        // the families differing in number.
        let all = ExactlyOne::of(
            &c,
            owed_of(
                [
                    vec![0, 1, 2],
                    vec![3, 4, 5],
                    vec![0, 3],
                    vec![1, 4],
                    vec![2, 5],
                ]
                .into_iter()
                .map(|a| (Ok(a), vec![]))
                .collect(),
            ),
        );
        assert_eq!(all.share(0).map(|s| (s.groups, s.owed)), [(2, 2), (3, 3)]);
        assert!(!all.is_bijection(0));
        // Two rows and two columns, every one owed: a bijection.
        let square = layer_over(4, &[(0, 1), (2, 3), (0, 2), (1, 3)], &[]);
        let c2 = Cover::of(&square, &square);
        let x2 = ExactlyOne::of(
            &c2,
            owed_of(
                [vec![0, 1], vec![2, 3], vec![0, 2], vec![1, 3]]
                    .into_iter()
                    .map(|a| (Ok(a), vec![]))
                    .collect(),
            ),
        );
        assert_eq!(x2.grids.grids.len(), 1);
        assert!(x2.is_bijection(0));
        // And with no table nothing is owed, whatever the groups.
        let none = ExactlyOne::of(
            &c,
            Owed {
                root_owes: 0,
                instances: Err(NoTable::NoObligation),
                singles_read: false,
                theory_read: false,
            },
        );
        assert!(none.owing.is_empty() && none.owed_groups().is_empty());
        assert_eq!(none.grids, g);
        assert_eq!(none.share(0).map(|s| s.owed), [0, 0]);
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
