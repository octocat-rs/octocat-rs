use super::prelude::*;

use crate::{
    methods::{add_to_org, get_user_followers, get_user_following, Pagination, Role},
    GithubRestError, Requester,
};

use super::organizations::AddToOrgResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

impl User {
    pub async fn add_to_org<T>(
        self,
        client: &T,
        org: impl Into<String>,
        role: Option<Role>,
    ) -> Result<AddToOrgResponse, GithubRestError>
    where
        T: Requester,
    {
        add_to_org(client, org, self.login, role).await
    }

    /// Get a list of the users that the user in question is following.
    ///
    /// * `followers_per_page` - The number of users to get per page. Default is
    ///   30.
    /// * `page_number` - The page number of the result to return. Default is 1.
    pub async fn get_following<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<User>, GithubRestError>
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

    /// Get a list of the users that are following the user in question.
    ///
    /// * `followers_per_page` - The number of followers to get per page.
    ///   Default is 30.
    /// * `page_number` - The page number of the result to return. Default is 1.
    pub async fn get_followers<T>(
        self,
        client: &T,
        followers_per_page: Option<u8>,
        page_number: Option<u8>,
    ) -> Result<Vec<User>, GithubRestError>
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

    fn get_num_or_default(val: Option<u8>, default: u8) -> String {
        let n = val.unwrap_or(default);

        n.to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}
