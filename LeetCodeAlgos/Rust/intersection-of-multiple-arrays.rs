// Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, return the list of integers that are present in each array of nums sorted in ascending order.

struct Solution;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut freq = std::collections::HashMap::<>::with_capacity(nums[0].len());
        for num in &nums[0]{
            freq.insert(*num, 1usize);
        }
        for i in 1..nums.len() {
            for num in &nums[i] {
                if let Some(count) = freq.get_mut(num) {
                    if *count == i {
                        *count += 1;
                    }
                }
            }
        }

        let mut ans = freq.into_iter()
            .filter(|(_, count)| *count == nums.len())
            .map(|(num, _)| num)
            .collect::<Vec<i32>>();
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![vec![3,1,2,4,5],vec![1,2,3,4],vec![3,4,5,6]];
        let expected = vec![3,4];
        assert_eq!(Solution::intersection(nums), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![vec![1,2,3],vec![4,5,6]];
        let expected: Vec<i32> = vec![];
        assert_eq!(Solution::intersection(nums), expected);
    }
}

// Example 1:

// Input: nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]
// Output: [3,4]
// Explanation: 
// The only integers present in each of nums[0] = [3,1,2,4,5], nums[1] = [1,2,3,4], and nums[2] = [3,4,5,6] are 3 and 4, so we return [3,4].
// Example 2:

// Input: nums = [[1,2,3],[4,5,6]]
// Output: []
// Explanation: 
// There does not exist any integer present both in nums[0] and nums[1], so we return an empty list [].
