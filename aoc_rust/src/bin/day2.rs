use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let input1 = fs::read_to_string("inputs/day2")?;
    let mut sum: i64 = 0;

    for range in input1.trim().split(',') {
        let range = range.trim();
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let start = parts[0].trim().parse::<i64>()?;
        let end = parts[1].trim().parse::<i64>()?;

        for n in start..=end {
            if is_invalid_range(n) {
                sum += n;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}

// Part 1: Check if the number is exactly two identical halves
// e.g., 123123 (two "123"s), 55 (two "5"s)
fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    &s[..mid] == &s[mid..]
}

// Part 2: Check if the number is any pattern repeated at least twice
// e.g., 123123123 (three "123"s), 1212 (two "12"s), 111 (three "1"s)
// Loop through all possible pattern lengths from 1 to half the total length
// For each pattern length that evenly divides the total, check if repeating
// that pattern recreates the entire number
fn is_invalid_range(n: i64) -> bool {
    let s = n.to_string();
    for pattern_len in 1..=s.len()/2 {
        if s.len() % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            let repeated = pattern.repeat(s.len() / pattern_len);
            if repeated == s {
                return true;
            }
        }
    }
    false
}
