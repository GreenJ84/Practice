// Given n slots numbered 0 to n-1, return the number of ways to fill all slots where each operation covers either 1 slot or 2 adjacent slots.

// Constraints
// - 0 <= n <= 1000


// fn countInstallationSequences(n: i32) -> String {

fn _count_installation_sequences1(n: i32) -> String {
    let mut a = 1u128;
    let mut b = 1u128;
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b.to_string()
}

fn count_installation_sequences(n: i32) -> String {
    if n <= 1 {
        return "1".to_string();
    }
    let mut a: Vec<u8> = vec![1];
    let mut b: Vec<u8> = vec![1];
    for _ in 1..n {
        let temp = add_big(&a, &b);
        a = b;
        b = temp;
    }
    vec_to_string(&b)
}

fn add_big(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut carry: u8 = 0;
    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        let da = if i < a.len() { a[i] } else { 0 };
        let db = if i < b.len() { b[i] } else { 0 };
        let sum = da + db + carry;
        res.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        res.push(carry);
    }
    res
}

fn vec_to_string(v: &Vec<u8>) -> String {
    v.iter().rev().map(|&d| (b'0' + d) as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let expected = "3".to_string();
        assert_eq!(count_installation_sequences(n), expected);
    }

    #[test]
    fn test_2() {
        let n = 5;
        let expected = "8".to_string();
        assert_eq!(count_installation_sequences(n), expected);
    }
}
