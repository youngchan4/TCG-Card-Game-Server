use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::battle_ready_monitor::service::response::battle_ready_response::BattleReadyResponse;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    ACCOUNT_REGISTER(AccountRegisterResponse),
    ACCOUNT_LOGIN(AccountLoginResponse),

    ACCOUNT_DECK_REGISTER(AccountDeckRegisterResponse),

    BATTLE_MATCH(BattleMatchResponse),
    BATTLE_READY(BattleReadyResponse),

    PROGRAM_EXIT(ClientProgramExitResponse),
}