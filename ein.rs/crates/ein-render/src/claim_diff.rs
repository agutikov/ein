//! A claim's models against what its run found: `ein test --diff`
//! ([`05-cli.md` § 2.3](../../../../docs/ein/user/04-reference/05-cli.md#23-ein-test)).
//!
//! [`ein_infer::claim::Compared`] is what a claim's check compares, the
//! models it lists and the states, owing nodes or projection classes the run
//! found, each read as the check reads it. [`diff`] pairs them: **a match**
//! first, a model and a found item the check pairs, one to one; then
//! **near**, a model a few facts off a found item no model matched, read as
//! the facts that differ rather than as a missing model beside an extra one;
//! and what is left is **missing**, a model nothing found is, and **extra**,
//! a found item no model is. [`lines`] is the block `ein test` prints after
//! the claim's line, and [`json`] the report's `diff`.
//!
//! **It explains and never decides.** The verdict is the check's, and a diff
//! is taken only of a claim the check decided: a held claim's diff is all
//! matches, since the check pairs every model with a found item; a FAILED
//! one's says which model and which found item differ, and in which facts,
//! wherever the models are why it failed.

use std::collections::{BTreeMap, BTreeSet};

use ein_infer::claim::{Against, Compared};

use crate::dump::Json;

/// **The memory bound on the near pairing's table, in bytes**: 16 MiB.
///
/// [`diff`] pairs the models it did not match **pairwise**, and a table over
/// pairs is quadratic in them: 1 000 models against 1 000 states, each one
/// fact off every other, hold 31 MiB of pairs. The table is priced from the
/// counts before anything is allocated, and past this bound nothing is
/// paired near and no nearest is sought, which is an answer and not an
/// error: every unmatched model is then a missing one and every unmatched
/// found item an extra one, and the block says why ([`SetDiff::unpaired`]).
pub const PAIR_TABLE_BOUND: u64 = 16 << 20;

/// How much of a diff to print: `ein test --diff stats|full`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    /// The head: what was compared, how many match, are near and how many
    /// facts off, are missing and are extra.
    Stats,
    /// The head, then each model against the found item it matches or is
    /// nearest, and each extra found item.
    Full,
}

impl Level {
    /// `stats` or `full`, `--diff`'s two values.
    pub fn parse(word: &str) -> Option<Level> {
        match word {
            "stats" => Some(Level::Stats),
            "full" => Some(Level::Full),
            _ => None,
        }
    }
}

/// `491 MiB`, rounded up, because a bound is passed by any part of a byte.
fn mib(bytes: u128) -> String {
    format!("{} MiB", group(bytes.div_ceil(1 << 20)))
}

/// Greedy wrap at 78 columns, the first line at `first`, the others at
/// `rest`: a line that does not fit a page is one nobody reads.
fn wrap(text: &str, first: &str, rest: &str) -> Vec<String> {
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
    let mut indent = first;
    let mut line = indent.to_string();
    for word in &words {
        if line.chars().count() > indent.chars().count()
            && line.chars().count() + 1 + word.chars().count() > 78
        {
            out.push(line);
            indent = rest;
            line = indent.to_string();
        } else if line.chars().count() > indent.chars().count() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.trim().is_empty() {
        out.push(line);
    }
    out
}

/// `118 030 185` — digit groups of three, so a nine-digit count reads as one
/// number ([`wrap`] keeps a group with its number).
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

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{} {}", group(n as u128), if n == 1 { one } else { many })
}

// ── a claim's models and what was found, paired ───────────────────────

/// One listed model against one found item: the model's facts the item
/// lacks, and the item's facts beyond the model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Off {
    /// The model, an index into [`Compared::models`].
    pub model: usize,
    /// The found item, an index into [`Compared::found`].
    pub found: usize,
    /// In the model and not in the found item, sorted: printed `-`.
    pub lacks: Vec<String>,
    /// In the found item and not in the model, sorted, a listed negative
    /// aside: printed `+`.
    pub has: Vec<String>,
}

