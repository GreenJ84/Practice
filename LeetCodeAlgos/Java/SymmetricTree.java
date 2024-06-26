// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

public class SymmetricTree {
  public static void main(String[] args) {
    Solution s = new Solution();
    TreeNode tree1 = new TreeNode(1,
      new TreeNode(2,
        new TreeNode(3),
        new TreeNode(4)
      ),
      new TreeNode(2,
        new TreeNode(4),
        new TreeNode(3)
      )
    );
    testSymmetricTree(1, tree1, true, s);

  TreeNode tree2 = new TreeNode(1,
      new TreeNode(2,
        null,
        new TreeNode(3)
      ),
      new TreeNode(2,
        null,
        new TreeNode(3)
      )
    );
    testSymmetricTree(2, tree2, false, s);
  }

  static void testSymmetricTree(int testNum, TreeNode tree, boolean expected, Solution s) {
    boolean result = s.isSymmetric(tree);

    System.out.println(String.format(
      "Test %d: %b / %b",
      testNum,
      result,
      expected
    ));
    assert (result == expected);
  }
}

// Definition for a binary tree node.
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
  public boolean isSymmetric(TreeNode root) {
    if (root.left == null && root.right == null) return true;
    else if (root.left == null ^ root.right == null) return false;
    else return followTree(root.left, root.right);
  }

  public boolean followTree(TreeNode left, TreeNode right) {
    // End of both tree lines
    if (left == null && right == null) return true;
    // If only one tree ends
    else if ((left == null ^ right == null) || left.val != right.val) {
        return false;
    }
    
    return followTree(left.left, right.right) && followTree(left.right, right.left);
  }
}