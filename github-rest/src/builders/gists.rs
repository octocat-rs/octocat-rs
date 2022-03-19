use crate::{
    builders::{builder, builder_nested_string_setters, builder_string_setters, Builder},
    methods::{get_user_gists, Pagination},
    model::gists::Gist,
    GithubRestError, Requester,
};
use async_trait::async_trait;

builder!(
    /// * tags gists
    /// * get `/users/{username}/gists`
    /// * docs <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
    ///
    /// List gists for a user
    /// Lists public gists for the specified user:
    GetGistsBuilder {
        owner: String,
        options: Pagination
    }
);

builder_string_setters!(GetGistsBuilder { owner });
builder_nested_string_setters!(GetGistsBuilder { options { per_page, page } });

#[async_trait]
impl Builder for GetGistsBuilder {
    type Response = Vec<Gist>;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        get_user_gists(client, self.owner, Some(&self.options)).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::{Builder, GetGistsBuilder},
        client::DefaultRequester,
    };

    #[tokio::test]
    async fn test_get_gists_builder() {
        let req = GetGistsBuilder::new().owner("proudmuslim-dev");
        let requester = DefaultRequester::new_none();

        let res = req.execute(&requester).await.unwrap();

        dbg!(res);
    }
}
