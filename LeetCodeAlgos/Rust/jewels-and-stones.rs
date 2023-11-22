// You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

// Letters are case sensitive, so "a" is considered a different type of stone from "A".

struct Solution {}
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        stones.chars()
            .collect::<Vec<char>>()
            .into_iter()
            .filter(|c| jewels.contains(*c)).count() as i32
    }
}

// impl Solution {
//     pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
//         let mut ans = 0;
//         for c in stones.chars() {
//             if jewels.contains(c) {
//                 ans += 1;
//             }
//         }
//         ans
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    }

    #[test]
    fn test_num_jewels_in_stones_2() {
        assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}