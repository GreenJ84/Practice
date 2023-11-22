# You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.

# Return the head of the linked list after doubling it.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def doubleIt(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return head

        def double(node):
            if node.next is None:
                temp = node.val * 2
            else:
                extra = double(node.next)
                temp = node.val * 2 + extra
            
            node.val = temp * 10
            return 1 if temp > 9 else 0

        extra = double(head)
        if extra:
            return ListNode(1, head)
        return head