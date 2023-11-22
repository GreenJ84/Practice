# Given an undirected tree consisting of n vertices numbered from 0 to n-1, which has some apples in their vertices. You spend 1 second to walk over one edge of the tree. Return the minimum time in seconds you have to spend to collect all apples in the tree, starting at vertex 0 and coming back to this vertex.
# The edges of the undirected tree are given in the array edges, where edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and bi. Additionally, there is a boolean array hasApple, where hasApple[i] = true means that vertex i has an apple; otherwise, it does not have any apple.

from typing import List


class Solution:
    def minTime(self, n: int, edges: List[List[int]], hasApple: List[bool]) -> int:
        # Array for each nodes connecting nodes
        conn = [[] for _ in range(n)]
        # loop through all edges adding connections both ways
        for i, j in edges:
            conn[i].append(j)
            conn[j].append(i)
        # For each node visited - no duplicates
        visited = set()
        # Depth First Search
        def dfs(node: int) -> int:
            # if we have already seen this node return
            if node in visited:
                return 0
            # add node to visited when first visiting
            visited.add(node)
            # Start seconds timer
            sec = 0
            # Start checking down nodes
            for child in conn[node]:
                sec += dfs(child)
            # any trips more than 1 sec add on the parents travel time
            if sec>0:
                return sec+2
            # any nodes 0 sec away
            return 2 if hasApple[node] else 0

        # Return a search on node 0 or just 0 if only 1 node
        return max(dfs(0)-2, 0)

s = Solution()
print(s.minTime(7, [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], [False,False,True,False,True,True,False]))
print(s.minTime(7, [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], [False,False,True,False,False,True,False]))
print(s.minTime(7, [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], [False,False,False,False,False,False,False]))
