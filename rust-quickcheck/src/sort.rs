#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    fn insertion_sort<T: Clone + Ord>(arr: &[T]) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        'outer: for i in 0..arr.len() {
            let elem = arr[i].clone();
            for j in 0..i {
                if elem < result[j] {
                    result.insert(j, elem);
                    continue 'outer
                }
            }
            result.push(elem);
        }
        result
    }

    fn merge_sort<T: Clone + Ord>(arr: &[T]) -> Vec<T> {
        if arr.len() <= 1 {
            return Vec::from(arr);
        }
        let left = merge_sort(&arr[.. arr.len() / 2]);
        let right = merge_sort(&arr[arr.len() / 2 ..]);
        let mut result: Vec<T> = Vec::new();

        let (mut i, mut j) = (0, 0);
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                result.push(left[i].clone());
                i += 1;
            }
            else {
                result.push(right[j].clone());
                j += 1;
            }
        }
        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);
        result
    }

    #[quickcheck]
    fn sort_algorithms_identical(list: Vec<i32>) -> bool {
        insertion_sort(&list) == merge_sort(&list)
    }
}
