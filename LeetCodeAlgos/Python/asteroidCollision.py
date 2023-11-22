# We are given an array asteroids of integers representing asteroids in a row.
# For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.
# Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.

from typing import List


class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        i = 0
        while i < len(asteroids)-1:
            i = max(i, 0)
            if len(asteroids) == 1: break
            # Asteroids same direction dont touch
            if (asteroids[i] >= 0 and asteroids[i+1] >= 0) or (asteroids[i] < 0 and asteroids[i+1] < 0):
                i+=1
                continue
            # asteroids going apart from each other
            elif asteroids[i] < 0 and asteroids[i+1] >= 0:
                i+=1
                continue
            # Asteroids are colliding
            else:
                if abs(asteroids[i]) == abs(asteroids[i+1]):
                    asteroids.pop(i)
                    asteroids.pop(i)
                    i-=2
                elif abs(asteroids[i]) < abs(asteroids[i+1]):
                    asteroids.pop(i)
                    i-=2
                else:
                    asteroids.pop(i+1)
                    i-=1
            i+=1
        return asteroids
    
s = Solution()
# print(s.asteroidCollision([5,10,-5]))
# print(s.asteroidCollision([8,-8]))
# print(s.asteroidCollision([10,2,-5]))
# print(s.asteroidCollision([-2,-1,1,2]))
# print(s.asteroidCollision([1,-2,-2,1]))
print(s.asteroidCollision([1,-1,1,-2]))