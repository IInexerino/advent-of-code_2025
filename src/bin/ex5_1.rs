use std::{fs::File, io::{BufRead, BufReader}};

struct FreshRange {
    lower: u128,
    upper: u128
}

fn parse_ranges_and_ids(path: &str) -> (Vec<FreshRange>, Vec<u128>) {
    let mut fresh_ranges = Vec::new();
    let mut ingredient_ids = Vec::new();

    
    let file = File::open(path).expect("Panicked while trying to open file");
    let reader = BufReader::new(file);
    let mut reader_lines = reader.lines();

    loop {
        let line = reader_lines.next().expect("lines in reader abscent").expect("Panicked while reading a line");
        if line == "" { break }

        let mut split_line = line.split('-');

        fresh_ranges.push(FreshRange {
                lower: split_line.next()
                    .expect("first element of range abscent")
                    .parse::<u128>()
                    .expect("first element of range is not numerical"),
                upper: split_line.next()
                    .expect("second element of range abscent")
                    .parse::<u128>()
                    .expect("second element of range is not numerical")
            }
        );
    }

    loop {
        if let Some(line_result) = reader_lines.next() {
                ingredient_ids.push(
                    line_result
                        .expect("Panicked while reading a line")
                        .parse::<u128>()
                        .expect("second element of range is not numerical")
                );
        } else { break }
    }

    return (fresh_ranges, ingredient_ids)
}

fn is_it_fresh(id: u128, fresh_ranges: &Vec<FreshRange>) -> bool {
    let mut is_fresh = false;

    println!("Checking id: '{id}'");

    for fresh_range in fresh_ranges {
        if id >= fresh_range.lower
        && id <= fresh_range.upper {
            is_fresh = true;
            break
        }    
    }

    return is_fresh;
}

fn main() {
    let mut total_fresh_ingredients: u32 = 0;
    
    let (fresh_ranges, ingredient_ids) = parse_ranges_and_ids("./assets/ex5_input.txt");

    let mut ingredient_ids = ingredient_ids.iter();

    loop {
        if let Some(id) = ingredient_ids.next() {
            if is_it_fresh(*id, &fresh_ranges) { total_fresh_ingredients += 1}
        } else { break }
    }

    println!("{total_fresh_ingredients}")
}