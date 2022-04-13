use crate::{
    builders::{
        builder, builder_nested_setters, builder_nested_string_setters, builder_nested_string_setters_required,
        builder_string_setters, Builder,
    },
    methods::{create_issue, get_issues, CreateIssueBody, GetIssuesBody, IssueState},
    model::issues::{Issue, Issues},
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
        body: GetIssuesBody
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

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        get_issues(client, self.owner, self.repo, Some(&self.body)).await
    }
}

builder!(
    /// * tags issues
    /// * post `/repos/{owner}/{repo}/issues`
    /// * docs <https://docs.github.com/rest/reference/issues#create-an-issue>
    ///
    /// Create an issue
    /// Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://help.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
    CreateIssueBuilder {
        owner: String,
        repo: String,
        body: CreateIssueBody
    }
);

builder_nested_setters!(CreateIssueBuilder { body { labels: Vec<String>, assignees: Vec<String> } });
builder_nested_string_setters!(CreateIssueBuilder { body { body, assignee, milestone } });
builder_nested_string_setters_required!(CreateIssueBuilder { body { title } });
builder_string_setters!(CreateIssueBuilder { owner, repo });

#[async_trait]
impl Builder for CreateIssueBuilder {
    type Response = Issue;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        create_issue(client, self.owner, self.repo, &self.body).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::methods::util;

    use super::*;

    #[tokio::test]
    async fn test_get_issues_builder() {
        let res = GetIssuesBuilder::new()
            .owner("microsoft")
            .repo("vscode")
            .per_page(1.to_string())
            .page(2.to_string())
            .state(IssueState::Open)
            .execute(&util::github_auth())
            .await
            .unwrap();

        dbg!(res);
    }

    #[tokio::test]
    async fn test_create_issue_builder() {
        let res = CreateIssueBuilder::new()
            .owner("octocat-rs")
            .repo("octocat-rs")
            .title("Title")
            .body("body")
            .execute(&util::github_auth())
            .await
            .unwrap();

        dbg!(res);
    }
}
