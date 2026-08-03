// We distribute some number of candies, to a row of n = num_people people in the following way:

// We then give 1 candy to the first person, 2 candies to the second person, and so on until we give n candies to the last person.

// Then, we go back to the start of the row, giving n + 1 candies to the first person, n + 2 candies to the second person, and so on until we give 2 * n candies to the last person.

// This process repeats (with us giving one more candy each time, and moving to the start of the row after we reach the end) until we run out of candies.  The last person will receive all of our remaining candies (not necessarily one more than the previous gift).

// Return an array (of length num_people and sum candies) that represents the final distribution of candies.

// Constraints:
// 1 <= candies <= 10^9
// 1 <= num_people <= 1000

struct Solution;
impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people as usize;
        let mut result = vec![0; n];
        let (mut idx, mut allot) = (0usize, 1i32);
        while candies > 0 {
            let give = allot.min(candies);
            result[idx] += give;
            candies -= give;
            allot += 1;

            idx += 1;
            idx %= n;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let candies = 7;
        let num_people = 4;
        let result = Solution::distribute_candies(candies, num_people);
        assert_eq!(result, vec![1, 2, 3, 1]);
    }

    #[test]
    fn test_2() {
        let candies = 10;
        let num_people = 3;
        let result = Solution::distribute_candies(candies, num_people);
        assert_eq!(result, vec![5, 2, 3]);
    }
}
