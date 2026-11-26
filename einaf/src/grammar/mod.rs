//! **The grammars**: GBNF texts that hold a model's output to Ein, the
//! filter's mechanism ([the grammars' page](../../../docs/einaf/06-grammars.md)).
//!
//! - [`whole`]: **the whole grammar**, generated from `ein.l` and `ein.y`
//!   through grmtools' readers, with the hand-written rules that stand for
//!   what Ein's lexer does beside them; written by `einaf grammar write` to
//!   [`WHOLE_FILE`], with two roots, `root` (what the contract admits) and
//!   `program` (every form `ein.y`'s `Program` takes).
//! - [`steps`]: **a grammar a step** of the formalization, written at
//!   request time from the program's own vocabulary.
//! - [`gbnf`]: GBNF itself: the model a grammar is written from, and the text
//!   read back as llama.cpp's parser reads it, run as its sampler runs it
//!   ([`gbnf::Compiled::accepts`], EinAf's acceptor), and walked to write the
//!   texts it allows ([`gbnf::Compiled::sample`]).
//!
//! The grammars read Ein's two grammar files, not Ein's crates: nothing here
//! names an item of `ein-*`. What a grammar is held to is the parser, which
//! the tests reach through [`crate::kernel::parse`].

pub mod gbnf;
pub mod steps;
pub mod whole;

/// **The rules a sample of the whole grammar favours**: the hand-written
/// name and variable, a name where a position word could stand, and white
/// space among the separators' pieces, so that a sampled text holds names
/// beside the words no name is, and comments without being made of them.
pub const FAVOURED: &[&str] = &["tok-name", "tok-var", "head-name", "space"];

/// Where the whole grammar is written, from the package's root.
pub const WHOLE_FILE: &str = "grammars/ein.gbnf";
/// The digests of the generated files, a line each: `sha256  path`.
pub const MANIFEST_FILE: &str = "grammars/MANIFEST.sha256";

/// **The whole grammar's text**, from the `ein.l` and `ein.y` this package
/// was built with.
pub fn whole_text() -> String {
    whole::whole(whole::EIN_L, whole::EIN_Y).expect("ein.l and ein.y make a grammar")
}

/// **The manifest's text**: each generated file's SHA-256 and its path.
pub fn manifest_text(whole: &str) -> String {
    let digest = crate::model::sha256_hex(whole.as_bytes());
    let hex = digest.strip_prefix("sha256:").unwrap_or(&digest);
    format!("{hex}  ein.gbnf\n")
}

/// **The root of a grammar that begins after the thinking**: the model's
/// closing tag of its thinking, at most two line breaks, and the grammar's
/// root. A lazy grammar is fed the text from its trigger's first group on,
/// so the trigger is the grammar's own first text, and the white space after
/// it is bounded, or greedy decoding can write white space for ever.
pub const AFTER_THINKING: &str = "after-thinking";
/// The closing tag of the model's thinking.
pub const THINK_END: &str = "</think>";
/// The trigger a grammar that begins after the thinking waits for: the
/// generated text, its first group the closing tag.
pub const THINK_TRIGGER: &str = "[\\s\\S]*?(</think>)[\\s\\S]*";

/// The root of a grammar whose program stands in a fenced block.
pub const FENCED: &str = "fenced";

/// **Where a grammar begins, in a model's output.**
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Frame {
    /// After the model's thinking, at its closing tag, where thinking is on.
    pub after_thinking: bool,
    /// Inside a fenced block: three backticks, a word, a line break, the
    /// program, a line break or two, three backticks. The frame a prompt that
    /// asks for one block gets, so that the model's first token is its own.
    pub fence: bool,
}

/// **A grammar as a request carries it**: `text` at `root`, from the first
/// token; inside a fence, with the root [`FENCED`] added; after the
/// thinking, with the root [`AFTER_THINKING`] added and begun at
/// [`THINK_TRIGGER`]. A token is held to it by the chain's choice first
/// ([`crate::model::GrammarCheck::ChosenFirst`]).
pub fn request_grammar(text: &str, root: &str, frame: Frame) -> crate::model::Grammar {
    use crate::model::{Grammar, GrammarCheck, GrammarStart};
    let mut text = text.to_string();
    let mut root = root.to_string();
    if frame.fence {
        text.push_str(&format!(
            "\n{FENCED} ::= \"```\" [a-z]* \"\\n\" {root} \"\\n\" \"\\n\"? \"```\"\n"
        ));
        root = FENCED.to_string();
    }
    if frame.after_thinking {
        text.push_str(&format!(
            "\n{AFTER_THINKING} ::= \"{THINK_END}\" \"\\n\"? \"\\n\"? {root}\n"
        ));
        Grammar {
            text,
            root: AFTER_THINKING.to_string(),
            start: GrammarStart::Patterns {
                patterns: vec![THINK_TRIGGER.to_string()],
            },
            check: GrammarCheck::ChosenFirst,
        }
    } else {
        Grammar {
            text,
            root,
            start: GrammarStart::First,
            check: GrammarCheck::ChosenFirst,
        }
    }
}
