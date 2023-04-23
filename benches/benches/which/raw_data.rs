pub struct RawData {
    data: Vec<u8>,
}

impl RawData {
    pub fn new(symblol: char, len: usize) -> Self {
        let mut v: Vec<u8> = Vec::new();

        for _ in 0..len {
            let mut byte_slice = [0u8; 4];
            symblol.encode_utf8(&mut byte_slice);
            v.extend_from_slice(&byte_slice[0..symblol.len_utf8()]);
        }

        RawData { data: v }
    }

    pub fn to_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular() {
        let raw_data = RawData::new('a', 16);
        assert_eq!(raw_data.to_str(), "aaaaaaaaaaaaaaaa");
    }

    #[test]
    fn test_utf8() {
        let raw_data = RawData::new('🦀', 4);
        assert_eq!(raw_data.to_str(), "🦀🦀🦀🦀");
    }
}
