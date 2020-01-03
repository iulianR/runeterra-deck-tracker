use crate::Card;

pub struct Deck<'a> {
    pub cards: Vec<Card<'a>>,
}
