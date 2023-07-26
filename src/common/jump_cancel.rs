use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::common::jump_cancel::ControlModule::check_button_on;

// Jump Cancel Grab
#[fighter_frame_callback]
pub fn jump_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [
        *FIGHTER_STATUS_KIND_JUMP_SQUAT  
        ].contains(&status) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH){
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            }
        } 
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        jump_cancel,
    );
}