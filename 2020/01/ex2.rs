use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// I'll now try a different approach
// I'll use a Vector and will be iterating through pairs of numbers, and checking if the remainder
// has any number that hits our 2020 target

// const FILE: &str = "./test.txt";
const FILE: &str = "./input.txt";
const GOAL: i32 = 2020;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    // read from the file and populate the vector
    if let Ok(lines) = read_lines(FILE) {
        for line in lines {
            if let Ok(input) = line {
                if let Ok(num) = input.parse::<i32>() {
                    numbers.push(num);
                }
            }
        }
    }

    for (i, first) in numbers.iter().enumerate() {
        let rest = [&numbers[0..i], &numbers[i + 1..]].concat(); // remainder (without first)
        for (j, second) in rest.iter().enumerate() {
            if first + second >= GOAL {
                // if the first and second sum is bigger than our goal, we cut it short
                continue;
            }
            let other_rest = [&rest[0..j], &rest[j + 1..]].concat(); // remainder (without second)

            for third in &other_rest {
                if first + second + third == GOAL {
                    println!("Found it: {} * {} * {} = {}", first, second, third, first * second * third);
                    return
                }
            }
        }
    }

    println!("not found")
}

// I basically googled for a read lines function in rust
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

