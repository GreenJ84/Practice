
struct Solution {}
impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let n: String = n.to_string();
        let mut sum = 0;

       for (idx, ch) in n.char_indices() {
            let val  = ch.to_digit(10).unwrap() as i32;
            sum += if idx % 2 == 0 { val } else { -val };
        }
        return sum;
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test1(){
        assert_eq!(
            Solution::alternate_digit_sum(521),
            4
        )
    }

    #[test]
    pub fn test2(){
        assert_eq!(
            Solution::alternate_digit_sum(111),
            1
        )
    }

    #[test]
    pub fn test3(){
        assert_eq!(
            Solution::alternate_digit_sum(886996),
            0
        )
    }
}