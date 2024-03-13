// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

public class Solution {
    public void Merge(int[] nums1, int m, int[] nums2, int n) {
        //* Console.WriteLine($"m: {m} - n: {n}");
        Queue<int> queue = new Queue<int>();

        int rightIdx = 0;
        for (int leftIdx = 0; leftIdx < nums1.Length; leftIdx++) {
          int useQueue = 0;
          bool pastHalf = leftIdx >= m;
          // Get left from nums1 or queue
          int leftNum = queue.TryPeek(out useQueue) ? useQueue : pastHalf ? int.MaxValue : nums1[leftIdx];
          // Get right from nums2
          int rightNum = rightIdx < n ? nums2[rightIdx] : int.MaxValue;
          //* Console.WriteLine($"{leftIdx}: [{string.Join(",", nums1)}] - {leftNum}\n{rightIdx}: [{string.Join(",", nums2)}] - {rightNum}\n");
          // If left greater and queue used
          if (leftNum <= rightNum && leftNum == useQueue) {
              // Add current base slot to queue
              if (!pastHalf){
                queue.Enqueue(nums1[leftIdx]);
              }
              // Move front of queue into current base slot
              nums1[leftIdx] = queue.Dequeue(); 
          } 
          // If right greater
          else if (leftNum > rightNum) {
            // Add current base slot to queue
            if (!pastHalf){
                queue.Enqueue(nums1[leftIdx]);
            }
            // Move right Array value into current base slot
            nums1[leftIdx] = nums2[rightIdx];
            // Move right Array
            rightIdx++;
          }
          // If left greater and queue not used -> continue
          //* Console.WriteLine($"[{string.Join(",", queue)}] :: [{string.Join(",", nums1)}]\n============");
        }
    }
}

public void TestMerge(int[] input1, int[] input2, int[] expected, int testNumber){
  Solution solution = new Solution();

  int[] copy = (int[])input1.Clone();
  int len2 = input2.Length;
  solution.Merge(input1, input1.Length - len2, input2, len2);
  bool passed = input1.SequenceEqual(expected);

  Console.WriteLine($"Test {testNumber}: [{string.Join(",", copy)}]|[{string.Join(",", input2)}] => [{string.Join(",", input1)}]/[{string.Join(",", expected)}]({passed})");
}

// TestMerge(new int[] {1,2,3,0,0,0}, new int[] {2, 5, 6}, new int[] {1, 2, 2, 3, 5, 6}, 1);
// TestMerge(new int[] {1}, new int[] {}, new int[] {1}, 2);
// TestMerge(new int[] {0}, new int[] {1}, new int[] {1}, 3);
TestMerge(new int[] {-1,0,0,0,3,0,0,0,0,0,0}, new int[] {-1,-1,0,0,1,2}, new int[] {-1,-1,-1,0,0,0,0,0,1,2,3}, 4);
