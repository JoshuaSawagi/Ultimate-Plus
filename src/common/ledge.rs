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
use crate::common::skyline_smash::app;

static mut LEDGE_POS: [smash::phx::Vector3f; 8] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 }; 8];

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]
unsafe extern "C" fn ledge_cling(fighter: &mut app::BattleObjectModuleAccessor) -> bool {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = GroundModule::hang_cliff_pos_3f(fighter);

    for i in 0..8 {
        if i == entry_id || LEDGE_POS[i].x == 0.0 {
            continue;
        }

        if (pos.x - LEDGE_POS[i].x).abs() < 1.0 && (pos.y - LEDGE_POS[i].y).abs() < 1.0 {
            return false;
        }
    }

    original!()(fighter)
}

#[skyline::hook(replace = GroundModule::entry_cliff)]
unsafe fn entry_cliff(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos = GroundModule::hang_cliff_pos_3f(fighter);
    LEDGE_POS[entry_id] = pos;

    original!()(fighter);
}

#[skyline::hook(replace = GroundModule::leave_cliff)]
unsafe fn leave_cliff(fighter: &mut app::BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(fighter, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

    original!()(fighter);
}


pub fn install() {
    skyline::install_hooks!(
        ledge_cling,
        entry_cliff,
        leave_cliff
    );
}