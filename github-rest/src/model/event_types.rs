use super::prelude::*;

/// Used to represent all possible values for the `x-github-event` header sent
/// with all webhook payloads.
///
/// See also: <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads>
#[derive(Deserialize, EnumString, EnumVariantNames, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EventTypes {
    // GitHub Apps
    GithubAppAuthorization,
    Installation,
    InstallationRepositories,

    // Repositories
    DeployKey,
    Gollum,
    // Wiki page created/updated
    Member,
    Milestone,
    Public,
    Release,
    Repository,
    RepositoryDispatch,
    RepositoryImport,
    RepositoryVulnerabilityAlert,
    SecretScanningAlert,
    SecurityAdvisory,
    Star,
    Watch,

    // Pulls
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,

    // Commits
    CommitComment,
    Push,
    Status,

    // Issues
    IssueComment,
    Issues,
    Label,

    // Discussions
    Discussion,
    DiscussionComment,

    // Branches/Tags
    BranchProtectionRule,
    Create,
    Delete,
    Fork,

    // CI/Workflows
    CheckRun,
    CheckSuite,
    CodeScanningAlert,
    Deployment,
    DeploymentStatus,
    PageBuild,
    WorkflowDispatch,
    WorkflowJob,
    WorkflowRun,

    // Organizations
    Membership,
    OrgBlock,
    Organization,
    Team,
    TeamAdd,

    // Projects (for both organizations and repositories)
    Project,
    ProjectCard,
    ProjectColumn,

    // GitHub Marketplace
    MarketplacePurchase,

    // Misc
    Meta,
    Package,
    /* <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#package> */
    Ping,
    Sponsorship,
}
