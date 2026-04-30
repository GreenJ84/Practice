// Given a string paragraph and a string array of the banned words banned, return the most frequent word that is not banned. It is guaranteed there is at least one word that is not banned, and that the answer is unique.

// The words in paragraph are case-insensitive and the answer should be returned in lowercase.

// Note that words can not contain punctuation symbols.

// Constraints:
// - 1 <= paragraph.length <= 1000
// - paragraph consists of English letters, space ' ', or one of the symbols: "!?',;.".
// - 0 <= banned.length <= 100
// - 1 <= banned[i].length <= 10
// - banned[i] consists of only lowercase English letters.

struct Solution;
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned_set: std::collections::HashSet<String> =
            banned.into_iter().map(|word| word.to_lowercase()).collect();

        let mut freq = std::collections::HashMap::<String, usize>::new();

        paragraph
            .split(|c: char| !c.is_alphanumeric())
            .for_each(|w| {
                if !w.is_empty() && !banned_set.contains(w) {
                    *freq.entry(w.to_lowercase()).or_default() += 1;
                }
            });

        freq.into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0
    }

    pub fn _most_common_word2(paragraph: String, banned: Vec<String>) -> String {
        let mut banned_set = std::collections::HashSet::<String>::new();
        banned.into_iter().for_each(|s| {
            banned_set.insert(s);
        });

        let paragraph = paragraph
            .split(' ')
            .flat_map(|w| {
                w.to_string()
                    .to_lowercase()
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .split(|c: char| !c.is_alphanumeric())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();

        let mut freq = std::collections::HashMap::<&String, i32>::new();

        for word in &paragraph {
            if !banned_set.contains(word) {
                freq.entry(word).and_modify(|f| *f += 1).or_insert(1);
            }
        }

        freq.into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0
            .to_owned()
    }

    pub fn _most_common_word1(paragraph: String, banned: Vec<String>) -> String {
        let mut banned_set = std::collections::HashSet::<String>::new();
        banned.into_iter().for_each(|s| {
            banned_set.insert(s);
        });

        let paragraph = paragraph
            .split(' ')
            .map(|w| {
                w.to_string()
                    .to_lowercase()
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string()
            })
            .collect::<Vec<String>>();

        let mut freq = std::collections::HashMap::<&String, i32>::new();

        for word in &paragraph {
            if !banned_set.contains(word) {
                freq.entry(word).and_modify(|f| *f += 1).or_insert(1);
            }
        }

        freq.into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
        let banned = vec!["hit".to_string()];
        assert_eq!(
            Solution::most_common_word(paragraph, banned),
            "ball".to_string()
        );
    }

    #[test]
    fn test_2() {
        let paragraph = "a.".to_string();
        let banned = vec![];
        assert_eq!(
            Solution::most_common_word(paragraph, banned),
            "a".to_string()
        );
    }
}
