pub fn mean(vector: &Vec<f32>) -> f32 {
    vector.iter().sum::<f32>() / vector.len() as f32
}

pub fn covariance(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() || x.len() < 2 || y.len() < 2 {
        panic!("x and y have to be in the same size!")
    }

    let x_mean = mean(&x);
    let y_mean = mean(&y);

    let prod_sum: f32 = x
        .iter()
        .zip(y.iter())
        .map(|(&x, &y)| (x - x_mean) * (y - y_mean))
        .collect::<Vec<f32>>()
        .iter()
        .sum();

    prod_sum / (x.len()) as f32
}

pub fn variance(x: &Vec<f32>) -> f32 {
    covariance(&x, &x)
}
