//! A GitHub API client written in Rust.
//!
//! To get started, take a look at the documentation for the [`GitHub`] struct.
//!
//! [`GitHub`]: crate::github::GitHub

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod github;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
