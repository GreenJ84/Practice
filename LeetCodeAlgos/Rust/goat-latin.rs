// You are given a string sentence that consist of words separated by spaces. Each word consists of lowercase and uppercase letters only.

// We would like to convert the sentence to "Goat Latin" (a made-up language similar to Pig Latin.) The rules of Goat Latin are as follows:

// If a word begins with a vowel ('a', 'e', 'i', 'o', or 'u'), append "ma" to the end of the word.
// For example, the word "apple" becomes "applema".
// If a word begins with a consonant (i.e., not a vowel), remove the first letter and append it to the end, then add "ma".
// For example, the word "goat" becomes "oatgma".
// Add one letter 'a' to the end of each word per its word index in the sentence, starting with 1.
// For example, the first word gets "a" added to the end, the second word gets "aa" added to the end, and so on.
// Return the final sentence representing the conversion from sentence to Goat Latin.

struct Solution;
impl Solution {
    pub fn to_goat_latin(mut sentence: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        sentence
          .split_ascii_whitespace()
          .enumerate()
          .map(|(idx, mut word)| {
            let mut word = word.to_owned();
            if !word.starts_with(vowels) {
              let ch = word.remove(0);
              word.push(ch);
            }
            word += "ma";
            word += &"a".repeat(idx + 1);
            word
          })
          .collect::<Vec<_>>()
          .join(" ")
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(
      Solution::to_goat_latin("I speak Goat Latin".to_string()),
      "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
    );
  }

  #[test]
  fn test_example_2() {
    assert_eq!(
      Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
      "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
    );
  }

  #[test]
  fn test_single_vowel_word() {
    assert_eq!(
      Solution::to_goat_latin("a".to_string()),
      "amaa"
    );
  }

  #[test]
  fn test_single_consonant_word() {
    assert_eq!(
      Solution::to_goat_latin("b".to_string()),
      "bmaa"
    );
  }

  #[test]
  fn test_all_vowels() {
    assert_eq!(
      Solution::to_goat_latin("aeiou".to_string()),
      "aeioumaa"
    );
  }

  #[test]
  fn test_mixed_case() {
    assert_eq!(
      Solution::to_goat_latin("Apple Egg".to_string()),
      "Applemaa Eggmaaa"
    );
  }
}
