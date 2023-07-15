pub fn convert(temp: f64) -> f64 {
    let temp_c: f64 = (temp - 32.0) * (5.0/9.0);
    return temp_c;
}

