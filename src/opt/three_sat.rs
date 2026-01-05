use crate::num::random::pcg::*;

pub struct ThreeSAT {
    n: usize, // 変数 [0,1,.., n-1]
    cnf: Vec<ThreeSATTerm>,
}

impl ThreeSAT {
    pub fn new(n: usize) -> Self {
        Self { n, cnf: vec![] }
    }

    /// 充足判定 - Schoening's Algorithm
    /// 割当が見つかれば Some(割当), 見つからなければ None
    pub fn solve(&self) -> Option<Vec<bool>> {
        if self.cnf.is_empty() {
            return Some(vec![false; self.n]);
        }
        let seed = self.n as u64;
        let mut rand = PCG::new(seed);
        // Schoening's Algorithm パラメータ
        let max_tries = 3 * self.n; // 外側のループ回数
        let max_flips = 3 * self.n; // 内側のループ回数
        for _ in 0..max_tries {
            let mut assignment: Vec<bool> = (0..self.n).map(|_| rand.gen()).collect();
            for _ in 0..max_flips {
                if let Some(unsatisfied_clause) = self.find_unsatisfied_clause(&assignment) {
                    let vars = self.get_variables(&unsatisfied_clause);
                    if !vars.is_empty() {
                        let idx = (rand.gen::<u64>() as usize) % vars.len();
                        let var_to_flip = vars[idx];
                        assignment[var_to_flip] = !assignment[var_to_flip];
                    }
                } else {
                    return Some(assignment);
                }
            }
            if self.is_satisfied(&assignment) {
                return Some(assignment);
            }
        }
        None
    }

    pub fn is_satisfied(&self, assignment: &Vec<bool>) -> bool {
        self.cnf
            .iter()
            .all(|clause| self.eval_term(clause, assignment))
    }

    fn eval_term(&self, term: &ThreeSATTerm, assignment: &Vec<bool>) -> bool {
        match term {
            ThreeSATTerm::Var(i) => assignment[*i],
            ThreeSATTerm::NotVar(i) => !assignment[*i],
            ThreeSATTerm::Or(terms) => terms.iter().any(|t| self.eval_term(t, assignment)),
        }
    }

    fn find_unsatisfied_clause(&self, assignment: &Vec<bool>) -> Option<ThreeSATTerm> {
        self.cnf
            .iter()
            .find(|clause| !self.eval_term(clause, assignment))
            .cloned()
    }

    fn get_variables(&self, term: &ThreeSATTerm) -> Vec<usize> {
        let mut vars = vec![];
        self.collect_variables(term, &mut vars);
        vars
    }

    fn collect_variables(&self, term: &ThreeSATTerm, vars: &mut Vec<usize>) {
        match term {
            ThreeSATTerm::Var(i) => vars.push(*i),
            ThreeSATTerm::NotVar(i) => vars.push(*i),
            ThreeSATTerm::Or(terms) => {
                for t in terms {
                    self.collect_variables(t, vars);
                }
            }
        }
    }
}

impl std::ops::AddAssign<ThreeSATTerm> for ThreeSAT {
    fn add_assign(&mut self, term: ThreeSATTerm) {
        self.cnf.push(term);
    }
}

#[derive(Debug, Clone)]
pub enum ThreeSATTerm {
    Var(usize),
    NotVar(usize),
    Or(Vec<ThreeSATTerm>),
}

#[macro_export]
macro_rules! clause3 {
    (not $i:tt) => { ThreeSATTerm::NotVar($i) };
    ($i:tt) => { ThreeSATTerm::Var($i) };
    (not $i:tt or $($rest:tt)+) => {
        ThreeSATTerm::Or(vec![
            ThreeSATTerm::NotVar($i),
            clause3!($($rest)+)
        ])
    };
    ($i:tt or $($rest:tt)+) => {
        ThreeSATTerm::Or(vec![
            ThreeSATTerm::Var($i),
            clause3!($($rest)+)
        ])
    };
}

#[cfg(test)]
mod test_three_sat {
    use crate::opt::three_sat::*;

    #[test]
    fn test_three_sat_1() {
        let mut sat = ThreeSAT::new(2);
        sat += clause3!(0 or 1);
        sat += clause3!(0 or not 1);
        sat += clause3!(not 0 or not 1);
        let assignment = sat.solve().unwrap();
        assert!(sat.is_satisfied(&assignment));
    }

    #[test]
    fn test_three_sat_2() {
        let mut sat = ThreeSAT::new(2);
        sat += clause3!(0 or 1);
        sat += clause3!(0 or not 1);
        sat += clause3!(not 0 or 1);
        sat += clause3!(not 0 or not 1);
        let assignment = sat.solve();
        assert!(assignment.is_none());
    }

    #[test]
    fn test_three_sat_kyouseibi() {
        let mut sat = ThreeSAT::new(4);
        sat += clause3!(0 or 1 or 2);
        sat += clause3!(3 or 1 or not 2);
        sat += clause3!(not 0 or 3 or 2);
        sat += clause3!(not 0 or not 3 or 1);
        sat += clause3!(not 3 or not 1 or 2);
        sat += clause3!(not 0 or not 1 or not 2);
        sat += clause3!(0 or not 3 or not 2);
        sat += clause3!(0 or 3 or not 1);
        let assignment = sat.solve();
        assert!(assignment.is_none());
    }

    #[test]
    fn test_three_sat_kyouseibi_2() {
        let mut sat = ThreeSAT::new(4);
        sat += clause3!(0 or 1 or 2);
        sat += clause3!(3 or 1 or not 2);
        sat += clause3!(not 0 or 3 or 2);
        sat += clause3!(not 0 or not 3 or 1);
        sat += clause3!(not 3 or not 1 or 2);
        sat += clause3!(not 0 or not 1 or not 2);
        sat += clause3!(0 or 3 or not 1);
        let assignment = sat.solve().unwrap();
        assert!(sat.is_satisfied(&assignment));
    }
}
