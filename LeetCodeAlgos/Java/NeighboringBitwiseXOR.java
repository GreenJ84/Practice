// A 0-indexed array derived with length n is derived by computing the bitwise XOR (⊕) of adjacent values in a binary array original of length n.
// Specifically, for each index i in the range [0, n - 1]:
// If i = n - 1, then derived[i] = original[i] ⊕ original[0].
// Otherwise, derived[i] = original[i] ⊕ original[i + 1].
// Given an array derived, your task is to determine whether there exists a valid binary array original that could have formed derived.
// Return true if such an array exists or false otherwise.
// A binary array is an array containing only 0's and 1's

public class NeighboringBitwiseXOR {
  public static void main(String[] args) {
    NeighboringBitwiseXOR obj = new NeighboringBitwiseXOR();

    System.out.println(obj.doesValidArrayExist(new int[]{1, 1, 0})); // true
    System.out.println(obj.doesValidArrayExist(new int[]{1, 1})); // true
    System.out.println(obj.doesValidArrayExist(new int[]{1, 0})); // false
  }

  public boolean doesValidArrayExist(int[] derived) {
    int XOR = 0;
    for (int element : derived) {
        XOR = XOR ^ element;
    }
    return XOR == 0;
}
}
