use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Faction {
    EC,
    CMG,
    VI,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Rank {
    #[default]
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}
