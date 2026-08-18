//! The oracle event protocol — `conformance/EVENTS.md`, ein.rs side.
//!
//! T2 parity is "the two engines took the same steps", and that needs both
//! implementations to narrate what they did in a comparable format. The schema
//! is fixed (`ein-events/1`) and was designed as a schema rather than as debug
//! output, so any other observer — a trace viewer, a benchmark harness, an
//! embedder — reads the same stream.
//!
//! Three properties the protocol depends on, and how this module gets them:
//!
//! - **Off is free.** The sink is an `Option`; every call site reads
//!   [`Events::on`] before building anything. ein.py's reason is the same and
//!   sharper — an unguarded `events.emit(...)` packs a `dict` whatever the flag
//!   says, and the firing path runs ~234 k times on an exhaustive `zebra2`.
//! - **Emitting cannot change behaviour.** Nothing here touches engine state.
//!   In particular it must never advance the saturator's tiebreaker or consume
//!   an iterator the caller will consume again. A protocol that perturbs the
//!   run it describes is not an oracle.
//! - **No internal ids.** Facts go out as the canonical s-expression, so
//!   nothing in the stream depends on either implementation's interning.
//!
//! The writer flushes per line: a crashed run's prefix is the most useful
//! artefact it can leave, and the differ is built to read one.

use std::io::Write;

use ein_core::{FactId, Symbol, Terms, Value};

pub const SCHEMA: &str = "ein-events/1";

/// `--events-level`. At `normal` a redundant firing is counted but not
/// emitted, which keeps a hand-readable file; **T2 comparisons run at
/// `verbose`**, because a dropped redundant firing is exactly the kind of
/// difference a port introduces.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Normal,
    Verbose,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Normal => "normal",
            Level::Verbose => "verbose",
        }
    }
}

/// The event writer. `None` sink ⇒ recording is off and every emit is one
/// branch.
pub struct Events {
    sink: Option<Box<dyn Write>>,
    seq: u64,
    level: Level,
}

impl Default for Events {
    fn default() -> Self {
        Events::off()
    }
}

impl Events {
    pub fn off() -> Events {
        Events {
            sink: None,
            seq: 0,
            level: Level::Normal,
        }
    }

    /// Start recording, emitting the `run` event the schema requires first —
    /// so a consumer can reject a file it does not understand before reading
    /// further, and so a truncated file still identifies itself.
    pub fn to(sink: Box<dyn Write>, level: Level) -> Events {
        Events::to_with(sink, level, |_| {})
    }

    /// [`Events::to`] with the CLI's own `run` fields appended — `impl`,
    /// `file`, `argv` and the resolved config, which
    /// [`EVENTS.md`](../../../../conformance/EVENTS.md) lists after `version`
    /// and `level`. The engine has no argv, so the caller supplies them.
    pub fn to_with(sink: Box<dyn Write>, level: Level, extra: impl FnOnce(&mut Line)) -> Events {
        let mut e = Events {
            sink: Some(sink),
            seq: 0,
            level,
        };
        e.emit("run", |l| {
            l.str("version", SCHEMA);
            l.str("level", level.as_str());
            extra(l);
        });
        e
    }

    pub fn on(&self) -> bool {
        self.sink.is_some()
    }

    pub fn verbose(&self) -> bool {
        self.level == Level::Verbose
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Write one event. `fields` runs only when recording.
    pub fn emit(&mut self, kind: &str, fields: impl FnOnce(&mut Line)) {
        let Some(sink) = self.sink.as_mut() else {
            return;
        };
        let mut line = Line {
            out: String::with_capacity(96),
        };
        line.out.push('{');
        line.str("e", kind);
        line.num("n", self.seq as i64);
        fields(&mut line);
        line.out.push('}');
        self.seq += 1;
        let _ = writeln!(sink, "{}", line.out);
        let _ = sink.flush();
    }
}

/// One event line under construction. Fields land in call order, which is the
/// order the schema fixes, so a raw `diff` of two logs stays readable.
pub struct Line {
    out: String,
}

impl Line {
    fn sep(&mut self) {
        if !self.out.ends_with('{') {
            self.out.push_str(", ");
        }
    }

    fn key(&mut self, key: &str) {
        self.sep();
        push_json_str(&mut self.out, key);
        self.out.push_str(": ");
    }

    pub fn str(&mut self, key: &str, value: &str) {
        self.key(key);
        push_json_str(&mut self.out, value);
    }

    pub fn num(&mut self, key: &str, value: i64) {
        self.key(key);
        self.out.push_str(&value.to_string());
    }

    pub fn bool(&mut self, key: &str, value: bool) {
        self.key(key);
        self.out.push_str(if value { "true" } else { "false" });
    }

