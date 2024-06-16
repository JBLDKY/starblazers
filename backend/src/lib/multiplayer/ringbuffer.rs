#[derive(Debug)]
pub struct RingBuffer<T, const S: usize> {
    buffer: [Option<T>; S], // Use an array of Option<T> to handle the buffer's storage and potential empty slots
    start: usize,           // Track the start index for reading
    end: usize,             // Track the end index for writing
    full: bool,             // Boolean flag to indicate if the buffer is full
}

impl<T, const S: usize> Default for RingBuffer<T, S> {
    fn default() -> Self {
        Self::new() // Implement Default to allow easy instantiation with default values
    }
}

impl<T, const S: usize> RingBuffer<T, S> {
    pub fn new() -> Self {
        // Initialize the buffer with None values.
        // Using .map() here instead of [None; S] lifts the restriction that T must implement Copy
        Self {
            buffer: [(); S].map(|_| None),
            start: 0,
            end: 0,
            full: false,
        }
    }

    pub fn push(&mut self, item: T) {
        self.buffer[self.end] = Some(item); // Insert the item at the end index
        self.end = (self.end + 1) % S; // Move the end index forward, wrapping around if necessary

        if self.full {
            self.start = (self.start + 1) % S; // If the buffer is full, move the start index forward
        }

        if self.end == self.start {
            self.full = true; // If the end index catches up to the start, the buffer is now full
        }
    }

    pub fn is_full(&self) -> bool {
        self.full // Simple getter to check if the buffer is full
    }

    pub fn get_buffer(&self) -> &[Option<T>; S] {
        &self.buffer // Provide access to the underlying buffer for inspection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_new() {
        let ring_buffer: RingBuffer<usize, 5> = RingBuffer::new();
        assert_eq!(ring_buffer.start, 0);
        assert_eq!(ring_buffer.end, 0);
        for elem in ring_buffer.buffer.iter() {
            assert!(elem.is_none());
        }
    }

    #[test]
    fn test_ring_buffer_push() {
        let mut ring_buffer: RingBuffer<usize, 3> = RingBuffer::new();
        ring_buffer.push(1);
        assert_eq!(ring_buffer.buffer[0], Some(1));
        assert_eq!(ring_buffer.end, 1);
        assert_eq!(ring_buffer.start, 0);

        ring_buffer.push(2);
        assert_eq!(ring_buffer.buffer[1], Some(2));
        assert_eq!(ring_buffer.end, 2);
        assert_eq!(ring_buffer.start, 0);

        ring_buffer.push(3);
        assert_eq!(ring_buffer.buffer[2], Some(3));
        assert_eq!(ring_buffer.end, 0);
        assert_eq!(ring_buffer.start, 0);

        // Pushing another element should overwrite the oldest element
        ring_buffer.push(4);
        assert_eq!(ring_buffer.buffer[0], Some(4));
        assert_eq!(ring_buffer.end, 1);
        assert_eq!(ring_buffer.start, 1);

        ring_buffer.push(5);
        assert_eq!(ring_buffer.buffer[1], Some(5));
        assert_eq!(ring_buffer.end, 2);
        assert_eq!(ring_buffer.start, 2);

        ring_buffer.push(6);
        assert_eq!(ring_buffer.buffer[2], Some(6));
        assert_eq!(ring_buffer.end, 0);
        assert_eq!(ring_buffer.start, 0);
    }

    #[test]
    fn test_ring_buffer_get_buffer() {
        let mut ring_buffer: RingBuffer<usize, 3> = RingBuffer::new();
        ring_buffer.push(1);
        ring_buffer.push(2);
        ring_buffer.push(3);

        let buffer = ring_buffer.get_buffer();
        assert_eq!(buffer, &[Some(1), Some(2), Some(3)]);

        ring_buffer.push(4);
        let buffer = ring_buffer.get_buffer();
        assert_eq!(buffer, &[Some(4), Some(2), Some(3)]);

        ring_buffer.push(5);
        let buffer = ring_buffer.get_buffer();
        assert_eq!(buffer, &[Some(4), Some(5), Some(3)]);

        ring_buffer.push(6);
        let buffer = ring_buffer.get_buffer();
        assert_eq!(buffer, &[Some(4), Some(5), Some(6)]);
    }
}
