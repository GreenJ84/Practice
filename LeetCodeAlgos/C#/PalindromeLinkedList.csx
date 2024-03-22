using System.Collections;
public class ListNode {
    public int val;
    public ListNode next;
    public ListNode(int val=0, ListNode next=null) {
        this.val = val;
        this.next = next;
    }
}

public class Solution {
  public bool IsPalindrome(ListNode head) {
    if (head.next == null){ return true; }
    ListNode slowRunner = head;
    ListNode fastRunner = head.next;
    Stack stack = new Stack();
    stack.Push(slowRunner.val);

    while (fastRunner.next != null) {
      fastRunner = fastRunner.next;

      // Odd Length
      if (fastRunner.next == null){
        slowRunner = slowRunner.next;
        break;
      } else {
        fastRunner = fastRunner.next;
        slowRunner = slowRunner.next;
        stack.Push(slowRunner.val);
      }
    }
    slowRunner = slowRunner.next;
    while (slowRunner != null){
      if ((int)stack.Pop() != slowRunner.val){ return false; }
      slowRunner = slowRunner.next;
    }
    return true;
  }
}