# Given the head of a singly linked list, return the middle node of the linked list.
# If there are two middle nodes, return the second middle node.
from typing import *

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cnt, run = 0, head
        while(run):
            cnt+=1
            run = run.next
        run = head
        cnt = int(cnt/2)
        for i in range(0, cnt):
            run = run.next
        return run

s = Solution()
s.middleNode(ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))) ))
s.middleNode(ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5, ListNode(6))))) ))
