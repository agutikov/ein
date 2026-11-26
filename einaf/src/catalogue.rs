//! **The catalogue**: the theories of Ein's standard library a program may
//! select, each an entry program the kernel runs, and what steps 2 and 3 of
//! the formalization are shown and held to.
//!
//! **Its written part** is [`ENTRIES`]: for each theory, an entry program
//! under `einaf/catalogue/` (the import line with the names it lists, the
//! facts that select the theory, a fact or two and the answer query), what
//! the selection means, what it gives the hypothesis mechanism, the library's
//! rules that root fires or owes once it is selected, the writer of H the
//! kernel reports, and the answer. The text a prompt shows is cut from the
//! entry programs ([`Catalogue::text`]), so the model is shown what the
//! kernel ran; EinAf's test `einaf/tests/catalogue.rs` holds every entry to
//! what it states.
//!
//! **Its generated part** is [`generated`]: every module of the library the
//! kernel resolves, each rule with its parameters, its own `:why` and what it
//! concludes, read off the module's parse; the library's manifest digest
//! stands at its top.
//!
//! **It holds the library's modules and no family's own.** The whole
//! catalogue has every entry; the minimal one ([`Size::Minimal`]) the
//! relation algebra's and the contradiction, the closure of `std.algebra`.
//! **A directory of modules may be given beside it** ([`Given`],
//! [`Catalogue::with_modules`]): the escape hatch of the ontology's provenance, the widest condition of
//! the library ablation, and no part of the main arms; its modules are shown
//! by their forms, offered to the grammars, and found from the directory
//! when a program that imports one is loaded.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::contract::{self, Category};
use crate::grammar::steps;
use crate::kernel::{self, RuleDoc, Source};

/// What a selection does once it is active.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// A rule that derives facts: it fires at root where its premises hold.
    Derives,
    /// A check: it concludes `(false)` and fires only where a state breaks
    /// it, so root fires it on no program that holds together.
    Check,
    /// An obligation, or a theory whose obligations write H: root owes, and
    /// H is the obligations'.
    Obligation,
    /// A module that acts on every program that imports it, with no fact to
    /// select it.
    Switch,
    /// A request of the calculator: a fact a rule states, answered by the
    /// module's result fact.
    Request,
}

/// **One theory of the catalogue**, and the program that shows it.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    /// `module.what`: the entry's name, and its program's file name.
    pub id: &'static str,
    /// The module the entry selects from, which its group is.
    pub module: &'static str,
    /// The selection, as a program writes it.
    pub selects: &'static str,
    /// What it means, in words.
    pub meaning: &'static str,
    pub kind: Kind,
    /// The library rule whose own `:why` the prompt quotes beside the
    /// meaning, as its module names it; `None` where none says it.
    pub rule: Option<&'static str>,
    /// The library's rules root fires or owes once the selection is active,
    /// as root's narration names them.
    pub fires: &'static [&'static str],
    /// What the selection gives the hypothesis mechanism, in words.
    pub gives: &'static str,
    /// The writer of H the kernel reports for the entry program.
    pub writer: &'static str,
    /// The entry program's outcome under `--single`: `one` or `inconsistent`.
    pub outcome: &'static str,
    /// The answer query's lines under `--single`, in their order.
    pub answer: &'static [&'static str],
    /// The entry program.
    pub program: &'static str,
}

impl Entry {
    /// The entry program's path, from the package's root.
    pub fn file(&self) -> String {
        format!("catalogue/{}.ein", self.id)
    }

    /// The lines a prompt shows of the entry program: all but its header,
    /// the `;;;` lines it opens with.
    pub fn shown(&self) -> String {
        let lines: Vec<&str> = self
            .program
            .lines()
            .skip_while(|l| l.starts_with(";;;") || l.trim().is_empty())
            .collect();
        format!("{}\n", lines.join("\n").trim_end())
    }

    /// Whether the minimal catalogue holds it: the closure of `std.algebra`.
    pub fn minimal(&self) -> bool {
        matches!(self.module, "std.algebra" | "std.contradiction")
    }
}

