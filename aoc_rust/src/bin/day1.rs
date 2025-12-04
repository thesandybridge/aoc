use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let input1 = fs::read_to_string("inputs/day1")?;
    let mut pointer = 50;
    let mut combinations: Vec<i32> = Vec::new();
    let mut total_crossings: i32 = 0;

    let lines = input1
        .trim()
        .lines();

    for line in lines {
        let (direction, amount) = line.split_at(1);
        let rot = amount.parse::<i32>().unwrap();

        let crossings = match direction {
            "L" => {
                if pointer == 0 {
                    rot / 100
                } else if rot >= pointer {
                    (rot - pointer) / 100 + 1
                } else {
                    0
                }
            }
            "R" => (pointer + rot) / 100,
            _ => 0
        };
        total_crossings += crossings;
        // First % 100 gets the remainder when dividing by 100
        // + 100 ensures the result is positive (handles negative remainders from Rust's modulo)
        // Second % 100 brings it back into 0-99 range
        match direction {
            "L" => {
                pointer = ((pointer - rot) % 100 + 100) % 100;
                combinations.push(pointer);
            }
            "R" => {
                pointer = ((pointer + rot) % 100 + 100) % 100;
                combinations.push(pointer);
            }
            _ => {}
        }
    }

    let count = combinations.iter().filter(|&&x| x == 0).count();

    println!("Part 1: {}", count);
    println!("Part 2: {}", total_crossings);

    Ok(())
}
