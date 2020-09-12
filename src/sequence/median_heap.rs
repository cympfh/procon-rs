/// Sequence - Median Heap
#[derive(Debug)]
struct MedianHeap<T>
where
    T: Ord + Copy,
{
    head: std::collections::BinaryHeap<T>,
    tail: std::collections::BinaryHeap<std::cmp::Reverse<T>>,
}
#[derive(Debug, PartialEq, Eq)]
enum Median<T> {
    None,
    Just(T),
    Between(T, T),
}
impl<T: Ord + Copy> MedianHeap<T> {
    fn new() -> MedianHeap<T> {
        MedianHeap {
            head: std::collections::BinaryHeap::new(),
            tail: std::collections::BinaryHeap::new(),
        }
    }
    fn len(&self) -> usize {
        (self.head.len() + self.tail.len()) / 2
    }
    fn push(&mut self, x: T) {
        match (self.head.peek(), self.tail.peek()) {
            (None, None) => {
                self.head.push(x);
                self.tail.push(std::cmp::Reverse(x));
            }
            (Some(&a), Some(&std::cmp::Reverse(b))) => {
                if a <= x && x <= b {
                    self.head.push(x);
                    self.tail.push(std::cmp::Reverse(x));
                } else if x < a {
                    self.head.push(x);
                    self.head.push(x);
                    let _ = self.head.pop();
                    self.tail.push(std::cmp::Reverse(a));
                } else {
                    self.tail.push(std::cmp::Reverse(x));
                    self.tail.push(std::cmp::Reverse(x));
                    let _ = self.tail.pop();
                    self.head.push(b);
                }
            }
            _ => {}
        }
    }
    fn peek(&self) -> Median<T> {
        let n = self.len();
        if n == 0 {
            Median::None
        } else if n % 2 == 1 {
            Median::Just(*self.head.peek().unwrap())
        } else {
            let a = self.head.peek().unwrap();
            let b = self.tail.peek().unwrap().0;
            Median::Between(*a, b)
        }
    }
    fn pop(&mut self) -> Median<T> {
        let n = self.len();
        if n == 0 {
            Median::None
        } else if n % 2 == 1 {
            let a = self.head.pop().unwrap();
            let _ = self.tail.pop();
            Median::Just(a)
        } else {
            let a = self.head.pop().unwrap();
            let b = self.tail.pop().unwrap().0;
            Median::Between(a, b)
        }
    }
}

#[cfg(test)]
mod test_median_heap {
    use crate::sequence::median_heap::*;

    #[test]
    fn test() {
        let mut mh = MedianHeap::new();
        mh.push(0);
        assert_eq!(mh.peek(), Median::Just(0));
        mh.push(2);
        assert_eq!(mh.peek(), Median::Between(0, 2));
        mh.push(1);
        assert_eq!(mh.pop(), Median::Just(1));
        assert_eq!(mh.pop(), Median::Between(0, 2));
    }
}
