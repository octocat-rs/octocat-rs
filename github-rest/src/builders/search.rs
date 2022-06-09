use crate::{
    builders::{builder, builder_nested_setters, builder_string_setters, Builder},
    methods::SearchRepositoryBody,
    GithubRestError, Requester,
};
use async_trait::async_trait;
use github_api_octocat::end_points::EndPoints;
use serde_json::Value;
use std::ops::Range;

builder!(SearchRepositoriesBuilder {
    query: String,
    body: SearchRepositoryBody
});

builder_string_setters!(SearchRepositoriesBuilder { query });
builder_nested_setters!(SearchRepositoriesBuilder {
    body {
        size: Range<usize>,
        followers: Range<usize>,
        forks: Range<usize>,
        stars: Range<usize>
    }
});

#[async_trait]
impl Builder for SearchRepositoriesBuilder {
    // TODO: Model response
    type Response = Value;

    async fn execute<T>(mut self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        self.query.push_str(self.body.into_query().as_str());

        client
            .req::<_, String, Value>(EndPoints::GetSearchRepositories(), Some(&[("q", self.query)]), None)
            .await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::DefaultRequester;

    #[tokio::test]
    async fn test_search_repositories_builder() -> Result<(), GithubRestError> {
        let res = SearchRepositoriesBuilder::new()
            // TODO: Human readable input
            .query("tetris+language:assembly&sort=stars&order=desc")
            .stars(4..6)
            .execute(&DefaultRequester::new_none())
            .await?;

        dbg!(res);

        Ok(())
    }
}
