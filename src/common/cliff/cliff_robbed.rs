use crate::consts::FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID;
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

unsafe extern "C" fn common_cliff_robbed_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.05);
    //sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.08);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .status(Init, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, common_cliff_robbed_init_status)
    .install()
    ;
}