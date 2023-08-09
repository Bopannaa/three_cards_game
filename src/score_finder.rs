use crate::player::Player;
use crate::cards::{CardSymbol, CardType};

#[derive(Debug)]
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


    let no = max+min/2;

    no == max - 1 && no == min + 1
}

fn is_pair(values: &Vec<u8>) -> bool {
    let first = &values[0];
    let second = &values[1];
    let third = &values[2];

    if first == second {
        true
    } else if first == third  {
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
