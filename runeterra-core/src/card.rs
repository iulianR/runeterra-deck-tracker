use crate::DB;

use strum;

use runeterra_database::db;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid card code set")]
    InvalidCardCodeSet {
        #[from]
        source: ParseIntError,
    },
    #[error("Invalid card property")]
    InvalidCardProperty {
        #[from]
        source: strum::ParseError,
    },
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card<'a> {
    pub associated_cards: Vec<CardCode>,
    pub assets: Vec<Asset<'a>>,
    pub region: Region,
    pub attack: u8,
    pub cost: u8,
    pub health: u8,
    pub description: &'a str,
    pub levelup_description: &'a str,
    pub flavor_text: &'a str,
    pub artist_name: &'a str,
    pub name: &'a str,
    pub card_code: CardCode,
    pub keywords: Vec<Keyword<'a>>,
    pub spell_speed: SpellSpeed,
    pub rarity: Rarity,
    pub supertype: Supertype,
    pub subtype: Subtype,
    pub r#type: Type,
    pub collectible: bool,
}

impl<'a> TryFrom<&'a db::Card> for Card<'a> {
    type Error = crate::card::Error;

    fn try_from(db_card: &'a db::Card) -> Result<Self, Self::Error> {
        Ok(Card {
            associated_cards: db_card
                .associated_card_refs
                .iter()
                .map(|s| CardCode::from_str(s.as_str()))
                .collect::<Result<Vec<CardCode>, crate::card::Error>>()?,
            assets: db_card.assets.iter().map(Asset::from).collect(),
            region: Region::from_str(&db_card.region_ref)?,
            attack: db_card.attack,
            cost: db_card.cost,
            health: db_card.health,
            description: &db_card.description,
            levelup_description: &db_card.levelup_description,
            flavor_text: &db_card.flavor_text,
            artist_name: &db_card.artist_name,
            name: &db_card.name,
            card_code: CardCode::from_str(&db_card.card_code)?,
            keywords: db_card
                .keyword_refs
                .iter()
                .map(AsRef::as_ref)
                .map(Keyword::from)
                .collect(),
            spell_speed: SpellSpeed::from_str(&db_card.spell_speed_ref)?,
            rarity: Rarity::from_str(&db_card.rarity_ref)?,
            supertype: Supertype::from_str(&db_card.supertype)?,
            subtype: Subtype::from_str(&db_card.subtype)?,
            r#type: Type::from_str(&db_card.r#type)?,
            collectible: db_card.collectible,
        })
    }
}

impl<'a> TryFrom<&'a str> for Card<'a> {
    type Error = crate::card::Error;

    fn try_from(card_code: &'a str) -> Result<Self, Self::Error> {
        let db_card = DB
            .collection
            .0
            .iter()
            .find(|db_card| card_code == db_card.card_code)
            .expect("Card does not exist");
        Card::try_from(db_card)
    }
}

lazy_static! {
    static ref REGION_TO_INT: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("DE", 0);
        map.insert("FR", 1);
        map.insert("IO", 2);
        map.insert("NX", 3);
        map.insert("PZ", 4);
        map.insert("SI", 5);
        map
    };
    static ref INT_TO_REGION: HashMap<u32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, "DE");
        map.insert(1, "FR");
        map.insert(2, "IO");
        map.insert(3, "NX");
        map.insert(4, "PZ");
        map.insert(5, "SI");
        map
    };
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum Region {
    #[strum(default = "true")]
    Unknown(String),

    Neutral,
    Demacia,
    Freljord,
    Ionia,
    Noxus,
    #[strum(serialize = "PiltoverZaun", to_string = "Piltover & Zaun")]
    PiltoverZaun,
    #[strum(serialize = "ShadowIsles", to_string = "Shadow Isles")]
    ShadowIsles,
}

