public class AddingSpacesToString {
  public String addSpaces(String s, int[] spaces) {
    int n = s.length();
    int m = spaces.length;
    char[] chars = new char[n + m];
    int space = m - 1, ins = m + n - 1;
    for (int i = n - 1; i >= 0; i--){
        chars[ins--] = s.charAt(i);
        if (space >= 0 && i == spaces[space]){
            chars[ins--] = ' ';
            space--;
        }
    }
    return String.valueOf(chars);
  }
}
