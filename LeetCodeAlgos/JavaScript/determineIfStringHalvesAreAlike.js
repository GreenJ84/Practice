// You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.

// Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.

// Return true if a and b are alike. Otherwise, return false.

const vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

/**
 * @param {string} s
 * @return {boolean}
 */
const halvesAreAlike = function (s) {
  let ans = 0;
  for (let i = 0; i < s.length / 2; i++){
    let j = s.length - 1 - i;
    if (vowels.includes(s[i])) {
      ans++;
    }
    if (vowels.includes(s[j])) {
      ans--;
    }
  }
  return ans === 0;
};

const testHalvesAreAlike = function (testNum, input, expectation) { 
  const res = halvesAreAlike(input);
  console.log(`Test #${testNum}: ${res} -> `, res === expectation ? "Passed ✅" : "Failed ❌");
}

testHalvesAreAlike(1, "book", true);
testHalvesAreAlike(2, "textbook", false);