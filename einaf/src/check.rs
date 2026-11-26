//! **The contract's check**: what EinAf finds of a program that the kernel
//! loads in silence ([the contract, § 1.7](../../docs/einaf/04-contract.md#17-what-a-formalization-comes-to)),
//! over a step's program or the complete one, the input text and the
//! catalogue. It reports and refuses nothing: its findings are the
//! sidecar's, kept apart from the kernel's diagnostics.
//!
//! The seven findings of the steps, each from the step that could have
//! avoided it ([`Finding::from_step`]):
//!
//! 1. a `:source` that is no sentence of the input (from step 1);
//! 2. a selection that is not active (from step 2): an activating fact under
//!    a name the file does not list from the module that declares its rule,
//!    or of a module no import brings, which the program's text shows; and,
//!    read off root's narration ([`crate::kernel::root`]), a selected rule
//!    root names no firing of and a selected obligation root names no `owe`
//!    of, a check aside, which fires only where a state breaks it; and an
//!    import or a selection the catalogue has no entry for;
//! 3. the mechanism (at step 3 alone): the writer of H the kernel reports,
//!    `blind` where the program states none, or one that does not agree with
//!    every mechanism the program states;
//! 4. a fact or a pattern of a relation the program does not declare and no
//!    imported module has, or at a count it is not declared at (a top-level
//!    fact and a goal from step 1, a fact inside a fact from step 2, a rule's
//!    from step 3);
//! 5. a number written as a name, outside a numeral's digit (from step 1);
//! 6. of the answer query (from step 1): none, or more than one; a reference
//!    the goal does not bind; a template with no reference, or one that
//!    writes a name of the program's facts the goal does not hold, which is
//!    an answer written without the kernel;
//! 7. a step-1 statement written again otherwise (from step 2): a fact a later
//!    step writes with the `:source` of a sentence step 1 stated, of no
//!    relation step 1's memberships are of, and not among step 1's
//!    statements.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::catalogue::{Catalogue, Generated};
use crate::contract;
use crate::grammar::steps as gsteps;
use crate::kernel::{self, Forms, Root, Shape, Source, Written};

/// **What a finding is**, by the seven of the steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// 1: a `:source` that is no sentence of the input.
    SourceNotASentence,
    /// 2: an activating fact under a name the file does not list from the
    /// module that declares its rule, or of a module no import brings.
    SelectionNotListed,
    /// 2: a selected rule root names no firing of, or a selected obligation
    /// no `owe` of.
    SelectionInactive,
    /// 2: an import or a selecting fact the catalogue has no entry for.
    SelectionNotInCatalogue,
    /// 3: the writer of H is `blind`, and the program states no mechanism.
    MechanismNotStated,
    /// 3: the writer of H is not every one the program states.
    MechanismNotTheOneStated,
    /// 4: a fact or a pattern of a relation the program does not declare
    /// and no imported module has.
    Undeclared,
    /// 4: a fact or a pattern at a count its relation is not declared at.
    WrongCount,
    /// 5: a number written as a name.
    NumberAsName,
    /// 6: no query with a `:goal-text`.
    NoAnswerQuery,
    /// 6: more than one query with a `:goal-text`.
    SeveralAnswerQueries,
    /// 6: a `:goal-text` reference the goal does not bind.
    UnboundReference,
    /// 6: a template with no reference, or one that writes a name of the
    /// program's facts the goal does not hold.
    AnswerInTemplate,
    /// 7: a step-1 statement written again otherwise by a later step.
    Rewritten,
}

impl Kind {
    /// The finding's number among the seven.
    pub fn number(self) -> u8 {
        match self {
            Kind::SourceNotASentence => 1,
            Kind::SelectionNotListed | Kind::SelectionInactive | Kind::SelectionNotInCatalogue => 2,
            Kind::MechanismNotStated | Kind::MechanismNotTheOneStated => 3,
            Kind::Undeclared | Kind::WrongCount => 4,
            Kind::NumberAsName => 5,
            Kind::NoAnswerQuery
            | Kind::SeveralAnswerQueries
            | Kind::UnboundReference
            | Kind::AnswerInTemplate => 6,
            Kind::Rewritten => 7,
        }
    }

