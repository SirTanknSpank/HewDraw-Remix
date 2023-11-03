use super::*;

unsafe extern "C" fn special_lw_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR){
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_cancel"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );

        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else{
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_cancel"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );    

        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));

    }

    VarModule::on_flag(fighter.battle_object, vars::dedede::instance::JET_HAMMER_MAX_CHARGE_FLAG);
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::JET_TIMER, 90);

    EffectModule::req_follow(fighter.boma(), Hash40::new("sys_steam3"), Hash40::new("hammer2"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);

    fighter.main_shift(special_lw_cancel_main_loop)


}

unsafe extern "C" fn special_lw_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if StatusModule::is_situation_changed(fighter.module_accessor){
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_cancel").into(), Hash40::new("special_air_lw_cancel").into(), true.into());

        if fighter.is_situation(*SITUATION_KIND_AIR){
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }

    if MotionModule::is_end(fighter.module_accessor){
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}


pub fn install(){
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_dedede"),
            statuses::dedede::SPECIAL_LW_CANCEL,
            StatusInfo::new()
                    .with_pre(special_lw_cancel_pre)
                    .with_main(special_lw_cancel_main)
                    .with_end(special_lw_cancel_end)
        );  
}