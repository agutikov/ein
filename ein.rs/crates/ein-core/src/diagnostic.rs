//! **A diagnostic as a value**: what the parse and the load refuse, and what a
//! load and a run note beside an answer, each with its [`Kind`], its
//! [`Severity`], its [`Place`] where the source has one, and its message,
//! the line's normative text byte for byte (Q-M2.9).
//!
//! **The vocabulary is the reference's.** A kind is a refusal of
//! `docs/ein/user/04-reference/07-diagnostics.md`, named beside the section
//! that states it, or a line of `08-read-out.md` § 10, the lines a question
//! writes beside an answer, named in that table's `kind` column; every kind
//! here is named on one of the two pages and every name there is one of
//! these ([`Kind::ALL`], held to the pages by `tests/diagnostic_kinds.rs`).
//! The kind names the check that makes the line: a line two sections list,
//! such as a goal's refusal by a check a rule's `:match` gets too, takes the
//! kind of the section that states the check.
//!
//! **The message is what is printed.** A finding's message is the line as
//! `ein` prints it, its place in front where the line carries one: the place
//! is the same position as a value, not a second statement of it, and no
//! place is invented where the line has none (Q-M1a.6). How a refusal's
//! findings are joined into what a command prints is the refusal's:
//! `ein_ir::KbLoadError` joins a load's by `; ` behind `kb load error: `,
//! `ein_ir::ParseError` a parse's errors by line breaks. A diagnostic of a
//! run stands alone on its line, [`Diagnostic::line`].

use std::fmt;

/// How a diagnostic stands to the run's answer and its exit code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Severity {
    /// The program is refused, a claim is refuted, or the run is not
    /// answered: the command's exit code says so. And a file the run was
    /// asked to write and could not, whose line says `error:` and whose run
    /// keeps its exit code (`07-diagnostics.md` § 2.4).
    Error,
    /// Something the answer does not claim, or a file the load did not
    /// believe: printed, and no answer and no exit code moves.
    Warning,
    /// What a load or a run noticed beside an answer: printed, and no answer
    /// and no exit code moves.
    Note,
}

