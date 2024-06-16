public class ValidateBinarySearchTree {
  public static void main(String[] args) {}
}

class Solution {
  private static boolean hitIntMax = false;
  private static boolean hitIntMin = false;
  public boolean isValidBST(TreeNode root) {
    // Shallow tree rule outs
    if (root == null ||
      (root.left == null && root.right == null)
    ) return true;
    // Setup min and max boundaries for checking
    if (root.val == Integer.MIN_VALUE) hitIntMin = true;
    else if (root.val == Integer.MAX_VALUE) hitIntMax = true;
    // Validate nodes as you traverse
    return validateNode(Integer.MIN_VALUE, root.val, root.left) && validateNode(root.val, Integer.MAX_VALUE, root.right);
  }

  // Validate node meets boundaries
  private static boolean validateNode(int min, int max, TreeNode node) {
    // Empty nodes auto pass (end of tree root)
    if (node == null) return true;
    // Validate node meets boundaries
    else if (node.val >= max || node.val <= min){
      // Single exemptions for first time reaching Integer boundaries
      if (node.val == max && max == Integer.MAX_VALUE && !hitIntMax) hitIntMax = true;
      else if (node.val == min && min == Integer.MIN_VALUE && !hitIntMin) hitIntMin = true;
      // Failed validation
      else return false;
    }
    // Validate children
    return validateNode(min, node.val, node.left) && validateNode(node.val, max, node.right);
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
