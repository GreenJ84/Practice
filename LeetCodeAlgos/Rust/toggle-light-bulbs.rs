// You are given an array bulbs of integers between 1 and 100.

// There are 100 light bulbs numbered from 1 to 100. All of them are switched off initially.

// For each element bulbs[i] in the array bulbs:

// If the bulbs[i]th light bulb is currently off, switch it on.
// Otherwise, switch it off.
// Return the list of integers denoting the light bulbs that are on in the end, sorted in ascending order. If no bulb is on, return an empty list.

// Constraints:
// 1 <= bulbs.length <= 100
// 1 <= bulbs[i] <= 100

struct Solution;
impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut check = vec![0; 100];
        for &bulb in &bulbs {
            check[(bulb - 1) as usize] += 1;
        }
        check
            .into_iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                if b % 2 == 1 {
                    Some((idx + 1) as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let bulbs = vec![10, 30, 20, 10];
        let expected = vec![20, 30];
        assert_eq!(Solution::toggle_light_bulbs(bulbs), expected);
    }

    #[test]
    fn test_2() {
        let bulbs = vec![100, 100];
        let expected: Vec<i32> = vec![];
        assert_eq!(Solution::toggle_light_bulbs(bulbs), expected);
    }

    #[test]
    fn test_3() {
        let bulbs = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::toggle_light_bulbs(bulbs), expected);
    }
}