/// The modules the groups are, in the order the catalogue shows them, each
/// with its words.
pub const GROUPS: &[(&str, &str)] = &[
    (
        "std.algebra",
        "properties of a relation of two places, each selected by one fact of its name; the import lists every name the program writes",
    ),
    (
        "std.slots",
        "things in places: one relation whose classes are the places, each holding one value of every kind; its two selecting facts are no module's names, so the import lists nothing",
    ),
    (
        "std.bijection",
        "a relation one to one between the members of its two labels; its selecting facts are no module's names, so the import lists nothing",
    ),
    (
        "std.contradiction",
        "a fact beside its stored negation is a contradiction; std.algebra, std.slots and std.bijection import it already",
    ),
    (
        "std.math.decimal-arith",
        "the calculator: a natural number is a numeral, z is zero, (n z 5) is 5, (n (n z 1) 2) is 12; a rule states a request and reads its result back; the three imports always go together",
    ),
];

/// **The entries**, in the order the catalogue shows them.
pub const ENTRIES: &[Entry] = &[
    Entry {
        id: "algebra.transitive",
        module: "std.algebra",
        selects: "(transitive R)",
        meaning: "R is transitive: R a b and R b c give R a c",
        kind: Kind::Derives,
        rule: Some("transitive"),
        fires: &["transitive"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann is an ancestor of Bob.", "Ann is an ancestor of Cy."],
        program: include_str!("../catalogue/algebra.transitive.ein"),
    },
    Entry {
        id: "algebra.symmetric",
        module: "std.algebra",
        selects: "(symmetric R)",
        meaning: "R is symmetric: R a b gives R b a, and a stored negation is mirrored too",
        kind: Kind::Derives,
        rule: Some("symmetric"),
        fires: &["symmetric"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Bob is married to Ann."],
        program: include_str!("../catalogue/algebra.symmetric.ein"),
    },
    Entry {
        id: "algebra.includes",
        module: "std.algebra",
        selects: "(includes R S)",
        meaning: "S includes R: every fact of R is a fact of S",
        kind: Kind::Derives,
        rule: Some("includes"),
        fires: &["includes"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann is an ancestor of Bob."],
        program: include_str!("../catalogue/algebra.includes.ein"),
    },
    Entry {
        id: "algebra.converse",
        module: "std.algebra",
        selects: "(converse R S)",
        meaning: "S is R turned round: R a b gives S b a, and S b a gives R a b",
        kind: Kind::Derives,
        rule: Some("converse"),
        fires: &["converse"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Bob is a child of Ann."],
        program: include_str!("../catalogue/algebra.converse.ein"),
    },
    Entry {
        id: "algebra.compose",
        module: "std.algebra",
        selects: "(compose R S T)",
        meaning: "R then S is in T: R a b and S b c give T a c",
        kind: Kind::Derives,
        rule: Some("compose"),
        fires: &["compose"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann is a grandparent of Cy."],
        program: include_str!("../catalogue/algebra.compose.ein"),
    },
    Entry {
        id: "algebra.top",
        module: "std.algebra",
        selects: "(top R isa Dom Ran)",
        meaning: "R holds between every member of Dom and every member of Ran, a member being what the membership relation isa says",
        kind: Kind::Derives,
        rule: Some("top"),
        fires: &["top"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann and Rex are a person and a pet."],
        program: include_str!("../catalogue/algebra.top.ein"),
    },
    Entry {
        id: "algebra.meet",
        module: "std.algebra",
        selects: "(meet R S T)",
        meaning: "where R a b and S a b both hold, T a b holds",
        kind: Kind::Derives,
        rule: Some("meet"),
        fires: &["meet"],
        gives: "nothing",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann is close to Bob."],
        program: include_str!("../catalogue/algebra.meet.ein"),
    },
    Entry {
        id: "algebra.functional",
        module: "std.algebra",
        selects: "(functional R)",
        meaning: "a check: R a b and R a c, b and c two things, is a contradiction",
        kind: Kind::Check,
        rule: Some("functional"),
        fires: &[],
        gives: "nothing; it kills a state that breaks it",
        writer: "hypothesis-rules",
        outcome: "one",
        answer: &["Ann sits in the Front seat."],
        program: include_str!("../catalogue/algebra.functional.ein"),
    },
    Entry {
        id: "algebra.injective",
        module: "std.algebra",
        selects: "(injective R)",
        meaning: "a check: R a c and R b c, a and b two things, is a contradiction",
        kind: Kind::Check,
        rule: Some("injective"),
        fires: &[],
        gives: "nothing; it kills a state that breaks it",
        writer: "hypothesis-rules",
        outcome: "one",
        answer: &["Ann sits in the front seat."],
        program: include_str!("../catalogue/algebra.injective.ein"),
    },
    Entry {
        id: "algebra.total-owed",
        module: "std.algebra",
        selects: "(total-owed R isa)",
        meaning: "an obligation: every member of R's first label owes a fact of R with a member of its second label",
        kind: Kind::Obligation,
        rule: Some("total-owed"),
        fires: &["total-owed"],
        gives: "the obligation's candidates are the hypotheses: the program states no other mechanism",
        writer: "obligations",
        outcome: "one",
        answer: &["Bob sits in the Back seat."],
        program: include_str!("../catalogue/algebra.total-owed.ein"),
    },
    Entry {
        id: "slots.partition",
        module: "std.slots",
        selects: "(slot-partition R isa sub Super Index)",
        meaning: "R is an equivalence whose classes are the places: each place holds exactly one member of every kind that is a sub of Super, and its member of the kind Index names it",
        kind: Kind::Obligation,
        rule: Some("slot-partition-setup"),
        fires: &["std.slots.slot-partition-setup"],
        gives: "its obligations propose every placement as a hypothesis: the program states no other mechanism",
        writer: "obligations",
        outcome: "one",
        answer: &["Bob lives in the Right house."],
        program: include_str!("../catalogue/slots.partition.ein"),
    },
    Entry {
        id: "slots.spatial",
        module: "std.slots",
        selects: "(slot-spatial R S isa Index)",
        meaning: "S holds between two values exactly where it holds between their places, the places being the members of Index",
        kind: Kind::Derives,
        rule: Some("slot-spatial-setup"),
        fires: &["std.slots.slot-spatial-setup"],
        gives: "nothing beside slot-partition's obligations",
        writer: "obligations",
        outcome: "one",
        answer: &["Ann lives in the Right house."],
        program: include_str!("../catalogue/slots.spatial.ein"),
    },
    Entry {
        id: "bijection.bijective",
        module: "std.bijection",
        selects: "(bijective R) and (bijection-hierarchy isa)",
        meaning: "R is one to one between the members of its two labels: each member of the first has exactly one of the second, and each of the second exactly one of the first",
        kind: Kind::Obligation,
        rule: Some("bijective-setup"),
        fires: &["std.bijection.bijective-setup"],
        gives: "its obligations propose the facts of R as hypotheses: the program states no other mechanism",
        writer: "obligations",
        outcome: "one",
        answer: &["Cy wears Blue."],
        program: include_str!("../catalogue/bijection.bijective.ein"),
    },
    Entry {
        id: "contradiction",
        module: "std.contradiction",
        selects: "(import std.contradiction)",
        meaning: "a fact beside its stored negation is a contradiction, for every relation the program declares",
        kind: Kind::Switch,
        rule: None,
        fires: &["std.core.inconsistent"],
        gives: "nothing",
        writer: "no-search",
        outcome: "inconsistent",
        answer: &[],
        program: include_str!("../catalogue/contradiction.ein"),
    },
    Entry {
        id: "calc.add",
        module: "std.math.decimal-arith",
        selects: "(add X Y), answered by (sum-of X Y S)",
        meaning: "S is X plus Y",
        kind: Kind::Request,
        rule: None,
        fires: &["std.math.positional.add-answer"],
        gives: "nothing; the entry states (solve :no-search)",
        writer: "no-search",
        outcome: "one",
        answer: &["They have (n (n z 1) 2) apples together."],
        program: include_str!("../catalogue/calc.add.ein"),
    },
    Entry {
        id: "calc.sub",
        module: "std.math.decimal-arith",
        selects: "(sub X Y), answered by (diff-of X Y D R)",
        meaning: "D is X less Y where X is not the smaller, else z; R is Y less X where X is the smaller, else z",
        kind: Kind::Request,
        rule: None,
        fires: &["std.math.positional.sub-answer"],
        gives: "nothing; the entry states (solve :no-search)",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann has (n z 7) more apples."],
        program: include_str!("../catalogue/calc.sub.ein"),
    },
    Entry {
        id: "calc.mul",
        module: "std.math.decimal-arith",
        selects: "(mul X Y), answered by (product-of X Y P)",
        meaning: "P is X times Y",
        kind: Kind::Request,
        rule: None,
        fires: &["std.math.positional.mul-answer"],
        gives: "nothing; the entry states (solve :no-search)",
        writer: "no-search",
        outcome: "one",
        answer: &["The hall has (n (n z 2) 4) seats."],
        program: include_str!("../catalogue/calc.mul.ein"),
    },
    Entry {
        id: "calc.div",
        module: "std.math.decimal-arith",
        selects: "(div X Y), answered by (quot-of X Y Q R)",
        meaning: "Q is X divided by Y and R what is left",
        kind: Kind::Request,
        rule: None,
        fires: &["std.math.positional.div-answer"],
        gives: "nothing; the entry states (solve :no-search)",
        writer: "no-search",
        outcome: "one",
        answer: &["Each friend gets (n z 3) sweets and (n z 2) are left."],
        program: include_str!("../catalogue/calc.div.ein"),
    },
    Entry {
        id: "calc.cmp",
        module: "std.math.decimal-arith",
        selects: "(cmp X Y), answered by (cmp-of X Y O)",
        meaning: "O is lt, eq or gt as X is less than, equal to or more than Y",
        kind: Kind::Request,
        rule: None,
        fires: &["std.math.positional.cmp-answer"],
        gives: "nothing; the entry states (solve :no-search)",
        writer: "no-search",
        outcome: "one",
        answer: &["Ann is older than Bob."],
        program: include_str!("../catalogue/calc.cmp.ein"),
    },
];

/// Which catalogue: the whole, or the minimal one of the library ablation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Size {
    #[default]
    Whole,
    Minimal,
}

impl Size {
    pub fn parse(s: &str) -> Option<Size> {
        match s {
            "whole" => Some(Size::Whole),
            "minimal" => Some(Size::Minimal),
            _ => None,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Size::Whole => "whole",
            Size::Minimal => "minimal",
        }
    }
}

/// **A catalogue**: its entries, and the library they were run against.
#[derive(Clone, Debug)]
pub struct Catalogue {
    pub size: Size,
    pub entries: Vec<Entry>,
    /// `sha256:` of the library's manifest, `ein --version`'s `stdlib` line.
    pub manifest: Option<String>,
    /// The modules of a directory given beside the library; `None` in the
    /// main arms.
    pub given: Option<Given>,
}

/// **A directory of theory modules given beside the library**: each `.ein`
/// file of it that states no `(query …)` and no `(solve …)`, a problem's file
/// being no module, by its name, the file's stem, in the order of the names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Given {
    /// The directory, as given: where a program that imports a module of it
    /// is loaded from.
    pub dir: PathBuf,
    /// Each module's name and text.
    pub modules: Vec<(String, String)>,
}

impl Given {
    /// **Read a directory's modules.**
    pub fn read(dir: &Path) -> Result<Given, String> {
        let mut modules = Vec::new();
        let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let mut paths: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "ein"))
            .collect();
        paths.sort();
        for p in paths {
            let text = std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?;
            let Some(name) = p.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
                continue;
            };
            let Ok(shapes) = kernel::shapes(&text, &name) else {
                continue;
            };
            let a_problem = shapes
                .iter()
                .any(|f| matches!(f.shape, kernel::Shape::Query { .. } | kernel::Shape::Solve));
            if !a_problem {
                modules.push((name, text));
            }
        }
        if modules.is_empty() {
            return Err(format!("{}: no module", dir.display()));
        }
        Ok(Given {
            dir: dir.to_path_buf(),
            modules,
        })
    }

    /// **The modules as a prompt shows them**: each by its forms, no
    /// comment of its file: the import, its relations as declared, each with
    /// its `:why`, and its rules, each with its parameters and its `:why`.
    pub fn text(&self) -> String {
        let mut s = String::from(
            "\n# Modules given beside the library\n\nEach is imported by its name, as a module of the library \
             is, listing every short name of it the program writes; a rule of no parameter fires once the module \
             is imported. Their relations and rules, as each module declares them:\n",
        );
        for (name, text) in &self.modules {
            s.push_str(&format!("\n## {name}\n\n```ein\n"));
            for f in kernel::shapes(text, name).unwrap_or_default() {
                if matches!(f.shape, kernel::Shape::Relation { .. }) {
                    s.push_str(&f.canonical);
                    s.push('\n');
                }
            }
            s.push_str("```\n\nIts rules:\n\n");
            for r in kernel::rule_docs(text, name).unwrap_or_default() {
                s.push_str(&format!(
                    "- `{}` ({}): {}\n",
                    r.name,
                    r.params
                        .iter()
                        .map(|p| format!("?{p}"))
                        .collect::<Vec<_>>()
                        .join(" "),
                    r.why.as_deref().unwrap_or("")
                ));
            }
        }
        s
    }

    /// **The modules as a step's grammar is offered them**: each module's
    /// declared names, its relations, its rules of a parameter or more, the
    /// relations its rules read that neither the library nor a given module
    /// declares, and the names it writes as values that none declares.
    pub fn grammar(&self) -> Vec<steps::Module> {
        let vocabs: Vec<(String, kernel::Vocabulary)> = self
            .modules
            .iter()
            .filter_map(|(m, t)| kernel::vocabulary(t, m).ok().map(|v| (m.clone(), v)))
            .collect();
        let library = steps::Catalogue::library();
        let mut declared_anywhere: BTreeSet<String> = library
            .modules
            .iter()
            .flat_map(|m| m.symbols.iter().cloned())
            .collect();
        declared_anywhere.extend(
            vocabs
                .iter()
                .flat_map(|(_, v)| v.declared.keys().chain(v.rules.keys()).cloned()),
        );
        // A word of Ein's is no relation a module reads and no value.
        let words = crate::grammar::whole::lexical(crate::grammar::whole::EIN_L)
            .expect("ein.l reads")
            .words;
        vocabs
            .iter()
            .map(|(m, v)| steps::Module {
                module: m.clone(),
                symbols: v.declared.keys().chain(v.rules.keys()).cloned().collect(),
                relations: v.declared.clone(),
                activators: v
                    .rules
                    .iter()
                    .filter(|(_, n)| **n > 0)
                    .map(|(r, &n)| (r.clone(), n))
                    .collect(),
                unlisted: v
                    .read
                    .iter()
                    .filter(|(r, _)| !declared_anywhere.contains(*r) && !words.contains(*r))
                    .map(|(r, c)| (r.clone(), c.clone()))
                    .collect(),
                values: v
                    .names
                    .iter()
                    .filter(|n| {
                        !declared_anywhere.contains(*n)
                            && !v.written.contains_key(*n)
                            && !words.contains(*n)
                    })
                    .cloned()
                    .collect(),
            })
            .collect()
    }
}

/// **One module of the generated part**: each rule of it, as its form says.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleDoc {
    pub module: String,
    pub rules: Vec<RuleDoc>,
}

