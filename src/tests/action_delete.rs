use crate::tests::EX_001;
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
  plane.delete_character(true);
  assert_eq!(EXPECTED_0001.trim(), plane.to_string());
}
