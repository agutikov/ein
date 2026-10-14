//! A model as **the grid a person would draw** — M1e2
//! [S1e2.1.4](../../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.4_a_model_you_can_read.md).
//!
//! `zebra2`'s unique model is 444 facts, and nobody checks 444 facts. A person
//! checks it as a 5 × 5 table: the houses down the side, an attribute per
//! column. This renders that table **where the program licenses it**, and says
//! so in one line where it does not, because a grid drawn from a guess about a
//! program's shape would be a picture of the guess.
//!
//! # The two licences
//!
//! A grid needs an **anchor** — the type whose objects are the rows — and
//! columns that each give a row at most one value. The program states both in
//! one of two ways, and each is read off the model's own facts, never the
//! source text, the rule `models.rs` already follows for its decision
//! variables:
//!
//! 1. **`(slot-partition R isa sub Root Anchor)`** — `std.slots`' statement
//!    that `R`'s classes are the anchor's objects: *one instance of every
//!    direct subtype of `Root` per class, and the `Anchor` member names it*
//!    (`examples/zebra.ein`). Rows are the `isa`-instances of `Anchor`;
//!    columns are the other `sub`-children of `Root`, in declaration order; a
//!    cell is every instance of the column's type that `R` relates to the row,
//!    in either argument order.
//! 2. **Single-valued relations of arity 2**, read by direction. A
//!    `functional` `(relation R A B)` gives each `A` at most one `B`, so it is a
//!    column of an `A`-row grid; an `injective` one gives each `B` at most one
//!    `A`, so it is a column of a `B`-row grid; `bijective` is both. The anchor
//!    is the type **the most such columns give a single value to** — `House`
//!    in `examples/zebra2.ein`, where five bijections run *onto* it — and a tie
//!    goes to the type declared first, which puts a relation's domain before
//!    its codomain (`branching/05`'s `color-of` draws houses with their
//!    colour, not colours with their house). Rows are every row-side value the
//!    columns hold in the model **and every `(is-a x Anchor)`** — membership
//!    read the way [`crate::kb_dot`] reads it, from the puzzle's own `is-a`
//!    facts — so an object with no value yet still has its row, and an open
//!    state's debt shows as the empty cell it is. A cell is every value on the
//!    other side.
//!
//! **`T` is never a row type.** It is the universal annotation, every object
//! is one, and a program that declares `(functional is-a)` — nine of the
//! `branching/` demos do — would otherwise draw the type hierarchy as its
//! answer. That was the first census's one wrong pick (S1e2.1.4 T1).
//!
//! The partition wins where a program states both, because it is the more
//! specific statement: it names the anchor rather than leaving it to a count.
//!
//! # A cell with two values is printed as two values
//!
//! A bijection gives every cell one value, and a model that breaks that — or a
//! licence that was wrong about the program — shows as `Red / Green`, never as
//! a choice between them. The renderer resolves nothing.

use ein_core::{FactId, Kb, Symbol, Terms, Value};

/// What licenses a grid, read off one KB.
#[derive(Clone, Debug)]
pub enum Licence {
    /// `(slot-partition rel isa sub root anchor)`.
    Partition {
        rel: Symbol,
        isa: Symbol,
        sub: Symbol,
        root: Value,
        anchor: Value,
    },
    /// Single-valued arity-2 relations giving each `anchor` object one value,
    /// in declaration order; `true` when the row is the relation's first
    /// argument (`functional`), `false` when it is the second (`injective`).
    Functional {
        anchor: Value,
        cols: Vec<(Symbol, bool)>,
    },
}