/// **The generated part**: the library's manifest digest, then every module
/// the kernel resolves, `std.core` among them, each rule with its
/// parameters, its own `:why` and what it concludes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Generated {
    pub manifest: Option<String>,
    pub modules: Vec<ModuleDoc>,
}

/// **Read the generated part** off the library the kernel resolves.
pub fn generated() -> Generated {
    let modules = kernel::library()
        .into_iter()
        .map(|(module, text)| ModuleDoc {
            rules: kernel::rule_docs(&text, &module).unwrap_or_default(),
            module,
        })
        .collect();
    Generated {
        manifest: kernel::stdlib_digest(),
        modules,
    }
}

impl Generated {
    /// A rule of a module, by its name in the module.
    pub fn rule(&self, module: &str, name: &str) -> Option<&RuleDoc> {
        self.modules
            .iter()
            .find(|m| m.module == module)?
            .rules
            .iter()
            .find(|r| r.name == name)
    }

    /// **Whether a rule is a check**: it concludes `(false)`, or states the
    /// engine's word, and derives nothing a later rule reads. Found by its
    /// name as a program writes it (short, or `module.name`).
    pub fn is_check(&self, written: &str) -> bool {
        self.modules.iter().any(|m| {
            m.rules.iter().any(|r| {
                (r.name == written || format!("{}.{}", m.module, r.name) == written)
                    && r.concludes
                        .as_deref()
                        .is_some_and(|c| c == "(false)" || c.starts_with(":ein"))
            })
        })
    }

