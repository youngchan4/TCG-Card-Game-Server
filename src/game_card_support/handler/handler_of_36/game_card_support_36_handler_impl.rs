use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;

pub struct SupportCard_36_Function;

impl GameCardSupportHandler for SupportCard_36_Function {
    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect {
        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 0);
        game_card_support_effect.set_removal_amount_of_opponent_field_energy(2);

        game_card_support_effect
    }
}