use std::{fs::File, io::{BufRead, BufReader}};

struct FreshRange {
    lower: u128,
    upper: u128
}

impl FreshRange {
    fn new(lower: u128, upper: u128) -> Self {
        FreshRange { lower, upper }
    }
}

fn parse_ranges_and_ids(path: &str) -> Vec<FreshRange> {
    let mut fresh_ranges = Vec::new();

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

    return fresh_ranges
}

fn get_non_overlapping_ranges(fresh_ranges: Vec<FreshRange>) -> Vec<FreshRange> {
    let mut non_overlapping_ranges: Vec<FreshRange> = Vec::new();

    'outer: for range in fresh_ranges {

        if non_overlapping_ranges.is_empty() { non_overlapping_ranges.push(range); continue }
        
        '_inner: for validated_range in &non_overlapping_ranges {
            if range.lower >= validated_range.lower 
            && range.upper <= validated_range.upper { continue 'outer }
            else if range.lower < validated_range.lower {}
        }
    } 


    return non_overlapping_ranges
}


fn main() {
    let fresh_ranges = parse_ranges_and_ids("./assets/ex5_input.txt");

    let mut all_fresh_ids: Vec<u128> = Vec::new();

    '_outer: for fresh_range in fresh_ranges {
        println!("Working on range: {}-{}",fresh_range.lower, fresh_range.upper);
        '_inner: for id in fresh_range.lower..=fresh_range.upper {
                all_fresh_ids.push(id);
        } 
    }

    println!("{}", all_fresh_ids.len());
}