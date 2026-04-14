// Given a sorted array of distinct integers and a target value, return the index of the target or -1 if not found.

// fn binarySearch(nums: &[i32], target: i32) -> i32 {
fn binary_search(nums: &[i32], target: i32) -> i32 {
    for idx in 0..nums.len() {
        if nums[idx] == target {
            return idx as i32;
        }
    }
    -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1, 2, 3, 4, 5];
        let target = 3;
        assert_eq!(binary_search(&nums, target), 2);
    }

    #[test]
    fn test_2() {
        let nums = [2, 4, 6, 8, 10, 12, 14, 16];
        let target = 16;
        assert_eq!(binary_search(&nums, target), 7);
    }

    #[test]
    fn test_not_found() {
        let nums = [1, 3, 5, 7];
        let target = 4;
        assert_eq!(binary_search(&nums, target), -1);
    }
}
