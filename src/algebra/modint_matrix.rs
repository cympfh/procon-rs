/// Algebra - Matrix of ModInt
use crate::algebra::matrix::*;
use crate::algebra::modint::*;

define_group!(ModInt, mint!(0));
define_ring!(ModInt, mint!(1));
type ModMatrix = Matrix<ModInt>;
