#[cfg(test)]
mod tests {
    use std::cmp::{min, max};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn min_same_number(a: i32) -> bool {
        min(a, a) == a
    }

    #[quickcheck]
    fn max_commutative(a: i32, b: i32) -> bool {
        max(a, b) == max(b, a)
    }

    #[quickcheck]
    fn min_max_opposite(a: i32, b: i32) -> bool {
        if min(a, b) == a {
            max(a, b) == b
        }
        else {
            max(a, b) == a
        }
    }
}
