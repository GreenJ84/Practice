# Given the head of a linked list, rotate the list to the right by k places.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        runner = head
        len = 1
        while runner.next:
            runner = runner.next
            len+=1

        if len == k:
            return head
        runner.next = head
        runner2 = head

        for _ in range(abs(len - (k % len))-1):
            runner2 = runner2.next
        res = runner2.next
        runner2.next = None
        print(res)
        return res
    
s = Solution()
print(s.rotateRight(
    ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))), 2
))
print(s.rotateRight(
    ListNode(0, ListNode(1, ListNode(2))), 4
))



