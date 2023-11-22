// Given an integer n, return a string array answer (1-indexed) where:

// answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
// answer[i] == "Fizz" if i is divisible by 3.
// answer[i] == "Buzz" if i is divisible by 5.
// answer[i] == i (as a string) if none of the above conditions are true.
import java.util.*;

class Solution {
    public List<String> fizzBuzz(int n) {
        ArrayList<String> ans = new ArrayList<>();
        for (int i = 1; i <= n; i++) {
          if (i % 3 == 0 && i % 5 == 0) {
            ans.add("FizzBuzz");
          }
          else if (i % 3 == 0) {
            ans.add("Fizz");
          }
          else if (i % 5 == 0) {
            ans.add("Buzz");
          }
          else {
            ans.add(String.valueOf(i));
          }
        }
        return ans;
    }
}

class FizzBuzz {
  public static void main(String[] args){
    Solution solution = new Solution();
    printArray(solution.fizzBuzz(3));
    printArray(solution.fizzBuzz(5));
    printArray(solution.fizzBuzz(15));
  }
  
  public static void printArray(List<String> elems){
        System.out.print("[");
        for (int i = 0; i < elems.size(); i++) {
            System.out.print(elems.get(i));
            if (i < elems.size() - 1) {
              System.out.print(", ");
            }
        }
        System.out.print("]");
        System.out.println();
        System.out.println("-----------------");
  }
}