use crate::{
    methods::{add_to_org, get_user_followers, get_user_following, Pagination, Role},
    model::{organizations::AddToOrgResponse, prelude::*},
    GithubRestError, Requester,
};

/// Embeds [`PublicUser`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateUser {
    pub collaborators: i64,
    pub disk_usage: i64,
    pub total_private_repos: i64,
    pub owned_private_repos: i64,
    pub private_gists: i64,
    pub two_factor_authentication: i64,
    #[serde(flatten)]
    pub shared: PublicUser,
}

as_ref_and_deref!(PrivateUser, PublicUser, shared);

impl AsRef<SimpleUser> for PrivateUser {
    fn as_ref(&self) -> &SimpleUser {
        &self.shared.shared
    }
}

/// Embeds [`SimpleUser`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub received_events_url: String,
    pub bio: Option<String>,
    pub blog: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub followers: i64,
    pub following: i64,
    pub hireable: Option<bool>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub public_gists: i64,
    pub public_repos: i64,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub shared: SimpleUser,
}

as_ref_and_deref!(PublicUser, SimpleUser, shared);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub html_url: String,
    pub repos_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub starred_url: String,
    pub received_events_url: String,
    pub gravatar_id: Option<String>,
    pub id: i64,
    pub node_id: String,
    pub login: String,
    pub site_admin: bool,
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl SimpleUser {
    pub async fn add_to_org<T, A>(
        &self,
        client: &T,
        org: A,
        role: Option<Role>,
    ) -> Result<AddToOrgResponse, GithubRestError>
    where
        T: Requester,
        A: Into<String> + Send,
    {
        add_to_org(client, org, self.login.as_str(), role).await
    }

    pub async fn get_following<T>(
        &self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        let followers_per_page = get_num_or_default(followers_per_page, 30u8);

        let page_number = get_num_or_default(page_number, 1u8);

        get_user_followers(
            client,
            self.login.clone(),
            Some(&Pagination {
                per_page: Some(followers_per_page),
                page: Some(page_number),
            }),
        )
        .await
    }

    pub async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        let followers_per_page = get_num_or_default(followers_per_page, 30u8);

        let page_number = get_num_or_default(page_number, 1u8);

        get_user_following(
            client,
            self.login.clone(),
            Some(&Pagination {
                per_page: Some(followers_per_page),
                page: Some(page_number),
            }),
        )
        .await
    }
}

fn get_num_or_default(val: Option<u8>, default: u8) -> String {
    let n = val.unwrap_or(default);

    n.to_string()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}
