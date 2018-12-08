use super::Result;
use lazy_static::*;
use regex::Regex;
use std::{fs::File, io::Read, str::FromStr};

pub fn run(input_path: Option<&str>) -> Result {
    let input_path = input_path.unwrap_or("data/day3.txt");
    let mut file = File::open(input_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    let claims: Vec<Rect> = source.lines().flat_map(parse_claim).collect();
    let (width, height) = get_dimensions(claims.iter().cloned());
    // Part 1
    println!(
        "Part 1: {}",
        count_overlapping_squares(&claims, Cloth::new(width, height))
    );
    // Part 2
    println!(
        "Part 2: {}",
        find_non_overlapping_claim(&claims, Cloth::new(width, height))?
    );
    Ok(())
}

fn count_overlapping_squares(claims: &[Rect], mut cloth: Cloth) -> u32 {
    for rect in claims {
        cloth.claim_rect(&rect);
    }
    cloth.squares().filter(|&&count| count > 1).count() as u32
}

fn find_non_overlapping_claim(claims: &[Rect], mut cloth: Cloth) -> Result<u32> {
    for rect in claims {
        cloth.claim_rect(&rect);
    }
    claims
        .into_iter()
        .find(move |&rect| cloth.squares_under_rect(*rect).all(|&count| count == 1))
        .map(|rect| rect.id)
        .ok_or_else(|| "Could not find a rectangle that doesn't overlap".into())
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Rect {
    id: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn new(id: u32, x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            id,
            x,
            y,
            width,
            height,
        }
    }

    fn right(&self) -> i32 {
        self.x + self.width
    }

    fn bottom(&self) -> i32 {
        self.y + self.height
    }
}

#[derive(Debug)]
struct Cloth {
    width: usize,
    data: Vec<u8>,
}

impl Cloth {
    fn new(width: usize, height: usize) -> Cloth {
        let data = vec![0; width * height];
        Cloth { width, data }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.data[y * self.width + x]
    }

    fn claim_rect(&mut self, rect: &Rect) {
        for i in rect.x..rect.right() {
            for j in rect.y..rect.bottom() {
                *self.get_mut(i as usize, j as usize) += 1;
            }
        }
    }

    fn squares(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }

    fn squares_under_rect(&self, rect: Rect) -> impl Iterator<Item = &u8> {
        self.data
            .chunks(self.width)
            .skip(rect.y as usize)
            .take(rect.height as usize)
            .flat_map(move |row| row.iter().skip(rect.x as usize).take(rect.width as usize))
    }
}

fn get_dimensions(rects: impl IntoIterator<Item = Rect>) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    for rect in rects {
        width = width.max(rect.right());
        height = height.max(rect.bottom());
    }
    (width as usize, height as usize)
}

fn parse_claim(source: &str) -> Result<Rect> {
    lazy_static! {
        static ref CLAIM_PATTERN: Regex =
            Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    fn parse_num_match<N: FromStr>(captures: Option<regex::Match>) -> Result<N> {
        captures
            .and_then(|value| value.as_str().parse().ok())
            .ok_or_else(|| "Failed to parse number".into())
    }
    CLAIM_PATTERN
        .captures(source)
        .ok_or_else(|| "Error parsing Rect".into())
        .and_then(|captures| {
            Ok(Rect::new(
                parse_num_match(captures.get(1))?,
                parse_num_match(captures.get(2))?,
                parse_num_match(captures.get(3))?,
                parse_num_match(captures.get(4))?,
                parse_num_match(captures.get(5))?,
            ))
        })
}

#[test]
fn test_simple_overlap() {
    let claims = vec![
        Rect::new(1, 1, 3, 4, 4),
        Rect::new(2, 3, 1, 4, 4),
        Rect::new(3, 5, 5, 2, 2),
    ];
    let cloth = Cloth::new(10, 10);
    let overlapping = count_overlapping_squares(&claims, cloth);
    assert_eq!(4, overlapping);
}

#[test]
fn test_parse_claim() {
    let claim = "#2 @ 32,10: 14x7";
    assert_eq!(Rect::new(2, 32, 10, 14, 7), parse_claim(claim).unwrap());
}

#[test]
fn test_find_non_overlapping() {
    let claims = vec![
        Rect::new(1, 1, 3, 4, 4),
        Rect::new(2, 3, 1, 4, 4),
        Rect::new(3, 5, 5, 2, 2),
    ];
    assert_eq!(
        3,
        find_non_overlapping_claim(&claims, Cloth::new(10, 10)).unwrap()
    );
}

#[test]
fn test_squares_under_rect() {
    let data: Vec<u8> = (0..100).collect();
    let cloth = Cloth { width: 10, data };
    let rect = Rect::new(1, 8, 8, 2, 2);
    let squares: std::collections::HashSet<_> = cloth.squares_under_rect(rect).cloned().collect();
    assert_eq!(hashset!(88, 89, 98, 99), squares);
}
