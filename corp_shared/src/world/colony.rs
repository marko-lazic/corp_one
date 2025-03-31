use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
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
