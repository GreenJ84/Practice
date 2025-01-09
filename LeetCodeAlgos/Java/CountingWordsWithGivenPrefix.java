// You are given an array of strings words and a string pref.

// Return the number of strings in words that contain pref as a prefix.

// A prefix of a string s is any leading contiguous substring of s.


class CountingWordsWithGivenPrefix {
  public static void main(String[] args) {
    CountingWordsWithGivenPrefix obj = new CountingWordsWithGivenPrefix();

    System.err.println(obj.prefixCount(new String[]{"pay","attention","practice","attend"}, "at"));
    System.err.println(obj.prefixCount(new String[]{"leetcode","win","loops","success"}, "code"));
  }
  public int prefixCount(String[] words, String pref) {
    int count = 0;
    for (String word : words) {
      if (word.startsWith(pref)) {
        count++;
      }
    }
    return count;
  }
}