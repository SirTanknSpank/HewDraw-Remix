use super::*;

// FIGHTER_STATUS_KIND_ESCAPE_AIR

pub unsafe extern "C" fn escape_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::pikmin::instance::SPECIAL_HI_CANCEL_ESCAPE_AIR);
    fighter.status_end_EscapeAir()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_end);
}
