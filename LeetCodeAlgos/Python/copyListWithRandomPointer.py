# A linked list of length n is given such that each node contains an additional random pointer, which could point to any node in the list, or null.
# Construct a deep copy of the list. The deep copy should consist of exactly n brand new nodes, where each new node has its value set to the value of its corresponding original node. Both the next and random pointer of the new nodes should point to new nodes in the copied list such that the pointers in the original list and copied list represent the same list state. None of the pointers in the new list should point to nodes in the original list.
# For example, if there are two nodes X and Y in the original list, where X.random --> Y, then for the corresponding two nodes x and y in the copied list, x.random --> y.
# Return the head of the copied linked list.
# The linked list is represented in the input/output as a list of n nodes. Each node is represented as a pair of [val, random_index] where:
    # val: an integer representing Node.val
    # random_index: the index of the node (range from 0 to n-1) that the random pointer points to, or null if it does not point to any node.
# Your code will only be given the head of the original linked list.

# Definition for a Node.
from typing import Optional
from collections import defaultdict


class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random


class Solution:
    def copyRandomList(self, head: Optional[Node]) -> Optional[Node]:
        if not head: return head
        head2 = Node(head.val)
        conn = defaultdict(list)

        runner1, runner2 = head, head2
        while runner1:
            if runner1.random == None:
                runner2.random == None
            else:
                conn[runner1.random].append(runner2)
            if not runner1.next: break
            runner1 = runner1.next
            runner2.next = Node(runner1.val)
            runner2 = runner2.next

        runner1, runner2 = head, head2
        while runner1 and conn:
            if runner1 in conn:
                for i in conn[runner1]:
                    i.random = runner2
                conn.pop(runner1, None)
            runner1 = runner1.next
            runner2 = runner2.next
        return head2

node5 = Node(1)
node4 = Node(10, node5)
node3 = Node(11, node4)
node2 = Node(13, node3)
node1 = Node(7, node2)

node2.random = node1
node3.random = node5
node4.random = node3
node2.random = node1

s = Solution()
print(s.copyRandomList(node1))
