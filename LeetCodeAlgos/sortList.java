// Given the head of a linked list, return the list after sorting it in ascending order.

/**
 * Definition for singly-linked list.
 */
import java.util.ArrayList;
import java.util.Collections;

class ListNode {
    Integer val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { 
      this.val = val; 
    }
    ListNode(int val, ListNode next) { 
      this.val = val; 
      this.next = next; 
    }


    @Override
    public String toString() {
      String s = new String();
      ListNode node = this;
      if (node.val == null){
        return s;
      }
      while (node != null){
        s += node.val.toString() + " -> ";
        node = node.next;
      }
      return s;
    }
}


class Solution {
    Solution() {}
    // Iterative implementation
    public ListNode sortList(ListNode head) {
        if (head == null || head.next == null){
          return head;
        }

        ArrayList<Integer> vals = new ArrayList<Integer>();
        while (head != null ){
            vals.add(head.val);
            head = head.next;
        }
        Collections.sort(vals);
        ListNode newHead = new ListNode(vals.get(0));
        ListNode headRef = new ListNode(484, newHead);
        for (int i = 1; i < vals.size(); i++){
          newHead.next = new ListNode(vals.get(i));
          newHead = newHead.next;
        }
        return headRef.next;
    }

    // Recursive implementation
    // public ListNode sortList(ListNode head) {
    //     if (head == null || head.next == null){
    //       return head;
    //     }

    //     ArrayList<Integer> vals = new ArrayList<Integer>();
    //     while (head != null ){
    //         vals.add(head.val);
    //         head = head.next;
    //     }
    //     Collections.sort(vals);
    //     return addNode(0, vals.size() - 1, vals);
    // }

    public static ListNode addNode(int idx, int max, ArrayList<Integer> vals){
      if (idx < max){
        return new ListNode(vals.get(idx), addNode(idx + 1, max, vals));
      } else {
        return new ListNode(vals.get(max));
      }
    }
}

public class sortList {
  public static void main(String[] args) {
    Solution s = new Solution();
    System.out.println(s.sortList(new ListNode()));

    ListNode test1 = new ListNode(1);
    System.out.println(s.sortList(test1));

    ListNode test2 = new ListNode(1, new ListNode(2));
    System.out.println(s.sortList(test2));

    ListNode test3 = new ListNode(4, new ListNode(2, new ListNode(1, new ListNode(3))));
    System.out.println(s.sortList(test3));
  }
}


