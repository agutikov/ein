//! A model **set** as a determining key — M1d
//! [S1d.3.3](../../../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)'s
//! answer to *print or describe*.
//!
//! [P1d.3](../../../../docs/history/m1d_satisfiability/README.md#p1d3--model-sets)
//! priced five ways to say *"there are 32 models"* on four columns — produce ·
//! size · exact · read — and the one that won is **(b)**: the smallest set of
//! slots that tells every model apart, plus the table of the combinations that
//! occur. On `examples/zebra2-minus-15.ein` that is 4 columns of 23 and 32
//! rows, 2 506 bytes against the model set's 13 920 fact lines, and it is
//! *exact*: appending a row to the puzzle and re-solving recovers that model
//! to the fact, on 30 of the 32 rows without entering a single commitment
//! ([`representations.md` §4.1](../../../../docs/history/m1d_satisfiability/representations.md)).
//!
//! **It is a rendering and never a replacement.** `verdict.solutions`,
//! `--json-summary`, `--events` and `:expect` are untouched by
//! [`ModelsForm`]; the flag chooses which projection of the same model set
//! `ein solve` puts on stdout, and the models stay enumerable everywhere a
//! consumer reads them. When no key is affordable the form *is* the
//! enumeration — [`KeyOutcome::Unaffordable`], and (e) was a legitimate
//! winner of the pricing all along. *Affordable* is priced before it is
//! spent: in memory by [`PAIR_TABLE_BOUND`], since M1e2 S1e2.4.2, and in
//! search nodes by the two budgets below.
//!
//! ## What a decision variable is
//!
//! A model set varies in *facts*; a key is a claim about *variables*, so
//! something has to turn one into the other. Two rules, and they are the
//! census's ([`model_set_census.md`
//! §1](../../../../docs/history/m1d_satisfiability/model_set_census.md)):
//!
//! 1. **Every varying positive atom is a Boolean variable.** A fact is in a
//!    model or it is not; no declaration needed.
//! 2. **A relation the program declares `functional`** (or `bijective`, which
//!    fans out into it) makes the atoms `(R a ·)` mutually exclusive, so for
//!    each `a` they collapse into **one** variable whose domain is the set of
//!    values it takes. That is the only refinement, and the declaration is
//!    exactly the licence for it.
//!
//! The declarations are read from the **models' own facts**, not from the
//! source text, so a program that says `bijective`, one that says `functional`
//! and one that derives the marker by a rule are read identically. The
//! refinement is only as good as what the program says about itself:
//! `examples/features/11_expect_ambiguity.ein` is a bijection puzzle written
//! with hand-rolled activators, so its `(seat …)` atoms stay Boolean.
//!
//! **Varying negatives are not variables.** Where negative completion writes
//! `(not (R a b))` beside every excluded value the two halves mirror exactly,
//! and counting both would square the description.

use std::cmp::Ordering;

use ein_core::{FactId, Kb, Symbol, Terms, Value};

/// How `ein solve` prints a model **set** — the `--models` flag.
///
/// `List` is what the engine has always printed: one block per model. `Key`
/// is [P1d.3](../../../../docs/history/m1d_satisfiability/README.md#p1d3--model-sets)'s
/// (b). Neither changes what is *recorded*, and every verdict but `Ambiguity`
/// ignores the choice, because a single model is its own smallest description.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum ModelsForm {
    #[default]
    List,
    Key,
    /// Each model as the grid a person would draw, where the program licenses
    /// one — M1e2 S1e2.1.4, [`crate::grid`]. Read by every arm that prints a
    /// state, `Solution` and `Open` included.
    Grid,
}

/// **The search budget, in recursion nodes** — the whole of what stands
/// between `--models key` and a hang.
///
/// Finding a *minimum* determining set is a minimum hitting set over the model
/// pairs, which is NP-hard, and the corpus already holds the entry that shows
/// it: `examples/branching/06_lookahead_on.ein` has 42 varying slots and 22
/// models, its minimum key is **8**, and proving that no 7-set works costs
/// 12 s in `utils/model_set_census.py` — forty times its own solve. A representation
/// may not cost more than the answer it describes, so the search is bounded
/// and the over-budget outcome is *the enumeration*.
///
/// Calibrated on the two corpus cases it has to separate: `zebra2-minus-15`
/// (23 slots, 32 models, key size 4) finds its key well inside the budget, and
/// `branching/06` exceeds it.
const KEY_NODE_BUDGET: u64 = 2_000_000;

/// Beyond this many `C(varying, size)` candidate sets the *count* of minimum
/// keys is not taken, so neither is the tightest-key rule that depends on it.
///
/// A second budget rather than the same one, because the two searches answer
/// different questions: the first finds *a* size, the second enumerates
/// **every** key of that size to answer *"why these"*
/// ([`representations.md` §4.2](../../../../docs/history/m1d_satisfiability/representations.md)).
/// A form that printed one key without being able to say how many others there
/// are would be exactly the arbitrary basis S1d.3.2 warned about.
const KEY_TABLE_BUDGET: u128 = 4_000_000;

/// **The memory bound on a pair table, in bytes**: 16 MiB. M1e2
/// [S1e2.4.2](../../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md).
///
/// Two forms compare the listed states **pairwise**, and a table over pairs
/// is quadratic in them. The key form holds a mask of k(k−1)/2 bits per
/// varying slot ([`key_pair_table_bytes`]), and `--diff-models` holds the
/// candidates for its *near* pairing. Each prices its table from the counts
/// before it allocates anything, and past this bound it declines with the
/// numbers, which is an answer and not an error: the states are listed above
/// it either way.
///
/// The node budgets bound how long a search runs, and this bounds what it
/// holds. Before it, `ein solve examples/saturation/type-exclusivity/pets.ein
/// --models key` held 8.1 GB for its 11 339 states (2026-11-02), and was
/// OOM-killed on a machine that had less. The corpus has ten runs, `--maximal` or `--minimal`
/// over 64 000 states, that no machine could finish.
///
/// Calibrated on the corpus (2026-11-02): the largest table it answers is
/// `colors.ein`'s, 169 states over 8 slots, 88 752 bytes. The smallest it
/// declines is `constraint-scopes.ein`'s, 4 096 states over 9, 52 MiB, which
/// the node budget declined anyway.
pub const PAIR_TABLE_BOUND: u64 = 16 << 20;

