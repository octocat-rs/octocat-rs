use super::prelude::*;

pub async fn zen<T>(client: &T) -> Result<String, GithubRestError>
where
    T: Requester,
{
    client.raw_req::<String, String>(EndPoints::GetZen(), None, None).await
}

pub async fn api_info<T>(client: &T) -> Result<GetResponse, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, GetResponse>(EndPoints::Get(), None, None)
        .await
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_zen() {
        let reqester = DefaultRequest::new_none();
        let r = zen(&reqester).await.unwrap();
        println!("{}", r)
    }

    #[tokio::test]
    async fn test_api_info() {
        let reqester = DefaultRequest::new_none();
        let r = api_info(&reqester).await.unwrap();
        println!("{:#?}", r)
    }
}
