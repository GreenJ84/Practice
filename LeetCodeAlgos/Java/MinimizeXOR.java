// Given two positive integers num1 and num2, find the positive integer x such that:

// x has the same number of set bits as num2, and
// The value x XOR num1 is minimal.
// Note that XOR is the bitwise XOR operation.

// Return the integer x. The test cases are generated such that x is uniquely determined.

// The number of set bits of an integer is the number of 1's in its binary representation.

public class MinimizeXOR {
  public static void main(String[] args) {
    MinimizeXOR xor = new MinimizeXOR();

    // System.out.println(xor.minimizeXor(3, 5)); // Output: 3
    // System.out.println(xor.minimizeXor(1, 12)); // Output: 3
    // System.out.println(xor.minimizeXor(91, 18)); // Output: 80
    // System.out.println(xor.minimizeXor(59, 27)); // Output: 58
    // System.out.println(xor.minimizeXor(65, 84)); // Output: 67
    // System.out.println(xor.minimizeXor(131, 67)); // Output: 58
    System.out.println(xor.minimizeXor(1, 536870911)); // Output: 58
  }

  public int minimizeXor(int num1, int num2) {
    String str2 = Integer.toBinaryString(num2);
    int n = str2.length();
    int bits = 0;
    for (char ch : str2.toCharArray()){
      if (ch == '1' ) bits += 1;
    }

    String str1 = Integer.toBinaryString(num1);
    int maxLength = str1.length();
    if (str1.length() < n){
      maxLength = n;
      str1 = String.format("%" + maxLength + "s", str1).replace(' ', '0');
    }
    if (bits == maxLength) return num2;
    System.out.println(str1 + " " + str2);
    System.out.println(maxLength);


    StringBuilder ans = new StringBuilder(maxLength);
    for (int i = 0; i < maxLength; i++){
      if (str1.charAt(i) == '1' && bits > 0){
        ans.append('1');
        --bits;
      } else {
        ans.append('0');
      }

      System.out.println(bits + " " + i);
      System.out.println(bits > 0 && n - i - 1 - bits == 0);
      if (bits > 0 && maxLength - i - 1 - bits == 0){
        ans.append("1".repeat(bits));
        System.out.println("break" + ans.toString());
        break;
      }
      System.out.println(ans.toString());
    }
    return Integer.parseUnsignedInt(ans.toString(), 2);
  }
}
