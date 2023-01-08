use std::{cell::RefCell, mem, rc::Rc};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Black,
}

pub type Node<T> = Rc<RefCell<RbTreeNode<T>>>;

#[derive(Debug)]
pub struct RbTreeNode<T> {
    pub val: T,
    pub color: Color,
    pub children: [Option<Node<T>>; 2],
}

impl<T> RbTreeNode<T> {
    // as alternative the tree can swap values instead of references and color
    fn swap(&mut self, other: *mut RbTreeNode<T>) {
        unsafe {
            mem::swap(&mut self.val, &mut (*other).val);
        }
    }
}
