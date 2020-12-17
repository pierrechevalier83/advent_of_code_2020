pub trait PointND: Eq + PartialEq + Clone + Copy + std::hash::Hash + Sized {
    fn from_2d(x: isize, y: isize) -> Self;

    fn neighbours(&self) -> Vec<Self>;
}
