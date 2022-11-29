pub trait Point {
  fn is_inbetween(&self, a: (u16, u16), b: (u16, u16)) -> bool;
}

impl Point for (u16, u16) {
  fn is_inbetween(&self, a: (u16, u16), b: (u16, u16)) -> bool {
    if self.0 < a.0 {
      return false;
    }
    if self.0 > b.0 {
      return false;
    }
    if self.1 < a.1 {
      return false;
    }
    if self.1 > b.1 {
      return false;
    }
    return true;
  }
}
