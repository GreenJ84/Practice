// There are n people standing in a line labeled from 1 to n. The first person in the line is holding a pillow initially. Every second, the person holding the pillow passes it to the next person standing in the line. Once the pillow reaches the end of the line, the direction changes, and people continue passing the pillow in the opposite direction.

// For example, once the pillow reaches the nth person they pass it to the n - 1th person, then to the n - 2th person and so on.
// Given the two positive integers n and time, return the index of the person holding the pillow after time seconds.

// Constraints:
// 2 <= n <= 1000
// 1 <= time <= 1000

struct Solution;
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let spot = time % (n - 1);
        let cycle = (time / (n - 1)) % 2 == 0;
        match cycle {
            true => 1 + spot,
            false => n - spot,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let time = 5;
        let result = Solution::pass_the_pillow(n, time);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let time = 2;
        let result = Solution::pass_the_pillow(n, time);
        assert_eq!(result, 3);
    }
}
