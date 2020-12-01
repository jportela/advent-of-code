use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// I don't have much Rust experience, so I tried implementing the following basic algorithm:
// 1. Read each line from the input.txt file
// 2. Parse those lines into numbers and add them to a Set
// 3. Iterate through that Set and check if (2020 - i) exists in the Set.
// 4. If it does, that's the pair that we're looking for

// Of course I immediately saw a few shortcomings with this algorithm. Namely my assumption
// that there won't be duplicate numbers in the input (ie: 1010, 1010).
// I also feel I could have chained the Result calls better (using ? for example)
// I need to study and refactor this into a better solution

// I don't really know how to implement unit tests in Rust, so for now, I'm testing manually
// const FILE: &str = "./test.txt";
const FILE: &str = "./input.txt";
const GOAL: i32 = 2020;

fn main() {
    let mut numbers: HashSet<i32> = HashSet::new();

    // read from the file and populate the hashset
    if let Ok(lines) = read_lines(FILE) {
        for line in lines {
            if let Ok(input) = line {
                if let Ok(num) = input.parse::<i32>() {
                    numbers.insert(num);
                }
            }
        }
    }

    // iterate through the set and check if there's a 2020 - num in there
    for num in &numbers {
        let target = GOAL - num; // 2020 - num
        if numbers.contains(&target) {
            // FOUND IT!
            println!("{} * {} = {}", num, target, num * target);
            return
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