    /// The text `einaf catalogue --generated` prints: a line a rule.
    pub fn text(&self) -> String {
        let mut s = format!(
            "# The library's rules, generated\n\nmanifest {}\n",
            self.manifest.as_deref().unwrap_or("none")
        );
        for m in &self.modules {
            s.push_str(&format!("\n## {} ({} rules)\n\n", m.module, m.rules.len()));
            for r in &m.rules {
                s.push_str(&format!(
                    "{} {} ({}) — {}\n",
                    r.form,
                    r.name,
                    r.params
                        .iter()
                        .map(|p| format!("?{p}"))
                        .collect::<Vec<_>>()
                        .join(" "),
                    r.why.as_deref().unwrap_or("")
                ));
            }
        }
        s
    }
}

impl Catalogue {
    /// The whole catalogue, or the minimal one.
    pub fn new(size: Size) -> Catalogue {
        Catalogue {
            size,
            entries: ENTRIES
                .iter()
                .filter(|e| size == Size::Whole || e.minimal())
                .copied()
                .collect(),
            manifest: kernel::stdlib_digest(),
            given: None,
        }
    }

    /// **This catalogue with a directory's modules given beside it.**
    pub fn with_modules(mut self, dir: &Path) -> Result<Catalogue, String> {
        self.given = Some(Given::read(dir)?);
        Ok(self)
    }

