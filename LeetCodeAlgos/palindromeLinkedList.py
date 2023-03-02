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
print(s.isPalindrome(ListNode(1, ListNode(2, ListNode(2, ListNode(1))))))
print(s.isPalindrome(ListNode(1, ListNode(2, ListNode(3, ListNode(2, ListNode(1)))))))
print(s.isPalindrome(ListNode(1, ListNode(2))))
print(s.isPalindrome(ListNode(1, ListNode(2, ListNode(1)))))

## Improvement attempt: 77/90 passing
# class Solution:
#     def isPalindrome(self, head: Optional[ListNode]) -> bool:
#         if not head: return False
#         if not head.next: return True

#         runner1, runner2, vals = head, head, []
#         while runner2:
#             if runner2.next:
#                 runner2 = runner2.next
#             if runner2: 
#                 vals.append(runner1.val)
#                 if runner2.next:
#                     runner2 = runner2.next
#             else:
#                 break
#             runner1 = runner1.next
#         i = -1
#         while runner1:
#             if runner1.val != vals[i]:
#                 return False
#             i -= 1
#             runner1 = runner1.next
#         return True