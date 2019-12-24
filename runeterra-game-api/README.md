# Runeterra Game Client API

Proxy API for the Legends of Runeterra [Game Client API]

[Game Client API]: https://developer.riotgames.com/docs/lor#game-client-api

## Examples
```rust
use runeterra_game_api::{Client, DEFAULT_PORT};
#[tokio::main]
async fn main() {
   let client = Client::new(DEFAULT_PORT);
   let result = client.get_static_decklist().await;
   dbg!(&result);
}
```
