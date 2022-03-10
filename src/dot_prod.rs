#![feature(test)]
use rayon::prelude::*;


pub fn dot_prod(arr1: &[f64], arr2: &[f64]) -> f64 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(x, y)| *x * *y)
        .sum()
}

pub fn dot_prod_parallel(arr1: &[f64], arr2: &[f64]) -> f64 {
    arr1.par_iter()
        .zip(arr2.par_iter())
        .map(|(x, y)| *x * *y)
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    const NDATA: usize = usize::pow(10, 5);


    struct TestData {
        arr1: Vec<f64>,
        arr2: Vec<f64>,
        ref_val: f64,
    }
    impl TestData {
        fn new() -> Self {
            TestData {
                arr1: (0..NDATA).into_iter().map(|x| x as f64).collect(),
                arr2: vec!(1.; NDATA),
                ref_val: (NDATA * (NDATA - 1) / 2) as f64,
            }
        }
        fn test_dot_prod(&self, dot_prod_func: fn (&[f64], &[f64]) -> f64) {
            let dot_prod = dot_prod_func(&self.arr1, &self.arr2);
            assert!(dot_prod == self.ref_val);
        }
        fn run_dot_prod(&self, dot_prod_func: fn (&[f64], &[f64]) -> f64) {
            let _dot_prod = dot_prod_func(&self.arr1, &self.arr2);
        }
    }

    #[test]
    fn test_dot_base() {
        let test_data = TestData::new();
        test_data.test_dot_prod(dot_prod);
    }
    #[test]
    fn test_dot_prod_parallel() {
        let test_data = TestData::new();
        test_data.test_dot_prod(dot_prod_parallel);
    }
    #[bench]
    fn bench_dot_prod(b: &mut test::Bencher) {
        let test_data = TestData::new();
        b.iter(|| {
            test::black_box(test_data.run_dot_prod(dot_prod));
        });
    }
    #[bench]
    fn bench_dot_prod_parallel(b: &mut test::Bencher) {
        let test_data = TestData::new();
        b.iter(|| {
            test::black_box(test_data.run_dot_prod(dot_prod_parallel));
        });
    }
}
