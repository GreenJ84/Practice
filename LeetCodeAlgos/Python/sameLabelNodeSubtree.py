# You are given a tree (i.e. a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. The root of the tree is the node 0, and each node of the tree has a label which is a lower-case character given in the string labels (i.e. The node with the number i has the label labels[i]).
# The edges array is given on the form edges[i] = [ai, bi], which means there is an edge between nodes ai and bi in the tree.
# Return an array of size n where ans[i] is the number of nodes in the subtree of the ith node which have the same label as node i.
# A subtree of a tree T is the tree consisting of a node in T and all of its descendant nodes.

from typing import List

class Solution:
    def countSubTrees(self, n: int, edges: List[List[int]], labels: str) -> List[int]:
        if len(edges)<1:
            return [1]

        ans = [0]*n
        ed = [[] for _ in range(n)]

        for i, v in edges:
            ed[i].append(v)
        visited = set()

        def dfs(_node: int, st=None):
            visited.add(_node)
            for j in ed[_node]:
                if _node == 0:
                    dfs(j, {})
                else:
                    dfs(j, st)
            if labels[_node] not in st:
                st[labels[_node]] = 1
            else:
                st[labels[_node]]+=1
            ans[_node] = st[labels[_node]]
        dfs(0)
        return ans

s = Solution()
print(s.countSubTrees(7, [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], 'abaedcd'))
# print(s.countSubTrees(4, [[0,1],[1,2],[0,3]], "bbbb"))