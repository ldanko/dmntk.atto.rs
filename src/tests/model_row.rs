use crate::Row;

#[test]
fn _0001() {
  let row = Row::new(vec!['a', 'b', 'c']);
  assert_eq!("abc", row.to_string());
  assert_eq!("abc", format!("{}", row));
}
