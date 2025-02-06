#[cfg(test)]
mod memory_pool_test {
    extern crate libc;
    use crate::memory_pool::MemoryPool;
    use crate::node::Node;

    #[test]
    fn test_memory_pool_creation() {
        let pool_size = 1024;
        let chunk_size = size_of::<Node<u8>>();
        let memory_pool = MemoryPool::<u8>::new(pool_size, chunk_size);

        assert!(memory_pool.is_ok());
        let memory_pool = memory_pool.unwrap();
        assert_eq!(memory_pool.chunk_size, chunk_size);
        assert_ne!(memory_pool.block, std::ptr::null_mut());
        assert!(memory_pool.free_list.len() > 0);
    }
    
    #[test]
    fn test_max_pool_size() {
        let pool_size = usize::MAX;
        let chunk_size = size_of::<Node<u8>>();
        let memory_pool = MemoryPool::<u8>::new(pool_size, chunk_size);

        assert!(memory_pool.is_err());
    }
    
    #[test]
    fn test_alloc() {
        let pool_size = 1024;
        let chunk_size = size_of::<Node<u8>>();
        let mut memory_pool = MemoryPool::<u8>::new(pool_size, chunk_size).unwrap();
        
        let allocation = memory_pool.alloc();
        assert!(allocation.is_ok());
        
        for _ in 1..(pool_size / chunk_size) {
            let allocation = memory_pool.alloc();
            assert!(allocation.is_ok());
        }
        
        let allocation = memory_pool.alloc();
        assert!(allocation.is_err());
    }
    
    #[test]
    fn test_dealloc() {
        let pool_size = 1024;
        let chunk_size = size_of::<Node<u8>>();
        let mut memory_pool = MemoryPool::<u8>::new(pool_size, chunk_size).unwrap();

        let allocation = memory_pool.alloc();
        assert!(allocation.is_ok());
        let node = allocation.unwrap();
        
        memory_pool.dealloc(node).unwrap();

        let free_node = memory_pool.free_list.get(node as *const _ as *mut _);
        assert!(free_node.is_some());
        assert_eq!(free_node.unwrap(), node as *const _ as *mut _);
        
        let free_node = memory_pool.free_list.get(node);
        let free_node = free_node.unwrap();

        let double_free = memory_pool.dealloc(free_node);
        assert!(double_free.is_err());
        assert_eq!(double_free.unwrap_err(), "Double-free detected!"); 
    }

    #[test]
    fn test_realloc() {
        let pool_size = 1024;
        let chunk_size = 32;
        let new_pool_size = 2048;

        let mut pool = MemoryPool::new(pool_size, chunk_size).expect("Failed to create memory pool");

        let ptr1: *mut Node<u8> = pool.alloc().expect("Failed to allocate memory");
        let ptr2: *mut Node<u8> = pool.alloc().expect("Failed to allocate memory");

        pool.realloc(new_pool_size).expect("Failed to reallocate memory pool");

        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());

        let ptr3: *mut Node<u8> = pool.alloc().expect("Failed to allocate memory after reallocation");
        assert!(!ptr3.is_null());

        pool.dealloc(ptr1).expect("Failed to deallocate memory");
        pool.dealloc(ptr2).expect("Failed to deallocate memory");
        pool.dealloc(ptr3).expect("Failed to deallocate memory");
    }
}