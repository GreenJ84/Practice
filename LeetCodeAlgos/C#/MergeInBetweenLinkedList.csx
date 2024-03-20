// You are given two linked lists: list1 and list2 of sizes n and m respectively.
// Remove list1's nodes from the ath node to the bth node, and put list2 in their place.

public class ListNode {
    public int val;
    public ListNode next;
    public ListNode(int val=0, ListNode next=null) {
        this.val = val;
        this.next = next;
    }

    public string toString() {
      string items = "";
      while (this!= null) {
        items += $"{this.val} -> ";
      }
      return items.Remove(items.Length - 4);
    }
}

public class Solution {
    public ListNode MergeInBetween(ListNode list1, int a, int b, ListNode list2) {
        ListNode runner1 = list1;
        for (int idx = 1;int idx < a;int idx++){
          runner1 = runner1.next;
        }
        ListNode runner2 = runner1.next;
        for (int idx = a; idx < b; idx++) {
          runner2 = runner2.next;
        }
        runner1.next = list2;
        while (runner1.next != null){
          runner1 = runner1.next;
        }
        runner1.next = runner2.next;
        return list1;
    }
}
