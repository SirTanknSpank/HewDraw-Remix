use super::*;
use globals::*;

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
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

#[status_script(agent = "dedede", status = *FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue{

    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    VarModule::set_float(fighter.battle_object, vars::dedede::instance::PRE_JETHAMMER_SPEED_X, x_speed);


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

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue{


    if MotionModule::motion_kind(fighter.boma()) ==smash::hash40("special_lw_max") || MotionModule::motion_kind(fighter.boma()) ==smash::hash40("special_air_lw_max"){
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){
            KineticModule::clear_speed_all(fighter.module_accessor);
        }
        if StatusModule::is_situation_changed(fighter.module_accessor){
            fighter.sub_change_motion_by_situation(Hash40::new("special_lw_max").into(), Hash40::new("special_air_lw_max").into(), true.into());  
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
    
        }

        if MotionModule::frame(fighter.module_accessor) ==61.0{
            AttackModule::clear_all(fighter.module_accessor);
        }
        if MotionModule::frame(fighter.module_accessor) ==71.0{
            let article = ArticleModule::get_article(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);

            ArticleModule::change_motion(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, Hash40::new("start"), false, -1.0);
            MotionModule::set_frame(article_boma, 17.0, true);
            MotionModule::set_rate(article_boma, -1.5);
        }

      }
    
    return 0.into();

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
        special_lw_attack_pre,
        special_lw_attack_exec,
        special_lw_pre,
        special_lw_wait_exec,
        special_lw_walk_exec,
        special_lw_fall_exec,
        dedede_gordo_dead_end,
        dedede_special_hi_status_main,
    );
}
