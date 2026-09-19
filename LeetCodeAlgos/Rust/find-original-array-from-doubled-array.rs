// An integer array original is transformed into a doubled array changed by appending twice the value of every element in original, and then randomly shuffling the resulting array.

// Given an array changed, return original if changed is a doubled array. If changed is not a doubled array, return an empty array. The elements in original may be returned in any order.

// Constraints:
// 1 <= changed.length <= 10^5
// 0 <= changed[i] <= 10^5

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let n = changed.len();
        if n % 2 != 0 {
            return vec![];
        }

        let mut freq = HashMap::<i32, i32>::new();
        for num in changed.into_iter() {
            *freq.entry(num).or_insert(0) += 1;
        }
        let mut changed = freq.into_iter().collect::<Vec<(i32, i32)>>();
        changed.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut original = vec![];
        let (mut o_v, mut c_v) = (0usize, 0usize);
        let mut found = (n / 2) as i32;
        if changed[0].0 == 0 {
            if changed[0].1 % 2 != 0 {
                return vec![];
            }
            original.extend(vec![0; (changed[0].1 / 2) as usize]);
            found -= changed[0].1 / 2;
            changed[0].1 = 0;
        }
        while found > 0 {
            while changed[o_v].1 == 0 {
                o_v += 1;
            }
            let (o_val, o_freq) = changed[o_v];

            while changed[c_v].0 < o_val * 2 {
                c_v += 1;
                if c_v == changed.len() {
                    return vec![];
                }
            }
            if changed[c_v].0 > o_val * 2 || changed[c_v].1 < o_freq {
                return vec![];
            }

            original.extend(vec![o_val; o_freq as usize]);
            changed[o_v].1 -= o_freq;
            changed[c_v].1 -= o_freq;
            found -= o_freq;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let changed = vec![1, 3, 4, 2, 6, 8];
        let expected = vec![1, 3, 4];
        let result = Solution::find_original_array(changed);
        assert_eq!(result.len(), expected.len());
        for num in expected {
            assert!(result.contains(&num));
        }
    }

    #[test]
    fn test_2() {
        let changed = vec![6, 3, 0, 1];
        let expected: Vec<i32> = vec![];
        let result = Solution::find_original_array(changed);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let changed = vec![1];
        let expected: Vec<i32> = vec![];
        let result = Solution::find_original_array(changed);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let changed = vec![0, 0];
        let expected: Vec<i32> = vec![0];
        let result = Solution::find_original_array(changed);
        assert_eq!(result, expected);
    }
}
