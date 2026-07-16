// You are given two integer arrays nums and divisors.

// The divisibility score of divisors[i] is the number of indices j such that nums[j] is divisible by divisors[i].

// Return the integer divisors[i] with the maximum divisibility score. If multiple integers have the maximum score, return the smallest one.

// Constraints:
// 1 <= nums.length, divisors.length <= 1000
// 1 <= nums[i], divisors[i] <= 109

struct Solution;
impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_div = (0, i32::MAX); // score, divisor
        for div in &divisors {
            let mut score = 0;
            for num in &nums {
                if num % div == 0 {
                    score += 1;
                }
            }
            if score > max_div.0 || (score == max_div.0 && div < &max_div.1) {
                max_div = (score, *div);
            }
        }
        max_div.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 9, 15, 50];
        let divisors = vec![5, 3, 7, 2];
        assert_eq!(Solution::max_div_score(nums, divisors), 2);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 7, 9, 3, 9];
        let divisors = vec![5, 2, 3];
        assert_eq!(Solution::max_div_score(nums, divisors), 3);
    }

    #[test]
    fn test3() {
        let nums = vec![20, 14, 21, 10];
        let divisors = vec![10, 16, 20];
        assert_eq!(Solution::max_div_score(nums, divisors), 10);
    }
}
