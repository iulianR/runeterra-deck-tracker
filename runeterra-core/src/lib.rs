#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate lazy_static;

mod card;
mod collection;
mod deck;
mod error;

pub use self::card::*;
pub use self::collection::*;
pub use self::deck::*;
pub use self::error::*;

lazy_static! {
    pub static ref DB: runeterra_database::db::Db = { runeterra_database::db::Db::new() };
}
