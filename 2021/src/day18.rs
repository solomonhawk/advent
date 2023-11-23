use itertools::Itertools;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{self, Debug};
use std::ops::Add;
use std::rc::Rc;
use std::str::FromStr;

/**
 * This all seems like a LOT of ceremony just to keep track of the left/explode/
 * right nodes while traversing the tree. I can't just use `Option` b/c it
 * doesn't implement `Copy` (doesn't work when it's moved into the closure).
 *
 * The solution seems to be using RefCells around the options so that I can
 * mutate the inner values from the callback.
 *
 * The other struggle is accessing the values inside the nodes.
 *
 * I need some indirection for the recursive tree structure, but Box might be
 * sufficient over Rc. Where do I have shared ownership???
 */

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub enum Node<T> {
    Leaf(T),
    Branch { left: NodeRef<T>, right: NodeRef<T> },
}

#[derive(Clone, Debug)]
pub struct BTree<T> {
    root: NodeRef<T>,
}

impl BTree<u32> {
    fn reduce(&mut self) {
        loop {
            if self.explode() || self.split() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) -> bool {
        let left_node: RefCell<Option<NodeRef<u32>>> = RefCell::new(None);
        let right_node: RefCell<Option<NodeRef<u32>>> = RefCell::new(None);
        let exploding_node: RefCell<Option<NodeRef<u32>>> = RefCell::new(None);

        let callback = |node: &NodeRef<u32>, depth| {
            let mut left_node = left_node.borrow_mut();
            let mut right_node = right_node.borrow_mut();
            let mut exploding_node = exploding_node.borrow_mut();

            match &*node.borrow_mut() {
                Node::Leaf(_) => {
                    // if no exploding node, keep track of the last leaf
                    if exploding_node.is_none() {
                        *left_node = Some(Rc::clone(node))
                    // if no right node and this leaf isn't part of the exploding node
                    } else if right_node.is_none() {
                        match &*exploding_node.as_ref().unwrap().borrow() {
                            Node::Branch { left, right } => {
                                if !Rc::ptr_eq(left, node) && !Rc::ptr_eq(right, node) {
                                    *right_node = Some(Rc::clone(node))
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Node::Branch { left, right } => {
                    if exploding_node.is_none()
                        && depth > 4
                        && matches!(*left.borrow(), Node::Leaf(_))
                        && matches!(*right.borrow(), Node::Leaf(_))
                    {
                        *exploding_node = Some(Rc::clone(node));
                    }
                }
            }

            // if we found everything we can stop traversing
            left_node.is_some() && exploding_node.is_some() && right_node.is_some()
        };

        // traverse the tree collecting the relevant nodes
        traverse(&self.root, 1, callback);

        // if we found an exploding node
        if let Some(e) = exploding_node.borrow().as_ref() {
            let mut e = e.borrow_mut();

            if let Node::Branch { left, right } = &*e {
                match (&*left.borrow(), &*right.borrow()) {
                    (Node::Leaf(lv), Node::Leaf(rv)) => {
                        // if we found a left node, add the exploded left value to it
                        if let Some(l) = left_node.borrow().as_ref() {
                            let mut l = l.borrow_mut();

                            if let Node::Leaf(v) = &*l {
                                *l = Node::Leaf(*v + lv);
                            }
                        }

                        // if we found a right node, add the exploded right value to it
                        if let Some(r) = right_node.borrow().as_ref() {
                            let mut r = r.borrow_mut();

                            if let Node::Leaf(v) = &*r {
                                *r = Node::Leaf(*v + rv);
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            // set the exploded node to a leaf with 0 in it
            *e = Node::Leaf(0);
        }

        let found_exploding_node = exploding_node.borrow().is_some();
        found_exploding_node
    }

    fn split(&mut self) -> bool {
        let splitting_node: RefCell<Option<NodeRef<u32>>> = RefCell::new(None);

        let callback = |node: &NodeRef<u32>, _depth| {
            let mut mut_node = node.borrow_mut();
            let mut splitting_node = splitting_node.borrow_mut();

            match &*mut_node {
                Node::Leaf(v) => {
                    if splitting_node.is_none() && *v >= 10 {
                        let new_left = ((*v as f32) / 2.0).floor() as u32;
                        let new_right = ((*v as f32) / 2.0).ceil() as u32;

                        *mut_node = Node::Branch {
                            left: Rc::new(RefCell::new(Node::Leaf(new_left))),
                            right: Rc::new(RefCell::new(Node::Leaf(new_right))),
                        };

                        *splitting_node = Some(Rc::clone(node));

                        // if we found the splitting node we can stop traversing
                        return true;
                    }
                }
                _ => (),
            }

            false
        };

        // traverse the tree collecting the relevant nodes
        traverse(&self.root, 1, callback);

        let found_splitting_node = splitting_node.borrow().is_some();
        found_splitting_node
    }
}

impl FromStr for BTree<u32> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = "Invalid Snailfish input";
        let mut stack: Vec<Node<u32>> = Vec::new();

        for c in s.chars() {
            match c {
                ']' => {
                    let right = stack.pop().ok_or(error)?;
                    let left = stack.pop().ok_or(error)?;

                    stack.push(Node::Branch {
                        right: Rc::new(RefCell::new(right)),
                        left: Rc::new(RefCell::new(left)),
                    })
                }
                n if n >= '0' && n <= '9' => {
                    stack.push(Node::Leaf(
                        n.to_digit(10).ok_or("Failed to parse char to digit")?,
                    ));
                }
                _ => (),
            }
        }

        if stack.len() == 2 {
            let right = stack.pop().ok_or(error)?;
            let left = stack.pop().ok_or(error)?;

            stack.push(Node::Branch {
                right: Rc::new(RefCell::new(right)),
                left: Rc::new(RefCell::new(left)),
            });
        }

        Ok(BTree {
            root: Rc::new(RefCell::new(stack.pop().ok_or(error)?)),
        })
    }
}

impl Add for BTree<u32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut tree = BTree {
            root: Rc::new(RefCell::new(Node::Branch {
                left: self.root,
                right: other.root,
            })),
        };

        tree.reduce();

        tree
    }
}

impl<T: Display> Display for BTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.root.borrow())
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Leaf(v) => write!(f, "{}", v),
            Node::Branch { left, right } => write!(f, "[{},{}]", left.borrow(), right.borrow()),
        }
    }
}

fn traverse<T>(
    node: &NodeRef<T>,
    depth: usize,
    mut callback: impl FnMut(&NodeRef<T>, usize) -> bool + Copy,
) {
    if callback(&Rc::clone(&node), depth) {
        return;
    }

    match &*node.borrow() {
        Node::Leaf(_) => (),
        Node::Branch { left, right } => {
            traverse(&Rc::clone(left), depth + 1, callback);
            traverse(&Rc::clone(right), depth + 1, callback);
        }
    }
}

#[aoc_generator(day18, part1)]
pub fn input_generator_part1(input: &str) -> Vec<BTree<u32>> {
    input
        .split("\n")
        .map(|s| s.trim().parse().expect("Invalid puzzle input"))
        .collect()
}

#[aoc_generator(day18, part2)]
pub fn input_generator_part2(input: &str) -> Vec<Vec<BTree<u32>>> {
    input
        .split("\n")
        .permutations(2)
        .map(|s| {
            s.iter()
                .map(|c| c.parse().expect("Invalid puzzle input"))
                .collect()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(trees: &[BTree<u32>]) -> usize {
    trees
        .iter()
        .cloned()
        .reduce(|prev, next| prev + next)
        .map(magnitude)
        .expect("Trees should sum correctly")
}

#[aoc(day18, part2)]
pub fn part2(pairs: &[Vec<BTree<u32>>]) -> usize {
    pairs
        .iter()
        .map(|pair| {
            pair.iter()
                .cloned()
                .reduce(|prev, next| prev + next)
                .map(magnitude)
                .expect("Could not calculate a maximum")
        })
        .max()
        .expect("Could not calculate a maximum")
}

fn magnitude(tree: BTree<u32>) -> usize {
    node_magnitude(&tree.root)
}

fn node_magnitude(node: &Rc<RefCell<Node<u32>>>) -> usize {
    match &*node.borrow() {
        Node::Leaf(v) => *v as usize,
        Node::Branch { left, right } => 3 * node_magnitude(&left) + 2 * node_magnitude(&right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let tree: BTree<u32> = "[[9,1],[1,9]]".parse().unwrap();

        assert_eq!(magnitude(tree), 129);
    }
}
