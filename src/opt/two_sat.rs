use crate::graph::directed::scc::*;

pub struct TwoSAT {
    n: usize,
    graph: Vec<Vec<usize>>,
}
impl TwoSAT {
    pub fn new(n: usize) -> Self {
        let graph = vec![vec![]; n * 2];
        Self { n, graph }
    }
    /// 充足判定
    pub fn solve(&self) -> bool {
        let (cmp, _) = scc(&self.graph);
        for i in 0..self.n {
            if cmp[i * 2] == cmp[i * 2 + 1] {
                return false;
            }
        }
        true
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TwoSATTerm(usize, bool);
impl TwoSATTerm {
    fn val(&self) -> usize {
        self.0 * 2 + if self.1 { 0 } else { 1 }
    }
    fn negate(&self) -> Self {
        Self(self.0, true ^ self.1)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TwoSATLogic {
    Or(TwoSATTerm, TwoSATTerm),
    Implies(TwoSATTerm, TwoSATTerm),
    Iff(TwoSATTerm, TwoSATTerm),
}
impl std::ops::AddAssign<TwoSATLogic> for TwoSAT {
    fn add_assign(&mut self, and: TwoSATLogic) {
        match and {
            TwoSATLogic::Implies(u, v) => {
                self.graph[u.val()].push(v.val());
                self.graph[v.negate().val()].push(u.negate().val());
            }
            TwoSATLogic::Or(u, v) => {
                self.graph[u.negate().val()].push(v.val());
                self.graph[v.negate().val()].push(u.val());
            }
            TwoSATLogic::Iff(u, v) => {
                self.graph[u.val()].push(v.val());
                self.graph[v.val()].push(u.val());
                self.graph[u.negate().val()].push(v.negate().val());
                self.graph[v.negate().val()].push(u.negate().val());
            }
        }
    }
}

#[macro_export]
macro_rules! clause2 {
    (not $i:tt or not $j:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, false), TwoSATTerm($j, false))
    };
    (not $i:tt or $j:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, false), TwoSATTerm($j, true))
    };
    ($i:tt or not $j:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, true), TwoSATTerm($j, false))
    };
    ($i:tt or $j:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, true), TwoSATTerm($j, true))
    };
    (not $i:tt => not $j:tt) => {
        TwoSATLogic::Implies(TwoSATTerm($i, false), TwoSATTerm($j, false))
    };
    (not $i:tt => $j:tt) => {
        TwoSATLogic::Implies(TwoSATTerm($i, false), TwoSATTerm($j, true))
    };
    ($i:tt => not $j:tt) => {
        TwoSATLogic::Implies(TwoSATTerm($i, true), TwoSATTerm($j, false))
    };
    ($i:tt => $j:tt) => {
        TwoSATLogic::Implies(TwoSATTerm($i, true), TwoSATTerm($j, true))
    };
    (not $i:tt <=> not $j:tt) => {
        TwoSATLogic::Iff(TwoSATTerm($i, false), TwoSATTerm($j, false))
    };
    (not $i:tt <=> $j:tt) => {
        TwoSATLogic::Iff(TwoSATTerm($i, false), TwoSATTerm($j, true))
    };
    ($i:tt <=> not $j:tt) => {
        TwoSATLogic::Iff(TwoSATTerm($i, true), TwoSATTerm($j, false))
    };
    ($i:tt <=> $j:tt) => {
        TwoSATLogic::Iff(TwoSATTerm($i, true), TwoSATTerm($j, true))
    };
    (not $i:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, false), TwoSATTerm($i, false))
    };
    ($i:tt) => {
        TwoSATLogic::Or(TwoSATTerm($i, true), TwoSATTerm($i, true))
    };
}

#[cfg(test)]
mod test_two_sat {
    use crate::opt::two_sat::*;

    #[test]
    fn test_two_sat_1() {
        let mut sat = TwoSAT::new(3);
        sat += clause2!(0 => 1);
        sat += clause2!(1 => 2);
        sat += clause2!(2 => 0);
        assert!(sat.solve());
    }
    #[test]
    fn test_two_sat_2() {
        let mut sat = TwoSAT::new(2);
        sat += clause2!(0 or 1);
        sat += clause2!(0 => not 1);
        sat += clause2!(1 => not 0);
        assert!(sat.solve());
    }
    #[test]
    fn test_two_sat_3() {
        let mut sat = TwoSAT::new(2);
        sat += clause2!(0 or 1);
        sat += clause2!(not 0 or 1);
        sat += clause2!(0 or not 1);
        sat += clause2!(not 1 or not 0);
        assert!(!sat.solve());
    }
    #[test]
    fn test_two_sat_4() {
        let mut sat = TwoSAT::new(2);
        sat += clause2!(0 => 1);
        sat += clause2!(1 => 0);
        sat += clause2!(0 or 1);
        sat += clause2!(not 0 or not 1);
        assert!(!sat.solve());
    }
    #[test]
    fn test_two_sat_5() {
        let mut sat = TwoSAT::new(2);
        sat += clause2!(0);
        sat += clause2!(1);
        assert!(sat.solve());
    }
    #[test]
    fn test_two_sat_6() {
        let mut sat = TwoSAT::new(2);
        sat += clause2!(0);
        sat += clause2!(not 0);
        assert!(!sat.solve());
    }

    #[test]
    fn test_iff_1() {
        let mut sat = TwoSAT::new(3);
        sat += clause2!(0 <=> 1);
        sat += clause2!(not 1 <=> not 2);
        sat += clause2!(2 <=> 0);
        assert!(sat.solve());
    }

    #[test]
    fn test_iff_2() {
        let mut sat = TwoSAT::new(3);
        sat += clause2!(0 <=> 1);
        sat += clause2!(not 1 <=> 2);
        sat += clause2!(2 <=> 0);
        assert!(!sat.solve());
    }
}
