use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::{Client, Error};

async fn get_request(url: &String, client: &Client) -> Result<String, Error> {
    client.get(url).send().await?.text().await
}

pub async fn get_requests(endpoints: &Vec<String>) -> Result<Vec<String>, Error> {
    let client = Client::new();

    endpoints
        .iter()
        .map(|url| get_request(url, &client))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect()
}
