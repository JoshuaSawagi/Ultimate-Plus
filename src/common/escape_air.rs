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
use crate::util::*;
use std::os::raw::c_int;
use std::os::raw::c_ulong;

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir_Main)]
unsafe extern "C" fn airdodge_momentum_stop(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);

    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        let current_frame = MotionModule::frame(boma);
        let stop_until_frame = 10.0; // Adjust to preference

        if current_frame <= stop_until_frame {
            let stop_momentum = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            KineticModule::clear_speed_energy_id(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            KineticModule::add_speed(boma, &stop_momentum);
        }

        // If airdodge is finishing, force Special Fall
        let motion_end = MotionModule::end_frame(boma); // Get the total length of the airdodge animation
        if current_frame >= motion_end {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
    }
}



pub fn install() {
	Agent::new("fighter")
    .on_line(Main, airdodge_momentum_stop)
	.install();
}