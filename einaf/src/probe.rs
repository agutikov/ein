//! **The probe: Ein written with no grammar.** Whether the model writes a
//! program the kernel parses and loads, before any grammar holds it to Ein,
//! and what it writes where it does not.
//!
//! **The plan is data, fixed before the first generation** ([`plan`]): each
//! run is an item (a text, its family, the program it is set against), a
//! condition (what the prompt holds), thinking on or off, and the request
//! sent, whole. A run's record is one JSON line ([`Line`]): the plan's
//! entry, the call as [`crate::model::Recorded`] keeps it, and the kernel's
//! judgement of the answer under two readings, *whole* (the answer is the
//! program) and *fenced* (its first fenced block).
//!
//! **Every judgement is the kernel's, through [`crate::kernel`]**: the parse
//! alone ([`kernel::parse`]), every error with its kind and place; then, for
//! a text that parses, the bounded call ([`kernel::call`]), which loads it as
//! `ein solve` does and asks it at root (`--no-search`) and `--single`,
//! under a clock; and the forms it holds ([`kernel::forms`]), set against
//! the program the item is measured from. A kind of error is read with what
//! would have prevented it ([`Prevention`]).

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::kernel::{self, Bounds, Feedback, Finding, Parsed, Question, Source};
use crate::model::{self, Message, Prompt, Recorded, Request, Sampler, first_fenced_block};

/// The schema of a line of the probe's record.
pub const SCHEMA: &str = "einaf-probe/1";

/// The excerpt of Ein every prompt holds but the described condition's.
pub const EXCERPT: &str = include_str!("../prompts/ein-excerpt.v1.md");
/// One paragraph on Ein, the described condition's whole knowledge of it.
pub const PARAGRAPH: &str = include_str!("../prompts/ein-paragraph.v1.md");
/// The word theory's vocabulary, the theory-given condition's.
pub const WORD_THEORY: &str = include_str!("../prompts/word-theory.v1.md");
/// The statement of step 1.
pub const STEP1: &str = include_str!("../prompts/step1.v1.md");

/// The repository's root: EinAf's manifest's directory's parent.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("einaf/ is in the repository")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// **What a prompt holds**, a condition of the probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Condition {
    /// **The user's first experiment**: the Zebra text, with a working
    /// Zebra program beside it as the one example, and the excerpt. The
    /// item is shown its own program: the floor of the probe, in no table.
    FirstExperiment,
    /// **The model alone**: the Zebra text and nothing else, no Ein.
    ModelAlone,
    /// **A whole program over the library**: the excerpt, and examples over
    /// the library alone, none of the item's family. The main arms'.
    Library,
    /// **A theory given**: the excerpt, the word theory's vocabulary, and the
    /// corpus's word problems as examples, none of the item's family. The
    /// library ablation's widest, and no arm's.
    TheoryGiven,
    /// **Described**: one paragraph on Ein and no example: what the model's
    /// knowledge of Datalog and Lisp gives alone.
    Described,
    /// **Step 1 alone**: the excerpt, the statement of step 1, and one worked
    /// example of another text's step 1.
    Step1,
}

impl Condition {
    pub fn name(self) -> &'static str {
        match self {
            Condition::FirstExperiment => "first-experiment",
            Condition::ModelAlone => "model-alone",
            Condition::Library => "library",
            Condition::TheoryGiven => "theory-given",
            Condition::Described => "described",
            Condition::Step1 => "step1",
        }
    }
}

/// A text the probe asks about.
#[derive(Clone, Debug)]
struct Text {
    id: &'static str,
    family: &'static str,
    text: String,
    /// The problems the text stands for: a text several problems share is
    /// asked once, since a greedy run of one prompt is one output.
    stands_for: Vec<&'static str>,
}

/// The families, fixed before the run: an item is never shown an example of
/// its own family, the first experiment aside.
pub const FAMILIES: [(&str, &str); 8] = [
    ("zebra", "an attribute puzzle of five houses"),
    ("ordering", "an order among people: who is the youngest"),
    ("seating", "people in a row of chairs"),
    (
        "transfer",
        "a giving between people over time: who has how many after",
    ),
    ("kinds", "two kinds of thing: how many of one kind together"),
    ("comparison", "two counts: who has more, and by how many"),
    ("boxes", "containers: how many are needed"),
    ("sharing", "a total shared equally: each share and the rest"),
];

