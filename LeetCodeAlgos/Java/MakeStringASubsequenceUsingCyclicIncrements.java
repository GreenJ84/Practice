public class MakeStringASubsequenceUsingCyclicIncrements {
  public boolean canMakeSubsequence(String str1, String str2) {
    int len1 = str1.length(), len2 = str2.length();
    if (len1 < len2) return false;

    int idx2 = 0;
    for (int idx1 = 0; idx1 < len1; idx1++){
        char ch1 = str1.charAt(idx1), ch2 = str2.charAt(idx2);
        if (
            ch1 == ch2 ||
            ch1 - ch2 == -1 ||
            (ch1 == 'z' && ch2 == 'a')
        ){
            idx2++;
            if (idx2 == len2) return true;
        }
    }
    return false;
}
}
