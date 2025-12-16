pub mod ex1 {
    use std::{fs::File, io::{BufRead, BufReader}};

    pub enum TurnDirection {
        Left,
        Right
    }


    pub struct DialTurn {
        pub dir: TurnDirection,
        pub amount: u32
    }

    impl DialTurn {
        pub fn new(dir: TurnDirection, amount: u32) -> Self {
            DialTurn { dir, amount }
        }
    }

    pub struct Dial(pub u32);

    impl Dial {
        pub fn new(amount: u32) -> Self {
            return Dial(amount)
        }

        pub fn turn_dial(&mut self, dial_turn: DialTurn) -> u32 {
            let mut passed_0: u32 = 0;

            match dial_turn.dir {
                TurnDirection::Right => {
                    for _ in 0..dial_turn.amount {
                        if self.0 < 99 {
                            self.0 += 1
                        } else {
                            self.0 -= 99;
                            passed_0 += 1;
                        }

                    }
                },
                TurnDirection::Left => {
                    for _ in 0..dial_turn.amount {
                        if self.0 > 0 {
                            self.0 -= 1
                        } else {
                            self.0 += 99;
                        }
                        if self.0 == 0 { passed_0 += 1 }

                    }
                }
            }
            passed_0
        }

        pub fn check_0(&self) -> bool {
            if self.0 == 0 { return true } 
            else { return false }
        }
    }


    pub fn parse_dial_turns(path: &str) -> Vec<DialTurn> {
        let mut dial_turns = Vec::new();

        let file = File::open(path).expect("Panicked while trying to open file");
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result.expect("Panicked while reading a line");
            let mut line_chars = line.trim().chars();

            let dir_char = line_chars.next().expect("Panicked: string slice does not contain chars");

            let turn_dir = if dir_char == 'L' {
                TurnDirection::Left
            } else if dir_char == 'R' {
                TurnDirection::Right
            } else { panic!("invalid dir character")};


            let amount = line_chars.as_str().parse::<u32>().expect("remaining chars of the str slice are not numerical");

            let dial_turn = DialTurn::new(turn_dir, amount);

            dial_turns.push(dial_turn);
        }

        return dial_turns;
    }

}

pub mod ex2 {
    use std::fs::read_to_string;

        
    pub struct IdRange{
        pub upper: u64,
        pub lower: u64,
    }

    pub fn parse_ids(path: &str) -> Vec<IdRange> {
        let mut id_ranges = Vec::new();

        let contents = read_to_string(path).expect("could not open and read file");

        let range_strings: Vec<&str> = contents.split(',').collect();

        for range_string in range_strings {
            let range_str: Vec<&str> = range_string.split('-').collect();
            id_ranges.push(IdRange{
                lower: range_str[0].parse::<u64>().unwrap(),
                upper: range_str[1].parse::<u64>().unwrap()
            });

        }


        return id_ranges
    }
}

pub mod ex3 {
    use std::{fs::File, io::{BufRead, BufReader}};

    
    #[derive(Default)]
    pub struct BatteryBank(pub Vec<char>);

    pub fn parse_battery_banks(path: &str) -> Vec<BatteryBank> {
        let mut battery_banks: Vec<BatteryBank> = Vec::new();
        
        let file = File::open(path).expect("Panicked while trying to open file");
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line_chars: Vec<char> = line_result
                .expect("invalid line")
                .trim()
                .chars()
                .collect();

            battery_banks.push(BatteryBank(line_chars));
        }

        battery_banks
    }

}

pub mod ex4 {
    use std::{fs::File, io::{BufRead, BufReader}};

    
    pub struct Pos{
        pub x: i64,
        pub y: i64
    }

    impl Pos {
        pub fn new(x: i64, y: i64) -> Self {
            Pos{x, y}
        }
    }

    pub struct Grid(pub Vec<Vec<bool>>);

    impl Grid {
        pub fn parse_input_into_grid(path: &str) -> Self {
            let mut rows = Vec::new();

            let file = File::open(path).expect("Panicked while trying to open file");
            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                let line = line_result.expect("Panicked while reading a line");
                let line_chars = line.trim()
                    .chars();

                let mut row = Vec::new();

                for char in line_chars {
                    if char == '@' {
                        row.push(true)
                    } else if char == '.' {
                        row.push(false)
                    } else { panic!("invalid character")};
                }

                rows.push(row);
                
            }

            return Grid(rows)
        }

        pub fn toggle(&mut self, position: &Pos) {
            if self.0[position.y as usize][position.x as usize] { 
                self.0[position.y as usize][position.x as usize] = false 
            }
            else { 
                self.0[position.y as usize][position.x as usize] = true 
            }
        }

        pub fn is_paper(&self, position: &Pos) -> bool {
            if self.0[position.y as usize][position.x as usize] { true } 
            else { false }
        }

        pub fn is_in_bounds(&self, position: &Pos) -> bool {
            if !(position.y < 0)
            && !(position.x < 0)
            && position.y < self.0.len() as i64
            && position.x < self.0[0].len() as i64 { true }
            else { false }
        }

        pub fn is_movable(&self, pos: &Pos) -> bool {
            let mut occupied_neighbors: u8 = 0;

            let neighbor_coords: [Pos ; 8] = [
                Pos::new(pos.x + 1, pos.y + 1),
                Pos::new(pos.x, pos.y + 1),
                Pos::new(pos.x - 1, pos.y + 1),
                Pos::new(pos.x + 1, pos.y),
                Pos::new(pos.x - 1, pos.y),
                Pos::new(pos.x + 1, pos.y - 1),
                Pos::new(pos.x, pos.y - 1),
                Pos::new(pos.x - 1, pos.y - 1),
            ];

            for neighbor_coord in neighbor_coords {
                if !self.is_in_bounds(&neighbor_coord) { continue }

                if self.is_paper(&neighbor_coord) { occupied_neighbors += 1}
            }
            
            return if occupied_neighbors < 4 { true } else { false }; 
        }
    }
}