    /// The name the record gives it.
    pub fn name(self) -> &'static str {
        match self {
            Kind::SourceNotASentence => "source-not-a-sentence",
            Kind::SelectionNotListed => "selection-not-listed",
            Kind::SelectionInactive => "selection-inactive",
            Kind::SelectionNotInCatalogue => "selection-not-in-catalogue",
            Kind::MechanismNotStated => "mechanism-not-stated",
            Kind::MechanismNotTheOneStated => "mechanism-not-the-one-stated",
            Kind::Undeclared => "undeclared",
            Kind::WrongCount => "wrong-count",
            Kind::NumberAsName => "number-as-name",
            Kind::NoAnswerQuery => "no-answer-query",
            Kind::SeveralAnswerQueries => "several-answer-queries",
            Kind::UnboundReference => "unbound-reference",
            Kind::AnswerInTemplate => "answer-in-template",
            Kind::Rewritten => "rewritten",
        }
    }
}

/// **One finding**: its kind, the step it applies from, the form it is of,
/// and a sentence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub kind: Kind,
    /// The step whose program it first applies to: the step that could have
    /// avoided it.
    pub from_step: u8,
    /// The form, as `ein_ir::dump_canonical` prints it.
    pub form: Option<String>,
    pub message: String,
}

/// **What step 1 stated**, which finding 7 holds a later step to.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step1 {
    /// Each statement's canonical text and its `:source`.
    pub statements: Vec<(String, Option<String>)>,
    /// The relations step 1's ontology wrote facts of: its memberships'.
    pub memberships: BTreeSet<String>,
}

/// **The mechanism**: what the kernel reports and what the program states.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mechanism {
    /// The writer of H, as `ein show hyps` names it.
    pub writer: Option<String>,
    /// How many candidates H holds.
    pub candidates: usize,
    /// What the program states, each a word: `no-search`,
    /// `hypothesis-relations`, `hypothesis-rules`, `obligations`; `[]` where
    /// it states none.
    pub stated: Vec<String>,
}

/// **The check's reading of one program.**
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checked {
    /// The findings that apply to the step.
    pub findings: Vec<Finding>,
    pub mechanism: Mechanism,
    /// The program did not load, so nothing past its parse was read: the
    /// kernel's refusal, a line.
    pub not_loaded: Option<String>,
}

/// What the check is given.
pub struct Input<'a> {
    /// The step's program, or the complete one.
    pub program: &'a str,
    /// The name the program's messages give it.
    pub name: &'a str,
    /// The step whose program it is: 1, 2 (its second call too) or 3; the
    /// complete program is step 3's.
    pub step: u8,
    /// The problem's text.
    pub text: &'a str,
    pub catalogue: &'a Catalogue,
    pub generated: &'a Generated,
    /// Step 1's statements, for finding 7; `None` at step 1.
    pub step1: Option<&'a Step1>,
    /// Root of the program, as [`kernel::root`] builds it.
    pub root: &'a Root,
}

/// **Run the check** on one program: the findings that apply to its step.
pub fn check(input: &Input<'_>) -> Checked {
    let mut out = Checked::default();
    let forms = match kernel::forms(Source::Text {
        text: input.program,
        name: input.name,
        base_dir: None,
    }) {
        Ok(f) => f,
        Err(findings) => {
            out.not_loaded = Some(
                findings
                    .first()
                    .map(|f| f.message.clone())
                    .unwrap_or_else(|| "refused".to_string()),
            );
            return out;
        }
    };
    let mut found = Vec::new();
    sources(&forms, input.text, &mut found);
    selections(&forms, input, &mut found);
    out.mechanism = mechanism(&forms, input);
    if input.step >= 3 {
        mechanism_findings(&out.mechanism, &mut found);
    }
    declarations(&forms, &mut found);
    numbers(&forms, &mut found);
    answer_query(&forms, &mut found);
    if let Some(step1) = input.step1 {
        rewritten(&forms, step1, &mut found);
    }
    found.retain(|f| f.from_step <= input.step);
    found.sort_by(|a, b| {
        (a.kind.number(), a.kind, &a.form).cmp(&(b.kind.number(), b.kind, &b.form))
    });
    found.dedup();
    out.findings = found;
    out
}

