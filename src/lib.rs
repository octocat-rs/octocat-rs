//! A GitHub API client written in Rust.
//!
//! To get started, take a look at the documentation for the [`GitHub`] trait.
//!
//! [`GitHub`]: crate::github::GitHubApplication

pub mod github;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
