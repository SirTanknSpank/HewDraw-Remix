
use super::*;

unsafe extern "C" fn ryu_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.200);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 9.0, 0.0, Some(0.0), Some(9.0), Some(9.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

unsafe extern "C" fn ryu_catch_dash_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn ryu_catch_turn_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 4.2, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-13.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 40, 30, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 15, 7);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        MeterModule::add(fighter.battle_object, 8.0 * MeterModule::damage_gain_mul(fighter.battle_object));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
    }
}

unsafe extern "C" fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 39, 64, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 15, 4);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        MeterModule::add(fighter.battle_object, 11.0 * MeterModule::damage_gain_mul(fighter.battle_object));
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
    }
}

unsafe extern "C" fn game_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let attr = if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
        Hash40::new("collision_attr_elec")
    } else {
        Hash40::new("collision_attr_normal")
    };
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 69, 40, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, attr, *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 50, 100, 40, 100, 4.5, 0.0, 4.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 50, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(boma, true, false);
        CHECK_FINISH_CAMERA(fighter, 1, 0);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        MeterModule::add(fighter.battle_object, 6.0 * MeterModule::damage_gain_mul(fighter.battle_object));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
    }
}

unsafe extern "C" fn game_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 86, 75, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 6, 20);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 30, 2.6, -1.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.0, 361, 100, 0, 30, 3.7, 5.3, 0.5, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(fighter.battle_object, false);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    smashline::Agent::new("ryu")
        .acmd("game_catch", ryu_catch_game)
        .acmd("game_catchdash", ryu_catch_dash_game)
        .acmd("game_catchturn", ryu_catch_turn_game)
        .acmd("game_throwf", game_throwf)
        .acmd("game_throwb", game_throwb)
        .acmd("game_throwlw", game_throwlw)
        .acmd("game_throwhi", game_throwhi)
        .install();
}
