// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

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
    if (root == null || (root.left == null && root.right == null)) return true;
    else if (root.left == null ^ root.right == null) return false;
    else return followTree(root.left, root.right);
  }
  public boolean followTree(TreeNode left, TreeNode right) {
    // End of both tree lines
    if (left == null && right == null) return true;
    // If only one tree ends
    else if (left == null ^ right == null) {
        System.out.println("fail null match");
        return false;
      }
    // Check both current values
    if (left.val != right.val) {
        System.out.println("fail value match");
        return false;
      }
    // Outside node checks
    else if (left.left == null ^ right.right == null) {
        System.out.println("fail outside null match");
        return false;
      }
    // inside node checks
    else if (left.right == null ^ right.left == null) {
        System.out.println("fail inside null match");
        return false;
      }
    else if (!followTree(left.left, right.right)) {
        System.out.println("fail outside children");
        return false;
      }
    else if (!followTree(left.right, right.left)) {
        System.out.println("fail inside children");
        return false;
      }
    
    
    return followTree(left.left, right.right) && followTree(left.right, right.left);
  }
}

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
    System.out.println(s.isSymmetric(tree1));

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
    System.out.println(s.isSymmetric(tree2));
  }
}