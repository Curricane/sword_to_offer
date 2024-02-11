use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }

    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        assert_eq!(preorder.len(), inorder.len());
        if preorder.is_empty() {
            return None;
        }
        let val = preorder.first().unwrap();
        let val_idx = inorder.iter().position(|x| x == val).unwrap();
        let mut root_node = TreeNode::new(*val);
        root_node.left = Self::build(&preorder[1..val_idx + 1], &inorder[0..val_idx]);
        root_node.right = Self::build(
            &preorder[val_idx + 1..],
            &inorder[val_idx + 1..inorder.len()],
        );

        Some(Rc::new(RefCell::new(root_node)))
    }
}

fn preorder_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    if node.is_none() {
        return vec![None];
    }
    let node = node.unwrap();
    let mut ret = Vec::new();
    ret.push(Some(node.borrow().val));
    ret.extend(preorder_tree(node.borrow().left.clone()));
    ret.extend(preorder_tree(node.borrow().right.clone()));
    ret
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_s1() {
        let preorder = Vec::new();
        let inorder = Vec::new();
        let ret = Solution::build_tree(preorder, inorder);
        assert_eq!(ret, None);

        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let ret = Solution::build_tree(preorder, inorder);
        assert_eq!(
            preorder_tree(ret),
            vec![
                Some(3),
                Some(9),
                None,
                None,
                Some(20),
                Some(15),
                None,
                None,
                Some(7),
                None,
                None
            ]
        );
    }

    #[test]
    fn test_preorder_tree() {
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n2 = None;
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));

        n1.borrow_mut().left = n2;
        n1.borrow_mut().right = Some(n3);

        let ret = preorder_tree(Some(n1));
        assert_eq!(ret, vec![Some(1), None, Some(3), None, None]);
    }
}
fn main() {
    println!("Hello, world!");
}
