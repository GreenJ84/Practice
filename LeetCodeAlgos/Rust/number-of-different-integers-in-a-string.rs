// You are given a string word that consists of digits and lowercase English letters.

// You will replace every non-digit character with a space. For example, "a123bc34d8ef34" will become " 123  34 8  34". Notice that you are left with some integers that are separated by at least one space: "123", "34", "8", and "34".

// Return the number of different integers after performing the replacement operations on word.

// Two integers are considered different if their decimal representations without any leading zeros are different.

struct Solution;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut unique = std::collections::HashSet::<String>::new();

        let mut curr = String::new();
        for ch in word.chars() {
          match ch.is_ascii_digit() {
            true => {
              match (curr == "0", ch == '0'){
                (true, true) => { continue; },
                (true, false) => { curr.pop(); },
                (_, _) => {},
              }
              curr.push(ch);
            },
            false => {
              if !curr.is_empty() && !unique.contains(&curr) {
                unique.insert(curr.clone());
              }
              curr.clear();
            }
          }
        }
        if !curr.is_empty() && !unique.contains(&curr) {
          return unique.len() as i32 + 1;
        }
        unique.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::num_different_integers("a123bc34d8ef34".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::num_different_integers("leet1234code234".to_string()),
            2
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::num_different_integers("a1".to_string()), 1);
    }

    #[test]
    fn no_digits() {
        assert_eq!(Solution::num_different_integers("abc".to_string()), 0);
    }

    #[test]
    fn only_digits() {
        assert_eq!(Solution::num_different_integers("123456".to_string()), 1);
    }

    #[test]
    fn multiple_same_numbers() {
        assert_eq!(Solution::num_different_integers("1a1a1".to_string()), 1);
    }

    #[test]
    fn leading_zeros() {
        assert_eq!(
            Solution::num_different_integers("001a002a003".to_string()),
            3
        );
    }

    #[test]
    fn alternating_pattern() {
        assert_eq!(
            Solution::num_different_integers("a1b2c3d1e2".to_string()),
            3
        );
    }
}
