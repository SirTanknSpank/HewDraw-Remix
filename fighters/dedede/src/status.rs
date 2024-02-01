use super::*;
use globals::*;

mod special_lw_hi_swing;
mod special_lw_down_swing;

#[smashline::fighter_init]
fn dedede_init(fighter: &mut L2CFighterCommon){
    unsafe{
        if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_DEDEDE{
            return;
        }
        VarModule::set_int(fighter.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0);
    }
}

/* SPECIAL LW */

// Prevent going into air jet hammer when Special is released during Jumpsquat
#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::is_jump_mini_button(fighter.module_accessor){
            VarModule::on_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP);
    }
    
    return 0.into();
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP){
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, -2.5, 0.0));
        VarModule::off_flag(fighter.battle_object, vars::dedede::instance::JET_MINI_JUMP);
    }
    
    return original!(fighter);
}

#[status_script(agent = "dedede", status = *FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    VarModule::set_float(fighter.battle_object, vars::dedede::instance::PRE_JETHAMMER_SPEED_X, x_speed);

    return original!(fighter);
}

#[status_script(agent = "dedede", status = *FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI))
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.motion_frame() > 8.0{
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, true);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, Hash40::new("wait"), false, 0.0);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, *WEAPON_DEDEDE_JETHAMMER_STATUS_KIND_WAIT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_START);
        let sound = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_dedede_special_l02"), true, false, false, false, smash::app::enSEType(0));
        WorkModule::set_int(fighter.module_accessor, sound as i32,  *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_INT_SE_HANDLE);
    }

    return original!(fighter);
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK  | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();

}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX){
        let hold = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT) * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hold_attack_mul"));

        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, hold);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX);
    }
    else{
        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, 0.0);
    }

    if ControlModule::get_stick_y(fighter. module_accessor) < -0.5 && fighter.is_situation(*SITUATION_KIND_AIR){
        let swing_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::dedede::SPECIAL_LW_DOWN_SWING);
        let hold = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT) * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hold_attack_mul"));

        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, hold);
        fighter.change_status(swing_status.into(), true.into());

        return 0.into();
    }
    else if ControlModule::get_stick_y(fighter. module_accessor) > 0.5 {
        let swing_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::dedede::SPECIAL_LW_HI_SWING);
        let hold = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT) * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hold_attack_mul"));

        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, hold);
        fighter.change_status(swing_status.into(), true.into());

        return 0.into();
    }
    else{
        ArticleModule::change_motion(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, Hash40::new("attack"), false, 0.0);
        return original!(fighter);
    }
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX){
        let max_hold = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT) * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hold_attack_mul"));

        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, max_hold);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_FLAG_HOLD_MAX);
        
    }
    else{
        VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, 0.0);
    }
    
    ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, false, *WEAPON_DEDEDE_JETHAMMER_STATUS_WORK_FLAG_HOLD_MAX);

    return original!(fighter);

}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_lw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, 0.0);

    return 0.into();
}
/* SPECIAL HI */

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dedede_special_hi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

    return 0.into();
}

/* GORDO */
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

pub fn install(){
    smashline::install_agent_init_callbacks!(dedede_init);
    smashline::install_status_scripts!(
        special_lw_jump_squat_exec,
        special_lw_jump_main,
        special_lw_attack_pre,
        special_lw_attack_main,
        special_lw_attack_exec,
        special_lw_pre,
        special_lw_exec,
        dedede_gordo_dead_end,
        dedede_special_hi_status_main,
    );
    special_lw_down_swing::install();
    special_lw_hi_swing::install();
}