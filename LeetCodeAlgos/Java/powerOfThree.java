// Given an integer n, return true if it is a power of three. Otherwise, return false.

// An integer n is a power of three, if there exists an integer x such that n == 3x.

class Solution {
    public boolean isPowerOfThree(int n) {
        double dividend = n * 1.0;
        while (dividend >= 1){
            if (dividend == 3 || dividend == 1){
                return true;
            }
            dividend = dividend / 3.0;
        }
        return false;
    }
}

class powerOfThree {
    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.isPowerOfThree(27));
        System.out.println(solution.isPowerOfThree(0));
        System.out.println(solution.isPowerOfThree(-1));
    }
}
