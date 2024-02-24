// You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.

// For each index i, names[i] and heights[i] denote the name and height of the ith person.

// Return names sorted in descending order by the people's heights.

struct Solution {}
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut names = names
            .iter()
            .enumerate()
            .map(|(idx, x)| (x.to_owned(), heights[idx]))
            .collect::<Vec<(String, i32)>>();
        names.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        names.into_iter().map(|x| x.0).collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_people() {
        assert_eq!(
            Solution::sort_people(vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()], vec![180, 165, 170]),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }

    #[test]
    fn test_sort_people_2() {
        assert_eq!(
            Solution::sort_people(vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()], vec![155, 185, 150]),
            ["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
