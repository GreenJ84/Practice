# There is an authentication system that works with authentication tokens. For each session, the user will receive a new authentication token that will expire timeToLive seconds after the currentTime. If the token is renewed, the expiry time will be extended to expire timeToLive seconds after the (potentially different) currentTime.
# Implement the AuthenticationManager class:
    # AuthenticationManager(int timeToLive) constructs the AuthenticationManager and sets the timeToLive.
    # generate(string tokenId, int currentTime) generates a new token with the given tokenId at the given currentTime in seconds.
    # renew(string tokenId, int currentTime) renews the unexpired token with the given tokenId at the given currentTime in seconds. If there are no unexpired tokens with the given tokenId, the request is ignored, and nothing happens.
    # countUnexpiredTokens(int currentTime) returns the number of unexpired tokens at the given currentTime.
# Note that if a token expires at time t, and another action happens on time t (renew or countUnexpiredTokens), the expiration takes place before the other actions.

class AuthenticationManager:

    def __init__(self, timeToLive: int):
        self.timeTL = timeToLive
        self.tokens = []

    def generate(self, tokenId: str, currentTime: int) -> None:
        self.tokens.append([tokenId, currentTime])

    def renew(self, tokenId: str, currentTime: int) -> None:
        for token in self.tokens:
            if token[0] == tokenId and token[1]+self.timeTL > currentTime:
                token[1] = currentTime


    def countUnexpiredTokens(self, currentTime: int) -> int:
        for token in self.tokens:
            res = 0
            if token[1]+self.timeTL > currentTime:
                res+=1
            return res



# Your AuthenticationManager object will be instantiated and called as such:
obj = AuthenticationManager(5)
obj.renew('aaa',1)
obj.generate('aaa',2)
print(obj.countUnexpiredTokens(6))
obj.generate('bbb',7)
obj.renew('aaa',8)
obj.renew('bbb',10)
print(obj.countUnexpiredTokens(15))
