/// Represents the memory for data in the Brainfuck state machine.
///
/// The memory is created with all data filled with 0, and with a specific
/// capacity. By default is 30,000 elements of capacity, but it can be created
/// with any other size. Each element in memory is represented with a `i64`.
///
/// The memory once used cannot be reused.
#[derive(Debug)]
pub struct DataMemory {
    data: Vec<i64>,
    pointer: usize,
}

impl DataMemory {
    /// Creates the memory with the specific size.
    pub fn new_with_size(size: usize) -> Self {
        Self {
            data: vec![0_i64; size],
            pointer: 0,
        }
    }

    /// Creates the memory with 30,000 elements of capacity.
    pub fn new() -> Self {
        Self::new_with_size(30_000)
    }

    pub(crate) fn next(&mut self) {
        #[cfg(feature = "with-debug")]
        if self.pointer == self.data.len() - 1 {
            panic!(
                "Attempt to > when pointer is already at position {}",
                self.data.len() - 1
            );
        }

        // note: I know this could end up crashing, but language is what it is
        self.pointer += 1;
    }

    pub(crate) fn previous(&mut self) {
        #[cfg(feature = "with-debug")]
        if self.pointer == 0 {
            panic!("Attempt to < when pointer is already at position 0");
        }

        // note: I know this could end up being 0 - 1, and funny things will happen
        self.pointer -= 1;
    }

    pub(crate) fn increment(&mut self) {
        self.data[self.pointer] += 1;
    }

    pub(crate) fn decrement(&mut self) {
        self.data[self.pointer] -= 1;
    }

    pub(crate) fn write(&mut self, value: i64) {
        self.data[self.pointer] = value;
    }

    pub(crate) fn read(&self) -> i64 {
        self.data[self.pointer]
    }

    #[cfg(feature = "with-debug")]
    pub(crate) fn position(&self) -> usize {
        self.pointer
    }
}

impl Default for DataMemory {
    fn default() -> Self {
        Self::new()
    }
}
