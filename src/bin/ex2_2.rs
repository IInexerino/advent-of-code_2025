use advent_of_code_2025::ex2::{IdRange, parse_ids};



fn process_range(range: IdRange) -> Vec<u64> {
    let mut invalid_codes = Vec::new();

    for current_id in range.lower..=range.upper {
        let id_as_str = current_id.to_string();
        let id_digit_chars: Vec<char> = id_as_str.chars().collect();

        for combo_size in 1..=(id_digit_chars.len() / 2) {
            if id_digit_chars.len() % combo_size != 0 { continue }

            let num_of_equally_sized_components = id_digit_chars.len() / combo_size;

            let mut id_digit_chars_iterator = id_digit_chars.iter();

            let mut combo_vec: Vec<u64> = Vec::new();

            for _ in 0..num_of_equally_sized_components {
                let mut char_vec = Vec::new();

                for _ in 0..combo_size {
                    if let Some(digit) = id_digit_chars_iterator.next() {
                        char_vec.push(*digit);
                    } else { panic!("AYO WTF") }
                }

                let parsed_id = char_vec.iter().collect::<String>().parse::<u64>().unwrap();
                combo_vec.push(parsed_id);
            }

            if combo_vec.iter().min() == combo_vec.iter().max() {
                invalid_codes.push(current_id);
                println!("current id: {current_id}");
                break;
            } else { continue }


        }
    }

    return invalid_codes
} 


fn main() {
    let id_ranges = parse_ids("./assets/ex2_input.txt");

    let mut all_invalid_codes = Vec::new();

    for id_range in id_ranges {
        println!("lower: ({}), upper: ({})", id_range.lower, id_range.upper);
        let invalid_codes = process_range(id_range);

        for code in invalid_codes {
            all_invalid_codes.push(code);
        }
    }
    println!("INVALID CODES:");
    let mut password: u128 = 0;
    for code in all_invalid_codes {
        password += code as u128;
        println!("{code}")
    }
    println!("PASSWORD: {password}")
}