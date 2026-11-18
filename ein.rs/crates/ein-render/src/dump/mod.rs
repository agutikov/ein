//! The answer as files: what `ein solve --dump-answer DIR`, `--dump-lattice
//! DIR` and `ein oracle --dump DIR` write.
//!
//! Three writers: the cube
//! ([`region`]), the answer directory ([`region_answer`]), and every node a
//! run visited ([`region_lattice`]).

pub mod json;
pub mod region;
pub mod region_answer;
pub mod region_lattice;

pub use json::Json;
pub use region::{RegionInputs, write_cube_dir};
