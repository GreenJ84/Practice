# You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a direct path going from cityAi to cityBi. Return the destination city, that is, the city without any path outgoing to another city.

# It is guaranteed that the graph of paths forms a line without any loop, therefore, there will be exactly one destination city.

from typing import List


class Solution:
    def destCity(self, paths: List[List[str]]) -> str:
        go = set()
        come = set()
        for path in paths:
            go.add(path[0])
            come.add(path[1])
        return (come - go).pop()

s = Solution()
print(s.destCity([["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]))
print(s.destCity([["B","C"],["D","B"],["C","A"]]))
print(s.destCity([["A","Z"]]))