use crate::{
    methods::{add_to_org, get_user_followers, get_user_following, Pagination, Role},
    model::{organizations::AddToOrgResponse, prelude::*},
    GithubRestError, Requester,
};
use async_trait::async_trait;

/// Embeds [`PublicUser`]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateUser {
    pub collaborators: i64,
    pub disk_usage: i64,
    pub total_private_repos: i64,
    pub owned_private_repos: i64,
    pub private_gists: i64,
    pub two_factor_authentication: i64,
    #[serde(flatten)] // TODO: Document the `shared` fields in the book & come up with a better name for them
    pub shared: PublicUser,
}

#[async_trait]
impl GitHubUser for PrivateUser {
    async fn add_to_org<T, A>(
        &self,
        client: &T,
        org: A,
        role: Option<Role>,
    ) -> Result<AddToOrgResponse, GithubRestError>
    where
        T: Requester,
        A: Into<String> + Send,
    {
        self.shared.add_to_org(client, org, role).await
    }

    async fn get_following<T>(
        &self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        self.shared.get_following(client, followers_per_page, page_number).await
    }

    async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        self.shared.get_followers(client, followers_per_page, page_number).await
    }
}

/// Embeds [`SimpleUser`]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[async_trait]
impl GitHubUser for PublicUser {
    async fn add_to_org<T, A>(
        &self,
        client: &T,
        org: A,
        role: Option<Role>,
    ) -> Result<AddToOrgResponse, GithubRestError>
    where
        T: Requester,
        A: Into<String> + Send,
    {
        self.shared.add_to_org(client, org, role).await
    }

    async fn get_following<T>(
        &self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        self.shared.get_following(client, followers_per_page, page_number).await
    }

    async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        self.shared.get_followers(client, followers_per_page, page_number).await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[async_trait]
impl GitHubUser for SimpleUser {
    async fn add_to_org<T, A>(
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

    async fn get_following<T>(
        &self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        let followers_per_page = Self::get_num_or_default(followers_per_page, 30u8);

        let page_number = Self::get_num_or_default(page_number, 1u8);

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

    async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester,
    {
        let followers_per_page = Self::get_num_or_default(followers_per_page, 30u8);

        let page_number = Self::get_num_or_default(page_number, 1u8);

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

impl SimpleUser {
    fn get_num_or_default(val: Option<u8>, default: u8) -> String {
        let n = val.unwrap_or(default);

        n.to_string()
    }
}
// TODO: Document this further, explain in book, explore possible methods
/// A trait for all GitHub user structs. Intended for [`SimpleUser`],
/// [`PublicUser`], and [`PrivateUser`]
#[async_trait]
pub trait GitHubUser {
    async fn add_to_org<T, A>(
        &self,
        client: &T,
        org: A,
        role: Option<Role>,
    ) -> Result<AddToOrgResponse, GithubRestError>
    where
        T: Requester,
        A: Into<String> + Send;

    /// Get a list of the users that the user in question is following.
    ///
    /// * `followers_per_page` - The number of users to get per page. Default is
    ///   30.
    /// * `page_number` - The page number of the result to return. Default is 1.
    async fn get_following<T>(
        &self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester;

    /// Get a list of the users that are following the user in question.
    ///
    /// * `followers_per_page` - The number of followers to get per page.
    ///   Default is 30.
    /// * `page_number` - The page number of the result to return. Default is 1.
    async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<SimpleUser>, GithubRestError>
    where
        T: Requester;
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}