/// The sixteen word problems' texts, each its file's first comment, by the
/// thirteen texts they have: `gap_assume` is `gap`'s text, and
/// `kinds_assume` and `kinds_listed` are `kinds`'.
const WORD: [(&str, &str, &[&str], &str); 13] = [
    (
        "bob1",
        "transfer",
        &["bob1"],
        "Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?",
    ),
    (
        "gap",
        "transfer",
        &["gap", "gap_assume"],
        "Bob has 5 apples. He gave 2 to Alice. How many apples do they have together?",
    ),
    (
        "gap_fact",
        "transfer",
        &["gap_fact"],
        "Bob has 5 apples. Alice has 4 apples. He gave 2 to Alice. How many apples does Bob have? How many does Alice have? How many apples do they have together?",
    ),
    (
        "contra",
        "transfer",
        &["contra"],
        "Bob has 5 apples. He gave 7 to Alice. How many apples does Bob have?",
    ),
    (
        "unused",
        "transfer",
        &["unused"],
        "Bob has 5 apples. Alice has 7 apples. Carol has 9 apples and a box that holds 3. Bob gave 2 apples to Alice. How many apples does Bob have?",
    ),
    (
        "bystander",
        "transfer",
        &["bystander"],
        "Bob has 5 apples. Carol has 9 apples. Bob gave 2 apples to Alice. How many apples does Carol have?",
    ),
    (
        "before",
        "transfer",
        &["before"],
        "Bob gave Alice 2 apples. Now Bob has 5. How many apples did Bob have before?",
    ),
    (
        "kinds",
        "kinds",
        &["kinds", "kinds_assume", "kinds_listed"],
        "Bob has 5 apples, Alice has 3 pears. How many apples do they have together?",
    ),
    (
        "kinds_full",
        "kinds",
        &["kinds_full"],
        "Bob has 5 apples and 2 pears. Alice has 3 apples and 4 pears. How many apples do they have together?",
    ),
    (
        "more",
        "comparison",
        &["more"],
        "Bob has 5 apples. Alice has 12 apples. Who has more apples, and by how many?",
    ),
    (
        "boxes12",
        "boxes",
        &["boxes12"],
        "Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need?",
    ),
    (
        "boxes13",
        "boxes",
        &["boxes13"],
        "Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need?",
    ),
    (
        "pencils",
        "sharing",
        &["pencils"],
        "A school has 12 boxes of pencils, 144 pencils in each. 250 pencils go to the office. The rest is shared equally among 31 classes. How many pencils does each class get, and how many are left over?",
    ),
];

/// An example a prompt quotes: a text beside its program.
#[derive(Clone, Debug)]
struct Example {
    family: &'static str,
    text: String,
    /// The program's path from the repository's root.
    program: String,
}

/// The examples of the library condition: programs over the library alone,
/// each a text of the formalization contract's worked steps.
fn library_examples() -> Vec<Example> {
    [
        ("ordering", "youngest"),
        ("transfer", "apples"),
        ("comparison", "more"),
        ("seating", "seating"),
    ]
    .into_iter()
    .map(|(family, dir)| Example {
        family,
        text: read(&format!("einaf/contract/{dir}/text.txt"))
            .trim()
            .to_string(),
        program: format!("einaf/contract/{dir}/step3.ein"),
    })
    .collect()
}

/// The examples of the theory-given condition: the corpus's own word
/// problems, one a family.
fn theory_examples() -> Vec<Example> {
    [
        ("transfer", "bob1"),
        ("kinds", "kinds_full"),
        ("comparison", "more"),
        ("boxes", "boxes12"),
        ("sharing", "pencils"),
    ]
    .into_iter()
    .map(|(family, id)| Example {
        family,
        text: WORD
            .iter()
            .find(|w| w.0 == id)
            .map(|w| w.3.to_string())
            .expect("a word problem"),
        program: format!("examples/word_problems/{id}.ein"),
    })
    .collect()
}

/// **One run of the plan**: an item, a condition, thinking on or off, and
/// the request, whole.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Planned {
    /// `condition.thinking.item`, unique in the plan.
    pub id: String,
    pub item: String,
    pub family: String,
    pub condition: Condition,
    pub thinking: bool,
    /// The problems the item's text stands for.
    pub stands_for: Vec<String>,
    /// The item's text.
    pub text: String,
    /// Where the program's local imports are found, from the repository's
    /// root.
    pub base_dir: Option<String>,
    /// Every program the prompt quotes, from the repository's root.
    pub examples: Vec<String>,
    /// The program the output is set against, from the repository's root.
    pub reference: Option<String>,
    pub request: Request,
}

