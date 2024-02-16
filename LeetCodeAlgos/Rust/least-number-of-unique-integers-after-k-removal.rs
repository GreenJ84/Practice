// Given an array of integers arr and an integer k. Find the least number of unique integers after removing exactly k elements.

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        arr.iter().for_each(|x| {
            *frequency_map.entry(*x).or_insert(0) += 1;
        });

        let mut frequencies = frequency_map.iter().collect::<Vec<_>>();
        let mut ans = frequencies.len() as i32;
        frequencies.sort_by(|a, b| a.1.cmp(&b.1));

        for (_, count) in frequencies.iter() {
            if **count > k {
                return ans;
            } else {
                k -= **count as i32;
                ans -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_least_num_of_unique_ints() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    }

    #[test]
    fn test_find_least_num_of_unique_ints2() {
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }
}
