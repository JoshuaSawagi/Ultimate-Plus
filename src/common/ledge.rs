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
use crate::common::Article::get_battle_object_id;

//No more ledgetrump
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::entry_cliff)]
pub unsafe fn disable_ledge_trump(boma: &mut smash::app::BattleObjectModuleAccessor) {
    if GroundModule::is_attach_cliff(boma) {
        return;
    }
    original!()(boma);
}

pub fn install() {
    skyline::install_hooks!(
        disable_ledge_trump
    );
}