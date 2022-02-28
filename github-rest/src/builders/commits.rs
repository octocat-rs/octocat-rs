use crate::{
    builders::{
        builder, builder_nested_string_setters, builder_nested_string_setters_required, builder_string_setters, Builder,
    },
    methods::{comment_on_commit, CommitCommentBody},
    model::commits::comments::CommitComment,
    GithubRestError, Requester,
};
use async_trait::async_trait;

builder!(
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
    /// * docs <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
    ///
    /// Create a commit comment
    /// Create a comment for a commit using its `:commit_sha`.
    ///
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    CommitCommentBuilder {
        owner: String,
        repo: String,
        sha: String,
        options: CommitCommentBody
    }
);

builder_string_setters!(CommitCommentBuilder { owner, repo, sha });
builder_nested_string_setters!(CommitCommentBuilder {
    options {
        path,
        position,
        line
    }
});

builder_nested_string_setters_required!(CommitCommentBuilder {
    options {
        body
    }
});

#[async_trait]
impl Builder for CommitCommentBuilder {
    type Response = CommitComment;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester,
    {
        comment_on_commit(client, self.owner, self.repo, self.sha, &self.options).await
    }
}

#[cfg(all(feature = "builders", feature = "client"))]
#[cfg(test)]
mod tests {
    use crate::{
        builders::{Builder, CommitCommentBuilder},
        client::DefaultRequest,
    };

    #[tokio::test]
    async fn test_comment_on_commit() {
        let comment = CommitCommentBuilder::new()
            .owner("octocat-rs")
            .repo("github-rest")
            .sha("2eb7eeba66a6adf2168391d0cd6dcac995a34489")
            .body("Losing my mind");

        // You'll need to add your auth to get this to pass
        let a = comment.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }
}
