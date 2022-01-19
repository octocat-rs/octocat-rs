use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EventsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/events{/privacy}")]
    HttpsApiGithubComUsersHubotEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/octocat/events{/privacy}")]
    HttpsApiGithubComUsersOctocatEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/other_user/events{/privacy}")]
    HttpsApiGithubComUsersOtherUserEventsPrivacy,
}

#[derive(Serialize, Deserialize)]
pub enum FollowingUrl {
    #[serde(rename = "https://api.github.com/users/hubot/following{/other_user}")]
    HttpsApiGithubComUsersHubotFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/octocat/following{/other_user}")]
    HttpsApiGithubComUsersOctocatFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/other_user/following{/other_user}")]
    HttpsApiGithubComUsersOtherUserFollowingOtherUser,
}

#[derive(Serialize, Deserialize)]
pub enum GistsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/gists{/gist_id}")]
    HttpsApiGithubComUsersHubotGistsGistId,

    #[serde(rename = "https://api.github.com/users/octocat/gists{/gist_id}")]
    HttpsApiGithubComUsersOctocatGistsGistId,

    #[serde(rename = "https://api.github.com/users/other_user/gists{/gist_id}")]
    HttpsApiGithubComUsersOtherUserGistsGistId,
}

#[derive(Serialize, Deserialize)]
pub enum StarredUrl {
    #[serde(rename = "https://api.github.com/users/hubot/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersHubotStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/octocat/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOctocatStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/other_user/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOtherUserStarredOwnerRepo,
}
