# Given the head of a linked list, remove the nth node from the end of the list and return its head.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        if not head.next:
            return None
        runner, length = head, 0
        while runner:
            length+=1
            runner = runner.next
        if n == length:
            return head.next
        runner = head
        for i in range(1, length):
            if i == length-n:
                temp = runner.next
                runner.next = temp.next
            runner = runner.next
        return head


