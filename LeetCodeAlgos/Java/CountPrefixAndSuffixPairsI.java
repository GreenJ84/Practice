
// You are given a 0-indexed string array words.

// Let's define a boolean function isPrefixAndSuffix that takes two strings, str1 and str2:

// isPrefixAndSuffix(str1, str2) returns true if str1 is both a prefix and a suffix of str2, and false otherwise.
// For example, isPrefixAndSuffix("aba", "ababa") is true because "aba" is a prefix of "ababa" and also a suffix, but isPrefixAndSuffix("abc", "abcd") is false.

// Return an integer denoting the number of index pairs (i, j) such that i < j, and isPrefixAndSuffix(words[i], words[j]) is true.

public class CountPrefixAndSuffixPairsI {
  public static void main(String[] args) {
    CountPrefixAndSuffixPairsI obj = new CountPrefixAndSuffixPairsI();

    System.out.println(obj.countPrefixSuffixPairs(new String[]{"a","aba","ababa","aa"}));
    System.out.println(obj.countPrefixSuffixPairs(new String[]{"pa","papa","ma","mama"}));
    System.out.println(obj.countPrefixSuffixPairs(new String[]{"abab","ab"}));
  }

  public int countPrefixSuffixPairs(String[] words) {
    int count = 0;
    for (int i = 0; i < words.length - 1; i++) {
      for (int j = i + 1; j < words.length; j++) {
        if (isPrefixAndSuffix(words[i], words[j])) count++;
      }
    }
    return count;
  }

  boolean isPrefixAndSuffix(String str1, String str2) {
    if (str2.length() < str1.length()) return false;
    return str2.startsWith(str1) && str2.endsWith(str1);
  }
}
