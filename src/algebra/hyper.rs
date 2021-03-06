/// Algebra - Hyper Numbers (numbers with infinity)
use crate::algebra::group::*;
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
            panic!()
        }
    }
}
impl<X: Group> std::ops::Add for Hyper<X> {
    type Output = Self;
    fn add(self, rhs: Hyper<X>) -> Hyper<X> {
        match (self, rhs) {
            (Real(x), Real(y)) => Real(x + y),
            (Inf, _) => Inf,
            (_, Inf) => Inf,
            _ => NegInf,
        }
    }
}
impl<X: Clone + Group> std::ops::AddAssign for Hyper<X> {
    fn add_assign(&mut self, rhs: Hyper<X>) {
        *self = (*self).clone() + rhs;
    }
}
impl<X: Group> std::ops::Sub for Hyper<X> {
    type Output = Self;
    fn sub(self, rhs: Hyper<X>) -> Hyper<X> {
        self + (-rhs)
    }
}
impl<X: Clone + Group> std::ops::SubAssign for Hyper<X> {
    fn sub_assign(&mut self, rhs: Hyper<X>) {
        *self = (*self).clone() - rhs;
    }
}
impl<X: Group> std::ops::Neg for Hyper<X> {
    type Output = Self;
    fn neg(self) -> Hyper<X> {
        match self {
            Inf => NegInf,
            NegInf => Inf,
            Real(x) => Real(-x),
        }
    }
}
impl<X: Group> std::iter::Sum for Hyper<X> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Hyper::zero(), std::ops::Add::add)
    }
}
impl<X: Group> Group for Hyper<X> {
    fn zero() -> Self {
        Hyper::Real(X::zero())
    }
}
impl<X: Group> std::ops::Add<X> for Hyper<X> {
    type Output = Self;
    fn add(self, other: X) -> Self {
        match self {
            Inf => Inf,
            NegInf => NegInf,
            Real(x) => Real(x + other),
        }
    }
}

#[cfg(test)]
mod test_hyper {
    use crate::algebra::hyper::*;
    #[test]
    fn it_works() {
        assert_eq!(Hyper::<i32>::zero(), Hyper::Real(0));
        assert_eq!(Hyper::Real(42).unwrap(), 42);
        assert!(Hyper::Real(0_i32) < Hyper::Real(1));
        assert!(Hyper::NegInf < Hyper::Real(1));
        assert!(Hyper::Real(0) < Hyper::Inf);
        assert!(Hyper::<i32>::NegInf < Hyper::Inf);
        assert_eq!(Hyper::Real(1) + 3, Hyper::Real(4));
    }
    #[test]
    fn assign_op() {
        {
            let mut a = Hyper::Real(0_i32);
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
}
