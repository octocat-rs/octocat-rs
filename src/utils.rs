use cfg_if::cfg_if;
use serde::Deserialize;
pub use strum::{EnumString, EnumVariantNames};

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[derive(Deserialize, Debug)]
pub struct ExampleBody {
    pub text: String,
}

#[derive(Deserialize, EnumString, EnumVariantNames, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EventTypes {
    // GitHub Apps
    GithubAppAuthorization,
    Installation,
    InstallationRepositories,

    // Wiki page created/updated
    Gollum,

    // Repositories
    DeployKey,
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
