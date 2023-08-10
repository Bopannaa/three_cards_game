mod cards;
mod player;
mod score_finder;
use player::Player;
use score_finder::find_score_type;

use crate::score_finder::ScoreType;

fn main() {
    let mycards = cards::get_shuffeled_deck();

    let mut players = vec![];

    for i in 0..7 {
        let start = 0 + (3 * i);
        let end = 3 + (3 * i);
        players.push(Player::new(i + 1, mycards[start..end].to_vec()));
    }

    for player in &players {
        println!("-------------------------------------");
        let player_score = find_score_type(&player);
        println!("Player-{:?} score: {:?}", player.get_id(), &player_score);


        //let cards = player.get_cards();
        // for card in cards {
        //     println!("{:?}", card);
        // }
    //     match player_score {
    //         ScoreType::StraightFlush => println!("Its a StraightFlush"),
    //         ScoreType::ThreeOfAKind => println!("Its 3 of a kind"),
    //         ScoreType::Straight => println!("Its a Straight"),
    //         ScoreType::Flush => println!("its a flush"),
    //         ScoreType::Pair => println!("its a pair"),
    //         ScoreType::HighCard => println!("Its a HighCard"),
    //     }
    }


    // let current_best = ScoreType::StraightFlush as u8;

    // println!("{:?}", current_best);
}
