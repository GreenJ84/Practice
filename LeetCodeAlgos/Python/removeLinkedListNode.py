# Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.


from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        if not head:
            return head
        if head.val == val:
            return head.next
        runner = head
        while(runner):
            if runner.next:
                if runner.next.val == val:
                    runner.next = runner.next.next
                    continue
            runner = runner.next
        return head

