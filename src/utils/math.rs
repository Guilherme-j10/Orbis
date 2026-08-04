pub fn interpolation(v: f32, i: Vec<f32>, o: Vec<f32>) -> f32 {
    o[0] + (((v - i[0]) / (i[1] - i[0])) * (o[1] - o[0]))
}