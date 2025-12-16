use advent_of_code_2025::ex1::{Dial, parse_dial_turns};

fn main() {
    let dial_turns = parse_dial_turns("./assets/ex1_input.txt");

    let mut dial = Dial::new(50);

    let mut count = 0;

    for dial_turn in dial_turns {
        count += dial.turn_dial(dial_turn);
        println!("{}", dial.0);
    }

    println!("number of 0s: {count}")
}