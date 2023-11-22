// Given a string licensePlate and an array of strings words, find the shortest completing word in words.

// A completing word is a word that contains all the letters in licensePlate. Ignore numbers and spaces in licensePlate, and treat letters as case insensitive. If a letter appears more than once in licensePlate, then it must appear in the word the same number of times or more.

// For example, if licensePlate = "aBc 12c", then it contains letters 'a', 'b' (ignoring case), and 'c' twice. Possible completing words are "abccdef", "caaacab", and "cbca".

// Return the shortest completing word in words. It is guaranteed an answer exists. If there are multiple shortest completing words, return the first one that occurs in words.

use std::collections::HashMap;

struct Solution {}

fn check_maps(matcher: &HashMap<char, i32>, check: &HashMap<char, i32>) -> bool {
    for (key, value) in &*matcher {
        if let Some(v) = check.get(&key) {
            if v < value {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut matcher: HashMap<char, i32> = HashMap::new();
        for ch in license_plate.chars() {
            if ch.is_ascii_alphabetic() {
                matcher.entry(ch.to_ascii_lowercase()).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        println!("{:?}", matcher);

        let mut run: HashMap<char, i32> = HashMap::new();
        let mut ans: Vec<String> = Vec::new();
        for word in words {
            for ch in word.chars() {
                if ch.is_ascii_alphabetic() {
                    run.entry(ch.to_ascii_lowercase()).and_modify(|v| *v += 1).or_insert(1);
                }
            }
            if check_maps(&matcher, &run) {
                ans.push(word);
            }
            println!("{:?}", run);
            run.clear();
        }
        println!("{:?}", ans);
        while ans.len() > 1 {
            if ans[0].len() > ans[1].len() {
                ans.remove(0);
            } else {
                ans.remove(1);
            }
        }
        return ans.get(0).unwrap().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_completing_word() {
        assert_eq!(
            Solution::shortest_completing_word("1s3 PSt".to_string(), vec!["step".to_string(), "steps".to_string(), "stripe".to_string(), "stepple".to_string()]),
            "steps"
        );
    }

    #[test]
    fn test_shortest_completing_word_1() {
        assert_eq!(
            Solution::shortest_completing_word("1s3 456".to_string(), vec!["looks".to_string(),"pest".to_string(),"stew".to_string(),"show".to_string()]),
            "pest"
        );
    }

     #[test]
    fn test_shortest_completing_word_2() {
        assert_eq!(
            Solution::shortest_completing_word("GrC8950".to_string(),
            vec!["measure".to_string(),"other".to_string(),"every".to_string(),"base".to_string(),"according".to_string(),"level".to_string(),"meeting".to_string(),"none".to_string(),"marriage".to_string(),"rest".to_string()]),
            "pest"
        );
    }
}