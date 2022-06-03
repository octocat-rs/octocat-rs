use crate::model::{organizations::SimpleOrganization, prelude::*, repositories::Repository, user::SimpleUser};

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

/// Used to represent the base fields provided by events originating from
/// repositories.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RepoEventInfo {
    pub repository: Repository,
    pub organization: Option<SimpleOrganization>,
    pub installation: Option<Value>,
    pub sender: SimpleUser,
}

/// Used to represent the base fields provided by events originating from
/// organizations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OrgEventInfo {
    pub organization: SimpleOrganization,
    pub installation: Option<Value>,
    pub sender: SimpleUser,
}

/// The base trait used to represent different types of events. This will
/// eventually have some subtraits with convenience methods.
///
/// **All** event types implement this.
pub trait Event<'de>: Serialize + Deserialize<'de> {
    type Origin: Serialize + Deserialize<'de>;

    fn origin(&self) -> Option<Self::Origin> {
        None
    }
}

pub(crate) mod macros {
    macro_rules! repo_origin {
        ($ev:ident) => {
            impl crate::model::event_types::Event<'_> for $ev {
                type Origin = crate::model::repositories::Repository;

                fn origin(&self) -> Option<Self::Origin> {
                    Some(self.event_info.repository.clone())
                }
            }
        };
    }

    macro_rules! org_origin {
        ($ev:ident) => {
            impl crate::model::event_types::Event<'_> for $ev {
                type Origin = crate::model::organizations::Organization;
            }
        };
    }

    pub(crate) use org_origin;
    pub(crate) use repo_origin;
}
