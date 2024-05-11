// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.

public class MergeTwoSortedLists {
  
}

class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
  public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
      ListNode ans = new ListNode(Integer.MIN_VALUE);
      ListNode mutator = ans;
      while (list1 != null && list2 != null) {
        if (list1.val <= list2.val) {
          mutator.next = list1;
          list1 = list1.next;
        } else {
          mutator.next = list2;
          list2 = list2.next;
        }
        mutator = mutator.next;
        mutator.next = null;
      }
      mutator.next = list1 == null ? list2 : list1;
      return ans.next;
  }
}