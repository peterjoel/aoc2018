use std::{
    fs::File,
    io::Read,
};
use regex::Regex;
use lazy_static::*;
use super::Result;

pub fn run(input_path: Option<&str>) -> Result {
    let input_path = input_path.unwrap_or("data/day3.txt");
    let mut file = File::open(input_path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    // Part 1
    println!("Part 1: {}", count_overlapping_claims(&source));
    Ok(())
}

fn count_overlapping_claims(source: &str) -> u32 {
    let (width, height) = get_dimensions(source.lines().flat_map(parse_claim));
    let cloth = Cloth::new(width, height);
    count_overlapping_squares(source.lines().flat_map(parse_claim), cloth)
}

#[derive(Debug, PartialEq)]
struct Rect {
    id: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn new(id: u32, x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { id, x, y, w, h }
    }
    fn right(&self) -> i32 {
        self.x + self.w
    }
    fn bottom(&self) -> i32 {
        self.y + self.h
    }
}

struct Cloth {
    w: usize,
    data: Vec<u8>,
}

impl Cloth {
    fn new(w: usize, h: usize) -> Cloth {
        let data = vec![0; w * h];
        Cloth { w, data }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.data[y * self.w + x]
    }

    fn claim_rect(&mut self, rect: &Rect) {
        for i in rect.x..rect.right() {
            for j in rect.y..rect.bottom() {
                *self.get_mut(i as usize, j as usize) += 1;
            }
        }
    }

    fn cells(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }
}

fn get_dimensions(rects: impl Iterator<Item = Rect>) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    for rect in rects {
        width = width.max(rect.right());
        height = height.max(rect.bottom());
    }
    (width as usize, height as usize)
}

fn count_overlapping_squares(rects: impl Iterator<Item = Rect>, mut cloth: Cloth) -> u32 {
    for rect in rects {
        cloth.claim_rect(&rect);
    }
    cloth.cells()
        .filter(|&&count| count > 1)
        .count() as u32
}

fn parse_claim(source: &str) -> Option<Rect> {
    lazy_static! {
        static ref CLAIM_PATTERN: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    CLAIM_PATTERN.captures(source)
        .and_then(|captures| Some(Rect::new(
            captures.get(1).and_then(|id| id.as_str().parse().ok())?,
            captures.get(2).and_then(|x| x.as_str().parse().ok())?,
            captures.get(3).and_then(|y| y.as_str().parse().ok())?,
            captures.get(4).and_then(|w| w.as_str().parse().ok())?,
            captures.get(5).and_then(|h| h.as_str().parse().ok())?,
        )))
}

#[test]
fn test_simple_overlap() {
    let source = vec![Rect::new(1, 1, 3, 4, 4), Rect::new(2, 3, 1, 4, 4), Rect::new(3, 5, 5, 2, 2)];
    let cloth = Cloth::new(10, 10);
    let overlapping = count_overlapping_squares(source.into_iter(), cloth);
    assert_eq!(4, overlapping);
}

#[test]
fn test_parse_claim() {
    let claim = "#2 @ 32,10: 14x7";
    assert_eq!(Rect::new(2, 32, 10, 14, 7), parse_claim(claim).unwrap());
}