//! The answer as files: what `ein solve --dump-answer DIR`, `--dump-lattice
//! DIR` and `ein oracle --dump DIR` write.
//!
//! Four writers: the cube
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
