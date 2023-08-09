mod cards;
mod player;

use cards::{CardSymbol, CardType};
use player::Player;

#[derive(Debug)]
enum ScoreType {
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

    values
        .iter()
        .any(|i| (max + min) as f64 / 2.0 == (*i) as f64)
}

fn find_score_type(player: &Player) -> ScoreType {
    if is_series(&player.get_values().to_vec()) {
        if is_equal::<CardSymbol>(&player.get_symbols().to_vec()) {
            return ScoreType::StraightFlush;
        }
        return ScoreType::Straight;
    } else if is_equal::<CardSymbol>(&player.get_symbols().to_vec()) {
        return ScoreType::Flush;
    } else if is_equal::<CardType>(&player.get_types().to_vec()) {
        ScoreType::ThreeOfAKind
    } else {
        ScoreType::HighCard
    }
}

fn main() {
    let mycards = cards::get_shuffeled_deck();

    let mut players = vec![];

    for i in 0..3 {
        let start = 0 + (3 * i);
        let end = 3 + (3 * i);
        players.push(Player::new(i + 1, mycards[start..end].to_vec()));
    }

    let player_score = find_score_type(&players[0]);

    match player_score {
        ScoreType::StraightFlush => println!("Its a StraightFlush"),
        ScoreType::ThreeOfAKind => println!("Its 3 of a kind"),
        ScoreType::Straight => println!("Its a Straight"),
        ScoreType::Flush => println!("its a flush"),
        ScoreType::HighCard => println!("Its a HighCard"),
        _ => println!("None"),
    }

    for card in players[0].get_cards() {
        println!("{:?}", card);
    }
}