impl Off {
    /// **How many facts off** the two are, as the check reads them: the
    /// facts to add, take away or replace to make one the other, a fact
    /// replaced counted once — `(p b)` where the model lists `(p a)` is one
    /// fact off, as a fact lacking or one too many is.
    pub fn facts(&self) -> usize {
        self.lacks.len().max(self.has.len())
    }
}

/// How a claim's models and what the run found differ.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SetDiff {
    /// `(model, found)`, each a pair the check pairs: equal as it reads them.
    pub matched: Vec<(usize, usize)>,
    /// A model and a found item neither of which matched, a few facts apart,
    /// in the models' order.
    pub near: Vec<Off>,
    /// A model nothing found is, with the found item nearest it: `None`
    /// where nothing was found, or the pairing was declined.
    pub missing: Vec<(usize, Option<Off>)>,
    /// A found item no model is, with the model nearest it: `None` where the
    /// claim lists none, or the pairing was declined.
    pub extra: Vec<(usize, Option<Off>)>,
    /// Why nothing was paired *near*, when the candidates would have passed
    /// [`PAIR_TABLE_BOUND`]. Every unmatched model is then a missing one and
    /// every unmatched found item an extra one, which is what each is.
    pub unpaired: Option<String>,
}

impl SetDiff {
    /// Every model matches a found item, and every found item a model.
    pub fn agrees(&self) -> bool {
        self.near.is_empty() && self.missing.is_empty() && self.extra.is_empty()
    }
}

/// Pair `c`'s models with what was found ([`diff_sets`]).
pub fn diff(c: &Compared) -> SetDiff {
    diff_sets(&c.found, &c.models, &c.presence)
}

