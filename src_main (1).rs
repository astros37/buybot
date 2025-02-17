use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Order {
    // Define the order structure based on the exchange's API requirements
    symbol: String,
    quantity: f64,
    price: f64,
    side: String,
    order_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set");

    let client = Client::new();

    let order = Order {
        symbol: "BTCUSDT".to_string(),
        quantity: 1.0,
        price: 50000.0,
        side: "BUY".to_string(),
        order_type: "LIMIT".to_string(),
    };

    let response = client
        .post("https://api.exchange.com/v1/order")
        .header("API-KEY", api_key)
        .header("API-SECRET", api_secret)
        .json(&order)
        .send()
        .await?;

    println!("Order response: {:?}", response.text().await?);

    Ok(())
}