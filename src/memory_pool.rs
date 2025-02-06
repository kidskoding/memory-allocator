extern crate libc;
use crate::linked_list::LinkedList;
use crate::node::Node;

pub struct MemoryPool<T: 'static> {
    pub block: *mut u8,
    pub chunk_size: usize,
    pub free_list: LinkedList<T>
}
impl<T> MemoryPool<T> {
    pub fn new(pool_size: usize, chunk_size: usize) -> Result<Self, &'static str> {
        if chunk_size < size_of::<Node<T>>() {
            return Err("Chunk size too small");
        }
        
        let block = unsafe {
            libc::malloc(pool_size) as *mut u8
        };
        if block.is_null() {
            return Err("Memory allocation failed! Pointer is null!");
        }
        
        let mut free_list = LinkedList::new();
        let mut current = block as *mut Node<T>;
        for _ in 0..(pool_size / chunk_size) {
            unsafe {
                free_list.insert(current);
                current = (current as *mut u8).add(chunk_size) as *mut Node<T>;
            }
        }
        
        Ok(Self {
            block,
            chunk_size,
            free_list,
        })
    }
    
    pub fn alloc(&mut self) -> Result<*mut Node<T>, &str> {
        match self.free_list.pop_front() {
            Some(node) => Ok(node),
            None => Err("Pool is exhausted! Cannot allocate anymore memory!")
        }
    }
    
    pub fn dealloc(&mut self, ptr: *mut Node<T>) {
        self.free_list.insert(ptr);
    }
}