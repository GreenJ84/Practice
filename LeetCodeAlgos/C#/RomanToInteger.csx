// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
  // Symbol       Value
  // I             1
  // V             5
  // X             10
  // L             50
  // C             100
  // D             500
  // M             1000
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
  // I can be placed before V (5) and X (10) to make 4 and 9. 
  // X can be placed before L (50) and C (100) to make 40 and 90. 
  // C can be placed before D (500) and M (1000) to make 400 and 900.
  // Given a roman numeral, convert it to an integer.

public class Solution {
  public int RomanToInt(string s) {
    int n = s.Length;
    int ans = 0;
    for (int i = 0; i < n; i++){
      switch( s[i] ){
        case 'M':
          ans += 1000;
          break;
        case 'D':
          ans += 500;
          break;
        case 'C':
          if (i+1 < n && (s[i+1] == 'D' || s[i+1] == 'M')){
            ans -= 100;
          } else{
            ans += 100;
          }
          break;
        case 'L':
          ans += 50;
          break;
        case 'X':
          if (i+1 < n && (s[i+1] == 'L' || s[i+1] == 'C')){
            ans -= 10;
          } else {
            ans += 10;
          }
          break;
        case 'V':
          ans += 5;
          break;
        case 'I':
          if (i+1 < n && (s[i+1] == 'V' || s[i+1] == 'X')){
            ans -= 1;
          } else {
            ans += 1;
          }
          break;
      }
    }
    return ans;
  }
}

public void TestRomanToInt(int testNum, string input, int expected) {
  Solution solution = new Solution();
  int result = solution.RomanToInt(input);
  
  Console.WriteLine($"Test {testNum}: {input} -> {result} / {expected} ({result == expected})");
}

TestRomanToInt(1, "III", 3);
TestRomanToInt(2, "LVIII", 58);
TestRomanToInt(3, "MCMXCIV", 1994);

TestRomanToInt(4, "IV", 4);
TestRomanToInt(5, "IX", 9);