extern crate libc;

use crate::linked_list::LinkedList;
use crate::node::Node;

pub struct MemoryPool<T: 'static> {
    pub block: *mut u8,
    pub chunk_size: usize,
    pub free_list: LinkedList<T>
}
impl<T> Drop for MemoryPool<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.block as *mut libc::c_void); }
    }
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
        match self.free_list.pop() {
            Some(node) => Ok(node),
            None => Err("Pool is exhausted! Cannot allocate anymore memory!")
        }
    }
    
    pub fn dealloc(&mut self, ptr: *mut Node<T>) -> Result<(), &str> {
        if ptr.is_null() {
            return Err("Memory deallocation failed! Pointer is null!");
        }
        
        let start = self.block as usize;
        let end = unsafe { self.block.add(self.chunk_size 
            * self.free_list.len()) as usize };
        let ptr_addr = ptr as usize;

        if ptr_addr < start || ptr_addr >= end {
            return Err("Pointer is out of bounds!");
        } 
        if self.free_list.get(ptr).is_some() {
            return Err("Double-free detected!");
        }
        
        self.free_list.insert(ptr);
        Ok(())
    }
}