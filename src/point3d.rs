use crate::pointnd::PointND;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl PointND for Point {
    fn from_2d(x: isize, y: isize) -> Self {
        Self::new(x, y, 0)
    }
    fn neighbours(&self) -> Vec<Self> {
        (-1..=1)
            .flat_map(move |x| {
                (-1..=1).flat_map(move |y| (-1..=1).map(move |z| *self + Self::new(x, y, z)))
            })
            .filter(move |x| *x != *self)
            .collect()
    }
}
