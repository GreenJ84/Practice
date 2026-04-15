// Given a sorted array of integers that may contain duplicates, return the index of the first occurrence of a target value or -1 if not found.

// fn findFirstOccurrence(nums: &[i32], target: i32) -> i32 {
fn find_first_occurrence(nums: &[i32], target: i32) -> i32 {
    let mut idx = 0;
    while idx < nums.len() {
        if nums[idx] == target {
            return idx as i32;
        }
        idx += 1;
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
        let expected = 2;
        assert_eq!(find_first_occurrence(&nums, target), expected);
    }

    #[test]
    fn test_2() {
        let nums = [1, 2, 2, 3, 4];
        let target = 2;
        let expected = 1;
        assert_eq!(find_first_occurrence(&nums, target), expected);
    }

    #[test]
    fn test_3() {
        let nums = [1, 2, 3, 4, 5];
        let target = 6;
        let expected = -1;
        assert_eq!(find_first_occurrence(&nums, target), expected);
    }
}

// Example

// Input:

// nums = [1, 2, 3, 4, 5]
// target = 3
// Output:

// 2
// Explanation:

// We perform binary search on [1,2,3,4,5].

// low=0, high=4 → mid=2 → nums[2]=3 equals target. Record result=2, then search left half.
// Update high=mid-1=1. Now low=0, high=1 → mid=0 → nums[0]=1 < target, so move low to mid+1=1.
// low=1, high=1 → mid=1 → nums[1]=2 < target, so move low to mid+1=2.
// Now low(2)>high(1), terminate. The first occurrence found is at index 2.