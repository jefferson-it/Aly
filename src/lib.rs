#![allow(nonstandard_style)]

/// Aly-lang library crate root.
///
/// Re-exports all public modules for use by integration tests and external consumers.
pub mod apg;
pub mod error;
pub mod math_eval;
pub mod aly;
pub mod compiler;
pub mod runtime;
pub mod lexer;
pub mod native;
pub mod tokens;
pub mod validators;
pub mod schema;
pub mod trait_system;
pub mod plugin;
pub mod module_system;
pub mod concurrency;

pub mod gui;
pub mod vm;
pub mod stdlib;
pub mod tools;
pub mod database;
pub mod render;