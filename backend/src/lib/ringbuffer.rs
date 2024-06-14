struct RingBuffer<T, const S: usize> {
    buffer: [Option<T>; S],
    start: usize,
    end: usize,
}

impl<T, const S: usize> RingBuffer<T, S> {
    fn new() -> Self {
        // Initialize the buffer with None values.
        // Using .map() here instead of [None; S] lifts the restriction that T must implement copy
        Self {
            buffer: [(); S].map(|_| None),
            start: 0,
            end: 0,
        }
    }

    fn push(&mut self, item: T) {
        self.buffer[self.end] = Some(item);
        self.end = (self.end + 1) % S;

        if self.end == self.start {
            self.start = (self.start + 1) % S
        }
    }
}
