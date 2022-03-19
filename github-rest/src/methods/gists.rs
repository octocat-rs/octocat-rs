use super::prelude::*;
use crate::{methods::Pagination, model::gists::Gist};

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
        let reqester = DefaultRequester::new_none();
        let r = get_user_gists(&reqester, "tricked-dev", None).await.unwrap();
        dbg!(r);
    }
}
