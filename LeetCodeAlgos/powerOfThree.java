// Given an integer n, return true if it is a power of three. Otherwise, return false.

// An integer n is a power of three, if there exists an integer x such that n == 3x.

class powerOfThree {
    public boolean isPowerOfThree(int n) {
        double dividen = n * 1.0;
        while (dividen >= 1){
            if (dividen == 3 || dividen == 1){
                return true;
            }
            dividen = dividen / 3.0;
        }
        return false;
    }

    public static void main(String[] args) {
      Solution solution = new Solution();
      System.out.println(solution.isPowerOfThree(27));
      System.out.println(solution.isPowerOfThree(0));
      System.out.println(solution.isPowerOfThree(-1));
    }
}