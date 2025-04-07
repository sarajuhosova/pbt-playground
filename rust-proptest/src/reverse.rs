use proptest::prelude::*;

proptest! {
  /// Reversing a list with one element results in the same list.
  #[test]
  fn test1(a: i32) {
    let list = vec![a];
    let mut reversed = list.clone();
    reversed.reverse();

    assert_eq!(list, reversed);
  }

  /// Reversing a list twice results in the original list.
  #[test]
  fn test2(list: Vec<String>) {
    let mut reversed = list.clone();
    reversed.reverse();
    reversed.reverse();

    assert_eq!(list, reversed);
  }

  /// The length of a reversed list is the same as the length of the original list.
  #[test]
  fn test3(list: Vec<f64>) {
    let mut reversed = list.clone();
    reversed.reverse();

    assert_eq!(list.len(), reversed.len());
  }
}
