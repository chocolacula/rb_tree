use super::node::Node;

pub struct Pos {}

impl Pos {
    pub const LEFT: usize = 0;
    pub const RIGHT: usize = 1;
}

pub struct Ancestor<T> {
    pub node: Node<T>,
    pub position: usize,
}

pub type Ancestry<T> = Vec<Ancestor<T>>;
