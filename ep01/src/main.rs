use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Elf {
    id: i32,
    calories: Vec<i32>,
}

impl Elf {
    fn total_cals(&self) -> i32 {
        return self.calories.iter().copied().sum::<i32>();
    }
}

fn main() {
    let mut elves: Vec<Elf> = vec![];
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./part1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut count = 1;
        let mut current_elf = Elf {
            id: count,

            calories: vec![],
        };

        for line in lines {
            if let Ok(cal) = line {
                if cal == "" {
                    elves.push(current_elf.clone());
                    count = count + 1;
                    current_elf = Elf {
                        id: count,
                        calories: vec![],
                    };
                } else {
                    current_elf.calories.push(cal.parse::<i32>().unwrap());
                }
            }
        }
    }
    elves.sort_by(|a, b| a.total_cals().cmp(&b.total_cals()));
    for elf in elves {
        let cals = elf.total_cals();
        let id = elf.id;
        println!("{} has {} cals ", id, cals);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
