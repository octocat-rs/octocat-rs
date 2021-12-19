//! A GitHub API client written in Rust.
//!
//! Getting started? Take a look at the [examples](https://github.com/proudmuslim-dev/octocat-rs/tree/main/examples) folder in the project repository!

pub mod github;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
