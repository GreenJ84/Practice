// You are given an integer n.

// We need to group the numbers from 1 to n according to the sum of its digits. For example, the numbers 14 and 5 belong to the same group, whereas 13 and 3 belong to different groups.

// Return the number of groups that have the largest size, i.e. the maximum number of elements.

struct Solution;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in 1..=n {
            let sum = {
                let mut sum = 0;
                let mut num = i;
                while num > 0 {
                    sum += num % 10;
                    num /= 10;
                }
                sum
            };
            *map.entry(sum).or_insert(0) += 1;
        }

        let mut max = 0;
        let mut count = 0;
        for &v in map.values() {
            if v > max {
                max = v;
                count = 1;
            } else if v == max {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_largest_group(13), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_largest_group(2), 2);
    }
}
