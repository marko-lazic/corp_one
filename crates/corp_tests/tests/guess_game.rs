use crate::guess_utils::number;

#[test]
#[should_panic]
fn test_validation() {
    Guess::new(0);
}

#[test]
fn test_get_value() {
    let guess = Guess::new(1);

    assert_eq!(guess.value(), 1);
}
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        number::validate(&value);
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
