# Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        runner = head
        while runner and runner.next:
            if runner.next.val == runner.val:
                runner.next = runner.next.next
            else:
                runner = runner.next
        return head
s = Solution()
print(s.deleteDuplicates(ListNode(1, ListNode(1, ListNode(2)))))
print(s.deleteDuplicates(ListNode( 1,ListNode( 1, ListNode( 2, ListNode( 3, ListNode( 3)))))))