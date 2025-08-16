use std::mem;
use sysinfo::System;

/// A type representing an unsigned integer with the maximum size that fits in the free memory of the machine.
pub struct AllOfIt {
    data: Vec<u8>,
}

impl AllOfIt {
    /// Attempts to allocate a vector that fills all available free memory.
    pub fn new() -> Option<Self> {
        let mut sys = System::new();
        sys.refresh_memory();
        let free_memory = sys.available_memory(); // in kilobytes

        // Convert to bytes
        let free_bytes = free_memory * 1024;

        // Try to allocate
        let allocation = Vec::with_capacity(free_bytes as usize);
        Some(Self { data: allocation })
    }

    /// Returns the capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}