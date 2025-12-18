use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Default, Debug)]
enum OperationType {
    Addition,
    Multiplication,
    #[default]
    Unknown
}

struct MathProblem {
    numbers: Vec<u64>,
    operation: OperationType
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

fn main() {
    let mut final_solution = 0;
    let mut solutions: Vec<u64> = Vec::new();

    let math_problems = parse_math_problems("./assets/ex6_input.txt");

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