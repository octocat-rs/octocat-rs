use std::sync::Arc;

use async_trait::async_trait;

use github_rest::model::{
    apps::events::{AppAuthorizationEvent, InstallationEvent, InstallationRepositoriesEvent},
    commits::events::{CommitCommentEvent, StatusEvent},
    discussions::events::{DiscussionCommentEvent, DiscussionEvent},
    issues::events::{IssueCommentEvent, IssueEvent},
    misc::events::{DeploymentEvent, DeploymentStatusEvent, LabelEvent, MarketplacePurchaseEvent, SponsorshipEvent},
    organizations::events::{MembershipEvent, OrgBlockEvent, OrganizationEvent, TeamEvent},
    pull_requests::events::{PullRequestEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent},
    releases::events::{CreateEvent, DeleteEvent, ReleaseEvent},
    repositories::{
        events::{
            BranchProtectionRuleEvent, CodeScanningAlertEvent, DeployKeyEvent, ForkEvent, MemberEvent, MetaEvent,
            MilestoneEvent, PackageEvent, PingEvent, ProjectCardEvent, ProjectColumnEvent, ProjectEvent, PublicEvent,
            PushEvent, RepositoryDispatchEvent, RepositoryEvent, RepositoryImportEvent,
            RepositoryVulnerabilityAlertEvent, SecretScanningAlertEvent, StarEvent, TeamAddEvent, WatchEvent,
        },
        security_advisory::events::SecurityAdvisoryEvent,
        wiki::events::GollumEvent,
        workflows::events::{
            CheckRunEvent, CheckSuiteEvent, PageBuildEvent, WorkflowDispatchEvent, WorkflowJobEvent, WorkflowRunEvent,
        },
    },
};

use crate::{client::GitHubClient, github::command::Command, Client};

/// An event handler that is used in all clients. For end users, this is passed
/// to a [`ClientBuilder`] instance when creating the client in your main
/// function.
///
/// [`ClientBuilder`]: crate::github::ClientBuilder
#[async_trait]
#[allow(unused_variables)]
pub trait EventHandler {
    type Message: std::fmt::Debug + Send;
    type GitHubClient: GitHubClient + Send + Sync;

    /// Utility function for setting the port used by the webhook.
    fn listener_port(&self) -> u16 {
        8080
    }

