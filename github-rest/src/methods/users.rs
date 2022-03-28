use crate::model::{organizations::Organization, user::User};

use super::prelude::*;

/// * tags users
/// * get `/user/followers`
/// * docs <https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user>
///
/// List followers of the authenticated user
/// Lists the people following the authenticated user.
pub async fn get_followers<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowers(), params, None)
        .await
}

/// * tags users
/// * get `/user/following`
/// * docs <https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows>
///
/// List the people the authenticated user follows
/// Lists the people who the authenticated user follows.
pub async fn get_following<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowing(), params, None)
        .await
}

/// * tags users
/// * get `/users/{username}/following`
/// * docs <https://docs.github.com/rest/reference/users#list-the-people-a-user-follows>
///
/// List the people a user follows
/// Lists the people who the specified user follows.
pub async fn get_user_following<T, A>(
    client: &T,
    user: A,
    params: Option<&Pagination>,
) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUsersusernameFollowing(user.into()), params, None)
        .await
}

/// * tags packages
/// * get `/users/{username}/packages`
/// * docs <https://docs.github.com/rest/reference/packages#list-packages-for-user>
///
/// List packages for a user
/// Lists all packages in a user's namespace for which the requesting user has
/// access.
///
/// To use this endpoint, you must authenticate using an access token with the
/// `packages:read` scope. If `package_type` is not `container`, your token must
/// also include the `repo` scope.
pub async fn get_user_organizations<T, A>(client: &T, user: A) -> Result<Vec<Organization>, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<String, String, Vec<Organization>>(EndPoints::GetUsersusernameOrgs(user.into()), None, None)
        .await
}

/// * tags users
/// * get `/users/{username}/keys`
/// * docs <https://docs.github.com/rest/reference/users#list-public-keys-for-a-user>
///
/// List public keys for a user
/// Lists the _verified_ public SSH keys for a user. This is accessible by
/// anyone.
pub async fn get_user_keys<T, A>(client: &T, user: A) -> Result<Vec<Key>, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<String, String, Vec<Key>>(EndPoints::GetUsersusernameKeys(user.into()), None, None)
        .await
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Key {
    pub id: usize,
    pub key: String,
}

/// * tags users
/// * get `/users/{username}/followers`
/// * docs <https://docs.github.com/rest/reference/users#list-followers-of-a-user>
///
/// List followers of a user
/// Lists the people following the specified user.
pub async fn get_user_followers<T, A>(
    client: &T,
    user: A,
    params: Option<&Pagination>,
) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUsersusernameFollowers(user.into()), params, None)
        .await
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Pagination {
    /// Results per page (max 100)
    /// Default: 30
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,
    /// Page number of the results to fetch.
    /// Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_get_followers() {
        // Note: this requires auth
        let client = DefaultRequester::new_none();
        let res = get_followers(&client, None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_following() {
        // Note: this requires auth
        let client = DefaultRequester::new_none();
        let res = get_followers(&client, None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_following() {
        let client = DefaultRequester::new_none();
        let res = get_user_following(&client, "proudmuslim-dev", None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_followers() {
        let client = DefaultRequester::new_none();
        let res = get_user_followers(&client, "bors", None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_get_user_organizations() {
        let client = DefaultRequester::new_none();
        let res = get_user_organizations(&client, "proudmuslim-dev").await.unwrap();
        dbg!(res);
    }
    #[tokio::test]
    async fn test_get_user_keys() {
        let client = DefaultRequester::new_none();
        let res = get_user_keys(&client, "proudmuslim-dev").await.unwrap();
        dbg!(res);
    }
}
