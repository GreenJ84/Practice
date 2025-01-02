// You are given a 0-indexed array of strings words and a 2D array of integers queries.

// Each query queries[i] = [li, ri] asks us to find the number of strings present in the range li to ri (both inclusive) of words that start and end with a vowel.

// Return an array ans of size queries.length, where ans[i] is the answer to the ith query.

// Note that the vowel letters are 'a', 'e', 'i', 'o', and 'u'.

public class CountVowelStringsInRanges {
  public static void main(String[] args) {
    CountVowelStringsInRanges obj = new CountVowelStringsInRanges();

    obj.printArray(obj.vowelStrings(
      new String[]{"aba","bcb","ece","aa","e"},
      new int[][]{{0, 2}, {1, 4}, {1, 1}}
    ));

    obj.printArray(obj.vowelStrings(
      new String[]{"a","e","i"},
      new int[][]{{0, 2}, {0, 1}, {2, 2}}
    ));
  }

  private void printArray(int[] ans){
    for (int i : ans){
      System.out.printf("%d ", i);
    }
    System.out.println();
  }

  private boolean isCharVowel(char ch){
    return ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u';
  }

  public int[] vowelStrings(String[] words, int[][] queries) {
    int[] prefix = new int[words.length + 1];
    for (int i = 0; i < words.length; i++) {
      prefix[i + 1] = prefix[i] + (isCharVowel(words[i].charAt(0)) && isCharVowel(words[i].charAt(words[i].length() - 1)) ? 1 : 0);
    }

    int[] ans = new int[queries.length];
    for (int i = 0; i < queries.length; i++){
      ans[i] = prefix[queries[i][1] + 1] - prefix[queries[i][0]];
    }
    return ans;
  }

  // public int[] vowelStrings(String[] words, int[][] queries) {
  //   HashSet<Character> vowels = new HashSet<>();
  //   for (char ch : new char[]{'a', 'e', 'i', 'o', 'u'}){
  //     vowels.add(ch);
  //   }

  //   boolean[] isVowel = new boolean[words.length];
  //   for (int i = 0; i < words.length; i++){
  //     isVowel[i] = vowels.contains(words[i].charAt(0)) && vowels.contains(words[i].charAt(words[i].length() - 1));
  //   }

  //   int[] ans = new int[queries.length];
  //   for (int i = 0; i < queries.length; i++){
  //     int count = 0;
  //     for (int j = queries[i][0]; j <= queries[i][1]; j++){
  //       if (isVowel[j]){
  //         count++;
  //       }
  //     }
  //     ans[i] = count;
  //   }
  //   return ans;
  // }
}
