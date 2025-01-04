// Given a string s, return the number of unique palindromes of length three that are a subsequence of s.

// Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.

// A palindrome is a string that reads the same forwards and backwards.

// A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

// For example, "ace" is a subsequence of "abcde".

import java.util.*;

public class UniqueLength3PalindromicSubsequences {
  public static void main(String[] args) {
    UniqueLength3PalindromicSubsequences obj = new UniqueLength3PalindromicSubsequences();

    System.out.println(obj.countPalindromicSubsequence("aabca"));
    System.out.println(obj.countPalindromicSubsequence("adc"));
    System.out.println(obj.countPalindromicSubsequence("bbcbaba"));
  }

  public int countPalindromicSubsequence(String s) {
    Map<Character, ArrayList<Integer>> indices = new HashMap<>();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      indices.computeIfAbsent(c, k -> new ArrayList<>()).add(i);
    }

    int count = 0;
    for (char c : indices.keySet()) {
      ArrayList<Integer> indexList = indices.get(c);
      if (indexList.size() < 2) { continue; }
      else if (indexList.size() > 2) { ++count; }


      Set<Character> set = new HashSet<>();
      for (int idx = 0; idx < indexList.size() - 1; idx++) {
        int left = indexList.get(idx);
        int right = indexList.get(idx + 1);
        for (int middle = left + 1; middle < right; middle++){
          set.add(s.charAt(middle));
        }
      }
      count += set.size();
    }
    return count;
  }
}
