use crate::{
    builders::{builder, builder_nested_string_setters, builder_string_setters, Builder},
    methods::{get_user_gists, patch_gist, FileContents, Pagination, PatchGistBody},
    model::gists::Gist,
    GithubRestError, Requester,
};
use async_trait::async_trait;
use std::fmt::Display;

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

builder!(
    /// * tags gists
    /// * patch `/gists/{gist_id}`
    /// * docs <https://docs.github.com/rest/reference/gists/#update-a-gist>
    ///
    /// Update a gist
    /// Allows you to update or delete a gist file and rename gist files. Files
    /// from the previous version of the gist that aren't explicitly changed
    /// during an edit are unchanged.
    PatchGistBuilder {
        gist_id: String,
        body: PatchGistBody
    }
);

builder_string_setters!(PatchGistBuilder { gist_id });
builder_nested_string_setters!(PatchGistBuilder { body { description } });

impl PatchGistBuilder {
    pub fn file<A, B>(mut self, name: A, contents: B) -> Self
    where
        A: Into<String>,
        B: Display,
    {
        self.body.files.insert(name.into(), FileContents::from(contents));
        self
    }
}

#[async_trait]
impl Builder for PatchGistBuilder {
    type Response = Gist;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        patch_gist(client, self.gist_id, &self.body).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::{Builder, GetGistsBuilder, PatchGistBuilder},
        client::DefaultRequester,
    };

    const GIST_ID: &'static str = "";

    #[tokio::test]
    async fn test_get_gists_builder() {
        let req = GetGistsBuilder::new().owner("proudmuslim-dev");
        let requester = DefaultRequester::new_none();

        let res = req.execute(&requester).await.unwrap();

        dbg!(res);
    }

    #[tokio::test]
    async fn test_patch_gist_builder() {
        let requester = DefaultRequester::new(std::env::var("GH_LOGIN").unwrap());
        let req = PatchGistBuilder::new().gist_id(GIST_ID).description("Test description");

        let res = req.execute(&requester).await.unwrap();

        dbg!(res);
    }
}