/// **Step 1's statements and memberships**, read off step 1's output: the
/// statements are the forms before its first `relation` form, the
/// memberships the facts after it.
pub fn step1_of(output: &str) -> Step1 {
    let Ok(forms) = kernel::shapes(output, "step 1") else {
        return Step1::default();
    };
    let first = forms
        .iter()
        .position(|f| matches!(f.shape, Shape::Relation { .. }))
        .unwrap_or(forms.len());
    let mut s = Step1::default();
    for (i, f) in forms.iter().enumerate() {
        if let Shape::Fact {
            relation, source, ..
        } = &f.shape
        {
            if i < first {
                s.statements.push((f.canonical.clone(), source.clone()));
            } else {
                s.memberships.insert(relation.clone());
            }
        }
    }
    s
}

// ── 1 ───────────────────────────────────────────────────────────────

fn sources(forms: &Forms, text: &str, out: &mut Vec<Finding>) {
    let sentences = contract::sentences(text);
    for f in &forms.forms {
        if let Shape::Fact {
            source: Some(source),
            ..
        } = &f.shape
            && !sentences.iter().any(|s| s == source.trim())
        {
            out.push(Finding {
                kind: Kind::SourceNotASentence,
                from_step: 1,
                form: Some(f.canonical.clone()),
                message: format!("its :source \"{source}\" is no sentence of the input"),
            });
        }
    }
}

// ── 2 ───────────────────────────────────────────────────────────────

fn selections(forms: &Forms, input: &Input<'_>, out: &mut Vec<Finding>) {
    let library = gsteps::Catalogue::library();
    let offered = input.catalogue.grammar();
    let offered_modules: BTreeSet<&str> =
        offered.modules.iter().map(|m| m.module.as_str()).collect();
    let offered_heads: BTreeSet<&str> = offered
        .modules
        .iter()
        .flat_map(|m| m.activators.keys().chain(m.unlisted.keys()))
        .map(String::as_str)
        .collect();
    let own_declared = forms.declared();
    let own_rules = forms.rules();
    let imported: BTreeSet<&str> = forms
        .forms
        .iter()
        .filter_map(|f| match &f.shape {
            Shape::Import { module } => Some(module.as_str()),
            _ => None,
        })
        .collect();
    for f in &forms.forms {
        if let Shape::Import { module } = &f.shape
            && !offered_modules.contains(module.as_str())
        {
            out.push(Finding {
                kind: Kind::SelectionNotInCatalogue,
                from_step: 2,
                form: Some(f.canonical.clone()),
                message: format!("the catalogue has no entry of {module}"),
            });
        }
        let Shape::Fact {
            relation,
            written: Written::Holds,
            source: None,
        } = &f.shape
        else {
            continue;
        };
        let head = relation.as_str();
        let short = head.rsplit('.').next().unwrap_or(head);
        let is_rule = forms.library.rules.contains(head);
        let is_read =
            forms.library.reads.contains(head) && !forms.library.declared.contains_key(head);
        if is_rule || is_read {
            if !offered_heads.contains(short) {
                out.push(Finding {
                    kind: Kind::SelectionNotInCatalogue,
                    from_step: 2,
                    form: Some(f.canonical.clone()),
                    message: format!("the catalogue has no entry that selects with {head}"),
                });
            }
            if is_rule && input.generated.is_check(head) {
                continue;
            }
            if !active(&f.canonical, head, input.root) && input.root.refused.is_empty() {
                out.push(Finding {
                    kind: Kind::SelectionInactive,
                    from_step: 2,
                    form: Some(f.canonical.clone()),
                    message: format!(
                        "root's saturation names no firing of {head} with these arguments and no owe of it"
                    ),
                });
            }
            continue;
        }
        if own_declared.contains(head) || own_rules.contains(head) {
            continue;
        }
        // A name of the library's that the file writes as its own.
        for m in &library.modules {
            let selects = m.activators.contains_key(head) || m.unlisted.contains_key(head);
            if !selects {
                continue;
            }
            let message = if imported.contains(m.module.as_str()) {
                format!(
                    "{head} is a rule of {} the import does not list: the name is the file's own, and the fact selects nothing",
                    m.module
                )
            } else {
                format!(
                    "{head} selects from {}, which no import brings: the fact selects nothing",
                    m.module
                )
            };
            out.push(Finding {
                kind: Kind::SelectionNotListed,
                from_step: 2,
                form: Some(f.canonical.clone()),
                message,
            });
            break;
        }
    }
}

