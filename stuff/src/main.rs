use std::{collections::HashMap};
use tokio;
use reqwest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new()

    let resp = reqwest::get("https://sawsha-is.gay/len")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{}", resp.get("origin").unwrap()); // Would return: 76.94.9.59
    Ok(())
}
