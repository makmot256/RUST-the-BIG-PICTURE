// A max-heap implementation in Rust
pub struct Heap<T> {
    elements: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { elements: Vec::new() }
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
        self.sift_up(self.elements.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }

        self.elements.swap(0, self.elements.len() - 1);
        let element = self.elements.pop().unwrap();
        self.sift_down(0);
        Some(element)
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    fn sift_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent_index = Self::parent(index);
        if self.elements[parent_index] >= self.elements[index] {
            return;
        }

        self.elements.swap(index, parent_index);
        self.sift_up(parent_index);
    }

    fn sift_down(&mut self, index: usize) {
        let left_child_index = Self::left_child(index);
        let right_child_index = Self::right_child(index);

        let mut max_index = index;
        if left_child_index < self.elements.len()
            && self.elements[left_child_index] > self.elements[max_index]
        {
            max_index = left_child_index;
        }

        if right_child_index < self.elements.len()
            && self.elements[right_child_index] > self.elements[max_index]
        {
            max_index = right_child_index;
        }

        if max_index != index {
            self.elements.swap(index, max_index);
            self.sift_down(max_index);
        }
    }
}
