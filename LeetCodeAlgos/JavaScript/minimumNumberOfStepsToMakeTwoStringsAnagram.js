// You are given two strings of the same length s and t. In one step you can choose any character of t and replace it with another character.
// Return the minimum number of steps to make t an anagram of s.

// An Anagram of a string is a string that contains the same characters with a different (or the same) ordering.


/**
 * @param {string} s
 * @param {string} t
 * @return {number}
 */
const minSteps = function (s, t) {
  if (s.length !== t.length) return -1;
  
  let dictS = {};
  for (let char of s) dictS[char] = (dictS[char] || 0) + 1;
  for (let char of t) {
    if (char in dictS && dictS[char] > 0) dictS[char] = dictS[char] - 1;
  }
  return Object.values(dictS).reduce((a, b) => a + Math.abs(b), 0);
};

const testMinSteps = function (testNum, input, expectation) { 
  let result = minSteps(...input);
  console.log(
    `Test #${testNum}: ${result} ->`,
    result === expectation? 'Passed ✅' : 'Failed ❌');
}

testMinSteps(1, ["bab", "aba"], 1);
testMinSteps(2, ["leetcode", "practice"], 5);
testMinSteps(3, ["anagram", "mangaar"], 0);