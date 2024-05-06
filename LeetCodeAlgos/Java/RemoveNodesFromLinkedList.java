// You are given the head of a linked list.

// Remove every node which has a node with a greater value anywhere to the right side of it.

// Return the head of the modified linked list.

import java.util.Stack;

public class RemoveNodesFromLinkedList {
  
}

class ListNode {
  int val;
  ListNode next;
  ListNode() {}
  ListNode(int val) { this.val = val; }
  ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
  public ListNode removeNodes(ListNode head) {
    ListNode runner = head;
    Stack<ListNode> stack = new Stack<ListNode>();

    while (runner!= null) {
      while (!stack.isEmpty() && stack.peek().val < runner.val) {
        stack.pop();
      }
      stack.push(runner);
      runner = runner.next;
    }

    ListNode next = null;
    while (!stack.isEmpty()) {
      runner = stack.pop();
      runner.next = next;
      next = runner;
    }
    return next;
  }
}