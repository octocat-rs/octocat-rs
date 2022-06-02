use crate::model::{prelude::*, releases::nested::*, user::SimpleUser};

/// <https://docs.github.com/en/rest/releases/releases#get-a-release=>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub created_at: String,
    pub published_at: String,
    pub draft: bool,
    pub id: i64,
    pub node_id: String,
    pub author: SimpleUser,
    pub html_url: String,
    pub name: String,
    pub prerelease: bool,
    pub tag_name: String,
    pub target_commitish: String,
    pub assets: Vec<ReleaseAsset>,
    pub url: String,
    pub body: Option<String>,
}

pub mod nested {
    use crate::model::{prelude::*, user::SimpleUser};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ReleaseAsset {
        pub id: i64,
        pub name: String,
        pub content_type: String,
        pub size: i64,
        pub state: ReleaseAssetState,
        pub url: String,
        pub node_id: String,
        pub download_count: i64,
        pub label: Option<String>,
        pub uploader: Option<SimpleUser>,
        pub browser_download_url: String,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ReleaseAssetState {
        Uploaded,
        Open,
    }
}
