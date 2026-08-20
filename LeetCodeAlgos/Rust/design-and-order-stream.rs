// There is a stream of n (idKey, value) pairs arriving in an arbitrary order, where idKey is an integer between 1 and n and value is a string. No two pairs have the same id.

// Design a stream that returns the values in increasing order of their IDs by returning a chunk (list) of values after each insertion. The concatenation of all the chunks should result in a list of the sorted values.

// Implement the OrderedStream class:

// OrderedStream(int n) Constructs the stream to take n values.
// String[] insert(int idKey, String value) Inserts the pair (idKey, value) into the stream, then returns the largest possible chunk of currently inserted values that appear next in the order.

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
*/

// Constraints:
// 1 <= n <= 1000
// 1 <= id <= n
// value.length == 5
// value consists only of lowercase letters.
// Each call to insert will have a unique id.
// Exactly n calls will be made to insert.

struct OrderedStream {
    size: usize,
    stream: Vec<Option<String>>,
    place: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            size: n,
            stream: vec![None; n],
            place: 0usize,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.stream[id_key as usize - 1] = Some(value);
        let mut chunk = vec![];
        while self.place < self.size {
            if self.stream[self.place].is_none() {
                break;
            }
            chunk.push(self.stream[self.place].take().unwrap());
            self.place += 1;
        }
        chunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut stream = OrderedStream::new(5);
        let values = vec![
            (3, "ccccc".to_string()),
            (1, "aaaaa".to_string()),
            (2, "bbbbb".to_string()),
            (5, "eeeee".to_string()),
            (4, "ddddd".to_string()),
        ];
        let expected = vec![
            vec![],
            vec!["aaaaa".to_string()],
            vec!["bbbbb".to_string(), "ccccc".to_string()],
            vec![],
            vec!["ddddd".to_string(), "eeeee".to_string()],
        ];
        for (i, (id, value)) in values.into_iter().enumerate() {
            let result = stream.insert(id, value);
            assert_eq!(result, expected[i]);
        }
    }
}
