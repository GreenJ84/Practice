// Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.

struct Solution {}
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = nums
            .iter()
            .enumerate()
            .map(|(i, _)| (i + 1) as i32)
            .collect::<Vec<i32>>();
        nums.sort();
        ans.retain(|&i| nums.binary_search(&i).is_err());
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        let result: Vec<i32> = Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
        assert_eq!(result, vec![5, 6]);
    }
    #[test]
    fn test_find_disappeared_numbers2() {
        let result: Vec<i32> = Solution::find_disappeared_numbers(vec![1, 1]);
        assert_eq!(result, vec![2]);
    }
}
