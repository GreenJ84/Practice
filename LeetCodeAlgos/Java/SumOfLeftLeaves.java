// Given the root of a binary tree, return the sum of all left leaves.

// A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.
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
  public int sumOfLeftLeaves(TreeNode root) {
    // Rule out no root or leaves
    if (root == null) {
      return 0;
    }
    return dp(root, false, 0);
  }

  private int dp(TreeNode node, boolean left, int sum) {
    // If a leaf and on left
    if ((node.left == null && node.right == null) && left) {
      sum += node.val;
    } else {
      // If left subs,  sum all left leaves
      sum += node.left != null ? dp(node.left, true, 0) : 0;
      // If right subs, sum all left leaves
      sum += node.right != null ? dp(node.right, false, 0) : 0;
    }
    // return sum
    return sum;
  }
}

class SumOfLeftLeaves {
  public static void main(String[] args) {
    Solution s = new Solution();

    TreeNode root1 = new TreeNode(3,
      new TreeNode(9),
      new TreeNode(20,
        new TreeNode(15),
        new TreeNode(7)
      )
    );
    testSumOfLeftLeaves(1, root1, 24, s);

    TreeNode root2 = new TreeNode(1);
    testSumOfLeftLeaves(2, root2, 0, s);
  }

  public static void testSumOfLeftLeaves(int testNum, TreeNode root, int expected, Solution s) {
    int result = s.sumOfLeftLeaves(root);
    System.out.println("Test " + testNum + ": " + result);
    assert (result == expected);
  }
}