impl Region {
    fn from_value(value: u32) -> Region {
        match value {
            0 => Region::Demacia,
            1 => Region::Freljord,
            2 => Region::Ionia,
            3 => Region::Noxus,
            4 => Region::PiltoverZaun,
            5 => Region::ShadowIsles,
            99 => Region::Unknown("Unimplemented region".to_string()),
            _ => panic!("Call with valid value"),
        }
    }
    fn value(&self) -> u32 {
        match *self {
            Region::Demacia => 0,
            Region::Freljord => 1,
            Region::Ionia => 2,
            Region::Noxus => 3,
            Region::PiltoverZaun => 4,
            Region::ShadowIsles => 5,
            Region::Unknown(_) => 99,
            _ => panic!("Not implemented for unknown types"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Asset<'a> {
    game_absolute_path: &'a str,
    full_absolute_path: &'a str,
}

impl<'a> From<&'a db::Asset> for Asset<'a> {
    fn from(db_asset: &'a db::Asset) -> Self {
        Asset {
            game_absolute_path: &db_asset.game_absolute_path,
            full_absolute_path: &db_asset.full_absolute_path,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CardCode {
    set: u8,
    faction: Region,
    number: u32,
    assoc: String,
}

impl FromStr for CardCode {
    type Err = crate::card::Error;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let faction = REGION_TO_INT.get(&code[2..4]).unwrap_or(&99);
        let assoc = {
            if code.len() == 7 {
                "".to_string()
            } else {
                (&code[7..9]).to_string()
            }
        };
        Ok(CardCode {
            set: (&code[0..2]).parse()?,
            faction: Region::from_value(*faction),
            number: (&code[4..7]).parse()?,
            assoc,
        })
    }
}

impl CardCode {
    pub fn to_code(&self) -> String {
        format!(
            "{:02}{}{}{}",
            self.set,
            INT_TO_REGION.get(&self.faction.value()).unwrap_or(&"UN"),
            self.number,
            self.assoc
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum KeywordType {
    #[strum(default = "true")]
    Unknown(String),

    Obliterate,
    Skill,
    // Double Attack
    DoubleStrike,
    Weakest,
    Elusive,
    Drain,
    Stun,
    // Trap
    Autoplay,
    // Overwhelm
    SpellOverwhelm,
    Barrier,
    Capture,
    Frostbite,
    Burst,
    Fleeting,
    Fast,
    Overwhelm,
    // Quick Attack
    QuickStrike,
    Tough,
    Recall,
    Ionia,
    Regeneration,
    Lifesteal,
    Enlightened,
    Slow,
    Ephemeral,
    LastBreath,
    Challenger,
    Imbue,
    Fearsome,
    CantBlock,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Keyword<'a> {
    name: &'a str,
    name_ref: KeywordType,
    description: &'a str,
}

impl<'a> From<&'a str> for Keyword<'a> {
    fn from(db_keyword: &'a str) -> Self {
        match DB
            .globals
            .keywords
            .iter()
            .find(|&x| x.name_ref == db_keyword)
        {
            Some(keyword_details) => Keyword {
                name: &keyword_details.name,
                name_ref: KeywordType::from_str(&keyword_details.name_ref).unwrap(),
                description: &keyword_details.description,
            },
            None => {
                let description = "Keyword not implemented yet";
                Keyword {
                    name: db_keyword,
                    name_ref: KeywordType::from_str(db_keyword).unwrap(),
                    description,
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum SpellSpeed {
    #[strum(default = "true")]
    Unknown(String),
    Slow,
    Fast,
    Burst,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum Rarity {
    #[strum(default = "true")]
    Unknown(String),
    Common,
    Rare,
    Epic,
    Champion,
    None,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum Type {
    #[strum(default = "true")]
    Unknown(String),
    Ability,
    Spell,
    Trap,
    Unit,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum Subtype {
    #[strum(default = "true")]
    Unknown(String),
    #[strum(serialize = "")]
    None,
    Elite,
    Elnuk,
    Poro,
    Spider,
    Tech,
    Yeti,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumString, ToString)]
pub enum Supertype {
    #[strum(default = "true")]
    Unknown(String),
    #[strum(serialize = "")]
    None,
    Champion,
}
