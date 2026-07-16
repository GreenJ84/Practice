// You have a bomb to defuse, and your time is running out! Your informer will provide you with a circular array code of length of n and a key k.

// To decrypt the code, you must replace every number. All the numbers are replaced simultaneously.

// If k > 0, replace the ith number with the sum of the next k numbers.
// If k < 0, replace the ith number with the sum of the previous -k numbers.
// If k == 0, replace the ith number with 0.
// As code is circular, the next element of code[n-1] is code[0], and the previous element of code[0] is code[n-1].

// Given the circular array code and an integer key k, return the decrypted code to defuse the bomb!

// Constraints:

// n == code.length
// 1 <= n <= 100
// 1 <= code[i] <= 100
// -(n - 1) <= k <= n - 1

struct Solution;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut prefix = vec![0; code.len()];
        prefix[0] = code[0];
        for i in 1..code.len() {
            prefix[i] = prefix[i - 1] + code[i];
        }

        code.iter()
            .enumerate()
            .map(|(idx, _)| match k {
                0 => 0,
                _ if k > 0 => {
                    let k = k as usize;
                    if idx + k < code.len() {
                        prefix[idx + k] - prefix[idx]
                    } else {
                        let k = (idx + k) % code.len();
                        prefix[prefix.len() - 1] - prefix[idx] + prefix[k]
                    }
                }
                _ => {
                    let k = k.abs() as usize;
                    match k {
                        _ if k == idx => prefix[idx - 1],
                        _ if k < idx => prefix[idx - 1] - prefix.get(idx - k - 1).unwrap_or(&0),
                        _ => {
                            let k = idx + code.len() - k;
                            let mut sum = prefix[prefix.len() - 1] - prefix[k - 1];
                            if idx > 0 {
                                sum += prefix[idx - 1];
                            }
                            sum
                        }
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let result = Solution::decrypt(code, k);
        assert_eq!(result, vec![12, 10, 16, 13]);
    }

    #[test]
    fn test2() {
        let code = vec![1, 2, 3, 4];
        let k = 0;
        let result = Solution::decrypt(code, k);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test3() {
        let code = vec![2, 4, 9, 3];
        let k = -2;
        let result = Solution::decrypt(code, k);
        assert_eq!(result, vec![12, 5, 6, 13]);
    }
}
