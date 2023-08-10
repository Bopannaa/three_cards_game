use crate::cards::{CardSymbol, CardType};
use crate::player::Player;

#[derive(Debug, PartialEq)]
pub enum ScoreType {
    StraightFlush,
    ThreeOfAKind,
    Straight,
    Flush,
    Pair,
    HighCard,
}

fn is_equal<T: PartialEq + Clone + Copy>(values: &Vec<T>) -> bool {
    let first_value = values[0];
    values.iter().all(|&value| first_value == value)
}

fn is_series(values: &Vec<u8>) -> bool {
    let mut max = 0;
    let mut min = 0;

    if let Some(x) = values.iter().max() {
        max = *x;
    };
    if let Some(x) = values.iter().min() {
        min = *x;
    };

    let no = (max + min) / 2;

    (no == max - 1) && (no == min + 1)
}

fn is_pair(values: &Vec<u8>) -> bool {
    let first = &values[0];
    let second = &values[1];
    let third = &values[2];

    if first == second {
        true
    } else if first == third {
        true
    } else if second == third {
        true
    } else {
        false
    }
}

pub fn find_score_type(player: &Player) -> ScoreType {
    if is_series(&player.get_values().to_vec()) {
        if is_equal::<CardSymbol>(&player.get_symbols().to_vec()) {
            return ScoreType::StraightFlush;
        }
        return ScoreType::Straight;
    } else if is_equal::<CardSymbol>(&player.get_symbols().to_vec()) {
        return ScoreType::Flush;
    } else if is_equal::<CardType>(&player.get_types().to_vec()) {
        return ScoreType::ThreeOfAKind;
    } else if is_pair(&player.get_values().to_vec()) {
        return ScoreType::Pair;
    } else {
        return ScoreType::HighCard;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;

    #[test]
    fn check_pair() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(3, CardType::Three, CardSymbol::Club));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::Pair);
    }

    #[test]
    fn check_flush() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(5, CardType::Five, CardSymbol::Diamond));
        mycards.push(Card::new(7, CardType::Seven, CardSymbol::Diamond));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::Flush);
    }

    #[test]
    fn check_straight() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(3, CardType::Three, CardSymbol::Club));
        mycards.push(Card::new(4, CardType::Four, CardSymbol::Diamond));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::Straight);
    }

    #[test]
    fn check_straight_flush() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(3, CardType::Three, CardSymbol::Diamond));
        mycards.push(Card::new(4, CardType::Four, CardSymbol::Diamond));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::StraightFlush);
    }

    #[test]
    fn check_three_of_a_kind() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Club));
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Spade));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::ThreeOfAKind);
    }

    #[test]
    fn highcard() {
        let mut mycards: Vec<Card> = vec![];
        mycards.push(Card::new(2, CardType::Two, CardSymbol::Diamond));
        mycards.push(Card::new(5, CardType::Five, CardSymbol::Club));
        mycards.push(Card::new(7, CardType::Seven, CardSymbol::Spade));

        let player = Player::new(1, mycards);

        let scoretype = find_score_type(&player);

        assert_eq!(scoretype, ScoreType::HighCard);
    }
}
