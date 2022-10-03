mod action_delete;
mod action_insert;
mod action_move;
mod model_plane;
mod model_row;

use crate::Plane;
use difference::Changeset;

/// Utility function for comparing plane with expected decision table.
fn eq(decision_table: &str, plane: &Plane) {
  let expected = decision_table.trim();
  let actual = plane.to_string();
  if expected != actual {
    println!("expected:\n{}", expected);
    println!("actual:\n{}", actual);
    println!("DIFF:\n{}", Changeset::new(expected, &actual, ""));
  }
  assert_eq!(expected, actual);
}

/// Utility function for comparing screen cursor position.
fn eq_cursor(row: i32, col: i32, plane: &Plane) {
  assert_eq!(row, plane.cur_screen_row());
  assert_eq!(col, plane.cur_screen_col());
}

/// Utility function for repeating operations.
fn repeat<F>(n: usize, mut f: F)
where
  F: FnMut(),
{
  for _ in 0..n {
    f();
  }
}

//--------------------------------------------------------------------------------------------------
// Decision table examples used as input.
//--------------------------------------------------------------------------------------------------

const TEST_INPUT_001: &str = r#"
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

const TEST_INPUT_002: &str = r#"
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
│ 3 │"│   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴─┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;

const TEST_INPUT_003: &str = r#"
┌──────────────────────────────────────┐
│ Order options                        │
├───┬───────────┬───────╥──────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options     ║             │           │
│   │ Customer  │ Order ╟───────────┬──────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount  │ Priority ║             │           │
│   ├───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,   │"Normal", ║             │           │
│   │"Private"  │ >=10  ║   0.15,   │ "High",  ║             │           │
│   │           │       ║   0.05    │ "Low"    ║             │           │
╞═══╪═══════════╪═══════╬═══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10    │ "Normal" ║ Small order │   Ref 1   │
├───┼───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15    │ "High"   ║ Large order │   Ref 2   │
├───┼───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05    │ "Low"    ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨───────────┴──────────╨─────────────┴───────────┘
"#;
