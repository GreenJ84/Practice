# You are given an array routes representing bus routes where routes[i] is a bus route that the ith bus repeats forever.
    # For example, if routes[0] = [1, 5, 7], this means that the 0th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.
# You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. You can travel between bus stops by buses only.
# Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.

from typing import List
from collections import defaultdict


class Solution:
    def numBusesToDestination(self, routes: List[List[int]], source: int, target: int) -> int:
        if source == target: return 0
        # Changes each route in routes to a set
        routes = list(map(set, routes))
        # Create a set dictionary to hold route connections
        graph = defaultdict(set[int])
        for routeI, route in enumerate(routes):
            for connI in range(routeI+1, len(routes)):
                connect = routes[connI]
                # Check if any of the route have a stop connection
                if any(r in connect for r in route):
                    # cross add connections
                    graph[routeI].add(connI)
                    graph[connI].add(routeI)
        # Create sets for routes to start and end on
        seen, targets = set(), set()
        for node, route in enumerate(routes):
            # Add the routes with the begining stop
            if source in route: seen.add(node)
            # Add the routes with the end stop
            if target in route: targets.add(node)

        queue = [(node, 1) for node in seen]
        for node, depth in queue:
            # if the current route connects to a target route
            if node in targets: return depth
            # iterate through the rest of the stops on the curr route
            for stop in graph[node]:
                # we haven't seen a stop before?
                if stop not in seen:
                    # Add the stop
                    seen.add(stop)
                    queue.append((stop, depth+1))
        # if we run out of stops then there is no way to get to target
        return -1


s = Solution()
print(s.numBusesToDestination([[1,2,7],[3,6,7]], 1, 6))
print(s.numBusesToDestination([[7,12],[4,5,15],[6],[15,19],[9,12,13]], 15, 12))
