use crate::{
    builders::{builder, builder_nested_setters, Builder},
    methods::{SearchRepositoriesBody, SearchRepositoriesResponse},
    GithubRestError, Requester,
};
use async_trait::async_trait;
use github_api_octocat::end_points::EndPoints;
use std::{fmt::Display, ops::Range};

builder!(
    /// * tags search
    /// * get `/search/repositories`
    /// * docs <https://docs.github.com/rest/reference/search#search-repositories>
    ///
    /// Search repositories
    /// Find repositories via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for repositories, you can get text match metadata for the **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to search for popular Tetris repositories
    /// written in assembly code, your query might look like this:
    ///
    /// `q=tetris+language:assembly&sort=stars&order=desc`
    ///
    /// This query searches for repositories with the word `tetris` in the name,
    /// the description, or the README. The results are limited to repositories
    /// where the primary language is assembly. The results are sorted by stars
    /// in descending order, so that the most popular repositories appear first
    /// in the search results.
    SearchRepositoriesBuilder {
        query: String,
        body: SearchRepositoriesBody
    }
);

builder_nested_setters!(SearchRepositoriesBuilder {
    body {
        size: Range<usize>,
        followers: Range<usize>,
        forks: Range<usize>,
        stars: Range<usize>,
        topics: Range<usize>,
        help_wanted_issues: Range<usize>,
        good_first_issues: Range<usize>
    }
});

impl SearchRepositoriesBuilder {
    pub fn query<T: Into<String>>(mut self, query: T) -> Self {
        self.query = {
            serde_urlencoded::to_string(
                serde_urlencoded::from_str::<Vec<(String, String)>>(query.into().as_str()).expect("Invalid query!"),
            )
            .unwrap()
        };

        self
    }

    pub fn language<T: Display>(mut self, val: T) -> Self {
        self.query.push_str(format!("&language:{val}").as_str());

        self
    }

    pub fn topic<T: Display>(mut self, val: T) -> Self {
        self.query.push_str(format!("&topic:{val}").as_str());

        self
    }
}

#[async_trait]
impl Builder for SearchRepositoriesBuilder {
    type Response = SearchRepositoriesResponse;

    async fn execute<T>(mut self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        self.query.push_str(self.body.into_query().as_str());

        client
            .req::<_, String, SearchRepositoriesResponse>(
                EndPoints::GetSearchRepositories(),
                Some(&[("q", self.query)]),
                None,
            )
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
        let requester = DefaultRequester::new_none();

        let res = SearchRepositoriesBuilder::new()
            .query("tetris")
            .language("assembly")
            .stars(20..30)
            .execute(&requester)
            .await?;

        dbg!(res);

        let res = SearchRepositoriesBuilder::new()
            .query("doom")
            .language("rust")
            .topic("game")
            .stars(1000..usize::MAX)
            .execute(&requester)
            .await?;

        dbg!(res);

        Ok(())
    }
}
