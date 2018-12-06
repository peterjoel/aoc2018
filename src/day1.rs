use std::{
    io::Read,
    fs::File,
};

use super::Result;

pub fn run(input_path: Option<&str>) -> Result {
    let input_path = input_path.unwrap_or("data/day1.txt");
    let mut file = File::open(input_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    let sum = sum_changes(source.lines())?;
    println!("{}", sum);
    Ok(())
}

fn sum_changes<'a>(changes: impl Iterator<Item = &'a str>) -> Result<i32> {
    let sum = changes
        .flat_map(|line| line.parse::<i32>().ok())
        .sum();
    Ok(sum)
}

#[test]
fn test_sum() {
    let changes = "+1 -1 +3 -2";
    let sum = sum_changes(changes.split_whitespace());
    assert_eq!(Ok(1), sum);
}