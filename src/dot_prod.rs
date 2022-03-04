pub fn dot_prod(arr1: &[f64], arr2: &[f64]) -> f64 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(x, y)| *x * *y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const NDATA: usize = usize::pow(10, 6);


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
    }

    #[test]
    fn test_dot_base() {
        let test_data = TestData::new();
        test_data.test_dot_prod(dot_prod);
    }
}
