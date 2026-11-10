//! **The one read of a program's text from a path** — M1f S1f.5.22, moved here
//! from `ein-cli`'s `common::read_program` (S1f.5.5's T1f.5.5.13), which keeps
//! a wrapper that is every command's read.
//!
//! Every command reads the program it is given through [`read_program`], and
//! since S1f.5.22 the import resolver reads each module a file-relative or a
//! checkout's `std.*` import names through it too ([`crate::imports`]), so a
//! module is refused with the line its file would be refused with as the
//! program a command is given, naming its own path. Until then the resolver
//! read a module with `std::fs::read_to_string(…).ok()` behind `is_file()`, so
//! a module that is not UTF-8, one that cannot be read, a directory and a
//! `.einb` container were each `module not found at … (None)` (the close
//! review's finding 2, W74).
//!
//! The bytes are read and told apart by their first eight: a `.einb` container
//! ([`looks_like_einb`]) or UTF-8 text. A file that cannot be read gives the
//! OS's error, and one that is neither gives [`not_utf8_line`]. The convention
//! is `ein-cli`'s: nothing here exits, and a failure is the line a command
//! prints.

use std::fmt;
use std::io::ErrorKind;
use std::path::Path;

/// What a program path holds, by its first bytes.
pub enum Held {
    /// UTF-8 text: a program as written.
    Text(String),
    /// A `.einb` container ([`looks_like_einb`]), which stores its program.
    Container(Vec<u8>),
}

impl Held {
    /// What [`read_program`] read, for the line `ein saturate` prints under
    /// `input:`.
    pub fn size_line(&self) -> String {
        match self {
            Held::Text(t) => format!("{} chars", t.chars().count()),
            Held::Container(b) => format!("{} bytes, a .einb container", b.len()),
        }
    }
}

/// A path [`read_program`] could not read as a program: the line a command
/// prints, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadError {
    /// The line, with the path as it was handed.
    pub line: String,
    /// The OS's reason, where the file could not be read; `None` where it was
    /// read and is neither a container nor UTF-8.
    pub kind: Option<ErrorKind>,
}

impl ReadError {
    /// The path names no file: the one case the import resolver reports in
    /// its own words, `module not found at …`.
    pub fn not_found(&self) -> bool {
        self.kind == Some(ErrorKind::NotFound)
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.line)
    }
}

impl std::error::Error for ReadError {}

/// Read a program from `path`: the one read every command makes of one, and
/// since M1f S1f.5.22 the import resolver of a module.
///
/// A file that cannot be read gives one line naming the error class, the
/// errno text and the quoted path (`FileNotFoundError: [Errno 2] No such file
/// or directory: '<path>'`, and its two siblings below), the spelling
/// [`defined_behaviour.md` §4](../../../../docs/kernel/defined_behaviour.md)
/// states and parks for the user (F23's D-1). A file that is neither a
/// container nor UTF-8 gives [`not_utf8_line`], in Ein's own words since M1f
/// S1f.5.5's T1f.5.5.13.
pub fn read_program(path: &Path) -> Result<Held, ReadError> {
    let bytes = std::fs::read(path).map_err(|e| {
        let shown = ein_core::repr::repr_str(&path.display().to_string());
        let line = match e.kind() {
            ErrorKind::NotFound => {
                format!("FileNotFoundError: [Errno 2] No such file or directory: {shown}")
            }
            ErrorKind::PermissionDenied => {
                format!("PermissionError: [Errno 13] Permission denied: {shown}")
            }
            ErrorKind::IsADirectory => {
                format!("IsADirectoryError: [Errno 21] Is a directory: {shown}")
            }
            _ => format!("OSError: {e}: {shown}"),
        };
        ReadError {
            line,
            kind: Some(e.kind()),
        }
    })?;
    if looks_like_einb(&bytes) {
        return Ok(Held::Container(bytes));
    }
    match String::from_utf8(bytes) {
        Ok(text) => Ok(Held::Text(text)),
        Err(e) => {
            let at = e.utf8_error().valid_up_to();
            Err(ReadError {
                line: not_utf8_line(path, e.as_bytes(), at),
                kind: None,
            })
        }
    }
}

/// A file that is neither UTF-8 text nor a `.einb` container, refused at its
/// first byte that is not UTF-8, with the line and column a parse error
/// gives (lines split at `\n`, columns in characters, both 1-based):
///
/// ```text
/// bad.ein:2:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container
/// ```
///
/// It was `UnicodeDecodeError: 'utf-8' codec can't decode bytes in '<path>'`
/// until M1f S1f.5.5's T1f.5.5.13: Python's spelling, with no position.
pub fn not_utf8_line(path: &Path, bytes: &[u8], at: usize) -> String {
    let before = String::from_utf8_lossy(&bytes[..at]);
    let line = before.matches('\n').count() + 1;
    let col = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
    format!(
        "{}:{line}:{col}: byte 0x{:02x} is not UTF-8: a program is UTF-8 text, or a .einb container",
        path.display(),
        bytes[at]
    )
}

/// `EINB\0` plus three pad bytes — the container's magic, and the **only**
/// sniff a reader of a program makes.
///
/// A second copy of `ein_einb::header::MAGIC`, because `ein-einb` stands
/// above this crate and is an optional dependency of `ein-cli`, whose
/// `--no-default-features` build must still recognise a container in order to
/// refuse it. That is the case
/// [`AR-M1`](../../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium)'s
/// rule covers explicitly — *compared by a test, since a shared crate for
/// eight bytes is worse than the problem* — and the test is
/// `einb_cli::the_two_magic_constants_agree`. It lived in `ein-cli`'s
/// `common` until M1f S1f.5.22 moved the read here.
///
/// It sniffed **five** bytes until M1e S1e.4.2's sibling stage (`EH-L2`), so
/// `EINB\0xyz` was *"a `.einb` container and this build has no `einb`
/// feature"* in the light build and a UTF-8 error in the default one — two
/// shipped feature sets disagreeing on garbage input.
pub const EINB_MAGIC: [u8; 8] = *b"EINB\0\0\0\0";

/// Whether these bytes open a `.einb` container, in **either** build of
/// `ein-cli`.
///
/// The one predicate, so no two readers can answer it differently.
pub fn looks_like_einb(bytes: &[u8]) -> bool {
    bytes.len() >= EINB_MAGIC.len() && bytes[..EINB_MAGIC.len()] == EINB_MAGIC
}

/// The line an imported module that is a `.einb` container is refused with:
/// an import reads a module as Ein text, and a container is opened only as
/// the program a command is given (M1f S1f.5.22, Q-M1f.91).
pub fn container_module_line(path: &Path) -> String {
    format!(
        "{}: a .einb container, and an imported module is Ein text: import the .ein it was saved from",
        path.display()
    )
}
