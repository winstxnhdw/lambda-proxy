mod get_requests;

use get_requests::get_requests;
use lambda_http::{run, service_fn, Body, Error, Request, RequestPayloadExt, Response};
use serde::Deserialize;
use serde_json::to_string;
use tracing::Level;
use tracing_subscriber::fmt;

#[derive(Deserialize)]
struct EndpointsStruct {
    endpoints: Vec<String>,
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let payload = match event.payload::<EndpointsStruct>() {
        Ok(Some(payload)) => payload,
        Ok(None) => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "text/plain")
                .body("No body found".into())
                .map_err(Box::new)?)
        }
        Err(err) => {
            return Ok(Response::builder()
                .status(500)
                .header("Content-Type", "text/plain")
                .body(format!("Error parsing body: {}", err).into())
                .map_err(Box::new)?)
        }
    };

    let url_responses = get_requests(&payload.endpoints).await?;
    let lambda_response = to_string(&url_responses)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(lambda_response.into())
        .map_err(Box::new)?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use super::handler;
    use lambda_http::request::from_str;
    use serde_json::from_slice;
    use tokio::test;

    #[test]
    async fn handler_test() {
        let input = include_str!("test.json");
        let request = from_str(input).expect("Failed to create request.");

        let response = handler(request)
            .await
            .expect("Failed to handle request.")
            .body()
            .to_owned();

        let deserialised_response =
            from_slice::<Vec<String>>(&response).expect("Failed to deserialise response.");

        let clean_response = deserialised_response
            .iter()
            .map(|response| response.trim())
            .next()
            .unwrap();

        assert_eq!(deserialised_response.len(), 2);
        assert_eq!(
            clean_response,
            "The quick brown fox jumps over the lazy dog."
        )
    }
}
