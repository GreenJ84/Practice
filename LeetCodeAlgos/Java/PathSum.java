// Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

// A leaf is a node with no children.

public class PathSum {
  
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
  public boolean hasPathSum(TreeNode root, int targetSum) {
    if (root == null) return false;
    return hasPathSum(root, targetSum, 0);
  }
  public boolean hasPathSum(TreeNode node, int targetSum, int running) {
    int sum = node.val + running;
    if (node.left == null && node.right == null) return node.val + running == targetSum;
    if (node.left != null){
      if (hasPathSum(node.left, targetSum, sum)) return true;
    }
    if (node.right != null){
      if (hasPathSum(node.right, targetSum, sum)) return true;
    }
    return false;
  }
}