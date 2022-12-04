use impl_ops::impl_op;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    true
  }

  pub fn zero() -> Self {
    Point::from(0)
  }
}
type TuplePoint = (u16, u16);
impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point { x: a.x + b.x, y: a.y + b.y } });
impl_op_ex!(+ |a: &Point, b: &TuplePoint| -> Point { Point { x: a.x + b.0, y: a.y + b.1 } });
impl_op_ex!(+ |a: &TuplePoint, b: &Point| -> Point { Point { x: a.0 + b.x, y: a.1 + b.y } });

impl_op_ex!(-|a: &Point, b: &Point| -> Point {
  Point {
    x: a.x - b.x,
    y: a.y - b.y,
  }
});
impl_op_ex!(-|a: &Point, b: &TuplePoint| -> Point {
  Point {
    x: a.x - b.0,
    y: a.y - b.1,
  }
});
impl_op_ex!(-|a: &TuplePoint, b: &Point| -> Point {
  Point {
    x: a.0 - b.x,
    y: a.1 - b.y,
  }
});

#[cfg(test)]
mod tests {
  use crate::point::Point;
  #[allow(clippy::op_ref)]
  #[test]
  fn it_should_add_two_points_together() {
    let a = Point::new(1, 2);
    let b = Point::new(4, 5);
    let result = Point::new(5, 7);
    assert_eq!(a + b, result);
    assert_eq!(&a + &b, result);
    assert_eq!(&a + b, result);
    assert_eq!(a + &b, result);
  }
  #[test]
  fn it_should_add_a_u16_tutple_to_a_point() {
    let a = Point::new(1, 2);
    let b = (4, 5);
    let result = Point::new(5, 7);
    assert_eq!(a + b, result);
    assert_eq!(a + &b, result);
    assert_eq!(a + &b, result);
    assert_eq!(&a + b, result);
    assert_eq!(b + a, result);
    assert_eq!(&b + &a, result);
    assert_eq!(&b + a, result);
    assert_eq!(b + &a, result);
  }
}
