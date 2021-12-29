use crate::algebra::group_additive::*;
use crate::algebra::hyper::*;
use crate::graph::directed::dinic::*;

#[derive(Debug, Clone)]
pub struct MoyasuUmeruSolver {
    size: usize,
    cost1: Vec<Vec<Hyper<i64>>>,
    cost2: Vec<(usize, usize, Hyper<i64>)>,
}
impl MoyasuUmeruSolver {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cost1: vec![vec![Hyper::zero(); 2]; size],
            cost2: vec![],
        }
    }
    /// +cost when var `i` is `b`.
    fn add_constraint_node(&mut self, i: usize, b: bool, cost: Hyper<i64>) {
        let j = if b { 0 } else { 1 };
        self.cost1[i][j] += cost;
    }
    /// +cost when var `i` is true AND var `j` is false.
    fn add_constraint_edge(&mut self, i: usize, j: usize, cost: Hyper<i64>) {
        self.cost2.push((i, j, cost));
    }
    pub fn min_cost(&self) -> Hyper<i64> {
        let mut base: Hyper<i64> = Hyper::zero();
        let s = self.size;
        let t = self.size + 1;
        let mut g = vec![vec![]; self.size + 2];
        for i in 0..self.size {
            match (self.cost1[i][0], self.cost1[i][1]) {
                (b1, b2) if b1 > b2 => {
                    base = base + b2;
                    g[s].push((i, b1 - b2));
                }
                (b1, b2) if b1 < b2 => {
                    base = base + b1;
                    g[i].push((t, b2 - b1));
                }
                (b, _) => base = base + b,
            }
        }
        for &(i, j, cost) in self.cost2.iter() {
            g[j].push((i, cost));
        }
        base + Dinic::new(s, t, &g).maxflow()
    }
    pub fn max_gain(&self) -> Hyper<i64> {
        -self.min_cost()
    }
}
pub enum MoyasuUmeruCost {
    Node(usize, bool, Hyper<i64>),
    Edge(usize, usize, Hyper<i64>),
}
impl std::ops::AddAssign<MoyasuUmeruCost> for MoyasuUmeruSolver {
    fn add_assign(&mut self, constraint: MoyasuUmeruCost) {
        match constraint {
            MoyasuUmeruCost::Node(i, b, cost) => {
                self.add_constraint_node(i, b, cost);
            }
            MoyasuUmeruCost::Edge(i, j, cost) => {
                self.add_constraint_edge(i, j, cost);
            }
        }
    }
}
#[macro_export]
macro_rules! cost {
    (inf ; $( $rest:tt )*) => {
        cost!(@ Hyper::Inf ; $($rest)*)
    };
    ($cost:expr ; $( $rest:tt )*) => {
        cost!(@ Hyper::Real($cost) ; $($rest)*)
    };
    (@ $cost:expr ; if not $i:tt) => {
        MoyasuUmeruCost::Node($i, false, $cost)
    };
    (@ $cost:expr ; if not $i:tt and $j:tt) => {
        MoyasuUmeruCost::Edge($j, $i, $cost)
    };
    (@ $cost:expr ; if $i:tt and not $j:tt) => {
        MoyasuUmeruCost::Edge($i, $j, $cost)
    };
    (@ $cost:expr ; if $i:tt) => {
        MoyasuUmeruCost::Node($i, true, $cost)
    };
}
#[macro_export]
macro_rules! gain {
    (inf ; $( $rest:tt )*) => {
        cost!(@ Hyper::NegInf ; $($rest)*)
    };
    ($cost:expr ; $( $rest:tt )*) => {
        cost!(-$cost ; $($rest)*)
    }
}

#[cfg(test)]
mod test_umeru_moyasu {
    use crate::algebra::hyper::Hyper;
    use crate::opt::umeru_moyasu::{MoyasuUmeruCost, MoyasuUmeruSolver};

    #[test]
    fn test_simple() {
        let mut solver = MoyasuUmeruSolver::new(2);
        solver += cost!(5; if 0);
        solver += cost!(3; if not 0);
        assert_eq!(solver.min_cost(), Hyper::Real(3));

        solver += gain!(100; if 1);
        assert_eq!(solver.max_gain(), Hyper::Real(97));

        solver += cost!(inf; if 1 and not 0);
        assert_eq!(solver.max_gain(), Hyper::Real(95));
    }

    /// https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem#Project_selection_problem
    #[test]
    fn test_wikipedia() {
        let mut solver = MoyasuUmeruSolver::new(6);
        // reward from projects
        solver += gain!(100; if 0);
        solver += gain!(200; if 1);
        solver += gain!(150; if 2);
        // cost for machines
        solver += cost!(200; if 3);
        solver += cost!(100; if 4);
        solver += cost!(50; if 5);
        // required machine for each projects
        solver += cost!(inf; if 0 and not 3);
        solver += cost!(inf; if 0 and not 4);
        solver += cost!(inf; if 1 and not 4);
        solver += cost!(inf; if 2 and not 5);

        assert_eq!(solver.max_gain(), Hyper::Real(200));
    }
}
