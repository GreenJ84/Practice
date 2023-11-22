# A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
# Implement the Trie class:
    # Trie() Initializes the trie object.
    # void insert(String word) Inserts the string word into the trie.
    # boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
    # boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.

class TrieNode:
    def __init__(self):
        self.children = [None]*26
        self.isEndOfWord = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        crawl = self.root
        for lvl in range(len(word)):
            char = ord(word[lvl])-ord('a')
            if not crawl.children[char]:
                crawl.children[char] = TrieNode()
            crawl = crawl.children[char]
        crawl.isEndOfWord = True

    def search(self, word: str) -> bool:
        crawl = self.root
        for lvl in range(len(word)):
            char = ord(word[lvl])-ord('a')
            if not crawl.children[char]:
                return False
            crawl = crawl.children[char]
        return crawl.isEndOfWord


    def startsWith(self, prefix: str) -> bool:
        crawl = self.root
        for lvl in range(len(prefix)):
            char = ord(prefix[lvl])-ord('a')
            if not crawl.children[char]:
                return False
            crawl = crawl.children[char]
        return True