/// The texts of the probe that are not word problems.
fn small_texts() -> Vec<Text> {
    [
        ("youngest", "ordering", "einaf/contract/youngest/text.txt"),
        ("seating", "seating", "einaf/contract/seating/text.txt"),
    ]
    .into_iter()
    .map(|(id, family, path)| Text {
        id,
        family,
        text: read(path).trim().to_string(),
        stands_for: vec![id],
    })
    .collect()
}

fn word_texts() -> Vec<Text> {
    WORD.iter()
        .map(|&(id, family, stands_for, text)| Text {
            id,
            family,
            text: text.to_string(),
            stands_for: stands_for.to_vec(),
        })
        .collect()
}

fn zebra_text() -> String {
    read("einaf/contract/zebra/text.txt")
}

/// The budget of generated tokens: what a program needs with thinking off,
/// and room to think with it on.
fn budget(condition: Condition, thinking: bool) -> u32 {
    match (condition, thinking) {
        (Condition::FirstExperiment | Condition::ModelAlone, true) => 55_000,
        (Condition::FirstExperiment, false) => 16_384,
        (_, true) => 16_384,
        (_, false) => 8_192,
    }
}

const SYSTEM: &str = "You formalize problems stated in English as programs in Ein, a small logic language. \
The reference below is what you know of Ein. Read it, then write the program you are asked for.";

fn fence(program: &str) -> String {
    format!("```ein\n{}\n```", program.trim_end())
}

fn examples_block(examples: &[Example]) -> String {
    let mut s = String::new();
    for (i, e) in examples.iter().enumerate() {
        s.push_str(&format!(
            "### Example {}\n\nText:\n{}\n\nProgram:\n{}\n\n",
            i + 1,
            e.text,
            fence(&read(&e.program))
        ));
    }
    s
}

/// The task's closing words: what the program holds, and the answer line.
const WHOLE_TASK: &str = "Write the whole Ein program for this text: each statement as a fact in the text's own \
words with its sentence as `:source`; the relations the facts use, each declared with a `:why`; the theory, imported \
from the standard library or written as rules; the hypothesis mechanism; and one query with a `:goal-text`, a \
sentence that says the answer to the text's question. Reply with the program alone, in one fenced block.";

const THEORY_TASK: &str = "Write the Ein program for this text over the word theory: the imports, each listing the \
short names the program writes; the facts with their sentences as `:source`; the question as an `asked` fact; and \
one query with a `:goal-text`, a sentence that says the answer to the text's question. Reply with the program \
alone, in one fenced block.";

const STEP1_TASK: &str = "Write step 1 of the formalization of this text, and nothing of the later steps. Reply with \
step 1's program alone, in one fenced block.";

fn chat(system: String, user: String, thinking: bool, max_tokens: u32) -> Request {
    Request {
        prompt: Prompt::Chat {
            messages: vec![Message::new("system", system), Message::new("user", user)],
            tools: Vec::new(),
        },
        thinking,
        grammar: None,
        sampler: Sampler::greedy(),
        seed: 1,
        max_tokens,
        logprobs: None,
        keep: None,
    }
}

