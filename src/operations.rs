pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}

pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

pub fn div(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero is not allowed")
    } else {
        Ok(a / b)
    }
}