/// The bytes the key form holds for `k` distinct states over `n` varying
/// slots, which is what [`PAIR_TABLE_BOUND`] is compared with:
///
/// - `n + 1` masks of k(k−1)/2 bits, one per slot and the full set
///   (`separating`);
/// - as many again for the search's path, one mask per level of depth;
/// - 4 bytes a pair for the order the search takes them in.
///
/// Saturating, because the magnitude is what the bound needs.
pub fn key_pair_table_bytes(k: usize, n: usize) -> u64 {
    let k = k as u128;
    let pairs = k * k.saturating_sub(1) / 2;
    let mask = pairs.div_ceil(64) * 8;
    let bytes = 2 * (n as u128 + 1) * mask + 4 * pairs;
    u64::try_from(bytes).unwrap_or(u64::MAX)
}

/// `491 MiB`, rounded up, because a bound is passed by any part of a byte.
fn mib(bytes: u128) -> String {
    format!("{} MiB", group(bytes.div_ceil(1 << 20)))
}

/// Why the key form will not build its table for `k` states over `n`
/// varying slots, or `None` when the table fits. `n` is `None` before the
/// variables are built, and the price is then the least any key over these
/// states could cost: one varying slot.
fn key_over_bound(k: usize, n: Option<usize>) -> Option<String> {
    let bytes = key_pair_table_bytes(k, n.unwrap_or(1));
    if bytes <= PAIR_TABLE_BOUND {
        return None;
    }
    let pairs = k as u128 * (k as u128).saturating_sub(1) / 2;
    let holds = match n {
        Some(n) => format!(
            "over {n} varying slot{} holds {}",
            if n == 1 { "" } else { "s" },
            mib(bytes as u128)
        ),
        None => format!(
            "holds at least {}, its size over a single varying slot",
            mib(bytes as u128)
        ),
    };
    Some(format!(
        "the {} states are {} pairs, and the pair table that tells them apart {holds}: \
         past the key form's bound of {}",
        group(k as u128),
        group(pairs),
        mib(PAIR_TABLE_BOUND as u128)
    ))
}

// ── decision variables ─────────────────────────────────────────────

/// One decision variable: a single-valued slot, or a bare atom.
#[derive(Clone, PartialEq, Eq)]
enum Var {
    /// `(R, a)` — one slot of a relation the program declares single-valued.
    /// Its value is whichever `(R a ·)` the model holds, or absent.
    Slot(Symbol, Value),
    /// A varying atom over a relation nobody declared functional. Boolean:
    /// the fact is in the model or it is not.
    Atom(FactId),
}

/// A variable's value in one model — `None` is *the slot is empty here*.
type Val = Option<Value>;

/// The label a column header carries.
fn var_label(terms: &Terms, v: &Var) -> String {
    match v {
        Var::Slot(rel, arg) => format!("{}:{}", terms.sym(*rel), terms.display(*arg)),
        Var::Atom(f) => ein_infer::events::sexpr(terms, *f),
    }
}

/// A cell. An absent slot is `—` and a Boolean is `yes` / `no`, because a
/// column of `0` and `1` under a relation name reads as a *value* named zero.
fn val_label(terms: &Terms, v: &Var, x: Val) -> String {
    match (v, x) {
        (Var::Slot(..), Some(x)) => terms.display(x),
        (Var::Slot(..), None) => "—".to_string(),
        (Var::Atom(_), Some(_)) => "yes".to_string(),
        (Var::Atom(_), None) => "no".to_string(),
    }
}

/// `(relation R …)` and `(functional R)` / `(bijective R)`, read off the facts
/// every model shares — which is where a declaration necessarily lives, since
/// it held before the search and no branch removed it.
fn single_valued(terms: &Terms, core: &[FactId]) -> Vec<Symbol> {
    let mut rels: Vec<Symbol> = Vec::new();
    let mut funct: Vec<Symbol> = Vec::new();
    for &f in core {
        let (rel, args) = terms.fact(f);
        match (terms.sym(rel), args.len()) {
            ("relation", n) if n >= 1 => rels.extend(args[0].as_sym()),
            ("functional" | "bijective", 1) => funct.extend(args[0].as_sym()),
            _ => {}
        }
    }
    funct.retain(|r| rels.contains(r));
    funct.sort_by_key(|r| terms.sym(*r).to_string());
    funct.dedup();
    funct
}

/// `k` fact sets, reduced to `k` assignments over one ordered variable list.
struct Vars {
    /// The varying variables, in print order.
    vary: Vec<Var>,
    /// One row per model, `vary`-aligned.
    rows: Vec<Vec<Val>>,
    /// `vary`-aligned domains, each sorted by rendered value.
    domains: Vec<Vec<Val>>,
}

