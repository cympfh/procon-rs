/// Algebra - Group (*, 1, inv)
use crate::algebra::monoid::*;

pub trait Group: Monoid {
    fn inv(self) -> Self;
}
