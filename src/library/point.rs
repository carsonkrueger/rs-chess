use num_traits::Num;

#[derive(PartialEq)]
pub struct Point<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> From<(T, T)> for Point<T> {
    fn from(points: (T, T)) -> Self {
        Self {
            x: points.0,
            y: points.1,
        }
    }
}