/// Turn the model set into decision variables. `models` are distinct by
/// state, which [`key_table_of`] sees to before it prices the pair table.
///
/// `None` when there is nothing to key on — fewer than two models, or models
/// that differ in no *positive* fact. Handled rather than asserted, because a
/// rendering must not panic on a shape the engine can reach.
fn variables(terms: &Terms, models: &[&Kb]) -> Option<Vars> {
    if models.len() < 2 {
        return None;
    }
    // The union and the intersection, over `FactId` — one `Terms` interns
    // every model, so a fact is the same id wherever it holds.
    let mut union: Vec<FactId> = Vec::new();
    for m in models {
        union.extend(m.facts());
    }
    union.sort_by(|a, b| terms.cmp_fact_semantic(*a, *b));
    union.dedup();
    let in_core: Vec<bool> = union
        .iter()
        .map(|f| models.iter().all(|m| m.contains(*f)))
        .collect();
    let core: Vec<FactId> = union
        .iter()
        .zip(&in_core)
        .filter(|(_, c)| **c)
        .map(|(f, _)| *f)
        .collect();
    let funct = single_valued(terms, &core);

    // Every positive fact of the union contributes to at most one variable.
    //
    // **A functional slot is a variable wherever it is, varying or not** — a
    // slot the puzzle pinned is a variable with a one-value domain rather than
    // an invisible part of the core, which is what makes *"4 of 23 varying,
    // and 2 already stated"* sayable at all. An unrefined atom gets the
    // opposite treatment and enters only when it varies: a Boolean has no slot
    // apart from its own presence, so a *core* atom is not a fixed decision,
    // it is just a fact.
    let mut var_of: Vec<(Var, Val, FactId)> = Vec::new();
    for (&f, &is_core) in union.iter().zip(&in_core) {
        let (rel, args) = terms.fact(f);
        if terms.sym(rel) == "not" {
            continue;
        }
        if args.len() == 2 && funct.contains(&rel) {
            var_of.push((Var::Slot(rel, args[0]), Some(args[1]), f));
        } else if !is_core {
            var_of.push((Var::Atom(f), Some(Value::fact(f)), f));
        }
    }

    // Print order: the Boolean atoms first, then the slots, each group by its
    // label — the census's order, so a table printed here and a table printed
    // by `utils/model_set_census.py --form key` have the same columns.
    let mut names: Vec<Var> = Vec::new();
    for (v, _, _) in &var_of {
        if !names.contains(v) {
            names.push(v.clone());
        }
    }
    names.sort_by(|a, b| match (a, b) {
        (Var::Atom(_), Var::Slot(..)) => Ordering::Less,
        (Var::Slot(..), Var::Atom(_)) => Ordering::Greater,
        _ => var_label(terms, a).cmp(&var_label(terms, b)),
    });

    let mut rows: Vec<Vec<Val>> = vec![vec![None; names.len()]; models.len()];
    for (v, val, f) in &var_of {
        let col = names.iter().position(|n| n == v).expect("collected above");
        for (mi, m) in models.iter().enumerate() {
            if m.contains(*f) {
                rows[mi][col] = *val;
            }
        }
    }

    // Domains, then the varying projection: a variable whose domain is one
    // value is not a decision and takes no column.
    let mut domains: Vec<Vec<Val>> = vec![Vec::new(); names.len()];
    for row in &rows {
        for (i, x) in row.iter().enumerate() {
            if !domains[i].contains(x) {
                domains[i].push(*x);
            }
        }
    }
    for (i, d) in domains.iter_mut().enumerate() {
        d.sort_by_key(|x| val_label(terms, &names[i], *x));
    }
    let keep: Vec<usize> = (0..names.len()).filter(|&i| domains[i].len() > 1).collect();
    if keep.is_empty() {
        return None;
    }
    Some(Vars {
        vary: keep.iter().map(|&i| names[i].clone()).collect(),
        rows: rows
            .iter()
            .map(|r| keep.iter().map(|&i| r[i]).collect())
            .collect(),
        domains: keep.iter().map(|&i| domains[i].clone()).collect(),
    })
}

// ── the minimum determining set ────────────────────────────────────

/// A set of model pairs, as machine words. `k = 32` is 496 pairs, so a `u64`
/// is not enough and the width has to be dynamic.
#[derive(Clone, PartialEq, Eq)]
struct Mask(Vec<u64>);

impl Mask {
    fn zero(bits: usize) -> Mask {
        Mask(vec![0; bits.div_ceil(64)])
    }
    fn set(&mut self, b: usize) {
        self.0[b / 64] |= 1 << (b % 64);
    }
    fn get(&self, b: usize) -> bool {
        self.0[b / 64] >> (b % 64) & 1 == 1
    }
    fn or(&self, o: &Mask) -> Mask {
        Mask(self.0.iter().zip(&o.0).map(|(a, b)| a | b).collect())
    }
    fn count(&self) -> u32 {
        self.0.iter().map(|w| w.count_ones()).sum()
    }
}

/// Per variable, the pairs of models it tells apart.
///
/// A variable set determines the model exactly when the OR of its masks is
/// full, which makes *the minimum determining set* a minimum hitting set and
/// lets both the search and the count run on machine words.
fn separating(v: &Vars) -> (Vec<Mask>, Mask, usize) {
    let k = v.rows.len();
    let bits = k * (k - 1) / 2;
    let mut sep = vec![Mask::zero(bits); v.vary.len()];
    let mut b = 0;
    for i in 0..k {
        for j in i + 1..k {
            for (c, mask) in sep.iter_mut().enumerate() {
                if v.rows[i][c] != v.rows[j][c] {
                    mask.set(b);
                }
            }
            b += 1;
        }
    }
    let mut full = Mask::zero(bits);
    for b in 0..bits {
        full.set(b);
    }
    (sep, full, bits)
}

/// The state both searches thread: what is left of the budget.
///
/// `None` out of either search means *the budget ran out*, and it is a real
/// answer rather than an error — the caller prints the models.
struct Budget(u64);

impl Budget {
    fn spend(&mut self) -> Option<()> {
        self.0 = self.0.checked_sub(1)?;
        Some(())
    }
}

/// The smallest determining set's size, by iterative deepening.
///
/// Branch on the *hardest* uncovered pair — the one fewest variables separate
/// — so the tree is narrow where it matters. The budget is checked at every
/// node rather than once per deepening round, because it is a single round
/// that blows up and a per-round check would let the last one run unbounded.
fn min_key_size(sep: &[Mask], full: &Mask, bits: usize, budget: &mut Budget) -> Option<usize> {
    // Widest-first, so the shallow branches cover the most pairs.
    let mut order: Vec<usize> = (0..sep.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(sep[i].count()));

    // **Which pair is hardest does not depend on the path**, so the pairs are
    // ordered once, by how many variables separate each, and a node takes the
    // first *uncovered* pair in that order. That is the same choice a
    // per-node minimum makes — ties included, since the order is stable and
    // leaves equal pairs in bit order — and it takes a node's cost from
    // `O(pairs x variables)` to `O(pairs + variables)`, which is what makes a
    // budget counted in **nodes** mean anything in seconds.
    let pairs = hardest_first(sep, bits);

    for d in 1..=sep.len() {
        if hits(&Mask::zero(bits), d, sep, full, &order, &pairs, budget)? {
            return Some(d);
        }
    }
    None
}

