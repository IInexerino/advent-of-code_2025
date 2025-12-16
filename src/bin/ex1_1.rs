use advent_of_code_2025::ex1::{Dial, DialTurn, parse_dial_turns};


fn turn_and_count_0s(dial: &mut Dial, dial_turns: Vec<DialTurn>) -> u32 {
    let mut counter = 0;

    for dial_turn in dial_turns {

        dial.turn_dial(dial_turn);
        println!("{}", dial.0);

        if dial.check_0() { counter += 1 };

    }

    return counter
}




fn main() {
    let dial_turns = parse_dial_turns("./assets/ex1_input.txt");

    let mut dial = Dial::new(50);

    let count = turn_and_count_0s(&mut dial, dial_turns);

    println!("number of 0s: {count}")
}