# Github-rest

A github api wrapper written in rust

```toml
github-rest = { git = "https://github.com/octocat-rs/octocat-rs" }
```

## Simple example

```rs
use github_rest::builders::GetIssuesBuilder;
use github_rest::client::DefaultRequest;

#[tokio:main]
async fn main() {
    let client = DefaultRequest::new_none();

    let mut builder = GetIssuesBuilder::new("microsoft".to_owned(), "vscode".to_owned());
    builder.per_page(1).page(2).state("open".to_owned());
    let issues = builder.execute(&client).await.unwrap();
}

```
