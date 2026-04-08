// Given an unsorted array of integers, find the smallest positive integer not present in the array in O(n) time and O(1) extra space.

// fn findSmallestMissingPositive(order_numbers: &[i32]) -> i32 {
fn find_smallest_missing_positive(order_numbers: &[i32]) -> i32 {
    let mut order_numbers = order_numbers.to_vec();
    let n = order_numbers.len();

    for i in 0..n {
        while order_numbers[i] > 0
            && (order_numbers[i] as usize) <= n
            && order_numbers[order_numbers[i] as usize - 1] != order_numbers[i]
        {
            let target_idx = order_numbers[i] as usize - 1;
            order_numbers.swap(i, target_idx);
        }
    }

    for i in 0..n {
        if order_numbers[i] != (i as i32 + 1) {
            return i as i32 + 1;
        }
    }
    (n as i32) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let order_numbers = [3, 4, -1, 1];
        assert_eq!(find_smallest_missing_positive(&order_numbers), 2);
    }

    #[test]
    fn test_2() {
        let order_numbers = [1, 2, 0];
        assert_eq!(find_smallest_missing_positive(&order_numbers), 3);
    }
}