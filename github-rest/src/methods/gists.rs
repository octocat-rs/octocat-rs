use super::prelude::*;
use crate::{methods::Pagination, model::gists::Gist};

/// * tags gists
/// * get `/users/{username}/gists`
/// * docs <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
///
/// List gists for a user
/// Lists public gists for the specified user:
pub async fn get_user_gists<T, A>(
    client: &T,
    user: A,
    params: Option<&Pagination>,
) -> Result<Vec<Gist>, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<Pagination, String, Vec<Gist>>(EndPoints::GetUsersusernameGists(user.into()), params, None)
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_get_gists() {
        let requester = DefaultRequester::new_none();
        let r = get_user_gists(&requester, "tricked-dev", None).await.unwrap();
        dbg!(r);
    }
}
