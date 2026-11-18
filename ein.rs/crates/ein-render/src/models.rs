//! Two model sets compared: `--diff-models FILE`, the states a run listed
//! against the models a file lists.
//!
//! The file is read by [`parse_expected`] into an [`Expected`], each state is
//! read as [`comparable_facts`], and [`diff_model_sets`] pairs them: equal
//! first, then *near*, a model a few facts off read as the facts that differ
//! rather than as a missing model beside a spurious one. [`diff_parts`]
//! is the block `ein solve` prints
//! ([`08-read-out.md` § 8](../../../../docs/ein/user/04-reference/08-read-out.md#8-the-forms-after-the-read-out)).
//!
//! **It is a rendering and never a replacement.** `region.states`,
//! `--json-summary`, `--events` and `:expect` are untouched by it: the flag
//! adds a block after the read-out and reaches nothing recorded.

use ein_core::{Kb, Terms};

/// **The memory bound on the near pairing's table, in bytes**: 16 MiB.
///
/// [`diff_model_sets`] pairs the models it did not match **pairwise**, and a
/// table over pairs is quadratic in them: `pets.ein`'s 11 339 states against
/// a file of them each one fact off hold hundreds of MiB of pairs. The table
/// is priced from the counts before anything is allocated, and past this
/// bound nothing is paired near, which is an answer and not an error: every
/// unmatched model is then a missing or a spurious one, and the block says
/// why ([`SetDiff::unpaired`]).
pub const PAIR_TABLE_BOUND: u64 = 16 << 20;

/// `491 MiB`, rounded up, because a bound is passed by any part of a byte.
fn mib(bytes: u128) -> String {
    format!("{} MiB", group(bytes.div_ceil(1 << 20)))
}

/// Greedy wrap at 78 columns, indent included: a line of the block that does
/// not fit a page is one nobody reads.
fn wrap(text: &str, indent: &str) -> Vec<String> {
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

// ── two model sets, compared ──────────────────────────────────────────

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
    /// [`PAIR_TABLE_BOUND`]. Every unmatched model is then a
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
/// **The near pairing is priced before it is taken.** It holds every
/// unmatched (got, want) pair within the distance bound, 24 bytes each, which
/// is quadratic in the models: `pets.ein`'s 11 339 states against a file of
/// them each one fact off hold hundreds of MiB of pairs. Past
/// [`PAIR_TABLE_BOUND`] it pairs nothing and says why in [`SetDiff::unpaired`].
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
/// (q A)               ; a fact may carry a comment, and span lines
/// model               a model with no fact over the closed relations
/// ```
///
/// `closes` is `corpus/regressions.toml`'s rule — naming a relation closes
/// it — and is what lets a file say *one model with no `q`*. Without it a
/// model is compared as its positive facts minus the program's given ones
/// ([the model](../../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#3-model)).
///
/// **A model's facts are Ein, compared by value.** The text between two `model`
/// lines is read by the generated parser as facts, so a fact may span lines
/// and carry a comment, and each is printed by the one printer
/// ([`Terms::fact_text`]), which prints the states' facts too: `(p  a)`,
/// `( p a )` and `(p a) ; the fact` are all `(p a)`. A line is never kept as
/// written and compared as text, which would let the same fact in another
/// spelling match nothing and `(p a` pass for a fact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Expected {
    pub closes: Option<Vec<String>>,
    pub models: Vec<Vec<String>>,
}

/// Why a `--diff-models` file could not be read.
#[derive(Debug)]
pub enum ExpectedError {
    /// A line that is not `closes …`, `model` or part of a fact, or a fact
    /// before the first `model`: `line n: …`.
    Format(String),
    /// A model's facts do not parse: every error recovery reaches, each
    /// segment's report in file order, positions the file's.
    Parse(Vec<ein_ir::ParseError>),
    /// A model lists a form that is no fact, or its facts do not load.
    NotFacts(String),
}

impl std::fmt::Display for ExpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedError::Format(m) | ExpectedError::NotFacts(m) => f.write_str(m),
            ExpectedError::Parse(reports) => {
                f.write_str("a model's facts do not parse")?;
                for r in reports {
                    write!(f, "\n{r}")?;
                }
                Ok(())
            }
        }
    }
}

/// Read a `--diff-models` file, `file` the name its parse errors carry.
pub fn parse_expected(text: &str, file: &str) -> Result<Expected, ExpectedError> {
    // Split at `\n` alone, as the parser counts lines, so that a model's
    // positions are the file's to the end of input.
    let lines: Vec<&str> = text.split('\n').collect();
    let mut out = Expected::default();
    // Each model's lines, by index: the lines between two `model` lines that
    // are not `closes …`, blank or a comment.
    let mut segments: Vec<Vec<usize>> = Vec::new();
    for (n, raw) in lines.iter().enumerate() {
        let code = raw.split(';').next().unwrap_or("").trim();
        let mut words = code.split_whitespace();
        match words.next() {
            Some("closes") => {
                if out.closes.is_some() {
                    return Err(ExpectedError::Format(format!(
                        "line {}: a second `closes` line",
                        n + 1
                    )));
                }
                out.closes = Some(words.map(str::to_string).collect());
            }
            Some("model") if words.next().is_none() => segments.push(Vec::new()),
            _ => match segments.last_mut() {
                Some(s) => s.push(n),
                None if code.is_empty() => {}
                None => {
                    return Err(ExpectedError::Format(format!(
                        "line {}: expected `closes …` or `model` before the first fact, got {:?}",
                        n + 1,
                        raw.trim()
                    )));
                }
            },
        }
    }
    // Each model's text, every other line blank, so that a parse error's
    // line and column are the file's; it ends with the model's last line, so
    // that a model's facts end where the next `model` line begins, and the
    // end of its input is there.
    let mut refused: Vec<ein_ir::ParseError> = Vec::new();
    for seg in &segments {
        let end = seg.last().map_or(0, |&i| i + 1);
        let mut masked: Vec<&str> = vec![""; end];
        for &i in seg {
            masked[i] = lines[i];
        }
        let masked = masked.join("\n");
        let mut ast = ein_ir::Ast::new();
        let forms = match ein_ir::parse(&mut ast, &masked, Some(file)) {
            Ok(forms) => forms,
            Err(e) => {
                refused.push(e);
                continue;
            }
        };
        if !refused.is_empty() {
            continue;
        }
        out.models.push(model_facts(&mut ast, &forms, file)?);
    }
    if !refused.is_empty() {
        return Err(ExpectedError::Parse(refused));
    }
    Ok(out)
}