    /// Where a program's local imports are found: the given directory.
    pub fn base_dir(&self) -> Option<&Path> {
        self.given.as_ref().map(|g| g.dir.as_path())
    }

    /// **The catalogue as a prompt shows it**: the library's digest, how a
    /// theory is selected, then each group's entries, each with its meaning,
    /// its rule's own `:why`, what it gives the hypothesis mechanism, the
    /// kernel's answer, and the entry program.
    pub fn text(&self) -> String {
        let generated = generated();
        let mut s = String::new();
        s.push_str("# The catalogue: the theories of the standard library\n\n");
        s.push_str(&format!(
            "The library is {}. A program selects a theory by importing its module, listing after :symbols every \
             short name of the module the program writes, and writing the facts that select the theory over its own \
             relations, exactly as the entry writes them. A name the program writes and the import does not list is \
             the program's own, and a fact of it selects nothing, in silence. Each entry is a program the kernel ran, \
             with its answer.\n",
            self.manifest.as_deref().unwrap_or("the library")
        ));
        for (module, words) in GROUPS {
            let entries: Vec<&Entry> = self
                .entries
                .iter()
                .filter(|e| e.module == *module)
                .collect();
            if entries.is_empty() {
                continue;
            }
            s.push_str(&format!("\n## {module}: {words}\n"));
            for e in entries {
                s.push_str(&format!(
                    "\n### {}\n\n{}.",
                    e.selects,
                    capitalise(e.meaning)
                ));
                if let Some(rule) = e.rule
                    && let Some(doc) = generated
                        .modules
                        .iter()
                        .filter(|m| m.module == e.module || m.module == "std.algebra")
                        .find_map(|m| m.rules.iter().find(|r| r.name == rule))
                    && let Some(why) = &doc.why
                {
                    s.push_str(&format!(" Its rule's own words: \"{why}\""));
                }
                s.push_str(&format!(
                    "\nWhat it gives the hypothesis mechanism: {}.\n",
                    e.gives
                ));
                match (e.outcome, e.answer) {
                    ("inconsistent", _) => {
                        s.push_str("The kernel's answer: the statements contradict each other.\n")
                    }
                    (_, []) => {}
                    (_, lines) => {
                        s.push_str(&format!("The kernel's answer: {}\n", lines.join(" ")))
                    }
                }
                s.push_str(&format!("\n```ein\n{}```\n", e.shown()));
            }
        }
        if let Some(g) = &self.given {
            s.push_str(&g.text());
        }
        s
    }