impl Licence {
    /// One line saying what licensed the grid — printed above it.
    pub fn describe(&self, terms: &Terms) -> String {
        match self {
            Licence::Partition {
                rel,
                isa,
                sub,
                root,
                anchor,
            } => format!(
                "rows: {anchor} — licensed by (slot-partition {} {} {} {} {})",
                terms.sym(*rel),
                terms.sym(*isa),
                terms.sym(*sub),
                terms.display(*root),
                terms.display(*anchor),
                anchor = terms.display(*anchor),
            ),
            Licence::Functional { anchor, cols } => format!(
                "rows: {} — given one value each by {} relation{} ({})",
                terms.display(*anchor),
                cols.len(),
                if cols.len() == 1 { "" } else { "s" },
                cols.iter()
                    .map(|(r, fwd)| format!(
                        "{} {}",
                        if *fwd { "functional" } else { "injective" },
                        terms.sym(*r)
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}

/// Is `f` over the relation named `name`?
fn head_is(terms: &Terms, f: FactId, name: &str) -> bool {
    terms.sym(terms.fact(f).0) == name
}

/// The licence a KB carries, or the one-line reason it carries none.
pub fn licence(terms: &Terms, kb: &Kb) -> Result<Licence, String> {
    // 1 — an explicit partition.
    for f in kb.facts() {
        let (rel, args) = terms.fact(f);
        if terms.sym(rel) == "slot-partition" && args.len() == 5 {
            let (Some(r), Some(isa), Some(sub)) =
                (args[0].as_sym(), args[1].as_sym(), args[2].as_sym())
            else {
                continue;
            };
            return Ok(Licence::Partition {
                rel: r,
                isa,
                sub,
                root: args[3],
                anchor: args[4],
            });
        }
    }
    // 2 — single-valued relations, each read in the direction it is
    // single-valued in.
    let (mut fwd, mut inv): (Vec<Symbol>, Vec<Symbol>) = (Vec::new(), Vec::new());
    for f in kb.facts() {
        let (rel, args) = terms.fact(f);
        let Some(r) = (args.len() == 1).then(|| args[0].as_sym()).flatten() else {
            continue;
        };
        match terms.sym(rel) {
            "functional" => fwd.push(r),
            "injective" => inv.push(r),
            "bijective" => {
                fwd.push(r);
                inv.push(r);
            }
            _ => {}
        }
    }
    // `(relation R A B)` in declaration order — the KB's own fact order — so a
    // type's first appearance, domain before codomain, is the tie-break.
    let top = terms.syms.get("T").map(Value::sym);
    let mut by_anchor: Vec<(Value, Vec<(Symbol, bool)>)> = Vec::new();
    let mut add = |anchor: Value, col: (Symbol, bool)| {
        if Some(anchor) == top {
            return;
        }
        match by_anchor.iter_mut().find(|(a, _)| *a == anchor) {
            Some((_, cols)) if !cols.contains(&col) => cols.push(col),
            Some(_) => {}
            None => by_anchor.push((anchor, vec![col])),
        }
    };
    for f in kb.facts() {
        let (rel, args) = terms.fact(f);
        if terms.sym(rel) != "relation" || args.len() != 3 {
            continue;
        }
        let Some(r) = args[0].as_sym() else { continue };
        if fwd.contains(&r) {
            add(args[1], (r, true));
        }
        if inv.contains(&r) {
            add(args[2], (r, false));
        }
    }
    // The most columns; `max_by_key` keeps the *last* of a tie, so walk the
    // list reversed to keep the first declared.
    let best = by_anchor
        .iter()
        .rev()
        .max_by_key(|(_, cols)| cols.len())
        .cloned();
    match best {
        Some((anchor, cols)) => Ok(Licence::Functional { anchor, cols }),
        None => Err(
            "no (slot-partition …) and no functional, injective or bijective relation \
             of arity 2 over a type other than T, so nothing says what a row is"
                .to_string(),
        ),
    }
}

/// A grid: the anchor's objects down the side, one column per attribute.
pub struct Grid {
    pub licence: String,
    pub anchor: String,
    pub columns: Vec<String>,
    /// `(row label, one cell per column)`; a cell is every value it holds.
    pub rows: Vec<(String, Vec<Vec<String>>)>,
}

/// Natural order for labels: `House-2` before `House-10`.
fn natural(a: &str, b: &str) -> std::cmp::Ordering {
    fn chunks(s: &str) -> Vec<(bool, String)> {
        let mut out: Vec<(bool, String)> = Vec::new();
        for c in s.chars() {
            let d = c.is_ascii_digit();
            match out.last_mut() {
                Some((kind, buf)) if *kind == d => buf.push(c),
                _ => out.push((d, c.to_string())),
            }
        }
        out
    }
    let (x, y) = (chunks(a), chunks(b));
    for (p, q) in x.iter().zip(&y) {
        let o = match (p.0, q.0) {
            (true, true) => p.1.len().cmp(&q.1.len()).then_with(|| p.1.cmp(&q.1)),
            _ => p.1.cmp(&q.1),
        };
        if o.is_ne() {
            return o;
        }
    }
    x.len().cmp(&y.len())
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort_by(|a, b| natural(a, b));
    v.dedup();
    v
}

/// The grid of one KB under a licence.
pub fn grid(terms: &Terms, kb: &Kb, lic: &Licence) -> Grid {
    let positive = |f: FactId| !head_is(terms, f, "not");
    match lic {
        Licence::Partition {
            rel,
            isa,
            sub,
            root,
            anchor,
        } => {
            let instances_of = |t: Value| -> Vec<Value> {
                kb.facts_of(*isa)
                    .filter_map(|f| {
                        let (_, a) = terms.fact(f);
                        (a.len() == 2 && a[1] == t).then_some(a[0])
                    })
                    .collect()
            };
            let mut cols: Vec<Value> = Vec::new();
            for f in kb.facts_of(*sub) {
                let (_, a) = terms.fact(f);
                if a.len() == 2 && a[1] == *root && a[0] != *anchor && !cols.contains(&a[0]) {
                    cols.push(a[0]);
                }
            }
            let mut rows_v = instances_of(*anchor);
            rows_v.sort_by(|a, b| natural(&terms.display(*a), &terms.display(*b)));
            rows_v.dedup();
            let related = |x: Value, y: Value| -> bool {
                kb.facts_of(*rel).filter(|&f| positive(f)).any(|f| {
                    let (_, a) = terms.fact(f);
                    a.len() == 2 && ((a[0] == x && a[1] == y) || (a[0] == y && a[1] == x))
                })
            };
            let rows = rows_v
                .iter()
                .map(|&h| {
                    let cells = cols
                        .iter()
                        .map(|&t| {
                            sorted(
                                instances_of(t)
                                    .into_iter()
                                    .filter(|&v| related(v, h))
                                    .map(|v| terms.display(v))
                                    .collect(),
                            )
                        })
                        .collect();
                    (terms.display(h), cells)
                })
                .collect();
            Grid {
                licence: lic.describe(terms),
                anchor: terms.display(*anchor),
                columns: cols.iter().map(|c| terms.display(*c)).collect(),
                rows,
            }
        }
        Licence::Functional { anchor, cols } => {
            // `(row, value)` pairs of one column: the relation's arguments in
            // the order its direction reads them.
            let pairs = |(r, fwd): (Symbol, bool)| -> Vec<(Value, Value)> {
                kb.facts_of(r)
                    .filter_map(|f| {
                        let (_, a) = terms.fact(f);
                        (a.len() == 2).then(|| if fwd { (a[0], a[1]) } else { (a[1], a[0]) })
                    })
                    .collect()
            };
            let mut rows_v: Vec<Value> = cols
                .iter()
                .flat_map(|&c| pairs(c))
                .map(|(row, _)| row)
                .collect();
            if let Some(is_a) = terms.syms.get("is-a") {
                rows_v.extend(kb.facts_of(is_a).filter_map(|f| {
                    let (_, a) = terms.fact(f);
                    (a.len() == 2 && a[1] == *anchor).then_some(a[0])
                }));
            }
            rows_v.sort_by(|a, b| natural(&terms.display(*a), &terms.display(*b)));
            rows_v.dedup();
            let rows = rows_v
                .iter()
                .map(|&row| {
                    let cells = cols
                        .iter()
                        .map(|&c| {
                            sorted(
                                pairs(c)
                                    .into_iter()
                                    .filter(|&(x, _)| x == row)
                                    .map(|(_, v)| terms.display(v))
                                    .collect(),
                            )
                        })
                        .collect();
                    (terms.display(row), cells)
                })
                .collect();
            Grid {
                licence: lic.describe(terms),
                anchor: terms.display(*anchor),
                columns: cols
                    .iter()
                    .map(|(r, _)| terms.sym(*r).to_string())
                    .collect(),
                rows,
            }
        }
    }
}

/// The grid as a markdown table under its licence line — the text a terminal
/// gets and the text the review bundle embeds, one and the same.
pub fn grid_lines(g: &Grid, indent: &str) -> Vec<String> {
    let mut out = vec![format!("{indent}grid — {}", g.licence)];
    if g.rows.is_empty() {
        out.push(format!(
            "{indent}  (no {} in this state — every cell would be empty)",
            g.anchor
        ));
        return out;
    }
    let head = std::iter::once(g.anchor.clone())
        .chain(g.columns.iter().cloned())
        .collect::<Vec<_>>();
    out.push(format!("{indent}| {} |", head.join(" | ")));
    out.push(format!(
        "{indent}|{}|",
        head.iter().map(|_| "---").collect::<Vec<_>>().join("|")
    ));
    for (label, cells) in &g.rows {
        let cells: Vec<String> = cells
            .iter()
            .map(|c| {
                if c.is_empty() {
                    "—".to_string()
                } else {
                    c.join(" / ")
                }
            })
            .collect();
        out.push(format!("{indent}| {label} | {} |", cells.join(" | ")));
    }
    out
}

/// What `--models grid` prints for one state: its grid, or the one line that
/// says there is none and why.
pub fn grid_or_reason(terms: &Terms, kb: &Kb, indent: &str) -> Vec<String> {
    match licence(terms, kb) {
        Ok(lic) => grid_lines(&grid(terms, kb, &lic), indent),
        Err(why) => vec![format!("{indent}grid — none: {why}")],
    }
}

#[cfg(test)]
mod tests {
    use super::natural;

    #[test]
    fn labels_sort_the_way_a_person_counts() {
        let mut v = vec!["House-10", "House-2", "House-1", "Blue", "H5"];
        v.sort_by(|a, b| natural(a, b));
        assert_eq!(v, ["Blue", "H5", "House-1", "House-2", "House-10"]);
    }
}
