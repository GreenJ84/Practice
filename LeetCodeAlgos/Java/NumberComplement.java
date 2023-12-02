// The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.

// For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
// Given an integer num, return its complement.

class Solution {
    public int findComplement(int num) {
        String binNum = Integer.toBinaryString(num);
        String comp = new String();
        for (char ch: binNum.toCharArray()){
            comp += ch == '0' ? '1' : '0';
        }
        return Integer.parseInt(comp, 2);
    }
}

class NumberComplement {
  public static void main(String[] args) {
    Solution s = new Solution();

    int test1 = s.findComplement(5);
    System.out.println(test1);
    assert (test1 == 2);

    int test2 = s.findComplement(1);
    System.out.println(test2);
    assert (test2 == 0);
  }
}