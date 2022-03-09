use crate::{
    builders::{builder, builder_nested_setters, builder_nested_string_setters, builder_string_setters, Builder},
    methods::{get_pulls, GetPullsBody},
    model::pull_requests::{PullRequestState, Pulls},
    GithubRestError, Requester,
};
use async_trait::async_trait;

builder!(
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls`
    /// * docs <https://docs.github.com/rest/reference/pulls#list-pull-requests>
    ///
    /// List pull requests
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    GetPullsBuilder {
        owner: String,
        repo: String,
        body: GetPullsBody
    }
);

builder_string_setters!(GetPullsBuilder { owner, repo });
builder_nested_string_setters!(GetPullsBuilder { body { head, base, sort, direction, since, per_page, page } });
builder_nested_setters!(GetPullsBuilder { body { state: PullRequestState } });

#[async_trait]
impl Builder for GetPullsBuilder {
    type Response = Pulls;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        get_pulls(client, self.owner, self.repo, Some(&self.body)).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_get_pulls_builder() {
        let r = GetPullsBuilder::new()
            .owner("microsoft")
            .repo("vscode")
            .per_page(1.to_string())
            .page(1.to_string())
            .state(PullRequestState::Open)
            .execute(&DefaultRequester::new_none())
            .await
            .unwrap();

        dbg!(r);
    }
}
