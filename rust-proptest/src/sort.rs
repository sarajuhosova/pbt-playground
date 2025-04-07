use proptest::prelude::*;

proptest! {
  /// Sort i32s.
  #[test]
  fn test1(list: Vec<i32>) {
    let mut sorted_1 = list.clone();
    let mut sorted_2 = list.clone();

    mergesort(&mut sorted_1);
    quicksort(&mut sorted_2);

    assert_eq!(sorted_1, sorted_2);
  }

  /// Sort f64s
  #[test]
  fn test2(list: Vec<f64>) {
    let mut sorted_1 = list.clone();
    let mut sorted_2 = list.clone();

    mergesort(&mut sorted_1);
    quicksort(&mut sorted_2);

    assert_eq!(sorted_1, sorted_2);
  }

  /// Sort Strings
  #[test]
  fn test3(list: Vec<String>) {
    let mut sorted_1 = list.clone();
    let mut sorted_2 = list.clone();

    mergesort(&mut sorted_1);
    quicksort(&mut sorted_2);

    assert_eq!(sorted_1, sorted_2);
  }
}

#[allow(dead_code)]
fn mergesort<T: Clone + PartialOrd>(arr: &mut [T]) {
  let len = arr.len();
  if len <= 1 {
    return;
  }

  let mid = len / 2;
  mergesort(&mut arr[0..mid]);
  mergesort(&mut arr[mid..len]);

  let left = arr[0..mid].to_vec();
  let right = arr[mid..len].to_vec();

  let mut i = 0; // Index for left
  let mut j = 0; // Index for right
  let mut k = 0; // Index for the original array

  while i < left.len() && j < right.len() {
    if left[i] <= right[j] {
      arr[k] = left[i].clone();
      i += 1;
    } else {
      arr[k] = right[j].clone();
      j += 1;
    }
    k += 1;
  }

  while i < left.len() {
    arr[k] = left[i].clone();
    i += 1;
    k += 1;
  }

  while j < right.len() {
    arr[k] = right[j].clone();
    j += 1;
    k += 1;
  }
}

#[allow(dead_code)]
pub fn quicksort<T: PartialOrd>(slice: &mut [T]) {
  if slice.len() <= 1 {
    return;
  }
  let pivot_index = partition(slice);
  let (left, right) = slice.split_at_mut(pivot_index);
  quicksort(left);
  quicksort(&mut right[1..]);
}

fn partition<T: PartialOrd>(slice: &mut [T]) -> usize {
  let len = slice.len();
  let mid = len / 2;
  slice.swap(mid, len - 1);
  let pivot_index = len - 1;
  let mut i = 0;
  for j in 0..pivot_index {
    if slice[j] <= slice[pivot_index] {
      slice.swap(i, j);
      i += 1;
    }
  }
  slice.swap(i, pivot_index);
  i
}
