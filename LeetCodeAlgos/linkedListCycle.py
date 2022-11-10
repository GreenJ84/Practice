# Given the head of a linked list, return the node where the cycle begins. If there is no cycle, return null.
# There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to (0-indexed). It is -1 if there is no cycle. Note that pos is not passed as a parameter.
# Do not modify the linked list.

from typing import *

class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        f1 = s1 = head
        while(f1 and f1.next):
            f1 = f1.next.next
            s1 = s1.next
            if f1 == s1:
                s1 = head
                while(f1 != s1):
                    f1 = f1.next
                    s1 = s1.next
                return f1
        return None

s = Solution()