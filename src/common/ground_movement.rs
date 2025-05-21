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
use smash::app::sv_system;
use smash::app;
use crate::common::ground_movement::PostureModule::lr;



unsafe extern "C" fn ground_functionality(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let prev_status_kind_1 = StatusModule::prev_status_kind(boma, 1);
    let curr_frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let lr = PostureModule::lr(boma); // <-- Needed for smash_turn

    // Call the logics
    unsafe {
        pivot(boma, status_kind, stick_x, curr_frame, prev_status_kind, prev_status_kind_1);
    }
}

const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;

unsafe fn pivot(boma: *mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32, curr_frame: f32, prev_status_kind: i32, prev_status_kind_1: i32) {
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let mut pivot_boost: Vector3f = Vector3f { x: dash_speed * 0.70, y: 0.0, z: 0.0, };

    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW
        && stick_value_x == 0.0
        && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&prev_status_kind)
        && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&prev_status_kind_1) {
        
        
        // Optional tweak
        if curr_frame == 3.0 {
            pivot_boost.x = dash_speed * 0.35;
        }

        PostureModule::reverse_lr(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
        KineticModule::clear_speed_all(boma);
        KineticModule::add_speed(boma, &pivot_boost);
    }
}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, ground_functionality)
	.install();
}
