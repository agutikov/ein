//! The `--dump-states` tree — `ein.py`'s `inference/monotonic/` dumpers.
//!
//! Three dumper implementations behind the engine's lifecycle hooks:
//! [`MonotonicDumper`] writes files, [`ProgressDumper`] streams the live
//! `-v` view (and subclasses it, so `-v` and `--dump-states` compose), and
//! [`LatticeDumper`] writes the richer
//! per-commitment tree with its proof summary.
//!
//! Structurally this is the same idea as the
//! [`--events` protocol](../../../../../docs/kernel/inference/events.md) — a
//! chronological log of what the search did — with a different schema and a
//! directory instead of one file. They are emitted from overlapping call
//! sites, so they have to agree about what happened, which is a property this
//! stage can check and no earlier one could.

pub mod json;
pub mod lattice;
pub mod serialise;
pub mod snapshot;
pub mod state;

pub use json::Json;
pub use lattice::LatticeDumper;
pub use serialise::kb_to_ein_text;
pub use snapshot::{LatticeSnapshot, lattice_snapshot};
pub use state::{MonotonicDumper, ProgressDumper};
