// You are given three arrays of length n that describe the properties of n coupons: code, businessLine, and isActive. The ith coupon has:
// - code[i]: a string representing the coupon identifier.
// - businessLine[i]: a string denoting the business category of the coupon.
// - isActive[i]: a boolean indicating whether the coupon is currently active.

// A coupon is considered valid if all of the following conditions hold:
// - code[i] is non-empty and consists only of alphanumeric characters (a-z, A-Z, 0-9) and underscores (_).
// - businessLine[i] is one of the following four categories: "electronics", "grocery", "pharmacy", "restaurant".
// - isActive[i] is true.

// Return an array of the codes of all valid coupons, sorted first by their businessLine in the order: "electronics", "grocery", "pharmacy", "restaurant", and then by code in lexicographical (ascending) order within each category.

struct Solution;
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut valid_codes = Vec::<(String, u8)>::new();

        for idx in 0..code.len() {
            if !is_active[idx] {
                continue;
            }

            let business_code = match business_line[idx].as_str() {
                "electronics" => 1,
                "grocery" => 2,
                "pharmacy" => 3,
                "restaurant" => 4,
                _ => continue,
            };

            if !code[idx].is_empty() && code[idx].chars().all(|c| c.is_alphanumeric() || c == '_') {
                valid_codes.push((code[idx].clone(), business_code));
            }
        }

        valid_codes.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        });

        valid_codes.into_iter().map(|(code, _)| code).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let code = vec![
            "SAVE20".to_string(),
            "".to_string(),
            "PHARMA5".to_string(),
            "SAVE@20".to_string(),
        ];
        let business_line = vec![
            "restaurant".to_string(),
            "grocery".to_string(),
            "pharmacy".to_string(),
            "restaurant".to_string(),
        ];
        let is_active = vec![true, true, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["PHARMA5".to_string(), "SAVE20".to_string()]
        );
    }

    #[test]
    fn test_example_2() {
        let code = vec![
            "GROCERY15".to_string(),
            "ELECTRONICS_50".to_string(),
            "DISCOUNT10".to_string(),
        ];
        let business_line = vec![
            "grocery".to_string(),
            "electronics".to_string(),
            "invalid".to_string(),
        ];
        let is_active = vec![false, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["ELECTRONICS_50".to_string()]
        );
    }

    #[test]
    fn test_all_inactive() {
        let code = vec!["CODE1".to_string(), "CODE2".to_string()];
        let business_line = vec!["electronics".to_string(), "pharmacy".to_string()];
        let is_active = vec![false, false];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_lexicographical_order_within_category() {
        let code = vec!["Z123".to_string(), "A456".to_string(), "M789".to_string()];
        let business_line = vec![
            "grocery".to_string(),
            "grocery".to_string(),
            "grocery".to_string(),
        ];
        let is_active = vec![true, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["A456".to_string(), "M789".to_string(), "Z123".to_string()]
        );
    }

    #[test]
    fn test_category_ordering() {
        let code = vec![
            "E1".to_string(),
            "G1".to_string(),
            "P1".to_string(),
            "R1".to_string(),
        ];
        let business_line = vec![
            "restaurant".to_string(),
            "electronics".to_string(),
            "pharmacy".to_string(),
            "grocery".to_string(),
        ];
        let is_active = vec![true, true, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec![
                "G1".to_string(),
                "R1".to_string(),
                "P1".to_string(),
                "E1".to_string(),
            ]
        );
    }

    #[test]
    fn test_special_characters_invalid() {
        let code = vec![
            "CODE-1".to_string(),
            "CODE.1".to_string(),
            "CODE@1".to_string(),
        ];
        let business_line = vec![
            "electronics".to_string(),
            "electronics".to_string(),
            "electronics".to_string(),
        ];
        let is_active = vec![true, true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_underscore_allowed() {
        let code = vec!["CODE_1".to_string(), "CODE_2_3".to_string()];
        let business_line = vec!["pharmacy".to_string(), "pharmacy".to_string()];
        let is_active = vec![true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["CODE_1".to_string(), "CODE_2_3".to_string()]
        );
    }

    #[test]
    fn test_alphanumeric_valid() {
        let code = vec!["abc123".to_string(), "XYZ789".to_string()];
        let business_line = vec!["restaurant".to_string(), "restaurant".to_string()];
        let is_active = vec![true, true];

        assert_eq!(
            Solution::validate_coupons(code, business_line, is_active),
            vec!["XYZ789".to_string(), "abc123".to_string()]
        );
    }
}
