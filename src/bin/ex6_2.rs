use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Default, Debug)]
enum OperationType {
    Addition,
    Multiplication,
    #[default]
    Unknown
}

#[derive(Default)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: OperationType
}

impl MathProblem {
    fn empty_from_operation(operation: OperationType) -> Self {
        MathProblem { numbers: Vec::new(), operation }
    }
}

struct MathProblemGrid {
    numbers: Vec<Vec<char>>,
    width: usize,
    operation: OperationType
}

impl MathProblemGrid {
    fn empty_from_width_and_operation(width: usize, operation: OperationType) -> Self {
        MathProblemGrid { numbers: Vec::new() , width, operation }
    }
}

fn parse_math_problem_grids(path: &str) -> Vec<MathProblemGrid> {
    let mut math_problem_grids = Vec::new();

    let regular_math_problems = parse_math_problems(path);

    for reg_math_problem in regular_math_problems {
        let mut longest_number_len: usize = 0;

        for number in reg_math_problem.numbers {
            let length_of_number = number.to_string().chars().count();
            if length_of_number > longest_number_len {
                longest_number_len = length_of_number
            }
        }

        math_problem_grids.push(
            MathProblemGrid::empty_from_width_and_operation(longest_number_len, reg_math_problem.operation)
        );
    }

    let file = File::open(path).expect("Panicked while trying to open file");
    let reader = BufReader::new(file);
    let mut reader_lines = reader.lines();

    for _ in 0..4 {
        let line = reader_lines.next()
            .expect("next item in reader_lines iterator is abscent")
            .expect("could not read line");
        let mut line_chars = line.chars();

        for m_problem_grid in math_problem_grids.iter_mut() {
            let mut m_problem_row: Vec<char> = Vec::new();

            for _ in 0..m_problem_grid.width {
                m_problem_row.push(
                    line_chars.next().expect("less chars than calculated widths and whitespaces")
                );
            }

            m_problem_grid.numbers.push(m_problem_row);
            // skip whitespace

            if line_chars.clone().count() > 0 {
                line_chars.next().expect("less chars than calculated widths and whitespaces");
            }
        }
    }

    return math_problem_grids
}


fn parse_math_problems(path: &str) -> Vec<MathProblem> {
    let file = File::open(path).expect("Panicked while trying to open file");
    let reader = BufReader::new(file);
    let mut reader_lines = reader.lines();

    let mut math_problems: Vec<MathProblem> = reader_lines.next()
            .expect("next item in reader_lines iterator is abscent")
            .expect("could not read line")
            .split_whitespace()
            .map(|a| {
                MathProblem { 
                    numbers: vec![a.parse::<u64>().expect("supposed number is not numerical")], 
                    operation: OperationType::default() 
                }
            })
            .collect();

    for _ in 0..3 {
        let numbers: Vec<u64> = reader_lines.next()
            .expect("next item in reader_lines iterator is abscent")
            .expect("could not read line")
            .split_whitespace()
            .map(|a| a.parse::<u64>().expect("supposed number is not numerical"))
            .collect();

        let mut numbers_iter = numbers.iter();

        for math_problem in math_problems.iter_mut() {
            math_problem.numbers.push(*numbers_iter.next().expect("less numbers than math problems"));
        }
    }


    let last_line = reader_lines.next()
        .expect("next item in reader_lines iterator is abscent")
        .expect("could not read line");
    let operators: Vec<&str> = last_line.split_whitespace()
        .collect();

    let mut operators_iter = operators.iter();

    for math_problem in math_problems.iter_mut() {
        let operator = *operators_iter.next().expect("less operators than math problems");
        if operator == "*" { math_problem.operation = OperationType::Multiplication }
        else if operator == "+" { math_problem.operation = OperationType::Addition }
    }

    return math_problems
}

fn transform_grids_into_math_problems(grids: Vec<MathProblemGrid>) -> Vec<MathProblem> {
    let mut math_problems: Vec<MathProblem> = Vec::new();

    for grid in grids {
        let mut math_problem = MathProblem::empty_from_operation(grid.operation);

        for idx in (0..grid.width).rev() {
            let mut current_num: Vec<char> = Vec::new();
            for row in &grid.numbers {
                current_num.push(row[idx])
            }
            let current_num: String = current_num.iter().collect();
            let current_num = current_num.trim();

            math_problem.numbers.push(
                current_num.parse::<u64>().expect("current num is not numerical")
            );
        }

        math_problems.push(math_problem);
    }

    return math_problems
}

fn main() {
    let mut final_solution = 0;
    let mut solutions: Vec<u64> = Vec::new();

    let math_problem_grids = parse_math_problem_grids("./assets/ex6_input.txt");

    let math_problems = transform_grids_into_math_problems(math_problem_grids);

    for problem in math_problems {
        let mut number_in_calculation = 0;

        match problem.operation {
            OperationType::Addition => {
                for number in problem.numbers {
                    number_in_calculation += number;
                    println!("{number_in_calculation}");
                }
            },
            OperationType::Multiplication => {
                for number in problem.numbers {
                    if number_in_calculation == 0 {
                        number_in_calculation += number;
                    } else {
                        number_in_calculation *= number;
                    }
                    println!("{number_in_calculation}");
                }
            },
            OperationType::Unknown => panic!("WTF"),
        }
        solutions.push(number_in_calculation);
    }

    for solution in solutions {
        final_solution += solution
    }

    println!("Final Solution - {final_solution}")
}