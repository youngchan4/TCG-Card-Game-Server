use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackUnitResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
}

impl AttackUnitResponseForm {
    pub fn new(is_success: bool,
               false_message_enum: i32,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> Self {
        AttackUnitResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_health_point_map,
            player_field_unit_harmful_effect_map,
            player_field_unit_death_map
        }
    }

    pub fn default() -> AttackUnitResponseForm {

        AttackUnitResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),)
    }
    pub fn from_response_with_message(false_message: FalseMessage) -> AttackUnitResponseForm {

        AttackUnitResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),)
    }
}