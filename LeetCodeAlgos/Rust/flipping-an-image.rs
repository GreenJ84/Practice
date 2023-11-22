// Given an n x n binary matrix image, flip the image horizontally, then invert it, and return the resulting image.

// To flip an image horizontally means that each row of the image is reversed.

// For example, flipping [1,1,0] horizontally results in [0,1,1].
// To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0.

// For example, inverting [0,1,1] results in [1,0,0].

struct Solution {}
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transformed: Vec<Vec<i32>> = Vec::new();
        for row in image {
            transformed.push(row.iter().rev().map(|&x| if x == 1 { 0 } else { 1 }).collect());
        }
        transformed
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        assert_eq!(
            Solution::flip_and_invert_image(
                vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]
            ), 
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
    }

    #[test]
    fn test_flip_and_invert_image_2() {
        assert_eq!(
            Solution::flip_and_invert_image(
                vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]]
            ),
            vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0],vec![0,0,0,1],vec![1,0,1,0]]);
    }
}

// Input: image = [[1,1,0],[1,0,1],[0,0,0]]
// Output: [[1,0,0],[0,1,0],[1,1,1]]


// Input: image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
// Output: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
