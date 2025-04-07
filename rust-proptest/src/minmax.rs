use proptest::prelude::*;

proptest! {
  /// Given any integer a, min(a, a) should return a.
  #[test]
  fn test1(a: i32) {
    assert_eq!(a, std::cmp::min(a, a));
  }

  /// Given any two integers a and b, max(a, b) should equal max(b, a).
  #[test]
  fn test2(a: i32, b: i32) {
    assert_eq!(std::cmp::max(a, b), std::cmp::max(b, a));
  }

  /// Given any two integers a and b, if min(a, b) returns a, then max(a, b) should return b.
  #[test]
  fn test3(a: i32, b: i32) {
    if std::cmp::min(a, b) == a {
      assert_eq!(b, std::cmp::max(a, b));
    }
  }
}
