use super::*;

unsafe extern "C" fn pickel_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch"), false, -1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {        
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -4.0, -1.0, Some(0.0), Some(-4.8), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(-0.5), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 45.0);
    FT_MOTION_RATE(fighter, 2.0);
}

unsafe extern "C" fn pickel_catch_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.8), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 3.2, Some(0.0), Some(7.0), Some(10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, -1.0, Some(0.0), Some(-4.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(-0.5), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 51.0);
    FT_MOTION_RATE(fighter, 2.0);
}

unsafe extern "C" fn pickel_catch_turn_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_CATCH_FLAG_SHOOT);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ArticleModule::set_visibility(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, Hash40::new("rod"), Hash40::new("rod_cast"), smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 1.0, Some(0.0), Some(-4.8), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, -3.2, Some(0.0), Some(7.0), Some(-10.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 3.6, 0.0, 7.0, -5.0, Some(0.0), Some(7.0), Some(-9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_G);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -5.0, 1.0, Some(0.0), Some(-4.0), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, 1.0, Some(0.0), Some(-0.5), Some(1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 2.5, 0.0, -3.0, -1.0, Some(0.0), Some(0.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    }
    frame(lua_state, 52.0);
    FT_MOTION_RATE(fighter, 2.0);
}

unsafe extern "C" fn pickel_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 25, 70, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, Hash40::new("throw_f"), false, -1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 17, 4.8);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_PUSHOBJECT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn pickel_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 65, 68, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_THROW_FLAG_FORGE_GENERATE_ENABLE);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 9, 2);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    if WorkModule::is_flag(boma, *FIGHTER_PICKEL_STATUS_THROW_FLAG_IS_GENERATE_FORGE) {
        frame(lua_state, 23.0);
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    smashline::Agent::new("pickel")
        .acmd("game_catch", pickel_catch_game)
        .acmd("game_catchdash", pickel_catch_dash_game)
        .acmd("game_catchturn", pickel_catch_turn_game)
        .acmd("game_throwf", pickel_throw_f_game)
        .acmd("game_throwlw", pickel_throw_lw_game)
        .install();
}
