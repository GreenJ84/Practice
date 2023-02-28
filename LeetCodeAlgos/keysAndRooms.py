# There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.
# When you visit a room, you may find a set of distinct keys in it. Each key has a number on it, denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.
# Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited room i, return true if you can visit all the rooms, or false otherwise.

from typing import List

class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        visited = []
        def openRoom(idx):
            visited.append(idx)
            for key in rooms[idx]:
                if key not in visited:
                    openRoom(key)
        openRoom(0)
        return len(visited) == len(rooms)

# class Solution:
#     def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
#         keys = [0]
#         visited = []
#         def openRoom(idx):
#             visited.append(idx)
#             keys.remove(idx)
#             for key in rooms[idx]:
#                 if key not in keys and key not in visited:
#                     keys.append(key)
#             for key in keys:
#                 openRoom(key)
#         openRoom(0)
#         return len(visited) == len(rooms)

s = Solution()
print(s.canVisitAllRooms([[1],[2],[3],[]]))
print(s.canVisitAllRooms([[1,3],[3,0,1],[2],[0]]))