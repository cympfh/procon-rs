/// Algebra - Field ((+, 0), (*, 1), /)
use crate::algebra::ring::*;

pub trait Field: Ring + std::ops::Div {}

impl Field for i64 {}
impl Field for f64 {}
