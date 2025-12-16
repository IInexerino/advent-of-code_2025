use advent_of_code_2025::ex2::{IdRange, parse_ids};

fn process_range(range: IdRange) -> Vec<u64> {
    let mut invalid_codes = Vec::new();

    for current_id in range.lower..=range.upper {
        let id_as_str = current_id.to_string();
        let id_digit_chars: Vec<char> = id_as_str.chars().collect();

        if id_digit_chars.len() % 2 != 0 { continue }

        let halfway_idx = id_digit_chars.len() / 2;

        let two_nums = id_digit_chars.split_at(halfway_idx);

        let left_num = two_nums.0.iter().collect::<String>().parse::<u64>().unwrap();
        let right_num = two_nums.1.iter().collect::<String>().parse::<u64>().unwrap();

        if left_num == right_num {
            invalid_codes.push(current_id);
        } else { continue }

    }

    return invalid_codes
} 


fn main() {
    let id_ranges = parse_ids("./assets/ex2_input.txt");

    let mut invalid_codes = Vec::new();

    for id_range in id_ranges {
        println!("lower: ({}), upper: ({})", id_range.lower, id_range.upper);
        invalid_codes.append(&mut process_range(id_range));

    }
    println!("INVALID CODES:");
    let mut password: u128 = 0;
    for code in invalid_codes {
        password += code as u128;
        println!("{code}")
    }
    println!("PASSWORD: {password}")
}