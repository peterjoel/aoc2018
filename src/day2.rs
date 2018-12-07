use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::Read,
};

use super::Result;

pub fn run(input_path: Option<&str>) -> Result {
    let input_path = input_path.unwrap_or("data/day2.txt");
    let mut file = File::open(input_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    // Part 1
    println!("Part 1: {}", calc_checksum(&source));
    // Part 2
    println!("Part 2: {}", find_one_letter_diff(&source)?);
    Ok(())
}

fn calc_checksum(source: &str) -> i32 {
    let (twos, threes) = source.lines().map(|id| count_repeats(id.chars())).fold(
        (0, 0),
        |(mut twos, mut threes), map| {
            if map.contains(&2) {
                twos += 1;
            }
            if map.contains(&3) {
                threes += 1;
            }
            (twos, threes)
        },
    );
    twos * threes
}

fn count_repeats<T: Eq + Hash>(chars: impl IntoIterator<Item = T>) -> HashSet<i32> {
    let mut counts = HashMap::new();
    for ch in chars {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts.values().cloned().collect()
}

fn find_one_letter_diff(source: &str) -> Result<String> {
    let mut counts = HashMap::new();
    for line in source.lines() {
        for i in 0..line.len() {
            let left = &line[..i];
            let right = &line[i + 1..];
            *counts.entry((left, right)).or_insert(0) += 1;
        }
    }
    counts
        .iter()
        .find(|(_, &count)| count == 2)
        .map(|(&(left, right), _)| {
            let mut result = left.to_owned();
            result.push_str(right);
            result
        })
        .ok_or_else(|| "Could not find value".into())
}

#[test]
fn test_count_repeats() {
    let repeats = count_repeats(b"abcdafffvf");
    assert_eq!(hashset![1, 2, 4], repeats);
}

#[test]
fn test_calc_checksum() {
    let checksum = calc_checksum(
        "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
    );
    assert_eq!(12, checksum);
}

#[test]
fn text_closest_matches() {
    let closest = find_one_letter_diff(
        "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
    );
    assert_eq!("fgij", closest.unwrap())
}
