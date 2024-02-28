use super::*;
use globals::*;

// Prevent going into air jet hammer when Special is released during Jumpsquat

unsafe extern "C" fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::is_jump_mini_button(fighter.module_accessor){
        VarModule::on_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP);
    }

    return 0.into();
}

unsafe extern "C" fn dedede_special_lw_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP){
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, -2.5, 0.0));
        VarModule::off_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP);
    }

    return original!(fighter);
}

unsafe extern "C" fn dedede_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    VarModule::set_float(fighter.battle_object, vars::dedede::instance::PRE_JETHAMMER_SPEED_X, x_speed);

    return original!(fighter);
}

// Reset Gordo recatch flags on despawn
unsafe extern "C" fn dedede_gordo_dead_end(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_DEDEDE{
        let dedede = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_flag(dedede, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, true); 
        VarModule::set_int(dedede, vars::dedede::instance::RECATCH_COUNTER, 0); 
        VarModule::set_flag(dedede, vars::dedede::instance::IS_DASH_GORDO, false);
    }
    return smashline::original_status(End, weapon, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD)(weapon)
}

unsafe extern "C" fn dedede_special_hi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

    return 0.into();
}


pub fn install() {
    smashline::Agent::new("dedede_gordo")
        .status(End, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, dedede_gordo_dead_end)
        .install();
    smashline::Agent::new("dedede")
        .status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, special_lw_jump_squat_exec)
        .status(Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, dedede_special_hi_status_main)
        .status(Pre, *FIGHTER_STATUS_SPECIAL_LW, special_lw_pre)
        .status (Main, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP, special_lw_jump_main)
        .install();
}