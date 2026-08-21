// There are n employees in a company, numbered from 0 to n - 1. Each employee i has worked for hours[i] hours in the company.

// The company requires each employee to work for at least target hours.

// You are given a 0-indexed array of non-negative integers hours of length n and a non-negative integer target.

// Return the integer denoting the number of employees who worked at least target hours.

// Constraints:
// 1 <= n == hours.length <= 50
// 0 <= hours[i], target <= 10^5

struct Solution;
impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours
            .iter()
            .fold(0i32, |tot, &h| if h >= target { tot + 1 } else { tot })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let hours = vec![0, 1, 2, 3, 4];
        let target = 2;
        let result = Solution::number_of_employees_who_met_target(hours, target);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let hours = vec![5, 1, 4, 2, 2];
        let target = 6;
        let result = Solution::number_of_employees_who_met_target(hours, target);
        assert_eq!(result, 0);
    }
}
