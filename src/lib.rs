use sysinfo::System;

/// A type representing an unsigned integer with the maximum size that fits in the free memory of the machine.

pub struct AllOfIt {
    chunks: Vec<Vec<u8>>,
    total_capacity: usize,
}

impl AllOfIt {

    /// Attempts to allocate memory in chunks, inching closer to all available free memory without crashing.
    pub fn new() -> Option<Self> {
        let mut sys = System::new();
        sys.refresh_memory();
        let free_memory = sys.available_memory(); // in kilobytes
    let mut free_bytes = (free_memory * 1024) as usize;
    let mut chunk_size = 100 * 1024 * 1024; // start with 100MB
    let max_chunk_size = free_bytes / 4; // never try to allocate more than 1/4 of free memory at once
        let mut chunks = Vec::new();
        let mut total_capacity = 0;

        while free_bytes > 0 && chunk_size > 0 {
            if chunk_size > max_chunk_size {
                chunk_size = max_chunk_size;
            }
            let result = std::panic::catch_unwind(|| Vec::with_capacity(chunk_size));
            match result {
                Ok(mut allocation) => {
                    let resize_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| allocation.resize(chunk_size, 0)));
                    match resize_result {
                        Ok(_) => {
                            total_capacity += allocation.capacity();
                            chunks.push(allocation);
                            free_bytes = free_bytes.saturating_sub(chunk_size);
                            // Try a bigger chunk next time
                            chunk_size = ((chunk_size as f64) * 1.2) as usize;
                        }
                        Err(_) => {
                            // Allocation failed, try a smaller chunk
                            chunk_size /= 2;
                        }
                    }
                }
                Err(_) => {
                    // Allocation failed, try a smaller chunk
                    chunk_size /= 2;
                }
            }
        }
        if total_capacity > 0 {
            Some(Self { chunks, total_capacity })
        } else {
            None
        }
    }

    /// Returns the total capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.total_capacity
    }
}