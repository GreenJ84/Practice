// You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.
// For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.
// Return the head of the modified linked list.

import java.util.List;

public class MergeNodesInBetweenZeros {
  public static void main(String[] args) {}

  private static void testMergeNodes(int testNum, ListNode head, ListNode expected, Solution s) {}
}

class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
  public ListNode mergeNodes(ListNode head){
    ListNode result = new ListNode();
    ListNode mutator = result;
    int sum = 0;
    while (head!= null) {
      if (head.val == 0) {
        if (sum != 0) {
          mutator.next = new ListNode(sum);
          mutator = mutator.next;
          sum = 0;
        }
      } else {
        sum += head.val;
      }
      head = head.next;
    }
    return result.next;
  }
}