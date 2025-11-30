// Your friend is typing his name into a keyboard. Sometimes, when typing a character c, the key might get long pressed, and the character will be typed 1 or more times.

// You examine the typed characters of the keyboard. Return True if it is possible that it was your friends name, with some characters (possibly none) being long pressed.

struct Solution;
impl Solution {
  pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let mut name = name.chars();
    let mut typed = typed.chars();

    let (mut old_name_char, mut curr_name_char) = (Some(' '), name.next());
    let mut typed_char = typed.next();

    return loop {
      match typed_char {
        None => {
          break curr_name_char.is_none();
        },
        t if t == curr_name_char => {
          (old_name_char, curr_name_char) = (curr_name_char, name.next());
          typed_char = typed.next();
        },
        t if t == old_name_char => {
          typed_char = typed.next();
        },
        _ => { break false; }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    assert!(Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()));
  }

  #[test]
  fn example_2() {
    assert!(!Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()));
  }

  #[test]
  fn exact_match_no_long_press() {
    assert!(Solution::is_long_pressed_name("abc".to_string(), "abc".to_string()));
  }

  #[test]
  fn single_char_name_repeated_typed() {
    assert!(Solution::is_long_pressed_name("a".to_string(), "aaaa".to_string()));
  }

  #[test]
  fn typed_missing_character() {
    assert!(!Solution::is_long_pressed_name("alex".to_string(), "ale".to_string()));
  }

  #[test]
  fn name_with_repeats_typed_insufficient_repeats() {
    assert!(!Solution::is_long_pressed_name("saeed".to_string(), "ssaaed".to_string()));
  }

  #[test]
  fn typed_has_extra_unmatched_characters() {
    assert!(!Solution::is_long_pressed_name("py".to_string(), "ppyzz".to_string()));
  }

  #[test]
  fn both_all_same_characters() {
    assert!(Solution::is_long_pressed_name("aaaa".to_string(), "aaaaaaaa".to_string()));
  }
}
