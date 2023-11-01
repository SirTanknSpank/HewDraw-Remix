use super::*;
use globals::*;

mod special_lw_cancel;

// Prevent going into air jet hammer when Special is released during Jumpsquat
#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

#[status_script(agent = "dedede", status = *FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_HAMMER_MAX_CHARGE_FLAG){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX);
        StatusModule::change_status_force(fighter.boma(), *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
        return original!(fighter);
    }
    return original!(fighter);

}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_HAMMER_MAX_CHARGE_FLAG){
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("dedede_heavyattack"), false);
        EffectModule::kill_joint_id(fighter.module_accessor, Hash40::new("hammer2"), false, false);
        VarModule::off_flag(fighter.battle_object, vars::dedede::instance::JET_HAMMER_MAX_CHARGE_FLAG);

    }
    if MotionModule::motion_kind(fighter.boma()) ==smash::hash40("special_lw_max"){
        //if MotionModule::frame(fighter.boma()) > 104.0
        //&& fighter.is_situation(*SITUATION_KIND_AIR){
            //StatusModule::change_status_force(fighter.boma(), *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        //}
        if AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT){
            fighter.check_jump_cancel(false, false);
        }
    }
    
    return original!(fighter);

}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_wait_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        let cancel_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::dedede::SPECIAL_LW_CANCEL);
        fighter.change_status(cancel_status.into(), true.into());

        return 0.into()
    }
    else{
        return original!(fighter);
    }
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_walk_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        let cancel_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::dedede::SPECIAL_LW_CANCEL);
        fighter.change_status(cancel_status.into(), true.into());

        return 0.into()
    }
    else{
        return original!(fighter);
    }
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_fall_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX) {
        let cancel_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::dedede::SPECIAL_LW_CANCEL);
        fighter.change_status(cancel_status.into(), true.into());

        return 0.into()
    }
    else{
        return original!(fighter);
    }
}

// Reset Gordo recatch flags on despawn
#[status_script(agent = "dedede_gordo", status = WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dedede_gordo_dead_end(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_DEDEDE{
        let dedede = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_flag(dedede, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, true); 
        VarModule::set_int(dedede, vars::dedede::instance::RECATCH_COUNTER, 0); 
        VarModule::set_flag(dedede, vars::dedede::instance::IS_DASH_GORDO, false);
    }
    return original!(weapon);
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dedede_special_hi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

    return 0.into();
}

pub fn install(){
    smashline::install_status_scripts!(
        special_lw_jump_squat_exec,
        special_lw_exec,
        special_lw_wait_exec,
        special_lw_walk_exec,
        special_lw_fall_exec,
        special_lw_attack_exec,
        dedede_gordo_dead_end,
        dedede_special_hi_status_main,
    );
}

pub fn add_statuses(){
    special_lw_cancel::install();
}