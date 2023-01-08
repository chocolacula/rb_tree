use std::{cell::RefCell, cmp::Ordering, mem, ptr, rc::Rc};

mod node;
mod tests;
use node::*;
mod ancestor;
use ancestor::*;

#[derive(Debug)]
pub struct RbTree<T> {
    pub root: Option<Node<T>>,
    len: usize,
}

impl<T> RbTree<T>
where
    T: std::fmt::Debug + std::cmp::Ord + std::cmp::Eq + std::fmt::Display,
{
    // RbTree rules:
    // - root is BLACK
    // - every node is either RED or BLACK(obvious)
    // - all NIL nodes are considered BLACK
    //
    // main rules:
    // - there is no two consecutive RED nodes
    // - numbers of BLACK levels in left and right subtries are the same

    pub fn new() -> Self {
        RbTree { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn new_node(val: T, color: Color) -> Node<T> {
        Rc::new(RefCell::new(RbTreeNode {
            val: val,
            color: color,
            children: [None, None],
        }))
    }

    pub fn add(&mut self, val: T) {
        if self.root.is_none() {
            self.root = Some(Self::new_node(val, Color::Black));
            self.len += 1;
            return;
        }

        let mut ancestors = Vec::new();
        ancestors.push(Ancestor {
            node: self.root.as_ref().unwrap().clone(),
            position: Pos::LEFT,
        });

        self.add_and_fix(val, &mut ancestors);

        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
        self.len += 1;
    }

    fn add_and_fix(&mut self, val: T, ancestors: &mut Ancestry<T>) {
        // build hierarchy(ancestry)
        Self::find_leaf(&val, ancestors);

        let leaf = ancestors.last().unwrap().node.clone();
        let new_one = Self::new_node(val, Color::Red);

        // add new node
        let pos: usize;
        if new_one.borrow().val <= leaf.borrow().val {
            pos = Pos::LEFT;
            leaf.borrow_mut().children[Pos::LEFT] = Some(new_one.clone());
        } else {
            // val > leaf.val
            pos = Pos::RIGHT;
            leaf.borrow_mut().children[Pos::RIGHT] = Some(new_one.clone());
        }

        // add the node to the ancestry to further rebalancing
        ancestors.push(Ancestor {
            node: new_one.clone(),
            position: pos,
        });

        // rebalance if needed
        self.fix_insert(ancestors);
    }

    fn find_leaf(val: &T, ancestors: &mut Ancestry<T>) {
        let node = ancestors.last().unwrap().node.clone();
        let r = node.borrow();

        if *val <= r.val {
            if let Some(child) = r.children[Pos::LEFT].as_ref() {
                ancestors.push(Ancestor {
                    node: child.clone(),
                    position: Pos::LEFT,
                });
                return Self::find_leaf(val, ancestors);
            }
        } else {
            // val > node.val
            if let Some(child) = r.children[Pos::RIGHT].as_ref() {
                ancestors.push(Ancestor {
                    node: child.clone(),
                    position: Pos::RIGHT,
                });
                return Self::find_leaf(val, ancestors);
            }
        }
        // if there is no children, do nothing, we found a leaf
    }

    fn fix_insert(&mut self, ancestors: &mut Ancestry<T>) {
        if ancestors.len() <= 2 {
            return;
        }

        //    gparent
        //    /    \
        // uncle  parent
        //        /    \
        //   sibling   node

        let node = ancestors.pop().unwrap();
        let parent = ancestors.pop().unwrap();
        let gparent = ancestors.last().unwrap();

        if parent.node.borrow().color == Color::Black {
            // everything is already balanced
            return;
        }

        let uncle = gparent.node.borrow().children[Self::opposite_pos(parent.position)].clone();

        if let Some(uncle_node) = uncle {
            if uncle_node.borrow().color == Color::Red {
                uncle_node.borrow_mut().color = Color::Black;
                parent.node.borrow_mut().color = Color::Black;
                gparent.node.borrow_mut().color = Color::Red;

                self.fix_insert(ancestors);
                return;
            }
        }

        // uncle exists and has BLACK color

        if parent.position == Pos::RIGHT {
            if node.position == Pos::RIGHT {
                // <left rotation>
                // nodes are on the right side
                // p and n are RED
                // gp
                //  \
                //   p  ->   p
                //    \     / \
                //     n   gp  n

                parent.node.borrow_mut().color = Color::Black;
                gparent.node.borrow_mut().color = Color::Red;

                self.rotate_left(ancestors);
            } else {
                // <right left rotation>
                // nodes on different sides
                // p and n are RED
                // gp      gp
                //  \       \
                //   p  ->   n  ->   n
                //  /       / \     / \
                // n      nil  p   gp  p

                node.node.borrow_mut().color = Color::Black;
                gparent.node.borrow_mut().color = Color::Red;

                ancestors.push(parent);
                self.rotate_right(ancestors);
                ancestors.pop().unwrap();

                self.rotate_left(ancestors);
            }
        } else {
            if node.position == Pos::LEFT {
                // <right rotation>
                // nodes on the left side
                // p and n are RED
                //     gp
                //    /
                //   p  ->   p
                //  /       / \
                // n       n  gp

                parent.node.borrow_mut().color = Color::Black;
                gparent.node.borrow_mut().color = Color::Red;

                self.rotate_right(ancestors);
            } else {
                // <left right rotation>
                // nodes on different sides
                // p and n are RED
                //   gp      gp
                //  /       /
                // p  ->   n  ->   n
                //  \     / \     / \
                //   n   p  nil  p  gp

                node.node.borrow_mut().color = Color::Black;
                gparent.node.borrow_mut().color = Color::Red;

                ancestors.push(parent);
                self.rotate_left(ancestors);
                ancestors.pop().unwrap();

                self.rotate_right(ancestors);
            }
        }
    }

    pub fn remove(&mut self, val: &T) -> bool {
        if self.root.is_none() {
            return false;
        }

        let mut ancestors = Vec::new();
        ancestors.push(Ancestor {
            node: self.root.clone().unwrap(),
            position: Pos::LEFT,
        });

        let found = Self::find_node(val, &mut ancestors);
        if !found {
            return false;
        }

        self.remove_last(&mut ancestors);
        self.len -= 1;
        true
    }

    fn find_node(val: &T, ancestors: &mut Ancestry<T>) -> bool {
        let node = ancestors.last().unwrap().node.clone();
        let r = node.borrow();

        match r.val.cmp(val) {
            Ordering::Equal => true,
            Ordering::Greater => {
                if let Some(child) = r.children[Pos::LEFT].as_ref() {
                    ancestors.push(Ancestor {
                        node: child.clone(),
                        position: Pos::LEFT,
                    });
                    return Self::find_node(val, ancestors);
                }
                false
            }
            Ordering::Less => {
                if let Some(child) = r.children[Pos::RIGHT].as_ref() {
                    ancestors.push(Ancestor {
                        node: child.clone(),
                        position: Pos::RIGHT,
                    });
                    return Self::find_node(val, ancestors);
                }
                false
            }
        }
    }

    fn remove_last(&mut self, ancestors: &mut Ancestry<T>) {
        let has_left;
        let has_right;
        {
            let last = ancestors.last().unwrap();
            has_left = last.node.borrow().children[Pos::LEFT].is_some();
            has_right = last.node.borrow().children[Pos::RIGHT].is_some();
        }

        if has_left {
            if has_right {
                // has both children

                let old_n = ancestors.len();

                let right = Pos::RIGHT;
                let child = ancestors.last().unwrap().node.borrow().children[right]
                    .clone()
                    .unwrap();

                ancestors.push(Ancestor {
                    node: child,
                    position: right,
                });

                Self::find_min_node(ancestors);
                let new_n = ancestors.len();

                // swap nodes via references
                self.swap_nodes(ancestors, old_n - 1, new_n - 1);
                // or swap values
                // ancestors[old_n - 1].node.borrow_mut().swap(ancestors[new_n - 1].node.as_ptr());
                self.remove_last(ancestors);
            } else {
                // has only left child
                self.extract_node(ancestors, Pos::LEFT);
            }
        } else {
            if has_right {
                // has only right child
                self.extract_node(ancestors, Pos::RIGHT);
            } else {
                // replace to any child which is None
                self.extract_node(ancestors, Pos::LEFT);
            }
        }
    }

    fn find_min_node(ancestors: &mut Ancestry<T>) {
        debug_assert!(ancestors.len() >= 1);

        // traverse to the left subtree
        // it gives to us the minimum successor

        let node = ancestors.last().unwrap().node.clone();

        if node.borrow().children[Pos::LEFT].is_some() {
            // have left child
            let pos = Pos::LEFT;
            let next = node.borrow().children[pos].clone().unwrap();

            ancestors.push(Ancestor {
                node: next,
                position: pos,
            });

            return Self::find_min_node(ancestors);
        }
    }

    // keep length of ancestors, changes ancestors data only
    fn swap_nodes(&mut self, ancestors: &mut Ancestry<T>, a_i: usize, b_i: usize) {
        debug_assert!(a_i < b_i);
        debug_assert!(b_i < ancestors.len());

        // x -> a -> v1..vN -> b -> y
        //  \    \    \   \     \    \
        //   #    j    #   #     k    #

        // x -> b -> v1..vN -> a -> y
        //  \    \    \   \     \    \
        //   #    j    #   #     k    #

        // - 'x', 'v..' and 'y' are optional
        // - 'j' has opposite of 'v1' position
        // - 'v1' and 'vN' could be the same node
        // - 'a' accepts all children of 'b' after swap

        let a = &ancestors[a_i];
        let v1 = &ancestors[a_i + 1];
        let vN = &ancestors[b_i - 1];
        let b = &ancestors[b_i];

        // set 'x'
        if a_i > 0 {
            let x = &ancestors[a_i - 1];
            x.node.borrow_mut().children[a.position] = Some(b.node.clone());
        } else {
            self.root = Some(b.node.clone());
        }

        // save 'j'
        let j = a.node.borrow().children[Self::opposite_pos(v1.position)].clone();

        // set both children, 'y' and 'k'
        a.node.borrow_mut().children = b.node.borrow().children.clone();

        // set 'j'
        b.node.borrow_mut().children[Self::opposite_pos(v1.position)] = j;

        // set 'v1..vN'
        if v1.node.as_ptr() == b.node.as_ptr() {
            // 'v1' and 'b' is the same nodes, so we have a -> b case

            b.node.borrow_mut().children[v1.position] = Some(a.node.clone());
        } else {
            b.node.borrow_mut().children[v1.position] = Some(v1.node.clone());
            vN.node.borrow_mut().children[b.position] = Some(a.node.clone());
        }

        // swap colors together with references
        mem::swap(
            &mut a.node.borrow_mut().color,
            &mut b.node.borrow_mut().color,
        );

        // swap ancestry
        unsafe {
            ptr::swap(&mut ancestors[a_i].node, &mut ancestors[b_i].node);
        }
    }

    // extracts node from the tree, pops last ancestor from ancestors
    fn extract_node(&mut self, ancestors: &mut Ancestry<T>, child: usize) {
        let node = ancestors.pop().unwrap();
        let child_node = node.node.borrow_mut().children[child].take();

        // root is the target
        if ancestors.len() == 0 {
            self.root = match child_node {
                Some(c) => {
                    c.borrow_mut().color = Color::Black;
                    Some(c)
                }
                None => None,
            };
            return;
        }

        let parent = ancestors.last().unwrap();
        parent.node.borrow_mut().children[node.position] = child_node.clone();

        // keep red black properties
        if let Some(c) = child_node {
            if (node.node.borrow().color == Color::Red) || (c.borrow().color == Color::Red) {
                // prevent two consecutive red nodes
                c.borrow_mut().color = Color::Black;
            } else {
                // keep number of black nodes in a path
                self.fix_remove(ancestors, Self::opposite_pos(node.position));
            }
        } else {
            if node.node.borrow().color == Color::Black {
                // keep number of black nodes in a path
                self.fix_remove(ancestors, Self::opposite_pos(node.position));
            }
        }
    }

    fn fix_remove(&mut self, ancestors: &mut Ancestry<T>, sibling_position: usize) {
        if ancestors.len() == 0 {
            return;
        }
        //    gparent
        //    /    \
        // uncle  parent
        //        /    \
        //    sibling  node(extracted)
        //    /    \
        // nephew nephew

        let sibling;
        {
            let parent = ancestors.last().unwrap();
            sibling = parent.node.borrow().children[sibling_position].clone();
        }
        if let Some(sibling_node) = sibling {
            if sibling_node.borrow().color == Color::Black {
                let nephew_mask = Self::red_children(sibling_node.clone());

                // both nephews are BLACK
                if nephew_mask == 0b00 {
                    sibling_node.borrow_mut().color = Color::Red;
                    let parent = ancestors.pop().unwrap();

                    if parent.node.borrow().color == Color::Black {
                        // do that recursively
                        self.fix_remove(ancestors, Self::opposite_pos(parent.position));
                    } else {
                        parent.node.borrow_mut().color = Color::Black;
                    }
                    return;
                } else {
                    // one or both nephews are RED

                    let parent = ancestors.last().unwrap();

                    if sibling_position == Pos::LEFT {
                        // both or left nephew is RED
                        if nephew_mask == 0b11 || nephew_mask == 0b10 {
                            // <right rotation>
                            // nodes on the left side
                            // s is BLACK, left nephew is RED
                            //      p       s
                            //     /       / \
                            //    s  ->  nep  p
                            //   / \         /
                            // nep nep     nep
                            //
                            // s moved to p position, keep their colors
                            // on the same place
                            // set nep and p colors BLACK
                            // to follow black heights rule

                            sibling_node.borrow_mut().color = parent.node.borrow().color;
                            {
                                let nephew = sibling_node.borrow_mut().children[Pos::LEFT]
                                    .clone()
                                    .unwrap();
                                nephew.borrow_mut().color = Color::Black;
                            }
                            parent.node.borrow_mut().color = Color::Black;

                            self.rotate_right(ancestors);
                        } else {
                            // <left right rotation>
                            // nodes on different sides
                            // s is BLACK, right nephew is RED
                            //   p       p
                            //  /       /
                            // s  ->  nep  -> nep
                            //  \     / \     / \
                            //  nep  s   ?   s   p
                            //
                            // nep moved to p position, keep their colors
                            // on the same place
                            // set s color the same as p
                            // to follow black heights rule
                            {
                                let nephew = sibling_node.borrow_mut().children[Pos::RIGHT]
                                    .clone()
                                    .unwrap();
                                nephew.borrow_mut().color = parent.node.borrow().color;
                            }
                            parent.node.borrow_mut().color = Color::Black;

                            ancestors.push(Ancestor {
                                node: sibling_node,
                                position: Pos::LEFT,
                            });
                            self.rotate_left(ancestors);
                            ancestors.pop();

                            self.rotate_right(ancestors);
                        }
                    } else {
                        if nephew_mask == 0b11 || nephew_mask == 0b01 {
                            // <left rotation>
                            // nodes are on the right side
                            // s is BLACK
                            //  p          s
                            //   \        / \
                            //    s  ->  p  nep
                            //   / \      \
                            // nep nep    nep
                            //
                            // s moved to p position, keep their colors
                            // on the same place
                            // set nep and p colors BLACK
                            // to follow black heights rule

                            sibling_node.borrow_mut().color = parent.node.borrow().color;
                            {
                                let nephew = sibling_node.borrow_mut().children[Pos::RIGHT]
                                    .clone()
                                    .unwrap();
                                nephew.borrow_mut().color = Color::Black;
                            }
                            parent.node.borrow_mut().color = Color::Black;

                            self.rotate_left(ancestors);
                        } else {
                            // <right left rotation>
                            // nodes on different sides
                            // s is BLACK
                            //  p       p
                            //   \       \
                            //    s  ->  nep  -> nep
                            //   /       / \     / \
                            // nep      ?   s   p   s
                            //
                            // nep moved to p position, keep their colors
                            // on the same place
                            // set s color the same as p
                            // to follow black heights rule
                            {
                                let nephew = sibling_node.borrow_mut().children[Pos::LEFT]
                                    .clone()
                                    .unwrap();
                                nephew.borrow_mut().color = parent.node.borrow().color;
                            }
                            parent.node.borrow_mut().color = Color::Black;

                            ancestors.push(Ancestor {
                                node: sibling_node,
                                position: Pos::RIGHT,
                            });
                            self.rotate_right(ancestors);
                            ancestors.pop();

                            self.rotate_left(ancestors);
                        }
                    }
                }
            } else {
                let parent_node = ancestors.last().unwrap().node.clone();
                parent_node.borrow_mut().color = Color::Red;
                sibling_node.borrow_mut().color = Color::Black;

                if sibling_position == Pos::LEFT {
                    // <right rotation>
                    // nodes on the left side
                    // s is RED, so p and nep should be BLACK
                    //      p           s
                    //     / \         / \
                    //    s   n  ->  nep  p
                    //   / \         / \
                    // nep nep     nep  n
                    //
                    // keep color fixing from new deleted node position

                    self.rotate_right(ancestors);
                } else {
                    // <left rotation>
                    // nodes are on the right side
                    // s is RED, so p and nep should be BLACK
                    //   p           s
                    //  / \         / \
                    // n   s  ->   p   nep
                    //    / \     / \
                    //  nep nep  n  nep
                    //
                    // keep color fixing from new deleted node position

                    self.rotate_left(ancestors);
                }
                ancestors.push(Ancestor {
                    node: parent_node,
                    position: Self::opposite_pos(sibling_position),
                });
                self.fix_remove(ancestors, sibling_position);
            }
        } else {
            // if sibling is None, cannot balance on that level
            // do balancing on upper level
            //    gparent
            //    /    \
            // uncle  parent (next node)
            //        /   \
            //       nil child (node was deleted)

            let parent = ancestors.pop().unwrap();
            self.fix_remove(ancestors, Self::opposite_pos(parent.position));
        }
    }

    #[inline]
    fn red_children(node: Node<T>) -> u8 {
        let mut mask = 0;
        if let Some(l) = node.borrow().children[Pos::LEFT].as_ref() {
            mask |= (l.borrow().color == Color::Red) as u8;
        }
        mask <<= 1;
        if let Some(r) = node.borrow().children[Pos::RIGHT].as_ref() {
            mask |= (r.borrow().color == Color::Red) as u8;
        }
        mask
    }

    #[inline]
    fn opposite_pos(pos: usize) -> usize {
        (pos + 1) % 2
    }

    // rotation starts from grandparent which should be the last one in Ancestry
    // it makes a bit easier keeping Ancestry
    fn rotate_left(&mut self, ancestors: &mut Ancestry<T>) {
        let mut parent = ancestors.pop().unwrap();
        let pivot = parent.node.borrow().children[Pos::RIGHT].clone().unwrap();
        // could be None
        let rest = pivot.borrow().children[Pos::LEFT].clone();

        parent.node.borrow_mut().children[Pos::RIGHT] = rest;
        pivot.borrow_mut().children[Pos::LEFT] = Some(parent.node.clone());

        // exchange last ancestor from parent to pivot because of rotation
        parent.node = pivot.clone();

        if ancestors.len() > 0 {
            let gparent = ancestors.last().unwrap();
            gparent.node.borrow_mut().children[parent.position] = Some(pivot);
        } else {
            self.root = Some(pivot);
        }
        ancestors.push(parent);
    }

    // rotation starts from grandparent which should be the last one in Ancestry
    // it makes a bit easier keeping Ancestry
    fn rotate_right(&mut self, ancestors: &mut Ancestry<T>) {
        let mut parent = ancestors.pop().unwrap();
        let pivot = parent.node.borrow().children[Pos::LEFT].clone().unwrap();
        // could be None
        let rest = pivot.borrow().children[Pos::RIGHT].clone();

        parent.node.borrow_mut().children[Pos::LEFT] = rest;
        pivot.borrow_mut().children[Pos::RIGHT] = Some(parent.node.clone());

        // exchange last ancestor from parent to pivot because of rotation
        parent.node = pivot.clone();

        if ancestors.len() > 0 {
            let gparent = ancestors.last().unwrap();
            gparent.node.borrow_mut().children[parent.position] = Some(pivot);
        } else {
            self.root = Some(pivot);
        }
        ancestors.push(parent);
    }

    pub fn print(&self) {
        Self::print_rec("".to_string(), self.root.clone(), true);

        let v = if self.is_valid() {
            "valid"
        } else {
            "NOT valid"
        };
        println!("RbTree is {}", v);
    }

    fn print_rec(mut prefix: String, node: Option<Node<T>>, is_left: bool) {
        if node.is_none() {
            return;
        }

        print!("{}", prefix);

        if is_left {
            print!("└─");
        } else {
            print!("├─");
        }

        // print the value of the node
        let node = node.unwrap();
        let node_b = node.borrow();

        let black = node_b.color == Color::Black;
        println!("{:?}{}", node_b.val, if black { 'b' } else { 'r' });

        // enter the next tree level - left and right branch
        prefix += if is_left { "  " } else { "│ " };

        Self::print_rec(prefix.clone(), node_b.children[Pos::RIGHT].clone(), false);
        Self::print_rec(prefix, node_b.children[Pos::LEFT].clone(), true);
    }

    fn is_valid(&self) -> bool {
        let root = self.root.clone();
        if let Some(r) = root {
            if r.borrow().color == Color::Red {
                println!("The root should be BLACK");
                return false;
            }
            let l_black = Self::black_height(r.borrow().children[Pos::LEFT].clone(), 1);
            let r_black = Self::black_height(r.borrow().children[Pos::RIGHT].clone(), 1);

            if l_black.is_err() {
                println!("{}", l_black.unwrap_err());
                return false;
            }
            if r_black.is_err() {
                println!("{}", r_black.unwrap_err());
                return false;
            }
        }
        true
    }

    // returns black height of subtree if it's valid
    fn black_height(node: Option<Node<T>>, mut level: u32) -> Result<u32, String> {
        if let Some(n) = node {
            let left = n.borrow().children[Pos::LEFT].clone();
            let right = n.borrow().children[Pos::RIGHT].clone();

            if left.is_some() && right.is_some() {
                let left = left.unwrap();
                let right = right.unwrap();

                let add;
                if n.borrow().color == Color::Red {
                    if left.borrow_mut().color == Color::Red
                        || right.borrow_mut().color == Color::Red
                    {
                        return Err(format!(
                            "Two consecutive RED nodes, see val: {} on level: {}",
                            n.borrow().val.to_string(),
                            level.to_string()
                        ));
                    }
                    add = 0;
                } else {
                    add = 1;
                }

                level += 1;
                let l_black = Self::black_height(Some(left), level)?;
                let r_black = Self::black_height(Some(right), level)?;

                if l_black == r_black {
                    return Ok(add + l_black);
                } else {
                    return Err(format!(
                        "Different black heights, see val: {} on level {}, left: {} right: {}",
                        n.borrow().val.to_string(),
                        level.to_string(),
                        l_black.to_string(),
                        r_black.to_string()
                    ));
                }
            } else {
                let mut add = 0;

                if let Some(l) = left {
                    add += (l.borrow().color == Color::Black) as u32
                }
                if let Some(r) = right {
                    add += (r.borrow().color == Color::Black) as u32
                }
                add += (n.borrow().color == Color::Black) as u32;

                return Ok(add);
            }
        }
        // nil node
        Ok(0)
    }
}