/// **The probe's plan**, in the order it runs: the first experiment first,
/// thinking off then on; the model alone; then each condition over its
/// items, thinking off then on.
pub fn plan() -> Vec<Planned> {
    let mut out = Vec::new();
    let zebra = zebra_text();
    let target = "examples/zebra3.1.ein";
    let excerpt_system = format!("{SYSTEM}\n\n{EXCERPT}");

    for thinking in [false, true] {
        let user = format!(
            "Here is a text, and an Ein program that formalizes it.\n\n{}### Your task\n\nText:\n{}\n\n{WHOLE_TASK}",
            examples_block(&[Example {
                family: "zebra",
                text: zebra.trim().to_string(),
                program: target.to_string(),
            }]),
            zebra.trim()
        );
        out.push(planned(
            Condition::FirstExperiment,
            thinking,
            "zebra",
            "zebra",
            &["zebra"],
            &zebra,
            None,
            vec![target.to_string()],
            Some(target.to_string()),
            chat(
                excerpt_system.clone(),
                user,
                thinking,
                budget(Condition::FirstExperiment, thinking),
            ),
        ));
    }

    for thinking in [true, false] {
        let request = Request {
            prompt: Prompt::Chat {
                messages: vec![Message::new("user", zebra.trim())],
                tools: Vec::new(),
            },
            ..chat(
                String::new(),
                String::new(),
                thinking,
                if thinking { 65_000 } else { 8_192 },
            )
        };
        out.push(planned(
            Condition::ModelAlone,
            thinking,
            "zebra",
            "zebra",
            &["zebra"],
            &zebra,
            None,
            Vec::new(),
            None,
            request,
        ));
    }

    let library = library_examples();
    let mut items = small_texts();
    items.extend(word_texts());
    for thinking in [false, true] {
        for t in &items {
            let shown: Vec<Example> = library
                .iter()
                .filter(|e| e.family != t.family)
                .cloned()
                .collect();
            let user = format!(
                "Here are examples: each is a text, and an Ein program that formalizes it over the standard \
                 library.\n\n{}### Your task\n\nText:\n{}\n\n{WHOLE_TASK}",
                examples_block(&shown),
                t.text
            );
            let reference = match t.family {
                "ordering" => Some("einaf/contract/youngest/step3.ein".to_string()),
                "seating" => Some("einaf/contract/seating/step3.ein".to_string()),
                _ => None,
            };
            out.push(planned(
                Condition::Library,
                thinking,
                t.id,
                t.family,
                &t.stands_for,
                &t.text,
                None,
                shown.iter().map(|e| e.program.clone()).collect(),
                reference,
                chat(
                    excerpt_system.clone(),
                    user,
                    thinking,
                    budget(Condition::Library, thinking),
                ),
            ));
        }
    }

    let theory = theory_examples();
    for thinking in [false, true] {
        for t in word_texts() {
            let shown: Vec<Example> = theory
                .iter()
                .filter(|e| e.family != t.family)
                .cloned()
                .collect();
            let user = format!(
                "Here are examples: each is a text, and an Ein program that formalizes it over the word \
                 theory.\n\n{}### Your task\n\nText:\n{}\n\n{THEORY_TASK}",
                examples_block(&shown),
                t.text
            );
            out.push(planned(
                Condition::TheoryGiven,
                thinking,
                t.id,
                t.family,
                &t.stands_for,
                &t.text,
                Some("examples/word_problems".to_string()),
                shown.iter().map(|e| e.program.clone()).collect(),
                Some(format!("examples/word_problems/{}.ein", t.stands_for[0])),
                chat(
                    format!("{excerpt_system}\n\n{WORD_THEORY}"),
                    user,
                    thinking,
                    budget(Condition::TheoryGiven, thinking),
                ),
            ));
        }
    }

    for t in &items {
        let user = format!("Text:\n{}\n\n{WHOLE_TASK}", t.text);
        out.push(planned(
            Condition::Described,
            false,
            t.id,
            t.family,
            &t.stands_for,
            &t.text,
            None,
            Vec::new(),
            None,
            chat(
                format!("{SYSTEM}\n\n{PARAGRAPH}"),
                user,
                false,
                budget(Condition::Described, false),
            ),
        ));
    }

    let steps = [
        ("zebra", "zebra"),
        ("youngest", "ordering"),
        ("seating", "seating"),
        ("apples", "transfer"),
        ("together", "transfer"),
        ("more", "comparison"),
    ];
    for thinking in [false, true] {
        for (dir, family) in steps {
            let example = if dir == "youngest" {
                "seating"
            } else {
                "youngest"
            };
            let shown = Example {
                family: "",
                text: read(&format!("einaf/contract/{example}/text.txt"))
                    .trim()
                    .to_string(),
                program: format!("einaf/contract/{example}/step1.ein"),
            };
            let text = read(&format!("einaf/contract/{dir}/text.txt"));
            let user = format!(
                "Here is an example of step 1: a text, and the program step 1 writes for it.\n\n{}### Your \
                 task\n\nText:\n{}\n\n{STEP1_TASK}",
                examples_block(std::slice::from_ref(&shown)),
                text.trim()
            );
            out.push(planned(
                Condition::Step1,
                thinking,
                dir,
                family,
                &[dir],
                &text,
                None,
                vec![shown.program.clone()],
                Some(format!("einaf/contract/{dir}/step1.ein")),
                chat(
                    format!("{excerpt_system}\n\n{STEP1}"),
                    user,
                    thinking,
                    budget(Condition::Step1, thinking),
                ),
            ));
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn planned(
    condition: Condition,
    thinking: bool,
    item: &str,
    family: &str,
    stands_for: &[&str],
    text: &str,
    base_dir: Option<String>,
    examples: Vec<String>,
    reference: Option<String>,
    request: Request,
) -> Planned {
    Planned {
        id: format!(
            "{}.{}.{}",
            condition.name(),
            if thinking { "on" } else { "off" },
            item
        ),
        item: item.to_string(),
        family: family.to_string(),
        condition,
        thinking,
        stands_for: stands_for.iter().map(|s| s.to_string()).collect(),
        text: text.trim().to_string(),
        base_dir,
        examples,
        reference,
        request,
    }
}

/// **What would have prevented an error**, by the four the stage names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Prevention {
    /// Any grammar of Ein: a paren, a string where none goes, a keyword a
    /// form does not take, a `:why` missing, a character not ASCII.
    AnyGrammar,
    /// A grammar that knows the program's own names: a goal over a relation
    /// the program never writes, a template's reference past the labels.
    NamesGrammar,
    /// The catalogue of the library: a module or a listed name that does not
    /// exist.
    Catalogue,
    /// Nothing a grammar can do: a refusal of meaning.
    Nothing,
}

impl Prevention {
    pub fn words(self) -> &'static str {
        match self {
            Prevention::AnyGrammar => "any grammar",
            Prevention::NamesGrammar => "a grammar that knows the program's names",
            Prevention::Catalogue => "the catalogue",
            Prevention::Nothing => "nothing a grammar can do",
        }
    }

    /// The prevention of a finding, by its kind.
    pub fn of(kind: &str) -> Prevention {
        match kind {
            "parse"
            | "not-ascii"
            | "nesting-limit"
            | "missing-why"
            | "eq-arity"
            | "open-shape"
            | "object"
            | "negated-name"
            | "declaration"
            | "engine-word"
            | "claim"
            | "malformed-primitive"
            | "rule-shape" => Prevention::AnyGrammar,
            "goal" | "solve-list" | "template" | "macro-fact" | "macro-call" => {
                Prevention::NamesGrammar
            }
            "import" | "import-list" | "module-unreadable" | "full-name" => Prevention::Catalogue,
            _ => Prevention::Nothing,
        }
    }
}

/// The forms of an output set against the program it is measured from, by
/// their canonical texts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Distance {
    /// Forms in both.
    pub same: usize,
    /// The output's forms the reference does not hold.
    pub only_output: Vec<String>,
    /// The reference's forms the output does not hold.
    pub only_reference: Vec<String>,
}

