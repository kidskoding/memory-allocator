pub struct Node<T> {
    pub value: T,
    pub next: Option<*mut Node<T>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}