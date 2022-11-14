mod utils;

use uwuki::{GatewayClient, HttpClient};
use utils::*;
use tokio;
use futures::{stream::StreamExt};
use log::{info};

const MYPREFIX: &str = "saw ";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = HttpClient::new()
        .name("Sawsha's bot".to_string())
        .rest_url("https://eludris.sawsha-is.gay/".to_string());

    let gateway = GatewayClient::new()
        .gateway_url("wss://eludris.sawsha-is.gay/ws/".to_string());
    
    info!("sadas");

    let mut events = gateway.get_events().await?;
    
    while let Some(mut msg) = events.next().await {
        if msg.content.starts_with(MYPREFIX) && msg.author != "Sawsha's bot" {
            msg.content.drain(..MYPREFIX.len());

            match msg.content.split_once(' ') {
                Some((cmd, args)) => match cmd {
                    "hi" => {

                        let mut cmd_args = args.to_string();
                        let idk = get_arg(&mut cmd_args);
                        
                        client.send(format!("The given args was: {:#?}", idk)).await?;
                    },
                    _ => {}
                },
                None => match msg.content.as_ref() {
                    "test" => {
                        client.send("Test").await?;
                    }
                    _ => {}
                }
        
            }

        }
    }
    
    Ok(())
}