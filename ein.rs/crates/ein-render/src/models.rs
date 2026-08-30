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
//! winner of the pricing all along.
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
}

/// **The search budget, in recursion nodes** — the whole of what stands
/// between `--models key` and a hang.
///
/// Finding a *minimum* determining set is a minimum hitting set over the model
/// pairs, which is NP-hard, and the corpus already holds the entry that shows
/// it: `examples/branching/06_lookahead_on.ein` has 42 varying slots and 22
/// models, its minimum key is **8**, and proving that no 7-set works costs
/// 12 s in the census's Python — forty times its own solve. A representation
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

/// Turn the model set into decision variables.
///
/// `None` when there is nothing to key on — fewer than two models, or models
/// that differ in no *positive* fact. Handled rather than asserted, because a
/// rendering must not panic on a shape the engine can reach.
///
/// **Deduplicated by state key first**, through the one function that decides
/// what *the same model* means (`ein_infer::canon::distinct_by_state`). Two
/// identical models are a pair no variable separates, so the hitting set would
/// be unsatisfiable and the search would spend its whole budget proving it — a
/// wrong answer arrived at expensively, which is the worst of both.
///
/// `key_table` is public, so this cannot assume its caller deduplicated:
/// `answer.rs` hands it `Verdict::models`, which is distinct by construction,
/// and an embedder may hand it anything.
fn variables(terms: &Terms, models: &[&Kb]) -> Option<Vars> {
    let models: Vec<&Kb> = ein_infer::canon::distinct_by_state(models, |m| *m)
        .into_iter()
        .copied()
        .collect();
    let models = &models[..];
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

    // **Which pair is hardest does not depend on the path**, so the candidate
    // list per pair is built once and the pairs are sorted by its length; a
    // node then takes the first *uncovered* pair in that order. That is the
    // same choice a per-node minimum makes — ties included, since the sort is
    // stable and leaves equal-length pairs in bit order — and it takes a
    // node's cost from `O(pairs x variables)` to `O(pairs)`, which is what
    // makes a budget counted in **nodes** mean anything in seconds.
    let mut by_pair: Vec<(usize, Vec<usize>)> = (0..bits)
        .map(|b| {
            (
                b,
                order.iter().copied().filter(|&v| sep[v].get(b)).collect(),
            )
        })
        .collect();
    by_pair.sort_by_key(|(_, c)| c.len());

    for d in 1..=sep.len() {
        if hits(&Mask::zero(bits), d, sep, full, &by_pair, budget)? {
            return Some(d);
        }
    }
    None
}

fn hits(
    covered: &Mask,
    depth: usize,
    sep: &[Mask],
    full: &Mask,
    by_pair: &[(usize, Vec<usize>)],
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
    let Some((_, best)) = by_pair.iter().find(|(b, _)| !covered.get(*b)) else {
        // Every pair covered while `covered != full`: unreachable so long as
        // `full` is exactly the pair set, and `false` is the safe reading.
        return Some(false);
    };
    for &v in best {
        if hits(&covered.or(&sep[v]), depth - 1, sep, full, by_pair, budget)? {
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
    /// because the enumeration is the fallback and always was.
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
    let Some(v) = variables(terms, models) else {
        return KeyOutcome::Unaffordable(
            "the models differ in no single-valued slot and no varying atom".to_string(),
        );
    };
    // `v.rows` is one row per **distinct** model, which is what `k` means
    // everywhere else in the verdict.
    let k = v.rows.len();
    let n = v.vary.len();
    let (sep, full, bits) = separating(&v);
    let mut budget = Budget(KEY_NODE_BUDGET);
    let Some(size) = min_key_size(&sep, &full, bits, &mut budget) else {
        return KeyOutcome::Unaffordable(format!(
            "no determining set of the {n} varying slots was found within \
             {KEY_NODE_BUDGET} search nodes"
        ));
    };
    let combos = binomial(n, size);
    if combos > KEY_TABLE_BUDGET {
        return KeyOutcome::Unaffordable(format!(
            "the smallest determining set is {size} of {n} varying slots, and \
             C({n}, {size}) = {} candidates is over the budget",
            group(combos)
        ));
    }
    let Some(keys) = all_keys(&v, &sep, &full, bits, size, &mut budget) else {
        return KeyOutcome::Unaffordable(format!(
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
    out.extend(rows.iter().map(|r| pad_row(indent, r, &w)));
    out.push(String::new());

    // The guarantee, and it is the whole reason this is a representation and
    // not merely a smaller print: re-saturating with a row is what makes the
    // table exact rather than a summary of one.
    let rest = match n - size {
        0 => String::new(),
        1 => "; the other varying slot follows".to_string(),
        m => format!("; the other {m} varying slots follow"),
    };
    let mut prose = format!(
        "{} rows, one per model{}. Add a row's facts to the program and it re-solves to \
         that model{}{rest}.",
        rows.len(),
        if exhausted { "" } else { " found" },
        if exhausted { " alone" } else { "" }
    );
    if !exhausted {
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
    out.extend(wrap(&prose, &format!("{indent}  ")));
    KeyOutcome::Table(out)
}

/// Greedy wrap at 78 columns, indent included — the width the record prices
/// (b) at, and the reason it beat the envelope on the *read* column: a form
/// that does not fit a page is a form nobody reads.
pub(crate) fn wrap(text: &str, indent: &str) -> Vec<String> {
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
