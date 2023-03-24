# There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
# You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
# Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.

from typing import List


## Daily attempt 34/36 Tests Passed
class Solution:
    def makeConnected(self, n: int, connections: List[List[int]]) -> int:
        if len(connections)+1 < n:
            return -1
        hold = {}
        seen = []
        self.networks = 0
        for conn in connections:
            if conn[0] in hold:
                hold[conn[0]].add(conn[1])
            else:
                hold[conn[0]] = set([conn[1]])

            if conn[1] in hold:
                hold[conn[1]].add(conn[0])
            else:
                hold[conn[1]] = set([conn[0]])
        def check(idx):
            if idx not in seen:
                seen.append(idx)
                if idx in hold:
                    for cpu in hold[idx]:
                        check(cpu)
        for i in range(n):
            if i not in seen:
                self.networks += 1
                check(i)
        return self.networks - 1


s = Solution()
print(s.makeConnected(4, [[0,1],[0,2],[1,2]]))
print(s.makeConnected(6, [[0,1],[0,2],[0,3],[1,2],[1,3]]))
print(s.makeConnected(6, [[0,1],[0,2],[0,3],[1,2]]))