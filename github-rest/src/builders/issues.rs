use crate::{
    methods::{get_issues, prelude::Issues, GetIssueBody, IssueState},
    GithubRestError, Requester,
};

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
/// requests by the `pull_request` key. Be aware that the `id` of a pull request
/// returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
pub struct GetIssuesBuilder {
    data: (String, String),
    body: GetIssueBody,
}

impl GetIssuesBuilder {
    pub fn new(user: String, repo: String) -> Self {
        GetIssuesBuilder {
            data: (user, repo),
            body: GetIssueBody {
                milestone: None,
                state: None,
                assignee: None,
                creator: None,
                mentioned: None,
                labels: None,
                sort: None,
                direction: None,
                since: None,
                per_page: None,
                page: None,
            },
        }
    }

    pub fn milestone(mut self, milestone: String) -> GetIssuesBuilder {
        self.body.milestone = Some(milestone);
        self
    }

    pub fn state(mut self, state: IssueState) -> GetIssuesBuilder {
        self.body.state = Some(state);
        self
    }

    pub fn assignee(mut self, assignee: String) -> GetIssuesBuilder {
        self.body.assignee = Some(assignee);
        self
    }

    pub fn creator(mut self, creator: String) -> GetIssuesBuilder {
        self.body.creator = Some(creator);
        self
    }

    pub fn mentioned(mut self, mentioned: String) -> GetIssuesBuilder {
        self.body.mentioned = Some(mentioned);
        self
    }

    pub fn labels(mut self, labels: String) -> GetIssuesBuilder {
        self.body.labels = Some(labels);
        self
    }

    pub fn sort(mut self, sort: String) -> GetIssuesBuilder {
        self.body.sort = Some(sort);
        self
    }

    pub fn direction(mut self, direction: String) -> GetIssuesBuilder {
        self.body.direction = Some(direction);
        self
    }

    pub fn since(mut self, since: String) -> GetIssuesBuilder {
        self.body.since = Some(since);
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

    pub async fn execute<T>(self, client: &T) -> Result<Issues, GithubRestError>
    where
        T: Requester,
    {
        get_issues(client, self.data.0, self.data.1, Some(&self.body)).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_issues_builder() {
        let reqester = DefaultRequest::new_none();

        let builder = GetIssuesBuilder::new("microsoft".to_owned(), "vscode".to_owned())
            .per_page(1)
            .page(2)
            .state(IssueState::Open);

        let r = builder.execute(&reqester).await.unwrap();
        println!("{:#?}", r)
    }
}
