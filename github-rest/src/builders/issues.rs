use crate::{
    builders::Builder,
    methods::{get_issues, GetIssueBody, IssueState},
    model::issues::Issues,
    GithubRestError, Requester,
};
use async_trait::async_trait;

/// * tags issues
/// * get `/repos/{owner}/{repo}/issues`
/// * docs <https://docs.github.com/rest/reference/issues#list-repository-issues>
///
/// List repository issues
/// List issues in a repository.
///
/// **Note**: GitHub's REST API v3 considers every pull request an issue, but
/// not every issue is a pull request. For this reason, "Issues" endpoints may
/// return both issues and pull requests in the response. You can identify pull
/// requests by the `pull_request` key. Be aware that the `id` of a pull
/// request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
#[derive(Default, Clone)]
pub struct GetIssuesBuilder {
    owner: String,
    repo: String,
    body: GetIssueBody,
}

impl GetIssuesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn milestone<T: Into<String>>(mut self, milestone: T) -> GetIssuesBuilder {
        self.body.milestone = Some(milestone.into());
        self
    }

    pub fn state(mut self, state: IssueState) -> GetIssuesBuilder {
        self.body.state = Some(state);
        self
    }

    pub fn assignee<T: Into<String>>(mut self, assignee: T) -> GetIssuesBuilder {
        self.body.assignee = Some(assignee.into());
        self
    }

    pub fn creator<T: Into<String>>(mut self, creator: T) -> GetIssuesBuilder {
        self.body.creator = Some(creator.into());
        self
    }

    pub fn mentioned<T: Into<String>>(mut self, mentioned: T) -> GetIssuesBuilder {
        self.body.mentioned = Some(mentioned.into());
        self
    }

    pub fn labels<T: Into<String>>(mut self, labels: T) -> GetIssuesBuilder {
        self.body.labels = Some(labels.into());
        self
    }

    pub fn sort<T: Into<String>>(mut self, sort: T) -> GetIssuesBuilder {
        self.body.sort = Some(sort.into());
        self
    }

    pub fn direction<T: Into<String>>(mut self, direction: T) -> GetIssuesBuilder {
        self.body.direction = Some(direction.into());
        self
    }

    pub fn since<T: Into<String>>(mut self, since: T) -> GetIssuesBuilder {
        self.body.since = Some(since.into());
        self
    }

    pub fn per_page(mut self, count: i32) -> GetIssuesBuilder {
        self.body.per_page = Some(count.to_string());
        self
    }

    pub fn page(mut self, page: i32) -> GetIssuesBuilder {
        self.body.page = Some(page.to_string());
        self
    }

    pub fn owner<T: Into<String>>(mut self, user: T) -> GetIssuesBuilder {
        self.owner = user.into();
        self
    }

    pub fn repo<T: Into<String>>(mut self, repo: T) -> GetIssuesBuilder {
        self.repo = repo.into();
        self
    }
}

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
            .per_page(1)
            .page(2)
            .state(IssueState::Open);

        let r = builder.execute(&requester).await.unwrap();
        println!("{:#?}", r)
    }
}
