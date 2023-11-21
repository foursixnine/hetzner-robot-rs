use reqwest::blocking;
use reqwest::Url;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use std::env;
use std::time::Duration;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

fn main() {
    
    let res = query();
    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    }

}

fn query () -> Result<Response, reqwest::Error> {
    // let's first create an http client so we can make requests to hetzner's api
    let api_key = env!["HETZNER_API_KEY"];
    let api_url_base = Url::parse("https://api.hetzner.cloud/v1").unwrap();


    let mut headers = HeaderMap::new();
    headers.insert("Auth-API-Token", api_key.parse().unwrap());
    headers.insert("ACCEPT", "application/json".parse().unwrap());
    headers.insert("User-Agent", APP_USER_AGENT.parse().unwrap());

    // use reqwest ClientBuilder to create a client as it allows us to fine tune the client interactions
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers) // set the default headers for all requests
        .timeout(Duration::from_secs(10))      // we can also set the timeout for the client
        .https_only(true)
        .build()?;
    // we need to get the client result, as it should be treated as a blocking operation
    // execute the client call and return the response to the caller
    let res = client.get(api_url_base.join("servers").unwrap()).send();
    res
}

