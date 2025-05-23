use crate::memory_pool::MemoryPool;
use crate::node::Node;

pub mod memory_pool;
pub mod node;
pub mod linked_list;

fn main() {
    let mut pool = MemoryPool::new(1024, 32).unwrap();
    let mut ptr1: *mut Node<u8> = pool.alloc().expect("Allocation failed");

    println!("Allocated at {:?}", ptr1);
    
    pool.dealloc(ptr1).unwrap();
    
    /*
    * THIS CODE WILL TRIGGER A SEGFAULT!
    *
    * unsafe {
    *   (*ptr1).value = 42;
    *   println!("{:?}", (*ptr1).value); 
    * }
    *
    */
  
    match pool.dealloc(ptr1) {
        Ok(()) => {}
        Err(e) => println!("{e} this pointer has already been freed from memory!") 
    }
    
    ptr1 = pool.alloc().unwrap();
    unsafe {
        (*ptr1).value = 42;
        println!("{:?}", (*ptr1).value);
    }
    
    drop(pool);
}