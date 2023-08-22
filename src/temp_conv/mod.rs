pub(crate) fn to_celsius(farenheit: f32) -> f32 {
    (5.0 / 9.0) * (farenheit - 32.0)
}

pub(crate) fn to_farenheit(celsius: f32) -> f32 {
    celsius * (9.0 / 5.0) + 32.0
}
