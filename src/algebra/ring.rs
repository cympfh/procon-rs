/// Algebra - Ring ((+, 0), (*, 1))
use crate::algebra::group_additive::*;
use crate::algebra::monoid::*;

pub trait Ring: AGroup + Monoid {}

impl Ring for i64 {}
impl Ring for f64 {}