/// Pair `models` with `found`, each a list of facts; a fact of `presence`
/// counts as a difference only where a model lists it and the found item
/// lacks it, which is how a claim's check reads a listed negative.
///
/// **Matches first**, a maximum matching of the pairs the check pairs, so
/// that every pair a held claim's check found is found here too: a model
/// listed twice against one found item is one match and one missing. **Then
/// nearest** among what is left, by how many facts off ([`Off::facts`]) and
/// then how many differ, greedily and smallest first, ties to the found
/// items' order and then the models' — paired only while the two are at most
/// half the larger of them off (and at least one fact), so two unrelated ones
/// stay a *missing* and an *extra* rather than becoming a misleading pair.
/// Each missing model is then given the found item nearest it, and each extra
/// found item the model nearest it, ties the same way.
///
/// **The near pairing is priced before it is taken.** It holds every
/// unmatched (found, model) pair within the distance bound, 32 bytes each,
/// which is quadratic in them. Past [`PAIR_TABLE_BOUND`] it pairs nothing,
/// seeks no nearest, and says why in [`SetDiff::unpaired`].
pub fn diff_sets(found: &[Vec<String>], models: &[Vec<String>], presence: &[String]) -> SetDiff {
    let p: BTreeSet<&str> = presence.iter().map(String::as_str).collect();
    let f: Vec<BTreeSet<&str>> = found
        .iter()
        .map(|x| x.iter().map(String::as_str).collect())
        .collect();
    let m: Vec<BTreeSet<&str>> = models
        .iter()
        .map(|x| x.iter().map(String::as_str).collect())
        .collect();
    let off = |j: usize, i: usize| -> Off {
        Off {
            model: j,
            found: i,
            lacks: m[j].difference(&f[i]).map(|s| s.to_string()).collect(),
            has: f[i]
                .difference(&m[j])
                .filter(|s| !p.contains(*s))
                .map(|s| s.to_string())
                .collect(),
        }
    };
    // Facts off, then the facts that differ, for an order that prefers a
    // fact replaced to one lacking beside one too many.
    let distance = |j: usize, i: usize| -> (usize, usize) {
        let lacks = m[j].difference(&f[i]).count();
        let has = f[i].difference(&m[j]).filter(|s| !p.contains(*s)).count();
        (lacks.max(has), lacks + has)
    };

    // The pairs the check pairs: a model and a found item equal outside the
    // listed negatives, the model's negatives held by the item. Found items
    // are keyed by what lies outside the negatives, so a model's candidates
    // are one lookup.
    let outside = |s: &BTreeSet<&str>| -> Vec<String> {
        s.iter()
            .filter(|x| !p.contains(*x))
            .map(|x| x.to_string())
            .collect()
    };
    let mut by_key: BTreeMap<Vec<String>, Vec<usize>> = BTreeMap::new();
    for (i, fi) in f.iter().enumerate() {
        by_key.entry(outside(fi)).or_default().push(i);
    }
    let adj: Vec<Vec<usize>> = m
        .iter()
        .map(|mj| {
            by_key.get(&outside(mj)).map_or_else(Vec::new, |is| {
                is.iter()
                    .copied()
                    .filter(|&i| {
                        mj.iter()
                            .filter(|x| p.contains(*x))
                            .all(|x| f[i].contains(x))
                    })
                    .collect()
            })
        })
        .collect();
    // A maximum matching by augmenting paths, each model in its order.
    let mut owner: Vec<Option<usize>> = vec![None; f.len()];
    let mut stamp: Vec<usize> = vec![0; f.len()];
    fn augment(
        j: usize,
        round: usize,
        adj: &[Vec<usize>],
        stamp: &mut [usize],
        owner: &mut [Option<usize>],
    ) -> bool {
        for &i in &adj[j] {
            if stamp[i] == round {
                continue;
            }
            stamp[i] = round;
            if owner[i].is_none_or(|k| augment(k, round, adj, stamp, owner)) {
                owner[i] = Some(j);
                return true;
            }
        }
        false
    }
    for j in 0..m.len() {
        augment(j, j + 1, &adj, &mut stamp, &mut owner);
    }
    let mut out = SetDiff::default();
    let mut f_free: Vec<bool> = vec![true; f.len()];
    let mut m_free: Vec<bool> = vec![true; m.len()];
    for (i, o) in owner.iter().enumerate() {
        if let Some(j) = *o {
            out.matched.push((j, i));
            f_free[i] = false;
            m_free[j] = false;
        }
    }
    out.matched.sort_unstable();

    let (fnum, mnum) = (
        f_free.iter().filter(|x| **x).count(),
        m_free.iter().filter(|x| **x).count(),
    );
    let candidates = fnum as u128 * mnum as u128;
    let width = std::mem::size_of::<((usize, usize), usize, usize)>();
    let bytes = candidates * width as u128;
    let declined = bytes > PAIR_TABLE_BOUND as u128;
    let mut pairs: Vec<((usize, usize), usize, usize)> = Vec::new();
    if declined {
        out.unpaired = Some(format!(
            "no model was paired as near: {} and {} left unmatched are {} candidate pairs, {} \
             at {width} bytes each, past the bound of {} on a pair table, and no nearest was \
             sought",
            plural(mnum, "model", "models"),
            plural(fnum, "found item", "found items"),
            group(candidates),
            mib(bytes),
            mib(PAIR_TABLE_BOUND as u128)
        ));
    } else {
        // Reserved whole, so what it holds is what was priced, and sorted
        // unstably, which allocates nothing and is the same order: no two
        // candidates share an `(i, j)`.
        pairs.reserve_exact(candidates as usize);
        for i in (0..f.len()).filter(|&i| f_free[i]) {
            for j in (0..m.len()).filter(|&j| m_free[j]) {
                let d = distance(j, i);
                let bound = (f[i].len().max(m[j].len()) / 2).max(1);
                if d.0 <= bound {
                    pairs.push((d, i, j));
                }
            }
        }
        pairs.sort_unstable();
    }
    for (_, i, j) in pairs {
        if f_free[i] && m_free[j] {
            f_free[i] = false;
            m_free[j] = false;
            out.near.push(off(j, i));
        }
    }
    out.near.sort_by_key(|n| n.model);
    // The nearest of all of the other side, ties to the lower index.
    let nearest_found = |j: usize| (0..f.len()).min_by_key(|&i| (distance(j, i), i));
    let nearest_model = |i: usize| (0..m.len()).min_by_key(|&j| (distance(j, i), j));
    out.missing = (0..m.len())
        .filter(|&j| m_free[j])
        .map(|j| {
            (
                j,
                (!declined)
                    .then(|| nearest_found(j))
                    .flatten()
                    .map(|i| off(j, i)),
            )
        })
        .collect();
    out.extra = (0..f.len())
        .filter(|&i| f_free[i])
        .map(|i| {
            (
                i,
                (!declined)
                    .then(|| nearest_model(i))
                    .flatten()
                    .map(|j| off(j, i)),
            )
        })
        .collect();
    out
}

