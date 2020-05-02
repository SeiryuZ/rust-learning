
#[derive(Debug)]
struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}


impl <T: Ord + Clone > Tree<T> {

    pub fn new() -> Self {
        Self{root: None}
    }

    pub fn in_order_traversal(&self) -> Vec<T>{
        match self.root {
            None => return Vec::<T>::new(),
            Some(ref node) => node.traverse_in_order(),
        }
    }
    pub fn pre_order_traversal(&self) -> Vec<T>{
        match self.root {
            None => return Vec::<T>::new(),
            Some(ref node) => node.traverse_pre_order(),
        }
    }
    pub fn post_order_traversal(&self) -> Vec<T>{
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
                self.root = Some(Box::new(TreeNode::<T>{value: value, left: None, right: None}));
                return true
            },
            Some(ref mut root) => {
               current_node = root
            }
        }

        loop {

            if current_node.value == value {
                return false
            }


            if value < current_node.value {

                match current_node.left {
                    None => {
                        current_node.left = Some(Box::new(TreeNode::<T>{value: value, left: None, right: None}));
                        return true
                    },
                    Some(ref mut node) => {
                        current_node = node;
                        continue
                    }
                }
            }

            
            if value > current_node.value {

                match current_node.right {
                    None => {
                        current_node.right = Some(Box::new(TreeNode::<T>{value: value, left: None, right: None}));
                        return true
                    },
                    Some(ref mut node) => {
                        current_node = node;
                        continue
                    }
                }
            }
        }
    }

    pub fn search(&self, value: T) -> bool {
        let mut current_node;
        match self.root {
            None => return false,
            Some(ref root) => current_node = root
        }

        loop {

            if current_node.value == value {
                return true
            }

            if value < current_node.value {
                match current_node.left {
                    None => break,
                    Some(ref node) => current_node = node,
                }
            }
            else {
                match current_node.right {
                    None => break,
                    Some(ref node) => current_node = node,
                }
            }

        }
        return false;
    }

}


impl <T: Clone> TreeNode<T> {

    pub fn traverse_in_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self.left.as_ref().map_or(vec![], |left| left.traverse_in_order());
        let mut right = self.right.as_ref().map_or(vec![], |right| right.traverse_in_order());

        result.append(&mut left);
        result.push(self.value.clone());
        result.append(&mut right);

        return result;
    }

    pub fn traverse_pre_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self.left.as_ref().map_or(vec![], |left| left.traverse_in_order());
        let mut right = self.right.as_ref().map_or(vec![], |right| right.traverse_in_order());

        result.push(self.value.clone());
        result.append(&mut left);
        result.append(&mut right);

        return result;
    }

    pub fn traverse_post_order(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let mut left = self.left.as_ref().map_or(vec![], |left| left.traverse_in_order());
        let mut right = self.right.as_ref().map_or(vec![], |right| right.traverse_in_order());

        result.append(&mut left);
        result.append(&mut right);
        result.push(self.value.clone());

        return result;
    }
}


fn main() {
    let mut tree = Tree::<i32>::new();

    tree.insert(30);
    tree.insert(20);
    tree.insert(51);
    tree.insert(50);
    tree.insert(34);
    tree.insert(23);
    tree.insert(60);

    println!("");
    println!("{:?}", tree);
    println!("");
    println!("In order: {:?}", tree.in_order_traversal());
    println!("Pre order: {:?}", tree.pre_order_traversal());
    println!("post order: {:?}", tree.post_order_traversal());

    println!("Search 55: {:?}", tree.search(55));
    println!("Search 34: {:?}", tree.search(34));

}
