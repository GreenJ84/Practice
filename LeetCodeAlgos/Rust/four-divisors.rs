// Given an integer array nums, return the sum of divisors of the integers in that array that have exactly four divisors. If there is no such integer in the array, return 0.

// Constraints:
// 1 <= nums.length <= 104
// 1 <= nums[i] <= 105

struct Solution;
impl Solution {
    pub fn sum_four_divisors(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let (mut sum, mut last) = (0, 0);
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                sum += last;
                continue;
            }
            last = Self::divisors(nums[i]);
            sum += last;
        }
        sum
    }

    fn divisors(num: i32) -> i32 {
        let (mut sum, mut div) = (num + 1, 2);
        for n in 2..=num.isqrt() {
            if num % n == 0 {
                if div == 4 || n * n == num {
                    return 0;
                }
                div += 2;
                sum += n + num / n;
            }
        }
        if div == 4 {
            sum
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![21, 4, 7];
        assert_eq!(Solution::sum_four_divisors(nums), 32);
    }

    #[test]
    fn test_2() {
        let nums = vec![21, 21];
        assert_eq!(Solution::sum_four_divisors(nums), 64);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::sum_four_divisors(nums), 0);
    }

    #[test]
    fn test_4() {
        let nums = vec![1,2,3,4,5,6,7,8,9,10];
        assert_eq!(Solution::sum_four_divisors(nums), 45);
    }
}
