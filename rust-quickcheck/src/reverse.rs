#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    fn reverse<T: Clone>(list: &[T]) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        for i in (0..list.len()).rev() {
            result.push(list[i].clone());
        }
        result
    }

    #[quickcheck]
    fn reverse_one_element(elem: i32) -> bool {
        let vec = vec![elem];
        reverse(&vec) == vec
    }

    #[quickcheck]
    fn reverse_twice_identity(list: Vec<i32>) -> bool {
        reverse(&reverse(&list)) == list
    }

    #[quickcheck]
    fn reverse_same_length(list: Vec<i32>) -> bool {
        reverse(&list).len() == list.len()
    }
}
