use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let input1 = fs::read_to_string("inputs/day3")?;

    let part1 = solve(&input1);
    let part2 = solve_part2(&input1);



    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solve(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let mut max = 0;

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let value = digits[i] * 10 + digits[j];
                    if value > max {
                        max = value;
                    }
                }
            }

            max
        })
        .sum::<u32>() as i32
}

fn solve_part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let len = digits.len();
            let target = 12;
            let mut result = 0u64;
            let mut start = 0;

            for remaining in (1..=target).rev() {
                let search_end = len - remaining;
                let mut best_pos = start;
                let mut best_digit = digits[start];

                for pos in start..=search_end {
                    if digits[pos] > best_digit {
                        best_digit = digits[pos];
                        best_pos = pos;
                    }
                }

                result = result * 10 + best_digit as u64;
                start = best_pos + 1;
            }

            result
        })
        .sum()
}
