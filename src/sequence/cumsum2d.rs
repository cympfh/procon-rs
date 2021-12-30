/// Sequence - Cumulative Summation 2D of Additive Group (+, 0)
use crate::algebra::group_additive::*;

#[derive(Debug)]
pub struct Cumsum2d<T>(Vec<Vec<T>>);
impl<T: Copy + AGroup> Cumsum2d<T> {
    pub fn new(data: &Vec<Vec<T>>) -> Self {
        let h = data.len();
        let w = data[0].len();
        let mut cs = vec![vec![T::zero(); w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                cs[i + 1][j + 1] = data[i][j] + cs[i][j + 1] + cs[i + 1][j] - cs[i][j];
            }
        }
        Self(cs)
    }
    fn sum_up(&self, x: usize, y: usize) -> T {
        let x = std::cmp::min(x, self.0.len());
        let y = std::cmp::min(y, self.0[0].len());
        self.0[x][y]
    }
    pub fn sum(&self, xrange: std::ops::Range<usize>, yrange: std::ops::Range<usize>) -> T {
        if xrange.end <= xrange.start || yrange.end <= yrange.start {
            T::zero()
        } else {
            self.sum_up(xrange.end, yrange.end)
                - self.sum_up(xrange.start, yrange.end)
                - self.sum_up(xrange.end, yrange.start)
                + self.sum_up(xrange.start, yrange.start)
        }
    }
}

#[cfg(test)]
mod test_cumsum {
    use crate::sequence::cumsum2d::*;

    fn naiiv(
        tate: std::ops::Range<usize>,
        yoko: std::ops::Range<usize>,
        xs: &Vec<Vec<i64>>,
    ) -> i64 {
        tate.map(|i| yoko.clone().map(|j| xs[i][j]).sum::<i64>())
            .sum::<i64>()
    }

    fn autocheck(xs: Vec<Vec<i64>>) {
        let h = xs.len();
        let w = xs[0].len();
        let cs = Cumsum2d::new(&xs);
        for top in 0..h {
            for bot in 0..h {
                for left in 0..w {
                    for right in 0..w {
                        assert_eq!(
                            cs.sum(top..bot, left..right),
                            naiiv(top..bot, left..right, &xs)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test() {
        autocheck(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]);
        autocheck(vec![vec![0, -1, 1], vec![1, 2, 3], vec![-1, -1, -1]]);
    }
}