/// **The kernel's judgement of one reading of an answer.**
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Judged {
    /// `whole` or `fenced`.
    pub reading: String,
    /// The text read; `null` where the reading finds none (no fenced block).
    pub text: Option<String>,
    /// The parser's word on it.
    pub parsed: Option<Parsed>,
    /// Whether it loaded: `null` where it did not parse.
    pub loaded: Option<bool>,
    /// Every finding of the parse or the load, in order.
    pub findings: Vec<Finding>,
    /// The kinds of the findings, each once, in order.
    pub kinds: Vec<String>,
    /// What would have prevented each kind, in the kinds' order.
    pub prevention: Vec<Prevention>,
    /// The bounded call's objects: `--no-search`, then `--single`, where it
    /// parsed.
    pub feedback: Vec<Feedback>,
    /// The program's top-level forms, canonical, where it loaded.
    pub forms: Option<Vec<String>>,
    /// The forms set against the reference, where both loaded.
    pub distance: Option<Distance>,
}

/// **A line of the probe's record.**
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    /// [`SCHEMA`].
    pub schema: String,
    pub planned: Planned,
    pub recorded: Recorded,
    pub judged: Vec<Judged>,
}

/// The clock each question of the judgement is asked under.
pub const CLOCK: Duration = Duration::from_secs(30);

fn canonical_forms(text: &str, name: &str, base: Option<&Path>) -> Option<Vec<String>> {
    kernel::forms(Source::Text {
        text,
        name,
        base_dir: base,
    })
    .ok()
    .map(|f| f.forms.into_iter().map(|f| f.canonical).collect())
}

