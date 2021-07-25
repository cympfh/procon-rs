#[derive(Debug, Default)]
pub struct DFS<X> {
    pub stack: Vec<DfsOrd<X>>,
}
impl<X: Clone> DFS<X> {
    pub fn push(&mut self, x: &X) {
        self.stack.push(DfsOrd::Post(x.clone()));
        self.stack.push(DfsOrd::Pre(x.clone()));
    }
    pub fn pop(&mut self) -> Option<DfsOrd<X>> {
        self.stack.pop()
    }
}
#[derive(Debug)]
pub enum DfsOrd<X> {
    Pre(X),
    Post(X),
}

#[cfg(test)]
mod test_dfs {
    use crate::algorithm::dfs::*;

    #[test]
    fn test_graph() {
        let mut graph = vec![vec![]; 7];
        for i in 0..3 {
            graph[i].push(i * 2 + 1);
            graph[i].push(i * 2 + 2);
        }

        let mut pre_order = vec![];
        let mut post_order = vec![];

        let mut dfs = DFS::default();
        dfs.push(&0);
        while let Some(ord) = dfs.pop() {
            match ord {
                DfsOrd::Pre(x) => {
                    pre_order.push(x);
                    for &y in graph[x].iter() {
                        dfs.push(&y);
                    }
                }
                DfsOrd::Post(x) => {
                    post_order.push(x);
                }
            }
        }

        assert_eq!(pre_order, vec![0, 2, 6, 5, 1, 4, 3]);
        assert_eq!(post_order, vec![6, 5, 2, 4, 3, 1, 0]);
    }
}
