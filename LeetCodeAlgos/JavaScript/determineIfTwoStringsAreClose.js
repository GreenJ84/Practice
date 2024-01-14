/** @format */

// Two strings are considered close if you can attain one from the other using the following operations:
// Operation 1: Swap any two existing characters.
// For example, abcde -> aecdb
// Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
// For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)
// You can use the operations on either string as many times as necessary.
// Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.

/**
 * @param {string} word1
 * @param {string} word2
 * @return {boolean}
 */
const closeStrings = function (word1, word2) {
  // Must have the same length
  if (word1.length !== word2.length) return false;
  // Count char frequency for both strings
  let word1Freq = {}; // {char: count}
  let word2Freq = {};
  for (let i = 0; i < word1.length; i++) {
    word1Freq[word1[i]] = (word1Freq[word1[i]] ?? 0) + 1;
    word2Freq[word2[i]] = (word2Freq[word2[i]] ?? 0) + 1;
  }

  if (Object.entries(word1Freq).length !== Object.entries(word2Freq).length) {
    return false;
  }


  let frequencyNeeded = [];
  let frequencyHave = [];
  // Loop through existing Word1 chars
  for (let char of Object.keys(word1Freq)) {
    console.log(char, word1Freq[char], word2Freq[char]);
    console.log(`Need ${frequencyNeeded}`);
    console.log(`Have ${frequencyHave}`);
    // If you dont have a certain char, no way to get it
    if (!(char in word2Freq)) return false;
    // If the frequency of the two chars is same, any number of operation 1 can make the same
    if (word1Freq[char] === word2Freq[char]) continue;
    // If not same frequency, must find a match to do Operation 2
    else {
      frequencyNeeded.push(word1Freq[char]);

      frequencyHave.push(word2Freq[char]);
    }
  }
  for (let have of frequencyHave) { 
    if (!frequencyNeeded.includes(have)) return false;
    frequencyNeeded.splice(frequencyNeeded.indexOf(have), 1);
  }

  return frequencyNeeded.length === 0;
};

function testCloseStrings(testNum, input, expectation) {
  let result = closeStrings(...input);
  console.log(
    `Test ${testNum}: ${result} -> `,
    result === expectation ? "Passed ✅" : "Failed ❌"
  );
}

// testCloseStrings(1, ["abc", "bca"], true);

// testCloseStrings(2, ["a", "aa"], false);

// testCloseStrings(3, ["cabbba", "abbccc"], true);

testCloseStrings(4, ["svotbsgqiqmeqjwdqqtkucrzqphqxqtqqlyfan", "aapyhufaaaalkqsvtjnaaoewxkrgsbsazadmci"], true);