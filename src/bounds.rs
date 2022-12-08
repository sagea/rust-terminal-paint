use crate::point::Point;

#[derive(Clone, Copy)]
pub struct Bounds {
  pub size: Point,
  pub offset: Point,
}

impl Bounds {
  pub fn contains_point(&self, point: &Point) -> bool {
    let end = self.size + self.offset;
    if point.x < self.offset.x {
      return false;
    }
    if point.x > end.x {
      return false;
    }
    if point.y < self.offset.y {
      return false;
    }
    if point.y > end.y {
      return false;
    }
    true
  }
}