    /// The route at which the listener should listen for payloads from GitHub.
    fn route(&self) -> &'static str {
        "/payload"
    }

    async fn message(&self, message: Self::Message) {
        {}
    }

    /// Someone revokes their authorization of a GitHub App
    async fn app_authorization_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        app_event: AppAuthorizationEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a GitHub App installation
    async fn installation_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        installation_event: InstallationEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to repositories being added to a GitHub App
    /// installation
    async fn installation_repositories_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        installation_repositories_event: InstallationRepositoriesEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Commit pushed to a repository
    async fn commit_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        push_event: PushEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Comment added to a repository commit
    async fn commit_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        commit_comment_event: CommitCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Release created
    async fn release_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        release_event: ReleaseEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository-related activity
    async fn repository_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Organization-related activity
    async fn organization_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        organization_event: OrganizationEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to an organization's team
    async fn team_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        team_event: TeamEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A repository is added to a team
    async fn team_add_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        team_add_event: TeamAddEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to project boards
    async fn project_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        project_event: ProjectEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to project cards
    async fn project_card_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        project_card_event: ProjectCardEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to columns in a project board
    async fn project_column_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        project_column_event: ProjectColumnEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// The webhook this event is configured on was deleted. This event will
    /// only listen for changes to the particular hook the event is installed on
    async fn meta_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        meta_event: MetaEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to GitHub Packages
    async fn package_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        package_event: PackageEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// New webhook created
    async fn ping_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        ping_event: PingEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a sponsorship listing
    async fn sponsorship_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        sponsorship_event: SponsorshipEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a GitHub Marketplace purchase
    async fn marketplace_purchase_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        marketplace_purchase_event: MarketplacePurchaseEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// GitHub App sends a POST request to the "[create a repository dispatch event](https://docs.github.com/en/rest/reference/repos#create-a-repository-dispatch-event)" endpoint
    async fn repository_dispatch_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryDispatchEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a repository being imported to GitHub
    async fn repository_import_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_event: RepositoryImportEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to security vulnerability alerts in a repository
    async fn repository_vulnerability_alert(
        &self,
        github_client: Arc<Self::GitHubClient>,
        repository_vulnerability_alert: RepositoryVulnerabilityAlertEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to secret scanning alerts in a repository
    async fn secret_scanning_alert(
        &self,
        github_client: Arc<Self::GitHubClient>,
        secret_scanning_alert: SecretScanningAlertEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Status of a Git commit changed
    async fn status_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        status_event: StatusEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a label.
    async fn label_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        label_event: LabelEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a discussion
    async fn discussion_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        discussion_event: DiscussionEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a comment in a discussion
    async fn discussion_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        discussion_comment_event: DiscussionCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a branch protection rule
    async fn branch_protection_rule_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        branch_protection_rule_event: BranchProtectionRuleEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Check suite activity has occurred
    async fn check_suite_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        check_suite_event: CheckSuiteEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to code scanning alerts in a repository
    async fn code_scanning_alert_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        code_scanning_alert_event: CodeScanningAlertEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A deployment is created
    async fn deployment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        deployment_event: DeploymentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A deployment is created
    async fn deployment_status_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        deployment_status_event: DeploymentStatusEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Represents an attempted build of a GitHub Pages site (successful or
    /// not)
    async fn page_build_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        page_build_event: PageBuildEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Someone triggers a workflow run on GitHub or sends a POST request to the
    /// "[create a workflow dispatch event](https://docs.github.com/en/rest/reference/actions#create-a-workflow-dispatch-event)" endpoint
    async fn workflow_dispatch_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        workflow_dispatch_event: WorkflowDispatchEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Deploy key added or removed from a repository
    async fn deploy_key_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        deploy_key_event: DeployKeyEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A wiki page is created or updated
    async fn gollum_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        deploy_key_event: GollumEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to team membership
    async fn membership_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        membership_event: MembershipEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to a user being blocked in an organization
    async fn org_block_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        org_block_event: OrgBlockEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Event related to repository collaborators
    async fn member_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        member_event: MemberEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository made public
    async fn public_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        public_event: PublicEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Event related to repository milestones
    async fn milestone_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        milestone_event: MilestoneEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Git branch or tag created
    async fn tag_created(&self, github_client: Arc<Self::GitHubClient>, event: CreateEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Git branch or tag deleted
    async fn tag_deleted(&self, github_client: Arc<Self::GitHubClient>, event: DeleteEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository receives a star
    async fn star_event(&self, github_client: Arc<Self::GitHubClient>, event: StarEvent) -> Command<Self::Message> {
        Command::none()
    }

    // Repository gets a new security advisory
    async fn security_advisory(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: SecurityAdvisoryEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Someone begins watching a repository
    async fn watch_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        watch_event: WatchEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Repository is forked
    async fn repository_forked(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: ForkEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to pull requests
    async fn pull_request_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to pull request reviews
    async fn pull_request_review_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestReviewEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to pull request review comments in the pull request's
    /// unified diff
    async fn pull_request_review_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: PullRequestReviewCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A GitHub Actions workflow run is requested or completed
    async fn workflow_run(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: WorkflowRunEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// A GitHub Actions workflow job has been queued, is in progress, or has
    /// been completed on a repository
    async fn workflow_job(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: WorkflowJobEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }

    /// Check run activity has occurred
    async fn check_run(&self, github_client: Arc<Self::GitHubClient>, event: CheckRunEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to an issue
    async fn issue_event(&self, github_client: Arc<Self::GitHubClient>, event: IssueEvent) -> Command<Self::Message> {
        Command::none()
    }

    /// Activity related to an issue or pull request comment
    async fn issue_comment_event(
        &self,
        github_client: Arc<Self::GitHubClient>,
        event: IssueCommentEvent,
    ) -> Command<Self::Message> {
        Command::none()
    }
}

#[derive(Debug)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    type Message = ();
    type GitHubClient = Client<Self>;

    async fn message(&self, _message: Self::Message) {}
}

impl DefaultEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultEventHandler {
    fn default() -> Self {
        Self::new()
    }
}
