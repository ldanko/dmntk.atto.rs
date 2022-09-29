use super::*;
use crate::Plane;

const EXPECTED_0001: &str = r#"
┌────────────────────────────────────┐
│ Order options                      │
├──┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U│           │       ║    Order options    ║             │           │
│  │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
│  │   type    │ size  ║ Discount │ Priority ║             │           │
│  ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│  │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│  │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│  │           │       ║   0.05   │ "Low"    ║             │           │
╞══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1│"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2│"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3│"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└──┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0001() {
  let mut plane = Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_x());
  assert_eq!(1, plane.cur_screen_y());
  plane.move_cursor(3, 2);
  plane.delete_character_before();
  eq(EXPECTED_0001, &plane);
}

const EXPECTED_0002: &str = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Custome   │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount │ Priority ║             │           │
│   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │           │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0002() {
  let mut plane = Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_x());
  assert_eq!(1, plane.cur_screen_y());
  plane.move_cursor(3, 13);
  plane.delete_character_before();
  eq(EXPECTED_0002, &plane);
}

const EXPECTED_0003: &str = r#"
┌────────────────────────────────────┐
│ Order options                      │
├───┬──────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Busine,  │  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private" │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │          │       ║   0.05   │ "Low"    ║             │           │
╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business"│ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private" │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

#[test]
fn _0003() {
  let mut plane = Plane::new(EX_001);
  assert_eq!(1, plane.cur_screen_x());
  assert_eq!(1, plane.cur_screen_y());
  plane.move_cursor(6, 14);
  for _ in 0..3 {
    plane.delete_character_before();
  }
  eq(EXPECTED_0003, &plane);
}
