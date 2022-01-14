use super::{Reactions, Repository, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

pub type Issues = Vec<Issue>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueAction {
    Opened,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueCommentEvent {
    action: IssueCommentAction,
    issue: Issue,
    comment: IssueComment,
    sender: User,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
    pub reactions: Reactions,
    pub performed_via_github_app: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Label {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    description: String,
    color: String,
    label_default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    number: i64,
    state: String,
    title: String,
    description: String,
    creator: User,
    open_issues: i64,
    closed_issues: i64,
    created_at: String,
    updated_at: String,
    closed_at: String,
    due_on: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PullRequest {
    url: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Issue {
    id: i64,
    node_id: String,
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    number: i64,
    state: String,
    title: String,
    body: String,
    user: User,
    labels: Vec<Label>,
    assignee: Option<User>,
    assignees: Vec<User>,
    milestone: Option<Milestone>,
    locked: bool,
    active_lock_reason: Option<String>,
    comments: i64,
    pull_request: Option<PullRequest>,
    closed_at: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
    closed_by: Option<User>,
    author_association: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestedTeam {
    id: i64,
    node_id: String,
    url: String,
    html_url: String,
    name: String,
    slug: String,
    description: String,
    privacy: String,
    permission: String,
    members_url: String,
    repositories_url: String,
    parent: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub enum EventsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/events{/privacy}")]
    HttpsApiGithubComUsersHubotEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/octocat/events{/privacy}")]
    HttpsApiGithubComUsersOctocatEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/other_user/events{/privacy}")]
    HttpsApiGithubComUsersOtherUserEventsPrivacy,
}

#[derive(Serialize, Deserialize)]
pub enum FollowingUrl {
    #[serde(rename = "https://api.github.com/users/hubot/following{/other_user}")]
    HttpsApiGithubComUsersHubotFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/octocat/following{/other_user}")]
    HttpsApiGithubComUsersOctocatFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/other_user/following{/other_user}")]
    HttpsApiGithubComUsersOtherUserFollowingOtherUser,
}

#[derive(Serialize, Deserialize)]
pub enum GistsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/gists{/gist_id}")]
    HttpsApiGithubComUsersHubotGistsGistId,

    #[serde(rename = "https://api.github.com/users/octocat/gists{/gist_id}")]
    HttpsApiGithubComUsersOctocatGistsGistId,

    #[serde(rename = "https://api.github.com/users/other_user/gists{/gist_id}")]
    HttpsApiGithubComUsersOtherUserGistsGistId,
}

#[derive(Serialize, Deserialize)]
pub enum StarredUrl {
    #[serde(rename = "https://api.github.com/users/hubot/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersHubotStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/octocat/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOctocatStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/other_user/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOtherUserStarredOwnerRepo,
}
