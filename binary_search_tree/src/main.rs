use std::fmt::Debug;

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone + Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        match self.root {
            None => return Vec::<T>::new(),
            Some(ref node) => node.traverse_in_order(),
        }
    }
    pub fn pre_order_traversal(&self) -> Vec<T> {
        match self.root {
            None => return Vec::<T>::new(),
            Some(ref node) => node.traverse_pre_order(),
        }
    }
    pub fn post_order_traversal(&self) -> Vec<T> {
        match self.root {
            None => return Vec::<T>::new(),
            Some(ref node) => node.traverse_post_order(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        /*
        1. Determine root TreeNode, create if not exists
        2. Starting from root TreeNode, check whether the value to insert is bigger/smaller than the current TreeNode
        3. Move current_node accordingly (left if smaller, right if bigger)
        4. If it hits an empty reference (None), create a new TreeNode and set current_node.left/right to refer to that new TreeNode
        */
        let mut current_node;
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::<T> {
                    value: value,
                    left: None,
                    right: None,
                }));
                return true;
            }
            Some(ref mut root) => current_node = root,
        }

        loop {
            if current_node.value == value {
                return false;
            }

            if value < current_node.value {
                match current_node.left {
                    None => {
                        current_node.left = Some(Box::new(TreeNode::<T> {
                            value: value,
                            left: None,
                            right: None,
                        }));
                        return true;
                    }
                    Some(ref mut node) => {
                        current_node = node;
                        continue;
                    }
                }
            }

            if value > current_node.value {
                match current_node.right {
                    None => {
                        current_node.right = Some(Box::new(TreeNode::<T> {
                            value: value,
                            left: None,
                            right: None,
                        }));
                        return true;
                    }
                    Some(ref mut node) => {
                        current_node = node;
                        continue;
                    }
                }
            }
        }
    }

    pub fn search(&self, value: T) -> bool {
        let mut current_node;
        match self.root {
            None => return false,
            Some(ref root) => current_node = root,
        }

        loop {
            if current_node.value == value {
                return true;
            }

            if value < current_node.value {
                match current_node.left {
                    None => break,
                    Some(ref node) => current_node = node,
                }
            } else {
                match current_node.right {
                    None => break,
                    Some(ref node) => current_node = node,
                }
            }
        }
        return false;
    }

    pub fn delete(&mut self, value: T) -> bool {
        match self.root {
            None => return false,
            Some(ref mut root) => {
                let result = root.delete(value);
                println!("DELETEEEEE {}", result);
                return result;
            }
        }
    }
}

impl<T: Ord + Clone + Debug> TreeNode<T> {
    pub fn traverse_in_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self
            .left
            .as_ref()
            .map_or(vec![], |left| left.traverse_in_order());
        let mut right = self
            .right
            .as_ref()
            .map_or(vec![], |right| right.traverse_in_order());

        result.append(&mut left);
        result.push(self.value.clone());
        result.append(&mut right);

        return result;
    }

    pub fn traverse_pre_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self
            .left
            .as_ref()
            .map_or(vec![], |left| left.traverse_in_order());
        let mut right = self
            .right
            .as_ref()
            .map_or(vec![], |right| right.traverse_in_order());

        result.push(self.value.clone());
        result.append(&mut left);
        result.append(&mut right);

        return result;
    }

    pub fn traverse_post_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self
            .left
            .as_ref()
            .map_or(vec![], |left| left.traverse_in_order());
        let mut right = self
            .right
            .as_ref()
            .map_or(vec![], |right| right.traverse_in_order());

        result.append(&mut left);
        result.append(&mut right);
        result.push(self.value.clone());

        return result;
    }

    pub fn is_leaf(&self) -> bool {
        if self.right.is_none() && self.left.is_none() {
            return true;
        }
        return false;
    }

    pub fn delete(&mut self, value: T) -> bool {
        if self.left.is_none() && self.right.is_none() {
            println!("LEAF: NOT FOUND");
            return false;
        }

        let mut next_node: &mut Option<Box<TreeNode<T>>>;
        if value < self.value {
            next_node = &mut self.left
        } else {
            next_node = &mut self.right
        }

        match &mut next_node {
            None => return false,
            Some(ref mut node) => {
                if node.value != value {
                    println!("MOVING TO: {:?}", node);
                    return node.delete(value);
                }

                if node.is_leaf() {
                    *next_node = None;
                    println!("IT IS LEAF, SAFE TO DELETE");
                    return true;
                } else {
                    // Only one child
                    if node.left.is_some() && node.right.is_none() {
                        node.value = node.left.as_ref().unwrap().value.clone();
                        node.left = None;
                        return true;
                    } else if node.right.is_some() && node.left.is_none() {
                        node.value = node.right.as_ref().unwrap().value.clone();
                        node.right = None;
                        return true;
                    }

                    // the fun part, where both child exists. Get the max on the left node
                    let mut max_node = node.left.as_mut().unwrap();
                    let max_value: T;
                    loop {
                        match max_node.right {
                            None => {
                                max_value = max_node.value.clone();
                                // TODO: fix this bug!
                                node.left = None;
                                break
                            },
                            Some(ref mut potential_max_node) => {
                                if potential_max_node.right.is_none() {
                                    max_value = potential_max_node.value.clone();
                                    max_node.right = None;
                                    break
                                }
                            }
                        }
                    }
                    println!("here {:?}", max_value);
                    node.value = max_value;
                    return true;
                }

            }
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::<i32>::new();

    tree.insert(30);
    tree.insert(20);
    tree.insert(51);
    tree.insert(50);
    tree.insert(34);
    tree.insert(23);
    tree.insert(60);
    tree.insert(65);

    tree.delete(51);

    println!("");
    println!("{:?}", tree);
    println!("");
    println!("In order: {:?}", tree.in_order_traversal());
    println!("Pre order: {:?}", tree.pre_order_traversal());
    println!("post order: {:?}", tree.post_order_traversal());

    println!("Search 55: {:?}", tree.search(55));
    println!("Search 34: {:?}", tree.search(34));

    
    println!("");
    println!("");


    let mut tree = BinarySearchTree::<String>::new();

    tree.insert("apa".to_string());
    tree.insert("kabar".to_string());
    tree.insert("darimana".to_string());
    tree.insert("test".to_string());
    tree.insert("saiko".to_string());
    tree.insert("last".to_string());
    tree.insert("dor".to_string());
    tree.insert("password".to_string());

    tree.delete("test".to_string());

    println!("");
    println!("{:?}", tree);
    println!("");
    println!("In order: {:?}", tree.in_order_traversal());
    println!("Pre order: {:?}", tree.pre_order_traversal());
    println!("post order: {:?}", tree.post_order_traversal());

    println!("Search 55: {:?}", tree.search("APA".to_string()));
    println!("Search 34: {:?}", tree.search("dor".to_string()));
}
