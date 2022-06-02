use crate::model::{prelude::*, repositories::nested::*, user::SimpleUser};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub notifications_url: String,
    pub owner: SimpleUser,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub forks: i64,
    pub forks_count: i64,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub mirror_url: Option<String>,
    pub open_issues: i64,
    pub open_issues_count: i64,
    pub license: Option<SimpleLicense>,
    pub pushed_at: String,
    pub size: i64,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub svn_url: String,
    pub watchers: i64,
    pub watchers_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub network_count: i64,
    pub subscribers_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub owner_url: String,
    pub url: String,
    pub html_url: String,
    pub columns_url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub body: String,
    pub number: usize,
    pub state: ProjectState,
    pub creator: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCard {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub note: String,
    pub creator: SimpleUser,
    pub created_at: String,
    pub updated_at: String,
    pub archived: bool,
    pub column_url: String,
    pub content_url: String,
    pub project_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectColumn {
    pub url: String,
    pub project_url: String,
    pub cards_url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

/// <https://docs.github.com/en/rest/reference/deployments#get-a-deploy-key>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeployKey {
    pub id: usize,
    pub key: String,
    pub url: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlert {
    pub number: usize,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub html_url: String,
    pub instances_url: String,
    pub state: CodeScanningAlertState,
    pub fixed_at: Option<String>,
    pub dismissed_by: Option<SimpleUser>,
    pub dismissed_at: Option<String>,
    pub rule: CodeScanningAlertRule,
    pub tool: Tool,
}

pub mod nested {
    use crate::model::prelude::*;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SimpleLicense {
        pub key: String,
        pub name: String,
        pub url: Option<String>,
        pub spdx_id: Option<String>,
        pub node_id: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Tool {
        pub name: String,
        pub version: Option<Value>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CodeScanningAlertRule {
        pub id: String,
        pub severity: String,
        pub description: String,
        pub full_description: String,
        pub tags: Vec<String>,
        pub help: String,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub enum ProjectState {
        Open,
        Closed,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub enum CodeScanningAlertState {
        Open,
        Closed,
        Dismissed,
        Fixed,
    }
}
