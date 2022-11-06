mod tree;

use crate::tree::b_tree::b_tree;

fn main() {
    let tree = b_tree::new_tree(3);
    tree.insert(10);
}