/// Every pair, fewest separating variables first and in bit order among
/// equals: a counting sort on the width, so the order is 4 bytes a pair.
///
/// It replaced a list of the separating variables per pair, sorted (M1e2
/// S1e2.4.2): the same order, but on `pets.ein`'s 64 280 791 pairs the lists
/// were 6 833 MiB and the sort's scratch 982 MiB. A node now reads a pair's
/// variables off the masks, which is `O(variables)` and allocates nothing.
fn hardest_first(sep: &[Mask], bits: usize) -> Vec<u32> {
    let width = |b: usize| sep.iter().filter(|m| m.get(b)).count();
    let mut start = vec![0usize; sep.len() + 2];
    for b in 0..bits {
        start[width(b) + 1] += 1;
    }
    for w in 1..start.len() {
        start[w] += start[w - 1];
    }
    let mut pairs = vec![0u32; bits];
    for b in 0..bits {
        let w = width(b);
        pairs[start[w]] = u32::try_from(b).expect("PAIR_TABLE_BOUND keeps the pairs below 2^32");
        start[w] += 1;
    }
    pairs
}

fn hits(
    covered: &Mask,
    depth: usize,
    sep: &[Mask],
    full: &Mask,
    order: &[usize],
    pairs: &[u32],
    budget: &mut Budget,
) -> Option<bool> {
    if covered == full {
        return Some(true);
    }
    if depth == 0 {
        return Some(false);
    }
    budget.spend()?;
    // The uncovered pair separated by fewest variables: the branch that has to
    // be taken, and the cheapest one to take.
    let Some(&b) = pairs.iter().find(|&&b| !covered.get(b as usize)) else {
        // Every pair covered while `covered != full`: unreachable so long as
        // `full` is exactly the pair set, and `false` is the safe reading.
        return Some(false);
    };
    for &v in order.iter().filter(|&&v| sep[v].get(b as usize)) {
        if hits(
            &covered.or(&sep[v]),
            depth - 1,
            sep,
            full,
            order,
            pairs,
            budget,
        )? {
            return Some(true);
        }
    }
    Some(false)
}

/// What [`all_keys`] found.
struct Keys {
    /// How many determining sets of the minimum size exist. The count is what
    /// says whether the basis is a *choice* or an accident.
    count: u64,
    /// The tightest of them — the key whose domains allow fewest combinations,
    /// which is a stated rule a reader can check rather than a coin toss.
    tightest: Vec<usize>,
    /// How many combinations that key's domains allow, against the `k` that
    /// occur.
    product: u128,
    /// What **every** minimum key contains — the answer to *"why these"*. On
    /// `zebra2-minus-15` two of the four columns are in all 22, so only the
    /// other two are a choice at all.
    always: Vec<usize>,
}

/// Every determining set of exactly `size`.
fn all_keys(
    v: &Vars,
    sep: &[Mask],
    full: &Mask,
    bits: usize,
    size: usize,
    budget: &mut Budget,
) -> Option<Keys> {
    let mut out = Keys {
        count: 0,
        tightest: Vec::new(),
        product: 0,
        always: Vec::new(),
    };
    let mut chosen: Vec<usize> = Vec::new();
    walk_keys(
        0,
        &Mask::zero(bits),
        &mut chosen,
        size,
        v,
        sep,
        full,
        budget,
        &mut out,
    )?;
    (out.count > 0).then_some(out)
}

#[allow(clippy::too_many_arguments)]
fn walk_keys(
    start: usize,
    covered: &Mask,
    chosen: &mut Vec<usize>,
    size: usize,
    v: &Vars,
    sep: &[Mask],
    full: &Mask,
    budget: &mut Budget,
    out: &mut Keys,
) -> Option<()> {
    let need = size - chosen.len();
    if need == 0 {
        if covered == full {
            out.count += 1;
            let product: u128 = chosen.iter().map(|&i| v.domains[i].len() as u128).product();
            if out.count == 1 || product < out.product {
                out.tightest = chosen.clone();
                out.product = product;
            }
            out.always = if out.count == 1 {
                chosen.clone()
            } else {
                out.always
                    .iter()
                    .copied()
                    .filter(|i| chosen.contains(i))
                    .collect()
            };
        }
        return Some(());
    }
    let n = sep.len();
    if n - start < need {
        return Some(());
    }
    budget.spend()?;
    for i in start..=n - need {
        chosen.push(i);
        walk_keys(
            i + 1,
            &covered.or(&sep[i]),
            chosen,
            size,
            v,
            sep,
            full,
            budget,
            out,
        )?;
        chosen.pop();
    }
    Some(())
}

// ── the form ───────────────────────────────────────────────────────

/// What `--models key` found — and the second arm is a first-class answer,
/// not an error.
pub enum KeyOutcome {
    /// The key table, ready to print.
    Table(Vec<String>),
    /// No key was affordable, with the reason. The caller prints the models,
    /// because the enumeration is the fallback and always was. Three things
    /// decline, each priced before it is spent: a pair table past
    /// [`PAIR_TABLE_BOUND`], a search past its node budget, and a count of
    /// minimum keys past its candidate budget.
    Unaffordable(String),
}

/// The determining key of a model set, rendered.
///
/// `exhausted` is not a footnote here: it decides *what the table is a table
/// of*. With the lattice exhausted the rows **are** the models; without it
/// they are the models the search recorded, and two things can go wrong that
/// cannot go wrong with a proof — an unfound model may **add** a row, or
/// **share** one, and a shared row means the key separates the models found
/// rather than the models.
///
/// What cannot go wrong either way is a row's *content*: its values are read
/// off a model that exists, so no row is ever falsified. The form fails in its
/// margins where the envelope (a) fails in its cells — an intersection over a
/// subset is a superset of the truth, so a 33rd model can contradict a printed
/// fact there and only a printed *claim about completeness* here.
pub fn key_table(terms: &Terms, models: &[&Kb], exhausted: bool, indent: &str) -> KeyOutcome {
    key_table_of(terms, models, exhausted, indent, KeyOf::Models)
}

