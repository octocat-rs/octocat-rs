use crate::model::organizations::AddToOrgResponse;

use super::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

pub async fn zen<T>(client: &T) -> Result<String, GithubRestError>
where
    T: Requester,
{
    client.raw_req::<String, String>(EndPoints::GetZen(), None, None).await
}

pub async fn api_info<T>(client: &T) -> Result<Value, GithubRestError>
where
    T: Requester,
{
    client.req::<String, String, Value>(EndPoints::Get(), None, None).await
}

//Role enum used for add to org function determines the function a user has
// within a organization
#[derive(Deserialize, Serialize, EnumString, EnumVariantNames, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    Admin,
    Member,
}

impl Default for Role {
    fn default() -> Role {
        Role::Member
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddToOrgBody {
    role: Role,
}

impl AddToOrgBody {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

/// * tags orgs
/// * put `/orgs/{org}/memberships/{username}`
/// * docs <https://docs.github.com/rest/reference/orgs#set-organization-membership-for-a-user>
///
/// Set organization membership for a user
/// Only authenticated organization owners can add a member to the organization
/// or update the member's role.
///
/// *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
///     
/// * Authenticated users can _update_ a user's membership by passing the `role`
///   parameter. If the authenticated user changes a member's role to `admin`,
///   the affected user will receive an email notifying them that they've been
///   made an organization owner. If the authenticated user changes an owner's
///   role to `member`, no email will be sent.
///
/// **Rate limits**
///
/// To prevent abuse, the authenticated user is limited to 50 organization
/// invitations per 24 hour period. If the organization is more than one month
/// old or on a paid plan, the limit is 500 invitations per 24 hour period.
pub async fn add_to_org<T>(
    client: &T,
    org: impl Into<String>,
    username: impl Into<String>,
    role: Option<Role>,
) -> Result<AddToOrgResponse, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, AddToOrgResponse>(
            EndPoints::PutOrgsorgMembershipsusername(org.into(), username.into()),
            None,
            Some(
                serde_json::to_string(&AddToOrgBody {
                    role: role.unwrap_or_default(),
                })
                .unwrap(),
            ),
        )
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequester;

    use super::*;

    #[tokio::test]
    async fn test_zen() {
        let requester = DefaultRequester::new_none();
        let res = zen(&requester).await.unwrap();
        println!("{res}")
    }

    #[tokio::test]
    async fn test_api_info() {
        let requester = DefaultRequester::new_none();
        let res = api_info(&requester).await.unwrap();
        println!("{res:#?}")
    }
}
