#[cfg(test)]
mod memory_pool_test {
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
        assert_ne!(memory_pool.block.as_ptr(), std::ptr::null_mut());
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
}