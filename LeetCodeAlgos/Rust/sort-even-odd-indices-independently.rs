// You are given a 0-indexed integer array nums. Rearrange the values of nums according to the following rules:

// Sort the values at odd indices of nums in non-increasing order.
// For example, if nums = [4,1,2,3] before this step, it becomes [4,3,2,1] after. The values at odd indices 1 and 3 are sorted in non-increasing order.
// Sort the values at even indices of nums in non-decreasing order.
// For example, if nums = [4,1,2,3] before this step, it becomes [2,1,4,3] after. The values at even indices 0 and 2 are sorted in non-decreasing order.
// Return the array formed after rearranging the values of nums.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n <= 2 {
            return nums;
        } else if n % 2 == 1 {
            nums.push(0);
        }
        let mut odd = Vec::with_capacity(50);
        let mut even = Vec::with_capacity(50);
        for (idx, num) in nums.into_iter().enumerate() {
            match idx {
                i if i % 2 == 0 => {
                    even.push(num);
                }
                _ => {
                    odd.push(num);
                }
            }
        }
        even.sort_unstable();
        odd.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = even
            .into_iter()
            .zip(odd.into_iter())
            .flat_map(|(e, o)| [e, o])
            .collect::<Vec<i32>>();
        return if n % 2 == 0 {
            ans
        } else {
            ans.pop();
            ans
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 1, 2, 3];
        let result = Solution::sort_even_odd(nums);
        assert_eq!(result, vec![2, 3, 4, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 1];
        let result = Solution::sort_even_odd(nums);
        assert_eq!(result, vec![2, 1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![
            5, 39, 33, 5, 12, 27, 20, 45, 14, 25, 32, 33, 30, 30, 9, 14, 44, 15, 21,
        ];
        let result = Solution::sort_even_odd(nums);
        assert_eq!(
            result,
            vec![5, 45, 9, 39, 12, 33, 14, 30, 20, 27, 21, 25, 30, 15, 32, 14, 33, 5, 44]
        );
    }
}
