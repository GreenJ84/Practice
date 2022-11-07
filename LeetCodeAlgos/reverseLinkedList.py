# Given the head of a singly linked list, reverse the list, and return the reversed list.

from typing import Optional


class ListNode:
    def __init__(self, val = 0, next = None) -> None:
        self.val = val
        self.next = next

class Solution:
    def reverseList(self, head):
        if head == None or head.next == None:
            return head
        f, m = head, head.next,
        f.next = None
        while m:
            temp = f
            f = m
            m = m.next
            f.next = temp
        return f

s = Solution()
s.reverseList(ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))))