/// What a key table's rows are, which decides the guarantee its last
/// sentence gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyOf {
    /// The walk's models.
    Models,
    /// States of R (M1e2 T1e2.1a.5.2). `maximal`: every one is on the maximal
    /// line, so a row added to the program leaves R that state alone; a
    /// state below the line leaves R the states above it.
    States { maximal: bool },
    /// The states stdout showed, a part of the `listed` the run listed (M1e2
    /// S1e2.3.3): the rows tell these apart and no more, since a state not
    /// shown may share a row, so the table claims nothing of R.
    Shown { listed: usize },
}

/// A key table in its three parts — M1e2 S1e2.3.3, so a caller that shares
/// stdout's lines can cut the rows and keep the rest: the lines above the
/// rows (the size, why these, the column heads), one line per row, and the
/// guarantee under them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyParts {
    pub head: Vec<String>,
    pub rows: Vec<String>,
    pub tail: Vec<String>,
}

/// [`key_table_of`] in its parts, or the reason it declined.
pub fn key_parts_of(
    terms: &Terms,
    models: &[&Kb],
    exhausted: bool,
    indent: &str,
    of: KeyOf,
) -> Result<KeyParts, String> {
    key_build(terms, models, exhausted, indent, of)
}

/// [`key_table`] over `of`.
pub fn key_table_of(
    terms: &Terms,
    models: &[&Kb],
    exhausted: bool,
    indent: &str,
    of: KeyOf,
) -> KeyOutcome {
    match key_build(terms, models, exhausted, indent, of) {
        Ok(p) => KeyOutcome::Table(p.head.into_iter().chain(p.rows).chain(p.tail).collect()),
        Err(why) => KeyOutcome::Unaffordable(why),
    }
}

fn key_build(
    terms: &Terms,
    models: &[&Kb],
    exhausted: bool,
    indent: &str,
    of: KeyOf,
) -> Result<KeyParts, String> {
    // `k` is one per **distinct** model, which is what it means everywhere
    // else in the verdict, and *the same model* is the same state key
    // (`ein_infer::canon::state_key`). Two identical models are a pair no
    // variable separates, so the hitting set would be unsatisfiable and the
    // search would spend its whole budget proving it — a wrong answer arrived
    // at expensively, which is the worst of both. This is public, so it cannot
    // assume its caller deduplicated: `answer.rs` hands it `Verdict::models`,
    // which is distinct by construction, and an embedder may hand it anything.
    //
    // **Priced before anything quadratic is allocated** (S1e2.4.2): first on
    // k alone, then on the real n. The first price takes k by sorting the
    // keys, `O(k log k)`, because `distinct_by_state`'s scan is `O(k²)` and
    // spent 6.2 s of a 64 514-state line's 8.0 s before it could decline.
    let k = {
        let keys: Vec<Box<[FactId]>> = models
            .iter()
            .map(|m| ein_infer::canon::state_key(m))
            .collect();
        let mut keys: Vec<&[FactId]> = keys.iter().map(|k| &**k).collect();
        // determinism-ok: a count, and no order of these keys reaches the
        // output.
        keys.sort_unstable();
        keys.dedup();
        keys.len()
    };
    if k >= 2
        && let Some(why) = key_over_bound(k, None)
    {
        return Err(why);
    }
    let models: Vec<&Kb> = ein_infer::canon::distinct_by_state(models, |m| *m)
        .into_iter()
        .copied()
        .collect();
    debug_assert_eq!(models.len(), k, "one notion of the same state");
    let Some(v) = variables(terms, &models) else {
        return Err("the models differ in no single-valued slot and no varying atom".to_string());
    };
    let n = v.vary.len();
    if let Some(why) = key_over_bound(k, Some(n)) {
        return Err(why);
    }
    let (sep, full, bits) = separating(&v);
    let mut budget = Budget(KEY_NODE_BUDGET);
    let Some(size) = min_key_size(&sep, &full, bits, &mut budget) else {
        return Err(format!(
            "no determining set of the {n} varying slots was found within \
             {KEY_NODE_BUDGET} search nodes"
        ));
    };
    let combos = binomial(n, size);
    if combos > KEY_TABLE_BUDGET {
        return Err(format!(
            "the smallest determining set is {size} of {n} varying slots, and \
             C({n}, {size}) = {} candidates is over the budget",
            group(combos)
        ));
    }
    let Some(keys) = all_keys(&v, &sep, &full, bits, size, &mut budget) else {
        return Err(format!(
            "the smallest determining set is {size} of {n} varying slots, and enumerating \
             them exceeded {KEY_NODE_BUDGET} search nodes"
        ));
    };

    let mut out: Vec<String> = Vec::new();
    let found = keys.count;
    out.push(format!(
        "{indent}determining key — {size} of {n} varying slots"
    ));
    let mut prose = format!(
        "{found} {size}-set{} determine{} the model; this one's domains allow fewest \
         combinations — {}, of which {k} occur.",
        if found == 1 { "" } else { "s" },
        if found == 1 { "s" } else { "" },
        group(keys.product)
    );
    // Only worth saying when there is a choice to explain: with one minimum
    // key, *"every one of the 1 contains …"* names the key twice.
    if found > 1 && !keys.always.is_empty() {
        let names: Vec<String> = keys
            .always
            .iter()
            .map(|&i| var_label(terms, &v.vary[i]))
            .collect();
        prose.push(' ');
        prose.push_str(&format!(
            "Every one of the {found} contains {}.",
            names.join(", ")
        ));
    }
    out.extend(wrap(&prose, &format!("{indent}  ")));
    out.push(String::new());

    let head: Vec<String> = keys
        .tightest
        .iter()
        .map(|&i| var_label(terms, &v.vary[i]))
        .collect();
    let mut rows: Vec<Vec<String>> = v
        .rows
        .iter()
        .map(|r| {
            keys.tightest
                .iter()
                .map(|&i| val_label(terms, &v.vary[i], r[i]))
                .collect()
        })
        .collect();
    rows.sort();
    let w: Vec<usize> = head
        .iter()
        .enumerate()
        .map(|(c, h)| {
            rows.iter()
                .map(|r| r[c].chars().count())
                .max()
                .unwrap_or(0)
                .max(h.chars().count())
        })
        .collect();
    out.push(pad_row(indent, &head, &w));
    out.push(pad_row(
        indent,
        &w.iter().map(|n| "-".repeat(*n)).collect::<Vec<_>>(),
        &w,
    ));
    let table: Vec<String> = rows.iter().map(|r| pad_row(indent, r, &w)).collect();
    let mut tail = vec![String::new()];

    // The guarantee, and it is the whole reason this is a representation and
    // not merely a smaller print: re-saturating with a row is what makes the
    // table exact rather than a summary of one.
    let rest = match n - size {
        0 => String::new(),
        1 => "; the other varying slot follows".to_string(),
        m => format!("; the other {m} varying slots follow"),
    };
    let found = if exhausted { "" } else { " found" };
    let alone = if exhausted { " alone" } else { "" };
    let mut prose = match of {
        KeyOf::Models => format!(
            "{} rows, one per model{found}. Add a row's facts to the program and it re-solves \
             to that model{alone}{rest}.",
            rows.len(),
        ),
        KeyOf::States { maximal: true } => format!(
            "{} rows, one per state{found}. Add a row's facts to the program and its R is \
             that state{alone}{rest}.",
            rows.len(),
        ),
        KeyOf::States { maximal: false } => format!(
            "{} rows, one per state{found}{rest}. Not every state is maximal: a row's facts \
             added to the program make its state root, and R the states above it.",
            rows.len(),
        ),
        KeyOf::Shown { listed } => format!(
            "{} rows, one per state shown above{rest}. They tell these states apart and no \
             more: the run listed {}, and a state not shown may share a row.",
            rows.len(),
            group(listed as u128),
        ),
    };
    if !exhausted && !matches!(of, KeyOf::Shown { .. }) {
        // *Alone* is the word the caveat takes back, and taking it back is the
        // whole of what `exhausted = false` costs this form. A row's four
        // values still hold in the model they came from — no row can be
        // falsified — but an unfound model may **share** a row, and then the
        // key separates the models found and not the models.
        prose.push_str(
            " A lower bound: the search did not exhaust the lattice, so a further model \
             would add a row — or share one, which would mean this key is too small.",
        );
    }
    tail.extend(wrap(&prose, &format!("{indent}  ")));
    Ok(KeyParts {
        head: out,
        rows: table,
        tail,
    })
}

