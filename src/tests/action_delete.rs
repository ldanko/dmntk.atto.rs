use super::*;
use crate::Plane;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(3, 2);
  plane.delete_character_before_cursor();
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├──┬───────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
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
  eq(plane, expected);
}

#[test]
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(3, 13);
  plane.delete_character_before_cursor();
  let expected = r#"
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
  eq(plane, expected);
}

#[test]
fn _0003() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(6, 14);
  repeat(3, || {
    plane.delete_character_before_cursor();
  });
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬──────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
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
  eq(plane, expected);
}

#[test]
fn _0004() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(3, 9);
  repeat(4, || {
    plane.delete_character_under_cursor();
  });
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Cust      │ Order ╟──────────┬──────────╢ Description │ Reference │
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
  eq(plane, expected);
}

#[test]
fn _0005() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(6, 14);
  plane.delete_character_under_cursor();
  plane.delete_character_under_cursor();
  eq_cursor(7, 14, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬──────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business"│  <10, ║   0.10,  │"Normal", ║             │           │
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
  eq(plane, expected);
}

#[test]
fn _0006() {
  let plane = &mut Plane::new(TEST_INPUT_002);
  eq_cursor(1, 1, plane);
  plane.move_cursor(14, 4);
  plane.delete_character_under_cursor();
  eq_cursor(15, 5, plane);
  let expected = r#"
┌───────────────────────────┐
│ Order options             │
├───┬─┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │ │       ║    Order options    ║             │           │
│   │ │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │ │ size  ║ Discount │ Priority ║             │           │
│   ├─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │ │  <10, ║   0.10,  │"Normal", ║             │           │
│   │ │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │ │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │ │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │ │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │ │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴─┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0007() {
  let plane = &mut Plane::new(TEST_INPUT_003);
  eq_cursor(1, 1, plane);
  plane.move_cursor(6, 34);
  eq_cursor(7, 35, plane);
  plane.delete_character_under_cursor();
  eq_cursor(7, 34, plane);
  let expected = r#"
┌──────────────────────────────────────┐
│ Order options                        │
├───┬───────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
  eq(plane, expected);
}

#[test]
fn _0008() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.move_cursor(3, 46);
  eq_cursor(4, 47, plane);
  plane.delete_character_before_cursor();
  eq_cursor(4, 47, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
  eq(plane, expected);
}
