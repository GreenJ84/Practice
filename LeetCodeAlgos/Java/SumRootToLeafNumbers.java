// You are given the root of a binary tree containing digits from 0 to 9 only.

// Each root-to-leaf path in the tree represents a number.

// For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
// Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.

// A leaf node is a node with no children.

public class SumRootToLeafNumbers {
  public static void main(String[] args) {
    Solution s = new Solution();
    testSumNumbers(1, new TreeNode(1, new TreeNode(2), new TreeNode(3)), 25, s);

    testSumNumbers(2, new TreeNode(4,
        new TreeNode(9,
            new TreeNode(5),
            new TreeNode(1)
        ),
        new TreeNode(0)
    ), 1026, s);

    testSumNumbers(3, new TreeNode(9), 9, s);
  }

  public static void testSumNumbers(int testNum, TreeNode root, int expected, Solution s) {
    int result = s.sumNumbers(root);
    System.out.println("Test " + testNum + ": " + result);
    assert (result == expected);
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
    public int sumNumbers(TreeNode root) {
        if (root == null) {
            return 0;
        }
        return dp(root, "", 0);
    }

    int dp(TreeNode node, String currentPath, int sum) {
        if (node == null) {
            return 0;
        }
        currentPath += node.val;
        if (node.left == null && node.right == null) {
            return Integer.parseInt(currentPath);
        } else {
            return dp(node.left, currentPath, sum) + dp(node.right, currentPath, sum);
        }
    }
}