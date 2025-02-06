use crate::node::Node;

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub len: usize,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }
    pub fn insert(&mut self, value: T) {
        let mut new_node = Box::new(
            Node::new(
                value,
            )
        );
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.len += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.len -= 1;
            Some(node.value)
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}