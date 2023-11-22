# Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.

from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        runner = head
        cnt = {}

        while runner:
            if runner.val in cnt:
                cnt[runner.val]+=1
            else:
                cnt[runner.val]=1
            runner = runner.next

        single = [k for k,v in cnt.items() if v == 1]
        runner = head
        while runner.next:
            if runner.next.val not in single:
                runner.next = runner.next.next
            else:
                runner = runner.next
        return head if head.val in single else head.next