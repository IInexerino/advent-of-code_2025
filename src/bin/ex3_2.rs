use advent_of_code_2025::ex3::{BatteryBank, parse_battery_banks};

fn process_battery_bank(battery_bank: BatteryBank, size_of_desired_output: usize) -> u128 {

    let mut string_of_digits = String::new();

    let mut remaining_digits = size_of_desired_output;

    let mut previous_idx = 0;

    for _ in 0..remaining_digits {

        let mut current_highest_num = 0;

        let mut idx_counter = previous_idx;

        remaining_digits -= 1;

        for digit in &battery_bank.0[idx_counter..]  {

            if idx_counter == battery_bank.0.len() - remaining_digits { break; }
            
            let digit = digit.to_digit(10).expect("Error: Character cannot be converted to digit");

            if digit > current_highest_num {
                current_highest_num = digit;
                previous_idx = idx_counter + 1
            }

            println!("num: {current_highest_num}, idx: {idx_counter}");

            idx_counter += 1;
        }

        string_of_digits += &current_highest_num.to_string();

        println!("completed 1 remaining number\nCurrent string of digits: {string_of_digits}");

    }

    return string_of_digits.parse::<u128>().expect("Error: digits string cannot be parsed into a u32");
}

fn main() {
     
    let mut total_number = 0;

    let battery_banks = parse_battery_banks("./assets/ex3_input.txt");

    for battery_bank in battery_banks {
        total_number += process_battery_bank(battery_bank, 12)
    }

    println!("{total_number}");
    
    
    /* 
    let battery_bank = BatteryBank(vec!['2','8','9','2','7','9','8','9','2','2']);

    let num = process_battery_bank(battery_bank, 4);

    println!("{num}");
    */
}