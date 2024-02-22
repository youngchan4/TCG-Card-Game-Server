use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDrawCountInfo {
    player_draw_count_map: HashMap<PlayerIndex, i32>,
}

impl PlayerDrawCountInfo {
    pub fn new(player_draw_count_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerDrawCountInfo {
            player_draw_count_map
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        todo!()
    }
}