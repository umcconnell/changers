//! # Change.rs
//!
//! Make Change. In Rust.
//!
//! Quickly calculate monetary change for a desired amount, all while
//! benefitting from Rust's speed and safety.
//!
//! Change.rs provides an easy-to-use CLI (command line interface) to calculate
//! change directly from the command line. Supports ouputing to `stdout` for
//! results and `stderr` for possible errors.
//! 
//! ## Usage
//! 
//! ```bash
//! changers [amount] [optional: coin_file]
//! ```
//! 
//! The change\.rs CLI accepts two arguments to calculate change:
//! 
//! -   `amount`: The monetary amount to make change for
//! -   `coin_file`, optional: An optional coin file (see [examples](examples/)
//!     folder) containing a list of coins seperated by a newline. If no file is
//!     passed, coins are read from stdin.
//!

pub mod cli;
pub mod logic;
