use num_traits::Num;

#[derive(PartialEq, Copy, Clone)]
pub struct Point<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

pub struct Dist<T: Num + Copy> {
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

impl Point<u8> {
    pub fn relative_point_dist(&self, point: &Point<u8>) -> Dist<i32> {
        Dist {
            x: self.x as i32 - point.x as i32,
            y: self.y as i32 - point.y as i32,
        }
    }
}
