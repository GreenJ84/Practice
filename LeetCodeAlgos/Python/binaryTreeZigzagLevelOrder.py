# Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).

from typing import List, Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if not root:
            return [[]]
        res = []
        lvl, nxt = [root], []
        step = 1
        while lvl:
            for i in range(len(lvl)-1,-1,-1):
                if step % 2 == 0:
                    if lvl[i].left: nxt.append(lvl[i].left)
                    if lvl[i].right: nxt.append(lvl[i].right)
                else:
                    if lvl[i].right: nxt.append(lvl[i].right)
                    if lvl[i].left: nxt.append(lvl[i].left)
            res.append(list(map(lambda x: x.val, lvl)))
            lvl, nxt, = nxt, []
            step+=1
        return res
    
s = Solution()
print(s.zigzagLevelOrder(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))))
print(s.zigzagLevelOrder(TreeNode(1)))
print(s.zigzagLevelOrder(TreeNode()))