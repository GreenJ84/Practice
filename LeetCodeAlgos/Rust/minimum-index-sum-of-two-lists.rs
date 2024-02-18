// Given two arrays of strings list1 and list2, find the common strings with the least index sum.
// A common string is a string that appeared in both list1 and list2.
// A common string with the least index sum is a common string such that if it appeared at list1[i] and list2[j] then i + j should be the minimum value among all the other common strings.
// Return all the common strings with the least index sum. Return the answer in any order.

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let list_1_dict = list1
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.to_owned(), idx as i16))
            .collect::<HashMap<String, i16>>();

        let mut min_items = Vec::<String>::new();
        let mut min_sum = i16::MAX;
        for (list2_idx, item) in list2.iter().enumerate() {
            // If both lists share words
            if let Some(list1_idx) = list_1_dict.get(item) {
                let sum = *list1_idx + list2_idx as i16;
                // If a smaller index sum is found
                if sum < min_sum {
                    min_sum = sum;
                    min_items = vec![item.to_string()]
                }
                // If a similar index sum is found
                else if sum == min_sum {
                    min_items.push(item.to_string());
                }
            }
        }
        min_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_find_restaurant() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }

    #[test]
    pub fn test_find_restaurant2() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }

    #[test]
    pub fn test_find_restaurant3() {
        assert_eq!(
            Solution::find_restaurant(
                vec!["happy".to_string(), "sad".to_string(), "good".to_string()],
                vec!["sad".to_string(), "happy".to_string(), "good".to_string()]
            ),
            vec!["sad".to_string(), "happy".to_string()]
        );
    }
}
