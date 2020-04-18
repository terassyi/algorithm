fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
pub struct PriorityQueue<T> {
    queue: Vec<T>,
}

impl<T: PartialOrd+Clone+Copy> PriorityQueue<T> {
    fn new(data: Vec<T>) -> PriorityQueue<T> {
        let mut p = PriorityQueue {
            queue: data,
        };
        p.build_heap();
        p
    }

    fn left_node(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn right_node(&self, index: usize) -> usize {
        2 * (index + 1)
    }

    fn parent_node(&self, depth: usize) -> usize {
        (depth - 1)/2
    }

    fn push(&mut self, val: T) {
        let l = self.queue.len();
        self.queue.push(val);
        self.build_heap();
    }

    fn pop(&mut self) -> Option<T> {
        if self.queue.len() == 0 {
            return None;
        }
        let end = self.queue.len() - 1;
        self.queue.swap(0, end);
        let val = self.queue.pop();
        self.build_down_heap();
        val
    }

    fn build_down_heap(&mut self) {
        let length = self.queue.len();
        let mut parent: usize = 0;
        let mut left = self.left_node(parent);
        let mut right = self.right_node(parent);

        while length > left || length > right {
            left = self.left_node(parent);
            right = self.right_node(parent);
            if length <= left || length <= right {
                break;
            }
            if self.queue[left] > self.queue[right] {
                self.queue.swap(parent, left);
                parent = left;
                println!("select left = {}", parent);
            } else {
                self.queue.swap(parent, right);
                parent = right;
                println!("select right = {}", parent);
            }
        }
    }

    fn swap(&mut self, index: usize) {
        let mut i = index;
        while i != 0 {
            let parent = self.parent_node(i);
            if self.queue[parent] < self.queue[i] {
                self.queue.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn build_heap(&mut self) {
        for i in 0..self.queue.len() {
            self.swap(i);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_swap() {
        let mut p = super::PriorityQueue {
            queue: vec![1,3,2,8],
        };

        p.swap(3);
        assert_eq!(vec![8,1,2,3], p.queue);
    }

    #[test]
    fn test_build_heap() {
        let mut p = super::PriorityQueue {
            queue: vec![1,3,2,8],
        };

        p.build_heap();
        assert_eq!(vec![8,3,2,1], p.queue);
    }

    #[test]
    fn test_new() {
        let data = vec![1,3,2,8];
        let p = super::PriorityQueue::new(data);

        assert_eq!(vec![8,3,2,1], p.queue);
    }

    #[test]
    fn test_push() {
        let data = vec![20,15,12,9,10,4,11,3,6,7,5,2];
        let mut p = super::PriorityQueue::new(data);
        p.push(13);

        assert_eq!(vec![20,15,13,9,10,12,11,3,6,7,5,2,4], p.queue);
    }

    #[test]
    fn test_pop() {
        let data = vec![20,15,12,9,10,4,11,3,6,7,5,2];
        let mut p = super::PriorityQueue::new(data); 
        let i = p.pop().unwrap();
        assert_eq!(20, i);
        assert_eq!(p.queue, vec![15,10,12,9,7,4,11,3,6,2,5]);
    }
}
