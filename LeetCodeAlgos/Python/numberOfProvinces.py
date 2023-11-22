# There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
# A province is a group of directly or indirectly connected cities and no other cities outside of the group.
#* You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.
# Return the total number of provinces.

from typing import List


class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        seenCities = set()
        provinces = 0
        # Iterate all cities on a connected route
        def checkConnections(city):
            for neighbor, connection in enumerate(isConnected[city]):
                # Check valid connection to a unseenCities city
                if connection and neighbor not in seenCities:
                    seenCities.add(neighbor)
                    # Look at that cities connections
                    checkConnections(neighbor)


        for i in range(len(isConnected)):
            if not i in seenCities:
                seenCities.add(i)
                checkConnections(i)
                provinces += 1
        return provinces

s = Solution()
print(s.findCircleNum([[1,1,0],[1,1,0],[0,0,1]]))
print(s.findCircleNum([[1,0,0],[0,1,0],[0,0,1]]))

