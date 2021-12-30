/// Algebra - Assign Monoidal Act
use crate::algebra::act::*;
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy)]
pub enum Assign<X> {
    Some(X),
    None,
}
monoid! {
    Assign<X> where [X];
    one = Assign::None;
    mul(self, other) = {
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

#[cfg(test)]
mod test_act_assign {
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
