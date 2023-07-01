use std::env;
use std::collections::HashMap;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserViewer {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    viewer: T
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    data: Data<T>
}



// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let token = match env::var_os("TIBBER_API_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$TIBBER_API_TOKEN is not set")
    };


    let mut body = HashMap::new();
    body.insert("query", "{ viewer { name } }");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.tibber.com/v1-beta/gql")
        .json(&body)
        .header(AUTHORIZATION, "Bearer ".to_owned() + &token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {

            match response.json::<ApiResponse<UserViewer>>().await {
              Ok(parsed) => println!("{:?}", parsed)  ,
                Err(e) => println!("FAILED TO PARSE! {:?}", e)
            };

            println!("SUCCESS!")
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}