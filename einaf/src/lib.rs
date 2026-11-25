//! **EinAf**: iterative autoformalization, an application of Ein.
//!
//! A model writes an Ein program for a problem stated in natural language,
//! and the kernel judges it, in a loop. This crate is EinAf's code; its
//! documentation is [`docs/einaf/`](../../docs/einaf/README.md).
//!
//! What exists so far is [`kernel`], the one module that calls Ein's library,
//! through one door, the bounded call [`kernel::call`]: a program and the
//! questions asked of it go in, and one [`kernel::Feedback`] a question comes
//! out, with its outcome, one of seven. Nothing else in the crate names an
//! `ein_*` item.

pub mod kernel;
