//! **Replay**: the runtime that answers from a record, and needs no model.
//!
//! A record is JSON lines, each a call as [`Recorded`] keeps it, at the top
//! of the line or under its `recorded` key (a line of the probe's record
//! carries the call beside the kernel's replies), or as a committed record
//! keeps it, [`Slim`], its request's digest with no request body. A request
//! is answered by the call whose digest is the request's, with the
//! generation or the failure that call returned; a request no call answers
//! is the failure [`Failure::NotRecorded`]. So a run replayed asks the model
//! nothing and returns, field for field, what the run that was recorded
//! returned, a slim call's generated tokens their digest.

use std::collections::HashMap;
use std::path::Path;

use super::{Failure, Generation, Recorded, Request, Runtime, Slim};

/// The calls of one or more records, by their requests' digests.
#[derive(Clone, Debug, Default)]
pub struct Replay {
    calls: HashMap<String, Result<Generation, Failure>>,
}

impl Replay {
    /// The calls of a record's text, JSON lines. A line that holds no call
    /// is skipped; a line that is no JSON is an error naming it.
    pub fn from_jsonl(text: &str) -> Result<Self, String> {
        let mut calls = HashMap::new();
        for (i, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let value: serde_json::Value =
                serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
            let call = match value.get("recorded") {
                Some(inner) => inner.clone(),
                None if value.get("request").is_some() => value,
                None if value.get("digest").is_some()
                    && (value.get("generation").is_some() || value.get("failure").is_some()) =>
                {
                    let slim: Slim = serde_json::from_value(value)
                        .map_err(|e| format!("line {}: {e}", i + 1))?;
                    calls.insert(slim.digest.clone(), slim.result());
                    continue;
                }
                None => continue,
            };
            let recorded: Recorded =
                serde_json::from_value(call).map_err(|e| format!("line {}: {e}", i + 1))?;
            calls.insert(recorded.digest.clone(), recorded.result());
        }
        Ok(Replay { calls })
    }

    /// The calls of the record at `path`; a file that ends in `.gz` is not
    /// read, its caller unpacks it.
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::from_jsonl(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// How many calls it holds.
    pub fn len(&self) -> usize {
        self.calls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.calls.is_empty()
    }
}

impl Runtime for Replay {
    fn generate(&mut self, request: &Request) -> Result<Generation, Failure> {
        let digest = request.digest();
        match self.calls.get(&digest) {
            Some(result) => result.clone(),
            None => Err(Failure::NotRecorded { digest }),
        }
    }
}
