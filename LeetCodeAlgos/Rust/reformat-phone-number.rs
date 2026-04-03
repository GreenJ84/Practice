// You are given a phone number as a string number. number consists of digits, spaces ' ', and/or dashes '-'.

// You would like to reformat the phone number in a certain manner. Firstly, remove all spaces and dashes. Then, group the digits from left to right into blocks of length 3 until there are 4 or fewer digits. The final digits are then grouped as follows:

// 2 digits: A single block of length 2.
// 3 digits: A single block of length 3.
// 4 digits: Two blocks of length 2 each.
// The blocks are then joined by dashes. Notice that the reformatting process should never produce any blocks of length 1 and produce at most two blocks of length 2.

// Return the phone number after formatting.

struct Solution;
impl Solution {
    pub fn reformat_number(number: String) -> String {
      let mut stack = Vec::new();
      for c in number.chars() {
        if c.is_digit(10) {
          stack.push(c);
        }
      }
      let mut result = String::new();
      while stack.len() > 4 {
        for _ in 0..3 {
          result.push(stack.remove(0));
        }
        result.push('-');
      }

      match stack.len() {
        2 => {
          result.push(stack.remove(0));
          result.push(stack.remove(0));
        }
        3 => {
          result.push(stack.remove(0));
          result.push(stack.remove(0));
          result.push(stack.remove(0));
        }
        4 => {
          result.push(stack.remove(0));
          result.push(stack.remove(0));
          result.push('-');
          result.push(stack.remove(0));
          result.push(stack.remove(0));
        }
        _ => {}
      }
      result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let number = "1-23-45 6".to_string();
        assert_eq!(Solution::reformat_number(number), "123-456");
    }

    #[test]
    fn test2() {
        let number = "123 4-567".to_string();
        assert_eq!(Solution::reformat_number(number), "123-45-67");
    }

    #[test]
    fn test3() {
        let number = "123 4-5678".to_string();
        assert_eq!(Solution::reformat_number(number), "123-456-78");
    }
}
