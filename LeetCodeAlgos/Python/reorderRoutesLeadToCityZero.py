# There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
# Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
# This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
# Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
# It's guaranteed that each city can reach city 0 after reorder.

from typing import List
from collections import defaultdict

## Daily Challenge Attempt 71/76 Test cases passed
class Solution:
    def minReorder(self, n: int, connections: List[List[int]]) -> int:
        mgr = defaultdict(list)
        for fro, to in connections:
            mgr[fro].append([to, True])
            mgr[to].append([fro, False])
        ans = 0
        queue = [0]
        seen = []
        while queue:
            city = queue.pop()
            seen.append(city)
            # Search all connections
            for route, canVisit in mgr[city]:
                # Get ready to search next city
                if route not in seen:
                    queue.append(route)
                # if canVisit from center, can't visit center from here
                    if canVisit:
                        ans += 1
        return ans
    
s = Solution()
print(s.minReorder(6, [[0,1],[1,3],[2,3],[4,0],[4,5]]))
print(s.minReorder(5, [[1,0],[1,2],[3,2],[3,4]]))
print(s.minReorder(3, [[1,0],[2,0]]))