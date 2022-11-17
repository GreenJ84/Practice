# You are playing the Bulls and Cows game with your friend.
# You write down a secret number and ask your friend to guess what the number is. When your friend makes a guess, you provide a hint with the following info:
# The number of "bulls", which are digits in the guess that are in the correct position.
# The number of "cows", which are digits in the guess that are in your secret number but are located in the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
# Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.
# The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows. Note that both secret and guess may contain duplicate digits.

class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls, cows, los, log = 0, 0, [], []
        for i in range(0, len(secret)):
            if secret[i] == guess[i]:
                bulls += 1
            else:
                los.append(secret[i])
                log.append(guess[i])
        for i in log:
            if i in los:
                cows += 1
                los.pop(los.index(i))
        return '{}A{}B'.format(bulls, cows)

s = Solution()

print(s.getHint("1807","7810"))

