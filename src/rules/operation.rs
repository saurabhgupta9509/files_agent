use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Operation {
    Create,
    Modify,
    Delete,
    Rename,
    Move,
}
