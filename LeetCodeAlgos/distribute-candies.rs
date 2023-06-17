// Alice has n candies, where the ith candy is of type candyType[i]. Alice noticed that she started to gain weight, so she visited a doctor.
// The doctor advised Alice to only eat n / 2 of the candies she has (n is always even). Alice likes her candies very much, and she wants to eat the maximum number of different types of candies while still following the doctor's advice.
// Given the integer array candyType of length n, return the maximum number of different types of candies she can eat if she only eats n / 2 of them.

use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let max: i32 = (candy_type.len() / 2) as i32;
        let mut count: i32 = 0;
        for candy in candy_type.iter() {
            if !set.contains(candy) {
                set.insert(*candy);
                count += 1;
            }
            if count >= max { break; }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn test_distribute_candies_2() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    }

    #[test]
    fn test_distribute_candies_3() {
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}