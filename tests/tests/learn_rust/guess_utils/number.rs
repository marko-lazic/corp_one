pub fn validate(&value: &i32) {
    if value < 1 || value > 100 {
        panic!("Guess value must be betwen 1 and 100, got {}", value);
    }
}
