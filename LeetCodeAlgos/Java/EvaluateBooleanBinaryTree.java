// You are given the root of a full binary tree with the following properties:
  // Leaf nodes have either the value 0 or 1, where 0 represents False and 1 represents True.
  // Non-leaf nodes have either the value 2 or 3, where 2 represents the boolean OR and 3 represents the boolean AND.
// The evaluation of a node is as follows:
  // If the node is a leaf node, the evaluation is the value of the node, i.e. True or False.
  // Otherwise, evaluate the node's two children and apply the boolean operation of its value with the children's evaluations.
// Return the boolean result of evaluating the root node.
// A full binary tree is a binary tree where each node has either 0 or 2 children.

// A leaf node is a node that has zero children.

public class EvaluateBooleanBinaryTree {
  public static void main(String[] args) {
    
  }
}

class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;
  TreeNode() {}
  TreeNode(int val) { this.val = val; }
  TreeNode(int val, TreeNode left, TreeNode right) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

class Solution {
  public boolean evaluateTree(TreeNode root) {
    if (root == null) return false;
    switch (root.val){
      case 0:
        return false;
      case 1:
        return true;
      case 2:
      case 3:
        if (root.left == null || root.right == null){
          throw new Error();
        }
        return root.val == 2 ?
          evaluateTree(root.left) || evaluateTree(root.right) :
          evaluateTree(root.left) && evaluateTree(root.right);
    }
    return false;
  }
}
