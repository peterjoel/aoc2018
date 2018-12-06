use std::{fs::File, io::Read};

use super::Result;

pub fn run(input_path: Option<&str>) -> Result {
    let input_path = input_path.unwrap_or("data/day1.txt");
    let mut file = File::open(input_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    // Part 1
    println!("Part 1: {}", sum_changes(&source)?);
    // Part 2
    println!("Part 2: {}", find_repeating_freq(&source)?);
    Ok(())
}

fn sum_changes(source: &str) -> Result<i32> {
    let sum = source
        .lines()
        .flat_map(|line| line.parse::<i32>().ok())
        .sum();
    Ok(sum)
}

fn find_repeating_freq(source: &str) -> Result<i32> {
    source
        .lines()
        .flat_map(|line| line.parse::<i32>().ok())
        .cycle()
        .scan((hashset![0], 0), |(seen, sum), n| {
            *sum += n;
            if seen.contains(sum) {
                Some(Some(*sum))
            } else {
                seen.insert(*sum);
                Some(None)
            }
        })
        .flatten()
        .next()
        .ok_or_else(|| "Unexpected error!".into())
}

#[test]
fn test_sum() {
    let sum = sum_changes("+1\n-1\n+3\n-2");
    assert_eq!(1, sum.unwrap());
}

#[test]
fn repeat_immediately_zero() {
    let sum = find_repeating_freq("+1\n-1");
    assert_eq!(0, sum.unwrap());
}

#[test]
fn repeat_after_loops() {
    let sum = find_repeating_freq("+1\n+4\n+6\n-9");
    assert_eq!(5, sum.unwrap());
}
