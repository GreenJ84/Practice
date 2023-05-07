// You are given a string num, representing a large integer. Return the largest-valued odd integer (as a string) that is a non-empty substring of num, or an empty string "" if no odd integer exists.

// A substring is a contiguous sequence of characters within a string.

struct Solution {}
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut largest: i32 = -1;
        let characters: Vec<char> = num.chars().collect();
        for i in 0..characters.len(){
            if characters[i].to_digit(10).unwrap() % 2 == 1 && i as i32 > largest{
                largest = i as i32;
            }
        }
        if largest == -1{
            String::from("")
        }
        else{
            num[0..(largest as usize) + 1].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_odd_number() {
        assert_eq!(Solution::largest_odd_number("52".to_string()), "5".to_string());
    }

    #[test]
    fn test_largest_odd_number2() {
        assert_eq!(Solution::largest_odd_number("4206".to_string()), "".to_string());
    }

    #[test]
    fn test_largest_odd_number3() {
        assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427".to_string());
    }

    #[test]
    fn test_largest_odd_number4() {
        assert_eq!(Solution::largest_odd_number("9".to_string()), "9".to_string());
    }

    #[test]
    fn test_largest_odd_number5() {
        assert_eq!(Solution::largest_odd_number("2".to_string()), "".to_string());
    }

    #[test]
    fn test_largest_odd_number6() {
        assert_eq!(Solution::largest_odd_number("123456789".to_string()), "123456789".to_string());
    }
}