/// Greedy wrap at 78 columns, indent included — the width the record prices
/// (b) at, and the reason it beat the envelope on the *read* column: a form
/// that does not fit a page is a form nobody reads.
pub fn wrap(text: &str, indent: &str) -> Vec<String> {
    // A grouped number is one word. `group` separates its triples with an
    // ordinary space — the repo's typography everywhere else — so the wrapper
    // has to know that `118 030 185` may not be broken after `118`, and this
    // is cheaper than putting a non-ASCII space into every terminal.
    let mut words: Vec<String> = Vec::new();
    for w in text.split_whitespace() {
        let joins = w.starts_with(|c: char| c.is_ascii_digit())
            && words
                .last()
                .is_some_and(|p: &String| p.ends_with(|c: char| c.is_ascii_digit()));
        match (joins, words.last_mut()) {
            (true, Some(prev)) => {
                prev.push(' ');
                prev.push_str(w);
            }
            _ => words.push(w.to_string()),
        }
    }
    let mut out: Vec<String> = Vec::new();
    let mut line = indent.to_string();
    for word in &words {
        if line.chars().count() > indent.chars().count()
            && line.chars().count() + 1 + word.chars().count() > 78
        {
            out.push(line);
            line = indent.to_string();
        } else if line.chars().count() > indent.chars().count() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if line.trim().is_empty() {
        return out;
    }
    out.push(line);
    out
}

/// One table row, padded to the column widths. Trailing blanks are trimmed,
/// so a short last cell does not leave the line ragged.
fn pad_row(indent: &str, cells: &[String], w: &[usize]) -> String {
    let mut s = format!("{indent}  ");
    for (i, c) in cells.iter().enumerate() {
        s.push_str(c);
        if i + 1 < cells.len() {
            s.push_str(&" ".repeat(w[i].saturating_sub(c.chars().count()) + 2));
        }
    }
    s.trim_end().to_string()
}

/// `C(n, k)`, saturating — the budget test needs the magnitude, not the digits.
fn binomial(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut acc: u128 = 1;
    for i in 0..k as u128 {
        acc = acc.saturating_mul(n as u128 - i) / (i + 1);
        if acc >= u128::MAX / 2 {
            return u128::MAX / 2;
        }
    }
    acc
}

/// `118 030 185` — thin-space grouping, the census's, so a nine-digit
/// candidate count reads as one number.
fn group(n: u128) -> String {
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

// ── two model sets, compared — M1e2 S1e2.1.4 ─────────────────────────

/// One recorded model that nearly matches an expected one: paired because they
/// share most of their facts, and printed as the facts that differ — so a
/// wrong model that is one fact off reads as *one wrong fact* rather than as a
/// missing model beside an unrelated spurious one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Near {
    pub got: usize,
    pub want: usize,
    /// In the recorded model and not the expected one.
    pub extra: Vec<String>,
    /// In the expected model and not the recorded one.
    pub lacking: Vec<String>,
}

/// How a recorded model set differs from an expected one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SetDiff {
    /// `(got, want)` — equal as fact sets.
    pub matched: Vec<(usize, usize)>,
    pub near: Vec<Near>,
    /// Expected models nothing recorded.
    pub missing: Vec<usize>,
    /// Recorded models nothing expected.
    pub spurious: Vec<usize>,
    /// Why nothing was paired *near*, when the candidates would have passed
    /// [`PAIR_TABLE_BOUND`] (M1e2 S1e2.4.2). Every unmatched model is then a
    /// missing or a spurious one, which is what it is.
    pub unpaired: Option<String>,
}

impl SetDiff {
    pub fn agrees(&self) -> bool {
        self.near.is_empty() && self.missing.is_empty() && self.spurious.is_empty()
    }
}

