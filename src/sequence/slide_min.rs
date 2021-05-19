/// Sliding Window Minimum, O(n)
/// Args:
///     Vec<T: Ord>
///     window_size
/// Returns:
///     Vec<usize>, indices of minimum for each windows
pub fn slide_min<T: Ord>(xs: &[T], window_size: usize) -> Vec<usize> {
    use std::collections::VecDeque;
    if xs.len() < window_size {
        return vec![];
    }
    fn push<T: Ord>(deq: &mut VecDeque<usize>, idx: usize, xs: &[T]) {
        while let Some(&i) = deq.back() {
            if xs[i] >= xs[idx] {
                let _ = deq.pop_back();
            } else {
                break;
            }
        }
        deq.push_back(idx);
    }
    let mut deq = VecDeque::new();
    for i in 0..window_size - 1 {
        push(&mut deq, i, &xs);
    }
    let mut ret = vec![];
    for i in window_size - 1..xs.len() {
        eprintln!("{}, {:?}", i, &deq);
        push(&mut deq, i, &xs);
        let j = *deq.front().unwrap();
        ret.push(j);
        if j + window_size == i + 1 {
            let _ = deq.pop_front();
        }
        eprintln!("=> {:?}; {:?}", &deq, &ret);
    }
    ret
}

#[cfg(test)]
mod test_total {
    use crate::sequence::slide_min::*;
    #[test]
    fn it_works() {
        assert_eq!(slide_min(&vec![0, 1, 2, 3, 4, 5], 3), vec![0, 1, 2, 3]);
        assert_eq!(slide_min(&vec![1, 3, 4, 2, 3, 4], 3), vec![0, 3, 3, 3]);
        assert_eq!(slide_min(&vec![5, 4, 3, 2, 1], 2), vec![1, 2, 3, 4]);
        assert_eq!(slide_min(&vec![1, 2, 1, 2, 1], 2), vec![0, 2, 2, 4]);
    }
}