// ── printed, and written ──────────────────────────────────────────────

/// What a found item is called: `state`, `owing node`, `class`.
fn noun(against: Against) -> (&'static str, &'static str) {
    match against {
        Against::States => ("state", "states"),
        Against::Owing => ("owing node", "owing nodes"),
        Against::Classes(_) => ("class", "classes"),
    }
}

/// `1 near (1 fact off)`, `3 near (2 at 1 fact off, 1 at 3 facts off)`.
fn near_words(near: &[Off]) -> String {
    let mut by: BTreeMap<usize, usize> = BTreeMap::new();
    for n in near {
        *by.entry(n.facts()).or_default() += 1;
    }
    let off = |d: usize| plural(d, "fact off", "facts off");
    match by.len() {
        0 => "0 near".to_string(),
        1 => {
            let d = *by.keys().next().expect("one");
            format!("{} near ({})", near.len(), off(d))
        }
        _ => format!(
            "{} near ({})",
            near.len(),
            by.iter()
                .map(|(d, n)| format!("{n} at {}", off(*d)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// The part of R a query's claim is about, as a noun: `R`, `the maximal
/// line`.
fn part(over: ein_ir::expect::Over) -> &'static str {
    match over {
        ein_ir::expect::Over::All => "R",
        ein_ir::expect::Over::Minimal => "the minimal line",
        ein_ir::expect::Over::Maximal => "the maximal line",
    }
}

/// The head: what was compared with what, and over what.
fn head(c: &Compared) -> String {
    let (one, many) = noun(c.against);
    let k = c.found.len();
    let what = match c.against {
        Against::States => format!("R's {}", plural(k, one, many)),
        Against::Owing => format!("the owing frontier's {}", plural(k, "node", "nodes")),
        Against::Classes(over) if c.exact => {
            format!("the {} of {}", plural(k, one, many), over.words())
        }
        Against::Classes(over) => format!(
            "the {} of the states found, a part of {}",
            plural(k, one, many),
            part(over)
        ),
    };
    let over = match c.against {
        Against::Classes(_) => String::new(),
        _ => {
            let rels = if c.over.is_empty() {
                "no relation".to_string()
            } else {
                c.over.join(", ")
            };
            let negatives = if c.presence.is_empty() {
                String::new()
            } else {
                format!(
                    " and the presence of {}",
                    plural(c.presence.len(), "listed negative", "listed negatives")
                )
            };
            format!(", compared over {rels}{negatives}")
        }
    };
    format!(
        "diff: {} against {what}{over}",
        plural(c.models.len(), "model", "models"),
    )
}

/// The counts: how many match, are near and how many facts off, are missing
/// and are extra.
fn counts(d: &SetDiff) -> String {
    format!(
        "{} match, {}, {} missing, {} extra",
        group(d.matched.len() as u128),
        near_words(&d.near),
        group(d.missing.len() as u128),
        group(d.extra.len() as u128),
    )
}

/// The diff as `ein test` prints it under a claim, every line at `indent` or
/// deeper: what was compared, and the counts; then, at [`Level::Full`], each
/// model in the claim's
/// order, `=` the found item it matches, `~` the one it is near, `-` missing
/// and the found item nearest it, the facts that differ under each pair
/// (`-` the model's the found item lacks, `+` the found item's beyond the
/// model); and each extra found item, `+`, with the model nearest it.
pub fn lines(c: &Compared, d: &SetDiff, level: Level, indent: &str) -> Vec<String> {
    let deeper = format!("{indent}  ");
    let mut out = wrap(&head(c), indent, &format!("{deeper}  "));
    out.push(format!("{deeper}{}", counts(d)));
    if let Some(why) = &d.unpaired {
        out.extend(wrap(&format!("{why}."), &deeper, &deeper));
    }
    if level == Level::Stats {
        return out;
    }
    let (one, _) = noun(c.against);
    let found = |i: usize| format!("{one} {}", i + 1);
    let facts = |o: &Off, out: &mut Vec<String>| {
        out.extend(o.lacks.iter().map(|f| format!("{deeper}    - {f}")));
        out.extend(o.has.iter().map(|f| format!("{deeper}    + {f}")));
    };
    let off = |o: &Off| plural(o.facts(), "fact off", "facts off");
    for j in 0..c.models.len() {
        if let Some(&(_, i)) = d.matched.iter().find(|(m, _)| *m == j) {
            out.push(format!("{deeper}= model {} is {}", j + 1, found(i)));
        } else if let Some(n) = d.near.iter().find(|n| n.model == j) {
            out.push(format!(
                "{deeper}~ model {} is {}, {}",
                j + 1,
                found(n.found),
                off(n)
            ));
            facts(n, &mut out);
        } else if let Some((_, nearest)) = d.missing.iter().find(|(m, _)| *m == j) {
            match nearest {
                Some(n) => {
                    out.push(format!(
                        "{deeper}- model {}: no {one} is it; the nearest is {}, {}",
                        j + 1,
                        found(n.found),
                        off(n)
                    ));
                    facts(n, &mut out);
                }
                None if c.found.is_empty() => out.push(format!(
                    "{deeper}- model {}: no {one} is it, and none was found",
                    j + 1
                )),
                None => out.push(format!("{deeper}- model {}: no {one} is it", j + 1)),
            }
        }
    }
    for (i, nearest) in &d.extra {
        out.push(match nearest {
            Some(n) => format!(
                "{deeper}+ {}: no model is it; the nearest is model {}, {}",
                found(*i),
                n.model + 1,
                off(n)
            ),
            None if c.models.is_empty() => {
                format!(
                    "{deeper}+ {}: no model is it, and the claim lists none",
                    found(*i)
                )
            }
            None => format!("{deeper}+ {}: no model is it", found(*i)),
        });
    }
    out
}

/// The diff as `--json-report` writes it under a claim's row: what was
/// compared, the counts, and at [`Level::Full`] the items, `null` at
/// [`Level::Stats`]. One shape on every arm: an item carries every key,
/// `null` where it has no such thing; indices are 1-based, as printed.
pub fn json(c: &Compared, d: &SetDiff, level: Level) -> Json {
    let strs = |v: &[String]| Json::Array(v.iter().map(Json::str).collect());
    let mut by: BTreeMap<usize, usize> = BTreeMap::new();
    for n in &d.near {
        *by.entry(n.facts()).or_default() += 1;
    }
    let item = |kind: &str, model: Option<usize>, found: Option<usize>, o: Option<&Off>| {
        let ix = |x: Option<usize>| x.map_or(Json::Null, |x| Json::int(x as i64 + 1));
        Json::obj(vec![
            ("kind", Json::str(kind)),
            ("model", ix(model)),
            ("found", ix(found)),
            ("off", o.map_or(Json::Null, |o| Json::int(o.facts() as i64))),
            ("lacks", o.map_or(Json::Null, |o| strs(&o.lacks))),
            ("has", o.map_or(Json::Null, |o| strs(&o.has))),
        ])
    };
    let items = match level {
        Level::Stats => Json::Null,
        Level::Full => {
            let mut v: Vec<Json> = Vec::new();
            for j in 0..c.models.len() {
                if let Some(&(_, i)) = d.matched.iter().find(|(m, _)| *m == j) {
                    let exact = Off {
                        model: j,
                        found: i,
                        lacks: Vec::new(),
                        has: Vec::new(),
                    };
                    v.push(item("match", Some(j), Some(i), Some(&exact)));
                } else if let Some(n) = d.near.iter().find(|n| n.model == j) {
                    v.push(item("near", Some(j), Some(n.found), Some(n)));
                } else if let Some((_, n)) = d.missing.iter().find(|(m, _)| *m == j) {
                    v.push(item(
                        "missing",
                        Some(j),
                        n.as_ref().map(|n| n.found),
                        n.as_ref(),
                    ));
                }
            }
            for (i, n) in &d.extra {
                v.push(item(
                    "extra",
                    n.as_ref().map(|n| n.model),
                    Some(*i),
                    n.as_ref(),
                ));
            }
            Json::Array(v)
        }
    };
    Json::obj(vec![
        (
            "against",
            Json::str(match c.against {
                Against::States => "states",
                Against::Owing => "owing",
                Against::Classes(_) => "classes",
            }),
        ),
        ("exact", Json::Bool(c.exact)),
        ("over", strs(&c.over)),
        ("presence", strs(&c.presence)),
        ("models", Json::int(c.models.len() as i64)),
        ("found", Json::int(c.found.len() as i64)),
        ("match", Json::int(d.matched.len() as i64)),
        ("near", Json::int(d.near.len() as i64)),
        ("missing", Json::int(d.missing.len() as i64)),
        ("extra", Json::int(d.extra.len() as i64)),
        (
            "near_off",
            Json::Array(
                by.iter()
                    .map(|(off, n)| {
                        Json::obj(vec![
                            ("off", Json::int(*off as i64)),
                            ("near", Json::int(*n as i64)),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "unpaired",
            d.unpaired.as_deref().map_or(Json::Null, Json::str),
        ),
        ("items", items),
    ])
}

#[cfg(test)]
mod diff_tests {
    use super::*;

    fn s(v: &[&[&str]]) -> Vec<Vec<String>> {
        v.iter()
            .map(|m| m.iter().map(|f| f.to_string()).collect())
            .collect()
    }

    fn none() -> Vec<String> {
        Vec::new()
    }

    #[test]
    fn a_missing_model_is_named_with_its_nearest() {
        let d = diff_sets(
            &s(&[&["(p B)", "(p C)"]]),
            &s(&[&["(p A)"], &["(p B)", "(p C)"]]),
            &none(),
        );
        assert_eq!(d.matched, [(1, 0)]);
        assert!(d.extra.is_empty() && d.near.is_empty());
        let (j, nearest) = &d.missing[0];
        assert_eq!(*j, 0);
        let n = nearest.as_ref().expect("a nearest");
        assert_eq!((n.found, n.facts()), (0, 2));
    }

    #[test]
    fn an_extra_found_item_is_named() {
        let found = s(&[
            &["(s A 1)", "(s B 2)"],
            &["(s A 2)", "(s B 1)"],
            &["(s A 1)", "(s A 2)", "(s B 1)"],
        ]);
        let models = s(&[&["(s A 1)", "(s B 2)"], &["(s A 2)", "(s B 1)"]]);
        let d = diff_sets(&found, &models, &none());
        assert_eq!(d.matched.len(), 2);
        assert_eq!(d.extra.len(), 1);
        assert_eq!(d.extra[0].0, 2);
        assert!(d.missing.is_empty() && d.near.is_empty());
    }

    #[test]
    fn a_model_one_fact_off_reads_as_one_wrong_fact() {
        let d = diff_sets(&s(&[&["(q A)"]]), &s(&[&["(p A)", "(q A)"]]), &none());
        assert_eq!(
            d.near,
            [Off {
                model: 0,
                found: 0,
                lacks: vec!["(p A)".to_string()],
                has: vec![],
            }]
        );
        assert!(d.missing.is_empty() && d.extra.is_empty());
    }

    /// A fact replaced is one fact off, as one lacking is.
    #[test]
    fn a_fact_replaced_is_one_fact_off() {
        let d = diff_sets(
            &s(&[&["(in A 0)", "(in B 1)", "(in C 0)"]]),
            &s(&[&["(in A 0)", "(in B 1)", "(in C 1)"]]),
            &none(),
        );
        assert_eq!(d.near.len(), 1, "{d:?}");
        assert_eq!(d.near[0].facts(), 1);
        assert_eq!(
            (d.near[0].lacks.as_slice(), d.near[0].has.as_slice()),
            (&["(in C 1)".to_string()][..], &["(in C 0)".to_string()][..])
        );
    }

    #[test]
    fn unrelated_models_are_not_paired_near() {
        let d = diff_sets(
            &s(&[&["(p a)", "(p b)"]]),
            &s(&[&["(q a)", "(q b)"]]),
            &none(),
        );
        assert!(d.near.is_empty());
        assert_eq!((d.missing.len(), d.extra.len()), (1, 1));
        // Each is still given the other as its nearest, two facts off.
        assert_eq!(d.missing[0].1.as_ref().map(Off::facts), Some(2));
    }

    #[test]
    fn the_empty_model_is_a_model() {
        let models = s(&[&[]]);
        assert!(diff_sets(&s(&[&[]]), &models, &none()).agrees());
        let d = diff_sets(&[], &models, &none());
        assert_eq!(d.missing, [(0, None)]);
    }

    /// A listed negative is checked for presence, as the claim's check reads
    /// it: a found item that holds one no model lists is no different for it,
    /// and a model that lists one the found item lacks is.
    #[test]
    fn a_listed_negative_counts_only_where_it_is_lacking() {
        let presence = vec!["(not (p b))".to_string(), "(not (p c))".to_string()];
        let found = s(&[&["(not (p b))", "(not (p c))", "(p a)"]]);
        let held = diff_sets(&found, &s(&[&["(not (p b))", "(p a)"]]), &presence);
        assert!(held.agrees(), "{held:?}");
        let lacked = diff_sets(
            &s(&[&["(p a)"]]),
            &s(&[&["(not (p b))", "(p a)"]]),
            &presence,
        );
        assert_eq!(lacked.near[0].lacks, ["(not (p b))"]);
        assert!(lacked.near[0].has.is_empty());
    }

    /// Matches are a maximum matching, not a greedy one: the first model could
    /// take the found item the second needs, and a held claim's check pairs
    /// both.
    #[test]
    fn matches_are_as_many_as_the_check_pairs() {
        let presence = vec!["(not (q a))".to_string()];
        // Found 0 holds the negative, found 1 does not; model 0 lists none of
        // it and fits both, model 1 lists it and fits found 0 alone.
        let found = s(&[&["(not (q a))", "(p a)"], &["(p a)"]]);
        let models = s(&[&["(p a)"], &["(not (q a))", "(p a)"]]);
        let d = diff_sets(&found, &models, &presence);
        assert!(d.agrees(), "{d:?}");
        assert_eq!(d.matched, [(0, 1), (1, 0)]);
    }

    /// The printed block: the head's counts and each kind of line.
    #[test]
    fn the_block_names_every_model_and_every_extra() {
        let c = Compared {
            against: Against::States,
            exact: true,
            over: vec!["p".to_string()],
            presence: Vec::new(),
            models: s(&[
                &["(p A)"],
                &["(p B)", "(p C)"],
                &["(p D)", "(p E)", "(p F)"],
            ]),
            found: s(&[&["(p A)"], &["(p B)"], &["(p X)"]]),
        };
        let d = diff(&c);
        let text = lines(&c, &d, Level::Full, "    ").join("\n");
        assert!(
            text.starts_with(
                "    diff: 3 models against R's 3 states, compared over p\n      1 match, 1 \
                 near (1 fact off), 1 missing, 1 extra\n"
            ),
            "{text}"
        );
        for want in [
            "      = model 1 is state 1",
            "      ~ model 2 is state 2, 1 fact off\n          - (p C)",
            // Three facts off every state: the first, by the tie rule.
            "      - model 3: no state is it; the nearest is state 1, 3 facts off\n          - (p D)",
            "      + state 3: no model is it; the nearest is model 1, 1 fact off",
        ] {
            assert!(text.contains(want), "{want:?} not in\n{text}");
        }
        assert_eq!(lines(&c, &d, Level::Stats, "    ").len(), 2);
    }
}
