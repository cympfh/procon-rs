/// 代数 - 環上の加群 (Right-Module)

// Self should be AGroup (Additive Group)
pub trait Module<R>: std::ops::Mul<R, Output = Self> {}

#[cfg(test)]
mod test_module {
    use crate::algebra::module::*;
    #[test]
    fn test_vector_is_module() {
        struct MyVect {
            data: Vec<i64>,
        }
        impl std::ops::Mul<i64> for MyVect {
            type Output = Self;
            fn mul(self, scalar: i64) -> Self::Output {
                let data: Vec<i64> = self.data.iter().map(|&x| x * scalar).collect();
                Self { data }
            }
        }
        impl Module<i64> for MyVect {}
        let v = MyVect {
            data: vec![1, 2, 3],
        };
        let u = v * (-1);
        assert_eq!(u.data, vec![-1, -2, -3]);
    }
}
