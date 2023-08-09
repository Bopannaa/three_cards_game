use rand::seq::SliceRandom;
trait Types<T> {
    fn get_types() -> Vec<T>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
pub struct Card(u8, CardType, CardSymbol);

impl Card {
    pub fn get_value(&self) -> &u8 {
        &self.0
    }
    pub fn get_type(&self) -> &CardType {
        &self.1
    }
    pub fn get_symbol(&self) -> &CardSymbol {
        &self.2
    }
}

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
    let cardyymbols = CardSymbol::get_types();

    let mut counter = 14;
    for cardtype in cardtypes.iter() {
        deck.push(Card(counter, *cardtype, cardyymbols[0]));
        deck.push(Card(counter, *cardtype, cardyymbols[1]));
        deck.push(Card(counter, *cardtype, cardyymbols[2]));
        deck.push(Card(counter, *cardtype, cardyymbols[3]));
        counter -= 1;
    }

    return deck;
}
