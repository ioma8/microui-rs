#[derive(Clone, Debug)]
pub(crate) struct FixedStack<T, const N: usize> {
    pub idx: usize,
    pub items: [T; N],
}

impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
    pub(crate) fn new() -> Self {
        Self {
            idx: 0,
            items: [T::default(); N],
        }
    }

    pub(crate) fn push(&mut self, value: T) {
        assert!(self.idx < N, "stack overflow");
        self.items[self.idx] = value;
        self.idx += 1;
    }

    pub(crate) fn pop(&mut self) -> T {
        assert!(self.idx > 0, "stack underflow");
        self.idx -= 1;
        self.items[self.idx]
    }

    pub(crate) fn last(&self) -> T {
        assert!(self.idx > 0, "stack empty");
        self.items[self.idx - 1]
    }
}
