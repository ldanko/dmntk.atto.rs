use super::*;
use crate::Plane;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cell_end();
  eq_cursor(1, 37, plane);
}

#[test]
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(2, 0);
  eq_cursor(3, 1, plane);
  plane.move_cell_next();
  eq_cursor(3, 5, plane);
}
