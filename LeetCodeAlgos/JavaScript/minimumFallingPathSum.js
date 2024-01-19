// Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.
// A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).

/**
 * @param {number[][]} matrix
 * @return {number}
 */
const minFallingPathSum = function(matrix) {
  const [n, m] = [matrix.length, matrix[0].length];
  const INFINITY = Number.POSITIVE_INFINITY;
  for (let row = n - 2; row >= 0; row--) { 
    for (let col = 0; col < m; col++) {
      matrix[row][col] += Math.min(
        matrix[row + 1][col - 1] ?? INFINITY, // Left Bottom
        matrix[row + 1][col], // Bottom
        matrix[row + 1][col + 1] ?? INFINITY, // Right Bottom
      );
    }
  }
  return Math.min(...matrix[0]);
};

function testMinFallingPathSum(testNum, matrix, expected){ 
  const result = minFallingPathSum(matrix);
  console.log(`Test #${testNum}: ${result} ->`, result === expected ? 'PASSED ✅' : 'FAILED ❌');
}

testMinFallingPathSum(1, [[2,1,3],[6,5,4],[7,8,9]], 13);
testMinFallingPathSum(2, [[-19,57],[-40,-5]], -59);