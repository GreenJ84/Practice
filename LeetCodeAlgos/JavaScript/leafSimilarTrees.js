// Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.
// Two binary trees are considered leaf-similar if their leaf value sequence is the same.
// Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
class TreeNode {
    val
    left
    right
    constructor(val = 0, left, right) {
        this.val = val
        this.left = left ?? null
        this.right = right ?? null
    }
}

/**
 * @param {TreeNode} root1
 * @param {TreeNode} root2
 * @return {boolean}
 */
const leafSimilar = function(root1, root2) {
  if (!root1 ^ !root2) return false;
  let leaves1 = [], leaves2 = [];
  depthFirstSearch(root1, leaves1);
  depthFirstSearch(root2, leaves2);
  return leaves1.toString() === leaves2.toString();
};

function depthFirstSearch(node, leafs) {
  if (!node.left && !node.right) leafs.push(node.val);
  if (node.left) leafs = depthFirstSearch(node.left, leafs);
  if (node.right) leafs = depthFirstSearch(node.right, leafs);
  return leafs;
}