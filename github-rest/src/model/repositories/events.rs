use crate::{
    methods::util,
    model::{
        commits::comments::CommitComment,
        event_types::{macros::repo_origin, RepoEventInfo},
        issues::milestones::Milestone,
        organizations::Team,
        prelude::*,
        pull_requests::events::nested::Change,
        repositories::{
            events::nested::{Commit, HeadCommit, Pusher},
            DeployKey, Project, Repository,
        },
        user::User,
    },
    GithubRestError, Requester,
};

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#repository>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryEvent {
    pub action: RepositoryAction,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAction {
    Created,
    Deleted,
    Archived,
    Unarchived,
    Edited,
    Renamed,
    Transferred,
    Publicized,
    Privatized,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#team_add>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamAddEvent {
    pub team: Team,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#repository_dispatch>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryDispatchEvent {
    pub action: String,
    #[serde(flatten)]
    pub event_info: Option<RepoEventInfo>,
    #[serde(flatten)]
    pub payload: Value,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#repository_dispatch>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryImportEvent {
    pub action: RepositoryImportAction,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryImportAction {
    Success,
    Cancelled,
    Failure,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#repository_vulnerability_alert>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryVulnerabilityAlertEvent {
    pub action: RepositoryVulnerabilityAlertAction,
    pub alert: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryVulnerabilityAlertAction {
    Create,
    Dismiss,
    Resolve,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#push>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub before: String,
    pub after: String,
    pub pusher: Pusher,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Value,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: Option<HeadCommit>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

impl PushEvent {
    /// Adds a comment to the commit that triggered the event.
    ///
    /// See also: <https://docs.github.com/en/rest/reference/commits#create-a-commit-comment>
    pub async fn add_comment_to_commit(
        &self,
        client: &impl Requester,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<CommitComment, GithubRestError> {
        let hc = self.head_commit.as_ref().unwrap();

        util::helper_for_helper_for_helper(client, hc.url.clone(), hc.id.clone(), body, path, position).await
    }
}

pub mod nested {
    use crate::model::{prelude::*, user::SimpleUser};

    #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
    pub struct Pusher {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
    pub struct Commit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }

    #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
    pub struct HeadCommit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#branch_protection_rule>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchProtectionRuleEvent {
    pub action: BranchProtectionRuleAction,
    // TODO: Create the relevant struct
    pub rule: Value,
    pub changes: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleAction {
    Created,
    Edited,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#star>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarEvent {
    pub action: StarAction,
    pub starred_at: String,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#watch>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchEvent {
    pub action: WatchAction,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum WatchAction {
    Started,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#secret_scanning_alert>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlertEvent {
    pub action: SecretScanningAlertAction,
    pub alert: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertAction {
    Created,
    Resolved,
    Reopened,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#code_scanning_alert>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertEvent {
    pub action: CodeScanningAlertAction,
    // TODO: Create the relevant struct.
    pub alert: Value,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub commit_oid: String,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningAlertAction {
    Created,
    ReopenedByUser,
    ClosedByUser,
    Fixed,
    AppearedInBranch,
    Reopened,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#fork>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForkEvent {
    pub forkee: Repository,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#public>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicEvent {
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#milestone>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneEvent {
    pub action: MilestoneAction,
    pub milestone: Milestone,
    pub changes: Option<MilestoneChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneChanges {
    pub title: Option<Change>,
    pub description: Option<Change>,
    pub due_on: Option<Change>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneAction {
    Created,
    Closed,
    /// A closed milestone is re-opened
    Opened,
    Edited,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#deploy_key>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeployKeyEvent {
    pub action: DeployKeyAction,
    pub key: DeployKey,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum DeployKeyAction {
    Created,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#member>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberEvent {
    pub action: MemberAction,
    pub member: User,
    pub changes: Option<MemberChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberChanges {
    pub old_permission: Option<Change>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum MemberAction {
    Added,
    Removed,
    Edited,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#project>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectEvent {
    pub action: ProjectAction,
    pub project: Project,
    pub changes: Option<ProjectChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum ProjectAction {
    Created,
    Edited,
    Closed,
    Reopened,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectChanges {
    pub name: Option<Change>,
    pub body: Option<Change>,
}

repo_origin!(RepositoryVulnerabilityAlertEvent);
repo_origin!(BranchProtectionRuleEvent);
repo_origin!(SecretScanningAlertEvent);
repo_origin!(RepositoryDispatchEvent);
repo_origin!(CodeScanningAlertEvent);
repo_origin!(RepositoryImportEvent);
repo_origin!(RepositoryEvent);
repo_origin!(MilestoneEvent);
repo_origin!(DeployKeyEvent);
repo_origin!(TeamAddEvent);
repo_origin!(ProjectEvent);
repo_origin!(PublicEvent);
repo_origin!(MemberEvent);
repo_origin!(WatchEvent);
repo_origin!(PushEvent);
repo_origin!(StarEvent);
repo_origin!(ForkEvent);
