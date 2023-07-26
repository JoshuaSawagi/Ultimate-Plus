use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use smash2::app::work_ids::fighter::packun::status::special_lw::SPEED_Y;
use smash2::app::work_ids::fighter::sheik::status::special_hi::SPEED_Y as OtherSPEED_Y;
use smash2::app::work_ids::fighter::status::air_lasso_hang::SPEED_Y as OtherOtherSPEED_Y;
use smash2::app::work_ids::fighter::status::bury::SPEED_Y as OtherOtherOtherSPEED_Y;
use smash2::app::work_ids::fighter::duckhunt::status::special_hi_fly::SPEED_X;
use smash2::app::work_ids::fighter::inkling::status::special_s::SPEED_X as OtherSPEED_X;
use smash2::app::work_ids::fighter::robot::status::armspin::SPEED_X as OtherOtherSPEED_X;
use smash2::app::work_ids::fighter::sheik::status::special_hi::SPEED_X as OtherOtherOtherSPEED_X;
static mut STALE_MAX : f32 = 1.0;
static mut STALE_TIMER_MAX : i32 = 480;
static mut FOOTSTOOL_STALE: [f32; 8] = [21.0; 8];
static mut FOOTSTOOL_STALE_TIMER: [i32; 8] = [0; 8];


//DJC
#[fighter_frame_callback]
pub fn djc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL,].contains(&fighter_kind) {
			if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(boma)) {
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				};
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
				};
			};
		};
    };
}

//Parry Cancellable into a dash
#[fighter_frame_callback]
pub fn parrycanceldash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let mut stickx = ControlModule::get_stick_x(boma);		
		let mut lr = PostureModule::lr(boma);
		stickx = stickx * lr;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        if MotionModule::motion_kind(boma) == hash40("just_shield_off") {
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
				StopModule::end_stop(boma);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				//println!("Parry Cancel Dash!");
			};
		};
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
    smashline::install_agent_frame_callbacks!(
		parrycanceldash,
		djc
	);
    let _ = skyline::nro::add_hook(nro_hook);
}