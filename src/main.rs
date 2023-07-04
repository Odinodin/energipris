use std::env;
use std::collections::HashMap;
use reqwest;
use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use clap::{Parser};
use rasciigraph::{plot, Config};


#[derive(Parser)]
#[command(name = "energipriser")]
#[command(author = "Odin Standal <odin@odinodin.com>")]
#[command(version = "1.0")]
#[command(about = "Henter energipriser", long_about = None)]
struct Cli {
    #[arg(long, help="Viser dagens priser")]
    idag: bool,
    #[arg(long, help="Viser morgendagens priser")]
    imorgen: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Price {
    total: f64,
    energy: f64,
    starts_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceInfo {
    current: Price,
    today: Vec<Price>,
    tomorrow: Vec<Price>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Subscription {
    price_info: PriceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Home {
    current_subscription: Subscription,
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

#[tokio::main]
async fn main() {
    let token = match env::var_os("TIBBER_API_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$TIBBER_API_TOKEN is not set")
    };

    let cli = Cli::parse();
    let show_today = cli.idag == true || cli.idag == false && cli.imorgen == false;
    let show_tomorrow = cli.imorgen == true;

    let price_response = fetch_prices(&token).await;

    if price_response.is_some() {
        let unwrapped = price_response.unwrap();
        let price = unwrapped.data.viewer.homes.get(0);
        if price.is_some() {
            let price_info = &price.unwrap().current_subscription.price_info;


            if show_today == true {
                println!("  I dag (Pris nå {})", &price_info.current.total);
                println!("");

                println!(
                    "{}",
                    plot(
                        price_info.today.clone().iter().map(|p| p.total).collect(),
                        Config::default()
                            .with_width(24 * 4)
                            .with_offset(0) // Where the y-axis starts
                            .with_height(10),
                    )
                );
                // The graph library does not support x-axis, so this is a little hack
                println!("       ‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|");
                println!("       00:00                   06:00                   12:00                   18:00               24:00")
            }

            if show_tomorrow == true {
                println!("  I morgen (Pris nå {})", &price_info.current.total);
                println!("");

                println!(
                    "{}",
                    plot(
                        price_info.tomorrow.clone().iter().map(|p| p.total).collect(),
                        Config::default()
                            .with_width(24 * 4)
                            .with_offset(0) // Where the y-axis starts
                            .with_height(10),
                    )
                );
                // The graph library does not support x-axis, so this is a little hack
                println!("       ‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|");
                println!("       00:00                   06:00                   12:00                   18:00               24:00")
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
                Ok(parsed) => Some(parsed),
                Err(e) => {
                    println!("FAILED TO PARSE! {:?} {:?}", e, response_text);
                    None
                }
            }
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