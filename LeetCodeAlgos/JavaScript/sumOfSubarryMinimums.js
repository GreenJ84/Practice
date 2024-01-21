// Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 109 + 7.

const sumSubarrayMins = function(arr) {
  let ans = 0;
  for (let i = 0; i < arr.length; i++) {
    // Track min of subarry starting at i
    let subarrayMin = arr[i];
    // Add single val subarray min
    ans += subarrayMin;
    // Check each subarray val against current min and add
    for (let j = i + 1; j < arr.length; j++) { 
      subarrayMin = Math.min(subarrayMin, arr[j]);
      ans += subarrayMin;
    }
  }
  return ans % (10 ** 9 + 7);
};


const testSumSubarrayMins = function (testNum, array, expected) { 
  const result = sumSubarrayMins(array);
  console.log(`Test ${testNum}: ${result} -> `, result === expected ? 'Passed ✅' : 'Failed ❌');
}

testSumSubarrayMins(1, [3,1,2,4], 17);
testSumSubarrayMins(2, [11,81,94,43,3], 444);
testSumSubarrayMins(3, [1], 1);
testSumSubarrayMins(4, [1, 2], 4);
testSumSubarrayMins(5, [1, 2, 3], 10);