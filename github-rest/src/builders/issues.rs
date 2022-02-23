use crate::{
    builders::{builder, builder_nested_setters, builder_nested_string_setters, builder_string_setters, Builder},
    methods::{get_issues, GetIssueBody, IssueState},
    model::issues::Issues,
    GithubRestError, Requester,
};
use async_trait::async_trait;

builder!(
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues`
    /// * docs <https://docs.github.com/rest/reference/issues#list-repository-issues>
    ///
    /// List repository issues
    /// List issues in a repository.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue,
    /// but not every issue is a pull request. For this reason, "Issues"
    /// endpoints may return both issues and pull requests in the response.
    /// You can identify pull requests by the `pull_request` key. Be aware
    /// that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    GetIssuesBuilder {
        owner: String,
        repo: String,
        body: GetIssueBody
    }
);

builder_string_setters!(GetIssuesBuilder { owner, repo });
builder_nested_setters!(GetIssuesBuilder { body { state: IssueState } });
builder_nested_string_setters!(GetIssuesBuilder {
    body {
        milestone,
        assignee,
        creator,
        mentioned,
        labels,
        sort,
        direction,
        since,
        per_page,
        page
    }
});

#[async_trait]
impl Builder for GetIssuesBuilder {
    type Response = Issues;

    async fn execute<T>(self, client: &T) -> Result<Issues, GithubRestError>
    where
        T: Requester,
    {
        get_issues(client, self.owner, self.repo, Some(&self.body)).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_issues_builder() {
        let requester = DefaultRequest::new_none();

        let builder = GetIssuesBuilder::new()
            .owner("microsoft")
            .repo("vscode")
            .per_page(1.to_string())
            .page(2.to_string())
            .state(IssueState::Open);

        let r = builder.execute(&requester).await.unwrap();
        println!("{:#?}", r)
    }
}
