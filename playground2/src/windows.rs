#![allow(unused)]

use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
// #[derive(Debug, Clone)]
struct Coordinate(i32, i32);

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn main() {
    println!("Test windows");
    let slice = [0, 1, 2, 3];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &[0, 1]);
    assert_eq!(iter.next().unwrap(), &[1, 2]);
    assert_eq!(iter.next().unwrap(), &[2, 3]);
    assert!(iter.next().is_none());

    let points: Vec<Coordinate> = vec![
        Coordinate(1, 2),
        Coordinate(5, -4),
        Coordinate(-10, 22),
        Coordinate(111, 233),
    ];

    println!("- Using a classic for loop");
    let mut differences = Vec::new();
    for i in 1..points.len() {
        let current = &points[i];
        let previous = &points[i-1];
        differences.push(*current - *previous)
    }
    println!("Coordinates: {points:?}");
    println!("Coordinates differences: {differences:?}");

    println!("- Using windows() in for loop");
    let mut differences = Vec::new();
    for window in points.windows(2) {
        differences.push(window[1] - window[0]);
    }
    println!("Coordinates: {points:?}");
    println!("Coordinates differences: {differences:?}");

    println!("- Using windows() and iterators");
    let mut differences = points
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    println!("Coordinates: {points:?}");
    println!("Coordinates differences: {differences:?}");

    println!("- Using array_windows() in for loop (nightly feature)");
    let mut differences = Vec::new();
    for [previous, current] in points.array_windows().copied() {
        differences.push(current - previous);
    }
    println!("Coordinates: {points:?}");
    println!("Coordinates differences: {differences:?}");

    println!("- Using array_windows() and iterators (nightly feature)");
    let mut differences: Vec<_> = points
        .array_windows()
        .copied()
        .map(|[previous, current]| current - previous)
        .collect();
    println!("Coordinates: {points:?}");
    println!("Coordinates differences: {differences:?}");

}
