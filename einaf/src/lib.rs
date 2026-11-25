//! **EinAf**: iterative autoformalization, an application of Ein.
//!
//! A model writes an Ein program for a problem stated in natural language,
//! and the kernel judges it, in a loop. This crate is EinAf's code; its
//! documentation is [`docs/einaf/`](../../docs/einaf/README.md).
//!
//! What exists so far is [`kernel`], the one module that calls Ein's library:
//! it loads a program, asks it a question about its answer, the region R, and
//! reads off which of the seven outcomes the run is. Nothing else in the
//! crate names an `ein_*` item.

pub mod kernel;
