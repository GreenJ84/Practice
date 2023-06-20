// You are given an array of unique integers salary where salary[i] is the salary of the ith employee.
// Return the average salary of employees excluding the minimum and maximum salary. Answers within 10-5 of the actual answer will be accepted.

struct Solution {}
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = salary[0];
        let mut max = 0;
        let mut ans = 0;
        for i in 1..salary.len() {
            println!("{}, {}, {}", min, max, salary[i]);
            if salary[i] < min {
                if max == 0 {
                    max = min;
                } else {
                    ans += min;
                }
                min = salary[i];
            } else if salary[i] > max {
                ans += max;
                max = salary[i];
            } else {
                ans += salary[i];
            }
        }
        ans as f64 / (salary.len() - 2) as f64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(Solution::average(vec![4000,3000,1000,2000]), 2500.00);
    }

    #[test]
    fn test_average2() {
        assert_eq!(Solution::average(vec![1000,2000,3000]), 2000.00);
    }
}