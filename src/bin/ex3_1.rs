use advent_of_code_2025::ex3::{BatteryBank, parse_battery_banks};

fn process_battery_bank(battery_bank: BatteryBank) -> u32 {
    let mut first_highest_num = 0;
    let mut first_idx = 0;

    let mut idx_counter = 0;
    
    for digit in battery_bank.0.clone()  {
        let digit = digit.to_digit(10).expect("Error: Character cannot be converted to digit");

        if digit > first_highest_num {
            first_highest_num = digit;
            first_idx = idx_counter
        }

        idx_counter += 1;
    }

    let mut second_highest_num = 0;
    let mut second_idx = 0;

    let mut idx_counter = if first_idx == battery_bank.0.len() - 1 { 0 } 
        else { first_idx + 1 };

    for digit in &battery_bank.0[idx_counter..] { 
        if idx_counter == first_idx { idx_counter += 1; continue }
        
        let digit = digit.to_digit(10).expect("Error: Character cannot be converted to digit");

        if digit > second_highest_num {
            second_highest_num = digit;
            second_idx = idx_counter;
        }

        idx_counter += 1;
    }

    let order = if first_idx < second_idx { 
        first_highest_num.to_string() + &second_highest_num.to_string() 
    } else {
        second_highest_num.to_string() + &first_highest_num.to_string() 
    };

    let final_number = order.parse::<u32>().expect("Error: digits string cannot be parsed into a u32");

    return final_number
}

fn main() {

    let mut total_number = 0;

    let battery_banks = parse_battery_banks("./assets/ex3_input.txt");

    for battery_bank in battery_banks {
        total_number += process_battery_bank(battery_bank)
    }

    println!("{total_number}");
    
    
    /*
    let battery_bank = BatteryBank(vec!['8','1','8','1','8','1','9','1','1','1','1','2','1','1','1']);

    let num = process_battery_bank(battery_bank);

    println!("{num}");
    */
}