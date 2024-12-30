// Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
//
// You must write an algorithm that runs in O(n) time and uses O(1) extra space.


struct Solution {}
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut lex_sorted: Vec<i32> = Vec::new();
        let mut num = 1;

        for _i in 0..n {
            lex_sorted.push(num);
            if num * 10 <= n { num *= 10; }
            else {
                while (num % 10 == 9 && num != 9) || num + 1 > n {
                    num /= 10;
                }
                num+=1;
            }
        }
        lex_sorted
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1(){
        assert_eq!(
            Solution::lexical_order(13),
            Vec::from([1,10,11,12,13,2,3,4,5,6,7,8,9])
        );
    }

    #[test]
    pub fn test2(){
        assert_eq!(
            Solution::lexical_order(2),
            Vec::from([1,2])
        );
    }
}