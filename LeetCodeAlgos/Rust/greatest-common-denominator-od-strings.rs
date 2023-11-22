// For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

struct Solution {}
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let str_1 = (&str1.chars().collect::<Vec<char>>(), &str1.len(), &str1);
        let str_2 = (&str2.chars().collect::<Vec<char>>(), &str2.len(), &str2);

        let longest = if str_1.1 > str_2.1 { str_1 } else { str_2 };
        let shortest = if longest.2 == &str1 { str_2 } else { str_1 };

        let mut result = String::new();
        let mut check = String::new();

        for i in 0..*shortest.1 {
            if longest.0[i] == shortest.0[i] {
                check.push(shortest.0[i]);

                println!("check: {}", check);
                println!("long: {}, short, {}", longest.1 % check.len(), shortest.1 % check.len());

                if longest.1 % check.len() == 0 && shortest.1 % check.len() == 0 {
                    println!("longRep: {}, shortRep, {}", check.repeat(longest.1 / check.len()), check.repeat(shortest.1 / check.len()));

                    if &check.repeat(longest.1 / check.len()) == longest.2 && &check.repeat(shortest.1 / check.len()) == shortest.2 {
                        result = check.clone();
                        println!("result: {}", result);
                    }
                }
            } else {
                break;
            }
        }
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_strings() {
        assert_eq!(Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()), "ABC".to_string());
    }

    #[test]
    fn test_gcd_of_strings_2() {
        assert_eq!(Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()), "AB".to_string());
    }

    #[test]
    fn test_gcd_of_strings_3() {
        assert_eq!(Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()), "".to_string());
    }
}