/// Whether root's narration shows a selecting fact at work: its rule fired
/// or owed with its arguments as the activator, or a firing matched it.
fn active(canonical: &str, head: &str, root: &Root) -> bool {
    let args: Vec<String> = match sexpr(canonical) {
        Some(Sx::List(items)) => items.iter().skip(1).map(Sx::text).collect(),
        _ => Vec::new(),
    };
    let fact = normalise(canonical);
    root.fired.iter().chain(root.owed.iter()).any(|e| {
        (e.rule == head && e.activator == args) || e.premises.iter().any(|p| normalise(p) == fact)
    })
}

fn normalise(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ── 3 ───────────────────────────────────────────────────────────────

fn mechanism(forms: &Forms, input: &Input<'_>) -> Mechanism {
    let mut stated = BTreeSet::new();
    for f in &forms.forms {
        match &f.shape {
            Shape::Solve => {
                if f.canonical.contains(":no-search") {
                    stated.insert("no-search");
                }
                if f.canonical.contains(":hypothesis-relations") {
                    stated.insert("hypothesis-relations");
                }
            }
            Shape::Fact {
                written: Written::Open,
                ..
            } => {
                stated.insert("hypothesis-rules");
            }
            Shape::Rule { .. }
                if f.canonical.contains(":assert (open") || f.canonical.contains("(open (") =>
            {
                stated.insert("hypothesis-rules");
            }
            Shape::Fact {
                relation,
                written: Written::Holds,
                source: None,
            } => {
                let short = relation.rsplit('.').next().unwrap_or(relation);
                let obligation = input.catalogue.entries.iter().any(|e| {
                    e.kind == crate::catalogue::Kind::Obligation
                        && e.selects.starts_with(&format!("({short} "))
                });
                if obligation
                    && (forms.library.rules.contains(relation.as_str())
                        || forms.library.reads.contains(relation.as_str()))
                {
                    stated.insert("obligations");
                }
            }
            _ => {}
        }
    }
    Mechanism {
        writer: input.root.writer.clone(),
        candidates: input.root.candidates,
        stated: stated.into_iter().map(str::to_string).collect(),
    }
}

fn mechanism_findings(m: &Mechanism, out: &mut Vec<Finding>) {
    let Some(writer) = m.writer.as_deref() else {
        return;
    };
    if m.stated.is_empty() {
        if writer == "blind" {
            out.push(Finding {
                kind: Kind::MechanismNotStated,
                from_step: 3,
                form: None,
                message: format!(
                    "the program states no hypothesis mechanism, and the blind enumerator wrote H: {} candidates",
                    m.candidates
                ),
            });
        }
        return;
    }
    // Every mechanism the program states agrees with the writer: a
    // `(solve :no-search)` beside a selected obligation takes the
    // obligations' candidates away, and the writer is `no-search`.
    let agrees = m.stated.iter().all(|s| match s.as_str() {
        "no-search" => writer == "no-search",
        "hypothesis-relations" => writer == "blind",
        "hypothesis-rules" => matches!(writer, "hypothesis-rules" | "union"),
        "obligations" => matches!(writer, "obligations" | "union" | "stuck" | "declined"),
        _ => false,
    });
    if !agrees {
        out.push(Finding {
            kind: Kind::MechanismNotTheOneStated,
            from_step: 3,
            form: None,
            message: format!(
                "the program states {}, and the writer of H is {writer}",
                m.stated.join(" and ")
            ),
        });
    }
}

// ── 4 ───────────────────────────────────────────────────────────────

const WORDS: [&str; 8] = ["and", "or", "not", "absent", "open", "eq", "neq", "false"];

fn declarations(forms: &Forms, out: &mut Vec<Finding>) {
    let declared = &forms.library.declared;
    let exempt = |r: &str| {
        WORDS.contains(&r)
            || r == "__closed__"
            || r == "relation"
            || forms.library.rules.contains(r)
            || forms.rules().contains(r)
            || (forms.library.reads.contains(r) && !declared.contains_key(r))
    };
    for f in &forms.forms {
        let (patterns, from): (Vec<(String, usize, u8)>, &str) = match &f.shape {
            Shape::Fact { .. } => (fact_patterns(&f.canonical), "fact"),
            Shape::Query { .. } => (keyword_patterns(&f.canonical, ":goal", 1), "goal"),
            Shape::Rule { .. } => {
                let mut p = keyword_patterns(&f.canonical, ":match", 3);
                p.extend(keyword_patterns(&f.canonical, ":assert", 3));
                (p, "rule")
            }
            _ => continue,
        };
        for (relation, count, step) in patterns {
            if exempt(&relation) {
                continue;
            }
            match declared.get(&relation) {
                None => out.push(Finding {
                    kind: Kind::Undeclared,
                    from_step: step,
                    form: Some(f.canonical.clone()),
                    message: format!(
                        "a {from} of {relation}, which the program does not declare and no imported module has"
                    ),
                }),
                Some(counts) if !counts.contains(&count) => out.push(Finding {
                    kind: Kind::WrongCount,
                    from_step: step,
                    form: Some(f.canonical.clone()),
                    message: format!(
                        "a {from} of {relation} with {count} arguments, which is declared with {}",
                        counts
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(" or ")
                    ),
                }),
                Some(_) => {}
            }
        }
    }
}

/// A top-level fact's relation and count (from step 1), and each fact
/// inside it (from step 2, where the theory that reads it is imported).
fn fact_patterns(canonical: &str) -> Vec<(String, usize, u8)> {
    let Some(Sx::List(items)) = sexpr(canonical) else {
        return Vec::new();
    };
    let items: Vec<Sx> = strip_keywords(&items);
    let mut out = Vec::new();
    let (fact, nested_from) = match items.first().map(Sx::text).as_deref() {
        Some("not" | "open") => match items.get(1) {
            Some(Sx::List(inner)) => (strip_keywords(inner), 2),
            _ => return out,
        },
        _ => (items, 2),
    };
    if let Some(Sx::Atom(head)) = fact.first() {
        out.push((head.clone(), fact.len() - 1, 1));
    }
    for a in fact.iter().skip(1) {
        nested(a, nested_from, &mut out);
    }
    out
}

fn nested(sx: &Sx, step: u8, out: &mut Vec<(String, usize, u8)>) {
    if let Sx::List(items) = sx {
        if let Some(Sx::Atom(head)) = items.first()
            && !WORDS.contains(&head.as_str())
            && !head.starts_with('?')
        {
            out.push((head.clone(), items.len() - 1, step));
        }
        for a in items.iter().skip(1) {
            nested(a, step, out);
        }
    }
}

/// The patterns under a form's keyword: each fact pattern, through the
/// connectives, at `step`.
fn keyword_patterns(canonical: &str, key: &str, step: u8) -> Vec<(String, usize, u8)> {
    let Some(Sx::List(items)) = sexpr(canonical) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if let Some(i) = items
        .iter()
        .position(|s| matches!(s, Sx::Atom(a) if a == key))
        && let Some(value) = items.get(i + 1)
    {
        patterns(value, step, &mut out);
    }
    out
}

fn patterns(sx: &Sx, step: u8, out: &mut Vec<(String, usize, u8)>) {
    let Sx::List(items) = sx else {
        return;
    };
    match items.first() {
        Some(Sx::Atom(h)) if WORDS.contains(&h.as_str()) => {
            if !matches!(h.as_str(), "eq" | "neq") {
                for a in items.iter().skip(1) {
                    patterns(a, step, out);
                }
            }
        }
        Some(Sx::Atom(h)) if !h.starts_with('?') => {
            out.push((h.clone(), items.len() - 1, step));
            for a in items.iter().skip(1) {
                nested(a, step, out);
            }
        }
        _ => {}
    }
}

fn strip_keywords(items: &[Sx]) -> Vec<Sx> {
    let mut out = Vec::new();
    let mut skip = false;
    for i in items {
        if skip {
            skip = false;
            continue;
        }
        if matches!(i, Sx::Atom(a) if a.starts_with(':')) {
            skip = true;
            continue;
        }
        out.push(i.clone());
    }
    out
}

// ── 5 ───────────────────────────────────────────────────────────────

fn numbers(forms: &Forms, out: &mut Vec<Finding>) {
    for f in &forms.forms {
        if !matches!(
            f.shape,
            Shape::Fact { .. } | Shape::Query { .. } | Shape::Rule { .. }
        ) {
            continue;
        }
        let Some(sx) = sexpr(&f.canonical) else {
            continue;
        };
        let mut names = Vec::new();
        digits(&sx, false, &mut names);
        for n in names {
            out.push(Finding {
                kind: Kind::NumberAsName,
                from_step: 1,
                form: Some(f.canonical.clone()),
                message: format!("{n} is a number written as a name; a quantity a rule computes with is a numeral, (n z 5)"),
            });
        }
    }
}

/// Every name of digits in `sx` but a numeral's digit, the last place of an
/// `(n X D)`.
fn digits(sx: &Sx, digit_place: bool, out: &mut Vec<String>) {
    match sx {
        Sx::Atom(a) => {
            if !digit_place && !a.is_empty() && a.chars().all(|c| c.is_ascii_digit()) {
                out.push(a.clone());
            }
        }
        Sx::Str(_) => {}
        Sx::List(items) => {
            let numeral =
                matches!(items.first(), Some(Sx::Atom(h)) if h == "n") && items.len() == 3;
            for (i, a) in items.iter().enumerate() {
                digits(a, numeral && i == 2, out);
            }
        }
    }
}

// ── 6 ───────────────────────────────────────────────────────────────

fn answer_query(forms: &Forms, out: &mut Vec<Finding>) {
    let answers: Vec<&kernel::Form> = forms
        .forms
        .iter()
        .filter(|f| {
            matches!(
                f.shape,
                Shape::Query {
                    goal_text: true,
                    ..
                }
            )
        })
        .collect();
    match answers.len() {
        0 => {
            out.push(Finding {
                kind: Kind::NoAnswerQuery,
                from_step: 1,
                form: None,
                message: "no query carries a :goal-text: the program has no answer".to_string(),
            });
            return;
        }
        1 => {}
        n => out.push(Finding {
            kind: Kind::SeveralAnswerQueries,
            from_step: 1,
            form: Some(answers[1].canonical.clone()),
            message: format!("{n} queries carry a :goal-text; the first is the answer query"),
        }),
    }
    let q = answers[0];
    let Some(Sx::List(items)) = sexpr(&q.canonical) else {
        return;
    };
    let get = |key: &str| {
        items
            .iter()
            .position(|s| matches!(s, Sx::Atom(a) if a == key))
            .and_then(|i| items.get(i + 1))
    };
    let mut goal_atoms = BTreeSet::new();
    if let Some(goal) = get(":goal") {
        atoms(goal, &mut goal_atoms);
    }
    let Some(Sx::Str(template)) = get(":goal-text") else {
        return;
    };
    let refs = references(template);
    if refs.is_empty() {
        out.push(Finding {
            kind: Kind::AnswerInTemplate,
            from_step: 1,
            form: Some(q.canonical.clone()),
            message: "its :goal-text holds no reference: the answer is written without the kernel"
                .to_string(),
        });
    }
    for r in &refs {
        if !goal_atoms.contains(&format!("?{r}")) {
            out.push(Finding {
                kind: Kind::UnboundReference,
                from_step: 1,
                form: Some(q.canonical.clone()),
                message: format!("{{?{r}}} is no variable of the goal, and prints as written"),
            });
        }
    }
    // A name of the program's facts that the template writes and the goal
    // does not hold.
    let mut fact_names = BTreeSet::new();
    for f in &forms.forms {
        if let Shape::Fact { .. } = f.shape
            && let Some(Sx::List(items)) = sexpr(&f.canonical)
        {
            for a in strip_keywords(&items).iter().skip(1) {
                atoms(a, &mut fact_names);
            }
        }
    }
    let mut stripped = template.clone();
    for r in &refs {
        stripped = stripped.replace(&format!("{{?{r}}}"), " ");
    }
    for word in stripped.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-')) {
        if word.is_empty() || !word.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
            continue;
        }
        if fact_names.contains(word) && !goal_atoms.contains(word) {
            out.push(Finding {
                kind: Kind::AnswerInTemplate,
                from_step: 1,
                form: Some(q.canonical.clone()),
                message: format!("its :goal-text writes {word}, a name of the program's facts the goal does not hold"),
            });
        }
    }
}

