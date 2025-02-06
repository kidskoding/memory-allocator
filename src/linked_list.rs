use crate::node::Node;

pub struct LinkedList<T> {
    pub head: Option<*mut Node<T>>,
    pub len: usize,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }
    pub fn insert(&mut self, node: *mut Node<T>) {
        unsafe {
            (*node).next = self.head;
        }
        self.head = Some(node);
        self.len += 1;
    }
    pub fn get(&mut self, node: *mut Node<T>) -> Option<*mut Node<T>> {
        let mut current = self.head;
        while let Some(ptr) = current {
            if ptr == node {
                return Some(ptr);
            }
            unsafe {
                current = (*ptr).next;
            }
        }
        None
    }
    pub fn pop(&mut self) -> Option<*mut Node<T>> {
        if let Some(node) = self.head.take() {
            unsafe { self.head = (*node).next; }
            return Some(node);
        }
        self.len -= 1;
        None
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}