use lightyear::prelude::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum Colony {
    Cloning,
    Iris,
    Liberte,
    Playground,
}

impl Default for Colony {
    fn default() -> Self {
        Self::Cloning
    }
}
