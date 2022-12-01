use std::ops::Add;

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

// todo: Figure out the right way to handle all the reference cases.
//       This implementation is verbose and rough.
impl Add<Point> for Point {
  type Output = Point;
  fn add(self, rhs: Self) -> Self::Output {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl Add<&Point> for &Point {
  type Output = Point;

  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Add<Point> for &Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Add<&Point> for Point {
  type Output = Point;

  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Add<(u16, u16)> for Point {
  type Output = Point;
  fn add(self, other: (u16, u16)) -> Point {
    Point {
      x: self.x + other.0,
      y: self.y + other.1,
    }
  }
}

impl Add<&(u16, u16)> for &Point {
  type Output = Point;

  fn add(self, other: &(u16, u16)) -> Point {
    Point {
      x: self.x + other.0,
      y: self.y + other.1,
    }
  }
}

impl Add<(u16, u16)> for &Point {
  type Output = Point;
  fn add(self, other: (u16, u16)) -> Point {
    Point {
      x: self.x + other.0,
      y: self.y + other.1,
    }
  }
}

impl Add<&(u16, u16)> for Point {
  type Output = Point;

  fn add(self, other: &(u16, u16)) -> Point {
    Point {
      x: self.x + other.0,
      y: self.y + other.1,
    }
  }
}
// impl<'a, 'b> Add<&'b (i32, i32)> for &'a Point {
//   type Output = Point;

//   fn add(self, other: &'b (i32, i32)) -> Point {
//     Point {
//       x: self.x + other.0 as u16,
//       y: self.y + other.1 as u16,
//     }
//   }
// }
