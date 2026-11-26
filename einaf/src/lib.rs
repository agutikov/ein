//! **EinAf**: iterative autoformalization, an application of Ein.
//!
//! A model writes an Ein program for a problem stated in natural language,
//! and the kernel judges it, in a loop. This crate is EinAf's code; its
//! documentation is [`docs/einaf/`](../../docs/einaf/README.md).
//!
//! What exists so far is [`kernel`], the one module that calls Ein's library,
//! through one door to an answer, the bounded call [`kernel::call`]: a program and the
//! questions asked of it go in, and one [`kernel::Feedback`] a question comes
//! out, with its outcome, one of seven; beside it, [`kernel::forms()`] reads a
//! program's forms and asks it nothing. And [`contract`], the formalization
//! contract's categories: each form of a program in one row, with the step
//! that writes it. Nothing but [`kernel`] names an `ein_*` item.
//!
//! [`model`] is the model's runtime: one trait, the envelope every call
//! returns, the chat template rendered, and replay from a record; the
//! binding to llama.cpp is behind the `llama` feature. [`probe`] is the
//! first measurement made with it: whether the model writes Ein with no
//! grammar, each output read through [`kernel`]. [`grammar`] is the
//! filter's mechanism: the whole grammar of Ein as GBNF, generated from
//! Ein's two grammar files, a grammar a step of the formalization, and
//! EinAf's acceptor and sampler of a grammar.

pub mod contract;
pub mod grammar;
pub mod kernel;
pub mod model;
pub mod probe;
