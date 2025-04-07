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

#[skyline::hook(replace = L2CFighterCommon_status_JumpSquat_Main)]
unsafe extern "C" fn jumpcancelgrab(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT
    ].contains(&status) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
        }
   }
}
pub fn install() {
	Agent::new("fighter")
	.on_line(Main, jumpcancelgrab)
	.install();
}