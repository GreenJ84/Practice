// You are given a positive number n.

// Return the smallest number x greater than or equal to n, such that the binary representation of x contains only set bits

public class SmallestNumberWithAllSetBits {
  public static void main(String[] args) {
    SmallestNumberWithAllSetBits obj = new SmallestNumberWithAllSetBits();

    System.out.println(obj.smallestNumber(5));
    System.out.println(obj.smallestNumber(10));
    System.out.println(obj.smallestNumber(3));
  }

  public int smallestNumber(int n) {
    return Integer.parseInt(
      "1".repeat(Integer.toBinaryString(n).length()),
      2
    );
  }
}