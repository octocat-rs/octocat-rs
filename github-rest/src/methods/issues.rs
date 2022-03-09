use crate::model::{
    issues::{Issue, Issues},
    pull_requests::{PullRequestState, Pulls},
};

use super::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CreateIssueBody {
    pub title: String,
    pub body: Option<String>,
    pub assignee: Option<String>,
    pub milestone: Option<String>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
}

//TODO: TEST THIS
/// * tags issues
/// * post `/repos/{owner}/{repo}/issues`
/// * docs <https://docs.github.com/rest/reference/issues#create-an-issue>
///
/// Create an issue
/// Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://help.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
///
/// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
pub async fn create_issue<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    body: &CreateIssueBody,
) -> Result<Issue, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, Issue>(
            EndPoints::PostReposownerrepoIssues(owner.into(), repo.into()),
            None,
            Some(serde_json::to_string(body)?),
        )
        .await
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GetIssuesBody {
    /// If an integer is passed, it should refer to a milestone by its number
    /// field. If the string * is passed, issues with any milestone are
    /// accepted. If the string none is passed, issues without milestones are
    /// returned.
    pub milestone: Option<String>,
    /// Indicates the state of the issues to return. Can be either open, closed,
    /// or all. Default: open
    pub state: Option<IssueState>,
    /// Can be the name of a user. Pass in none for issues with no assigned
    /// user, and * for issues assigned to any user.
    pub assignee: Option<String>,
    /// The user that created the issue.
    pub creator: Option<String>,
    /// A user that's mentioned in the issue.
    pub mentioned: Option<String>,
    /// A list of comma separated label names. Example: bug,ui,@high
    pub labels: Option<String>,
    /// What to sort results by. Can be either created, updated, comments.
    /// Default: created
    pub sort: Option<String>,
    /// One of asc (ascending) or desc (descending).
    /// Default: desc
    pub direction: Option<String>,
    /// Only show notifications updated after the given time. This is a
    /// timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    pub since: Option<String>,
    /// Results per page (max 100)
    /// Default: 30
    pub per_page: Option<String>,
    /// Page number of the results to fetch.
    /// Default: 1
    pub page: Option<String>,
}

/// * docs <https://docs.github.com/en/rest/reference/issues#list-issues-assigned-to-the-authenticated-user--parameters>
///
/// Represents the state of an issue. Possible variants are open, closed, and
/// all.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    Open,
    Closed,
    All,
}

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
pub async fn get_issues<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    options: Option<&GetIssuesBody>,
) -> Result<Issues, GithubRestError>
where
    T: Requester,
{
    client
        .req::<GetIssuesBody, String, Issues>(
            EndPoints::GetReposownerrepoIssues(owner.into(), repo.into()),
            options,
            None,
        )
        .await
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct GetPullsBody {
    /// Either open, closed, or all to filter by state.
    /// Default: open
    pub state: Option<PullRequestState>,
    /// Filter pulls by head user or head organization and branch name in the
    /// format of user:ref-name or organization:ref-name. For example:
    /// github:new-script-format or octocat:test-branch.
    pub head: Option<String>,
    /// Filter pulls by base branch name. Example: gh-pages.
    pub base: Option<String>,
    /// What to sort results by. Can be either created, updated, popularity
    /// (comment count) or long-running (age, filtering by pulls updated in the
    /// last month). Default: created
    pub sort: Option<String>,
    /// One of asc (ascending) or desc (descending).
    /// Default: desc
    pub direction: Option<String>,
    /// Only show notifications updated after the given time. This is a
    /// timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    pub since: Option<String>,
    /// Results per page (max 100)
    /// Default: 30
    pub per_page: Option<String>,
    /// Page number of the results to fetch.
    /// Default: 1
    pub page: Option<String>,
}

/// * tags pulls
/// * get `/repos/{owner}/{repo}/pulls`
/// * docs <https://docs.github.com/rest/reference/pulls#list-pull-requests>
///
/// List pull requests
/// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
pub async fn get_pulls<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    options: Option<&GetPullsBody>,
) -> Result<Pulls, GithubRestError>
where
    T: Requester,
{
    client
        .req::<GetPullsBody, String, Pulls>(
            EndPoints::GetReposownerrepoPulls(owner.into(), repo.into()),
            options,
            None,
        )
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_create_issue() {
        let requester = DefaultRequester::new("TOKEN");

        let body = CreateIssueBody {
            title: "tricked is cool".to_owned(),
            body: Some("This is very true".to_owned()),
            assignee: None,
            milestone: None,
            labels: None,
            assignees: None,
        };

        let r = create_issue(&requester, "Tricked-dev", "octo-computing-machine", &body)
            .await
            .unwrap();

        dbg!(r);
    }

    #[tokio::test]
    async fn test_get_issues() {
        let requester = DefaultRequester::new_none();

        let r = get_issues(&requester, "microsoft", "vscode", None).await.unwrap();
        println!("{:#?}", r)
    }

    #[tokio::test]
    async fn test_get_issues2() {
        let requester = DefaultRequester::new_none();
        let body = GetIssuesBody {
            milestone: None,
            state: None,
            assignee: None,
            creator: None,
            mentioned: None,
            labels: None,
            sort: None,
            since: None,
            direction: None,
            per_page: Some("1".to_owned()),
            page: None,
        };

        let r = get_issues(&requester, "microsoft", "vscode", Some(&body))
            .await
            .unwrap();

        dbg!(r);
    }
}