/// Compare two model sets, each model a list of facts.
///
/// **Equality first**, as a multiset: a model recorded twice against one
/// expected once is one match and one spurious. **Then nearest** among what
/// is left, by symmetric difference, greedily and smallest first, ties to the
/// lower indices — paired only while the difference is at most half the
/// larger model (and at least one fact), so two unrelated models stay a
/// *missing* and a *spurious* rather than becoming a misleading pair.
///
/// **The near pairing is priced before it is taken** (M1e2 S1e2.4.2). It
/// holds every unmatched (got, want) pair within the distance bound, 24 bytes
/// each, which is quadratic: 889 MiB on `pets.ein`'s 11 339 states against a
/// file of them each one fact off (2026-11-02). Past [`PAIR_TABLE_BOUND`] it pairs nothing
/// and says why in [`SetDiff::unpaired`].
pub fn diff_model_sets(got: &[Vec<String>], want: &[Vec<String>]) -> SetDiff {
    use std::collections::BTreeSet;
    let g: Vec<BTreeSet<&str>> = got
        .iter()
        .map(|m| m.iter().map(String::as_str).collect())
        .collect();
    let w: Vec<BTreeSet<&str>> = want
        .iter()
        .map(|m| m.iter().map(String::as_str).collect())
        .collect();
    let mut out = SetDiff::default();
    let mut g_free: Vec<bool> = vec![true; g.len()];
    let mut w_free: Vec<bool> = vec![true; w.len()];
    for (i, gi) in g.iter().enumerate() {
        if let Some(j) = (0..w.len()).find(|&j| w_free[j] && w[j] == *gi) {
            out.matched.push((i, j));
            g_free[i] = false;
            w_free[j] = false;
        }
    }
    let (gn, wn) = (
        g_free.iter().filter(|f| **f).count(),
        w_free.iter().filter(|f| **f).count(),
    );
    let candidates = gn as u128 * wn as u128;
    let bytes = candidates * std::mem::size_of::<(usize, usize, usize)>() as u128;
    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    if bytes > PAIR_TABLE_BOUND as u128 {
        out.unpaired = Some(format!(
            "no model was paired as near: {} unmatched recorded against {} unmatched expected \
             are {} candidate pairs, {} at {} bytes each, past the bound of {} on a pair table",
            group(gn as u128),
            group(wn as u128),
            group(candidates),
            mib(bytes),
            std::mem::size_of::<(usize, usize, usize)>(),
            mib(PAIR_TABLE_BOUND as u128)
        ));
    } else {
        // Reserved whole, so what it holds is what was priced, and sorted
        // unstably, which allocates nothing and is the same order: no two
        // candidates share an `(i, j)`.
        pairs.reserve_exact(candidates as usize);
        for i in (0..g.len()).filter(|&i| g_free[i]) {
            for j in (0..w.len()).filter(|&j| w_free[j]) {
                let d = g[i].symmetric_difference(&w[j]).count();
                let bound = (g[i].len().max(w[j].len()) / 2).max(1);
                if d <= bound {
                    pairs.push((d, i, j));
                }
            }
        }
        pairs.sort_unstable();
    }
    for (_, i, j) in pairs {
        if g_free[i] && w_free[j] {
            g_free[i] = false;
            w_free[j] = false;
            out.near.push(Near {
                got: i,
                want: j,
                extra: g[i].difference(&w[j]).map(|s| s.to_string()).collect(),
                lacking: w[j].difference(&g[i]).map(|s| s.to_string()).collect(),
            });
        }
    }
    out.near.sort_by_key(|n| n.got);
    out.missing = (0..w.len()).filter(|&j| w_free[j]).collect();
    out.spurious = (0..g.len()).filter(|&i| g_free[i]).collect();
    out
}

/// An expected model set, as `--diff-models FILE` reads it.
///
/// ```text
/// ;; a comment
/// closes p q          compare over these relations only (optional)
/// model
/// (p A)
/// (q A)
/// model               a model with no fact over the closed relations
/// ```
///
/// `closes` is `corpus/wrong_answers.toml`'s rule — naming a relation closes
/// it — and is what lets a file say *one model with no `q`*. Without it a
/// model is compared as its positive facts minus the program's given ones
/// ([`solution_semantics.md` § 3](../../../../docs/kernel/inference/solution_semantics.md)).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Expected {
    pub closes: Option<Vec<String>>,
    pub models: Vec<Vec<String>>,
}

pub fn parse_expected(text: &str) -> Result<Expected, String> {
    let mut out = Expected::default();
    let mut current: Option<Vec<String>> = None;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("closes") {
            if out.closes.is_some() {
                return Err(format!("line {}: a second `closes` line", n + 1));
            }
            out.closes = Some(rest.split_whitespace().map(str::to_string).collect());
        } else if line == "model" {
            if let Some(m) = current.take() {
                out.models.push(m);
            }
            current = Some(Vec::new());
        } else if line.starts_with('(') {
            match current.as_mut() {
                Some(m) => m.push(line.to_string()),
                None => return Err(format!("line {}: a fact before the first `model`", n + 1)),
            }
        } else {
            return Err(format!(
                "line {}: expected `closes …`, `model` or a fact, got {line:?}",
                n + 1
            ));
        }
    }
    if let Some(m) = current {
        out.models.push(m);
    }
    for m in &mut out.models {
        m.sort();
    }
    Ok(out)
}

/// A recorded state's facts the way an [`Expected`] compares them: over
/// `closes` when it names relations, otherwise every positive fact that is not
/// a given (`:source`) one and not a relation declaration.
pub fn comparable_facts(terms: &Terms, kb: &Kb, closes: Option<&[String]>) -> Vec<String> {
    let mut out: Vec<String> = kb
        .facts()
        .filter(|&f| {
            let rel = terms.sym(terms.fact(f).0);
            match closes {
                Some(c) => c.iter().any(|r| r == rel),
                None => {
                    rel != "not"
                        && rel != "relation"
                        && kb
                            .primary(f)
                            .is_none_or(|p| terms.provs.get(p).kind != ein_core::ProvKind::Source)
                }
            }
        })
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect();
    out.sort();
    out
}

/// The diff as lines: what it compared, then every disagreement by name.
/// `got_label(i)` and `want_label(j)` say what each index is called.
pub fn diff_lines(
    diff: &SetDiff,
    what: &str,
    got_label: &dyn Fn(usize) -> String,
    want_label: &dyn Fn(usize) -> String,
    indent: &str,
) -> Vec<String> {
    let (head, items) = diff_parts(diff, what, got_label, want_label, indent);
    head.into_iter()
        .chain(items.into_iter().flatten())
        .collect()
}

