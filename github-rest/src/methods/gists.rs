use super::prelude::*;
use crate::{methods::Pagination, model::gists::Gist};
use std::{collections::HashMap, fmt::Display};

user_and_pagination_methods!(
    /// * tags gists
    /// * get `/users/{username}/gists`
    /// * docs <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
    ///
    /// List gists for a user
    /// Lists public gists for the specified user:
    get_user_gists,
    EndPoints::GetUsersusernameGists,
    Vec<Gist>
);

/// * tags gists
/// * delete `/gists/{gist_id}`
/// * docs <https://docs.github.com/rest/reference/gists#delete-a-gist>
///
/// Delete a gist
pub async fn delete_gist<T, A>(client: &T, gist_id: A) -> Result<(), GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .raw_req::<String, String>(EndPoints::DeleteGistsgistId(gist_id.into()), None, None)
        .await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CreateGistBody {
    /// Schema:
    /// ```json
    /// "files": {
    ///     "filename": {
    ///         "content": "file contents"
    ///     }
    /// }
    /// ```
    pub files: HashMap<String, FileContents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub public: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct FileContents {
    pub content: String,
}

impl<T: Display> From<T> for FileContents {
    fn from(val: T) -> Self {
        FileContents {
            content: format!("{val}"),
        }
    }
}

/// * tags gists
/// * post `/gists`
/// * docs <https://docs.github.com/rest/reference/gists#create-a-gist>
///
/// Create a gist
/// Allows you to add a new gist with one or more files.
///
/// **Note:** Don't name your files "gistfile" with a numerical suffix. This is
/// the format of the automatic naming scheme that Gist uses internally.
pub async fn create_gist<T>(client: &T, body: &CreateGistBody) -> Result<Gist, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, Gist>(EndPoints::PostGists(), None, Some(serde_json::to_string(body).unwrap()))
        .await
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PatchGistBody {
    /// Schema:
    /// ```json
    /// "files": {
    ///     "filename": {
    ///         "content": "file contents"
    ///     }
    /// }
    /// ```
    pub files: HashMap<String, FileContents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// * tags gists
/// * patch `/gists/{gist_id}`
/// * docs <https://docs.github.com/rest/reference/gists/#update-a-gist>
///
/// Update a gist
/// Allows you to update or delete a gist file and rename gist files. Files from
/// the previous version of the gist that aren't explicitly changed during an
/// edit are unchanged.
pub async fn patch_gist<T, A>(client: &T, gist_id: A, body: &PatchGistBody) -> Result<Gist, GithubRestError>
where
    T: Requester,
    A: Into<String>,
{
    client
        .req::<String, String, Gist>(
            EndPoints::PatchGistsgistId(gist_id.into()),
            None,
            Some(serde_json::to_string(&body).unwrap()),
        )
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    const GIST_ID: &'static str = "";

    #[tokio::test]
    async fn test_get_user_gists() {
        let requester = DefaultRequester::new_none();
        let res = get_user_gists(&requester, "tricked-dev", None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_create_gist() {
        let requester = DefaultRequester::new(std::env::var("GH_LOGIN").unwrap());

        let mut files = HashMap::new();
        files.insert("1.rs".to_owned(), r#"fn main() { println!("testing")}"#.into());

        let body = CreateGistBody {
            files,
            ..Default::default()
        };

        let res = create_gist(&requester, &body).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn test_delete_gist() {
        let requester = DefaultRequester::new(std::env::var("GH_LOGIN").unwrap());

        delete_gist(&requester, GIST_ID).await.unwrap()
    }

    #[tokio::test]
    async fn test_patch_gist() {
        let requester = DefaultRequester::new(std::env::var("GH_LOGIN").unwrap());

        let body = PatchGistBody {
            description: Some("Something".to_owned()),
            ..Default::default()
        };

        let res = patch_gist(&requester, GIST_ID, &body).await.unwrap();
        dbg!(res);
    }
}
