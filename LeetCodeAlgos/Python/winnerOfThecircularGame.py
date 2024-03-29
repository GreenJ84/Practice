# There are n friends that are playing a game. The friends are sitting in a circle and are numbered from 1 to n in clockwise order. More formally, moving clockwise from the ith friend brings you to the (i+1)th friend for 1 <= i < n, and moving clockwise from the nth friend brings you to the 1st friend.
# The rules of the game are as follows:
    # Start at the 1st friend.
    # Count the next k friends in the clockwise direction including the friend you started at. The counting wraps around the circle and may count some friends more than once.
    # The last friend you counted leaves the circle and loses the game.
    # If there is still more than one friend in the circle, go back to step 2 starting from the friend immediately clockwise of the friend who just lost and repeat.
    # Else, the last friend in the circle wins the game.
# Given the number of friends, n, and an integer k, return the winner of the game.

# Significant improvement on runtime and memory usage
class Solution:
    def findTheWinner(self, n: int, k: int) -> int:
        game = [i+1 for i in range(n)]
        pointer = 0
        while len(game) > 1:
            pointer = (pointer + k-1) % len(game)
            game.pop(pointer)
            if pointer > len(game)-1:
                pointer = pointer % len(game)
        return game[0]
    
# class Solution:
#     def findTheWinner(self, n: int, k: int) -> int:
#         game = [i+1 for i in range(n)]
#         pointer = 0
#         for _ in range(n - 1):
#             for c in range(k):
#                 if c == k-1:
#                     game.pop(pointer)
#                     pointer -= 1
#                 pointer = (pointer + 1) % len(game)
#         return game[0]

s = Solution()
# print(s.findTheWinner(5, 2))
print(s.findTheWinner(6, 5))