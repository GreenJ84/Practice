# Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next: return head

        hold = ListNode(0, head)
        runner = hold
        while runner.next:
            if not runner.next.next: return hold.next
            # Variable for the next Node pair
            frontTemp = runner.next
            backTemp = runner.next.next
            # Attatch the front Node to the back of the list
            frontTemp.next = backTemp.next
            # Attach the back Node to the Front Node
            backTemp.next = frontTemp
            # Attatch the back Node to the front of the list
            runner.next = backTemp
            # Move the runner to check the next pair
            runner = runner.next.next
        return hold.next



