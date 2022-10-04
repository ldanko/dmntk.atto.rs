mod action_delete;
mod action_insert;
mod action_move;
mod model_plane;
mod model_row;

use crate::Plane;
use difference::Changeset;

macro_rules! repeat {
  ($c:expr, $p:expr, $f:tt) => {{
    rep_op($c, || {
      $p.$f();
    });
  }};
}

pub(crate) use repeat;

/// Utility function for comparing plane with expected decision table.
fn eq(plane: &Plane, decision_table: &str) {
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
  assert_eq!(row, plane.cursor_row());
  assert_eq!(col, plane.cursor_col());
}

/// Utility function for repeating operations.
fn rep_op<F>(n: usize, mut f: F)
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
