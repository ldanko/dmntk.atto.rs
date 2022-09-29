use super::*;
use crate::Plane;

#[test]
fn _0001() {
  let plane = Plane::new(EX_001);
  assert_eq!(EX_001.trim(), plane.to_string());
  assert_eq!(EX_001.trim(), format!("{}", plane));
}
