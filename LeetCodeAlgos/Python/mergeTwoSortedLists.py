# You are given the heads of two sorted linked lists list1 and list2.
# Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
# Return the head of the merged linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeTwoLists(self, list1, list2):
        res = newNode =  ListNode()
        while list1 and list2:
            if list1.val <= list2.val:
                newNode.next = list1
                list1 = list1.next
                newNode = newNode.next
            else:
                newNode.next = list2
                list2 = list2.next
                newNode = newNode.next
        if list1:
            newNode.next = list1
        if list2:
            newNode.next = list2
        return res.next

s = Solution()
print(s.mergeTwoLists(ListNode(1, ListNode(2, ListNode(4))),ListNode(1, ListNode(3, ListNode(5)))))
print(s.mergeTwoLists([],[]))
print(s.mergeTwoLists([],[ListNode(0)]))


