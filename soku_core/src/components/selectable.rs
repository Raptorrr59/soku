use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Selectable {
    pub is_selected: bool,
    pub is_hovered: bool,
}
