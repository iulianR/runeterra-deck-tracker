use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
use mockito;

#[allow(dead_code)]
pub const DEFAULT_PORT: u16 = 21337;

#[derive(Debug)]
pub struct Client {
    session: reqwest::Client,
    base_url: Url,
}

/// The player's current deck in an active game.
///
/// See official ['API'] for more information.
///
/// ['API']: https://developer.riotgames.com/docs/lor#game-client-api_active-deck
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticDecklist {
    pub deck_code: Option<String>,
    pub cards_in_deck: Option<HashMap<String, u8>>,
}

/// Screen information.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Screen {
    pub screen_width: u16,
    pub screen_height: u16,
}

/// Information about a rectangle on screen.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rectangle {
    #[serde(rename = "GameID")]
    pub card_id: i32,
    pub card_code: String,
    pub top_left_x: u16,
    pub top_left_y: u16,
    pub width: u16,
    pub height: u16,
    pub local_player: bool,
}

/// The position of the cards in the collection, deck builder, Expedition drafts, and active games.
///
/// See official ['API'] for more information.
///
/// ['API']: https://developer.riotgames.com/docs/lor#game-client-api_card-positions
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PositionalRectangles {
    pub player_name: Option<String>,
    pub opponent_name: Option<String>,
    pub game_state: String,
    pub screen: Screen,
    pub rectangles: Vec<Rectangle>,
}

/// The player's drafted cards during an Expedition.
///
/// See official ['API'] for more information.
///
/// ['API']: https://developer.riotgames.com/docs/lor#game-client-api_expeditions
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionsState {
    pub is_active: bool,
    pub state: String,
    pub record: Option<Vec<String>>,
    // Not implemented, type unknown
    pub draft_picks: Option<Vec<String>>,
    pub deck: Option<Vec<String>>,
    pub games: u8,
    pub wins: u8,
    pub losses: u8,
}

/// Result of the player's most recently completed game.
///
/// See official ['API'] for more information.
///
/// ['API']: https://developer.riotgames.com/docs/lor#game-client-api_game-result
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameResult {
    #[serde(rename = "GameID")]
    pub game_id: i32,
    pub local_player_won: bool,
}

impl Client {
    pub fn new(port: u16) -> Self {
        #[cfg(test)]
        let url = &mockito::server_url();

        #[cfg(not(test))]
        let url = format!("http://localhost:{}", port);

        Client {
            session: reqwest::Client::builder().build().unwrap(),
            base_url: Url::parse(&url).unwrap(),
        }
    }

    pub async fn get_static_decklist(&self) -> Result<StaticDecklist, crate::Error> {
        Ok(self
            .session
            .get(self.base_url.join("/static-decklist").unwrap())
            .send()
            .await?
            .json::<StaticDecklist>()
            .await?)
    }

    pub async fn get_positional_rectangles(&self) -> Result<PositionalRectangles, crate::Error> {
        Ok(self
            .session
            .get(self.base_url.join("/positional-rectangles").unwrap())
            .send()
            .await?
            .json::<PositionalRectangles>()
            .await?)
    }

    pub async fn get_expeditions_state(&self) -> Result<ExpeditionsState, crate::Error> {
        Ok(self
            .session
            .get(self.base_url.join("/expeditions-state").unwrap())
            .send()
            .await?
            .json::<ExpeditionsState>()
            .await?)
    }

    pub async fn get_game_result(&self) -> Result<GameResult, crate::Error> {
        Ok(self
            .session
            .get(self.base_url.join("/game-result").unwrap())
            .send()
            .await?
            .json::<GameResult>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Client, ExpeditionsState, GameResult, PositionalRectangles, Rectangle, Screen,
        StaticDecklist,
    };
    use mockito;

    #[tokio::test]
    async fn test_static_decklist() -> std::result::Result<(), crate::Error> {
        let body = StaticDecklist {
            deck_code: Some(
                "CEAAECABAMGA6EYXEYVS4NYIAECQCGY5FAVTCMRVAICACAYCBELDGBABAURCMKJW".to_string(),
            ),
            cards_in_deck: Some(
                [
                    ("01NX012".to_string(), 2),
                    ("01NX015".to_string(), 2),
                    ("01NX019".to_string(), 2),
                    ("01NX023".to_string(), 2),
                    ("01NX038".to_string(), 2),
                    ("01NX043".to_string(), 2),
                    ("01NX046".to_string(), 2),
                    ("01NX055".to_string(), 2),
                    ("01SI001".to_string(), 2),
                    ("01SI027".to_string(), 2),
                    ("01SI029".to_string(), 2),
                    ("01SI040".to_string(), 2),
                    ("01SI043".to_string(), 2),
                    ("01SI049".to_string(), 2),
                    ("01SI050".to_string(), 2),
                    ("01SI053".to_string(), 2),
                    ("01NX002".to_string(), 1),
                    ("01NX009".to_string(), 1),
                    ("01NX022".to_string(), 1),
                    ("01NX051".to_string(), 1),
                    ("01SI034".to_string(), 1),
                    ("01SI038".to_string(), 1),
                    ("01SI041".to_string(), 1),
                    ("01SI054".to_string(), 1),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
        };

        let m = mockito::mock("GET", "/static-decklist")
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body)?)
            .create();

        let client = Client::new();
        let res = client.get_static_decklist().await?;

        assert_eq!(body, res);
        m.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_positional_rectangles() -> std::result::Result<(), crate::Error> {
        let body = PositionalRectangles {
            player_name: Some("Player One".to_string()),
            opponent_name: Some("Player Two".to_string()),
            game_state: "InProgress".to_string(),
            screen: Screen {
                screen_width: 1920,
                screen_height: 1080,
            },
            rectangles: vec![Rectangle {
                card_id: 1427904082,
                card_code: "face".to_string(),
                top_left_x: 179,
                top_left_y: 481,
                width: 117,
                height: 117,
                local_player: true,
            }],
        };

        let m = mockito::mock("GET", "/positional-rectangles")
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body)?)
            .create();

        let client = Client::new();
        let res = client.get_positional_rectangles().await?;

        assert_eq!(body, res);
        m.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_expeditions_state() -> std::result::Result<(), crate::Error> {
        let body = ExpeditionsState {
            is_active: false,
            state: "Inactive".to_string(),
            record: None,
            draft_picks: None,
            deck: None,
            games: 0,
            wins: 0,
            losses: 0,
        };

        let m = mockito::mock("GET", "/expeditions-state")
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body)?)
            .create();

        let client = Client::new();
        let res = client.get_expeditions_state().await?;

        assert_eq!(body, res);
        m.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_game_result() -> std::result::Result<(), crate::Error> {
        let body = GameResult {
            game_id: -1,
            local_player_won: false,
        };

        let m = mockito::mock("GET", "/game-result")
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body)?)
            .create();

        let client = Client::new();
        let res = client.get_game_result().await?;

        assert_eq!(body, res);
        m.assert();

        Ok(())
    }
}
