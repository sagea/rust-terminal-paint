use std::{ops::Add};

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: u16,
  pub y: u16,
}

impl Point {
  pub fn new(x: u16, y: u16) -> Self {
    Point { x, y }
  }
  pub fn from(num: u16) -> Self {
    Point::new(num, num)
  }

  pub fn from_tuple(item: &(u16, u16)) -> Self {
    Point::new(item.0, item.1)
  }

  pub fn is_inbetween(&self, a: Point, b: Point) -> bool {
    if self.x < a.x {
      return false;
    }
    if self.x > b.x {
      return false;
    }
    if self.y < a.y {
      return false;
    }
    if self.y > b.y {
      return false;
    }
    return true;
  }
}

impl Add for Point {
  type Output = Point;
  fn add(self, rhs: Self) -> Self::Output {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
  type Output = Point;

  fn add(self, other: &'b Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
