use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::read_dir;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::vec;

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Tree {
    pub root_node: TreeNode,
}

#[derive(Serialize, Deserialize,PartialEq, Eq, Debug, Clone)]
pub struct TreeNode {
    pub id: u64,
    pub file_name: String,
    pub path: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub children: Option<Vec<TreeNode>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            root_node: TreeNode::new(),
        }
    }
}

impl Tree {
    pub fn find_node(&self, id: u64) -> Option<TreeNode> {
        //use BFS to find the node in the tree
        let mut q: VecDeque<&TreeNode> = VecDeque::new();

        q.push_back(&self.root_node);
        while !q.is_empty() {
            let node = q.pop_front();
            match node {
                Some(node) => {
                    if node.id == id {
                        return Some(node.clone());
                    }
                    let children = &node.children;
                    match children {
                        Some(children) => {
                            for child in children {
                                q.push_back(&child);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        None
    }

    pub fn expand_node(&mut self, id: u64) -> Result<& mut TreeNode> {
        //use BFS to find the right node that needs to be expanded
        let mut q: VecDeque<& mut TreeNode> = VecDeque::new();
        q.push_back(&mut self.root_node);
        while !q.is_empty() {
            let node = q.pop_front();
            match node {
                Some(mut node) => {
                    if node.id == id {
                       if node.is_dir {
                        node.expanded = true;
                        add_children(&mut node);
                        return Ok(node);
                       }                   
                    }
                    let ref mut children = node.children;
                    match children {
                        Some( children) => {
                            for child in children {
                                q.push_back(child);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        let custom_error = Error::new(ErrorKind::Other, "Error");
        Err(custom_error)

    }
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            id: 0,
            file_name: "".to_string(),
            path: "".to_string(),
            is_dir: false,
            expanded: false,
            children: None,
        }
    }
}

pub fn create_tree(dir: &Path) -> Result<Tree> {
    let mut root = path_to_node((&dir).to_path_buf())?;
    if dir.is_dir() {
        root.is_dir = true;
        root.expanded = true;
        add_children(&mut root);
    }
    let tree = Tree { root_node: root };
    Ok(tree)
}

fn add_children(node: &mut TreeNode) {
    if node.expanded {
        let mut children: Vec<TreeNode> = vec![];
        for entry in read_dir(&node.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let mut child = path_to_node(path.clone()).unwrap();
            if path.is_dir() {
                child.is_dir = true;
            }
            children.push(child);
        }
        children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir));
        node.children = Some(children);
    }
}

fn path_to_node<'a>(path: PathBuf) -> Result<TreeNode> {
    let path_string = path.to_string_lossy().to_string();
    let mut node = TreeNode::new();
    node.id = calculate_path_hash(&path);
    node.file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    node.path = path_string;
    Ok(node)
}

pub fn calculate_path_hash(t: &Path) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