/// A model's forms, each a fact or a stored negation, printed by the one
/// printer, sorted: the facts loaded into a fresh store, as a program's are,
/// so a value is the value a program's fact of the same text holds.
fn model_facts(
    ast: &mut ein_ir::Ast,
    forms: &[ein_ir::NodeId],
    file: &str,
) -> Result<Vec<String>, ExpectedError> {
    const NOT_FACTS: [&str; 9] = [
        "relation", "rule", "hrule", "query", "solve", "config", "trace", "macro", "import",
    ];
    for &form in forms {
        if let Some(head) = ast.head_name(form).filter(|h| NOT_FACTS.contains(h)) {
            let at = first_loc(ast, form)
                .map_or_else(String::new, |l| format!("{file}:{}:{}: ", l.line, l.col));
            return Err(ExpectedError::NotFacts(format!(
                "{at}a model lists facts, and ({head} …) is no fact"
            )));
        }
    }
    let mut terms = Terms::new();
    let kb = ein_ir::load(ast, &mut terms, forms, None)
        .map_err(|e| ExpectedError::NotFacts(format!("a model's facts do not load: {e}")))?;
    let mut facts: Vec<String> = kb.facts().map(|f| terms.fact_text(f)).collect();
    facts.sort();
    Ok(facts)
}

/// The first position under `node`: a top-level form keeps none.
fn first_loc(ast: &ein_ir::Ast, node: ein_ir::NodeId) -> Option<ein_ir::Loc> {
    if let Some(l) = ast.loc(node) {
        return Some(l);
    }
    match ast.node(node) {
        ein_ir::Node::SForm { head, .. } => first_loc(ast, head)
            .or_else(|| ast.form_args(node).iter().find_map(|&a| first_loc(ast, a))),
        ein_ir::Node::KwPair { key, value } => {
            first_loc(ast, key).or_else(|| first_loc(ast, value))
        }
        _ => None,
    }
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
/// spurious state, so a caller that shares stdout's lines can cut the items.
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
        let e = parse_expected("closes q\nmodel\n", "<want>").expect("parses");
        assert_eq!(e.closes.as_deref(), Some(&["q".to_string()][..]));
        assert_eq!(e.models, vec![Vec::<String>::new()]);
        assert!(diff_model_sets(&s(&[&[]]), &e.models).agrees());
        assert_eq!(diff_model_sets(&[], &e.models).missing, [0]);
    }

    /// The format's own refusals keep their `line n:` sentences; a model's
    /// facts are the parser's, so `p A` is refused as a line the format does
    /// not know, and a form that is no fact is refused with its position.
    #[test]
    fn the_file_format_refuses_what_it_cannot_read() {
        let err = |t: &str| parse_expected(t, "<want>").expect_err(t).to_string();
        assert_eq!(
            err("(p A)\n"),
            "line 1: expected `closes …` or `model` before the first fact, got \"(p A)\""
        );
        assert_eq!(
            err("closes p\ncloses q\n"),
            "line 2: a second `closes` line"
        );
        assert!(
            err("model\np A\n")
                .starts_with("a model's facts do not parse\n<want>:2:1: unexpected `p`: a name"),
            "{}",
            err("model\np A\n")
        );
        assert_eq!(
            err("model\n(p a)\n(relation p T)\n"),
            "<want>:3:11: a model lists facts, and (relation …) is no fact"
        );
    }

    /// A fact's spelling is not its value: the parser reads it, the one
    /// printer prints it, so spacing, a comment and a line break move nothing,
    /// and a fact listed twice is one.
    #[test]
    fn a_models_facts_are_read_by_value() {
        let e = parse_expected(
            "; the probe\ncloses p q\nmodel\n( p  a )  ; the fact\n(q\n  (p a))\n(p a)\nmodel\n",
            "<want>",
        )
        .expect("parses");
        assert_eq!(
            e.closes.as_deref(),
            Some(&["p".to_string(), "q".to_string()][..])
        );
        assert_eq!(
            e.models,
            vec![
                vec!["(p a)".to_string(), "(q (p a))".to_string()],
                Vec::<String>::new()
            ]
        );
        let unclosed = parse_expected("model\n(p a\nmodel\n(q b\n", "<want>")
            .expect_err("two models' facts do not parse")
            .to_string();
        assert!(
            unclosed.contains("<want>:2:5: unexpected end of input")
                && unclosed.contains("<want>:5:1: unexpected end of input"),
            "every model's report: {unclosed}"
        );
    }
}
