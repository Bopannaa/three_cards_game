use crate::cards::{Card, CardColor, CardSymbol, CardType};

#[derive(Debug)]
pub struct Player(usize, Vec<Card>);

impl Player {
    pub fn new(id: usize, cards: Vec<Card>) -> Player {
        Player(id, cards)
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.1
    }

    pub fn get_values(&self) -> Vec<u8> {
        let mut list = vec![];
        for i in &self.1 {
            list.push(*i.get_value());
        }
        list
    }

    pub fn get_colors(&self) -> Vec<CardColor> {
        let mut list = vec![];
        for i in &self.1 {
            list.push(*i.get_color());
        }
        list
    }

    pub fn get_types(&self) -> Vec<CardType> {
        let mut list = vec![];
        for i in &self.1 {
            list.push(*i.get_type());
        }
        list
    }

    pub fn get_symbols(&self) -> Vec<CardSymbol> {
        let mut list = vec![];
        for i in &self.1 {
            list.push(*i.get_symbol());
        }
        list
    }
}
