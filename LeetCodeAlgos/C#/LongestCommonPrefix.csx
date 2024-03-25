// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

public class Solution {
    public string LongestCommonPrefix(string[] strs) {
      if (strs.Length == 1)
        return strs[0];

      string prefix = "";
      for(int chIdx = 0; chIdx < 200; chIdx++)
      {
        if (chIdx >= strs[0].Length)
          return prefix;
        char ch = strs[0][chIdx];
        for(int strIdx = 1; strIdx < strs.Length; strIdx++)
        {
          if (chIdx >= strs[strIdx].Length || strs[strIdx][chIdx] != ch)
            return prefix;
        }
        prefix += ch;
      }
      return prefix;
    }
}