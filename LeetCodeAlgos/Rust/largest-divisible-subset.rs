// Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
// answer[i] % answer[j] == 0, or answer[j] % answer[i] == 0
// If there are multiple solutions, return any of them.

struct Solution {}
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let n = nums.len();
        let mut dp = vec![vec![]; n];
        // Go through and find the matching divisible numbers (j) for each number (i)
        for i in 0..n {
            dp[i] = vec![nums[i]];

            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if dp[j].len() >= dp[i].len() {
                        let mut tmp = dp[j].clone();
                        tmp.push(nums[i]);
                        dp[i] = tmp;
                    }
                }
            }
        }
        print!("{:?}", dp);
        dp.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        dp.first().unwrap().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisible_subset() {
        let result: Vec<i32> = Solution::largest_divisible_subset(vec![1, 2, 3]);
        assert!(result == vec![1, 2] || result == vec![1, 3]);
    }

    #[test]
    fn test_largest_divisible_subset2() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
            vec![1, 2, 4, 8]
        );
    }
}
