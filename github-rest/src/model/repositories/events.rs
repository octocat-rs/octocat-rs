use crate::{
    methods::util,
    model::{
        commits::comments::CommitComment,
        event_types::{macros::repo_origin, RepoEventInfo},
        issues::milestones::Milestone,
        organizations::SimpleTeam,
        prelude::*,
        repositories::{
            events::nested::{
                BranchProtectionRule, Commit, HeadCommit, MemberChanges, MilestoneChanges, ProjectCardChanges,
                ProjectChanges, ProjectColumnChanges, Pusher,
            },
            CodeScanningAlert, DeployKey, Project, ProjectCard, ProjectColumn, Repository,
        },
        user::SimpleUser,
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
    pub team: SimpleTeam,
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

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#branch_protection_rule>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchProtectionRuleEvent {
    pub action: BranchProtectionRuleAction,
    pub rule: BranchProtectionRule,
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
    pub starred_at: Option<String>,
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
    pub alert: CodeScanningAlert,
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
    pub member: SimpleUser,
    pub changes: Option<MemberChanges>,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
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

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#project_card>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCardEvent {
    pub action: ProjectCardAction,
    pub changes: Option<ProjectCardChanges>,
    pub after_id: usize,
    pub project_card: ProjectCard,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCardAction {
    Created,
    Edited,
    Moved,
    Converted,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#project_column>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectColumnEvent {
    pub action: ProjectColumnAction,
    pub changes: Option<ProjectColumnChanges>,
    pub after_id: usize,
    pub project_column: ProjectColumn,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum ProjectColumnAction {
    Created,
    Edited,
    Moved,
    Deleted,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#package>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageEvent {
    pub action: PackageAction,
    pub package: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
pub enum PackageAction {
    Published,
    Updated,
}

/// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#ping>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingEvent {
    pub zen: String,
    pub hook_id: usize,
    pub hook: Value,
    #[serde(flatten)]
    pub event_info: RepoEventInfo,
}

pub mod nested {
    use crate::model::{prelude::*, pull_requests::events::nested::Change, user::GitUser};

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
        pub author: GitUser,
        pub committer: GitUser,
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
        pub author: GitUser,
        pub committer: GitUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BranchProtectionRule {
        pub id: usize,
        pub repository_id: usize,
        pub name: String,
        pub created_at: String,
        pub updated_at: String,
        pub pull_request_reviews_enforcement_level: MultiLevelConfiguration,
        pub required_approving_review_count: usize,
        pub dismiss_stale_reviews_on_push: bool,
        pub require_code_owner_review: bool,
        pub authorized_dismissal_actors_only: bool,
        pub ignore_approvals_from_contributors: bool,
        pub required_status_checks: Vec<String>,
        pub required_status_checks_enforcement_level: MultiLevelConfiguration,
        pub strict_required_status_checks_policy: bool,
        pub signature_requirement_enforcement_level: String,
        pub linear_history_requirement_enforcement_level: MultiLevelConfiguration,
        pub admin_enforced: bool,
        pub allow_force_pushes_enforcement_level: MultiLevelConfiguration,
        pub allow_deletions_enforcement_level: MultiLevelConfiguration,
        pub merge_queue_enforcement_level: MultiLevelConfiguration,
        pub required_deployments_enforcement_level: MultiLevelConfiguration,
        pub required_conversation_resolution_level: MultiLevelConfiguration,
        pub authorized_actors_only: bool,
        pub authorized_actor_names: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum MultiLevelConfiguration {
        Off,
        NonAdmins,
        Everyone,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MemberChanges {
        pub old_permission: Option<Change>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MilestoneChanges {
        pub title: Option<Change>,
        pub description: Option<Change>,
        pub due_on: Option<Change>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProjectChanges {
        pub name: Option<Change>,
        pub body: Option<Change>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProjectCardChanges {
        pub note: Option<Change>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProjectColumnChanges {
        pub name: Option<Change>,
    }
}

repo_origin!(RepositoryVulnerabilityAlertEvent);
repo_origin!(BranchProtectionRuleEvent);
repo_origin!(SecretScanningAlertEvent);
repo_origin!(CodeScanningAlertEvent);
repo_origin!(RepositoryImportEvent);
repo_origin!(ProjectColumnEvent);
repo_origin!(ProjectCardEvent);
repo_origin!(RepositoryEvent);
repo_origin!(MilestoneEvent);
repo_origin!(DeployKeyEvent);
repo_origin!(TeamAddEvent);
repo_origin!(PackageEvent);
repo_origin!(ProjectEvent);
repo_origin!(PublicEvent);
repo_origin!(MemberEvent);
repo_origin!(WatchEvent);
repo_origin!(PushEvent);
repo_origin!(StarEvent);
repo_origin!(ForkEvent);
repo_origin!(PingEvent);
