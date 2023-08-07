mod cards;

fn main() {
    let mycards = cards::get_shuffeled_deck();
    println!("{:?}", mycards[0]);
    println!("{:?}", mycards[1]);
    println!("{:?}", mycards[2]);
}
