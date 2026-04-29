// Given a string of digits where '2'-'9' map to letters (like on a phone keypad) and '0','1' map to themselves, return all possible letter combinations in lexicographical order.

// Constraints:
// - 0 <= digits.length <= 10
// - For all 0 <= i < digits.length, digits[i] is one of the characters '0','1','2','3','4','5','6','7','8','9'
// - Mapping is defined as:
//    - '2'->3 letters,
//    - '3'->3 letters,
//    - '4'->3 letters,
//    - '5'->3 letters,
//    - '6'->3 letters,
//    - '7'->4 letters,
//    - '8'->3 letters,
//    - '9'->4 letters,
//    - '0'->"0",
//    - '1'->"1"
// - Total number of combinations will not exceed 10^6 under the above length bound

const MAPPING: [&str; 10] = ["0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
// fn minTasksToCancelForNoConflict(digits: &str) -> Vec<String> {
fn min_tasks_to_cancel_for_no_conflict(digits: &str) -> Vec<String> {
    let mut ans = Vec::new();
    if digits.is_empty() {
      return ans;
    }
    let digits = digits.chars().collect::<Vec<_>>();
    let idx = 0;
    recurse(&digits, idx, String::new(), &mut ans);
    ans
}

fn recurse(digits: &Vec<char>, idx: usize, current: String, collection: &mut Vec<String>) {
  if idx == digits.len() {
    collection.push(current);
    return;
  }
  for ch in MAPPING[(digits[idx] as u8 - b'0') as usize].chars() {
    recurse(digits, idx + 1, current.clone() + &ch.to_string(), collection);
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let digits = "23";
        let expected: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|s| s.to_string()).collect();
        assert_eq!(min_tasks_to_cancel_for_no_conflict(digits), expected);
    }

    #[test]
    fn test_2() {
        let digits = "203";
        let expected: Vec<String> = vec!["a0d", "a0e", "a0f", "b0d", "b0e", "b0f", "c0d", "c0e", "c0f"].iter().map(|s| s.to_string()).collect();
        assert_eq!(min_tasks_to_cancel_for_no_conflict(digits), expected);
    }

    #[test]
    fn test_3() {
        let digits = "27";
        let expected: Vec<String> = vec!["ap", "aq", "ar", "as", "bp", "bq", "br", "bs", "cp", "cq", "cr", "cs"].iter().map(|s| s.to_string()).collect();
        assert_eq!(min_tasks_to_cancel_for_no_conflict(digits), expected);
    }
}

// Examples
// Example 1

// Input:

// digits = 23
// Output:

// ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']
// Explanation:

// Step 1: Map '2' → [a, b, c], '3' → [d, e, f].
// Step 2: Use backtracking: fix first letter then append each letter from the second set.
// For 'a': combine with [d, e, f] → [ad, ae, af]
// For 'b': combine with [d, e, f] → [bd, be, bf]
// For 'c': combine with [d, e, f] → [cd, ce, cf]
// Step 3: Collect all combinations and sort lexicographically (they are already in order).
// Example 2

// Input:

// digits = 203
// Output:

// ['a0d', 'a0e', 'a0f', 'b0d', 'b0e', 'b0f', 'c0d', 'c0e', 'c0f']
// Explanation:

// Step 1: Map '2' → [a, b, c], '0' → ['0'], '3' → [d, e, f].
// Step 2: Backtracking over three positions:
// First letter from '2': a, then insert '0', then each of [d, e, f] → [a0d, a0e, a0f]
// Next '2' letter: b → [b0d, b0e, b0f]
// Next '2' letter: c → [c0d, c0e, c0f]
// Step 3: The results are collected in lexicographical order.