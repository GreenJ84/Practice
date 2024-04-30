public class LinkedListCycle {
  public static void main(String[] args) {
    
  }

  static void testHasCycle(int testNum, ListNode head, Boolean expected, Solution s) {
    Boolean result = s.hasCycle(head);

    System.out.println(String.format(
      "Test %d: %b / %b",
      testNum,
      result,
      expected
    ));
  }
}

class ListNode {
    int val;
    ListNode next;
    ListNode(int x) {
        val = x;
        next = null;
    }
}

class Solution {
    public boolean hasCycle(ListNode head) {
        if (head == null || head.next == null) return false;
        ListNode slow = head;
        ListNode fast = head.next;
        while (slow != fast) {
            if (fast == null || fast.next == null) return false;
            slow = slow.next;
            fast = fast.next.next;
        }
        return true;
    }
}