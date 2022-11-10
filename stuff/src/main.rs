use reqwest::{Client};
use tokio;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct SomeData {
    len: u32,
    status: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Client::new();
    let resp = session.post("https://sawsha-is.gay/len")
        .send()
        .await?
        .json::<SomeData>()
        .await?;

    println!("Amount of images: {}\nStatus: {}", resp.len, resp.status);

    Ok(())
}