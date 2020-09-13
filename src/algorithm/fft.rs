/// Algorithm - Fast Fourier Transform
use crate::algebra::complex::*;

pub struct FFT;
impl FFT {
    fn _fft(f: &Vec<Complex<f64>>, dir: f64) -> Vec<Complex<f64>> {
        let n = f.len();
        if n < 2 {
            return f.clone();
        }
        let f0: Vec<Complex<f64>> = (0..n / 2).map(|i| f[i * 2].clone()).collect();
        let f1: Vec<Complex<f64>> = (0..n / 2).map(|i| f[i * 2 + 1].clone()).collect();
        let g0 = FFT::_fft(&f0, dir);
        let g1 = FFT::_fft(&f1, dir);
        let pi = (-1.0_f64).acos();
        let theta = 2.0 * pi / (n as f64);
        let z = Complex(theta.cos(), theta.sin() * dir);
        let mut ac = Complex::unit();
        let mut g = vec![];
        for i in 0..n {
            g.push(g0[i % (n / 2)] + (ac * g1[i % (n / 2)]));
            ac = z * ac;
        }
        g
    }
    fn fft(f: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        FFT::_fft(f, 1.0)
    }
    fn defft(f: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        FFT::_fft(f, -1.0)
    }

    fn _convolution(x: &Vec<Complex<f64>>, y: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let m = x.len();
        let xh = FFT::fft(&x);
        let yh = FFT::fft(&y);
        let xyh = (0..m).map(|i| (xh[i] * yh[i]) * (1.0 / m as f64)).collect();
        FFT::defft(&xyh)
    }

    pub fn convolution(f: &Vec<f64>, g: &Vec<f64>) -> Vec<f64> {
        assert!(f[0] == 0.0);
        assert!(g[0] == 0.0);
        let n = std::cmp::max(f.len(), g.len());
        let mut m = 1;
        while m < n {
            m <<= 1
        }
        m <<= 1; // length should be 2**pow
        let x: Vec<Complex<f64>> = f
            .iter()
            .map(|&k| Complex(k, 0.0))
            .chain((0..m - f.len()).map(|_| Complex(0.0, 0.0)))
            .collect();
        let y: Vec<Complex<f64>> = g
            .iter()
            .map(|&k| Complex(k, 0.0))
            .chain((0..m - g.len()).map(|_| Complex(0.0, 0.0)))
            .collect();

        let z = FFT::_convolution(&x, &y);
        z.iter().map(|c| c.0).collect()
    }
}

#[cfg(test)]
mod test_fft {
    use crate::algorithm::fft::*;

    #[test]
    fn it_works() {
        let a = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let b = vec![0.0, 1.0, 2.0, 4.0, 8.0];
        let expected = vec![0.0, 0.0, 1.0, 4.0, 11.0, 26.0, 36.0, 40.0, 32.0];
        let c = FFT::convolution(&a, &b);
        for k in 0..expected.len() {
            assert!((expected[k] - c[k]).abs() < 1e-6);
        }
    }
}
