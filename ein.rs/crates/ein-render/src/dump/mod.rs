//! The answer as files: what `ein solve --dump-answer DIR`, `--dump-lattice
//! DIR` and `ein oracle --dump DIR` write.
//!
//! The walk's `--dump-states` tree — its three dumpers behind the search's
//! lifecycle hooks, the per-entering files and the lattice snapshot — was
//! retired at M1e2 S1e2.1a.5's switch and deleted with the walk at S1e2.1c.5
//! (2026-11-03). What is left is the region's: the cube
//! ([`region`]), the answer directory ([`region_answer`]), every node a run
//! visited ([`region_lattice`]), and the KB-to-`.ein` text every state file
//! is written in ([`serialise`]).

pub mod json;
pub mod region;
pub mod region_answer;
pub mod region_lattice;
pub mod serialise;

pub use json::Json;
pub use region::{RegionInputs, write_cube_dir};
pub use serialise::kb_to_ein_text;
