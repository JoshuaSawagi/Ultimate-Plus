mod cancel;
mod dacus;
mod escape_air_slide;
mod jab;
mod landing;
mod melee;
mod movement;
mod remove_quake;
mod ledge;

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;
pub static mut IS_GLOW: bool = false;
pub static mut DI_DIR: i32 = 0;
pub static mut LEDGE_OPTION: i32 = 0;
// 0 - Neutral Getup
// 1 - Ledge Attack
// 2 - Ledge Roll
// 3 - Ledge Jump
// 4 - Ledge Drop -> Double Jump
// 5 - Wait at ledge for 30 more frames
pub static mut LEDGE_OPTION_AFTER: i32 = 0;
// 0 - None
// 1 - Shield/Airdodge
// 2 - Aerial/Tilt (Have 2 lists of whether to ftilt/dtilt)
pub static mut LEDGE_DELAY : [i32; 8] = [0; 8];
pub static mut DJ_DELAY : [i32; 8] = [0; 8];
pub static mut DELAY_FRAMES: i32 = 30;
pub static mut DJ_DELAY_FRAMES: i32 = 21;

pub fn install() {
    cancel::install();
    dacus::install();
    escape_air_slide::install();
    jab::install();
    landing::install();
    melee::install();
    movement::install();
	remove_quake::install();
    ledge::install();
}