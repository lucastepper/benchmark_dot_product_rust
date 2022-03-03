pub fn dot_prod(arr1: &[f64], arr2: &[f64]) -> f64 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(x, y| *x * *y)
        .sum()
}