    /// **The catalogue a step's grammar is rendered on**: the modules the
    /// entries import, each with the names the entries list of it, the
    /// relations among those names, the facts the entries select with at
    /// their counts, and the names they write as values that the module's
    /// rules read. Narrower than [`steps::Catalogue::library`], which is
    /// every module and every rule.
    pub fn grammar(&self) -> steps::Catalogue {
        let library = steps::Catalogue::library();
        let texts = kernel::library();
        let vocabs: Vec<kernel::Vocabulary> = texts
            .iter()
            .filter_map(|(m, t)| kernel::vocabulary(t, m).ok())
            .collect();
        let library_names: BTreeSet<&str> = vocabs
            .iter()
            .flat_map(|v| v.names.iter().map(String::as_str))
            .collect();
        let library_declared: BTreeSet<&str> = vocabs
            .iter()
            .flat_map(|v| v.declared.keys().chain(v.rules.keys()).map(String::as_str))
            .collect();
        let mut modules: BTreeMap<String, steps::Module> = BTreeMap::new();
        for e in &self.entries {
            let Ok(vocab) = kernel::vocabulary(e.program, e.id) else {
                continue;
            };
            let Ok(forms) = kernel::forms(Source::Text {
                text: e.program,
                name: e.id,
                base_dir: None,
            }) else {
                continue;
            };
            for imp in &vocab.imports {
                let Some(whole) = library.module(&imp.module) else {
                    continue;
                };
                let m = modules
                    .entry(imp.module.clone())
                    .or_insert_with(|| steps::Module {
                        module: imp.module.clone(),
                        ..steps::Module::default()
                    });
                for name in imp.symbols.iter().flatten() {
                    m.symbols.insert(name.clone());
                    if let Some(counts) = whole.relations.get(name) {
                        m.relations
                            .entry(name.clone())
                            .or_default()
                            .extend(counts.iter().copied());
                    }
                    if let Some(&n) = whole.activators.get(name) {
                        m.activators.insert(name.clone(), n);
                    }
                }
            }
            // The facts that select: a generic-theory fact, at its count.
            for row in contract::categories(&forms, "") {
                if row.category != Category::GenericTheory || row.canonical.starts_with("(import") {
                    continue;
                }
                let Ok(v) = kernel::vocabulary(&row.canonical, e.id) else {
                    continue;
                };
                for (head, counts) in &v.written {
                    if !row.canonical.starts_with(&format!("({head}")) {
                        continue;
                    }
                    for whole in &library.modules {
                        if whole.unlisted.contains_key(head) {
                            let m = modules.entry(whole.module.clone()).or_insert_with(|| {
                                steps::Module {
                                    module: whole.module.clone(),
                                    ..steps::Module::default()
                                }
                            });
                            m.unlisted
                                .entry(head.clone())
                                .or_default()
                                .extend(counts.iter().copied());
                        }
                    }
                }
            }
            // The names an entry writes as values that the library's rules
            // read and no module declares: the calculator's orders. Each is
            // given to every module the entry imports.
            let declared: BTreeSet<&str> = vocab.declared.keys().map(String::as_str).collect();
            let values: Vec<&String> = vocab
                .names
                .iter()
                .filter(|n| {
                    library_names.contains(n.as_str())
                        && !library_declared.contains(n.as_str())
                        && !declared.contains(n.as_str())
                        && !vocab.written.contains_key(*n)
                })
                .collect();
            for imp in &vocab.imports {
                if let Some(m) = modules.get_mut(&imp.module) {
                    m.values.extend(values.iter().map(|v| (*v).clone()));
                }
            }
        }
        // A numeral is written with the ten digits, which are names: a
        // program that imports the calculator may write any of them.
        if let Some(m) = modules.get_mut("std.math.decimal") {
            m.values.extend((0..=9).map(|d| d.to_string()));
        }
        let mut out = steps::Catalogue {
            modules: modules.into_values().collect(),
        };
        if let Some(g) = &self.given {
            out.modules.extend(g.grammar());
        }
        out
    }
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}
