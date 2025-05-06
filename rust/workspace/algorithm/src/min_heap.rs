pub struct MinHeap<T> {
    elements: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { elements: vec![] }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
        self.sift_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop().map(|mut item| {
            if !self.empty() {
                std::mem::swap(&mut item, &mut self.elements[0]);
                self.sift_down(0);
            }
            item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    fn sift_up(&mut self, index: usize) {
        let mut current = index;
        while current > 0 {
            let parent = (current - 1) / 2;
            if self.elements[parent] <= self.elements[current] {
                break;
            }
            self.elements.swap(parent, current);
            current = parent;
        }
    }

    fn sift_down(&mut self, index: usize) {
        let mut current = index;

        loop {
            let mut smallest = current;

            let left = 2 * current + 1;
            if left < self.len() && self.elements[left] < self.elements[smallest] {
                smallest = left;
            }

            let right = 2 * current + 2;
            if right < self.len() && self.elements[right] < self.elements[smallest] {
                smallest = right;
            }

            if smallest == current {
                break;
            }

            self.elements.swap(smallest, current);
            current = smallest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut heap = MinHeap::new();
        heap.push(3);
        heap.push(2);
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }
}
