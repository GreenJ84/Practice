// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
// For example:
    // A -> 1
    // B -> 2
    // C -> 3
    // ...
    // Z -> 26
    // AA -> 27
    // AB -> 28 
    // ...

struct Solution{}
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let alphabets: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        let mut res: String = String::new();
        let mut columns: i32 = column_number.clone();
        
        while columns > 0{
            if columns > 26{
                let rem = columns % 26;
                columns = columns.div_euclid(26);
                if rem != 0 {
                    res.push(alphabets[(rem - 1) as usize]);
                } else{ 
                    res.push('Z');
                    columns -= 1;
                }
            } else {
                res.push(alphabets[(columns - 1) as usize]);
                columns = 0;
            }
        }
        res.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A")
    }

    #[test]
    fn test_convert_to_title2() {
        assert_eq!(Solution::convert_to_title(28), "AB")
    }

    #[test]
    fn test_convert_to_title3() {
        assert_eq!(Solution::convert_to_title(701), "ZY")
    }

    #[test]
    fn test_convert_to_title4() {
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW")
    }

    #[test]
    fn test_convert_to_title5() {
        assert_eq!(Solution::convert_to_title(52), "AZ")
    }

}