/// **Judge one reading** of an answer: parse it, and where it parses ask the
/// kernel at root and `--single` under [`CLOCK`], and read its forms. A
/// reading with nothing but white space is no program: an answer the model
/// never wrote (its thinking ran to the budget) is not an empty program that
/// loads.
pub fn judge(reading: &str, text: Option<&str>, planned: &Planned) -> Judged {
    let text = text.filter(|t| !t.trim().is_empty());
    let mut j = Judged {
        reading: reading.to_string(),
        text: text.map(str::to_string),
        parsed: None,
        loaded: None,
        findings: Vec::new(),
        kinds: Vec::new(),
        prevention: Vec::new(),
        feedback: Vec::new(),
        forms: None,
        distance: None,
    };
    let Some(text) = text else { return j };
    let name = format!("{}.{}.ein", planned.id, reading);
    let parsed = kernel::parse(text, &name);
    j.findings = parsed.findings.clone();
    let parses = parsed.forms.is_some();
    j.parsed = Some(parsed);
    let base = planned.base_dir.as_ref().map(|d| root().join(d));
    if parses {
        let bounds = Bounds {
            clock: Some(CLOCK),
            ..Bounds::default()
        };
        j.feedback = kernel::call(
            Source::Text {
                text,
                name: &name,
                base_dir: base.as_deref(),
            },
            &[Question::NoSearch, Question::Single(kernel::Part::All)],
            bounds,
        );
        let invalid = j.feedback.first().is_some_and(|f| {
            f.outcome
                .iter()
                .flatten()
                .any(|r| r.outcome == kernel::Outcome::Invalid)
        });
        j.loaded = Some(!invalid);
        if invalid {
            j.findings = j.feedback[0]
                .diagnostics
                .iter()
                .filter(|d| d.severity == "error")
                .cloned()
                .collect();
        } else {
            j.forms = canonical_forms(text, &name, base.as_deref());
        }
    }
    for f in &j.findings {
        if !j.kinds.contains(&f.kind) {
            j.kinds.push(f.kind.clone());
            j.prevention.push(Prevention::of(&f.kind));
        }
    }
    if let (Some(forms), Some(reference)) = (&j.forms, &planned.reference) {
        let path = root().join(reference);
        let ref_text = read(reference);
        if let Some(theirs) = canonical_forms(&ref_text, reference, path.parent()) {
            let same = forms.iter().filter(|f| theirs.contains(f)).count();
            j.distance = Some(Distance {
                same,
                only_output: forms
                    .iter()
                    .filter(|f| !theirs.contains(f))
                    .cloned()
                    .collect(),
                only_reference: theirs
                    .iter()
                    .filter(|f| !forms.contains(f))
                    .cloned()
                    .collect(),
            });
        }
    }
    j
}

/// **Run one planned call** through `runtime`, and judge its answer under
/// both readings. A call that failed is judged under neither.
pub fn run_one(runtime: &mut dyn model::Runtime, planned: &Planned) -> Line {
    let result = runtime.generate(&planned.request);
    let recorded = Recorded::new(&planned.request, &result);
    let judged = match &result {
        Ok(g) => {
            let fenced = first_fenced_block(&g.answer);
            vec![
                judge("whole", Some(&g.answer), planned),
                judge("fenced", fenced.as_deref(), planned),
            ]
        }
        Err(_) => Vec::new(),
    };
    Line {
        schema: SCHEMA.to_string(),
        planned: planned.clone(),
        recorded,
        judged,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_plan_runs_the_first_experiment_first_and_keeps_every_family_rule() {
        let plan = plan();
        assert_eq!(plan[0].id, "first-experiment.off.zebra");
        assert_eq!(plan[1].id, "first-experiment.on.zebra");
        let mut ids: Vec<&str> = plan.iter().map(|p| p.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), plan.len(), "an id twice");
        for p in &plan {
            if p.condition == Condition::FirstExperiment {
                continue;
            }
            for e in &p.examples {
                let family_of = |path: &str| {
                    library_examples()
                        .into_iter()
                        .chain(theory_examples())
                        .find(|x| x.program == path)
                        .map(|x| x.family)
                };
                if let Some(f) = family_of(e) {
                    assert_ne!(f, p.family, "{}: shown {e}, of its own family", p.id);
                }
            }
        }
    }
}
