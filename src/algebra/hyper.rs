/// Algebra - Hyper Numbers (numbers with infinity)
use crate::agroup; // IGNORE
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::algebra::ring::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hyper<X> {
    NegInf,
    Real(X),
    Inf,
}
use Hyper::{Inf, NegInf, Real};
impl<X> Hyper<X> {
    pub fn unwrap(self) -> X {
        if let Hyper::Real(x) = self {
            x
        } else {
            panic!("Could not unwrap Hyper")
        }
    }
}
agroup! {
    Hyper<X> where [X: AGroup];
    zero = Real(X::zero());
    add(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x + y),
            (Inf, _) => Inf,
            (_, Inf) => Inf,
            _ => NegInf,
        }
    };
    neg(self) = {
        match self {
            Inf => NegInf,
            NegInf => Inf,
            Real(x) => Real(-x),
        }
    };
}
monoid! {
    Hyper<X> where [X: Monoid];
    one = Real(X::one());
    mul(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x * y),
            (Inf, Inf) | (NegInf, NegInf) => Inf,
            _ => NegInf,
        }
    };
}
impl<X: AGroup + Monoid> Ring for Hyper<X> {}
impl<X: std::ops::Add<Output = X>> std::ops::Add<X> for Hyper<X> {
    type Output = Self;
    fn add(self, y: X) -> Hyper<X> {
        match (self, y) {
            (Real(x), y) => Real(x + y),
            (Inf, _) => Inf,
            _ => NegInf,
        }
    }
}
impl<X: Clone + AGroup> std::ops::AddAssign<X> for Hyper<X> {
    fn add_assign(&mut self, y: X) {
        *self = (*self).clone() + Real(y);
    }
}

#[cfg(test)]
mod test_hyper {
    use crate::algebra::hyper::*;
    #[test]
    fn it_works() {
        assert_eq!(Hyper::<i64>::zero(), Hyper::Real(0));
        assert_eq!(Hyper::Real(42).unwrap(), 42);
        assert!(Hyper::Real(0_i64) < Hyper::Real(1));
        assert!(Hyper::NegInf < Hyper::Real(1));
        assert!(Hyper::Real(0) < Hyper::Inf);
        assert!(Hyper::<i64>::NegInf < Hyper::Inf);
        assert_eq!(Hyper::Real(1) + 3, Hyper::Real(4));
    }
    #[test]
    fn assign_op() {
        {
            let mut a = Hyper::Real(0_i64);
            a += Hyper::Real(3);
            assert_eq!(a, Hyper::Real(3));
            a -= Hyper::Real(6);
            assert_eq!(a, Hyper::Real(-3));
            a += Hyper::Inf;
            assert_eq!(a, Hyper::Inf);
            a -= Hyper::Inf;
            assert_eq!(a, Hyper::Inf);
        }
    }

    #[test]
    #[should_panic]
    fn test_inf_cannot_unwrap() {
        Hyper::<i64>::Inf.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_neginf_cannot_unwrap() {
        Hyper::<i64>::NegInf.unwrap();
    }
}