impl Severity {
    /// The word, as a machine surface writes it.
    pub fn name(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

macro_rules! kinds {
    ($( $(#[$doc:meta])* $variant:ident = $name:literal, )*) => {
        /// **What a diagnostic is about**: a refusal of
        /// `07-diagnostics.md`, one a section, or a line of `08-read-out.md`
        /// § 10, by the name the page writes beside it.
        ///
        /// Ein grows refusals as it grows, so a match on a kind outside this
        /// crate names a wildcard arm.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[non_exhaustive]
        pub enum Kind {
            $( $(#[$doc])* $variant, )*
        }

        impl Kind {
            /// Every kind, in the pages' order: `07-diagnostics.md`'s
            /// sections, then the lines of `08-read-out.md` § 10.
            pub const ALL: &'static [Kind] = &[$( Kind::$variant, )*];

            /// The kind's name, as the pages write it beside its section or
            /// its line, and as a machine surface writes it.
            pub fn name(self) -> &'static str {
                match self {
                    $( Kind::$variant => $name, )*
                }
            }
        }
    };
}

kinds! {
    // ── 07-diagnostics.md § 1: the parse ──
    /// § 1: the parse's refusal, `unexpected X` or `no token starts with X`,
    /// with its notes and its repairs; the parse errors of § 2.3, § 2.8,
    /// § 2.12, § 2.31 and the parse rows of § 2.1, § 2.17 and § 2.22 too.
    Parse = "parse",
    /// § 1.6: a character that is not ASCII outside a string and a comment.
    NotAscii = "not-ascii",
    /// § 1.7: a text nested past the parser's limit.
    NestingLimit = "nesting-limit",
    // ── 07-diagnostics.md § 2: the rest ──
    /// § 2: a program file that cannot be read, is neither UTF-8 text nor a
    /// `.einb` container, or is a container this build cannot read: its own
    /// refusals, its magic, its version, its digest, a truncated or malformed
    /// section, and a build with no `einb` feature.
    Unreadable = "unreadable",
    /// § 2.1: a query's, a solve's and a claim's refusals.
    Claim = "claim",
    /// § 2.2: what an obligation owes.
    Obligation = "obligation",
    /// § 2.4: an artefact a run was asked to write, and could not.
    Artefact = "artefact",
    /// § 2.5: the store's own limits.
    StoreLimit = "store-limit",
    /// § 2.7: the module every program gets cannot be read.
    StdCore = "std-core",
    /// § 2.9: a rule shape that reads nothing.
    RuleShape = "rule-shape",
    /// § 2.10: a form with no `:why`.
    MissingWhy = "missing-why",
    /// § 2.11: a template reference no reference can reach.
    Template = "template",
    /// § 2.13: a conclusion's variable nothing binds.
    UnboundConclusion = "unbound-conclusion",
    /// § 2.14: `eq` at another arity.
    EqArity = "eq-arity",
    /// § 2.15: a nested `(or …)`, a wide `:match` and a goal's relation
    /// variable, the shapes no plan can be made of.
    NoPlan = "no-plan",
    /// § 2.16: a malformed primitive a macro's expansion writes.
    MalformedPrimitive = "malformed-primitive",
    /// § 2.17: where `open` is written.
    OpenShape = "open-shape",
    /// § 2.18: an imported module that cannot be read as a program is.
    ModuleUnreadable = "module-unreadable",
    /// § 2.20: a `(solve …)` list names what the program has.
    SolveList = "solve-list",
    /// § 2.21: a goal is held to what a rule's `:match` is held to.
    Goal = "goal",
    /// § 2.22: an object is an atom, or a fact a state could hold.
    Object = "object",
    /// § 2.23: `not` of a name.
    NegatedName = "negated-name",
    /// § 2.24: a macro's call is no fact.
    MacroFact = "macro-fact",
    /// § 2.25: a listed name the module does not provide.
    ImportList = "import-list",
    /// § 2.26: a program's hypothesis mechanisms judged together: a
    /// conflict refuses the program, and a redundant declaration is a note.
    Mechanisms = "mechanisms",
    /// § 2.27: an import is by full name, and a short name names one
    /// declaration in a file.
    FullName = "full-name",
    /// § 2.29: a saved file is its program, its modules and one commitment
    /// of H.
    SavedFile = "saved-file",
    /// § 2.30: a rule's word to the engine.
    EngineWord = "engine-word",
    /// § 2.32: a declaration's name, and a rule's clauses.
    Declaration = "declaration",
    /// § 2.33: a macro's call, its count and its depth.
    MacroCall = "macro-call",
    /// § 2.34: an import that does not resolve to one module.
    Import = "import",
    /// § 2.35: a written derivation that loops.
    DerivationCycle = "derivation-cycle",
    // ── 08-read-out.md § 10: the lines a question writes beside an answer ──
    /// The static check flags the program: nothing is claimed of its answer.
    NotMonotone = "not-monotone",
    /// A run a bound stopped: what it established, and what it did not.
    Partial = "partial",
    /// A claim the run could not check.
    ClaimNotChecked = "claim-not-checked",
    /// A claim the run refutes.
    ClaimFailed = "claim-failed",
    /// `-T`: the clock ran out before the question was decided.
    Deadline = "deadline",
    /// A `.einb` container written under other inputs, opened anyway.
    StaleContainer = "stale-container",
}

impl Kind {
    /// The kind a page names `name`, if one does.
    pub fn of_name(name: &str) -> Option<Kind> {
        Kind::ALL.iter().copied().find(|k| k.name() == name)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// **Where a diagnostic is**: the file as the line names it, and the line and
/// the column, 1-based, lines split at `\n` and columns counted in
/// characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Place {
    pub file: String,
    pub line: u32,
    pub col: u32,
}

impl fmt::Display for Place {
    /// `file:line:col`, as a line writes it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

/// **One diagnostic**: a refusal's finding, or a line a load or a run writes
/// beside an answer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    pub kind: Kind,
    pub severity: Severity,
    /// Where the source has a position for it; `None` where the line names
    /// none.
    pub place: Option<Place>,
    /// The line's text as `ein` prints it, its place in front where it
    /// carries one; a run's line without its severity's word
    /// ([`Diagnostic::line`]).
    pub message: String,
}

impl Diagnostic {
    /// A refusal, or a run's error, with no place.
    pub fn error(kind: Kind, message: impl Into<String>) -> Self {
        Diagnostic {
            kind,
            severity: Severity::Error,
            place: None,
            message: message.into(),
        }
    }

    /// A warning, with no place.
    pub fn warning(kind: Kind, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warning,
            ..Diagnostic::error(kind, message)
        }
    }

    /// A note, with no place.
    pub fn note(kind: Kind, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Note,
            ..Diagnostic::error(kind, message)
        }
    }

    /// The same diagnostic, at `place`.
    pub fn at(mut self, place: Option<Place>) -> Self {
        self.place = place;
        self
    }

    /// **The line stderr prints for this diagnostic alone**: a note after
    /// `note: `, the static check's warning after `WARNING: `, another
    /// warning after `warning: `, the clock's abort between `** aborted: `
    /// and ` **`, an artefact's failure after `error: `, and any other error
    /// as its message is. A refusal of the
    /// load is printed as its findings together, behind `kb load error: `,
    /// which is the refusal's line and no one finding's.
    pub fn line(&self) -> String {
        match (self.severity, self.kind) {
            (Severity::Note, _) => format!("note: {}", self.message),
            (Severity::Warning, Kind::NotMonotone) => format!("WARNING: {}", self.message),
            (Severity::Warning, _) => format!("warning: {}", self.message),
            (Severity::Error, Kind::Deadline) => format!("** aborted: {} **", self.message),
            (Severity::Error, Kind::Artefact) => format!("error: {}", self.message),
            (Severity::Error, _) => self.message.clone(),
        }
    }
}

impl fmt::Display for Diagnostic {
    /// The message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_is_one_kind_and_written_as_a_page_writes_it() {
        let mut seen = std::collections::BTreeSet::new();
        for &k in Kind::ALL {
            assert!(seen.insert(k.name()), "{} named twice", k.name());
            assert_eq!(Kind::of_name(k.name()), Some(k));
            assert!(
                k.name()
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}",
                k.name()
            );
        }
        assert_eq!(Kind::of_name("no-such-kind"), None);
    }

    #[test]
    fn a_line_is_its_message_after_its_severitys_word() {
        let n = Diagnostic::note(Kind::Partial, "f.ein: partial — why");
        assert_eq!(n.line(), "note: f.ein: partial — why");
        let w = Diagnostic::warning(Kind::NotMonotone, "f.ein: THIS");
        assert_eq!(w.line(), "WARNING: f.ein: THIS");
        let s = Diagnostic::warning(Kind::StaleContainer, "x.einb was written");
        assert_eq!(s.line(), "warning: x.einb was written");
        let d = Diagnostic::error(Kind::Deadline, "--max-time 0 s reached before x");
        assert_eq!(d.line(), "** aborted: --max-time 0 s reached before x **");
        let a = Diagnostic::error(Kind::Artefact, "--trace t.md: Is a directory (os error 21)");
        assert_eq!(
            a.line(),
            "error: --trace t.md: Is a directory (os error 21)"
        );
        let e = Diagnostic::error(Kind::ClaimFailed, "f.ein: query :expect FAILED — why");
        assert_eq!(e.line(), e.message);
        assert_eq!(e.to_string(), e.message);
        let placed = Diagnostic::error(Kind::Claim, "m").at(Some(Place {
            file: "f.ein".into(),
            line: 2,
            col: 3,
        }));
        assert_eq!(
            placed.place.as_ref().map(|p| p.to_string()).as_deref(),
            Some("f.ein:2:3")
        );
    }
}
