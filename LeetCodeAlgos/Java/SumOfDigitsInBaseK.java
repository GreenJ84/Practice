// Given an integer n (in base 10) and a base k, return the sum of the digits of n after converting n from base 10 to base k.

// After converting, each digit should be interpreted as a base 10 number, and the sum should be returned in base 10.

class Solution {
    public int sumBase(int n, int k) {
        String baseK = Integer.toString(n, k);
        int sum = 0;
        for (int idx = 0; idx < baseK.length(); idx++){
            sum += Character.getNumericValue(baseK.charAt(idx));
        }
        return sum;
    }
}

class SumOfDigitsInBaseK {
  public static void main(String[] args) {
    Solution s = new Solution();

    int test1 = s.sumBase(34, 6);
    System.out.println(test1);
    assert test1 == 9;

    int test2 = s.sumBase(10, 10);
    System.out.println(test2);
    assert test2 == 1;
  }
}