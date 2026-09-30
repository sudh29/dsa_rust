#[derive(Debug, Clone)]
pub struct AVLNode {
    pub key: i32,
    pub height: i32,
    pub left: Option<Box<AVLNode>>,
    pub right: Option<Box<AVLNode>>,
}

impl AVLNode {
    pub fn new(key: i32) -> Self {
        AVLNode {
            key,
            height: 1,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree {
    pub root: Option<Box<AVLNode>>,
}

impl Default for AVLTree {
    fn default() -> Self {
        Self::new()
    }
}

impl AVLTree {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    fn height(node: &Option<Box<AVLNode>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn get_balance(node: &Option<Box<AVLNode>>) -> i32 {
        node.as_ref()
            .map_or(0, |n| Self::height(&n.left) - Self::height(&n.right))
    }

    pub fn insert(&mut self, key: i32) {
        self.root = Self::insert_node(self.root.take(), key);
    }

    fn insert_node(node: Option<Box<AVLNode>>, key: i32) -> Option<Box<AVLNode>> {
        let mut node = match node {
            Some(n) => n,
            None => return Some(Box::new(AVLNode::new(key))),
        };

        if key < node.key {
            node.left = Self::insert_node(node.left.take(), key);
        } else if key > node.key {
            node.right = Self::insert_node(node.right.take(), key);
        } else {
            return Some(node);
        }

        node.height = 1 + Self::height(&node.left).max(Self::height(&node.right));
        let balance = Self::get_balance(&Some(node.clone()));

        if balance > 1 && key < node.left.as_ref().unwrap().key {
            return Self::right_rotate(node);
        }
        if balance < -1 && key > node.right.as_ref().unwrap().key {
            return Self::left_rotate(node);
        }
        if balance > 1 && key > node.left.as_ref().unwrap().key {
            node.left = Self::left_rotate(node.left.take().unwrap());
            return Self::right_rotate(node);
        }
        if balance < -1 && key < node.right.as_ref().unwrap().key {
            node.right = Self::right_rotate(node.right.take().unwrap());
            return Self::left_rotate(node);
        }

        Some(node)
    }

    fn right_rotate(mut y: Box<AVLNode>) -> Option<Box<AVLNode>> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        y.height = 1 + Self::height(&y.left).max(Self::height(&y.right));
        x.right = Some(y);
        x.height = 1 + Self::height(&x.left).max(Self::height(&x.right));
        Some(x)
    }

    fn left_rotate(mut x: Box<AVLNode>) -> Option<Box<AVLNode>> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        x.height = 1 + Self::height(&x.left).max(Self::height(&x.right));
        y.left = Some(x);
        y.height = 1 + Self::height(&y.left).max(Self::height(&y.right));
        Some(y)
    }

    pub fn to_inorder(&self) -> Vec<i32> {
        let mut res = Vec::new();
        fn dfs(n: &Option<Box<AVLNode>>, res: &mut Vec<i32>) {
            if let Some(node) = n {
                dfs(&node.left, res);
                res.push(node.key);
                dfs(&node.right, res);
            }
        }
        dfs(&self.root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avl() {
        let mut avl = AVLTree::new();
        avl.insert(10);
        avl.insert(20);
        avl.insert(30);
        avl.insert(40);
        avl.insert(50);
        avl.insert(25);
        assert_eq!(avl.to_inorder(), vec![10, 20, 25, 30, 40, 50]);
    }
}
