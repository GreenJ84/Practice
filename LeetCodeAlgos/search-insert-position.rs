// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start: i32 = 0;
        let mut end: i32 = nums.len() as i32 - 1;
        // Check boundaries
        if target < nums[start as usize] {
            return 0;
        } else if target > nums[end as usize] {
            return nums.len() as i32;
        }
        // Start searching
        while start <= end {
            let mid: usize = ((start + end) / 2) as usize;
            if target == nums[mid] {
                return mid as i32;
            } else if nums[mid] < target {
                if nums[mid + 1] > target {
                    return (mid + 1) as i32;
                }
                start = (mid + 1) as i32;
            } else {
                if nums[mid - 1] < target {
                    return (mid) as i32;
                }
                end = (mid - 1) as i32;
            }
        }
        0
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1,3,5,6], 5));
    println!("{}", Solution::search_insert(vec![1,3,5,6], 2));
    println!("{}", Solution::search_insert(vec![1,3,5,6], 7));
    println!("{}", Solution::search_insert(vec![1], 1));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
    }

    #[test]
    fn test_search_insert2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_search_insert3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_search_insert4() {
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
    }
}