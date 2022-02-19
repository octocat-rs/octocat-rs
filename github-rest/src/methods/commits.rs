use crate::model::{
    commits::{comments::CommitComment, Commit, Commits},
    reactions::{CommitCommentReactionCreated, Reaction},
};

use super::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCommitBody {
    /// Page number of the results to fetch.
    page: usize,
    /// Results per page (maximum of 100)
    per_page: u8,
}

pub async fn get_commit<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    commit_id: impl Into<String>,
    options: Option<&GetCommitBody>,
) -> Result<Commit, GithubRestError>
where
    T: Requester,
{
    client
        .req::<GetCommitBody, String, Commit>(
            EndPoints::GetReposownerrepoCommitsref(owner.into(), repo.into(), commit_id.into()),
            options,
            None,
        )
        .await
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCommitsBody {
    ///SHA or branch to start listing commits from. Default: the repository’s
    /// default branch (usually master).
    sha: Option<String>,
    ///Only commits containing this file path will be returned.
    path: Option<String>,
    ///GitHub login or email address by which to filter by commit author.
    author: Option<String>,
    ///Only show notifications updated after the given time. This is a
    /// timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    since: Option<String>,
    ///Only commits before this date will be returned. This is a timestamp in
    /// ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    until: Option<String>,
    ///Results per page (max 100)
    ///Default: 30
    per_page: Option<String>,
    ///Page number of the results to fetch.
    ///Default: 1
    page: Option<String>,
}

/// * tags repos
/// * get `/repos/{owner}/{repo}/commits`
/// * docs <https://docs.github.com/rest/reference/repos#list-commits>
///
/// List commits
/// **Signature verification object**
///
/// The response will include a `verification` object that describes the result
/// of verifying the commit's signature. The following fields are included in
/// the `verification` object:
///
/// | Name | Type | Description |
/// | ---- | ---- | ----------- |
/// | `verified` | `boolean` | Indicates whether GitHub considers the signature
/// in this commit to be verified. | | `reason` | `string` | The reason for
/// verified value. Possible values and their meanings are enumerated in table
/// below. | | `signature` | `string` | The signature that was extracted from
/// the commit. | | `payload` | `string` | The value that was signed. |
///
/// These are the possible values for `reason` in the `verification` object:
///
/// | Value | Description |
/// | ----- | ----------- |
/// | `expired_key` | The key that made the signature is expired. |
/// | `not_signing_key` | The "signing" flag is not among the usage flags in the
/// GPG key that made the signature. | | `gpgverify_error` | There was an error
/// communicating with the signature verification service. |
/// | `gpgverify_unavailable` | The signature verification service is currently
/// unavailable. | | `unsigned` | The object does not include a signature. |
/// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
/// | `no_user` | No user was associated with the `committer` email address in
/// the commit. | | `unverified_email` | The `committer` email address in the
/// commit was associated with a user, but the email address is not verified on
/// her/his account. | | `bad_email` | The `committer` email address in the
/// commit is not included in the identities of the PGP key that made the
/// signature. | | `unknown_key` | The key that made the signature has not been
/// registered with any user's account. | | `malformed_signature` | There was an
/// error parsing the signature. | | `invalid` | The signature could not be
/// cryptographically verified using the key whose key-id was found in the
/// signature. | | `valid` | None of the above errors applied, so the signature
/// is considered to be verified. |
pub async fn get_commits<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    options: Option<&GetCommitsBody>,
) -> Result<Commits, GithubRestError>
where
    T: Requester,
{
    client
        .req::<GetCommitsBody, String, Commits>(
            EndPoints::GetReposownerrepoCommits(owner.into(), repo.into()),
            options,
            None,
        )
        .await
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommitCommentBody {
    /// **Required**. The contents of the comment.
    pub body: String,
    /// Relative path of the file to comment on.
    pub path: Option<String>,
    /// Line index in the diff to comment on.
    pub position: Option<String>,
    /// **Deprecated**. Use position parameter instead. Line number in the file
    /// to comment on.
    pub line: Option<String>,
}

/// * tags repos
/// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
/// * docs <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
///
/// Create a commit comment
/// Create a comment for a commit using its `:commit_sha`.
///
/// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
pub async fn comment_on_commit<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    sha: impl Into<String>,
    options: &CommitCommentBody,
) -> Result<CommitComment, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, CommitComment>(
            EndPoints::PostReposownerrepoCommitscommitShaComments(owner.into(), repo.into(), sha.into()),
            None,
            Some(serde_json::to_string(options)?),
        )
        .await
}

/// * tags reactions
/// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
/// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
///
/// Create reaction for a commit comment
/// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
pub async fn react_to_commit_comment<T>(
    client: &T,
    owner: impl Into<String>,
    repo: impl Into<String>,
    comment_id: i64,
    reaction: Reaction,
) -> Result<CommitCommentReactionCreated, GithubRestError>
where
    T: Requester,
{
    // dbg!(serde_json::to_string(&reaction).unwrap());
    let reaction = "{\"content\":stuff}"
        .to_owned()
        .replace("stuff", serde_json::to_string(&reaction).unwrap().as_str());

    client
        .req::<String, String, CommitCommentReactionCreated>(
            EndPoints::PostReposownerrepoCommentscommentIdReactions(owner.into(), repo.into(), comment_id.to_string()),
            None,
            Some(reaction),
        )
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_commits() {
        let reqester = DefaultRequest::new_none();

        let r = get_commits(&reqester, "microsoft", "vscode", None).await.unwrap();
        println!("{:#?}", r)
    }
}
