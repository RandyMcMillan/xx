#[macro_use]
extern crate log;

pub use error::{XXError, XXResult};

pub mod context;
pub mod error;
pub mod file;
pub mod git;
pub mod process;
mod regex;

#[cfg(test)]
pub mod test;
