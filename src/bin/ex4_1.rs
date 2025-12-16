use advent_of_code_2025::ex4::{Grid, Pos};

fn main() {
    let grid = Grid::parse_input_into_grid("./assets/ex4_input.txt");

    let mut movable: u64 = 0;

    for y in 0..grid.0.len() as i64 {
        for x in 0..grid.0[y as usize].len() as i64 {
            let pos = Pos::new(x, y);

            if grid.is_paper(&pos) 
            && grid.is_movable(&pos){
                movable += 1;
            }
            
            println!("{movable}")
        }
    }

    println!("Final Result: {movable}")
}