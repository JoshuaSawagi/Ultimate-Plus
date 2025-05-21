
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::SituationKind;
use super::*;
use smash::phx::Vector3f;
use smash::app::sv_math;
use std::convert::TryInto;
use smash::app;
use smash::app::GroundCorrectKind;
use smash::phx::Vector2f;
use smash::app::KineticUtility;
use smash::lua2cpp::L2CFighterCommon_status_end_JumpSquat;
use smash::lua2cpp::L2CFighterCommon_status_JumpSquat_Main;
use crate::consts::globals::SITUATION_KIND;
use crate::consts::globals::CMD_CAT1;
use crate::consts::globals::CMD_CAT2;
use crate::consts::globals::CHECK_ATTACK_HI4_UNIQ;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH;
use crate::consts::FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY;
use crate::consts::globals::JUMP_SQUAT_MAIN_UNIQ;
use crate::consts::globals::PREV_STATUS_KIND;
use smash::app::GroundCliffCheckKind;
use crate::ext::Vec3Ext;
use crate::ext::Vec2Ext;
use interpolation::Lerp;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_EscapeAir)]
unsafe extern "C" fn status_pre_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let pos = *PostureModule::pos(fighter.module_accessor);
    let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    
    // Check for ground underneath the player
    let lower_bound = Vector2f::new(pos.x, pos.y - 3.0);
    let ground_pos_any = &mut Vector2f::zero();
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_any = GroundModule::line_segment_check(
        fighter.module_accessor,
        &Vector2f::new(pos.x, pos.y + 3.0),
        &lower_bound,
        &Vector2f::zero(),
        ground_pos_any,
        true,
    );

    let is_touch_stage = GroundModule::line_segment_check(
        fighter.module_accessor,
        &Vector2f::new(pos.x, pos.y + 3.0),
        &lower_bound,
        &Vector2f::zero(),
        ground_pos_stage,
        false,
    );

    // Stick input read
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);

    // Require diagonally downward input (not purely horizontal or purely downward)
    let is_diagonal_down = stick_y < -0.2 && stick_x.abs() > 0.2;

    // Determine if we can snap to ground
    let can_snap = is_diagonal_down && !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && dir_y > 0.0));

    if prev_status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH)
        && can_snap {
        
        GroundModule::attach_ground(fighter.module_accessor, true);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        PostureModule::set_pos(
            fighter.module_accessor,
            &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z),
        );
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    // Fallback to normal EscapeAir behavior
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_DISABLE, 
        false,
        false, 
        false, 
        0, 
        0, 
        0, 
        0, 
    );
    0.into()
}

//Setup Escape Air Slide Common
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_setup_escape_air_slide_common)]
unsafe extern "C" fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {
    let mut escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed"));
    let mut escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame"));
    let escape_air_slide_u_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_u_stiff_frame"));
    let escape_air_slide_d_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_d_stiff_frame"));
    let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
    let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
    let escape_air_add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    let stick_vec = sv_math::vec2_normalize(stick_x.get_f32(), stick_y.get_f32());
    let escape_air_angle = (stick_vec.y/stick_vec.x.abs()).atan().to_degrees();
    if escape_air_angle > 80.0 {
        escape_air_slide_speed = 2.1;
    }
    if escape_air_angle > 45.0 && escape_air_angle <= 80.0 {
        escape_air_slide_speed = 2.8;
    }
    let escape_air_slide_speed_vec = Vector2f{x: escape_air_slide_speed*stick_vec.x, y: escape_air_slide_speed*stick_vec.y};
    let lerp;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        WorkModule::set_float(fighter.module_accessor, stick_vec.x, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        WorkModule::set_float(fighter.module_accessor, stick_vec.y, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, escape_air_slide_speed_vec.x, escape_air_slide_speed_vec.y, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        if escape_air_angle < 0.0 {
            lerp = (escape_air_angle*-1.0)/90.0;
            escape_air_slide_stiff_frame = Lerp::lerp(&lerp, &escape_air_slide_d_stiff_frame, &escape_air_slide_stiff_frame);
        }
        else {
            lerp = escape_air_angle/90.0;
            escape_air_slide_stiff_frame = Lerp::lerp(&lerp, &escape_air_slide_u_stiff_frame, &escape_air_slide_stiff_frame);
        }
        WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
        WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame+escape_air_add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
        WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_smash_flash_s"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 4.0, z: 8.0}, &Vector3f::zero(), 1.1, &Vector3f{x: 18.0, y: 12.0, z: 0.0}, &Vector3f::zero(), false, 0, 0, 0);
    }
}

//Status Jumpsquat Main, enables Wavedash out of Jumpsquat
#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe extern "C" fn status_jumpsquat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_UNIQ].get_ptr()); callable(fighter).get_bool()} {
        return 1.into();
    }
    /* START OF NEW ADDITIONS */
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return 0.into();
        }
        if !fighter.sub_transition_specialflag_hoist().get_bool() {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() && {let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr()); callable(fighter).get_bool()} {
                    return 0.into();
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                    return 0.into();
                }
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
            && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1daca540be));
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    return 0.into();
                }
            }
        }
    }
    0.into()
}

//Status End Jumpsquat, clears flags
#[skyline::hook(replace = L2CFighterCommon_status_end_JumpSquat)]
unsafe extern "C" fn status_end_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_escapeair,
            setup_escape_air_slide_common,
            status_jumpsquat_main,
            status_end_jumpsquat
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}