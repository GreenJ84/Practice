# A bus has n stops numbered from 0 to n - 1 that form a circle. We know the distance between all pairs of neighboring stops where distance[i] is the distance between the stops number i and (i + 1) % n.

# The bus goes along both directions i.e. clockwise and counterclockwise.

# Return the shortest distance between the given start and destination stops.

from typing import List


class Solution:
    def distanceBetweenBusStops(self, distance: List[int], start: int, destination: int) -> int:
        n = len(distance)
        print(n)
        left = right = 0
        leftCheck = rightCheck = True
        for i in range(n):
            if leftCheck:
                leftPoint = (start - i - 1) % n
                left += distance[leftPoint]
                print("Left:", leftPoint, distance[leftPoint], left)
            if leftPoint == destination:
                leftCheck = False

            if rightCheck:
                rightPoint = (start + i) % n
                right += distance[rightPoint]
                print("Right:", rightPoint + 1, distance[rightPoint], right)
                # Account for looping edge case
                if rightPoint + 1 == n:
                    if destination == 0:
                        rightCheck = False
            if rightPoint + 1 == destination:
                rightCheck = False

            print(f"Left: {left}, Right: {right}")

            if not leftCheck and not rightCheck:
                break;

        return min(left, right)
    
s = Solution()
print(s.distanceBetweenBusStops([1, 2, 3, 4], 0, 1))
print(s.distanceBetweenBusStops([1, 2, 3, 4], 0, 2))
print(s.distanceBetweenBusStops([1, 2, 3, 4], 0, 3))
print(s.distanceBetweenBusStops([6,47,48,31,10,27,46,33,52,33,38,25,32,45,36,3,0,33,22,53,8,13,18,1,44,41,14,5,38,25,48], 22, 0))