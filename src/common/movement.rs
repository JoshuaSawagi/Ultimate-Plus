use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use std::{fs, path::Path};
use smash::phx::Vector2f;
use crate::controls::consts::globals::STICK_X;
use smash::app::sv_system;
use crate::controls::consts::STICK_HISTORY_FRAMES as OtherSTICK_HISTORY_FRAMES;
use crate::controls::ext::AnalogTracker;
use crate::util::*;
static mut STALE_MAX : f32 = 1.0;
static mut STALE_TIMER_MAX : i32 = 480;
static mut FOOTSTOOL_STALE: [f32; 8] = [21.0; 8];
static mut FOOTSTOOL_STALE_TIMER: [i32; 8] = [0; 8];
static HOLD_BUFFER_LIMIT : i32 = 20;
const MAX_PLAYERS: usize = 8;
const STICK_HISTORY_FRAMES: usize = 10;
static mut ANALOG_TRACKERS: [AnalogTracker; MAX_PLAYERS] = [AnalogTracker::new(); MAX_PLAYERS];

unsafe extern "C" fn pivot(fighter: &mut L2CFighterCommon) {
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lr = PostureModule::lr(boma);
    let status = StatusModule::status_kind(boma);

    // Update the analog tracker each frame
    ANALOG_TRACKERS[entry_id].update(
        ControlModule::get_stick_x(boma),
        ControlModule::get_stick_y(boma),
    );

    // Only allow pivots during initial dash
    if status == *FIGHTER_STATUS_KIND_DASH {
        let tracker = &ANALOG_TRACKERS[entry_id];
        if tracker.is_dash_pivot(lr) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
            JostleModule::set_status(boma, false); // Optional: disable collision for clean pivot visuals
        }
    }
}




//Moonwalk
unsafe extern "C" fn moonwalk(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let mut stickx = ControlModule::get_stick_x(boma);		
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lr = PostureModule::lr(boma);
        let walk_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0);
        let walk_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
        let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
        let max_moonwalk = walk_speed_max * 1.8;
		stickx = stickx * lr;
        let mw_modifier = 2.0;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind){
			if stickx < -0.2 {
                let moonwalk_speed = (stickx*walk_accel_mul - walk_accel_add)*mw_modifier;
                //println!("Moonwalk stuff! {} speed, {} mw change, {} mw max",  (get_speed_x(boma)*lr), moonwalk_speed, max_moonwalk);
                if (get_speed_x(boma)*lr)+moonwalk_speed > -max_moonwalk {
                    let speed = smash::phx::Vector3f { x: moonwalk_speed, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
                } else {
                    let current_back = (get_speed_x(boma)*lr);
                    let speed = smash::phx::Vector3f { x:-(current_back+max_moonwalk), y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &speed);
                }
            }
		};
    };
}

unsafe extern "C" fn hold_buffer_killer(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let buttons_list = [
            *CONTROL_PAD_BUTTON_ATTACK,
            *CONTROL_PAD_BUTTON_JUMP,
            *CONTROL_PAD_BUTTON_CATCH,
            *CONTROL_PAD_BUTTON_GUARD,
            *CONTROL_PAD_BUTTON_SMASH,
            *CONTROL_PAD_BUTTON_SPECIAL,
            *CONTROL_PAD_BUTTON_CSTICK_ON,
            *CONTROL_PAD_BUTTON_JUMP_MINI,
            *CONTROL_PAD_BUTTON_ATTACK_RAW,
            *CONTROL_PAD_BUTTON_SPECIAL_RAW,
            *CONTROL_PAD_BUTTON_SPECIAL_RAW2
        ];
        let mut hold_buffer_lim = HOLD_BUFFER_LIMIT;

        //Multiplies hold buffer duration by 2x during damage states to allow for pressing buttons out of hitstun as per usual
        if (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
            hold_buffer_lim *= 2;
        }
        //If time since you've pressed the button exceeds hold buffer limit, kills the input
        for i in buttons_list {
                if ControlModule::get_trigger_count(fighter.module_accessor, i as u8) > hold_buffer_lim && ControlModule::check_button_on(boma, i) 
                && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) 
                && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ItemModule::is_have_item(boma, 0))
                && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) && ItemModule::is_have_item(boma, 0))
                && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) //So taunts dont tpose
                && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind){ //Ignores shield and js
                    ControlModule::reset_trigger(fighter.module_accessor);
                    ControlModule::clear_command(fighter.module_accessor, true);
                    //ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
                }
        }
    };
}

unsafe extern "C" fn dash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && get_to_run_flag(boma) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
			};
		};
		if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if ControlModule::get_stick_y(boma) <= -0.6875 && ControlModule::get_flick_y(boma) >= 5 && ControlModule::get_flick_y(boma) < 20 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                };
            }
            let mut is_taunt_pressed = false;
            for i in [*CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_R] {
                if ControlModule::check_button_on_trriger(boma, i) {
                    is_taunt_pressed = true;
                }
            }
            if is_taunt_pressed {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_APPEAL, false);
            }
        }
    };
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont;
    if fighter.global_table[0x30].get_bool() == false {
        cont = false;
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x30].get_ptr());
        cont = callable(fighter).get_bool();
    }
    if cont == false {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
			let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 
			|| (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 
			|| ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_TreadJump)]
unsafe fn status_treadjump(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Added taunt buttons to the "Is Button Footstool" check
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON);
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    else {
        ControlModule::reset_flick_y(fighter.module_accessor);
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);
    let tread_jump_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_jump_disable_frame"));
    WorkModule::set_int(fighter.module_accessor, tread_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_TREAD, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
    fighter.sub_tread_jump_unique_process_init_inner();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_tread_jump_uniq_check();
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_jump_uniq_check as *const () as _));
    let mut tread_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_attack_frame"));
    if MotionModule::is_flag_start_1_frame(fighter.module_accessor) {
        tread_attack_frame -= 1;
    }
    WorkModule::set_float(fighter.module_accessor, tread_attack_frame as f32, *FIGHTER_STATUS_TREAD_WORK_FLOAT_ATTACK_FRAME);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadJump_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_tread_jump_uniq_check)]
unsafe fn sub_tread_jump_uniq_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_NO_REACTION) {
        let jump_mini = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON) {
            // If any valid footstool button is held, do not turn on the short hop flag
            ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        }
        else {
            let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            fighter.global_table[0x1B].get_f32() < jump_neutral_y
        };
        if jump_mini {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_air_tread_jump,
            status_treadjump,
            sub_tread_jump_uniq_check
        );
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, pivot)
    .on_line(Main, moonwalk)
    .on_line(Main, dash)
	.on_line(Main, hold_buffer_killer)
	.install();
    skyline::nro::add_hook(nro_hook);
}