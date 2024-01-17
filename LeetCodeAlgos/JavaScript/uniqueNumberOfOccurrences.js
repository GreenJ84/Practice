// Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.

/**
 * @param {number[]} arr
 * @return {boolean}
 */
const uniqueOccurrences = function(arr) {
  const freq = {};
  arr.map(item => { freq[item] = (freq[item] ?? 0) + 1; });
  const counts = Object.values(freq);
  return counts.length === new Set(counts).size;
};

function testUniqueOccurrences(testNum, arr, expected) { 
  const result = uniqueOccurrences(arr);
  console.log(`Test ${testNum}: ${result}`, result === expected ? 'Passed ✅' : 'Failed ❌');
}

testUniqueOccurrences(1, [1, 2, 2, 1, 1, 3], true);
testUniqueOccurrences(2, [1, 2], false);
testUniqueOccurrences(3, [-3,0,1,-3,1,1,1,-3,10,0], true);