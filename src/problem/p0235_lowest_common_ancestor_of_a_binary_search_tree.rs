/**
 * [235] Lowest Common Ancestor of a Binary Search Tree
 *
 * Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
 * Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
 * Output: 6
 * Explanation: The LCA of nodes 2 and 8 is 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
 * Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
 * Output: 2
 * Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
 *
 * Example 3:
 *
 * Input: root = [2,1], p = 2, q = 1
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 10^5].
 * 	-10^9 <= Node.val <= 10^9
 * 	All Node.val are unique.
 * 	p != q
 * 	p and q will exist in the BST.
 *
 */
//  Tree Depth-First Search Binary Search Tree Binary Tree
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// use std::borrow::Borrow;
use std::mem::swap;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {        
        let (pval, qval, rootval) = (
            p.as_ref().unwrap().borrow().val,
            q.as_ref().unwrap().borrow().val,
            root.as_ref().unwrap().borrow().val,
        );
        if (pval > qval) {
            return Solution::lowest_common_ancestor(root, q, p);
        }
        if pval <= rootval && rootval <= qval {
            root
        } else if qval < rootval {
            Solution::lowest_common_ancestor(root.unwrap().as_ref().borrow().left.clone(), p, q)
        } else {
            Solution::lowest_common_ancestor(root.unwrap().as_ref().borrow().right.clone(), p, q)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_235() {
        let mut rootnode = TreeNode::new(2);
        let rootleft = TreeNode::new(1);
        rootnode.left = Some(Rc::new(RefCell::new(rootleft)));
        let root = Some(Rc::new(RefCell::new(rootnode)));
        let rootle = root.clone().unwrap().clone().as_ref().borrow().left.clone();
        assert_eq!(
            Solution::lowest_common_ancestor(root.clone(), root.clone(), rootle),
            root.clone()
        );
    }
}
