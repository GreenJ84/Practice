# Given the head of a singly linked list, return true if it is a palindrome or false otherwise.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        if not head: return False
        runner, vals = head, []
        while runner:
            vals.append(runner.val)
            runner = runner.next
        j, i = 0, len(vals)-1
        while j<i:
            if vals[i] != vals[j]:
                return False
            i-=1
            j+=1
        return True
    
s = Solution()
print(s.isPalindrome(ListNode(1, ListNode(2, ListNode(3, ListNode(4))))))
print(s.isPalindrome(ListNode(1, ListNode(2))))