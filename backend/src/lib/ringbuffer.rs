struct RingBuffer<T, const S: usize> {
    buffer: [Option<T>; S],
    start: usize,
    end: usize,
    full: bool,
}

impl<T, const S: usize> RingBuffer<T, S> {
    fn new() -> Self {
        // Initialize the buffer with None values.
        // Using .map() here instead of [None; S] lifts the restriction that T must implement copy
        Self {
            buffer: [(); S].map(|_| None),
            start: 0,
            end: 0,
            full: false,
        }
    }

    fn push(&mut self, item: T) {
        self.buffer[self.end] = Some(item);
        self.end = (self.end + 1) % S;

        if self.full {
            self.start = (self.start + 1) % S;
        }

        if self.end == self.start {
            self.full = true;
        }
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
}
