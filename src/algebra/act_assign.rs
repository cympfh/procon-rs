/// Algebra - Assign Monoidal Act
use crate::algebra::act::*;
use crate::algebra::monoid::*;

#[derive(Debug, Clone, Copy)]
enum Assign<X> {
    Some(X),
    None,
}
impl<X> std::ops::Mul for Assign<X> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, &other) {
            (x, Assign::None) => x,
            _ => other,
        }
    }
}
impl<X: Copy> Act<X> for Assign<X> {
    fn act(&self, other: X) -> X {
        match *self {
            Assign::None => other,
            Assign::Some(x) => x,
        }
    }
}
impl<X: Copy> Monoid for Assign<X> {
    fn unit() -> Self {
        Assign::None
    }
}

#[cfg(test)]
mod test_ratio {
    use crate::algebra::act_assign::*;
    #[test]
    fn it_works() {
        assert_eq!(Assign::Some(1).act(0), 1);
        assert_eq!(Assign::None.act(0), 0);
        assert_eq!((Assign::Some(1) * Assign::Some(2)).act(0), 2);
        assert_eq!((Assign::None * Assign::Some(2)).act(0), 2);
        assert_eq!((Assign::Some(1) * Assign::None).act(0), 1);
        assert_eq!((Assign::None * Assign::None).act(0), 0);
        assert_eq!((Assign::None * Assign::None * Assign::Some(9)).act(0), 9);
    }
}
