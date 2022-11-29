pub fn plot_line_low(x0: i16, y0: i16, x1: i16, y1: i16) -> Vec<(u16, u16)> {
  let mut pts = vec![];
  let dx = x1 - x0;
  let mut dy = y1 - y0;
  let mut yi = 1;
  if dy < 0 {
    yi = -1;
    dy = -dy;
  }
  let mut d = (2 * dy) - dx;
  let mut y = y0;

  for x in x0..x1 {
    pts.push((x as u16, y as u16));
    if d > 0 {
      y = y + yi;
      d = d + (2 * (dy - dx));
    } else {
      d = d + 2 * dy;
    }
  }
  pts
}

pub fn plot_line_high(x0: i16, y0: i16, x1: i16, y1: i16) -> Vec<(u16, u16)> {
  let mut pts = vec![];
  let mut dx = x1 - x0;
  let dy = y1 - y0;
  let mut xi = 1;
  if dx < 0 {
    xi = -1;
    dx = -dx;
  }
  let mut d = (2 * dx) - dy;
  let mut x = x0;

  for y in y0..y1 {
    pts.push((x as u16, y as u16));
    if d > 0 {
      x = x + xi;
      d = d + (2 * (dx - dy));
    } else {
      d = d + 2 * dx
    }
  }
  pts
}

pub fn plot_line(start: (u16, u16), end: (u16, u16)) -> Vec<(u16, u16)> {
  let x0 = start.0 as i16;
  let y0 = start.1 as i16;
  let x1 = end.0 as i16;
  let y1 = end.1 as i16;
  if x0 == x1 && y0 == y1 {
    return vec![start.clone()];
  }
  if (y1 - y0).abs() < (x1 - x0).abs() {
    if x0 > x1 {
      return plot_line_low(x1, y1, x0, y0);
    } else {
      return plot_line_low(x0, y0, x1, y1);
    }
  } else {
    if y0 > y1 {
      plot_line_high(x1, y1, x0, y0)
    } else {
      plot_line_high(x0, y0, x1, y1)
    }
  }
}
