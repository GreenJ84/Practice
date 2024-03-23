// You are given the head of a singly linked-list. The list can be represented as:
  //! L0 → L1 → … → Ln - 1 → Ln
// Reorder the list to be on the following form:
  //! L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
public class ListNode {
    public int val;
    public ListNode next;
    public ListNode(int val=0, ListNode next=null) {
        this.val = val;
        this.next = next;
    }
}

public class Solution {
    public void ReorderList(ListNode head) {
        if ( head.next == null || head.next.next == null ){ return;}
        ListNode slowRunner = head;
        ListNode fastRunner = head.next;
        while (fastRunner.next != null){
            fastRunner = fastRunner.next;
            slowRunner = slowRunner.next;
            if (fastRunner.next != null){
                fastRunner = fastRunner.next;
            }
        }
        fastRunner = slowRunner.next;
        slowRunner.next = null;
        slowRunner = head;

        Stack stk = new Stack();
        while (fastRunner != null){
            stk.Push(fastRunner.val);
            fastRunner = fastRunner.next;
        }
        while (stk.Count > 0){
            ListNode temp = slowRunner;
            slowRunner = slowRunner.next;
            temp.next = new ListNode((int)stk.Pop());
            temp = temp.next;
            temp.next = slowRunner;
        }
    }

}