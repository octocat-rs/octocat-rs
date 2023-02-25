use crate::{
    methods::Role,
    model::{prelude::*, user::SimpleUser},
};

/// <https://docs.github.com/en/rest/orgs/orgs#get-an-organization=>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub html_url: String,
    pub has_organization_projects: bool,
    pub has_repository_projects: bool,
    pub public_repos: usize,
    pub public_gists: usize,
    pub followers: usize,
    pub following: usize,
    #[serde(rename = "type")]
    pub type_field: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub shared: SimpleOrganization,
}

as_ref_and_deref!(Organization, SimpleOrganization, shared);

/// <https://docs.github.com/en/rest/orgs/orgs#list-organizations=>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleOrganization {
    pub login: String,
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: Option<String>,
}

/// <https://docs.github.com/en/rest/teams/teams#get-a-team-by-name=>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub created_at: String,
    pub updated_at: String,
    pub members_count: usize,
    pub repos_count: usize,
    pub organization: Organization,
    #[serde(flatten)]
    pub shared: SimpleTeam,
}

as_ref_and_deref!(Team, SimpleTeam, shared);

/// <https://docs.github.com/en/rest/teams/teams#list-teams=>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleTeam {
    pub id: usize,
    pub node_id: String,
    pub url: String,
    pub members_url: String,
    pub name: String,
    pub description: Option<String>,
    pub permission: String,
    pub html_url: String,
    pub repositories_url: String,
    pub slug: String,
    pub parent: Option<nested::ParentTeam>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AddToOrgResponse {
    pub url: String,
    pub state: String,
    pub role: Role,
    pub organization_url: String,
    pub organization: SimpleOrganization,
    pub user: SimpleUser,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ParentTeam {
        pub id: usize,
        pub node_id: String,
        pub url: String,
        pub members_url: String,
        pub name: String,
        pub description: Option<String>,
        pub permission: String,
        pub html_url: String,
        pub repositories_url: String,
        pub slug: String,
    }
}