/// [`diff_lines`] as its head (what was compared, the counts, why nothing
/// paired) and its items, one per matched pair, near pair, missing model and
/// spurious state — M1e2 S1e2.3.3, so a caller that shares stdout's lines can
/// cut the items.
pub fn diff_parts(
    diff: &SetDiff,
    what: &str,
    got_label: &dyn Fn(usize) -> String,
    want_label: &dyn Fn(usize) -> String,
    indent: &str,
) -> (Vec<String>, Vec<Vec<String>>) {
    let mut head = vec![format!("{indent}{what}")];
    head.push(format!(
        "{indent}  {} matched, {} near, {} missing, {} spurious — {}",
        diff.matched.len(),
        diff.near.len(),
        diff.missing.len(),
        diff.spurious.len(),
        if diff.agrees() {
            "the sets agree"
        } else {
            "the sets differ"
        }
    ));
    if let Some(why) = &diff.unpaired {
        head.extend(wrap(&format!("{why}."), &format!("{indent}  ")));
    }
    let mut items: Vec<Vec<String>> = Vec::new();
    for (i, j) in &diff.matched {
        items.push(vec![format!(
            "{indent}  = {} is {}",
            got_label(*i),
            want_label(*j)
        )]);
    }
    for n in &diff.near {
        let mut b = vec![format!(
            "{indent}  ~ {} is nearly {}",
            got_label(n.got),
            want_label(n.want)
        )];
        for f in &n.lacking {
            b.push(format!("{indent}      lacks {f}"));
        }
        for f in &n.extra {
            b.push(format!("{indent}      has   {f}"));
        }
        items.push(b);
    }
    for j in &diff.missing {
        items.push(vec![format!("{indent}  - missing: {}", want_label(*j))]);
    }
    for i in &diff.spurious {
        items.push(vec![format!("{indent}  + spurious: {}", got_label(*i))]);
    }
    (head, items)
}

#[cfg(test)]
mod key_tests {
    use super::*;

    /// **The counting sort is the order the stable sort gave** (S1e2.4.2):
    /// the pairs by how many variables separate them, ties in bit order. The
    /// list it replaced is rebuilt here, the way `min_key_size` built it, and
    /// compared on masks where widths repeat, so ties are exercised.
    #[test]
    fn the_hardest_first_order_is_the_sorted_lists_order() {
        let bits = 997;
        for seed in 1..=6u64 {
            let mut x = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut next = || {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                x
            };
            let n = 3 + seed as usize;
            let sep: Vec<Mask> = (0..n)
                .map(|_| {
                    let mut m = Mask::zero(bits);
                    for b in 0..bits {
                        if next() % 3 == 0 {
                            m.set(b);
                        }
                    }
                    m
                })
                .collect();
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by_key(|&i| std::cmp::Reverse(sep[i].count()));
            let mut by_pair: Vec<(usize, Vec<usize>)> = (0..bits)
                .map(|b| {
                    (
                        b,
                        order.iter().copied().filter(|&v| sep[v].get(b)).collect(),
                    )
                })
                .collect();
            by_pair.sort_by_key(|(_, c)| c.len());
            let was: Vec<u32> = by_pair.iter().map(|(b, _)| *b as u32).collect();
            assert_eq!(hardest_first(&sep, bits), was, "seed {seed}");
        }
    }
}

#[cfg(test)]
mod diff_tests {
    use super::*;

    fn s(v: &[&[&str]]) -> Vec<Vec<String>> {
        v.iter()
            .map(|m| m.iter().map(|f| f.to_string()).collect())
            .collect()
    }

    #[test]
    fn a_missing_model_is_named() {
        let d = diff_model_sets(
            &s(&[&["(p B)", "(p C)"]]),
            &s(&[&["(p A)"], &["(p B)", "(p C)"]]),
        );
        assert_eq!(d.matched, [(0, 1)]);
        assert_eq!(d.missing, [0]);
        assert!(d.spurious.is_empty() && d.near.is_empty());
    }

    #[test]
    fn a_spurious_model_is_named() {
        let got = s(&[
            &["(s A 1)", "(s B 2)"],
            &["(s A 2)", "(s B 1)"],
            &["(s A 1)", "(s A 2)", "(s B 1)"],
        ]);
        let want = s(&[&["(s A 1)", "(s B 2)"], &["(s A 2)", "(s B 1)"]]);
        let d = diff_model_sets(&got, &want);
        assert_eq!(d.matched.len(), 2);
        assert_eq!(d.spurious, [2]);
        assert!(d.missing.is_empty() && d.near.is_empty());
    }

    #[test]
    fn a_model_one_fact_off_reads_as_one_wrong_fact() {
        let d = diff_model_sets(&s(&[&["(q A)"]]), &s(&[&["(p A)", "(q A)"]]));
        assert_eq!(
            d.near,
            [Near {
                got: 0,
                want: 0,
                extra: vec![],
                lacking: vec!["(p A)".to_string()],
            }]
        );
        assert!(d.missing.is_empty() && d.spurious.is_empty());
    }

    #[test]
    fn unrelated_models_are_not_paired() {
        let d = diff_model_sets(&s(&[&["(p one)"]]), &s(&[&["(q one)"]]));
        assert!(d.near.is_empty());
        assert_eq!(
            (d.missing.as_slice(), d.spurious.as_slice()),
            (&[0][..], &[0][..])
        );
    }

    #[test]
    fn the_empty_model_is_a_model() {
        let e = parse_expected("closes q\nmodel\n").expect("parses");
        assert_eq!(e.closes.as_deref(), Some(&["q".to_string()][..]));
        assert_eq!(e.models, vec![Vec::<String>::new()]);
        assert!(diff_model_sets(&s(&[&[]]), &e.models).agrees());
        assert_eq!(diff_model_sets(&[], &e.models).missing, [0]);
    }

    #[test]
    fn the_file_format_refuses_what_it_cannot_read() {
        assert!(parse_expected("(p A)\n").is_err());
        assert!(parse_expected("model\np A\n").is_err());
        assert!(parse_expected("closes p\ncloses q\n").is_err());
    }
}
