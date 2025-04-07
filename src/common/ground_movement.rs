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
use crate::common::KineticEnergy::clear_speed;
use crate::util::*;

static mut ANALOG_TRACKERS: [AnalogTracker; MAX_PLAYERS] = [AnalogTracker::new(); MAX_PLAYERS];

unsafe extern "C" fn perfectpivot(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let lr = PostureModule::lr(boma);

    let stick_x = ControlModule::get_stick_x(boma);
    let stick_y = ControlModule::get_stick_y(boma);

    // Update the stick history buffer
    ANALOG_TRACKERS[entry_id].update(stick_x, stick_y);

    let tracker = &ANALOG_TRACKERS[entry_id];

    let status_kind = StatusModule::status_kind(boma);
    let is_dash_status = [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind);

    // Detect flick reversal
    if is_dash_status && tracker.was_flicked_left(0.5 * lr) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
        JostleModule::set_status(boma, false);
    }
}

