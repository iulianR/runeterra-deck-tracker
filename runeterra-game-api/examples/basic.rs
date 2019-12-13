use runeterra_game_api::{Client, DEFAULT_PORT};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Client::new(DEFAULT_PORT));

    {
        let client = client.clone();
        tokio::spawn(async move {
            let result = client.get_static_decklist().await;
            dbg!(&result);
        });
    }

    {
        let client = client.clone();
        tokio::spawn(async move {
            let result = client.get_positional_rectangles().await;
            dbg!(&result);
        });
    }

    {
        let client = client.clone();
        tokio::spawn(async move {
            let result = client.get_expeditions_state().await;
            dbg!(&result);
        });
    }

    {
        let client = client.clone();
        tokio::spawn(async move {
            let result = client.get_game_result().await;
            dbg!(&result);
        });
    }

    sleep(Duration::from_secs(5));
    Ok(())
}
