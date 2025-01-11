public class ConstructKPalindromeStrings {
  public static void main(String[] args) {
    ConstructKPalindromeStrings obj = new ConstructKPalindromeStrings();
    
    System.out.println(obj.canConstruct("annabelle", 2));
    System.out.println(obj.canConstruct("leetcode", 3));
    System.out.println(obj.canConstruct("true", 4));
    System.out.println(obj.canConstruct("yzyzyzyzyzyzyzy", 2));
    System.out.println(obj.canConstruct("ab", 12));
  }

  public boolean canConstruct(String s, int k) {
    if (s.length() < k) return false;

    int[] count = new int[26];
    for (char c : s.toCharArray()) {
      count[c - 'a']++;
    }
    int oddCount = 0;
    for (int c : count) {
      oddCount+= c % 2;
    }
    return oddCount <= k;
  }
}
