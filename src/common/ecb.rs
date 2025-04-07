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


#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let next_status = status_kind;
    let curr_status = StatusModule::status_kind(boma);
    //Wavedash
    if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if next_status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            // Allow wavedash if air dodge is inputted
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
            } else {
                original!()(boma, status_kind, arg3)
            }
        } else {
            original!()(boma, status_kind, arg3)
        }
    } else {
        original!()(boma, status_kind, arg3)
    }
}


pub fn install() {
    Agent::new("fighter");
    skyline::install_hooks!(
        change_status_request_hook
    );
}