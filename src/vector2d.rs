use std::ops;

pub struct Vector2d<T>
where
    T: ops::Add + ops::Mul,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2d<T>
where
    T: ops::Add + ops::Mul,
{
    pub fn new(x: T, y: T) -> Vector2d<T> {
        Vector2d { x, y }
    }
}

impl<T> ops::Add for Vector2d<T>
where
    T: ops::Add<Output = T> + ops::Mul,
{
    type Output = Vector2d<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::Mul for Vector2d<T>
where
    T: ops::Mul<Output = T> + ops::Add,
{
    type Output = Vector2d<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2d::new(self.x * rhs.x, self.y * rhs.y)
    }
}
