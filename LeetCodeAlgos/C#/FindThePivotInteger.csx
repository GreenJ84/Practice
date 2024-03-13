// Given a positive integer n, find the pivot integer x such that:

// The sum of all elements between 1 and x inclusively equals the sum of all elements between x and n inclusively.
// Return the pivot integer x. If no such integer exists, return -1. It is guaranteed that there will be at most one pivot index for the given input.

public class Solution {
    public int PivotInteger(int n) {
      if (n == 1){ return n; }
        int mid = n / 2;
        int left = rangeSum(1, mid);
        int right = rangeSum(mid, n);
        while (left < right){
            right -= mid;
            mid += 1;
            left += mid;
        }
        return left == right ? mid : -1;
    }

    public int rangeSum(int left, int right){
        int sum = left;
        for (int i = left + 1; i <= right; i++){
            sum += i;
        }
        return sum;
    }
}

public void TestPivotInteger(int input, int expected, int testNumber) {
  Solution solution = new Solution();

  int result = solution.PivotInteger(input);
  bool passed = result == expected;

  Console.WriteLine($"Test {testNumber}: {input} => {result}/{expected}({passed})");
}

TestPivotInteger(8, 6, 1);
TestPivotInteger(1, 1, 2);
TestPivotInteger(4, -1, 3);