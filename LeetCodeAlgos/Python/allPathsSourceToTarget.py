# Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1, find all possible paths from node 0 to node n - 1 and return them in any order.
# The graph is given as follows: graph[i] is a list of all nodes you can visit from node i (i.e., there is a directed edge from node i to node graph[i][j]).

from typing import List


class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        end = len(graph)-1
        ans = []
        def checkConnections(node, currPath):
            x = currPath[:]+[node]
            if node == end:
                ans.append(x)
            else:
                for conn in graph[node]:
                    if conn in x:
                        continue
                    else:
                        checkConnections(conn, x)

        checkConnections(0, [])
        return ans


s = Solution()
print(s.allPathsSourceTarget([[1,2],[3],[3],[]]))
print(s.allPathsSourceTarget([[4,3,1],[3,2,4],[3],[4],[]]))