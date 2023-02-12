mod get_requests;

use get_requests::get_requests;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::Deserialize;
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
                .body("No body found".into())
                .map_err(Box::new)?)
        }
        Err(err) => {
            return Ok(Response::builder()
                .status(400)
                .body(format!("Error parsing body: {}", err).into())
                .map_err(Box::new)?)
        }
    };

    let url_responses = get_requests(&payload.endpoints).await?;
    let lambda_response = serde_json::to_string(&url_responses)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/json")
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
