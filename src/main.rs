use std::env;
use std::collections::HashMap;
use reqwest;
use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Price {
    total: f32,
    energy: f32,
    startsAt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceInfo {
    current: Price,
    today: Vec<Price>,
    tomorrow: Vec<Price>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Subscription {
    priceInfo: PriceInfo,
}


#[derive(Serialize, Deserialize, Debug)]
struct Home {
    currentSubscription: Subscription,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceViewer {
    homes: Vec<Home>,
}


#[derive(Serialize, Deserialize, Debug)]
struct UserViewer {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    viewer: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    data: Data<T>,
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let token = match env::var_os("TIBBER_API_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$TIBBER_API_TOKEN is not set")
    };

    //fetch_user(&token).await;
    let priceResponse = fetch_prices(&token).await;

    if priceResponse.is_some() {
        let unwrapped = priceResponse.unwrap();
        let price = unwrapped.data.viewer.homes.get(0);
        if price.is_some() {
            let price_info = &price.unwrap().currentSubscription.priceInfo;

            println!("Current price {}", &price_info.current.total);

            for price in &price_info.today {
                println!("{} : {}", price.startsAt, price.total)
            }
        }
    }
}


async fn fetch_prices(token: &String) -> Option<ApiResponse<PriceViewer>> {
    let q = "{
  viewer {
    homes {
      currentSubscription{
        priceInfo{
          current{
            total
            energy
            tax
            startsAt
          }
          today {
            total
            energy
            tax
            startsAt
          }
          tomorrow {
            total
            energy
            tax
            startsAt
          }
        }
      }
    }
  }
}
";

    let response = query(&token, q).await;
    return match response.status() {
        reqwest::StatusCode::OK => {
            let response_text = response.text().await.unwrap();

            match serde_json::from_str::<ApiResponse<PriceViewer>>(&response_text) {
                Ok(parsed) => {
                    //println!("{:?}", parsed);
                    Some(parsed)
                }
                Err(e) => {
                    println!("FAILED TO PARSE! {:?}", e);
                    println!("RESPONSE {:?}", response_text);
                    None
                }
            }
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}

async fn fetch_user(token: &String) {
    let q = "{ viewer { name } }";

    let response = query(&token, q).await;

    return match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse<UserViewer>>().await {
                Ok(parsed) => {
                    println!("{:?}", parsed);
                    Some(parsed)
                }
                Err(e) => {
                    println!("FAILED TO PARSE! {:?}", e);
                    None
                }
            };
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}

async fn query(token: &String, q: &str) -> Response {
    let mut body = HashMap::new();
    body.insert("query", q);

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
    response
}