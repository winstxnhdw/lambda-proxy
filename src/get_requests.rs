use futures::{stream::FuturesUnordered, TryStreamExt};
use reqwest::header::USER_AGENT;
use reqwest::{Client, Error};

async fn get_request(url: &String, client: &Client) -> Result<String, Error> {
    client
        .get(url)
        .header(USER_AGENT, "lambda-proxy")
        .send()
        .await?
        .text()
        .await
}

pub async fn get_requests(endpoints: &[String]) -> Result<Vec<String>, Error> {
    let client = Client::new();

    endpoints
        .iter()
        .map(|url| get_request(url, &client))
        .collect::<FuturesUnordered<_>>()
        .try_collect()
        .await
}