    pub fn strs<'s>(&mut self, key: &str, values: impl IntoIterator<Item = &'s str>) {
        self.key(key);
        self.out.push('[');
        for (i, v) in values.into_iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }
            push_json_str(&mut self.out, v);
        }
        self.out.push(']');
    }

    pub fn owned_strs(&mut self, key: &str, values: impl IntoIterator<Item = String>) {
        self.key(key);
        self.out.push('[');
        for (i, v) in values.into_iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }
            push_json_str(&mut self.out, &v);
        }
        self.out.push(']');
    }

    /// `events.bindings` — ordered `[name, value]` pairs, a list rather than an
    /// object because **binding order is the observable** and a JSON object's
    /// key order is not something a differ should have to trust.
    /// A list of string lists — the `verdict` event's `models`, one sorted
    /// fact list per branch.
    pub fn str_lists(&mut self, key: &str, values: &[Vec<String>]) {
        self.key(key);
        self.out.push('[');
        for (i, inner) in values.iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }
            self.out.push('[');
            for (j, v) in inner.iter().enumerate() {
                if j > 0 {
                    self.out.push_str(", ");
                }
                push_json_str(&mut self.out, v);
            }
            self.out.push(']');
        }
        self.out.push(']');
    }

    /// An object of `key: value` string pairs — the `run` event's `config`.
    pub fn obj_strs(&mut self, key: &str, pairs: &[(&str, String)]) {
        self.key(key);
        self.out.push('{');
        for (i, (k, v)) in pairs.iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }
            push_json_str(&mut self.out, k);
            self.out.push_str(": ");
            self.out.push_str(v);
        }
        self.out.push('}');
    }

    pub fn bindings(&mut self, key: &str, pairs: impl IntoIterator<Item = (String, String)>) {
        self.key(key);
        self.out.push('[');
        for (i, (k, v)) in pairs.into_iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }
            self.out.push('[');
            push_json_str(&mut self.out, &k);
            self.out.push_str(", ");
            push_json_str(&mut self.out, &v);
            self.out.push(']');
        }
        self.out.push(']');
    }
}

/// `json.dumps(s, ensure_ascii=False)` for a string: escape what JSON requires
/// and nothing else, so non-ASCII goes out as UTF-8 rather than `\uXXXX`.
pub fn push_json_str(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// An in-memory sink, shared with the caller so the log can be read back.
///
/// The differential tests want the whole log as a `String`; the CLI wants a
/// file. `Events` takes a `Box<dyn Write>` so it does not have to know which.
#[derive(Clone, Default)]
pub struct Buffer(std::rc::Rc<std::cell::RefCell<Vec<u8>>>);

impl Buffer {
    pub fn new() -> Buffer {
        Buffer::default()
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0.borrow()).into_owned()
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// A fact as its canonical s-expression — `cli._factdump.fact_sexpr`.
///
/// Note the nullary case: `(q)`, with no trailing space. `Terms::compact`
/// spells the same fact `(q )`, because the loader's error messages build it
/// with an unconditional separator; the two renderers are deliberately
/// distinct rather than one approximating the other.
pub fn sexpr(terms: &Terms, id: FactId) -> String {
    let (rel, args) = terms.facts.get(id);
    let inner: Vec<String> = args.iter().map(|a| sexpr_value(terms, *a)).collect();
    if inner.is_empty() {
        format!("({})", terms.sym(rel))
    } else {
        format!("({} {})", terms.sym(rel), inner.join(" "))
    }
}

/// One argument: `str(arg)` for a name or a number, the s-expression for a
/// nested fact.
pub fn sexpr_value(terms: &Terms, v: Value) -> String {
    match v.as_fact() {
        Some(id) => sexpr(terms, id),
        None => terms.display(v),
    }
}

pub fn sexpr_facts(terms: &Terms, ids: &[FactId]) -> Vec<String> {
    ids.iter().map(|&f| sexpr(terms, f)).collect()
}

pub fn binding_pairs(terms: &Terms, pairs: &[(Symbol, Value)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), sexpr_value(terms, *v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_keeps_its_field_order_and_python_spacing() {
        let mut e = Events::to(Box::new(Vec::new()), Level::Verbose);
        e.emit("fire", |l| {
            l.str("rule", "symmetric");
            l.num("priority", 100);
            l.bool("redundant", false);
        });
        // The sink is opaque once boxed, so re-emit into a local buffer.
        let mut line = Line {
            out: String::from("{"),
        };
        line.str("e", "fire");
        line.num("n", 41);
        line.str("rule", "symmetric");
        line.out.push('}');
        assert_eq!(line.out, r#"{"e": "fire", "n": 41, "rule": "symmetric"}"#);
    }

    #[test]
    fn strings_escape_the_way_ensure_ascii_false_does() {
        let mut out = String::new();
        push_json_str(&mut out, "a\"b\\c\nd é");
        assert_eq!(out, "\"a\\\"b\\\\c\\nd é\"");
    }
}
