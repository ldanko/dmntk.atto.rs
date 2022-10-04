use super::*;
use crate::Plane;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_cell_end();
  eq_cursor(1, 37, plane);
}

#[test]
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  eq_cursor(3, 1, plane);
  plane.cursor_move_cell_right();
  eq_cursor(3, 5, plane);
}
