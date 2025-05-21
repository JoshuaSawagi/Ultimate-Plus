use smash::lib::lua_const::*;
use smash::app::utility;
use smash::lib::{self, lua_const::*};
use smash::app::{self, lua_bind::*};
use smashline::L2CFighterCommon;
use smash::app::BattleObjectModuleAccessor;
use smash::app::BattleObject;
use crate::offsets;

//Aerial ECB Fixes, mainly for things like Link, Captain, Simon, and Richter (Credit to HDR)
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    smash::app::lua_bind::WorkModule::get_int(boma, *smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

extern "C"{
    #[link_name = "\u{1}_ZN3app14sv_information11is_ready_goEv"]
    pub fn is_ready_go() -> bool;
}

extern "C"{
    #[link_name = "\u{1}_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE"]
    pub fn get_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32;
}


pub fn in_range(num: f32, lower: f32, upper: f32) -> bool{
    num>lower && num<upper
}

pub unsafe fn clamp(x: f32, min: f32, max: f32) -> f32 {
    return if x < min { min } else if x < max { x } else { max };
}


pub unsafe fn clear_buffered_action(flag: i32, cmd: i32) -> i32 {
    return flag & !(flag & cmd);
}

pub unsafe fn add_buffered_action(flag: i32, cmd: i32) -> i32 {
    return flag | cmd;
}

pub unsafe fn compare_cat(cat: i32, fighter_pad_cmd_flag: i32) -> bool {
    return (cat & fighter_pad_cmd_flag) != 0;
}

pub fn get_category(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    get_category(boma) as i32
}

#[skyline::from_offset(offsets::get_battle_object_from_id())]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

pub fn get_battle_object_from_accessor(boma: *mut BattleObjectModuleAccessor) -> *mut BattleObject {
    unsafe {
        get_battle_object_from_id((*boma).battle_object_id)
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}


