// A word is considered valid if:

// It contains a minimum of 3 characters.
// It contains only digits (0-9), and English letters (uppercase and lowercase).
// It includes at least one vowel.
// It includes at least one consonant.
// You are given a string word.

// Return true if word is valid, otherwise, return false.

// Notes:

// 'a', 'e', 'i', 'o', 'u', and their uppercases are vowels.
// A consonant is an English letter that is not a vowel.

struct Solution {}
impl Solution {
    pub fn is_valid(word: String) -> bool {
      // Make sure word is long enough first
      if word.len() < 3 { return false; }

      let mut vowel = false;
      let mut consonant = false;
      for char in word.chars() {
        // Must be only digits/english chars
        if !char.is_digit(10) && ! char.is_ascii_alphabetic() {
          return false;
        }

        // Make sure to get at least 1 vowel/consonant
        if char.is_ascii_alphabetic() && (!vowel || !consonant) {
          match char.to_lowercase().next(){
            Some('a' | 'e' | 'i' | 'o' | 'u') => {
              if !vowel { vowel = true; }
            },
            Some(ch) => {
              if !consonant { consonant = true; }
            }
            _ => {}
          }
        }
      }
      println!("{}-{}", vowel, consonant);
      vowel && consonant
    }
}