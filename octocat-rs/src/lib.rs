#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::str_to_string)]
#![deny(rust_2018_idioms)]

//! A GitHub API client written in Rust.
//!
//! Getting started? Take a look at the [examples](https://github.com/octocat-rs/octocat-rs/tree/main/examples) folder in the project repository!

pub use github::*;

pub mod github;

pub use github_api as api;
pub use github_rest as rest;

#[cfg(all(feature = "default", feature = "workers"))]
compile_error!("feature \"default\" and feature \"workers\" cannot be enabled at the same time");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
