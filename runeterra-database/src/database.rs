use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Db {
    pub globals: Globals,
    pub collection: Collection,
}

impl Default for Db {
    fn default() -> Self {
        Db::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Globals {
    pub regions: Vec<Region>,
    pub keywords: Vec<Keyword>,
    pub spell_speeds: Vec<SpellSpeed>,
    pub rarities: Vec<Rarity>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub abbreviation: String,
    pub icon_absolute_path: String,
    pub name: String,
    pub name_ref: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    pub description: String,
    pub name: String,
    pub name_ref: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSpeed {
    pub name: String,
    pub name_ref: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rarity {
    pub name: String,
    pub name_ref: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection(pub Vec<Card>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub associated_cards: Vec<String>,
    pub associated_card_refs: Vec<String>,
    pub assets: Vec<Asset>,
    pub region: String,
    pub region_ref: String,
    pub attack: u8,
    pub cost: u8,
    pub health: u8,
    pub description: String,
    pub description_raw: String,
    pub levelup_description: String,
    pub levelup_description_raw: String,
    pub flavor_text: String,
    pub artist_name: String,
    pub name: String,
    pub card_code: String,
    pub keywords: Vec<String>,
    pub keyword_refs: Vec<String>,
    pub spell_speed: String,
    pub spell_speed_ref: String,
    pub rarity: String,
    pub rarity_ref: String,
    pub subtype: String,
    pub supertype: String,
    pub r#type: String,
    pub collectible: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub game_absolute_path: String,
    pub full_absolute_path: String,
}

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/resources/en_us/"]
struct DatabaseAsset;

impl Db {
    pub fn new() -> Self {
        let file =
            DatabaseAsset::get("data/globals-en_us.json").expect("Invalid globals.json file path");
        let globals = serde_json::from_slice(&file).expect("Invalid globals.json format");

        let file = DatabaseAsset::get("data/set1-en_us.json").expect("Invalid set1.json file path");
        let collection = serde_json::from_slice(&file).expect("Invalid set1.json format");

        Db {
            globals,
            collection,
        }
    }
}
