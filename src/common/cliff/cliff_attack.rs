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
use crate::consts::FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CliffAttack)]
unsafe fn status_end_cliffattack(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_end_cliffattack);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}