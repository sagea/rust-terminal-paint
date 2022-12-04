use crate::point::Point;

pub fn get_increment(iterations: f32, a: f32, b: f32) -> f32 {
  let diff = b - a;
  diff / iterations
}

pub fn plot_line(start: Point, end: Point) -> Vec<Point> {
  let x0 = start.x as f32;
  let y0 = start.y as f32;
  let x1 = end.x as f32;
  let y1 = end.y as f32;
  let mut list = vec![];
  if x0 == x1 && y0 == y1 {
    return vec![start];
  }
  let x_iterations = (x0 - x1).abs();
  let y_iterations = (y0 - y1).abs();
  let total_iterations = x_iterations.max(y_iterations);
  let x_slope = get_increment(total_iterations, x0, x1);
  let y_slope = get_increment(total_iterations, y0, y1);
  let mut i = 0;
  while i <= total_iterations as i32 {
    let x = x0 + (i as f32 * x_slope).round();
    let y = y0 + (i as f32 * y_slope).round();

    list.push(Point::new(x as u16, y as u16));
    i += 1;
  }
  list
}

#[cfg(test)]
mod tests {
  use super::plot_line;
  use crate::point::Point;

  #[test]
  pub fn should_draw_line_inbetween_points_1() {
    let start = Point::new(60, 29);
    let end = Point::new(62, 27);
    let result = plot_line(start, end);
    let expected_result = vec![Point::new(60, 29), Point::new(61, 28), Point::new(62, 27)];

    assert_eq!(result, expected_result, "Should have been equal");
  }
  #[test]
  pub fn should_draw_line_inbetween_points_2() {
    let start = Point::new(50, 50);
    let end = Point::new(53, 53);
    let result = plot_line(start, end);
    let expected_result = vec![
      Point::new(50, 50),
      Point::new(51, 51),
      Point::new(52, 52),
      Point::new(53, 53),
    ];

    assert_eq!(result, expected_result, "Should have been equal");
  }

  #[test]
  pub fn should_draw_line_inbetween_points_3() {
    let start = Point::new(50, 50);
    let end = Point::new(53, 50);
    let result = plot_line(start, end);
    let expected_result = vec![
      Point::new(50, 50),
      Point::new(51, 50),
      Point::new(52, 50),
      Point::new(53, 50),
    ];

    assert_eq!(result, expected_result, "Should have been equal");
  }

  #[test]
  pub fn should_draw_line_inbetween_points_4() {
    let start = Point::new(50, 50);
    let end = Point::new(50, 50);
    let result = plot_line(start, end);
    let expected_result = vec![Point::new(50, 50)];

    assert_eq!(result, expected_result, "Should have been equal");
  }

  #[test]
  pub fn should_draw_line_inbetween_points_5() {
    let start = Point::new(50, 50);
    let end = Point::new(55, 51);
    let result = plot_line(start, end);
    let expected_result = vec![
      Point::new(50, 50),
      Point::new(51, 50),
      Point::new(52, 50),
      Point::new(53, 51),
      Point::new(54, 51),
      Point::new(55, 51),
    ];

    assert_eq!(result, expected_result, "Should have been equal");
  }
}
