
use rand::seq::SliceRandom;
trait Types<T> {
    fn get_types() -> Vec<T>;
}

#[derive(Debug, Clone, Copy)]
pub enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Types<CardType> for CardType {
    fn get_types() -> Vec<CardType> {
        vec![
            CardType::Ace,
            CardType::King,
            CardType::Queen,
            CardType::Jack,
            CardType::Ten,
            CardType::Nine,
            CardType::Eight,
            CardType::Seven,
            CardType::Six,
            CardType::Five,
            CardType::Four,
            CardType::Three,
            CardType::Two,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardColor {
    Red,
    Black,
}

impl Types<CardColor> for CardColor {
    fn get_types() -> Vec<CardColor> {
        vec![CardColor::Red, CardColor::Black]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardSymbol {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Types<CardSymbol> for CardSymbol {
    fn get_types() -> Vec<CardSymbol> {
        vec![
            CardSymbol::Spade,
            CardSymbol::Heart,
            CardSymbol::Diamond,
            CardSymbol::Club,
        ]
    }
}

#[derive(Debug)]
pub struct Card(u8, CardColor, CardType, CardSymbol);

fn shuffle<T>(vec: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
}

pub fn get_shuffeled_deck() -> Vec<Card> {
    let mut cards = get_deck_of_cards();
    shuffle(&mut cards);
    cards
}

fn get_deck_of_cards() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    let cardtypes = CardType::get_types();
    let cardcolors = CardColor::get_types();
    let cardyymbols = CardSymbol::get_types();

    let mut counter = 1;
    for cardtype in cardtypes.iter() {
        deck.push(Card(counter, cardcolors[0], *cardtype, cardyymbols[0]));
        deck.push(Card(counter, cardcolors[0], *cardtype, cardyymbols[1]));
        deck.push(Card(counter, cardcolors[1], *cardtype, cardyymbols[2]));
        deck.push(Card(counter, cardcolors[1], *cardtype, cardyymbols[3]));
        counter += 1;
    }

    return deck;
}