fn references(template: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = template;
    while let Some(i) = rest.find("{?") {
        let after = &rest[i + 2..];
        match after.find('}') {
            Some(j) => {
                out.push(after[..j].to_string());
                rest = &after[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn atoms(sx: &Sx, out: &mut BTreeSet<String>) {
    match sx {
        Sx::Atom(a) => {
            out.insert(a.clone());
        }
        Sx::Str(_) => {}
        Sx::List(items) => items.iter().for_each(|i| atoms(i, out)),
    }
}

// ── 7 ───────────────────────────────────────────────────────────────

fn rewritten(forms: &Forms, step1: &Step1, out: &mut Vec<Finding>) {
    let stated: BTreeSet<&str> = step1.statements.iter().map(|(c, _)| c.as_str()).collect();
    let by_source: BTreeMap<&str, &str> = step1
        .statements
        .iter()
        .filter_map(|(c, s)| s.as_deref().map(|s| (s, c.as_str())))
        .collect();
    for f in &forms.forms {
        let Shape::Fact {
            relation,
            source: Some(source),
            ..
        } = &f.shape
        else {
            continue;
        };
        if stated.contains(f.canonical.as_str()) || step1.memberships.contains(relation) {
            continue;
        }
        if let Some(was) = by_source.get(source.as_str()) {
            out.push(Finding {
                kind: Kind::Rewritten,
                from_step: 2,
                form: Some(f.canonical.clone()),
                message: format!("a later step states \"{source}\" otherwise: step 1 wrote {was}"),
            });
        }
    }
}

// ── A form's text, read as an s-expression ─────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
enum Sx {
    Atom(String),
    Str(String),
    List(Vec<Sx>),
}

impl Sx {
    fn text(&self) -> String {
        match self {
            Sx::Atom(a) => a.clone(),
            Sx::Str(s) => format!("\"{s}\""),
            Sx::List(items) => format!(
                "({})",
                items.iter().map(Sx::text).collect::<Vec<_>>().join(" ")
            ),
        }
    }
}

/// A canonical form's text as an s-expression: lists, strings and atoms.
fn sexpr(text: &str) -> Option<Sx> {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    parse(&chars, &mut i)
}

fn parse(c: &[char], i: &mut usize) -> Option<Sx> {
    while *i < c.len() {
        match c[*i] {
            ';' => {
                while *i < c.len() && c[*i] != '\n' {
                    *i += 1;
                }
            }
            w if w.is_whitespace() => *i += 1,
            _ => break,
        }
    }
    match *c.get(*i)? {
        '(' => {
            *i += 1;
            let mut items = Vec::new();
            loop {
                while *i < c.len() && c[*i].is_whitespace() {
                    *i += 1;
                }
                if *i >= c.len() {
                    return Some(Sx::List(items));
                }
                if c[*i] == ')' {
                    *i += 1;
                    return Some(Sx::List(items));
                }
                items.push(parse(c, i)?);
            }
        }
        '"' => {
            *i += 1;
            let mut s = String::new();
            while *i < c.len() && c[*i] != '"' {
                if c[*i] == '\\' && *i + 1 < c.len() {
                    *i += 1;
                }
                s.push(c[*i]);
                *i += 1;
            }
            *i += 1;
            Some(Sx::Str(s))
        }
        _ => {
            let mut s = String::new();
            while *i < c.len() && !c[*i].is_whitespace() && !matches!(c[*i], '(' | ')' | '"') {
                s.push(c[*i]);
                *i += 1;
            }
            Some(Sx::Atom(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_numeral_s_digit_is_no_number_written_as_a_name() {
        let mut found = Vec::new();
        digits(
            &sexpr("(has Bob (n (n z 1) 2) 5)").unwrap(),
            false,
            &mut found,
        );
        assert_eq!(found, ["5"]);
    }

    #[test]
    fn a_fact_s_patterns_are_its_relation_and_the_facts_inside_it() {
        assert_eq!(
            fact_patterns("(has Bob (n z 5) Apple :source \"Bob has 5 apples.\")"),
            [("has".to_string(), 3, 1), ("n".to_string(), 2, 2)]
        );
        assert_eq!(
            keyword_patterns(
                "(query :goal (and (p ?x) (not (q ?x a))) :goal-text \"{?x}\")",
                ":goal",
                1
            ),
            [("p".to_string(), 1, 1), ("q".to_string(), 2, 1)]
        );
    }
}
