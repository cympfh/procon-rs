/// Algebra - Addition Monoidal Act
use crate::algebra::act::*;
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;
use crate::monoid; // IGNORE

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Add<X: AGroup>(pub X);
monoid! {
    Add<X> where [X:AGroup];
    one = Add(X::zero());
    mul(self, other) = {
        Add(self.0 + other.0)
    }
}
impl<X: AGroup + Copy> Act<X> for Add<X> {
    fn act(&self, other: X) -> X {
        self.0 + other
    }
}

#[cfg(test)]
mod test_act_add {
    use crate::algebra::act_add::*;
    #[test]
    fn it_works() {
        assert_eq!(Add(2).act(5), 7);
        assert_eq!((Add(1) * Add(2)).act(5), 8);
    }
}
