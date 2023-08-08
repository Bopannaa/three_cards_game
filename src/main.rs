mod cards;
mod player;

use cards::{Card, CardSymbol, CardType};
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

fn is_equal<T: PartialEq>(values: &Vec<T>) -> bool {
    let first_value = values[0];
    values.iter().all(|&value| first_value == value)
}

fn is_series<T>(values: &Vec<T>) -> bool {
    let max = 0;
    let min = 0;
    true
}

fn find_score_type(player: Player) -> ScoreType {
    if is_equal::<CardSymbol>(&player.get_symbols().to_vec()) {
        ScoreType::Flush
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

    let player_score = find_score_type(players[0].get_cards());

    match player_score {
        ScoreType::Flush => println!("its a flush"),
        ScoreType::ThreeOfAKind => println!("Its 3 of a kind"),
        _ => println!("None"),
    }
}
