use advent_of_code_2025::ex4::{Grid, Pos};

fn remove_what_we_can(grid: &mut Grid) -> u64 {

    let mut coords_to_remove: Vec<Pos> = Vec::new();

    for y in 0..grid.0.len() as i64 {
        for x in 0..grid.0[y as usize].len() as i64 {
            let pos = Pos::new(x, y);

            if grid.is_paper(&pos) 
            && grid.is_movable(&pos){
                coords_to_remove.push(pos);
            }
        }
    }

    for coord in &coords_to_remove {
        grid.toggle(coord);
    }

    return coords_to_remove.len() as u64;
}

fn main() {
    let mut grid = Grid::parse_input_into_grid("./assets/ex4_input.txt");

    let mut total_rolls_removed = 0;

    loop {
        let removed = remove_what_we_can(&mut grid);

        println!("Last removed: {removed}");

        if removed == 0 { break }
        else {
            total_rolls_removed += removed;
        }
    }

    println!("Removed in Total: {total_rolls_removed}");
}