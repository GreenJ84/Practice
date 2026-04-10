// Given a string containing letters, digits, and symbols, determine if it reads the same forwards and backwards when considering only alphabetic characters (case-insensitive).

// fn isAlphabeticPalindrome(code: &str) -> bool {
fn is_alphabetic_palindrome(code: &str) -> bool {
  if code.len() <= 1 {
    return true;
  }
  
  let (mut left, mut right) = (0usize, code.len() - 1);
  let code = code.chars().collect::<Vec<_>>();
  while left < right {
    while left < right && !code[left].is_ascii_alphabetic() {
      left += 1;
    }
    while left < right && !code[right].is_ascii_alphabetic() {
      right -= 1;
    }
    if code[left].to_ascii_lowercase() != code[right].to_ascii_lowercase() {
      return false;
    }
    left += 1;
    right -= 1;
  }
  true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let code = "A1b2B!a";
        assert_eq!(is_alphabetic_palindrome(code), true);
    }

    #[test]
    fn test_2() {
        let code = "abc123";
        assert_eq!(is_alphabetic_palindrome(code), false);
    }

    #[test]
    fn test_3() {
        let code = "a!b@c#d$e%f^g&h(i)j";
        assert_eq!(is_alphabetic_palindrome(code), false);
    }
}

// Example

// Input

// code = A1b2B!a
// Output

// 1
// Explanation

// - Step 1: Extract only letters → ['A','b','B','a'] 
// - Step 2: Convert to lowercase → ['a','b','b','a'] 
// - Step 3: Compare sequence forward and backward: 'abba' == 'abba' → true
// Input Format

// A string code containing letters (A–Z, a–z), digits (0–9), and symbols
// Constraints

// 0 <= code.length <= 1000
// For all 0 <= i < code.length: 33 <= ASCII(code[i]) <= 126
// code contains only printable ASCII characters (letters, digits, symbols)