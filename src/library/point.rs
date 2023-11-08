use num_traits::Num;

#[derive(PartialEq, Copy, Clone)]
pub struct Point<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> From<(T, T)> for Point<T> {
    fn from(points: (T, T)) -> Self {
        Self {
            x: points.0,
            y: points.1,
        }
    }
}

impl<T: Num + Copy> Point<T> {
    pub fn relative_point_dist(&self, point: &Point<T>) -> Point<T> {
        Point {
            x: self.x - point.x,
            y: self.y - point.y,
        }
    }
}
