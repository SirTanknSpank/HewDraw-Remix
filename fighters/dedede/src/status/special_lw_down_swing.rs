use super::*;

unsafe extern "C" fn special_lw_down_swing_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
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
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );



   0.into()

}


unsafe extern "C" fn special_lw_down_swing_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); //This may cause double drift?
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_down_swing"), 0.0, 1.0, false, 0.0, false, false);
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("dedede_hammer"), false);
    VisibilityModule::set_int64(fighter.boma(), hash40("hammer") as i64, hash40("hammer_disp_off") as i64);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, false, -1);
    }
    let article = ArticleModule::get_article(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    let article_boma = sv_battle_object::module_accessor(object_id);

    ArticleModule::change_motion(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, Hash40::new("attack"), false, 0.0);
    ArticleModule::set_rate(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, 0.7);
    LinkModule::set_model_constraint_pos_ort(article_boma, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("haver"),   *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);    

    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_down_swing_main_loop as *const () as _))

}

unsafe extern "C" fn special_lw_down_swing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor){
        if fighter.is_situation(*SITUATION_KIND_GROUND){
            if MotionModule::frame(fighter.module_accessor) < 50.0 && !fighter.is_motion(Hash40::new("landing_heavy")){
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_down_swing_landing"), 0.0, 1.0, false, 0.0, false, false);
            }
            else{
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("dedede_hammer"), true);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_heavy"), 0.0, 1.0, false, 5.0, false, false);
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
            }
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::clear_speed_all(fighter.module_accessor);
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    if MotionModule::is_end(fighter.module_accessor){
        if fighter.is_situation(*SITUATION_KIND_GROUND){
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else{
        if MotionModule::frame(fighter.module_accessor) == 40.0{
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("dedede_hammer"), true);
        }
    }

    return 0.into()
}


unsafe extern "C" fn special_lw_down_swing_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    VarModule::set_float(fighter.battle_object, vars::dedede::instance::ADDED_JET_DAMAGE, 0.0);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("dedede_hammer"), true);

    0.into()
}

pub fn install(){
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_dedede"),
        statuses::dedede::SPECIAL_LW_DOWN_SWING,
        StatusInfo::new()
            .with_pre(special_lw_down_swing_pre)
            .with_main(special_lw_down_swing_main)
            .with_end(special_lw_down_swing_end)
    );
}