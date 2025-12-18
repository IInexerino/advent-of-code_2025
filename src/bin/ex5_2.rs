use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Clone)]
struct FreshRange {
    lower: u128,
    upper: u128
}

impl FreshRange {
    fn new(lower: u128, upper: u128) -> Self {
        FreshRange { lower, upper }
    }

    fn len(&self) -> u128 {
        self.upper - self.lower + 1
    }

    fn check_overlap(&self, b: &FreshRange) -> RangeOverlap {
        if self == b {
            return RangeOverlap::AEqualsB
        }
        else if self.lower >= b.lower 
        && self.upper <= b.upper {
            return RangeOverlap::AInsideB
        }
        else if self.lower <= b.lower 
        && self.upper >= b.upper {
            return RangeOverlap::AContainsB
        }
        else if self.lower <= b.upper
        && self.lower > b.lower {
            return RangeOverlap::AStartsInB
        } 
        else if self.upper < b.upper
        && self.upper >= b.lower {
            return RangeOverlap::AEndsInB
        } 
        else {
            RangeOverlap::None
        }
    }
}

impl PartialEq for FreshRange {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower
        && self.upper == other.upper
    }
}

enum RangeOverlap {
    AContainsB,
    AInsideB,
    AEndsInB,
    AStartsInB,
    AEqualsB,
    None,
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

    // we check every unvalidated range, if it is validated we push it to non_overlapping_ranges. If it is invalid we 
    for mut range in fresh_ranges {
        if non_overlapping_ranges.is_empty() { non_overlapping_ranges.push(range); continue }
        println!("Working on new range: {}-{}", range.lower, range.upper);

        for non_overlapping_range in non_overlapping_ranges.clone() {
            match range.check_overlap(&non_overlapping_range) {
                RangeOverlap::AContainsB => non_overlapping_ranges.retain(|a | a != &non_overlapping_range),
                RangeOverlap::AInsideB => range = FreshRange::new(0, 0),
                RangeOverlap::AEndsInB => range.upper = non_overlapping_range.lower - 1,
                RangeOverlap::AStartsInB => range.lower = non_overlapping_range.upper + 1,
                RangeOverlap::AEqualsB => range = FreshRange::new(0, 0),
                RangeOverlap::None => { continue },
            }
        }
        if range != FreshRange::new(0, 0) { non_overlapping_ranges.push(range); }
    } 

    return non_overlapping_ranges
}


fn main() {
    let mut total_ids = 0;
    let fresh_ranges = parse_ranges_and_ids("./assets/ex5_input.txt");

    let non_overlapping_ranges = get_non_overlapping_ranges(fresh_ranges);

    for range in non_overlapping_ranges {
        println!("{}-{}",range.lower, range.upper);
        total_ids += range.len();
    }

    println!("Total IDs: {}", total_ids);
}