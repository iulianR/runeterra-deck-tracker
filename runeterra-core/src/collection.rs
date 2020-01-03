use crate::{Card, DB};
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct Collection<'a> {
    pub cards: Vec<Card<'a>>,
}

impl<'a> Collection<'a> {
    pub fn new() -> Self {
        let cards: Vec<Card> = DB
            .collection
            .0
            .iter()
            .map(|db_card| Card::try_from(db_card).expect("Card does not exist in database"))
            .collect();

        Collection { cards }
    }
}
