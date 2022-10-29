use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    {
        // Current FAF in motion list is 50, frame is 0 indexed so subtract a frame
        let special_n_fire_cancel_frame_ground = 49.0;
        // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        let landing_lag = 12.0;
        if MotionModule::frame(fighter.module_accessor) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, special_n_fire_cancel_frame_ground - landing_lag, true, true, false);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        //fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Banjo Grenade Airdodge Cancel
unsafe fn grenade_ac(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_STATUS_KIND_SPECIAL_LW])
    && fighter.motion_frame() > 15.0
    && fighter.is_cat_flag(Cat1::AirEscape)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    }
}

// Banjo Dair bounce
unsafe fn dair_bounce(fighter: &mut L2CFighterCommon){
    if fighter.is_motion(Hash40::new("attack_air_lw"))
    && fighter.motion_frame() < 45.0
    {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 45.0, true, true, false);
        }
    }
}

// Banjo Wondering Fail on command
unsafe fn wonderwing_fail(fighter: &mut L2CFighterCommon){
    if ((fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) && fighter.motion_frame() > 16.0)
    || (fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END) && fighter.motion_frame() < 3.0))
    && fighter.is_button_on(Buttons::Guard)
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
    }
}


unsafe fn indicator_breegull_fatigue(fighter: &mut L2CFighterCommon){
	let eggs_shot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    let eggs_Weakest = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"),hash40("bakyun_power_down_2_num"));
    let eggs_Weak = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"),hash40("bakyun_power_down_1_num"));
	if (eggs_shot >= eggs_Weak
	&& !fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END))
	{
		let sweatRate = if (eggs_shot<eggs_Weakest) {25.0} else {15.0};
		let sweatSize = if (eggs_shot<eggs_Weakest) {0.625} else {0.9};
		let modulo = fighter.motion_frame() % sweatRate;
		println!("{}",modulo);
		if (modulo<1.0)
		{
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_sweat"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, sweatSize, true);
		}
	}
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dair_bounce(fighter);
    //wonderwing_fail(fighter);
    blue_eggs_land_cancels(fighter);
    grenade_ac(fighter);
    indicator_breegull_fatigue(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_BUDDY)]
pub unsafe fn buddy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    buddy_frame(fighter);
}

pub unsafe fn buddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}